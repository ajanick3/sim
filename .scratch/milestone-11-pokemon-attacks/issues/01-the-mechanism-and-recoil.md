# The mechanism, and Recoil

Type: task
Status: resolved

Nothing has ever read an attack's own printed effect text —
`read_attack` refuses any non-empty `effect` field unconditionally.

- [x] `AttackEffect`, a new enum parallel to `TrainerEffect`, read from
      `attack()` rather than `resolve_trainer`
- [x] `known_attack(pokemon_name, attack_name)`, mirroring
      `known_trainer`, checked in `read_attack` before the refusal
- [x] `Recoil(u32)`: this many damage counters land on the attacker
      itself, alongside the attack's own damage to the defender

Recorded in [ADR 0054](../../../docs/adr/0054-an-attacks-effect-is-its-own-vocabulary.md).

## Resolution

New `Attack.effect` field (every existing `Attack` literal across the
codebase updated), new `AttackEffect` enum, new `resolve_attack_effect`
dispatch in `engine.rs`, new `known_attack` lookup in `import.rs`.

Built: `Carvanha` (1 print), `Tapu Bulu` (2 prints), and `Rellor`'s
single-attack print (`sv05-023`). `Paldean Tauros`'s recoil attack
(`Double-Edge`) is matched too, but that print's other attack (`Raging
Charge`, a damage multiplier) still refuses it — ticket 02's card.

Coverage: `admitted` 515 -> 519 (4 prints); `AttackHasText` (refused)
1658 -> 1654.
