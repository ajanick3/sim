# A legal action reports whether it is a safe fallback

**Status:** Accepted — 2026-09-25

Several `Phase`s add one action that is always legal beside their
per-card choices — `EndTurn` in `Main`, `DeclineBonusDraws` in
`TakingBonusDraws`, `FinishDeciding` in `Deciding`, and more — so a
player who wants no per-card choice can still act. The codex UI had no
way to read this from the wire: it matched the action's label text
against regexes (`/^End turn$/i`, `/^(Stop |Finish|...)/`) to guess
which button was safe to treat as a dialog's Cancel or Done action. Two
bugs came from this guess drifting out of step with the engine: a
"Stop taking cards" dialog stacked on top of another dialog because its
action slipped through a `null`/`null` catch-all meant for something
else, and a mandatory "Take a bonus card" prompt gained a Cancel button
that soft-locked the game, because the heuristic saw *some* other legal
action existed without checking whether it was a real way out.

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
  to find the deck-search drawer's Done action, replacing the label
  regex that had already needed one exclusion (`finishSearch`) to stop
  colliding with the dialog catch-all.
- The rest of `web/app` (`session.ts`'s `ACTION_GROUPS`/`groupOf`) and
  `packages/engine-client`'s selection flow still match on label text.
  They are unchanged by this record; migrating them is a separate
  decision, since it touches the shared board UI, not the isolated
  codex route.
- The Cancel-eligibility heuristic in `CodexGameShell` (`dialogIndices`
  populated, or `EndTurn` legal) is unchanged by this record — it was
  already correct. `is_fallback` fixes the *other* heuristic, the one
  that picks a dialog's own Done action.
