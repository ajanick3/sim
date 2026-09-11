# Save, resume, and share a game across clients

Type: task
Status: ready-for-human

The operator asked for "a way to export board state to resume later" and
"some sort of server persistent state across clients".

A game today lives only in one tab's memory. This ticket makes a game
outlive the tab and be reachable from another client.

## Shape

Follow ADR 0096: a game is not a serialized `GameState`, it is the recipe
to rebuild one — `{ mode, seed, deck_a, deck_b, actions: [index, …] }` —
replayed from the start. `sim-wasm` gains a constructor that takes this
recipe and applies each index in order.

- **Export / resume, no server.** The page can hand the player the recipe
  (a link with the recipe encoded, or a downloaded file) and rebuild the
  game from one it is given. Pure client, no new infrastructure.
- **Shared, server-held.** A store keeps a recipe under an unguessable id
  and appends an action index as each move is applied. Any client that
  opens the id replays to the current point. Last-writer-wins is
  acceptable; live turn-by-turn sync is out of scope (see spec).

## Storage: Neon serverless Postgres

The operator supplied Neon's "Add Postgres to a Next.js app on Vercel"
guide. The direction:

- Connect a Neon project through the Vercel dashboard; `vercel env pull`
  brings `DATABASE_URL` into `.env.development.local`.
- `npm install @neondatabase/serverless`; call `neon(process.env.DATABASE_URL)`
  from a Next.js Server Action (`'use server'`).
- Schema, created in the Neon SQL editor:
  - `games (id text primary key, recipe jsonb not null, engine_version
    text not null, created_at timestamptz default now())`
  - `moves (game_id text references games(id), seq int, action_index int,
    primary key (game_id, seq))`
- A Server Action `createGame(recipe)` inserts the row and returns the id;
  `appendMove(id, seq, index)` inserts one `moves` row; a loader reads the
  recipe and every `moves` row ordered by `seq`.

## Decided at triage (2026-09-11)

- **`appendMove` trusts the client; the reader validates.** No replay on
  write. The reader already replays every move through `sim-wasm` to build
  a view — a corrupt or illegal index just fails that replay, at read time,
  where it's needed anyway. No duplicate wasm cost on every write.
- **An `engine_version` mismatch warns, and still replays.** The page shows
  a banner ("this game was created on an older version of the engine") but
  attempts the replay regardless. Blocking outright would strand every
  saved game after any engine change at all, including ones that don't
  affect replay; a warning is honest without being destructive.
- **No extra handling for `moves.seq` gaps or write races beyond the
  primary-key conflict.** Per the spec, last-writer-wins is acceptable — a
  duplicate `seq` simply fails to insert. No gap-detection or locking
  logic is added.

All three decisions above are settled, but the two remaining AC items are
still blocked on the operator: a Neon project must exist and be linked to
this Vercel project (`vercel env pull` bringing `DATABASE_URL` into
`web/.env.development.local`) before `@neondatabase/serverless` or any
Server Action can be written against a real database. Neither exists yet
(`web/.env*` and `.vercel/project.json` are both absent as of 2026-09-11).
This is why the status is `ready-for-human`, not `ready-for-agent` — the
provisioning step needs the operator's own Neon/Vercel dashboards, e.g.
via `/wizard`. Once `DATABASE_URL` is available, this ticket's remaining
AC (the schema, the two Server Actions, and the cross-client resume test)
is agent-workable and the status should move to `ready-for-agent`.

## Acceptance criteria

- [x] `sim-wasm` rebuilds a game from a recipe and its list of action
      indices, matching the game those moves produced live. — PR #238
      (`Game::replay_standard`, `Game::history`).
- [x] The page exports the current game as a recipe and resumes one from a
      recipe it is given, with no server. — PR #247. The recipe rides in
      the `?g=` query string; `syncUrl` rewrites it on every move; a
      "Copy link" button shares the current URL. `app/recipe.ts` encodes,
      decodes, and validates.
- [ ] A Neon Postgres store, written through Next.js Server Actions, keeps
      a recipe by id, accepts appended action indices, and serves both to
      any client that has the id. — blocked: needs the Neon project
      created and linked to Vercel (`DATABASE_URL` via `vercel env pull`).
- [ ] Two browsers opening the same id see the same board, one client's
      moves visible to the other on reload. — a shared *link* already
      works (PR #247); a shared, updating *id* needs the server above.
- [ ] The recipe records which engine version produced it. — deferred to
      the server slice, where a persisted game outlives a redeploy. The
      recipe already carries `v: 1`, its own format version.
