# An Ability's own history fact reuses `knocked_out_last_turn`, unchanged

**Status:** Accepted — 2026-09-07

`Fezandipiti ex`'s `Flip the Script` — "Once during your turn, if any
of your Pokémon were Knocked Out during your opponent's last turn, you
may draw 3 cards" — reads the exact fact `Requirement::KnockedOutDuringOpponentsLastTurn`
already reads for `Unfair Stamp` (ADR 0024): `knocked_out_last_turn: [bool; 2]`,
set the moment a knockout happens and cleared once, for the owning
player, when their own turn ends. Nothing about that field's own
meaning changes because an Ability reads it instead of a Trainer's
requirement — it is the same "true for the whole turn right after the
knockout, false one cycle later" fact either way. `AbilityEffect::OncePerTurnIfKnockedOutLastTurnMayDrawCards(u32)`
reads it as `Action::UseAbility`'s own eligibility check, alongside
`OncePerTurnWhileActiveMayDrawCards`'s "is this Pokémon Active" check —
a standing Ability, not a triggered one, since the player may reach
for it any time during their turn once the fact holds, the same shape
`Run Errand` already takes.

`Fezandipiti ex`'s own attack, `Cruel Arrow` ("This attack does 100
damage to 1 of your opponent's Pokémon"), needed
`AttackEffect::DamageChosenOpponentPokemon(u32)` — the first
bench-damage-shaped effect to offer the opponent's Active as a target
too, not only the Bench, since nothing about this card's own choice is
restricted the way `DamageCountersToOpponentBenchAnyWay` and
`DiscardsOwnEnergyThenDamagesChosenBenched` are.
