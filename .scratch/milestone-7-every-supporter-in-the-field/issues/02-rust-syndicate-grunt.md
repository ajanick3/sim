# Rust Syndicate Grunt

Type: task
Status: resolved

*"You can use this card only if any of your Pokémon were Knocked Out
during your opponent's last turn. Discard an Energy from 1 of your
opponent's Pokémon."* 3 slots.

Both pieces are close to built. `Requirement::KnockedOutDuringOpponentsLastTurn`
is `Unfair Stamp`'s requirement, unchanged. `Phase::DiscardingOpponentEnergy`
already offers exactly the choice this card needs — but the only way into
it today is `Crushing Hammer`'s coin flip. This card opens the same phase
outright, no flip: a small new `TrainerEffect` variant, not a new phase.

- [x] A Trainer can send the player straight into
      `Phase::DiscardingOpponentEnergy`, without a coin flip first
- [x] `Rust Syndicate Grunt` plays, and the decks are measured after

## Resolution

`TrainerEffect::DiscardOpponentEnergy` opens `Phase::DiscardingOpponentEnergy`
directly — the same phase `CoinFlipDiscardOpponentEnergy` opens on heads,
with no flip in front of it. `Requirement::KnockedOutDuringOpponentsLastTurn`
is `Unfair Stamp`'s requirement, unchanged.

Coverage went 416 → 418 (2 prints), and the field went 1595 → 1598
playable slots of 3660 — 43.7%.
