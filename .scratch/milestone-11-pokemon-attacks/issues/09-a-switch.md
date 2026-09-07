# A switch

Type: task
Status: resolved

*"Switch this Pokémon with 1 of your Benched Pokémon."* — `Abra`'s
`Teleportation Attack`.

- [x] `AttackEffect::SwitchOwnActive`, dispatching into the same
      `Phase::Promoting` shape `TrainerEffect::SwitchOwnActive` already
      opens
- [x] An empty Bench opens no phase

Recorded in [ADR 0062](../../../docs/adr/0062-an-attack-switch-dispatches-straight-into-promoting.md).

## Resolution

One new `AttackEffect` variant, dispatching into existing machinery —
no new `Phase` or `Action`.

`Abra`'s me01-054 print is admitted; its other print's only attack has
no printed text.

`Metagross`'s `Bounce Back` (the `SwitchOpponentActive` half of this
ticket) is deferred: every current print pairs it with a shape
(`Meteor Mash`'s persistent own-next-turn bonus, `Conjoined Beams`'s
board-count bonus) this milestone has not built yet.

Coverage: `admitted` 538 -> 539 (1 print).
