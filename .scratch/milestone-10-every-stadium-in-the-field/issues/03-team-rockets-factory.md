# Team Rocket's Factory

Type: task
Status: resolved

*"Once during each player's turn, if they played a Supporter card that
has 'Team Rocket' in its name from their hand this turn, they may draw
2 cards."*

Follows [ADR 0049](../../../docs/adr/0049-a-stadiums-own-action-is-offered-not-dispatched.md)'s
shape: no new ADR needed. New per-player fact,
`played_a_team_rocket_supporter_this_turn`, set when `PlayTrainer` plays
a matching Supporter, cleared in `begin_turn` — the same "this turn"
lifetime `turn_bonus` already carries — and read alongside
`Limit::StadiumEffectUsed` the same way `Academy at Night` already is.

- [x] Offered only after a "Team Rocket" Supporter is played this turn
- [x] Once a turn
- [x] The fact does not survive into the next turn

## Resolution

One new per-player fact, one new `TrainerEffect`, one new `Action`.

Coverage: `admitted` 502 -> 504 (2 prints); `trainers` (refused)
289 -> 287.
