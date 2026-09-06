# A card's raw record is kept as `serde_json::Value`, not a typed struct

**Status:** Accepted — 2026-09-06

`load` kept the printed fields of the 274 cards the engine could play and dropped everything else at load, so a refused card survived only as its id, its name, and the reason. That was right for playing a game and wrong for anything around one: a deck builder, a card browser, and a legality check all need the printed fields of a card the engine cannot yet run.

Two shapes were live for keeping them. A typed record, mirroring `Attack` and `Pokemon`, parses once and can refuse a malformed card at load — but it has to model every field a Trainer or a special Energy carries, most of which the engine has no use for today, and it grows every time the artifact's shape does. A raw `serde_json::Value` per card costs nothing to write, since the crate already depends on `serde_json` for `load` itself, and it needs no maintenance when the artifact adds a field this project has not yet decided to model.

The raw value was chosen. No caller of it exists yet — the spec's own fog named that as the condition under which this effort should close instead of build — but the two acceptance criteria that do have a concrete answer (a refused card's text is readable; the admitted cards and the coverage count are unchanged) are satisfied by the simpler shape, and a typed record would answer them no better while costing more to keep current.

## Consequences

`CardRef.raw` holds the card's record exactly as the artifact printed it, for every one of the 3051 cards, whether or not the engine can run it. Reading `raw["effect"]` or `raw["hp"]` costs a lookup and a type check per field.

The memory cost is measured, not guessed: `cargo run --release --bin memcheck` reports **+32,408 KiB resident** to load and keep the raw record of all 3051 cards, against a re-serialized text size of 1.46 MB — the difference is `serde_json::Value`'s own per-node overhead (a `String` allocation and an enum tag for every field, not a flat byte count), which a typed struct would not carry at this scale but would trade for a maintenance cost this project has not yet earned back with a caller.

Nothing about a card's admission changed. A card plays only when the engine can run all of it — ADR 0008 decided that, and `raw` does not revisit it: refused cards stay refused.
