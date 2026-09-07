# Hand Trimmer

Type: task
Status: resolved

*"Each player discards cards from their hand until they have 5 cards in
their hand. Your opponent discards first."*

New: `Phase::DiscardingFromHand` grows a `then: Option<DiscardFollowUp>`
field, the same shape `Phase::Promoting`'s `then` already carries, so
the opponent's discard can chain into the player's own once it ends.

- [x] The opponent discards down to 5 first, their own choice
- [x] The player then discards their own hand down to 5

## Resolution

One new field, one new `TrainerEffect::BothDiscardDownTo`, one new
`DiscardFollowUp` enum. Recorded in [ADR 0035](../../../docs/adr/0035-a-hand-discard-can-chain-into-the-other-hand.md).

This ticket's commit also carries the map.md correction moving `Tool
Scrapper` to the Tools milestone: it discards an *attached* Tool, and
nothing built attaches a Tool yet — `PlayTrainer` sends every
`TrainerKind::Tool` straight to discard, same as an Item.

Coverage: `admitted` 476 -> 477 (1 print); `trainers` (refused)
315 -> 314.
