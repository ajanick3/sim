# A prize value is read from the card's name

**Status:** Accepted — 2026-09-06

A knockout took one Prize whatever it knocked out, though 559 Pokémon in Standard are worth two or three. The card data was expected to answer this through TCGdex's `suffix` field, and it cannot: the suffix is absent on 21 ex cards, `Mega Charizard X ex` among them, and it is written both `ex` and `EX`. Refusing every card the suffix cannot classify was live and was rejected, because it would refuse cards the engine can otherwise run for a field that is merely untidy.

The name is exact where the suffix is not. Every Pokémon carrying a suffix also ends in ` ex`, and 21 more do, so a Pokémon whose name ends in ` ex` is worth two Prizes, and one whose name also begins with `Mega ` is worth three. 559 cards, 130 of them Mega. The rule reads a Pokémon only: `Mega Signal` is a Trainer, and a name rule applied to every category would have made it a three-Prize card.

## Consequences

A card carries what a knockout of it is worth, and the engine reads that at knockout time rather than assuming one, so a card that adjusts the count has somewhere to act. Nothing adjusts it yet; `Legacy Energy`, `Lillie's Pearl`, and `Briar` are the cards that will.

The rule leans on a naming convention the game has kept so far. If a set prints an ex whose name does not end in ` ex`, or a three-Prize card that is not named `Mega `, this reads it wrong and silently. A test asserts the counts, so a change in the data fails loudly rather than quietly.

For the current card pool, a Rule Box and a prize value above one are the same fact, so the effect filter "a Pokémon that doesn't have a Rule Box" can read `prizes == 1`. That equivalence is a property of this pool, not of the game: a Radiant Pokémon has a Rule Box and gives one Prize, and none is in Standard today.
