# Reattaching after its own discard

Type: task
Status: resolved

`Boomerang Energy`: *"As long as this card is attached to a Pokémon,
it provides {C} Energy. If this card is discarded by an effect of an
attack used by the Pokémon this card is attached to, attach this card
from your discard pile to that Pokémon after attacking."* 7 slots.

Needs the engine to know *why* a card left play — nothing tracks that
today; a discard is a discard. Likely needs a narrow fact recorded
only for this shape (the card that was just discarded, and whether an
attack's own effect caused it) rather than a general "discard reason"
mechanism the rest of the engine has no other use for.

- [x] Discarding this Energy as part of the carrying Pokémon's own
      attack effect (a cost, a self-discard effect) returns it to
      that Pokémon after the attack resolves
- [x] Discarding it any other way (a Trainer, an opponent's effect,
      Knockout) does not return it
- [x] `Boomerang Energy` plays

Blocked by: 01

## Resolution

No general "why was this discarded" mechanism was needed — only one
site in the whole engine discards the attacker's own attached Energy
as part of an attack's own effect,
`AttackEffect::DiscardsOwnEnergyThenDamagesChosenBenched`. Read
there directly: after the discard loop finishes, any discarded card
carrying `EnergyEffect::ReattachesAfterOwnDiscardByAttackEffect`
moves straight back from discard to the attacker's own `attached`.
A plain discard elsewhere (a Trainer, an opponent's effect, a
retreat cost, Knockout) never touches this code path at all, so
"any other way" needed no separate check either — the same shape
ticket 02's own acceptance criterion took.

Admits `Boomerang Energy`. Coverage moves from 682 to 683.
