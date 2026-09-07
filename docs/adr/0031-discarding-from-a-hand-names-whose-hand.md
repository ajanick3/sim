# Discarding from a hand names whose hand, distinct from who chooses

**Status:** Accepted — 2026-09-07

`Xerosic's Machinations` and `Eri` both discard from a hand, and neither
reads the player's own: the first has the opponent discard their own hand
down to a count, their own choice; the second has the player choose what
to take from the opponent's, filtered to Item cards. Nothing built before
either reaches into a zone the opponent controls at all —
`DiscardingOpponentEnergy` reaches into an opponent's *attachments*, and a
hand is not that.

`Phase::DiscardingFromHand { chooser, of, filter, remaining }` is the
same shape `DiscardingOpponentEnergy { chooser, of }` already
established, over a hand instead of a Pokémon's attachments: `chooser`
picks, `of` owns the zone, and the two are not assumed equal.
`Phase::Deciding` was not extended to cover this instead, because every
`Deciding` phase already assumes its `chooser` and the zone's owner are
the same player — true for every card built so far, and the assumption
Eri specifically breaks. Reusing it would have meant unpicking that
assumption everywhere `Deciding` reads a zone, for two cards that fit an
existing, smaller shape exactly as it stands.

`chooser == of` reads as `Xerosic's Machinations`: the opponent is both
who chooses and whose hand it is. `chooser != of` reads as `Eri`: the
player chooses, the opponent's hand is read. One phase, one action pair
— `Action::DiscardFromHand` and `Action::FinishDiscardingFromHand` —
serves both without a flag distinguishing them, since the two fields
already do.

## Consequences

`TrainerEffect::OpponentDiscardsDownTo(u32)` computes its `remaining` at
play time — `hand_len - target`, floored at zero — rather than printing a
fixed count the way every other `Decide`-shaped search does. It is the
first effect in the pool whose limit is not a number the card itself
names.
