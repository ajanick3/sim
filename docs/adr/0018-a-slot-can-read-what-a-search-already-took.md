# A slot can read what a search already took, and can carry a target

**Status:** Accepted — 2026-09-06

`Crispin` searches for up to 2 Basic Energy of different types, puts one in
hand, and attaches the other. Two facts about it do not fit the search
machinery ADR 0016 built. First, whether the second Energy is legal to take
depends on the type of the first — a fact no static `CardFilter` can hold,
since a filter is checked against one card in isolation. Second, its second
slot attaches straight to a Pokémon, which needs a target chosen at the same
time as the card, not only a destination.

Two alternatives were live for the first problem: give `CardFilter` a
variant parameterized by a type to exclude, computed fresh each time a slot
opens; or have the slot read the search's own history. The first was
rejected because it would require `resolve_trainer` to inspect the previous
slot's actual pick and rebuild a filter from it — logic that belongs to the
search, not to a value describing one card. Instead, `Phase::Deciding` grew
a `previous: Option<CardId>` that survives every slot transition, and `Slot`
grew a `bool` naming whether it reads that field. Only the flag is a static
fact of the card; the value it compares against is state.

For the second problem, `Destination` grew a third variant, `Attach`, and
`legal_actions` offers `Action::TakeCardOnto { card, target }` in its place —
the same shape `Action::MoveEnergy` already established for choosing a card
and a Pokémon together, rather than opening a second phase to ask "attach it
where?" after the card is already chosen.

## Consequences

`Phase::Deciding` now carries two fields no card before `Crispin` reads:
`excludes_type_of_previous` and `previous`. Every other slot sets the first
to `false` and ignores the second — the continuation costs nothing to a
one-slot search, the same as `step` did.
