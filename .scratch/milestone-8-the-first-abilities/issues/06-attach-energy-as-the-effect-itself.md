# Attaching Energy from hand as the effect itself

Type: task
Status: resolved

`Teal Mask Ogerpon ex`'s Ability, "Teal Dance": *"Once during your
turn, you may attach a Basic {G} Energy card from your hand to this
Pokémon. If you attached Energy to a Pokémon in this way, draw a
card."* 38 slots.

- [x] The Ability opens a choice of which Energy (if any), unlike
      every earlier ticket's immediate-resolution shape
- [x] The once-per-turn limit is spent on actually attaching, not on
      opening the choice
- [x] `Teal Mask Ogerpon ex` plays

Recorded in [ADR 0074](../../../docs/adr/0074-an-energy-attaching-ability-spends-its-limit-on-attach-not-open.md).

## Resolution

`AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(Type)`.
`Action::UseAbility` opens `Phase::DecidingToUseTealDance` for this
effect instead of resolving immediately — the first Ability needing a
further choice. `Limit::AbilityUsed` moves out of `Action::UseAbility`'s
own unconditional spend into each resolving arm, so this one can defer
it to `Action::AttachEnergyForTealDance`. The attach does not touch
`Limit::EnergyAttached`, the ordinary manual-attach gate — independent
per the card's own ruling.

`Teal Mask Ogerpon ex`'s own attack, `Myriad Leaf Shower`, needed
`Count::EnergyOnBothActivesCount` — the first count read from both
sides of the board at once. `count_for_attack` gains a `defender`
parameter.

Admits all 8 Teal Mask Ogerpon ex prints. Coverage: `admitted`
594 -> 602.
