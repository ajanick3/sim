# A third basic unaided

Type: task
Status: resolved
Blocked by: 01, 02

Nick writes a third Basic Pokémon into `src/cards.rs` without help. This is
the milestone's real purpose: the engine exists to be learned from.

- [x] The card exists, with an attack the engine can already express
- [x] A test covers what the card does

## Answer

Resolved 2026-09-05 on branch `feat/aquabear-tests`.

Nick wrote Aquabear: a Water Basic, 70 HP, weak to Lightning, retreat 1, with
Bubblebeam at Water and Colorless for 30. He added a Water Energy for it and
put four copies of each in the starter decklist, so the card reaches play
rather than sitting unreachable in the database.

The engine needed no work. A check confirmed it already expresses every part of
the card: the named Water entry refuses a Fire, the Colorless entry takes the
spare, and the damage lands as printed.

Claude wrote the tests in `tests/cards.rs`, not Nick, so the unaided half of
this ticket covers the card and not its tests. Each test was mutation-checked:
changing Bubblebeam's cost and damage fails both behaviour tests.
