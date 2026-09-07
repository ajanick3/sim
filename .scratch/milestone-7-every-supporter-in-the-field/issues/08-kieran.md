# Kieran

Type: task
Status: resolved

*"Choose 1: Switch your Active Pokémon with 1 of your Benched Pokémon. •
During this turn, attacks used by your Pokémon do 30 more damage to your
opponent's Active Pokémon ex and Active Pokémon V."* 4 slots.

Both branches are built by the time this ticket starts — `SwitchOwnActive`
from ticket 06, the this-turn damage bonus from ticket 07. What is new is
the choice itself: nothing built offers a card whose own text branches
into two named effects, one of which the player picks before either runs.

- [x] A Trainer can name two effects and let the player pick one, rather
      than running a single fixed effect
- [x] `Kieran` plays, offering the switch and the this-turn bonus as
      alternatives, never both

## Resolution

`TrainerEffect::ChooseOneOf(Box<TrainerEffect>, Box<TrainerEffect>)`
carries both branches. `Phase::ChoosingOneOf` names the card rather than
the two effects themselves — the same continuation `Deciding` and `Paying`
already use — so `Phase` keeps its `Copy` derive.
[ADR 0029](../../../docs/adr/0029-a-choice-names-its-card-not-its-effects.md)
records why. `Kieran`'s two options are `SwitchOwnActive` and
`BonusDamageThisTurn`, both already built, with nothing added to make
them choosable.

Coverage went 445 → 450 (5 prints), and the field went 1628 → 1632
playable slots of 3660 — 44.6%.
