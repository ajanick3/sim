# Effort: UI and domain review

A review session, not a build. It checked two things and opened one more.

## What it checked

**`packages/engine-client`.** ADR 0104 already covers this package. A
read of its source confirms the design holds: `wire/` and
`selection-flow/` stay one-directional, `typecheck` and `test` both pass.
No change needed. `web/` and `packages/chatgpt` still carry their own
duplicate copies, on purpose, per the ADR — migrating them is separate,
future work, not started here.

**`web/`'s framework.** It already runs Next.js with client-side
rendering: almost every component under `app/` is marked `"use client"`,
and `next.config.mjs` sets no server rendering or static-export option.
No migration is needed to get plain React with client rendering — the
repository already has it.

## What it opened

A read of `src/state.rs` and `src/engine.rs` turned into a full map of
every zone a card can be in (Deck, Hand, Discard, Prizes, Active, Bench,
Stadium, and the evolution stack and attachments that ride on a
Pokémon), plus the per-card and per-turn state that is not a zone
(damage, Special Conditions, the once-a-turn flags). The glossary now
carries a **Zone** entry from this ([docs/architecture/glossary.md](../../docs/architecture/glossary.md))
distinguishing the plain sense from `card::Zone`, the narrow Rust type
that covers only Hand/Discard/Deck.

## Left for later

The visual/UX complaint that started this ("the UI is so bad") is still
open. The review ruled out the framework as the cause; the actual
visual and interaction review has not started. Pick a starting zone or
screen next session — the zone map above is the reference to work from.
