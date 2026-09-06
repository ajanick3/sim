# Glossary

This repository's domain vocabulary. The domain is new; a term joins this glossary when the project pins it down.

**Record**:
A permanent document: a decision record, a standard, a runbook, an architecture document. It stands alone for a reader who was not there.

**Decklist**:
The 60 cards a player brings to a game, as input to the engine. Distinct from a Deck, which is a tournament fact.

**Library**:
The draw pile in play. It starts as the shuffled Decklist less the opening hand and the Prizes, and a player who cannot draw from it loses.

**Card**:
One physical card in one game. It is created at setup, it never moves in memory, and a zone holds its id.

**Card definition**:
What a card says as printed, shared by every copy of it.

**Seat**:
One of the two sides of a game. A seat is not a turn order: the opening coin flip decides who goes first, so either seat may start.

**Special Condition**:
Asleep, Paralyzed, Confused, Burned, or Poisoned. Only the Active carries one. The first three rotate the card and replace each other; the last two are independent.

**Pokémon Checkup**:
The step between two turns. It resolves the Special Conditions, then knocks out anything left at 0 HP, and only then does the next turn start.

**Prize**:
One of the six cards a player sets aside at setup and takes for a knockout. Taking the last one wins the game.

**Masked view**:
The game as one player may see it. It hides the opponent's hand, both libraries, and both Prize piles, and keeps a count for each.

