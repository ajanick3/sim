# Brave Bangle & Binding Mochi

Type: task
Status: resolved

*"If the Pokémon this card is attached to doesn't have a Rule Box, the
attacks it uses do 30 more damage to your opponent's Active Pokémon
ex..."* (`Brave Bangle`) and *"Attacks used by the Poisoned Pokémon this
card is attached to do 40 more damage to your opponent's Active
Pokémon..."* (`Binding Mochi`).

Both add damage on every attack, not "this turn" — a new lifetime
`damage_dealt` didn't have yet. New step 32b reads the attacker's own
`attached` Tools fresh on every call. Recorded in
[ADR 0043](../../../docs/adr/0043-a-tools-damage-bonus-reads-every-attack.md).

- [x] Brave Bangle: +30, only without a Rule Box and only against an ex
- [x] Binding Mochi: +40, only while Poisoned

## Resolution

Two new `TrainerEffect` variants, one new step in `damage_dealt`.

Coverage: `admitted` 487 -> 491 (2 prints of each); `trainers`
(refused) 304 -> 300.
