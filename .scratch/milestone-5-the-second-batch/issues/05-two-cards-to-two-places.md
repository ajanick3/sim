# A search for one of each of several cards

Type: task
Status: resolved

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

- [x] A search can hold a sequence of slots, each with its own filter
- [x] A search can send its cards to more than one destination
- [x] A constraint between two choices is expressible, and is a value
- [x] `Hilda`, `Dawn`, and `Crispin` all play

## Where this stands

All three parts are done and merged.

**Part one** put the printed stage on `Pokemon` — the artifact carries it as
its own field, 1437 Basic, 842 Stage 1, 306 Stage 2 — and added the filters
`EvolutionPokemon`, `PokemonOfStage`, and `BasicEnergy`. It also fixed a bug
found on the way: a deck search did not shuffle the deck, though every card
that searches prints the shuffle and the player has seen the whole order.

**Part two** made a search a sequence of slots. `TrainerEffect::Decide`
carries `Vec<Slot>`, and `Phase::Deciding` carries the Trainer's `CardId` and
a `step` — the continuation pattern `Phase::Paying` established, so the next
slot is read back from the card rather than copied into the phase. Every card
built before is a search of one slot, so there is one path and not two.
`Hilda` and `Dawn` play.

ADR 0012 was upheld along the way: a slot whose limit runs out does not move
the search on by itself, because the choice to stop is the player's.
`FinishDeciding` ends a slot and the search goes to the next one there. A
first attempt advanced automatically, and `tests/trainers.rs` caught it.

**Part three** built `Crispin`. `Destination` grew `Attach`, and
`Action::TakeCardOnto { card, target }` chooses the card and the Pokémon it
attaches to together — the same shape `MoveEnergy` already used, rather than
a second phase asking where once the card is already picked. `Slot` grew
`excludes_type_of_previous: bool`, and `Phase::Deciding` grew
`previous: Option<CardId>` that survives every slot transition: only the
flag is a static fact of the card, the value it compares against is state.
[ADR 0018](../../../docs/adr/0018-a-slot-can-read-what-a-search-already-took.md)
records both decisions.

Coverage went 388 → 392 (4 prints), and the field went 1281 → 1348 playable
slots of 3660 — 36.8%.

The ticket's four things `Phase::Deciding` had never done are now all built:
a sequence of slots, more than one destination, a constraint between two
choices, and attaching directly from a search.
