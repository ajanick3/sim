# Xerosic's Machinations and Eri

Type: task
Status: resolved

`Xerosic's Machinations`: *"Your opponent discards cards from their hand
until they have 3 cards in their hand."* `Eri`: *"Your opponent reveals
their hand, and you discard up to 2 Item cards you find there."* 5 and
20 slots.

Nothing built reaches into a zone the opponent controls. Both cards do,
by two different agents: `Xerosic's Machinations` has the opponent choose
what they lose, down to a count; `Eri` has the player who played it choose
what to take from the opponent's hand, filtered to Item cards.
`Phase::Deciding` already carries a `chooser` distinct from whoever played
the card — check whether setting it to the opponent, over the opponent's
own zone, is already enough for the first, before adding anything new for
the second.

- [x] A card can discard from the opponent's hand, the opponent's own
      choice, down to a count
- [x] A card can discard from the opponent's hand, the player's choice,
      filtered by kind
- [x] `Xerosic's Machinations` plays
- [x] `Eri` plays, offering only the opponent's Item cards

## Resolution

`Phase::DiscardingFromHand { chooser, of, filter, remaining }` — the same
shape `DiscardingOpponentEnergy { chooser, of }` already established for
attachments, now over a hand. `chooser == of` is `Xerosic's Machinations`;
`chooser != of` is `Eri`. One phase and one action pair serves both, since
the two fields already tell them apart.
[ADR 0031](../../../docs/adr/0031-discarding-from-a-hand-names-whose-hand.md)
records why `Phase::Deciding` was not extended instead: every `Deciding`
phase assumes its chooser and the zone's owner are the same player, and
`Eri` is the first card to break that.

`TrainerEffect::OpponentDiscardsDownTo` is also the first effect whose
limit is computed at play time — `hand_len - target` — rather than a
number the card itself prints.

Coverage went 453 → 459 (6 prints), and the field went 1633 → 1658
playable slots of 3660 — 45.3%, the biggest single jump in this
milestone.
