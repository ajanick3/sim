# The engine goes deep and the card set stays tiny

**Status:** Accepted — 2026-09-03

The obvious first goal was to implement the Standard pool, about 3023 cards. It was rejected. This project exists to learn Rust, and the card implementations are repetitive: the first thirty teach the language and the rest are grind. The engine core is what teaches Rust — enums, exhaustive `match`, ownership over a mutable game graph, error types, traits. So the engine goes deep and the card set stays synthetic and small, and it grows a card only when that card teaches the engine something it cannot yet express.

## Consequences

Progress is measured by what the engine can express, not by a card count. The card data pipeline can wait, because a synthetic card set needs none of it. If the goal ever stops being learning Rust, this decision must be re-derived rather than inherited.
