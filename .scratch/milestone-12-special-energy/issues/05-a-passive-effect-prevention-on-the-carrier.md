# A passive effect-prevention on the carrier

Type: task
Status: open

`Mist Energy`: *"As long as this card is attached to a Pokémon, it
provides {C} Energy. Prevent all effects of attacks used by your
opponent's Pokémon done to the Pokémon this card is attached to.
(Existing effects are not removed. Damage is not an effect.)"* 21
slots.

Damage still lands; every other `AttackEffect` an opposing attack
would otherwise apply to the carrier does not. This is the same
shape the deferred `Flower Curtain`/`Spherical Shield` Bench-protection
Abilities need, narrowed to one Pokémon instead of a whole Bench —
worth checking whether solving it here also unblocks those, or
whether it stays a narrower, one-off read.

- [ ] An opponent's attack still damages the carrier normally
- [ ] Every other effect that same attack would apply to the carrier
      (a Special Condition, a stat change, anything past damage) does
      not apply
- [ ] An effect already in place before this Energy attached is not
      retroactively removed
- [ ] `Mist Energy` plays

Blocked by: 01
