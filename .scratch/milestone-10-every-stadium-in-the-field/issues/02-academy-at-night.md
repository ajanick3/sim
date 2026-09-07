# Academy at Night

Type: task
Status: resolved

*"Once during each player's turn, that player may put a card from
their hand on top of their deck."*

New: a Stadium's own once-a-turn action, offered directly in
`legal_actions`' Main-phase generation rather than dispatched through
`PlayTrainer` — the Stadium is already in play, so nothing is played to
trigger it. New `Limit::StadiumEffectUsed`, shared by whichever
Stadium's own action is in play. Recorded in
[ADR 0049](../../../docs/adr/0049-a-stadiums-own-action-is-offered-not-dispatched.md).

- [x] Offered once a turn, to either player, while this Stadium is in
      play
- [x] Spent for the turn after use, even with cards left in hand
- [x] Not offered without this Stadium in play

## Resolution

One new `Limit` variant, one new `TrainerEffect`, one new `Action`, one
new offer site in `legal_actions`.

Coverage: `admitted` 501 -> 502 (1 print); `trainers` (refused)
290 -> 289.
