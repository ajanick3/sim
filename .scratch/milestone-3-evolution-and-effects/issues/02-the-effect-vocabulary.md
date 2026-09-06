# The effect vocabulary

Type: research
Status: resolved

Decide the first effect primitives by counting, not by reading the pool.

Rank the Trainers in `decks/` by how many copies they account for, then read
those cards' text and name the primitives they share. The answer is a short
list, because the same staples appear in every deck.

- [x] A ranked table of the Trainers the committed decks play
- [x] The primitives those cards need, named and defined
- [x] A recorded decision on how an effect is written as a value

## Answer

Resolved 2026-09-06 on branch `feat/the-effect-vocabulary`.

The ranked table and the primitives are in
[the effect vocabulary](../../../docs/architecture/effects.md). The decision is
[ADR 0009](../../../docs/adr/0009-an-effect-is-a-value-the-engine-executes.md):
an effect is a value the engine executes, never text read at run time.

Nineteen Trainers, 57 copies, across the two committed decks. Nine primitives
cover them, and one — move cards, from a zone to a zone by a filter — covers
fourteen of the nineteen on its own. That is the answer the counting was for:
the first effect worth building is a search, not a special case.

Three findings that change what ticket 03 can do:

- **A card filter is a value of its own**, and two of its terms need data the
  engine does not hold. `Poké Pad` and `Gwynn`, the two most-played cards
  between them, both read "doesn't have a Rule Box", which is the same fact as
  what a knockout is worth. Ticket 04 supplies it. A stage filter waits on
  ticket 01.
- **A requirement is not an effect.** `Ultra Ball` and `Special Red Card` gate
  on the state before they may be played, so the check belongs in
  `legal_actions`. `Unfair Stamp` needs the state to remember a knockout from
  the opponent's last turn, which nothing records today.
- **`Team Rocket's Watchtower` is out of reach**, and not for want of a
  primitive. It is a continuous rule that turns Abilities off, and Abilities do
  not exist.
