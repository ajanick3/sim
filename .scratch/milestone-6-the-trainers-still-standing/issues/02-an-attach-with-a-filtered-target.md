# An attach whose target is filtered

Type: task
Status: resolved

`N's PP Up`: *"Attach a Basic Energy card from your discard pile to 1 of
your Benched N's Pokémon."* `Wondrous Patch`: *"Attach a Basic Psychic
Energy card from your discard pile to 1 of your Benched Psychic Pokémon."*
28 and 18 slots.

`Destination::Attach` exists since `Crispin`, but nothing built has ever
refused a target: every Pokémon the chooser controls is offered. These two
cards each read the target's own name or type before offering it — one by
a name prefix ("N's"), the other by printed type — and `Wondrous Patch`
also restricts the Energy attached to that same type, not any Basic
Energy.

Check whether one shape covers both restrictions before building two: a
filter on the card being attached, and a separate filter on the Pokémon
receiving it, are two different things read against two different values.
`Wondrous Patch` also needs `CardFilter` to name a Basic Energy of one
type, not any Basic Energy — `BasicEnergy` today matches every one.

- [x] The Pokémon `Destination::Attach` offers can be filtered, not only
      "any Pokémon in play"
- [x] `N's PP Up` plays, offered only for a Benched Pokémon whose name
      starts with "N's"
- [x] `Wondrous Patch` plays, offered only for a Psychic Benched Pokémon,
      and only a Psychic Basic Energy

## Resolution

`Destination::Attach` carries a `TargetFilter`, matched against a Pokémon
in play the same way `CardFilter` matches a card in a zone —
[ADR 0021](../../../docs/adr/0021-an-attach-can-filter-its-target.md)
records why it is a value of its own rather than folded into `CardFilter`.
`Crispin`'s "any Pokémon the chooser controls" becomes
`TargetFilter::AnyInPlay`, not a special case removed.

`Wondrous Patch` needed both halves the ticket predicted: a filter on the
card (`CardFilter::BasicEnergyOfType`) and a separate filter on the target
(`TargetFilter::BenchedOfType`) — two facts, two filters, matching `Slot`'s
existing split between what a card must be and where it goes.

Coverage went 397 → 402 (5 prints), and the field went 1421 → 1467
playable slots of 3660 — 40.1%.
