# A Strategy reads a masked view, not the raw GameState

**Status:** Accepted — 2026-09-09

`play.rs` needed a decision-making interface behind each seat, built to be swapped out over time — a hand-written heuristic today, and eventually something trained rather than written. The question was what that interface hands the decision-maker to read. `GameState` was the obvious first answer: `legal_actions` already reads it, and every existing tool in this repository (`selfplay`, `blockers`, `coverage`) works against it directly.

`GameState` was rejected in favor of `PlayerView` (`src/view.rs`), the masked read this repository already builds for exactly this purpose — hiding the opponent's hand, both libraries, and both Prize piles. A Strategy that reads the raw state can silently learn to read the opponent's hand, since nothing stops it; that mistake costs nothing while every Strategy is a fixed heuristic, and everything the day one is trained against self-play, where an information leak like that produces a policy that only works against an engine that leaks the same way. Building the discipline in from the first Strategy, while it costs nothing to enforce, was cheaper than retrofitting it once a real one exists.

`PlayerView`'s own `CardView` carried only a card's `id` and printed `name`, not enough for a Strategy to read a Trainer's own effect and rank it. Widening the mask further to include effect text was rejected — that read as expanding what `PlayerView` hides, not what it exposes. Instead `CardView` gained a `def: CardDefId` field, and `Strategy::choose` also takes `&CardDb` directly: neither is masked information — a player already knows their own hand's card identities, and `CardDb` is every card's printed rules text, true of every copy, not a fact about which cards are where. Only zone membership stays behind the mask.

A Strategy owns its own randomness, separate from `GameState`'s own `rng` field, for the same reason: a real decision-maker plugged in later has no way to read the engine's own internal random stream, so none should exist for today's Strategy to lean on either.
