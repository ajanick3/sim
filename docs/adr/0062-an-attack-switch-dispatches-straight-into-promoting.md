# An attack's own switch dispatches straight into `Phase::Promoting`

**Status:** Accepted — 2026-09-07

`Abra`'s `Teleportation Attack` switches the attacker itself with one
of the player's own Benched Pokémon — the milestone's first switch
read from an attack rather than a Trainer. `TrainerEffect::SwitchOwnActive`
already opens `Phase::Promoting { of: player, chooser: player, then: None }`
for exactly this choice, so the question was whether an attack needed
its own primitive or could dispatch into the same phase.  Nothing
about the choice differs by source — a switch is a switch, whichever
card causes it — so `AttackEffect::SwitchOwnActive` opens the same
`Phase::Promoting`, matching the mechanism section's stated intent to
reuse `SwitchOwnActive`/`SwitchOpponentActive` outright rather than
duplicating the machinery. An empty Bench opens no phase at all,
matching every other attack effect with no valid target.
`Metagross`'s `Bounce Back` (switch the opponent's Active, the
`SwitchOpponentActive` half of this ticket) waits for a print whose
other attack is also readable — every current print pairs it with a
shape (a persistent same-attack bonus, a board-count bonus) this
milestone has not built yet.
