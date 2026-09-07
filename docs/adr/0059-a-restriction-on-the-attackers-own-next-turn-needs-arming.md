# A restriction on the attacker's own next turn needs an "armed" flag

**Status:** Accepted — 2026-09-08

## Context

`N's Zekrom`'s `Rampaging Thunder` reads: *"During your next turn,
this Pokémon can't use attacks."* This is `opponent_next_turn_restriction`'s
mirror (ADR 0058) — a restriction granted mid-turn, read on a later
turn — but the direction is reversed: it is granted during the
*target's own* turn (the attacker grants it to themselves), and must
apply on the target's *next* occurrence of their own turn, not the
occurrence already in progress.

`opponent_next_turn_restriction`'s single clearing condition
(`target.owner != current`) does not fit here unmodified: checked
naively, it would already read `target.owner == current` at the moment
of granting (the attacker's own turn, still running) and again two
turns later — both true, with no way to tell "the turn granting it"
from "the turn it should restrict" apart, since both are the target
owner's own turn.

## Decision

`own_next_turn_restriction: Option<(PokemonId, AttackEffect, bool)>` —
the third field, `armed`, distinguishes the two. Granted with
`armed: false`. `begin_turn` arms it (`armed: true`) the first time it
sees `target.owner != current` after granting — the opponent's turn
starting, which does not itself get restricted but marks that the
*next* time it is the target's own turn is the restricted one. Once
armed, `begin_turn` clears it the next time it sees `target.owner !=
current` again — the turn after the restricted one has ended. The
Attack-offering site in `legal_actions` only reads the restriction as
active when `armed` is `true`, so the granting turn itself is never
affected — the field exists but is not yet armed, and the read site
does not distinguish beyond that.

## Consequences

A restriction's lifetime relative to when it is granted (during the
target's own turn vs. during the opponent's, ADR 0058) now determines
which of the two fields it belongs in, and whether it needs the arming
flag: granted on the opponent's turn (about the opponent's own next
turn) needs no arming, since the granting and restricted turns are
already different owners; granted on the target's own turn (about
their own next turn) does, since the granting and restricted turns
share an owner and must be told apart some other way.
