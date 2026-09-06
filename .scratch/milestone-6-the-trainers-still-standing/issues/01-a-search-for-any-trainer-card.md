# A search for any Trainer card

Type: task
Status: resolved

`Team Rocket's Petrel`: *"Search your deck for a Trainer card, reveal it,
and put it into your hand. Then, shuffle your deck."* 33 slots, the
most-played unbuilt Trainer that is not already ruled out of scope.

Every primitive this card needs is built: a search, a filter, a
destination, a shuffle when it ends. `CardFilter` has no variant that
matches a Trainer, only Pokémon and Energy. That is the whole of this
ticket.

- [x] `CardFilter` can match any Trainer card
- [x] `Team Rocket's Petrel` plays, and the decks are measured after

## Resolution

`CardFilter::AnyTrainer` matches any `CardDef::Trainer`, of any kind.
`Team Rocket's Petrel` plays as an ordinary one-slot search — nothing about
the search machinery needed to change.

Coverage went 394 → 397 (3 prints), and the field went 1388 → 1421
playable slots of 3660 — 38.8%.
