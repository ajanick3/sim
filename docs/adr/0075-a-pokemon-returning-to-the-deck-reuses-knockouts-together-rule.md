# A Pokémon returning to the deck reuses the knockout's "moves together" rule

**Status:** Accepted — 2026-09-07

`Dudunsparce`'s `Run Away Draw` — "Once during your turn, you may
draw 3 cards. If you drew any cards in this way, shuffle this Pokémon
and all attached cards into your deck" — is the milestone's last
planned ticket: a Pokémon that removes itself from play as part of its
own Ability, into the library rather than to hand
(`ReturnSelfAndAttachedToHand`, ADR 0071) or discard (`knock_out`).
All three now share the same shape — a card's whole stack and every
attachment move together, rule 22's own rule, to a different zone
each time — none of them reuses a shared helper, since each callsite's
surrounding bookkeeping (spending the right limit, opening
`Phase::Promoting` only when needed, shuffling only where the
destination is the library) differs enough that a shared function
would carry more parameters than body.

`OncePerTurnMayDrawThenShuffleSelfIntoDeck` draws first, then checks
whether any card actually landed (a library too short to pay the full
draw still drew what it had) before doing anything else — an empty
draw changes nothing. Only when at least one card came in does the
Pokémon leave: if it was the Active, `Phase::Promoting` opens only
when the player has a Bench to promote from, the same guard
`ReturnSelfAndAttachedToHand` already takes; with no Bench, the
Pokémon simply stays in play, since removing it would leave nothing
to replace it with.
