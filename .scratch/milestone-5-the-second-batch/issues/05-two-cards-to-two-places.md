# A search for one of each of several cards

Type: task
Status: ready-for-agent

Three cards search for two or three *different* cards, one of each. 152
slots between them.

- `Crispin` (67): up to 2 Basic Energy of **different types**; one goes to
  hand and the other is attached.
- `Hilda` (41): an Evolution Pokémon **and** an Energy card, both to hand.
- `Dawn` (44): a Basic, a Stage 1, and a Stage 2, all to hand.

`Decide` carries one filter, one limit, and one destination, so none of the
three fits it. Ticket 02 gave this ticket its `Hilda` line when it found the
shape; the filters each slot names are built already.

Four things `Phase::Deciding` has never had to do: hold a *sequence* of
slots rather than one repeated choice, send its cards to more than one
destination, hold a constraint between two choices, and attach directly from
a search.

- [ ] A search can hold a sequence of slots, each with its own filter
- [ ] A search can send its cards to more than one destination
- [ ] A constraint between two choices is expressible, and is a value
- [ ] `Hilda`, `Dawn`, and `Crispin` play
