# Damage to a Benched Pokémon

Type: task
Status: resolved

*"Put 6 damage counters on your opponent's Benched Pokémon in any way you
like."* — `Dragapult ex`'s `Phantom Dive`.

The first attack effect to touch anything but the Active Pokémon.

- [x] `AttackEffect::DamageCountersToOpponentBenchAnyWay(u32)`
- [x] New `Phase::DistributingDamageCounters { player, remaining }`,
      mandatory placement (no decline), one counter at a time via
      `Action::PlaceDamageCounter { target }`
- [x] Auto-transitions to `Phase::Main` and calls `settle` once
      `remaining` reaches 0
- [x] An empty opponent Bench opens no phase at all

Recorded in [ADR 0060](../../../docs/adr/0060-benched-damage-lands-through-a-mandatory-placement-phase.md).

## Resolution

One new `AttackEffect` variant, one new `Phase` variant, one new
`Action` variant with its `player_to_act`, `legal_actions`, `describe`,
and `apply` arms.

`Dragapult ex` is admitted (5 prints); its other attack (`Jet
Headbutt`) has no printed text.

Coverage: `admitted` 530 -> 535 (5 prints).
