# Repoint ADR 0042 and ADR 0096 from ADR 0002 to ADR 0007

Type: task
Status: needs-triage

The senior review (Spec axis, drift) found two Accepted ADRs still cite a
superseded record for the purity guarantee.

## The facts

ADR 0002 header: `Status: Superseded by 0007 — 2026-09-06`. ADR 0007
restates the purity clause in full and says "This record carries the whole
decision."

Two Accepted ADRs still point at 0002 by number:

```
0042:37  the engine's purity and replay guarantees (ADR 0002, ADR 0009) make a
0096:17  keeps the seam of [ADR 0002](0002-pure-engine-with-a-json-seam.md)
```

A reader following either citation lands on a "Superseded" banner and must
know to jump to 0007.

## The decision

This is citation plumbing, not a decision between alternatives, so it needs
no new ADR. Per the ADR template: "Maintenance that preserves the recorded
meaning needs no erratum — a link repair." Repoint both citations to 0007.
Keep the wording; change only the reference.

`AGENTS.md` requires that the conflict be raised rather than settled
silently — this ticket is that raise. Resolve it with the edit.

## Acceptance criteria

- [ ] ADR 0042 line 37 cites ADR 0007 for the purity guarantee.
- [ ] ADR 0096 line 17 cites ADR 0007 for the seam.
- [ ] No other ADR cites 0002 for a guarantee 0007 now carries (grep to
      confirm).
