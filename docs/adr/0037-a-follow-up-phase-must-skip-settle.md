# A follow-up that opens a new phase must skip `settle`

**Status:** Accepted — 2026-09-07

## Context

`Prime Catcher` chains a second `Phase::Promoting` onto the first, the
same way `PromoteFollowUp` already chains `HealDisplacedIfEx` and
`DrawUpTo` onto a switch. Its own follow-up, `AlsoSwitchOwnActive`,
opened the new phase correctly — and `Action::Promote`'s unconditional
`settle(state)` call, at the end of the handler, immediately reset it
back to `Phase::Main`. `settle`'s own loop ends with `if
!state.pending_turn_start { state.phase = Phase::Main; return; }` —
correct for every existing follow-up, none of which touch `state.phase`
themselves, and wrong the moment one does.

This is the same trap ADR 0030 named for a paid cost's effect: code
written before any follow-up needed to open its own phase assumed
whatever ran after it would leave `state.phase` for `settle` to decide.
`AlsoSwitchOwnActive` is the first follow-up that does not hold.

## Decision

`Action::Promote`'s handler returns early, skipping `settle`, exactly
when `AlsoSwitchOwnActive` actually opens a new `Phase::Promoting` (its
own Bench was non-empty). Every other path — no follow-up, a follow-up
that only touches damage or a hand, or an empty Bench that skips the
switch — still falls through to `settle` unchanged.

## Consequences

A future follow-up that opens its own phase must return early the same
way, rather than call `settle` and assume it will not run. This is now
the second such trap (`PayWithCard`, ADR 0030) `resolve_trainer` and its
callers carry; a third occurrence would be worth naming the pattern
explicitly rather than fixing it site by site again.
