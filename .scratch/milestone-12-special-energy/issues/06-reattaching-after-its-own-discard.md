# Reattaching after its own discard

Type: task
Status: open

`Boomerang Energy`: *"As long as this card is attached to a Pokémon,
it provides {C} Energy. If this card is discarded by an effect of an
attack used by the Pokémon this card is attached to, attach this card
from your discard pile to that Pokémon after attacking."* 7 slots.

Needs the engine to know *why* a card left play — nothing tracks that
today; a discard is a discard. Likely needs a narrow fact recorded
only for this shape (the card that was just discarded, and whether an
attack's own effect caused it) rather than a general "discard reason"
mechanism the rest of the engine has no other use for.

- [ ] Discarding this Energy as part of the carrying Pokémon's own
      attack effect (a cost, a self-discard effect) returns it to
      that Pokémon after the attack resolves
- [ ] Discarding it any other way (a Trainer, an opponent's effect,
      Knockout) does not return it
- [ ] `Boomerang Energy` plays

Blocked by: 01
