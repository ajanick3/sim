# Keep the record of every card

Type: task
Status: resolved

`load` drops the printed fields of the 2777 cards the engine cannot run. Keep
them, so a caller can read any card in the artifact.

The shape is the decision. A raw `serde_json::Value` per card costs nothing to
write and pushes the parsing onto every reader. A typed record parses once and
refuses malformed data at load, but it has to model fields the engine has no
use for, such as a Trainer's text and an Energy's kind.

Weigh the memory too: the artifact is 2.0 MB on disk, so keeping every record
roughly doubles what a load holds.

Whatever the shape, a refused card must stay refused. The engine plays a card
only when it can run all of it, which [ADR 0008](../../../docs/adr/0008-the-engine-refuses-a-card-it-cannot-run.md)
decided and this ticket does not revisit.

- [x] A recorded decision on the kept shape, raw or typed, and why
- [x] Any card in the artifact can be looked up by its id and its printed
      fields read
- [x] The admitted cards still play, and the coverage count does not move
- [x] The cost in memory, measured rather than guessed

## Answer

Resolved 2026-09-06 on branch `feat/keep-the-raw-card-record`.

The decision is [ADR 0011](../../../docs/adr/0011-a-cards-raw-record-is-kept-as-json.md):
a raw `serde_json::Value` per card, not a typed record. `CardRef` gains
`raw: Value`, the card exactly as the artifact printed it. Written test
first, `tests/card_records.rs`: a refused Supporter's `effect` text reads
back in full, an admitted card's `hp` reads back too, and coverage stays at
346/3051 — nothing about admission changed.

**The memory cost is measured, not guessed.** `cargo run --release --bin
memcheck` reports **+32,408 KiB resident** to load and keep the raw record
of all 3051 cards, against 1.46 MB of re-serialized text — the gap is
`serde_json::Value`'s own per-node overhead, which the ADR records as the
cost a typed record would trade for a maintenance burden nothing yet needs.

No caller of `raw` exists yet, which the map's own fog named as the
condition for closing this effort instead of building it. The two concrete
acceptance criteria — a refused card's text is readable, and admission is
unchanged — are answered either way, and the simpler shape answers them for
less.
