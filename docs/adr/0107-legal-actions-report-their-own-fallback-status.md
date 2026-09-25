# A legal action reports whether it is a safe fallback

**Status:** Accepted — 2026-09-25

Several `Phase`s add one action that is always legal beside their
per-card choices — `EndTurn` in `Main`, `DeclineBonusDraws` in
`TakingBonusDraws`, `FinishDeciding` in `Deciding`, and more — so a
player who wants no per-card choice can still act. The codex UI had no
way to read this from the wire: it matched the action's label text
against regexes (`/^End turn$/i`, `/^(Stop |Finish|...)/`) to guess
which button was safe to treat as a dialog's Cancel or Done action. A
"Stop taking cards" dialog stacked on top of another dialog because its
action slipped through a `null`/`null` catch-all meant for something
else — the bug this record fixes. A related bug, where a "Take a bonus
card" prompt gained a Cancel button that soft-locked the game, was
fixed separately (ADR-less, in the commit history): the engine already
lists a `Decline*` fallback beside a mandatory-looking choice in every
phase that has one, so the fix was to stop inventing Cancel eligibility
beyond the two cases where dismissing costs nothing — not to detect
"no fallback exists," which this phase never actually lacked.

## Decision

`Game.action_meta()` gains a field, `is_fallback`, on each entry: `true`
when the action's `Action` variant is `EndTurn` or starts with `Finish`
or `Decline` — the engine's own naming convention for "always legal on
its own." A helper, `is_fallback(Action) -> bool`, reads the same
`Debug`-derived name `action_kind` already computes, so a new `Finish*`
or `Decline*` variant needs no change here, the same guarantee ADR 0099
gives `kind`.

`is_fallback` answers one question only: is this action safe to fall
back on without picking a card or target first? It does not mean the
action is the *only* one open, and `false` does not mean an action is
mandatory — a targeted action like `PlayBasic` is never a fallback, but
it is rarely mandatory either.

The TypeScript field is optional (`is_fallback?: boolean`) so existing
fixtures across `web/app` and `packages/engine-client`, most of which
predate this decision and do not exercise fallback logic, keep
type-checking unchanged.

## Consequences

- `web/app/codex/_game/CodexGameShell.tsx` reads `action.is_fallback`
  to find the deck-search drawer's Done action, but only while the
  drawer itself is showing (`searchCards.length > 0`). A phase with a
  `Decline*` action and no deck-search cards in view — `TakingBonusDraws`,
  say — has nowhere else to render that action if this code claimed it
  for a drawer that is not on screen; scoping it this way keeps every
  `Finish*`/`Decline*` action visible somewhere.
- Most phases never needed a Cancel button in the first place: the
  engine already lists their fallback (`DeclineBonusDraws`, and the
  rest) as an ordinary choice beside the per-card options, so tapping
  it directly is how a player exits. `CodexGameShell`'s Cancel button
  stays reserved for the two cases where the fallback is *not* already
  a visible choice: a card-tapped selection (clearing it costs nothing)
  and the auto-shown dialog while `EndTurn` is legal (a board-level
  fallback outside the dialog itself). `is_fallback` did not change
  that logic — it only replaced the label regex for the drawer's Done
  action.
- The rest of `web/app` (`session.ts`'s `ACTION_GROUPS`/`groupOf`) and
  `packages/engine-client`'s selection flow still match on label text.
  They are unchanged by this record; migrating them is a separate
  decision, since it touches the shared board UI, not the isolated
  codex route.
