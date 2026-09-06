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
  print-id → implementation lookup, not by print id.

## The imported artifact

`data/cards.json` holds every Standard card, imported from TCGdex by
`tools/import_cards.py`. The crawl of 2026-09-05 read 3051 cards: 1303 with
mark H, 1278 with I, and 470 with J. Of those, 2585 are Pokémon, 445 are
Trainers, and 21 are Energy; 2587 carry an attack and 589 an ability.

Three shapes a reader must handle:

- **Damage is not always a number.** Of the attacks that deal damage, 2455
  carry an integer and 734 carry a string: `30+`, `60×`, `120-`. 642 attacks
  deal no damage at all.
- **An attack cost is a list of type names**, and `Colorless` means any Energy.
- **One card carried a lower-case mark** (`mep-051`). The importer upper-cases
  every mark, because legality reads it.

## Known problems in the crawled data

- Legality reads the **regulation mark** and nothing else. A `legal_standard`
  flag, wherever one appears, is stale or empty; the `pkmn` database carries
  one of each kind.
- Nothing in the data carries a **prize value**.
- **The `suffix` field is unreliable, and the name is not.** The suffix is
  absent on 21 ex cards, `Mega Charizard X ex` among them, and it is written
  both `ex` and `EX`. Every Pokémon carrying a suffix also ends in ` ex`, and
  21 more do: 559 by name against 538 by suffix, with no card contradicting the
  other. A Mega ex is separated only by the `Mega ` at the start of its name,
  and only for a Pokémon — `Mega Signal` is a Trainer.
- **TCGdex has no ACE SPEC field.** Deck construction rule 3 — one ACE SPEC per
  deck — cannot be enforced from this data. It needs a second source or a
  hand-kept list.
- The ability-coverage worry was **investigated and dismissed**: ability rate is
  a consistent ~22% across marks H (247/1086), I (254/1101), J (74/370).
  `abilities_json` being null usually means the card genuinely has no ability.
  There is nothing to fix here.

## The current era

Standard = regulation marks **H, I, J** — 3023 legal cards.
Series: **Mega Evolution**, 8 sets, 2025-09-25 → 2026-07-17:
`mee` (Mega Evolution Energy), `mep` (promos), `me01` MEG, `me02` PFL,
`me02.5` ASC, `me03` POR, `me04` CRI, `me05` PBL (Pitch Black).
