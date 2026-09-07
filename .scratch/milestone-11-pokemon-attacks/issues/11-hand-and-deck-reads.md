# Hand and deck reads

Type: task
Status: resolved

*"Your opponent reveals their hand."* — `Hoothoot`'s `Silent Wing`.

- [x] `AttackEffect::RevealOpponentsHand` — logs the reveal; the engine
      already tracks every zone in full, so there is no other state to
      change

Recorded in [ADR 0064](../../../docs/adr/0064-a-hand-reveal-lands-in-the-log-alone.md).

## Resolution

One new `AttackEffect` variant, no new `Phase` or `Action` — the
effect needs no choice and changes nothing `apply` or `legal_actions`
reads.

`Hoothoot`'s sv05-126 print (its only attack) is admitted.

`Mega Excadrill ex`'s `Undermine` (discard the top 2 of the opponent's
deck — the destructive top-of-deck read half of this ticket) is
deferred: every current print pairs it with `Maximum Drilling`, whose
"if this Pokémon has at least 2 extra Energy attached" bonus is a
count this milestone has not built. `Hoothoot`'s other attack
(`Triple Stab`, a per-heads coin-flip count) is a separate print,
already outside this ticket's scope.

Coverage: `admitted` 542 -> 543 (1 print).
