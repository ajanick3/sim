# Map: choices and conditions

## Destination

Every choice Milestone 1 made for the player becomes the player's, and the
between-turn step does its work.

## Notes

Milestone 1 left each shortcut recorded in its own ticket Answer. The frontier
starts at ticket 01.

## Decisions so far

Ticket 01 resolved 2026-09-05: setup became four phases and every choice in it is the player's; details under [the ticket's Answer](issues/01-setup-as-a-phase.md).
Ticket 02 resolved 2026-09-05: an attack cost names Energy types, and the player chooses which Energy a retreat discards; details under [the ticket's Answer](issues/02-energy-types-in-a-cost.md).
Ticket 03 resolved 2026-09-05: the five Special Conditions and the Pokémon Checkup work, and a player orders their own between-turn effects; details under [the ticket's Answer](issues/03-special-conditions-and-checkup.md).
Ticket 05 resolved 2026-09-05: a player reads the game through an owned masked view, at 23% of a decision; details under [the ticket's Answer](issues/05-masked-observation-views.md).

## Fog

- Whether a Benched attacker applies Weakness and Resistance is unanswered, and
  it blocks nothing until a card can attack from the Bench.
- Nothing removes a condition except the checkup, a retreat, and a knockout. A
  Trainer or an Ability that heals one has nowhere to hook in yet.
- A mulligan reveals the hand, so that hand is public. No view records what was
  revealed.
- The card data bridge has no format. Ticket 06 opens the question.
