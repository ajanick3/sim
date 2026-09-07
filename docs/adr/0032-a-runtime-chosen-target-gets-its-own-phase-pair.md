# A runtime-chosen target gets its own phase pair, not a `Decide` slot

**Status:** Accepted — 2026-09-07

## Context

`Janine's Secret Art` chooses up to 2 of the player's own Darkness Pokémon,
then runs one search per chosen Pokémon, attaching what it finds to that
Pokémon by name. The generic search machinery — `TrainerEffect::Decide`, its
`Vec<Slot>`, and the `Phase::Deciding` continuation — re-reads a card's own
`slots()` list to open each step. That list is fixed at card-definition
time. It cannot hold a `PokemonId` chosen by the player one phase earlier,
because the card definition does not know the game state.

Two shapes were live. First: stretch `Slot`/`Destination` to carry a filled-in
target, threading it through `Phase::Deciding`. This asked `Phase` to hold a
`PokemonId` picked at runtime inside a structure built to stay `Copy` and
generic across every Trainer. Second: give this card its own phase pair,
naming the runtime choice directly.

## Decision

`Phase::ChoosingJaninesTargets { player, remaining, chosen: [Option<PokemonId>; 2] }`
collects the choice. `Phase::JaninesSearch { player, targets, index, attached_to_active }`
runs one search per chosen target in turn, reading `targets[index]` — a
`PokemonId` already fixed by the first phase — as the attach point. Neither
phase reads `slots()`; both are bespoke to this card's own two-step shape.

A `Destination::AttachToNamed(PokemonId)` was drafted during this design to
keep the attach step inside the shared `Destination` vocabulary. It went
unused: `Phase::JaninesSearch`'s own `legal_actions` arm reads the library
and attaches directly, since the target is already fixed and needs no
filter. The variant was removed rather than wired in after all — a `Slot`
whose `Destination` is already decided does no work a plain attach does not.

The Special Condition this card applies (Poisoned, if the Active received
the Energy) is also new: every other Special Condition affects the engine
during attack resolution. Here it is asked for by a Supporter, tracked by
the same `attached_to_active` flag that follows the search from phase to
phase, and applied once the whole effect is done.

## Consequences

A card whose runtime choice must survive into a later phase gets a bespoke
phase pair naming that choice directly, rather than forcing the generic
`Decide`/`Slot` continuation to carry state it was not built to hold. This
keeps `Phase` `Copy` at the cost of a phase pair per such card; the
alternative cost was a shared structure carrying dead weight for every
Trainer that does not need it.
