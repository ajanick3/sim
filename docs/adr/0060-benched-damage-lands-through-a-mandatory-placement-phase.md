# Damage counters placed on the Bench go through a mandatory placement phase

**Status:** Accepted — 2026-09-07

Dragapult ex's Phantom Dive puts 6 damage counters on the opponent's Bench,
in any split the attacker likes. No earlier attack effect had touched a
Bench Pokémon's damage; this is the first one. Two shapes were live: an
optional, declinable placement (matching the engine's `Decide` pattern used
for searches), or a mandatory one that must place every counter before
anything else can happen. The card's own text — "in any way you like" —
describes freedom in the split, not in whether to place at all, so
placement is mandatory: `Phase::DistributingDamageCounters` offers only
`Action::PlaceDamageCounter`, one per Bench Pokémon the opponent has, with
no decline action. The phase auto-transitions to `Phase::Main` and calls
`settle` once its `remaining` count reaches zero, the same auto-completion
`Action::TakeEnergyForJanine` already uses, rather than asking for an
explicit "done" action. An empty opponent Bench never opens the phase at
all — the effect resolves to nothing, matching how every other effect with
no valid target already behaves.
