# A flexible provided type

Type: task
Status: open

`Prism Energy`: *"As long as this card is attached to a Pokémon, it
provides {C} Energy. If this card is attached to a Basic Pokémon,
this card provides every type of Energy but provides only 1 Energy at
a time."* 12 slots.

The most invasive ticket in this milestone: `Energy.kind` is a fixed
`Type` read directly by `attached_energy_types` and `pays_cost`, and
every other Special Energy in this milestone keeps that assumption.
`Prism Energy` provides whichever type an attack's cost still needs,
one unit only, only while attached to a Basic — genuinely
context-dependent at the moment cost is checked, not a fact fixed at
attach time. Likely needs `pays_cost` (and any other direct reader of
`attached_energy_types`) to treat a card carrying this effect as a
wildcard, matched last after every named type is satisfied, the same
priority `pays_cost`'s own doc comment already gives a Colorless
entry.

- [ ] Attached to a Basic Pokémon, it pays any single type an
      attack's cost still needs, contributing exactly 1 Energy
- [ ] Attached to a Stage 1 or 2 Pokémon, it pays only Colorless,
      like a plain Basic Energy of no named type
- [ ] `Prism Energy` plays

Blocked by: 01
