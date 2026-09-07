# Battle Cage & Team Rocket's Watchtower are refused: nothing to prevent

**Status:** Accepted — 2026-09-08

## Context

`Team Rocket's Watchtower` reads: *"Colorless Pokémon in play (both
yours and your opponent's) have no Abilities."* `Battle Cage` reads:
*"Prevent all damage counters from being placed on Benched Pokémon
(both yours and your opponent's) by effects of attacks and Abilities
from the opponent's Pokémon."* Both restrict a mechanism this engine
does not have. No Ability exists anywhere in this engine (the Abilities
effort has not started) — `Team Rocket's Watchtower` would disable a
concept that already never happens. No built or plannable-in-this-
milestone attack ever places damage on a Benched Pokémon — this engine
attacks only ever hit the opponent's Active (the single-Active format
this engine plays) — so `Battle Cage` would prevent a thing that never
happens either.

## Decision

Refuse both, the same reasoning [ADR 0034](0034-enhanced-hammer-is-refused-for-a-card-kind-out-of-scope.md)
already applied to `Enhanced Hammer`: a card whose whole effect is
"disable X" is not faithfully built by admitting it and letting the
check always pass, when X can never occur at all. That is not a
faithful build of the card, it is a card that always does nothing,
dressed as a working one. Both would need real code — a check against
every Ability read site, a check against every place damage lands on a
Bench — with nothing for that code to ever intercept.

## Consequences

Coverage does not move for either. Both stay refused until Abilities
exist (`Team Rocket's Watchtower`) or an attack that hits a Bench does
(`Battle Cage`); the day either changes, this record — not ADR 0034 — is
the one to revisit for that card.
