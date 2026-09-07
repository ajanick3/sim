# A requirement read from last turn

Type: task
Status: resolved

`Unfair Stamp`: *"You can use this card only if any of your Pokémon were
Knocked Out during your opponent's last turn. Each player shuffles their
hand into their deck. Then, you draw 5 cards, and your opponent draws 2
cards."* 25 slots.

Two things: a `Requirement` read from history rather than the board as it
stands right now, and a shuffle-then-draw where the two players draw
different amounts — `BothShuffleHandThenDraw` today draws the same count
for both.

Whether "a Pokémon of mine was Knocked Out during the opponent's last
turn" is a new field on `GameState`, cleared at the start of a turn, or
answerable from `history: Vec<Action>` without a new field, is this
ticket's first question — check before building either. Milestone 7 left
the identical question open for `Fezandipiti ex`'s Ability; whichever
answer this ticket settles on, record the reasoning so that milestone does
not have to ask it twice.

- [x] "A Pokémon of mine was Knocked Out during the opponent's last turn"
      can be asked, and answers correctly across a turn boundary
- [x] Both players shuffle their hand into their deck, and draw different
      counts
- [x] `Unfair Stamp` plays, and cannot be played without a Knockout to
      point to

## Resolution

`GameState` grows `knocked_out_last_turn: [bool; 2]` — a field, not
something derived from `history: Vec<Action>`. No `Action` records a
knockout as its own event, so answering from the log would mean
re-simulating everything since the last turn boundary on every ask, a
search rather than a read; [ADR 0024](../../../docs/adr/0024-a-requirement-can-read-history.md)
records the reasoning, which resolves this milestone's open fog question
before `Fezandipiti ex`'s Ability has to ask it too. The field is set the
moment a knockout happens, and cleared once — for the player it belongs
to, when *their own* turn ends, not when it begins — which is what leaves
it true for exactly one turn and false again one full cycle later.

`BothShuffleHandThenDraw` grew `you` and `opponent` in place of one shared
`count`, a rename with a single caller (`Judge`, unaffected since it wants
the same number for both).

Coverage went 408 → 409 (1 print), and the field went 1527 → 1552
playable slots of 3660 — 42.4%.
