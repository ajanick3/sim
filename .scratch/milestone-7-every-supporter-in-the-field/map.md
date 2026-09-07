# Map: every Supporter in the field

## Destination

All seventeen Supporters play. Ordered so each ticket spends the least,
given what the last one built.

## Notes

| # | Card(s)                                  | Slots | Needs |
| - | ------------------------------------------ | ----- | ------- |
| 01 | Lana's Aid                                 | 6     | A filter for "a Pokémon without a Rule Box, or a Basic Energy" — close to one already built, not quite it |
| 02 | Rust Syndicate Grunt                       | 3     | Nothing new — a Requirement and an effect, both already built |
| 03 | N's Plan                                   | 3     | Moving Energy between two specific zones (Bench → Active), more than one at a time |
| 04 | Pokémon Center Lady                        | 3     | Healing a *chosen* Pokémon, not the fixed Active |
| 05 | Rosa's Encouragement                       | 9     | A Requirement comparing Prizes to the opponent's; a target filtered to Stage 2 |
| 06 | AZ's Tranquility, Surfer                   | 2     | A follow-up after a switch, conditioned on what got displaced |
| 07 | Black Belt's Training, Gladion's Final Battle | 13 | A bonus that lasts only this turn, against a restricted target |
| 08 | Kieran                                      | 4     | A choice between two named effects |
| 09 | Morty's Conviction                          | 1     | A self-paid cost already built, and a count read from the opponent's Bench |
| 10 | Xerosic's Machinations, Eri                 | 25    | The opponent's own hand — read, and chosen from, by either player |
| 11 | Brock's Scouting                            | 8     | Two filters in one search, each with its own count |
| 12 | Wally's Compassion                          | 3     | Heal to full; a bulk move of every attachment to hand, conditioned on the heal |
| 13 | Janine's Secret Art                         | 2     | A search-and-attach repeated per chosen target, then a condition applied outside an attack |
| 14 | Briar                                       | 1     | A Prize taken outside the ordinary knockout count, conditioned on this turn's attacker |

Tickets 01–04 need nothing beyond what Milestones 5 and 6 already built —
they are proof the rest of the milestone is new ground, not that the
easy ones were miscounted.

## Decisions so far

Nothing resolved yet.

## Fog

- Whether "the opponent's own hand" is read through the existing
  `Zone`/`Deciding` machinery with `chooser` set to the opponent, or needs
  a phase of its own. `Eri`'s chooser is the player who played it;
  `Xerosic's Machinations`'s chooser is the opponent themselves, over
  their own hand — the same zone, two different choosers, one existing
  concept (`Phase::Deciding` already carries `chooser` distinct from the
  player who played the card) that may already be enough.
- Whether a this-turn bonus is a field on `GameState` cleared at
  `begin_turn`, or a `Limit`-shaped value spent once. `Black Belt's
  Training`, `Gladion's Final Battle`, and `Kieran` all need the same
  shape; build it once, on the first of the three, and let the other two
  prove it.
- Whether "a choice between two named effects" is a `TrainerEffect`
  variant carrying two others, or a phase that reads the card's own
  definition twice. `Kieran` is the only card that needs it today.
- **`Brock's Scouting`'s two-count search needed nothing new.** Taking
  from an earlier slot in a sequence never reduces what a later slot
  allows, since each slot's own `remaining` is set fresh when it opens —
  "two independent counts" and "a sequence of slots" reach the same final
  hand, they only differ in the order choices are presented. `Brock's
  Scouting` plays as an ordinary two-slot `Decide`, the same shape `Dawn`
  already is.
