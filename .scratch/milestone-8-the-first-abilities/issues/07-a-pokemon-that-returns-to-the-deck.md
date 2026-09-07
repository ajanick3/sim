# A Pokémon that returns to the deck

Type: task
Status: resolved

`Dudunsparce`'s Ability, "Run Away Draw": *"Once during your turn,
you may draw 3 cards. If you drew any cards in this way, shuffle this
Pokémon and all attached cards into your deck."* 27 slots.

- [x] Drawing and the shuffle-back are one Ability, one action, but
      the shuffle only happens if the draw actually landed something
- [x] Removing an Active `Dudunsparce` opens `Phase::Promoting` only
      with a Bench to promote from
- [x] `Dudunsparce` plays

Recorded in [ADR 0075](../../../docs/adr/0075-a-pokemon-returning-to-the-deck-reuses-knockouts-together-rule.md).

## Resolution

`AbilityEffect::OncePerTurnMayDrawThenShuffleSelfIntoDeck(u32)`, a
standing Ability. Draws first; only if at least one card actually
landed does the Pokémon's whole card stack and every attachment move
into the library together (rule 22's "moves together," the third
zone this milestone has moved a Pokémon's own stack to, after hand
and discard). Removing the Active opens `Phase::Promoting` only when
a Bench exists to promote from, the same guard
`ReturnSelfAndAttachedToHand` already takes; with no Bench, it stays
in play.

Admits both Dudunsparce prints (`Land Crush` has no printed text).
Coverage: `admitted` 602 -> 604.

This closes Milestone 8's own planned ticket order (01-07). `blockers`
still shows real weight in `HasAnAbility` — the milestone continues
past its own map the same way Milestone 11 did, picking off whatever
`blockers` shows next.
