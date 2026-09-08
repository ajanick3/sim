# The Special Energy primitive

Type: task
Status: resolved

`Growing Grass Energy`: *"As long as this card is attached to a
Pokémon, it provides {G} Energy. The {G} Pokémon this card is
attached to gets +20 HP."* 12 slots, and the simplest shape a Special
Energy comes in: one fixed type, one passive numeric modifier, no
trigger, no target, no choice.

Nothing in the engine reads a Special Energy's own text today —
`Refusal::IsASpecialEnergy` refuses every Energy-category card, the
same way for all of them, before any print's own effect is read. This
ticket builds the first Special Energy primitive and admits the first
card that needs it.

- [x] `Energy` can carry an effect the engine runs, the way an attack
      or an Ability already does
- [x] A Special Energy still pays an attack's cost by its own `kind`,
      exactly like a Basic Energy of the same type
- [x] A filter that names "a Basic Energy card" does not also match a
      Special Energy now that one can exist
- [x] `Growing Grass Energy` plays, its HP bonus read while attached

Recorded in [ADR 0080](../../../docs/adr/0080-a-special-energy-carries-its-own-effect.md),
which supersedes [ADR 0034](../../../docs/adr/0034-enhanced-hammer-is-refused-for-a-card-kind-out-of-scope.md).

## Resolution

`Energy` gains `effect: Option<EnergyEffect>`. `EnergyEffect` mirrors
`AttackEffect`/`AbilityEffect`'s own shape; its first variant,
`IncreasesCarrierHp(u32)`, is read directly by `effective_hp` — a
standing effect, not a choice, the same discipline ADR 0079 already
set for a passive Ability. `known_energy(name) -> Option<(Type,
EnergyEffect)>` in `src/import.rs` matches a print by name, since the
artifact carries no field naming what type a Special Energy provides
or what its own text does.

`CardFilter::BasicEnergy`, `BasicEnergyOfType`, `PokemonOrBasicEnergy`,
`PokemonOfTypeOrBasicEnergyOfType`, and
`PokemonWithoutRuleBoxOrBasicEnergy` all now read `effect.is_none()`
rather than merely `is_energy()` — until this ticket every `Energy`
instance was Basic by construction, so the distinction did not exist
to check.

`CardDef::as_energy()` added, mirroring `as_pokemon()`/`as_trainer()`.

35 existing `CardDef::Energy(Energy { .. })` literals across `src/`
and `tests/` gained `effect: None` — every one of them a Basic Energy
fixture, none needing a behavior change.

Admits `Growing Grass Energy`. Coverage moves from 676 to 677.
