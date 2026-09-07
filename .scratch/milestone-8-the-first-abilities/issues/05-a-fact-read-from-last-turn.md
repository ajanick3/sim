# A fact read from last turn

Type: task
Status: resolved

`Fezandipiti ex`'s Ability, "Flip the Script": *"Once during your
turn, if any of your Pokémon were Knocked Out during your opponent's
last turn, you may draw 3 cards. You can't use more than 1 Flip the
Script Ability each turn."* 54 slots.

- [x] The Ability reads `knocked_out_last_turn`, the field
      `Requirement::KnockedOutDuringOpponentsLastTurn` already reads
      for `Unfair Stamp` — unchanged, no new state
- [x] `Fezandipiti ex` plays

Recorded in [ADR 0073](../../../docs/adr/0073-an-ability-reads-the-same-history-field-a-requirement-does.md).

## Resolution

`AbilityEffect::OncePerTurnIfKnockedOutLastTurnMayDrawCards(u32)`, a
standing Ability (offered through `Action::UseAbility`, the same as
ticket 01's `Run Errand`) gated on `knocked_out_last_turn[player]`
instead of "is this Pokémon Active."

`Fezandipiti ex`'s own attack, `Cruel Arrow`, needed
`AttackEffect::DamageChosenOpponentPokemon(u32)` too — flat damage to
one opponent Pokémon of the player's choice, Active or Benched alike,
the first bench-damage shape not restricted to the Bench.

Admits all 5 Fezandipiti ex prints. Coverage: `admitted` 589 -> 594.
