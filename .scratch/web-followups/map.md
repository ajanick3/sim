# Map: web front-end follow-ups

## Destination

A game in the browser can be saved, resumed, and shared by a link, and a
player can report a bug without leaving the page.

## Notes

The front end today (`web/app/table.tsx`) holds one `Game` handle in a
React ref. Nothing persists. `sim-wasm` exposes `legal_actions`,
`apply(index)`, `log`, `view` — all as JSON strings — plus `Game::synthetic`
and `Game::standard`.

ADR 0096 already recorded the state-transport decision: no `serde` on
`GameState`; a shareable game is `{seed, deck a, deck b, chosen action
indices}`, replayed from the start on load. Games are short and replay is
fast (`selfplay` runs thousands per second).

## Tickets

- 01 — save and resume a game, then share it across clients from a server.
- 02 — report a bug from inside the UI, routed to an agent.

## Decisions so far

Storage direction set 2026-09-10: Neon serverless Postgres, written through
Next.js Server Actions, per the operator-supplied "Add Postgres to a
Next.js app on Vercel" guide. `DATABASE_URL` comes from `vercel env pull`.
Tables live in tickets 01 (`games`, `moves`) and 02 (`bug_reports`).
Not yet built — the Neon project and Vercel link are provisioning steps
the operator runs.

Ticket 01 triage resolved 2026-09-11: `appendMove` trusts the client and
validates on read, an `engine_version` mismatch warns rather than blocks,
and `moves.seq` races get no handling beyond the primary-key conflict.
Status set to `ready-for-human` — the Neon/Vercel provisioning step still
gates the remaining AC; details under
[the ticket](issues/01-save-resume-and-share-a-game.md).
