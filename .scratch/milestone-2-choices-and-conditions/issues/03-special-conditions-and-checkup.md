# Special conditions and checkup

Type: task
Status: resolved

Add the five Special Conditions and the Pokémon Checkup between turns.

- [x] Only the Active can carry a condition
- [x] Asleep, Paralyzed, and Confused replace each other; Burned and Poisoned do not
- [x] The checkup runs conditions first, then knocks out anything at 0 HP
- [x] A player chooses the order of their own effects in the checkup

## Answer

Resolved 2026-09-05 on branch `feat/special-conditions`, commits `8af87b9`,
`b72e76a`, `621d6c7`, and `eb6feb4`.

All five conditions work. An attack carries an `inflicts` field, which is how a
condition reaches a Pokémon. The checkup is a phase: the player whose turn just
ended resolves their own effects first, and chooses the order within them,
which rule 47 asks for. Paralysis recovers without asking, because its recovery
is not a choice.

Written test first, six slices. Two tests passed for the wrong reason before
they were fixed, both because the fixture never paid for an attack, so no
attack was legal whatever the condition did.

Two fixture lessons worth keeping:

- A scripted generator makes the shuffle degenerate, so a test that needs a
  real deal must use a seeded one. `retreating_removes_every_condition` does.
- A knockout ends the game when the owner has no Bench, so a test about
  knockouts searches seeds for a deal that benched both players.
