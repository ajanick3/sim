# A passive Ability reads directly from its own computation site, never as a standing choice

**Status:** Accepted — 2026-09-08

Every `AbilityEffect` built through Milestone 8 modeled a choice: the
player decides to draw, to search, to attach, to switch. `Latias ex`'s
`Skyliner` ("Your Basic Pokémon in play have no Retreat Cost") is not a
choice at all — it holds while the Pokémon is in play, with nothing to
accept or decline. Two shapes were live: model it as an `Action` the
player always takes at the start of their turn (keeping every
`AbilityEffect` a standing choice, at the cost of a needless action
in every game log), or let a passive effect skip `Action::UseAbility`
outright and have the one place that computes the affected fact —
`effective_retreat_cost`, here — check for it directly on every one
of the owner's in-play Pokémon. The direct read won: it costs nothing
in the action log, and it matches how a Tool's own reduction already
works (`effective_retreat_cost` already reads a Tool's
`TrainerEffect::ReducesRetreatCost` the same way, rather than making
attaching it a standing choice to use).

## Consequences

A passive `AbilityEffect` variant is named `Passive...` and carries a
doc comment naming exactly which computation reads it. `legal_actions`
never offers `Action::UseAbility` for one (an explicit `=> false` arm,
mirroring the play-triggered effects' own `=> false` arms); the
`Action::UseAbility` apply handler treats reaching one as
`unreachable!`, the same way a play-triggered effect already does.
This scales to any future passive Ability, so long as its computation
site can be named in one sentence — an Ability affecting several
unrelated computations at once would need its own decision.
