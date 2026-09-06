# Map: evolution and the first effects

## Destination

A real deck plays. Not every card in it, but its evolution lines and the
Trainer staples it is built on.

## Notes

Where the two committed decks stand on the day this effort opened, counted by
each card's first blocker out of 60:

| Blocker      | brent-tonisson | mateusz-laszkiewicz |
| ------------ | -------------- | ------------------- |
| Trainer      | 32             | 31                  |
| Evolution    | 10             | 11                  |
| Attack text  | 3              | 8                    |
| Ability      | 3              | 5                   |
| Special Energy | 0            | 4                   |
| Plays today  | 12             | 1                   |

A card is counted by its first blocker, so an evolution that also has an
ability sits in the evolution row. Ticket 01 will not unlock every card in it.

## Decisions so far

Ticket 06 resolved 2026-09-06: every kind of card is named, and a refusal says which kind it refused; details under [the ticket's Answer](issues/06-the-missing-card-types.md).
Ticket 04 resolved 2026-09-06: a knockout takes what the card is worth, read from the card's name because the data's suffix field cannot be trusted; details under [the ticket's Answer](issues/04-prize-values.md).
Ticket 02 resolved 2026-09-06: nine primitives cover the Trainers the committed decks play, and an effect is a value the engine executes; details under [the ticket's Answer](issues/02-the-effect-vocabulary.md).

## Fog

- Nothing records that a Pokémon was knocked out during the opponent's last
  turn, which `Unfair Stamp` needs.
- Whether a Trainer needs its own place in the state, beyond being played and
  discarded.
