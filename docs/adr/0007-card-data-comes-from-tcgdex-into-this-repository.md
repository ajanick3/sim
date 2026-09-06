# Card data comes from TCGdex, crawled by a script in this repository

**Status:** Accepted — 2026-09-05

[ADR 0002](0002-pure-engine-with-a-json-seam.md) put the export script in the `pkmn` repository, because that repository owned Turso and the credentials for it. The operator now takes card data from TCGdex directly, which needs no credentials and no database, so the reason for the placement is gone. `tools/import_cards.py` reads the TCGdex API and writes `data/cards.json`, and it uses the standard library only, so the crate carries no dependency for a crawl it runs by hand.

ADR 0002's decision itself stands: the engine holds no I/O, and a JSON artifact is the seam. Only the source of that artifact and the home of the script have changed.

The artifact holds every card in Standard — regulation marks H, I, and J, 3051 cards on the day of the crawl — and carries a `schema` field so a reader can refuse a shape it does not know. It keeps what a rules engine reads: identity, category, mark, stage, evolution, HP, types, retreat, weakness, resistance, attacks, abilities, and Trainer and Energy kinds. It drops prices, images, variants, rarity, and illustrator, which change often and which the engine never reads.

## Consequences

The crawl is a command someone runs, not a build step, so the artifact is committed and its age is visible in the file. Re-running it takes about 95 seconds over 3051 requests.

TCGdex cannot express two things the rules need, and both are recorded in [card data findings](../architecture/card-data.md): a Mega ex is indistinguishable from an ordinary ex except by the "Mega " in its name, and no field marks an ACE SPEC, so deck construction cannot enforce the one-per-deck rule from this data alone. Neither blocks the engine today, and both need a second source or a hand-kept list.
