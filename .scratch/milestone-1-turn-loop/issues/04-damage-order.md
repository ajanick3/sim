# Damage order

Type: task
Status: resolved

Apply damage in the order the rulebook sets out.

- [x] Base damage, then effects on the attacker, then Weakness and Resistance
- [x] Stop at 0 damage before Weakness applies
- [x] Damage lands in tens, because a counter is 10

## Answer

Resolved 2026-09-05 on branch `feat/milestone-1-turn-loop`, squashed to
`42cbd65` on `main` as pull request #2.

`damage_dealt` in `src/engine.rs`, with each numbered step in its own comment. Milestone 1 has no card that changes damage, so two steps are the identity; the shape is there for the card that does.
