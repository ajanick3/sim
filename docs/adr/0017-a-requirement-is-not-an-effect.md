# A requirement is a field of its own, not an effect

**Status:** Accepted — 2026-09-06

`Ultra Ball` may be played only by discarding 2 other cards from hand, and
`Special Red Card` only while the opponent holds 3 or fewer Prize cards. Both
lines could have been written as the first step of the card's effect, which
was the live alternative and the cheaper change. It was rejected because an
effect runs *after* the card is played, and both of these lines decide
whether the card may be played at all. A card whose demand is unmet must not
appear in `legal_actions` — ADR 0003 makes that list the whole of what a
player may do, so a card offered and then found unplayable would be the
engine refusing its own offer. `Trainer` therefore carries
`requirement: Option<Requirement>` beside its effect, and `legal_actions`
reads it.

## Consequences

The two requirements in the pool differ in one way that the value records.
`OpponentPrizesAtMost` is read from the board and costs nothing, so a card
carrying it resolves the moment it is played. `DiscardOtherCardsFromHand` is
a cost, and a cost is a choice: which two cards. It opens `Phase::Paying`,
which names the Trainer already played rather than a copy of its effect, and
the effect is read back from that card once the last payment lands. A phase
that names a card is how a mid-play continuation is held without making
`Phase` carry an effect of its own.

The import table now returns a requirement and an effect together, under one
match on the printed name. A card's whole behaviour is one fact, and a second
table keyed by the same name would let a card gain an effect in one place and
lose its requirement in the other.
