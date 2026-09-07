# A cost can be paid in a second physical copy of the card itself

**Status:** Accepted — 2026-09-07

## Context

`Transformation Tome` reads: *"You must play 2 Transformation Tome
cards at once."* Every `Requirement` built so far reads the board, the
player's history, or asks the player to choose which cards to give up
(`DiscardOtherCardsFromHand`). None names a specific second card — the
same print, sitting in the same hand — as the cost itself, with no
choice involved: there is exactly one way to pay it.

Two shapes were live. First: model this as a two-card combo the player
assembles by playing one copy while holding another, checked entirely
in `has_a_target`-style code with no new `Requirement`. Second: a
`Requirement::SecondCopyOfThisInHand`, checked in `legal_actions` the
same way every other requirement is, and consumed automatically in
`Action::PlayTrainer`'s handler once played — no `Phase::Paying`, since
there is nothing to ask the player.

## Decision

`Requirement::SecondCopyOfThisInHand` joins the read-only requirements
`legal_actions` already checks. Unlike them, playing the card has a
side effect beyond the card played: `Action::PlayTrainer`'s dispatch
gets a new arm that finds the other copy by matching `CardDefId` and
moves it straight to discard, then calls `resolve_trainer` the same way
every free requirement already does. No `Phase::Paying` opens, because
`legal_actions` already guarantees exactly one qualifying card exists
to consume — nothing to choose between.

## Consequences

A requirement can now carry a side effect at play time without opening
a phase, as long as satisfying it never involves a choice. A future
card whose cost is "a second copy of a *different* named card" would
need the `CardDefId` comparison generalized to a name comparison, or a
new `Requirement` naming that card; this one only ever compares a card
to itself.
