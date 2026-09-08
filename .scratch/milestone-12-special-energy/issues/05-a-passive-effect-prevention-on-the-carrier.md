# A passive effect-prevention on the carrier

Type: task
Status: resolved

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

- [x] An opponent's attack still damages the carrier normally
- [x] Every other effect that same attack would apply to the carrier
      (a Special Condition, a stat change, anything past damage) does
      not apply
- [x] An effect already in place before this Energy attached is not
      retroactively removed
- [x] `Mist Energy` plays

Blocked by: 01

## Resolution

Not a central interception point — the fog note's second option won.
`GameState::attack_effects_on_it_prevented(id)` is a small helper
(does any attached Energy carry
`EnergyEffect::PreventsAttackEffectsOnCarrier`), and each grant site
that applies an effect directly to `defender` now checks it first,
gated with the site's existing logic rather than routed through one
shared chokepoint: `attack.inflicts`, `InflictsCondition`,
`CoinFlipInflicts` (the coin still flips; only the infliction is
skipped — the flip itself is not "done to" the carrier),
`DefenderCannotRetreatNextTurn`, and `DefenderDealsLessDamageNextTurn`.

Left out, deliberately, and worth flagging for whoever reads this
next: `AttackEffect::DiscardsDefendersTools` (`Seaking`'s `Peck Off`)
runs before `attack()`'s own damage calculation, at a different site
than the four gated here, and was not touched — a real (if narrow)
gap where Mist Energy would not protect against a Tool being
discarded. `OpponentCannotPlayItemsNextTurn` was deliberately left
ungated too: it restricts what the *player* may do next turn, not an
effect "done to" the Pokémon itself, even though it happens to be
keyed by the defender's id today. Any new `AttackEffect` that writes
directly onto a Pokémon should check this helper going forward, the
same way any new passive Ability must remember to check the ones
`Watchful Eye`/`Damp` already set.

Admits `Mist Energy`. Coverage moves from 681 to 682.
