# A next-turn restriction's clearing reads an explicit granting player

**Status:** Accepted — 2026-09-07

**Errata (fixing a bug in an already-merged record).** `opponent_next_turn_restriction`
cleared itself by comparing the *target*'s own owner against
`self.current`: every restriction built through `CoinFlipSelfInvulnerableNextTurn`
(ticket, this session) named the opponent as the target, so "the
target's owner's turn just ended" correctly identified when the
covered turn was over. `Genesect ex`'s `Protect Charge` — "this
Pokémon takes 30 less damage... during your opponent's next turn" —
reads no differently from `CoinFlipSelfInvulnerableNextTurn` and
`DefenderDealsLessDamageNextTurn`'s own self-targeted sibling: both
name the *granting* player's own Pokémon as the target. For those, the
same inference broke — the moment the granting turn ended and the
opponent's turn began, `target`'s owner (the granting player) no
longer matched `self.current` (now the opponent), and the field
cleared itself one full turn early, before the restriction ever had
its one turn to apply. `CoinFlipSelfInvulnerableNextTurn`'s own tests
did not catch this: they branched on whichever way the coin actually
landed rather than asserting the restriction survived, so a "heads"
outcome that cleared instantly still passed silently.

The fix stores the granting player explicitly as a third tuple field —
`state.current` at the moment `resolve_attack_effect` grants it, which
is always the attacker's owner — rather than inferring it from the
target's own ownership. `begin_turn`'s own clear now reads that field
directly: cleared once `self.current == granted_by`, correct for a
restriction on the opponent or on the granting player's own Pokémon
alike.
