# Rust Syndicate Grunt

Type: task
Status: ready-for-agent

*"You can use this card only if any of your Pokémon were Knocked Out
during your opponent's last turn. Discard an Energy from 1 of your
opponent's Pokémon."* 3 slots.

Both pieces are close to built. `Requirement::KnockedOutDuringOpponentsLastTurn`
is `Unfair Stamp`'s requirement, unchanged. `Phase::DiscardingOpponentEnergy`
already offers exactly the choice this card needs — but the only way into
it today is `Crushing Hammer`'s coin flip. This card opens the same phase
outright, no flip: a small new `TrainerEffect` variant, not a new phase.

- [ ] A Trainer can send the player straight into
      `Phase::DiscardingOpponentEnergy`, without a coin flip first
- [ ] `Rust Syndicate Grunt` plays, and the decks are measured after
