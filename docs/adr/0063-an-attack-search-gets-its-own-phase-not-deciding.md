# An attack's search opens its own phase, not `Phase::Deciding`

**Status:** Accepted — 2026-09-07

`Drilbur` and `Toxel`'s `Call for Family` searches the library for up
to 2 Basic Pokémon and puts them on the Bench — the same shape
`Buddy-Buddy Poffin`'s `TrainerEffect::Decide` already reads, but from
an attack. `Phase::Deciding` and its `enter_slot` machinery name the
Trainer card being resolved and read its slots back from
`state.def_of(card).as_trainer()` — a Pokémon's attack has no such
card to point at, and generalizing `enter_slot` to accept either kind
of source would touch machinery every existing Trainer search still
depends on, for a shape this ticket needs only once. `Phase::SearchingLibraryForBasics`
carries its own `remaining` count directly rather than a card and a
step, the same choice ticket 07's `Phase::DistributingDamageCounters`
and ticket 08's `Phase::ChoosingBenchDamageTarget` already made for
attack-driven phases with no card to read back from. Its filter is
fixed at `CardFilter::PokemonOfStage(Stage::Basic)` and its
destination at the Bench — the one shape this ticket's cards need —
rather than carrying a `Slot` for generality nothing yet asks for.
`Duskull`'s `Come and Get You` (a Bench search from the discard,
filtered to its own name) is a narrower version of the same idea, but
needs a name filter `CardFilter` does not yet have; deferred to its
own ticket rather than forcing it in ahead of a live need.
