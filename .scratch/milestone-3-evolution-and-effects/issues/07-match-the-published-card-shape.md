# Match the published card shape

Type: task
Status: resolved

Read the importer against the [published card reference](https://tcgdex.dev/reference/card)
and close whatever it misses.

- [x] Every documented field is either kept or dropped on purpose
- [x] A card the engine cannot fully read is refused, not admitted in part
- [x] A decklist line is checked against the card its number names

## Answer

Resolved 2026-09-06 on branch `feat/match-the-documented-shape`.

The reference names two fields the importer dropped: `item`, a held item with
rules text of its own, and `level`. No Standard card carries either, so nothing
had been half-loaded — but a card with a held item would have been admitted
with its item ignored, which
[ADR 0008](../../../docs/adr/0008-the-engine-refuses-a-card-it-cannot-run.md)
forbids. The importer keeps both fields and the engine refuses a card with a
held item. The guard catches nothing today, which is the point of it.

The reference disagrees with the live API three times, recorded in
[card data findings](../../../docs/architecture/card-data.md): `abilities` and
`resistances` are missing from its table though 589 and 458 cards carry them;
`energyType` is documented `Basic`/`Special` and is live `Normal`/`Special`;
and `localId` is documented as a string or a number and is always a string.
Live data outranks the document.

The operator's point that a decklist carries only a name, a set, and a number
found a real gap. The checker resolved a line by set and number and ignored the
printed name, so a typo in a number matched the wrong card silently. The name
is now checked against the card the number names, comparing letters and digits
only, because an accent and an apostrophe vary between export tools. Both
committed decks pass with the check on: 45 names, all agreeing.
