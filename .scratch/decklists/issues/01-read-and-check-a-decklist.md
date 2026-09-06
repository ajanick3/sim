# Read and check a decklist

Type: task
Status: resolved

Parse the decklist text format and check the list against deck construction.

Rules 1, 2, and 4 are checkable from the artifact: exactly 60 cards, at most 4
by name with basic Energy exempt, and every card in a Standard regulation mark.
Rule 3, one ACE SPEC per deck, is not: TCGdex has no ACE SPEC field.

- [x] A line of the official export becomes a card in the artifact
- [x] A line that matches nothing is reported, never skipped
- [x] The three checkable deck construction rules are checked
- [x] The report says how many cards of the deck the engine can play
- [x] The uncheckable rule is named as uncheckable

## Answer

Resolved 2026-09-06 on branch `feat/decklist-parser`.

`src/decklist.rs` reads the export format and checks a list; `cargo run --bin
deckcheck -- <file>` prints the report. The importer now writes a set table
into the artifact, because a decklist names a set by its official
abbreviation — `MEG`, `TWM` — and TCGdex holds that per set. The artifact's
schema went to 2.

`Import` grew a `CardRef` for every card read, whether or not the engine can
play it, which is what a decklist matches against. That is a first step into
the [card-records](../../card-records/spec.md) effort's question, and it
answers only what this ticket needed: identity, set, number, mark, and whether
the card is playable. The printed fields are still dropped.

Tried against a real tournament list of 60 cards. Every one of its 21 Pokémon
and Trainer lines matched, across ten set codes, and the deck came back legal.
The engine can play 4 of its 60 cards.
