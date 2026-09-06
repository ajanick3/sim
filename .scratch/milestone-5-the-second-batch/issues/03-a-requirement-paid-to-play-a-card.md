# A requirement paid to play a card

Type: task
Status: ready-for-agent

`Ultra Ball` may be played only by discarding 2 other cards from hand, and
`Special Red Card` only while the opponent holds 3 Prizes or fewer. 196 slots
between them.

Milestone 3 recorded the shape of the question and did not answer it: a
requirement is not an effect. It gates whether the card may be played at all,
which is `legal_actions`, and `Ultra Ball`'s is also a choice of which two
cards — a cost, paid on play, not a condition read from the board.

- [ ] A card can carry a requirement `legal_actions` reads
- [ ] A requirement that costs the player something is paid as a phase
- [ ] `Ultra Ball` cannot be played holding nothing else to discard
- [ ] `Special Red Card` is offered only when the opponent's Prizes allow
