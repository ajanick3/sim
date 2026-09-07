# A per-count attack computes its base before `damage_dealt` runs

**Status:** Accepted — 2026-09-08

## Context

`N's Reshiram`, `N's Darmanitan`, `Dudunsparce ex`, `Passimian`, and
`Paldean Tauros` each print "N damage for each X" — the printed damage
field itself is a multiplier marker (`"20×"`), not a flat number. Real
play applies Weakness, Resistance, and any Tool or Stadium bonus to the
*total*, the same as a plain attack's damage — the count is not a
bonus added after those steps (the way `Recoil` and every attacker-side
Tool bonus, ADR 0043, add to a number already through the damage
order); it replaces the base `damage_dealt` starts from.

## Decision

`AttackEffect::DamagePerCount(Count, u32)` is read in `attack()`
*before* `damage_dealt` is called, substituting for `attack.base_damage`
entirely — `count_for_attack` reads whichever board fact `Count` names,
multiplies by the per-unit value, and that product is the `base`
`damage_dealt` receives. `resolve_attack_effect`'s own arm for this
variant is a no-op: by the time it would run, the count has already
been spent.

`read_attack`'s damage parsing grows a case for a `"N×"` string,
accepted only when `known_attack` already matched an effect for this
attack — the digits are read only to confirm the print agrees with
what `known_attack` supplies, not as the source of truth. A `"×"` field
with no matching effect still refuses on `DamageIsNotANumber`, the same
as before.

`Count` starts with five variants, one per counted fact the sample
needs: the attacker's own damage counters, the opponent's Basic Energy
in discard, the opponent's Pokémon ex in play, the player's own Basic
Pokémon in play, and the player's own damaged Pokémon sharing a name
prefix (`Paldean Tauros`'s "Pokémon that has 'Tauros' in its name").

## Consequences

Two attack-effect shapes now read "before" vs. "after" `damage_dealt`:
`DamagePerCount` computes the base itself; everything else (`Recoil`,
and whatever ADR 0043's attacker-Tool-bonus step covers) adds to a
number the damage order already finished with. A future attack whose
count needs a board fact not in `Count` yet extends the enum; one whose
multiplier should stack with a flat base (nothing in the sample prints
that shape) would need `DamagePerCount` to carry both, not yet built.
