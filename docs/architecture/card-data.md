# Card data: findings and known problems

What the crawled card data holds, what it cannot express, and the modelling
findings already banked. Do not re-research these.

## Modelling findings

- **Prize values.** `Pokémon ex rule` = **2 prizes**; `Mega Evolution ex Rule` =
  **3 prizes**. No V/VMAX/VSTAR/GX in the current pool — rotated out.
- **Prize count is computed at KO time, not a card property.** Cards adjust it:
  `Legacy Energy` (−1, once per game), `Lillie's Pearl` (−1), `Briar` (+1,
  conditional), `Anthea & Concordia` (+3), `Redeemable Ticket` (rewrites the
  prize pile). Model it as a mutable field on a knockout effect that starts at 1
  and is adjusted — NOT as a static column.
- **Subtype tags eventually needed:** `ex` (~497), `MEGA` (~103), `Tera` (~94),
  `Pokémon Tool` (~52), `Ancient` (~43), `Future` (~40), `ACE SPEC` (~33),
  `Special` (~22).
- **Card identity:** key implementations by **name + behaviour version** with a
  print-id → implementation lookup, not by print id. The `pkmn` repo hits the
  same problem with its canonical-card lookup.

## Known problems in the crawled data

- `sets.legal_standard` is `1` for **zero rows**; `cards.legal_standard` is
  populated but stale. **Do not trust either** — use regulation mark.
- `cards` has **no prize-value column** and no way to identify Mega ex.
- The ability-coverage worry was **investigated and dismissed**: ability rate is
  a consistent ~22% across marks H (247/1086), I (254/1101), J (74/370).
  `abilities_json` being null usually means the card genuinely has no ability.
  There is nothing to fix here.

## The current era

Standard = regulation marks **H, I, J** — 3023 legal cards.
Series: **Mega Evolution**, 8 sets, 2025-09-25 → 2026-07-17:
`mee` (Mega Evolution Energy), `mep` (promos), `me01` MEG, `me02` PFL,
`me02.5` ASC, `me03` POR, `me04` CRI, `me05` PBL (Pitch Black).
