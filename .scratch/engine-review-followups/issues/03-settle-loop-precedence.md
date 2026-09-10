# Record the settle loop precedence

Type: task
Status: needs-triage

The senior review (Spec axis, unrecorded-decision 3) found that `settle`
fixes an order of between-step work that no single document records.

## What the code does

`engine.rs:4564-4640`, the `settle` loop, runs in this order:

1. game-over check
2. `knock_out_the_dead`
3. Area Zero bench-shrink phase
4. rule-40 promote
5. end-turn / checkup / turn-start

`is_over()` is re-checked three times through the loop. The Area Zero
bench-shrink phase opens before a pending rule-40 promote.

## Why it needs a record

Rules 44 and 48, and ADRs 0053 and 0082, each cover a fragment of this
order. None states the whole precedence. A future change that reorders two
steps has nothing to check itself against, and nothing to explain why the
order is what it is.

## The decision

Write an ADR that states the full `settle` precedence and the reason each
step sits where it does — in particular why the bench-shrink phase opens
before the promote. The ADR records the order the code already runs; it
does not change behaviour unless triage finds the order wrong.

## Acceptance criteria

- [ ] An ADR lists the `settle` steps in order with a one-line reason for
      each ordering constraint.
- [ ] The ADR cross-references rules 44 and 48 and ADRs 0053 and 0082.
- [ ] `settle` carries a comment that points to the ADR.
