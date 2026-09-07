# A hand discard can chain into the other player's own

**Status:** Accepted — 2026-09-07

## Context

`Hand Trimmer` reads: *"Each player discards cards from their hand
until they have 5 cards in their hand. Your opponent discards first."*
`Phase::DiscardingFromHand` already runs one player's discard down to a
size (`Xerosic's Machinations`) or lets one player choose from another's
hand (`Eri`); nothing built runs the same discard twice, once per
player, in a fixed order.

Two shapes were live. First: a bespoke phase pair naming both players'
discards up front, the way `Janine's Secret Art` names both search
targets before either search runs. Second: `Phase::DiscardingFromHand`
grows a `then: Option<DiscardFollowUp>` field, the same shape
`Phase::Promoting`'s `then: Option<PromoteFollowUp>` already carries —
read once the current step ends, naming what runs next.

## Decision

`Phase::DiscardingFromHand` carries `then`. `TrainerEffect::BothDiscardDownTo`
opens the opponent's discard with
`then: Some(DiscardFollowUp::AlsoDiscardOwnHandDownTo(target))`;
`Action::FinishDiscardingFromHand` reads it once that discard ends, and
opens a second, unchained `DiscardingFromHand` for the other player.
The size to discard to is computed fresh when each half opens, the same
way `OpponentDiscardsDownTo` already computes it — not carried through
from the first half.

A bespoke phase pair was not needed: unlike `Janine's Secret Art`'s two
searches, which need both targets fixed before either search can read
them, `Hand Trimmer`'s two discards are independent of each other.
Nothing about the second discard depends on what the first one did.

## Consequences

`Phase::DiscardingFromHand` gained one field; every match on it already
used `..`, so only its four construction sites needed the new field. A
future card chaining a third step would extend `DiscardFollowUp` rather
than add another phase.
