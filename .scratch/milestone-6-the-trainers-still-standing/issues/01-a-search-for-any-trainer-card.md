# A search for any Trainer card

Type: task
Status: ready-for-agent

`Team Rocket's Petrel`: *"Search your deck for a Trainer card, reveal it,
and put it into your hand. Then, shuffle your deck."* 33 slots, the
most-played unbuilt Trainer that is not already ruled out of scope.

Every primitive this card needs is built: a search, a filter, a
destination, a shuffle when it ends. `CardFilter` has no variant that
matches a Trainer, only Pokémon and Energy. That is the whole of this
ticket.

- [ ] `CardFilter` can match any Trainer card
- [ ] `Team Rocket's Petrel` plays, and the decks are measured after
