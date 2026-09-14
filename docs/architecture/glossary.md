# Glossary

This repository's domain vocabulary. The domain is new; a term joins this glossary when the project pins it down.

**Record**:
A permanent document: a decision record, a standard, a runbook, an architecture document. It stands alone for a reader who was not there.

**Decklist**:
The 60 cards a player brings to a game, as input to the engine.

**Deck**:
The draw pile in play. It starts as the shuffled Decklist less the opening hand and the Prizes, and a player who cannot draw from it loses.

**Card**:
One physical card in one game. It is created at setup, it never moves in memory, and a zone holds its id.

**Card definition**:
What a card says as printed, shared by every copy of it.

**Print**:
One release of a Card definition, identified by its TCGdex print id (set code and card number). TCGdex relates cards only by a shared name, and a shared name is not proof of a shared Card definition: two Pokémon can print the same species name with different HP, attacks, and Abilities — unrelated cards, not two Prints of one card. Two Prints of one Card definition are the ones that also share every rules-relevant field; only then are they the same rules text in different art. The engine treats each Print as pinned independently regardless — a Decklist line names an exact Print — and a viewer's preferred Print is a display choice only; it never changes which Card definition a game plays.

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

**Strategy**:
The decision a seat's own player makes each turn: which legal Action to take, reading only that seat's own masked view. One per seat, swappable independently of the other. The engine offers legal Actions; a Strategy is what picks among them.

**Sequencing**:
The order in which a Strategy takes the actions a turn allows, before or instead of attacking. A hand of legal plays says nothing about which to take first; sequencing is that choice, made new every decision.

**Entrant**:
One player registered for a tournament. Every entrant plays the Swiss rounds; only some reach a standing.

**Top cut**:
The entrants a tournament ranks by a unique final standing, once Swiss rounds end. Not every entrant reaches it.

**Deck field**:
The Decklists a tournament contributes to `decks/`, one file per entrant actually kept. It need not match the top cut: an entrant outside it can still be worth keeping, and one inside it can still be dropped for a card gap.

