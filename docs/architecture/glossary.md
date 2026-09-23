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

**Zone**:
Any place a card can be: the Deck, the Hand, the Discard pile, the Prizes, the Active spot, the Bench, a Stadium slot, or attached to a Pokémon (as an Energy, a Tool, or a step of its evolution stack). This is the plain-language sense. `card::Zone` in the engine is a narrower Rust type: it names only three of these — Hand, Discard, Deck — for a Trainer effect that moves a card generically. Bench, Active, Stadium, and attachments each need their own rules, so the engine handles them with separate code, not `card::Zone`. Read "zone" as the plain sense unless the text names the Rust type.

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

**Wire**:
The JSON shape a Masked view (and the moves legal against it) takes crossing the wasm boundary — `WireView`, `WireCard`, `WireActionMeta` in TypeScript, mirroring `crates/sim-wasm`'s own wire structs field for field. Kept deliberately separate from the engine's internal view types ([ADR 0096](../adr/0096-the-engine-compiles-to-wasm-behind-an-opaque-handle.md)), so a UI's contract does not shift when those change.

**Wire client**:
The pure engine-access half of `packages/engine-client`: loads the wasm module, exposes typed Wire data, applies a move by its index into `legal_actions`. Carries no opinion about how a UI presents or gathers that choice — any UI needs exactly this, unmodified.

**Selection**:
What a player has tapped on the board and not yet resolved into a move — a hand Card or a Pokémon. Not an engine concept; input to a Selection flow.

**Selection flow**:
The reference tap-to-choose interaction model in `packages/engine-client`, built on a Wire client but not required by one: narrows `legal_actions` to a current Selection, groups the rest for display, and decides when a single forced choice auto-advances rather than waiting for a tap. One particular pointer-and-tap paradigm, not the only way to drive a Wire client — a CLI menu or a Strategy-like automated driver has no use for it.

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

