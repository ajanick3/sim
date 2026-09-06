# Map: the record

## Destination

A game can be replayed from its seed and its actions, undone a step, and
saved. The zones and the limits a card needs are there to be recorded
against.

## Notes

What the engine records today, and what it does not:

| Fact                       | Recorded            |
| -------------------------- | ------------------- |
| The seed                   | Yes, in the state   |
| The actions applied        | **No**              |
| The log of what happened   | Yes, as prose only  |
| Hand, Discard, Library     | Yes, as `Zone`      |
| Lost Zone, Stadium in play | **No**              |
| Evolved this turn          | Yes, per Pokémon    |
| Supporter, Stadium a turn  | Yes, per player     |
| Ability used this turn     | **No**              |
| Once per game              | **No**              |

`GameState.log` holds prose for a reader. It is not the record: it cannot be
replayed, and nothing reads it back.

## Decisions so far

Ticket 01 resolved 2026-09-06: the state records every action applied to it, and a game replays from its cards, its seed, and that log; details under [the ticket's Answer](issues/01-the-action-log.md).

## Fog

- Whether the log stores every action or only the ones that changed the
  state. A refused action changes nothing and is not worth keeping; the
  engine already refuses those before they apply.
- Whether undo re-runs the log from the start or reverses one action. The
  first is simple and correct and costs a replay; the second is quick and
  needs every action to know its own inverse, which a shuffle does not.
- Carried from milestone 3: nothing records that a Pokémon was knocked out
  during the opponent's last turn, which `Unfair Stamp` needs. It is the same
  class of problem as the rest of this milestone.
- Carried from milestone 3: `known_trainer_effect` matches by printed name.
  Safe today, checked; a name with two different effects would need matching
  by print id.
