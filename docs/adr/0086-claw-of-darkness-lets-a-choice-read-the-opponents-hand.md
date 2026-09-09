# Claw of Darkness lets a choice read the opponent's hand

**Status:** Accepted — 2026-09-09

## Context

Every choice built so far — search a library, discard an Energy,
pick a Bench target — reads from the *acting* player's own zones, or
from board state visible to both players already. `Mega Absol ex`'s
`Claw of Darkness` ("Your opponent reveals their hand, and you
discard a card you find there") is the first effect where the
acting player's own choice is offered over the *opponent's* hand.

The engine already tracks every zone in full internally — `view::
PlayerView` only hides a zone at render time, and nothing in
`legal_actions` has ever gated a choice on hidden information — so
"reveals" itself changes no state, the same non-event
`AttackEffect::RevealOpponentsHand` already is. The only new thing
is a choice phase whose offered targets come from `state.player
(whose.opponent()).hand` instead of `state.player(whose).hand`.

## Decision

`Phase::ChoosingCardFromOpponentsHandToDiscard { player }` reads
`state.player(player.opponent()).hand` directly in its own
`legal_actions` arm — no new zone-visibility primitive, no change to
`view`. The card text has no "may," so an empty opponent hand opens
no phase at all, the same "no qualifying targets, no phase" shape
every other optional search or choice already takes.

## Consequences

A future effect that acts on the opponent's hand more than
"discard a card you pick" (moving a card to hand, forcing a
specific discard by name) can read the same zone path directly;
nothing here generalizes it into its own helper, since one call
site doesn't earn one yet.
