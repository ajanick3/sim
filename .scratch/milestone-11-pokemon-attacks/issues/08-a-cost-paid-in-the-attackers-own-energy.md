# A cost paid in the attacker's own Energy

Type: task
Status: resolved

*"Discard all Energy from this Pokémon, and this attack also does 90
damage to 1 of your opponent's Benched Pokémon."* — `N's Darmanitan`'s
`Flamebody Cannon`.

- [x] `AttackEffect::DiscardsOwnEnergyThenDamagesChosenBenched(u32)`
- [x] New `Phase::ChoosingBenchDamageTarget { player, damage }`, a
      single-target counterpart to ticket 07's split-across-the-Bench
      phase
- [x] The discard is unconditional and needs no phase; the damage
      opens one only when the opponent has a Bench to choose from

Recorded in [ADR 0061](../../../docs/adr/0061-a-fixed-energy-discard-reads-as-part-of-its-own-attack-effect.md).

## Resolution

One new `AttackEffect` variant, one new `Phase` variant, one new
`Action` variant (`DamageBenchedPokemon`) with its `player_to_act`,
`legal_actions`, `describe`, and `apply` arms.

`N's Darmanitan` completes (3 prints): its other attack (`Back Draft`)
was already read in ticket 02.

The rest of this shape's cards — `Metagross`, `Raging Bolt ex`,
`Mega Sharpedo ex`, `Mega Excadrill ex`, `Mega Skarmory ex` — each pair
a different Energy cost with a different payoff (an optional discard
for bonus damage, a variable-amount discard, a shuffle-back rather
than a discard). Deferred to their own tickets rather than forcing one
primitive over all of them; see ADR 0061.

Coverage: `admitted` 535 -> 538 (3 prints).
