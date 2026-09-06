# Prize values

Type: task
Status: resolved

A knockout always takes 1 Prize. In Standard, 492 cards carry the `ex` rule
and are worth 2, and a Mega ex is worth 3.

The data cannot tell a Mega ex from an ordinary ex except by the `Mega ` at the
start of its name, which is recorded in
[card data findings](../../../docs/architecture/card-data.md). Decide whether
the importer trusts that or refuses the card.

- [x] A card carries what a knockout of it is worth
- [x] The count is read at knockout time, not stored as a fixed number
- [x] A decision on the Mega ex, recorded

## Answer

Resolved 2026-09-06 on branch `feat/prize-values`.

The decision is [ADR 0010](../../../docs/adr/0010-a-prize-value-is-read-from-the-card-name.md):
the prize value is read from the card's name, not from TCGdex's `suffix`.

The ticket assumed the suffix would answer this. It cannot. The suffix is
absent on 21 ex cards, `Mega Charizard X ex` among them, and it is written both
`ex` and `EX`. The name is exact: every Pokémon carrying a suffix also ends in
` ex`, and 21 more do — 559 by name against 538 by suffix, with no card
contradicting the other. 130 of the 559 are Mega ex and worth 3.

The rule reads a Pokémon only. `Mega Signal` is a Trainer, and a name rule
applied to every category would have made it worth 3 Prizes.

`knock_out_the_dead` reads the value at knockout time rather than assuming 1,
so a card that adjusts the count has somewhere to act. `Legacy Energy`,
`Lillie's Pearl`, and `Briar` are the cards that will.

A test asserts the two counts, so a set that breaks the naming convention fails
loudly rather than quietly.

For the current pool, a Rule Box and a prize value above 1 are the same fact,
so ticket 03's filter "a Pokémon that doesn't have a Rule Box" can read
`prizes == 1`. The ADR records that this is a property of the pool, not of the
game.
