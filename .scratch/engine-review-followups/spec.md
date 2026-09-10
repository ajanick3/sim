# Spec: engine-review follow-ups

The senior review of the engine (`.scratch/senior-review/review.md`, PR
#241) raised open questions that each need a decision. This effort tracks
those decisions. It does not carry the review's mechanical clean-ups —
those go straight to code.

## In scope

- Decide the order of the Rule 32 "stop at 0" check against the step-32
  damage additions, and record it.
- Decide whether the action list and the `Phase` payload are outside the
  masked view by design, and record it or close the leak.
- Decide the `settle` loop precedence and record it.
- Decide whether an effect-enum `match` is a compile-time tripwire or a
  tolerant classifier, and record the rule.
- Repoint the two Accepted ADRs that still cite superseded ADR 0002.

## Out of scope

- The Standards clean-ups (the `state.rs` scan helper, `Phase::tag()`, the
  `Cargo.toml` fmt/clippy gap). They need no decision; open a separate
  effort or a direct PR.
- The `legal_actions` / `player_to_act` extraction. It is a planned
  refactor, not a decision, and gets its own effort.

## Constraints

- A decision between live alternatives gets an ADR when it is made
  (`AGENTS.md`). Tickets 01-04 each end in an ADR or an erratum.
- Where a document and the running code disagree, raise it; never settle
  it silently in either direction (`AGENTS.md`). Evidence from the live
  code outranks both documents.
- The rulebook in `docs/architecture/rules.md` is the reference for
  ticket 01. Do not adjudicate against a different rules text.
