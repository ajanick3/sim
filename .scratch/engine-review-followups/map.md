# Map: engine-review follow-ups

## Destination

Every open question from the senior review has a decision on record — four
ADRs (or errata) written, and two stale ADR citations repointed.

## Notes

The review ran the `code-review` skill's Standards and Spec axes over the
whole engine. The Spec axis found the load-bearing ADRs (0002/0007, 0003,
0006, 0009, 0013, 0014, 0042) hold. What it could not settle:

- **Rule 32 order.** `engine.rs:4413-4416` runs "stop at 0" against
  `base`, then adds `turn_bonus`, attacker Tools, Cobalt Command, Lose
  Cool at 4417-4502. `rules.md:70-71` places the stop after the
  your-Pokemon effects. A 0-base attack with `BonusDamageWithoutRuleBoxVsEx`
  yields 0 and never reaches the Tool loop.
- **The mask.** `view.rs` copies `state.phase` verbatim. Several `Phase`
  variants carry deck `CardId`s. ADR 0086 carved out one card
  (Claw of Darkness); the general rule that `legal_actions` output and
  `Phase` fields sit outside the mask has no record.
- **`settle` precedence.** `engine.rs:4564-4640` fixes an order — game
  over, `knock_out_the_dead`, Area Zero bench-shrink, rule-40 promote,
  end-turn. Rules 44/48 and ADRs 0053/0082 cover fragments, not the whole
  order.
- **Exhaustive match versus catch-all.** `state.rs:1035` spells out every
  `EnergyEffect`; `strategy.rs:178` uses `_ =>`. Both carry a defending
  comment. A reader cannot tell which to write next.
- **Stale citations.** ADR 0002 is `Superseded by 0007`. ADR 0042:37 and
  ADR 0096:17 still cite 0002 for the purity guarantee.

## Tickets

- 01 — Rule 32: does "stop at 0" run before or after the step-32 additions?
- 02 — Is the action list outside the masked view by design, or a leak?
- 03 — Record the `settle` loop precedence.
- 04 — Is an effect-enum match a tripwire or a classifier?
- 05 — Repoint ADR 0042 and ADR 0096 from ADR 0002 to ADR 0007.

## Decisions so far

None yet. The effort opens 2026-09-10.
