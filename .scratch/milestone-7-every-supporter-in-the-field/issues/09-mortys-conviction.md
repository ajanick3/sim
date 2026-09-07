# Morty's Conviction

Type: task
Status: ready-for-agent

*"You can use this card only if you discard another card from your hand.
Draw a card for each of your opponent's Benched Pokémon."* 1 slot.

The cost is `Ultra Ball`'s shape exactly: discard a chosen card from hand
to pay, `Phase::Paying` already built. The draw count is new only in
where it is read from — the size of the *opponent's* Bench, not a fixed
number.

- [ ] A draw effect can count the opponent's Benched Pokémon, rather than
      naming a fixed number
- [ ] `Morty's Conviction` plays, and cannot be played holding nothing
      else to discard
