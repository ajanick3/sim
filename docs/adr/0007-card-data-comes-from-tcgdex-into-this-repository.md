# The engine holds no I/O; card data crosses a JSON seam from TCGdex

**Status:** Accepted — 2026-09-05

Supersedes [0002](0002-pure-engine-with-a-json-seam.md), which decided the same seam but drew the data from the `pkmn` repository's Turso database. The operator now takes card data from TCGdex, which needs no credentials and no database, so the reason that put the export script in that repository is gone. This record carries the whole decision, so nothing is owed to the superseded one.

The engine holds no I/O. It queries no database and opens no socket while a game runs, because an engine that reaches out mid-game cannot be replayed from a seed, cannot run thousands of games, and drags async lifetimes into the first Rust written for this project. A JSON artifact is the seam. `tools/import_cards.py` reads the TCGdex API and writes `data/cards.json`, using the standard library only, so the crate carries no dependency for a crawl someone runs by hand.

The artifact holds every card in Standard — regulation marks H, I, and J, 3051 cards on the day of the crawl — and carries a `schema` field so a reader can refuse a shape it does not know. It keeps what a rules engine reads: identity, category, mark, stage, evolution, HP, types, retreat, weakness, resistance, attacks, abilities, and Trainer and Energy kinds. It drops prices, images, variants, rarity, and illustrator, which change often and which the engine never reads.

## Consequences

The engine is a pure function of its state, its seed, and its actions, which makes a replay and a test the same thing.

The crawl is a command someone runs, not a build step, so the artifact is committed and its age is visible in the file. Re-running it takes about 95 seconds over 3051 requests.

TCGdex cannot express two things the rules need, and both are recorded in [card data findings](../architecture/card-data.md): a Mega ex is indistinguishable from an ordinary ex except by the "Mega " in its name, and no field marks an ACE SPEC, so deck construction cannot enforce the one-per-deck rule from this data alone. Neither blocks the engine today, and both need a second source or a hand-kept list.
