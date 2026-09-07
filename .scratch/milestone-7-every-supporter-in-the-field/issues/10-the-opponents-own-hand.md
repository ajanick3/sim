# Xerosic's Machinations and Eri

Type: task
Status: ready-for-agent

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

- [ ] A card can discard from the opponent's hand, the opponent's own
      choice, down to a count
- [ ] A card can discard from the opponent's hand, the player's choice,
      filtered by kind
- [ ] `Xerosic's Machinations` plays
- [ ] `Eri` plays, offering only the opponent's Item cards
