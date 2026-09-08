# Ancient and Future join `markers`, scoped to the decks and open

**Status:** Accepted — 2026-09-08

## Context

`Iron Crown ex`'s Ability, `Cobalt Command`, keys on "your Future
Pokémon." The artifact carries no field for the Paradox Ancient or
Future trait at all — every card's own `trait` field is empty — so
this is the same data gap `Marker::Tera` closed in ADR 0081: an
external source, hand-verified, spliced against the artifact's own
print ids.

Two questions had to be answered before writing a table, not after:
whether a name can be trusted alone, and how wide to scope the list.

**Name vs. print id.** Tera forced print-id precision because a
name could split — `Hydreigon ex` prints both a Tera version and a
plain one. Checking the same question for Paradox names (pkmncards's
`is:ancient`/`is:future` search, cross-referenced print by print
against every print of each name this artifact carries) found the
same shape: `Koraidon ex` prints ten times in the wider pool, but
only its Temporal Forces print (`sv05-120`) is Ancient — the rest,
`me02.5-121` (itself a Tera print) and `svp-197`, are not. Every
other deck-relevant name checked this way (`Brute Bonnet`, `Flutter
Mane`, `Raging Bolt ex`, `Iron Crown ex`, `Iron Leaves ex`) turned
out consistent across every one of its own prints — but one split is
enough to rule out name matching as a general rule. `markers_for`
reads Ancient and Future from print id, the same shape Tera already
takes.

**Scope.** Tera's own table covers the whole artifact pool (104
prints) because Pokémon TCG confirmed it stopped printing Tera
cards — a closed list, safe to build once and forget. Nobody has
confirmed the same for the Paradox traits, and the "is Ancient
still being printed" question was never asked. Building a pool-wide
table on an unconfirmed premise would either be wrong the moment a
new set imports, or force redoing print-by-print verification work
that only three sample-deck slots currently need. `ANCIENT_PRINT_IDS`
and `FUTURE_PRINT_IDS` cover only the six species names the sample
decks actually use, and their own doc comments say so plainly:
absence from the list means "never checked," not "confirmed
untagged."

## Decision

`Marker` gains `Ancient` and `Future`. Both are read from print id
against their own const tables, deck-scoped and explicitly open —
the opposite framing from `TERA_PRINT_IDS`'s "closed, permanent."
Extending either table to a name outside its current six is the
same hand-verification work Tera and this table both already did,
not a new decision.

`Cobalt Command` itself reads `Marker::Future` off the attacker at
the point `damage_dealt_with` already computes damage against the
opponent's Active — the format's single-Active shape means a call
there is never against a Benched target, so no extra check names
"the opponent's Active Pokémon" explicitly. The self-exclusion in
the card's own text ("except any Iron Crown ex") reads the
attacker's printed name, not "the carrier" — a second copy on the
Bench doesn't boost the one attacking, and the one attacking is
excluded even holding its own Ability.

## Consequences

A future card keying on Ancient or Future for a name outside this
session's six needs its own hand-verification pass against
pkmncards before extending either table — the same cost Tera's own
table already paid once, paid again in smaller pieces as new names
come up.
