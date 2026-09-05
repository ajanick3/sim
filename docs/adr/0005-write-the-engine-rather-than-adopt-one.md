# We write the engine rather than adopt an existing one

**Status:** Accepted — 2026-09-03

Five existing Pokémon TCG projects were surveyed before any code was written: `ryuu-play`, `tcgone-engine-contrib`, `deckgym-core`, `PTCG-Bench`, and the TCGdex data behind them. Each was rejected. `ryuu-play` is the best design reference but carries zero cards from the current regulation marks, is an application rather than a library, and models a mid-effect choice as a generator. `tcgone-engine-contrib` stops at generation 8. `deckgym-core` implements a different game, TCG Pocket, and is AGPL-3.0, which would reach the sibling repository. `PTCG-Bench` is a small research benchmark. The survey's conclusion is that no executable card logic exists for the current era, in any language, under any license, so this repository writes its own and takes no code from any of them.

## Consequences

The rejected projects stay useful as references — `ryuu-play` for its state machine and its prize-count model, `tcgone-engine-contrib` for its vocabulary of effect primitives — and citing a design idea from one is not taking its code. The survey is recorded in [card data sources](../architecture/sources.md) so it is not repeated.
