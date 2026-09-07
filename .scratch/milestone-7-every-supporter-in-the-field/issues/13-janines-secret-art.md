# Janine's Secret Art

Type: task
Status: resolved

*"Choose up to 2 of your Darkness Pokémon. For each of those Pokémon,
search your deck for a Basic Darkness Energy card and attach it to that
Pokémon. Then, shuffle your deck. If you attached Energy to your Active
Pokémon in this way, it is now Poisoned."* 2 slots.

The most involved card in this milestone. Choosing up to 2 targets first,
then running a search-and-attach *per* target — not a single search
that then distributes, the way `Crispin` splits one search across two
destinations. And a Special Condition applied outside attack resolution,
the first time one has been.

- [x] Up to 2 targets can be chosen before any search runs, one search
      following per target chosen
- [x] A Special Condition can be applied outside attack resolution,
      conditioned on which target the search attached to
- [x] `Janine's Secret Art` plays

## Resolution

The generic `Decide`/`Slot` continuation reads a card's own `slots()` at
each step — a list fixed at definition time. It cannot carry a `PokemonId`
chosen by the player one phase earlier, since the card definition never
sees game state. This card needed a runtime-chosen target to survive into
a later phase, so it got a bespoke phase pair instead of forcing that
shape through the shared machinery: `Phase::ChoosingJaninesTargets`
collects up to 2 targets, then `Phase::JaninesSearch` runs one search per
target in turn, reading the target already fixed by the first phase.
Recorded as [ADR 0032](../../../docs/adr/0032-a-runtime-chosen-target-gets-its-own-phase-pair.md).

A `Destination::AttachToNamed(PokemonId)` was drafted to keep the attach
step inside the shared `Destination` vocabulary, then removed: the search
phase already knows its target and attaches directly, so the variant did
no work. ADR 0032 records this too.

The Special Condition is tracked by one flag (`attached_to_active`) that
follows the search across both phases, then applied once the whole effect
ends — the first Special Condition this engine applies outside attack
resolution.

`has_a_target` for this card is unconditionally `true`: "up to 2" reads as
playable even holding no Darkness Pokémon at all, unlike a card whose
minimum is nonzero.

Coverage: `admitted` 464 → 468 (4 prints of `Janine's Secret Art`);
`trainers` (still-refused, by kind) 327 → 323.
