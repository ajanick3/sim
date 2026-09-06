# An attach whose target is filtered

Type: task
Status: ready-for-agent

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

- [ ] The Pokémon `Destination::Attach` offers can be filtered, not only
      "any Pokémon in play"
- [ ] `N's PP Up` plays, offered only for a Benched Pokémon whose name
      starts with "N's"
- [ ] `Wondrous Patch` plays, offered only for a Psychic Benched Pokémon,
      and only a Psychic Basic Energy
