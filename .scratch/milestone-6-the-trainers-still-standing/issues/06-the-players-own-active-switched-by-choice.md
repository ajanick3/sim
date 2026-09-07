# The player's own Active switched by choice

Type: task
Status: resolved

`Switch`: *"Switch your Active Pokémon with 1 of your Benched Pokémon."*
11 slots — the smallest card in this milestone, and the plainest form of a
primitive worth more than its own slot count: `Subjugating Chains`
(Pecharunt ex, an Ability out of scope for now) needs the same switch with
a Special Condition applied on top.

`Phase::Promoting` opens today after a knockout (`of == chooser`) or a card
that switches the *opponent's* Active (`of != chooser`, `Boss's Orders`).
Neither is this: the player switches their own Active, by choice, mid-turn,
with nothing forcing it. Check whether the phase already fits this case as
it stands, or needs to change shape to admit it.

- [x] A player may switch their own Active with a Benched Pokémon, by
      choice, outside of a knockout
- [x] `Switch` plays

## Resolution

`Phase::Promoting` needed no new shape at all: `TrainerEffect::SwitchOwnActive`
opens it with `of == chooser`, the same as after a knockout.

Building it surfaced a real bug the knockout case had always hidden:
`Action::Promote` set the new Active and never asked what happened to the
old one. After a knockout that is correct — the old Active was already
cleared to `None` before the phase opened — but `Switch` and `Boss's
Orders` displace a *live* Active, and the old code silently dropped it
from play, unreachable by any rule, forever.
[ADR 0025](../../../docs/adr/0025-a-live-switch-returns-the-old-active-to-the-bench.md)
records the fix: one line in the shared handler, not a second action.

Coverage went 409 → 411 (2 prints), and the field went 1552 → 1563
playable slots of 3660 — 42.7%.
