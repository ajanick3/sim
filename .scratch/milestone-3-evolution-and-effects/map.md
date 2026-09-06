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

## Decisions so far (continued)

Ticket 03 resolved 2026-09-06: `CardDef::Trainer` and eight real Trainers
wired to the artifact by name; coverage moved to 368/3051 (12.1%), and both
committed decks moved further, since Trainers are their largest blocker;
details under [the ticket's Answer](issues/03-the-first-trainer-effects.md).

## Fog

- Nothing records that a Pokémon was knocked out during the opponent's last
  turn, which `Unfair Stamp` needs.
- `known_trainer_effect` matches by printed name, checked safe today because
  each of the eight has one distinct effect text across every printing. A
  card with the same name and a genuinely different effect would need
  matching by print id instead; nothing here does that yet.
