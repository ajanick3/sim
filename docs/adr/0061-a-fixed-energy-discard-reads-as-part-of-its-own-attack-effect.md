# A fixed Energy discard reads as part of its own attack effect, not a separate primitive

**Status:** Accepted — 2026-09-07

`N's Darmanitan`'s `Flamebody Cannon` discards every Energy attached to
the attacker, then deals flat damage to one Benched Pokémon the player
chooses — the milestone's first attack effect to pay a cost in the
attacker's own Energy. Two shapes were live for the discard: a general
`AttackEffect::DiscardOwnEnergy` primitive, reusable across every card
in this ticket's family, matched separately from whatever each card's
Energy pays for; or one effect per card, folding the discard and its
payoff into a single variant. The card's own text ties the two
together unconditionally — "discard all... and this attack also
does..." — with no case where one happens without the other, so a
shared discard primitive would still need per-card glue to sequence it
against the bench-damage choice. `DiscardsOwnEnergyThenDamagesChosenBenched`
keeps the discard and the choice in the order the card prints them: the
discard needs no player choice ("all" has nothing to pick), so it runs
immediately in `resolve_attack_effect`; the damage does need a choice
of target, so it opens `Phase::ChoosingBenchDamageTarget`, mirroring
`Phase::DistributingDamageCounters`'s shape but for a single Pokémon
rather than a split. A future card with a genuinely reusable "discard
N of your own Energy, chosen or not" shape gets its own primitive when
one actually recurs — this ticket's remaining cards (`Metagross`,
`Raging Bolt ex`, `Mega Sharpedo ex`, `Mega Excadrill ex`,
`Mega Skarmory ex`) each pair a different cost with a different
payoff, and none is built here.
