# Save, resume, and share a game across clients

Type: task
Status: needs-triage

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

## Open questions for triage

- Whether `appendMove` validates server-side (replay through `sim-wasm` in
  the action) or trusts the client and only the reader replays.
- How a resumed game handles an engine change that alters a replay — the
  recipe carries `engine_version`; decide whether a mismatch warns, blocks,
  or is ignored.
- Whether `moves.seq` gaps or races need handling beyond the primary-key
  conflict (last-writer-wins is acceptable per the spec).

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
