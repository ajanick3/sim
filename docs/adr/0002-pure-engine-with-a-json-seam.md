# The engine holds no I/O; card data crosses a JSON seam

**Status:** Accepted — 2026-09-03

The card data lives in Turso, in the `pkmn` repo, so the obvious course was to let the engine query it. That was rejected. An engine that reaches a database mid-game cannot be replayed from a seed, cannot run thousands of games, and drags `tokio` and async lifetimes into the first Rust written for this project. Instead there are three layers: `pkmn` owns Turso and the crawl and gains one export script; a JSON artifact is the seam; `sim` reads that JSON at startup and then runs entirely in memory. The export script belongs to `pkmn` because it queries a schema that repo owns and holds the credentials.

## Consequences

The engine is a pure function of its state, its seed, and its actions, which makes a replay and a test the same thing. The seam has to be versioned once real cards cross it. Milestone 1 needs no card data at all — its cards are literals — so the seam is built later.
