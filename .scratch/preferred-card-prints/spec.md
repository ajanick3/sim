# Preferred card prints

Status: ready-for-agent

## Problem Statement

Many cards in the catalog have more than one Print — the same Card
definition released under different set codes, with different art. A
Decklist line pins one exact Print (`4 Mega Venusaur ex MEG 3`), so
whichever Print a player happened to type is the one that renders
everywhere that card shows up: the deck builder, the board, the log.
A player who prefers a different Print's art has no way to see it —
the app shows them art they did not choose, every time that name comes
up, with no way to change it.

## Solution

A Settings page section lets a player browse the whole card catalog as
a grid, grouped by category then alphabetically, and tap a card to see
every known Print of it. Choosing one saves it as that player's
preferred Print for that card name, in this browser. From then on,
wherever the app renders that card's art, the preferred Print wins —
even over a Decklist line's own pinned Print. A card with only one
known Print shows as a disabled tile; there is nothing to prefer.
Nothing changes for a card until the player has chosen a preference
for it.

## User Stories

1. As a player, I want to open a grid of every card in the catalog, so
   that I can find a card I want to change the art of.
2. As a player, I want the grid grouped by category (Pokémon,
   Supporter, Item, Tool, Stadium, Special Energy, Energy) then
   alphabetically within each, so that it reads the same way my hand
   already does.
3. As a player, I want a card with only one known Print to show as a
   disabled tile, so that I don't waste a tap on a card with nothing
   to choose.
4. As a player, I want a card's tile to show the art I would currently
   see for it — my preference if I've set one, otherwise whatever this
   page falls back to — so that the grid previews what I'll actually
   see, not some arbitrary default.
5. As a player, I want to tap a multi-Print card and see every known
   Print of it, so that I can compare their art before choosing.
6. As a player, I want the picker to open with nothing highlighted, so
   that it never looks like I already made a choice I didn't make.
7. As a player, I want to tap a Print in the picker and have it become
   my preference immediately, so that I don't need a separate save
   step.
8. As a player, I want my preference to apply everywhere that card's
   art renders — the deck builder, the board, the log — so that I only
   set it once.
9. As a player, I want my preference to override a Decklist line's own
   pinned Print, so that typing an exact Print into a decklist (the
   client's export format) doesn't lock me out of my own preferred
   art.
10. As a player, I want a card I haven't set a preference for to keep
    showing whatever Print the current context already resolved to
    (the Decklist's pin, or the engine's own pick for a name-only
    lookup), so that nothing changes until I opt in.
11. As a player, I want my preferences to persist in this browser
    across sessions, so that I don't have to re-pick them every visit.
12. As a player, I want to search the catalog grid by name, so that I
    can find a card without scrolling the whole thing.

## Implementation Decisions

- **Scope: display only.** A Print never changes which Card definition
  a game plays. `CardDef::print_id()` and Decklist resolution
  (`src/decklist.rs`) are untouched; this whole feature lives in
  `web/`.
- **No new data pipeline.** `public/cards.json` already carries every
  Standard-legal Print (grouped only by shared `name`, per TCGdex's
  own data shape — there is no cross-Print id). `public/art-index.json`
  already resolves art by print id. Both are fetched today
  (`game-shell.tsx`); the catalog grid and picker read them the same
  way, client-side, with no server work.
- **Domain term added**: **Print**, defined in
  `docs/architecture/glossary.md` — one release of a Card definition,
  identified by its TCGdex print id, pinned independently wherever
  it's named. A shared card name is *not* enough to say two Prints
  share a Card definition (see Further Notes and `identityOf` below);
  `identityOf` is the real test.
- **Card identity, not bare name, is the grouping and storage key
  throughout** ("card name" below is shorthand for this): `identityOf`
  (`web/app/prints.ts`) fingerprints a card's name plus every
  rules-relevant field (stage, hp, types, retreat, weaknesses,
  resistances, attacks, abilities, trainerType, energyType, effect —
  excluding `id`/`set`/`localId` and flavor-only `description`). Two
  Prints share an identity only if they share all of that; two cards
  that merely share a `name` render as two separate catalog entries,
  each showing its own (also unrelated) art — no extra label needed to
  tell them apart, since the art already differs.
- **Storage**: a `localStorage` map from a card's identity to a chosen
  print id, alongside the existing `sim.recent` key, read/written
  through a small wrapped adapter (`loadPrintPrefs` / `savePrintPref`)
  matching the try/catch-and-degrade shape `loadRecent`/`clearRecent`
  already use in `web/app/settings/page.tsx`.
- **Grouping**: a new `CATALOG_ORDER` constant, its own symbol
  alongside `HAND_ORDER` rather than reusing it — the two start
  identical (pokémon, supporter, item, tool, stadium, special-energy,
  energy) but are allowed to drift; a hand's ordering needs and a
  catalog browse's needs are not guaranteed to stay the same thing.
  Within a category, the catalog additionally sorts alphabetically by
  name — a need `splitHandRows` never had, since a hand is small and
  unordered within a category.
- **Print resolution has three tiers, checked in order:**
  1. The player's stored preference for that card name, if set.
  2. The current context's own resolved Print — a Decklist line's
     pin, or whatever Print id the engine's name lookup already
     produced. (Outside the Settings page, this is always available;
     there is always a concrete Print already in play.)
  3. On the Settings page itself, where no live Decklist or game
     exists to supply tier 2, a deterministic fallback: the first
     Print by print id, sorted, for that card name. This tier applies
     to the grid tile's own thumbnail only — it does not touch the
     picker modal's highlight state (tier 3 is a display default, not
     an implied choice).
