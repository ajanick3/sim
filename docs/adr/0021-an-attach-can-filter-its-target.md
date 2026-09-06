# An attach can filter its target, the same way a search filters its card

**Status:** Accepted — 2026-09-06

`Destination::Attach` let a search put a card straight onto a Pokémon since
`Crispin`, and until now every such attach offered every Pokémon the
chooser controlled — `Crispin`'s own text never says otherwise. `N's PP Up`
and `Wondrous Patch` both do: one targets only a Benched Pokémon whose name
starts with "N's", the other only a Benched Pokémon printed as Psychic,
and matching the wrong Energy to the right Pokémon, or the right Energy to
the wrong Pokémon, is exactly the mistake a real player cannot make and
this engine must not offer.

The alternative was to let `TakeCardOnto`'s legality fall out of a general
check written against the printed card's own text — a closure, or a second
free function threaded through `legal_actions` case by case. That was
rejected on the same grounds ADR 0009 already settled for `CardFilter`: a
value the engine can inspect and compare, not a function it can only call.
`TargetFilter` is that value for a Pokémon in play, the same role
`CardFilter` already plays for a card in a zone. `Destination::Attach` now
carries one, and `Crispin`'s reading of it — any Pokémon the chooser
controls — is `TargetFilter::AnyInPlay`, not a case removed.

## Consequences

`TargetFilter` and `CardFilter` stay two separate types rather than one
combined value: a `CardFilter` reads a card sitting in a zone, and a
`TargetFilter` reads a Pokémon already in play, and nothing in the pool so
far needs to filter on both a card's own properties and a target's at once
in the same expression. `Wondrous Patch` looks like it might — a Psychic
Energy onto a Psychic Pokémon — but the two facts are checked by two
different filters against two different values, matching `Slot`'s existing
split between what a card must be (`filter`) and where it goes (`to`).
`GameState::matches_target` is the one place that reads a `TargetFilter`,
the same way `matches_slot` is the one place that reads a `CardFilter`.
