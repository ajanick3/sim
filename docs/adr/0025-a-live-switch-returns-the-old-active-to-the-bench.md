# A live switch returns the old Active to the Bench

**Status:** Accepted — 2026-09-07

`Switch` reads *"Switch your Active Pokémon with 1 of your Benched
Pokémon"* — the player's own Active, by their own choice, mid-turn.
`Phase::Promoting` already covered the shape it needed: `of == chooser`,
the same as after a knockout. Building it surfaced a bug in the phase's
one handler, `Action::Promote`, that a knockout's own case had always hidden:
it set the new Active and never asked what happened to the old one. After
a knockout, that is correct — `knock_out` already cleared `active` to
`None` before the phase opened, so there was nothing to lose. `Switch`
and `Boss's Orders` are not knockouts: their Active is *displaced*, not
gone, and the old code silently dropped it from play — off the Bench, off
the Active spot, unreachable by any rule, forever. No test had exercised a
live switch closely enough to notice; `boss_orders_switches_the_opponents_active`
checked only that the new Active landed correctly.

The fix belongs in `Action::Promote` itself, not in a second action for a
live swap: `side.active.replace(pokemon)` returns whatever was there
before, and if that is `Some`, it goes onto the Bench the promoted Pokémon
just left. One line serves both callers — `Switch` and the retroactively
corrected `Boss's Orders` — because the phase never needed to change shape
at all, only the one place that resolves it.

## Consequences

`TrainerEffect::SwitchOwnActive` is the plain, symmetric form of
`SwitchOpponentActive`: `of` and `chooser` are the same player instead of
different ones. A regression test plays a live `Boss's Orders`-shaped
switch and checks the displaced Pokémon is still findable on the Bench
afterward — the fact the original test never checked.
