# A player sees the game through a masked view

**Status:** Accepted — 2026-09-05

A bot handed the whole game state can read the opponent's hand and the Prize cards and cheat without meaning to, and the mask is painful to retrofit once bots exist. Three ways to build it were live: pass the state and trust every caller; hand out a borrowing view that reads the state through accessors; or build an owned view per decision. Trust was rejected, because nothing enforces it and a mistake is silent. The borrowing view was rejected for now, because it ties every bot to the state's lifetime for a saving the measurement says is small. So `PlayerView::of(state, player)` builds an owned value that carries only what that player may see.

A view hides the cards in the opponent's hand, the cards in either library and their order, and the cards in either Prize pile — a player cannot see their own Prizes either. Each hidden zone keeps its count, because a count is public. Everything on the board is public: the Pokémon, their damage, their conditions, and what is attached to them, for both players, along with both discard piles.

A bot never calls `legal_actions` itself, because that takes the state. The engine computes the list and hands it to the bot with the view, which keeps [ADR 0003](0003-legal-actions-is-the-engine-interface.md) intact: a bot is `(view, legal_actions) -> Action`.

## Consequences

The mask costs an allocation and a copy per decision. Measured over 5000 headless games — 74333 decisions, release build, three runs each — self-play runs at 1.02 microseconds per decision without views and 1.26 with, so 26400 games per second falls to 21500. That is 23% of the decision, and the engine can still play 21500 games a second, so the cost is paid.

If self-play throughput ever becomes the constraint, the escape hatch is the rejected alternative: a borrowing view with the same field names, which removes the copy and costs every bot a lifetime parameter. Nothing in a bot's code would have to change shape.

The measurement is repeatable: `cargo run --release --bin selfplay -- 5000` and the same command with `views`.
