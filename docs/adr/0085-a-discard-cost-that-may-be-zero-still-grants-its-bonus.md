# A discard cost that may be zero still grants its bonus

**Status:** Accepted — 2026-09-08

## Context

Three cards this session needed the player to choose which of their
own attached Energy to discard, rather than the engine picking or
discarding everything attached (the shape `DiscardsOwnEnergyThen-
DamagesChosenBenched` already took, for an attack that always
discards its attacker's *entire* attachment): `Zeraora`'s `Strong
Volt` (discard exactly 1), `Metagross`'s `Luster Blast` (discard
exactly 2), `Metagross`'s `Metallic Hammer` (may discard up to 3 of
one type for a flat bonus), and `Raging Bolt ex`'s `Bellowing
Thunder` (may discard any number of Basic Energy from *any* of the
player's own Pokémon, not only the attacker, for damage scaled to
the count).

The Rabsca/Slowking trivia exchange earlier in this session had
already surfaced the ruling that decided `Metallic Hammer`'s own
shape: choosing to attempt a "may discard up to N for a bonus"
effect grants the bonus outright, even with zero qualifying Energy
attached to discard. The cost is optional to attempt, not "discard N
or get nothing."

## Decision

Three distinct `AttackEffect` variants, not one generalized one — the
three cards' own texts differ on axes that don't collapse cleanly:
mandatory vs. optional, one Pokémon vs. the whole side, and (for the
optional shapes) a flat bonus vs. one scaled by count.

- `DiscardsFixedOwnEnergyChosen(u32)`: mandatory, the attacker only,
  any type. `Phase::ChoosingOwnEnergyToDiscardForAttack` offers every
  attached Energy card each pick, closing once `remaining` reaches
  zero. The attack's own printed cost already guarantees enough
  Energy is attached by the time this runs.
- `MayDiscardUpToOwnEnergyOfTypeForFlatBonusDamage(Type, max, bonus)`:
  optional, the attacker only, one type. `Phase::DecidingToDiscard-
  OwnEnergyForBonusDamage` is a yes/no gate — accepting grants
  `bonus` immediately, before any card is actually discarded, then
  moves to `ChoosingOwnEnergyToDiscardForBonusDamage` to pick up to
  `max`, free to stop early without losing the bonus already
  granted. Declining the initial yes/no grants nothing.
- `MayDiscardAnyOwnBasicEnergyForDamagePerCard(u32)`: optional, any
  of the player's own Pokémon, Basic Energy only, damage scaled to
  the count actually discarded. `Phase::DiscardingAnyBasicEnergyFor-
  DamagePerCard` offers every Basic Energy on any own Pokémon plus a
  "finish" action available from zero discards; the damage is
  computed only once the player stops, since the count is not known
  until then.

The third variant's own damage cannot go through the normal
`damage_dealt_with` pipeline — that call already ran and returned,
before `resolve_attack_effect`'s interactive phase even opens, since
the discard count is a mid-resolution player choice. Weakness and
Resistance are reapplied by hand to the discard-based portion at the
point it is added; a Tool bonus, a turn bonus, and `Cobalt Command`
are not — those already had their one chance, against a `base_damage`
this effect always reads as 0, before this phase existed to give
them anything to attach to.

## Consequences

A future card whose damage depends on a mid-resolution choice — not
only Energy discarded — inherits this same gap: any pipeline step
gated on `damage_dealt_with`'s own single call (a Tool, a turn bonus,
an Ability like `Cobalt Command`) will not reach it. Closing that
gap generally would mean restructuring `resolve_attack_effect` to
resolve its own interactive choices before the main damage
calculation runs, not only after — worth doing if a second such card
turns up, not speculatively for one.
