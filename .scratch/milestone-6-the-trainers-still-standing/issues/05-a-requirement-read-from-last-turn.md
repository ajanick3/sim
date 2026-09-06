# A requirement read from last turn

Type: task
Status: ready-for-agent

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

- [ ] "A Pokémon of mine was Knocked Out during the opponent's last turn"
      can be asked, and answers correctly across a turn boundary
- [ ] Both players shuffle their hand into their deck, and draw different
      counts
- [ ] `Unfair Stamp` plays, and cannot be played without a Knockout to
      point to
