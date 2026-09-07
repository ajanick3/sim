# Risky Ruins

Type: task
Status: resolved

*"Whenever any player puts a Basic non-{D} Pokémon onto their Bench
during their turn, place 2 damage counters on that Pokémon."*

New: a shared `apply_risky_ruins` helper, called from both places a
Pokémon newly arrives on a Bench — `PlayBasic` and a search's own
`Destination::Bench` — rather than one. "During their turn" needed no
separate check: nothing in this engine benches a Pokémon outside the
acting player's own turn.

- [x] A non-Darkness Basic benched from hand takes 20 damage
- [x] A Darkness Basic is exempt
- [x] Applies on either player's own turn, to their own Bench

## Resolution

One new `TrainerEffect`, one new helper, two call sites.

Coverage: `admitted` 509 -> 510 (1 print); `trainers` (refused)
282 -> 281.
