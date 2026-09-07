# A Stadium's own action is offered directly, not dispatched at play time

**Status:** Accepted — 2026-09-08

## Context

`Academy at Night` reads: *"Once during each player's turn, that player
may put a card from their hand on top of their deck."* This is not
something that happens when the Stadium is played — it is an action
the Stadium *being in play* makes available, to either player, on their
own turn, once. Nothing built offers an action gated on which card
merely sits in a zone (`state.stadium`) rather than one a hand card's
own effect names.

## Decision

`legal_actions`' Main-phase generation checks `state.stadium_effect() ==
Some(TrainerEffect::MayPutHandCardOnTopOfDeck)` directly, the same
place `AttachEnergy` and `PlayTool` are generated — before the general
hand loop, not inside `PlayTrainer`'s dispatch. A new `Limit` variant,
`StadiumEffectUsed(PlayerId)`, gates it to once a turn; since only one
Stadium is ever in play, one variant covers whichever Stadium's own
action this turn offers, the same way `Limit::StadiumPlayed` already
covers playing any Stadium regardless of which.

`resolve_trainer` still gets a no-op arm for `MayPutHandCardOnTopOfDeck`
— playing the Stadium itself does nothing, the same as `ReducesHpForStage`
and `RemovesRetreatCostForNamePrefix` (ADR 0048); the action this
effect actually grants is generated separately, from `state.stadium`
being set, not from anything `resolve_trainer` runs.

## Consequences

A Stadium's own once-a-turn action is now a third kind of Trainer
effect, alongside a played effect (`resolve_trainer`) and a derived
stat (ADR 0042, ADR 0048): an action generated directly in
`legal_actions`, gated by a shared `Limit` variant. `Team Rocket's
Factory` and `Lumiose City` — the two other once-a-turn Stadium actions
this milestone still owes — extend the same check with their own
`TrainerEffect` match arms, sharing `Limit::StadiumEffectUsed`.
