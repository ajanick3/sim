# A choice between two effects names its card, not its effects

**Status:** Accepted — 2026-09-07

`Kieran` prints two options and lets the player pick one — the first
card in the pool whose own text branches. `TrainerEffect::ChooseOneOf`
carries both, boxed, since `TrainerEffect` gave up `Copy` for a `Vec` back
in ADR 0018 and a second heap-allocated branch costs nothing new. The
harder question was the phase: does `Phase::ChoosingOneOf` hold the two
candidate effects, or only enough to find them again?

Holding the two effects directly was rejected. `Phase` is `Copy`
everywhere else in the engine, matched and reassigned by value at every
call site; a `Box<TrainerEffect>` field would cost that for every phase,
not only this one. The phase instead names the card —
`ChoosingOneOf { player, card: CardId }` — the same continuation
`Phase::Deciding` and `Phase::Paying` already use for a search's next slot
and a cost's own effect: the fact the phase needs is already sitting in
the `CardDb`, read back through `def_of(card)` rather than copied out a
second time.

`Action::ChooseOption { first: bool }` resolves it: read the card's
effect, match `ChooseOneOf`, take the named half, discard the other
unread, and run it exactly the way `Action::PlayTrainer` runs any other —
`Phase::Main` first, so a branch with no phase of its own leaves the
right phase behind it.

## Consequences

Only the chosen branch ever executes; the other is dropped without
running any of its own logic, matching the printed card exactly —
`Kieran`'s switch and its damage bonus never both happen from one play.
`ChooseOneOf` composes with anything already built: `Kieran`'s two options
are `SwitchOwnActive` and `BonusDamageThisTurn`, both existing effects,
with nothing added to either to make them choosable.
