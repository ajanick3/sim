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

Ticket 05 resolved 2026-09-06: a card definition carries the print id it came from, and the text interface names it; details under [the ticket's Answer](issues/05-card-identity.md).
Ticket 01 resolved 2026-09-06: a Pokémon evolves under rules 19-22, and `is_basic_pokemon` was fixed at its one definition so an evolution can no longer be placed directly; details under [the ticket's Answer](issues/01-evolution.md).
Ticket 07 resolved 2026-09-06: the importer matches the published card shape, and a decklist line is checked against the card its number names; details under [the ticket's Answer](issues/07-match-the-published-card-shape.md).
Ticket 06 resolved 2026-09-06: every kind of card is named, and a refusal says which kind it refused; details under [the ticket's Answer](issues/06-the-missing-card-types.md).
Ticket 04 resolved 2026-09-06: a knockout takes what the card is worth, read from the card's name because the data's suffix field cannot be trusted; details under [the ticket's Answer](issues/04-prize-values.md).
Ticket 02 resolved 2026-09-06: nine primitives cover the Trainers the committed decks play, and an effect is a value the engine executes; details under [the ticket's Answer](issues/02-the-effect-vocabulary.md).

## Progress on ticket 03

The resolution machinery is built ahead of any card: `Phase::Deciding`
covers six of the eight primitives (a move between zones, filtered), and
`Phase::Promoting` was generalized with a `chooser` distinct from `of` to
cover the other Trainer that needs a phase at all (switching an opponent's
Active). Recorded as [ADR 0012](../../docs/adr/0012-trainer-resolution-shares-two-phases.md).
Still to build: `CardDef::Trainer`, the eight cards themselves, and the
`CardFilter` variants beyond `AnyPokemon` they need.

## Fog

- Nothing records that a Pokémon was knocked out during the opponent's last
  turn, which `Unfair Stamp` needs.
