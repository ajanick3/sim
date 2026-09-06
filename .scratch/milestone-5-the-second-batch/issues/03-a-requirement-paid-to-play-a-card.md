# A requirement paid to play a card

Type: task
Status: resolved

`Ultra Ball` may be played only by discarding 2 other cards from hand, and
`Special Red Card` only while the opponent holds 3 Prizes or fewer. 196 slots
between them.

Milestone 3 recorded the shape of the question and did not answer it: a
requirement is not an effect. It gates whether the card may be played at all,
which is `legal_actions`, and `Ultra Ball`'s is also a choice of which two
cards — a cost, paid on play, not a condition read from the board.

- [x] A card can carry a requirement `legal_actions` reads
- [x] A requirement that costs the player something is paid as a phase
- [x] `Ultra Ball` cannot be played holding nothing else to discard
- [x] `Special Red Card` is offered only when the opponent's Prizes allow

## Resolution

`Trainer` carries `requirement: Option<Requirement>` beside its effect.
[ADR 0017](../../../docs/adr/0017-a-requirement-is-not-an-effect.md) records
why it did not become the first step of the effect.

`Phase::Paying` holds the cost. It names the Trainer already played rather
than a copy of its effect, and the effect is read back from that card once
the last payment lands — the cheapest form of a mid-play continuation, and
one that keeps `Phase` free of effects.

`Special Red Card` also needed a new effect to be admitted at all, since
ADR 0008 admits a card only when the engine runs all of it:
`OpponentHandToBottomThenDraw`. The hand is shuffled and put *under* the
deck, not shuffled into it, so what the opponent gave up is the last thing
they draw again.

Coverage went 375 → 380 (3 prints of `Ultra Ball`, 2 of `Special Red Card`),
and the field went 955 → 1151 playable slots of 3660 — 31.4%.