- **New pure logic**, landed in `web/app/prints.ts`:
  - `identityOf`, computing the card-identity fingerprint above.
  - `catalogEntries`, grouping the full card list by identity into
    category-then-name order for the grid, each entry carrying every
    Print id sharing that identity.
  - `resolvePrint`, the three-tier print choice above, given a card's
    identity, its known Prints, the stored preference map, and an
    optional context print id.
- **Settings page** (`web/app/settings/page.tsx`) grows a new section
  below Motion: a searchable grid of tiles (art via `artUrl`, one tile
  per distinct card name), each tile disabled when its name has only
  one known Print. Tapping an enabled tile opens a modal listing every
  Print for that name; tapping a Print in the modal saves it as the
  preference and closes (or updates) the modal with no prior
  selection highlighted.
- **Everywhere else art renders** (board, deck builder, log, wherever
  `artUrl` is currently called with a print id) threads the same
  three-tier resolution in front of the existing `artUrl(index,
  printId)` call, substituting the resolved print id before lookup.

## Testing Decisions

- Pure logic (grouping, Print-collection, three-tier resolution) gets
  direct vitest coverage in the module's own `.test.ts`, following the
  `web/app/board/shared.test.ts` pattern already used for
  `countersToPlace`, `splitHandRows`, `monHooks`: one `describe` block
  per function, each testing external behavior (given inputs, this
  output) with no DOM or storage involved.
- The `localStorage` adapter is a thin wrapper and stays untested in
  detail, matching how `loadRecent`/`clearRecent` are handled today —
  covered indirectly through whatever component test exercises the
  Settings page, if one exists, rather than a dedicated unit test of
  the storage calls themselves.
- No Rust tests are added; the engine and wasm boundary are untouched
  by this feature.
- No new wire fields — `WireView`/`WireCard` are untouched; this stays
  entirely inside `web/`, reading `public/cards.json` and
  `public/art-index.json` the app already fetches.

## Out of Scope

- Inline print-switching anywhere other than the Settings page (e.g.
  tapping a card's art directly on the board or in the deck builder to
  swap its Print). Noted as a possible v2, not part of this effort.
- Per-deck preferences — a preference is global to the browser, not
  saved with a Decklist.
- Any change to which Print a Decklist resolves to, or to deck
  legality/construction checking.
- Any change to the TCGdex import pipeline (`tools/import_cards.py`,
  `data/cards.json`) — no new fields, no variant fetching beyond what
  already lands there.
- Changing how the *engine* resolves a Decklist line's card name during
  deck import (`docs/adr/0020-a-trainer-name-is-matched-unless-a-print-overrides-it.md`'s
  `known_trainer_by_print` override table). Ticket 01 found that
  same-name-different-rules-text Prints are common, not rare (see
  Further Notes), and the catalog now groups by rules text instead of
  bare name to keep them apart in the picker — but that's a
  display-layer fix only, unrelated to how the engine picks a
  `CardDef` from a Decklist line's name.
- Scoping the catalog grid to only the cards a player has actually
  used (their saved decks / recent games). The grid always shows the
  whole catalog; a search box is the only narrowing tool.

## Further Notes

- `public/cards.json` relates cards only by the literal `name` string —
  this is TCGdex's own relation, not something this repository imposes
  (confirmed: TCGdex's API carries no shared "card" id across Prints).
  Ticket 01 found this relation is not enough on its own: of 889 names
  that share with another card in the catalog, 468 differ in some
  rules-relevant field (a name collision between two unrelated cards,
  e.g. two Pokémon both named "Abra" with 50 HP/no Ability and 40
  HP/an Ability) and only 421 are genuinely the same card reprinted.
  The catalog now groups by name **and** a rules-text fingerprint
  (`identityOf` in `web/app/prints.ts`) — 716 of 2035 resulting
  entries have 2+ known Prints once name collisions are split out;
  this is a lower bound, since `data/cards.json` only holds
  Standard-legal (regulation mark H/I/J) Prints — an older, rotated-out
  Print of a still-legal card is not in the file.
- This spec's design work happened as a `/grill-with-docs` session;
  the domain-modeling half of that session already landed the
  **Print** glossary term ahead of this spec.
