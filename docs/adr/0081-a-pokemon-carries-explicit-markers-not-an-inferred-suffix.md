# A Pokémon carries explicit markers, not an inferred suffix

**Status:** Accepted — 2026-09-09

## Context

Before this record, only one fact beyond a Pokémon's species was tracked at all: `prizes`, inferred from the printed name (`prizes_for`, ADR 0010) — ending in ` ex` is worth 2, starting with `Mega ` and ending in ` ex` is worth 3. TCGdex's own `suffix` field was already known unreliable for this (absent on 21 ex cards, and split across two differently-cased values, `ex` and `EX`), so the name was the only trustworthy source.

Tera is a third fact a print can carry, and the name cannot answer it at all: `Hydreigon ex`, `Pikachu ex`, `Terapagos ex`, `Lapras ex`, `Greninja ex`, `Koraidon ex`, and `Miraidon ex` each print both a Tera version and a plain one under the exact same name. Building `Nighttime Mine`, `Area Zero Underdepths`, `Glass Trumpet`, `Briar`, and `Tera Orb` — every one of them reads "is this Pokémon Tera" — needs a source of truth keyed by print, not species.

## Decision

`Pokemon` gains `markers: Vec<Marker>`, where `Marker` is `Ex`, `Mega`, or `Tera`. `ex` and `Mega` are still read from the printed name, unchanged in substance from `prizes_for`'s old logic — only the destination changed, from a `u32` straight to a marker. `Tera` is read from a fixed table, `TERA_PRINT_IDS`, cross-checked by hand against pkmncards.com's `is:tera` listing (150 cards) and matched against this artifact's own H/I/J-regulation pool: 104 of the 150 are present here. `prizes` is now computed *from* `markers` (`prizes_for(&markers)`), not derived independently — one fact, one source.

`TERA_PRINT_IDS` is a closed, permanent list. Pokémon TCG stopped printing new Tera cards after the sets this artifact already carries; the day that stops being true, this record is the one to revisit — the same clause ADR 0034 and ADR 0080 already carry for their own tables.

A "Trainer's Pokémon" marker (`N's Zoroark ex`, `Cynthia's Garchomp ex`) was considered and dropped: the restriction it would exist to enforce — evolving only from that same Trainer's own Basic — already holds for free. The artifact prints the full name in `evolveFrom` (`"N's Zorua"`, not `"Zorua"`), and evolution already matches by exact string equality, so no engine change was needed there at all.

## Consequences

`CardFilter::TeraPokemon` reads `markers.contains(&Marker::Tera)`, the same shape `PokemonEx` already reads `prizes > 1` by. `markers_for` and `prizes_for` are `pub` in `src/import.rs`, mirroring `attacks_read`/`ability_reads` — read directly by tests, rather than only reachable through a full artifact load, since most Tera-marked prints in the pool are not otherwise admitted yet (their own attacks or Abilities are unbuilt), so testing the marker itself cannot lean on `Import::admitted`.
