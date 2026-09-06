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
| The actions applied        | Yes                 |
| The log of what happened   | Yes, as prose only  |
| Hand, Discard, Library     | Yes, as `Zone`      |
| Stadium in play            | Yes                 |
| Lost Zone                  | Not in this format  |
| Evolved this turn          | Yes, per Pokémon    |
| Supporter, Stadium a turn  | Yes, per player     |
| Ability used this turn     | **No**              |
| Once per game              | **No**              |

`GameState.log` holds prose for a reader. It is not the record: it cannot be
replayed, and nothing reads it back.

## Decisions so far

Ticket 03 resolved 2026-09-06: the Stadium in play has a slot, with rule 59 alongside; the Lost Zone was built and withdrawn, because no card in this format uses one; details under [the ticket's Answer](issues/03-the-missing-zones.md).
Ticket 02 resolved 2026-09-06: undo replays the log without its last entry, because a shuffle has no inverse the engine can compute; details under [the ticket's Answer](issues/02-undo-one-action.md).
Ticket 01 resolved 2026-09-06: the state records every action applied to it, and a game replays from its cards, its seed, and that log; details under [the ticket's Answer](issues/01-the-action-log.md).

## Fog

- A survey is not evidence about this pool. The Lost Zone was taken from
  `ptcg-sim`, built, and withdrawn within the day: no card of marks H, I, or
  J mentions one. Check the artifact before taking a mechanic from anywhere
  else.
- `twinleaf.gg` is another simulator worth reading, raised 2026-09-06. The
  site answers, and no public repository turned up in a first search, so a
  survey of it would have to read the client it serves rather than a source
  tree.

- Carried from milestone 3: nothing records that a Pokémon was knocked out
  during the opponent's last turn, which `Unfair Stamp` needs. It is the same
  class of problem as the rest of this milestone.
- Carried from milestone 3: `known_trainer_effect` matches by printed name.
  Safe today, checked; a name with two different effects would need matching
  by print id.
