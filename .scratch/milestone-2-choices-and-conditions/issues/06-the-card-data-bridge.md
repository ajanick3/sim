# The card data bridge

Type: research
Status: resolved

Decide the JSON format that carries card data from `pkmn` to this engine.

The export script belongs to `pkmn`, which owns the schema and the
credentials. Nothing needs this until the engine outgrows literal cards, so it
waits on the operator to say when.

- [x] A format, with a version field
- [x] A decision on what the engine reads at startup and what it ignores

## Answer

Resolved 2026-09-06 on branch `feat/import-card-data`, commit `f04f2ce`.

The operator chose TCGdex over the `pkmn` Turso database, which removes the
credentials that put the export script in that repository. The decision is
[ADR 0007](../../../docs/adr/0007-card-data-comes-from-tcgdex-into-this-repository.md),
which supersedes ADR 0002 and carries the whole seam decision, the engine's
purity included.

`tools/import_cards.py` writes `data/cards.json`: 3051 cards of marks H, I, and
J, in about 95 seconds, using the standard library only. The artifact carries a
`schema` field. It keeps identity, category, mark, stage, evolution, HP, types,
retreat, weakness, resistance, attacks, abilities, and the Trainer and Energy
kinds; it drops prices, images, variants, rarity, and illustrator.

Three things the crawl settled that a reader must handle, recorded in
[card data findings](../../../docs/architecture/card-data.md): attack damage is
an integer for 2455 attacks and a string such as `30+` or `60×` for 734; an
attack cost is a list of type names where `Colorless` means any Energy; and one
card carried a lower-case mark, which the importer now upper-cases.

Two things the data cannot express, both needing a second source: a Mega ex is
indistinguishable from an ordinary ex except by the `Mega ` in its name, and
nothing marks an ACE SPEC.

The engine does not read the artifact yet — that is ticket 07.
