# A Special Energy carries its own effect, the same way an attack or an Ability does

**Status:** Accepted — 2026-09-08

[ADR 0034](0034-enhanced-hammer-is-refused-for-a-card-kind-out-of-scope.md)
refused every Special Energy outright: `Energy` carried only a
`print_id`, a `name`, and a `kind: Type`, so no Special Energy's own
text could ever run, and admitting one with a filter that could never
match anything would not be a faithful build of the card. That
reasoning held only as long as `Energy` had nowhere to put the text.
Counted across the 61 committed decks, Special Energy is 128 slots
behind seven distinct names — large enough, and repeated enough
across real decks, to be worth the same treatment `AttackEffect` and
`AbilityEffect` already got.

## Decision

Give `Energy` an `effect: Option<EnergyEffect>` field: `None` for a
Basic Energy every deck supplies for itself, `Some(_)` for a Special
Energy, matched by its own print name through a new `known_energy`
function in `src/import.rs` — the same discipline `known_attack` and
`known_ability` already hold, since the artifact carries no field
naming what type a Special Energy provides or what its text does.

A card whose text `known_energy` does not yet name stays refused for
`Refusal::IsASpecialEnergy`, the same reason ADR 0034 gave — only the
refusal's cause moved, from "the engine cannot represent this at all"
to "this print is not built yet."

## Consequences

`CardFilter::BasicEnergy` and its three siblings
(`BasicEnergyOfType`, `PokemonOrBasicEnergy`,
`PokemonOfTypeOrBasicEnergyOfType`, `PokemonWithoutRuleBoxOrBasicEnergy`)
now read `effect.is_none()` rather than merely `is_energy()` — until
this decision, every `Energy` the engine ever instantiated was Basic,
so the distinction was invisible; a Special Energy make it real, and
a Trainer or attack that names "a Basic Energy card" must not also
find one.

[ADR 0034](0034-enhanced-hammer-is-refused-for-a-card-kind-out-of-scope.md)'s
own status becomes superseded by this record. `Enhanced Hammer`
("discard a Special Energy from 1 of your opponent's Pokémon") stays
refused for now regardless — its own target needs at least one
Special Energy actually admitted and in play to mean anything, which
this record's own first ticket does not yet reach — but its refusal
reason is no longer "this cannot exist," only "this needs a
discard-any-Special-Energy shape not yet built."
