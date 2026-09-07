# A paid cost clears its own phase before the effect runs

**Status:** Accepted — 2026-09-07

`Action::PayWithCard`'s final payment ran the paid-for effect directly,
without first setting `state.phase` back to `Main`. Every card that paid a
cost before `Morty's Conviction` — `Ultra Ball`, `N's Zoroark ex` — has an
effect that opens a phase of its own, and that phase overwrote `Paying` in
the same step, so the gap was invisible. `TrainerEffect::DrawPerOpponentBenched`
opens no phase at all: it drew the right cards, then left `Phase::Paying`
sitting there with nothing left to pay and no way out. The game was
correctly stuck — every future action would be refused, since nothing in
`legal_actions` reads a stale `Paying` as anything but real.

The fix is one line: set `state.phase = Phase::Main` before calling
`resolve_trainer`, the same place `PlayTrainer` already leaves `Main`
behind it before an effect with no phase of its own runs, and the same
fix `Action::ChooseOption` (ADR 0029) needed for the identical reason one
ticket earlier. Three call sites now share the same shape: clear the
phase the choice or cost just finished, then let the effect either
overwrite it or leave it alone.

## Consequences

No test before this ticket exercised a paid cost whose effect opens no
phase, so the bug shipped unnoticed through two earlier cards. The lesson
generalizes past this one fix: a phase-clearing bug hides behind any
effect that happens to open its own phase, and only shows once a
no-phase effect reaches the same call site — worth checking for at each
of the few remaining places an effect resolves from inside another phase.
