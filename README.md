# sim

A Pokémon TCG rules engine, written in Rust.

The engine is pure. It holds no I/O, no async, and no clock, so a game is a
value: give it a seed and a list of actions and it replays exactly.

```sh
cargo run      # play a game in the terminal
cargo test     # run the engine's tests

cargo run --release --bin selfplay -- 5000        # headless games
cargo run --release --bin selfplay -- 5000 views  # the same, through masked views
```

## Scope

The engine is deep and the card set is tiny. Three synthetic Basic Pokémon and
their Energy live in `src/cards.rs` as literals. This is deliberate — see
[ADR 0004](docs/adr/0004-a-deep-engine-and-a-tiny-card-set.md).

What the engine does today: setup with mulligans, the turn loop, the damage
order, Energy costs by type, the five Special Conditions and the Pokémon
Checkup, knockouts, Prizes, the three win conditions, evolution, every kind
of Trainer, Pokémon Abilities, Pokémon attacks with their own effects, and a
growing set of Special Energy.

## The code

| Path            | Holds                                                     |
| --------------- | --------------------------------------------------------- |
| `src/state.rs`  | The whole game as one value, and how a game is dealt      |
| `src/engine.rs` | Applying an action: the damage order, knockouts, the turn |
| `src/action.rs` | `legal_actions`, the engine's interface                   |
| `src/card.rs`   | What a printed card says                                  |
| `src/cards.rs`  | The card set, as literals                                 |
| `src/ids.rs`    | The typed indices into the arenas                         |
| `src/rng.rs`    | A seeded generator, and a scripted one for tests          |
| `src/view.rs`   | What one player is allowed to see                         |
| `src/import.rs` | Reading the card artifact, and refusing what it cannot run |
| `src/decklist.rs` | Reading a decklist, and checking deck construction       |
| `src/main.rs`   | The text interface                                        |
| `src/bin/`      | `selfplay`, `coverage`, `deckcheck`, `blockers`, `progress_table` |

## Card data

`data/cards.json` holds every Standard card — regulation marks H, I, and J.
`tools/import_cards.py` writes it from the TCGdex API and needs no
credentials:

```sh
python3 tools/import_cards.py        # about 95 seconds, 3051 cards
```

The engine reads the artifact, and admits only the cards it can run all of:

```sh
cargo run --bin coverage             # 1004 of 3051 Standard cards (32.9%)
cargo run --bin coverage -- refused  # every refused card, and why
cargo run --bin deckcheck -- decks/brent-tonisson.txt  # check a decklist
```

A card it cannot run is refused by name and reason, never half-loaded. Its own
games still use the literals in `src/cards.rs`.

## Decks

`decks/` holds decklists in the format the official client exports, one file
per deck, named for its player in lower-case words joined by dashes. A test
reads every one of them and checks that it parses, matches, and is legal, so a
change to the parser or the card data fails loudly rather than quietly.

`decks/2026-worlds/` holds the field of the 2026 World Championships, one file
per player, named `<placement>-<player-slug>.txt`, zero-padded to three
digits. `tools/fetch_worlds_decklists.py` fetches all 143 from limitlesstcg's
Decklists tab, the tournament's Day 2 standings — the site holds no decklist
for an entrant who did not reach one. All 143 are kept and check clean.

### Standard coverage

Every card in the artifact, by name. The Trainer and Special Energy tables below cover the whole pool; the Pokémon table stays scoped to the field (the decks under `decks/`).

| Kind | Built | Total |
| --- | --- | --- |
| Supporters | 73 | 78 |
| Items | 62 | 85 |
| Tools | 26 | 35 |
| Stadiums | 25 | 31 |
| Special Energy | 15 | 17 |

### Supporters (73/78 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1358) | ✅ |
| [Acerola's Mischief](src/import.rs#L1560) | ✅ |
| [Amarys](src/import.rs#L812) | ✅ |
| Anthea & Concordia | ❌ — not yet triaged |
| [Bianca's Devotion](src/import.rs#L1343) | ✅ |
| [Billy & O'Nare](src/import.rs#L799) | ✅ |
| [Black Belt's Training](src/import.rs#L1366) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L921) | ✅ |
| [Brock's Scouting](src/import.rs#L1428) | ✅ |
| [Canari](src/import.rs#L758) | ✅ |
| [Caretaker](src/import.rs#L1659) | ✅ |
| [Carmine](src/import.rs#L715) | ✅ |
| [Cassiopeia](src/import.rs#L776) | ✅ |
| [Cheren](src/import.rs#L234) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1297) | ✅ |
| [Clemont's Quick Wit](src/import.rs#L913) | ✅ |
| [Colress's Tenacity](src/import.rs#L1146) | ✅ |
| [Cook](src/import.rs#L380) | ✅ |
| [Crispin](src/import.rs#L1203) | ✅ |
| [Cyrano](src/import.rs#L1051) | ✅ |
| [Dawn](src/import.rs#L1169) | ✅ |
| [Drasna](src/import.rs#L808) | ✅ |
| [Drayton](src/import.rs#L858) | ✅ |
| [Emcee's Hype](src/import.rs#L791) | ✅ |
| [Emma](src/import.rs#L807) | ✅ |
| [Eri](src/import.rs#L1421) | ✅ |
| [Ethan's Adventure](src/import.rs#L452) | ✅ |
| [Explorer's Guidance](src/import.rs#L844) | ✅ |
| [Fennel](src/import.rs#L909) | ✅ |
| [Firebreather](src/import.rs#L744) | ✅ |
| [Friends in Paldea](src/import.rs#L234) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1374) | ✅ |
| Grimsley's Move | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Gwynn](src/import.rs#L1065) | ✅ |
| [Harlequin](src/import.rs#L893) | ✅ |
| [Hassel](src/import.rs#L830) | ✅ |
| [Hilda](src/import.rs#L1122) | ✅ |
| [Iris's Fighting Spirit](src/import.rs#L905) | ✅ |
| [Jacinthe](src/import.rs#L693) | ✅ |
| [Janine's Secret Art](src/import.rs#L1452) | ✅ |
| [Jasmine's Gaze](src/import.rs#L390) | ✅ |
| [Jett](src/import.rs#L790) | ✅ |
| [Judge](src/import.rs#L944) | ✅ |
| [Kieran](src/import.rs#L1406) | ✅ |
| [Kofu](src/import.rs#L1651) | ✅ |
| [Lacey](src/import.rs#L885) | ✅ |
| [Lana's Aid](src/import.rs#L1323) | ✅ |
| [Larry's Skill](src/import.rs#L1378) | ✅ |
| [Lillie's Determination](src/import.rs#L945) | ✅ |
| [Lisia's Appeal](src/import.rs#L1554) | ✅ |
| Lt. Surge's Bargain | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Lucian](src/import.rs#L881) | ✅ |
| [Misty's Vitality](src/import.rs#L700) | ✅ |
| [Morty's Conviction](src/import.rs#L1416) | ✅ |
| [N's Plan](src/import.rs#L1341) | ✅ |
| [Naveen](src/import.rs#L904) | ✅ |
| [Perrin](src/import.rs#L1655) | ✅ |
| [Philippe](src/import.rs#L730) | ✅ |
| [Picnicker](src/import.rs#L714) | ✅ |
| [Pokémon Center Lady](src/import.rs#L1342) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1344) | ✅ |
| [Roxie's Performance](src/import.rs#L400) | ✅ |
| Ruffian | ❌ — needs a target-then-discard-both phase, scoped to one Pokémon chosen up front |
| [Rust Syndicate Grunt](src/import.rs#L1337) | ✅ |
| [Salvatore](src/import.rs#L1553) | ✅ |
| [Surfer](src/import.rs#L1362) | ✅ |
| [Tarragon](src/import.rs#L716) | ✅ |
| [Team Rocket's Archer](src/import.rs#L772) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1514) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1522) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1227) | ✅ |
| [Team Rocket's Proton](src/import.rs#L466) | ✅ |
| Tyme | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Urbain](src/import.rs#L379) | ✅ |
| [Waitress](src/import.rs#L816) | ✅ |
| [Wally's Compassion](src/import.rs#L1451) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1420) | ✅ |

### Items (62/85 built)

| Card | Status |
| --- | --- |
| Accompanying Flute | ❌ — not yet triaged |
| Antique Armor Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Cover Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Jaw Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Plume Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Root Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Sail Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Skull Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Arven's Sandwich | ❌ — not yet triaged |
| [Awakening Drum](src/import.rs#L406) | ✅ |
| Blowtorch | ❌ — not yet triaged |
| [Boxed Order](src/import.rs#L637) | ✅ |
| [Brilliant Blender](src/import.rs#L581) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L1037) | ✅ |
| [Bug Catching Set](src/import.rs#L1283) | ✅ |
| Call Bell | ❌ — needs the "playable on the first turn" allowance |
| Chill Teaser Toy | ❌ — needs the "playable on the first turn" allowance |
| [Crushing Hammer](src/import.rs#L980) | ✅ |
| [Dangerous Laser](src/import.rs#L569) | ✅ |
| [Dark Bell](src/import.rs#L576) | ✅ |
| Deduction Kit | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Dragon Elixir](src/import.rs#L680) | ✅ |
| [Dusk Ball](src/import.rs#L1482) | ✅ |
| [Energy Coin](src/import.rs#L435) | ✅ |
| [Energy Recycler](src/import.rs#L1637) | ✅ |
| [Energy Retrieval](src/import.rs#L1467) | ✅ |
| [Energy Search](src/import.rs#L1453) | ✅ |
| [Energy Search Pro](src/import.rs#L995) | ✅ |
| Energy Swatter | ❌ — not yet triaged |
| [Energy Switch](src/import.rs#L1107) | ✅ |
| [Enhanced Hammer](src/import.rs#L925) | ✅ |
| [Fighting Gong](src/import.rs#L1488) | ✅ |
| [Glass Trumpet](src/import.rs#L926) | ✅ |
| Great Haul Net | ❌ — not yet triaged |
| [Hand Trimmer](src/import.rs#L1481) | ✅ |
| [Hole-Digging Shovel](src/import.rs#L381) | ✅ |
| [Hop's Bag](src/import.rs#L480) | ✅ |
| [Hyper Aroma](src/import.rs#L609) | ✅ |
| [Iron Defender](src/import.rs#L394) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1319) | ✅ |
| Love Ball | ❌ — not yet triaged |
| [Lumiose Galette](src/import.rs#L679) | ✅ |
| [Master Ball](src/import.rs#L595) | ✅ |
| [Max Rod](src/import.rs#L651) | ✅ |
| Meddling Memo | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Mega Signal](src/import.rs#L1009) | ✅ |
| Megaton Blower | ❌ — not yet triaged |
| [Miracle Headset](src/import.rs#L665) | ✅ |
| [N's PP Up](src/import.rs#L1241) | ✅ |
| [Night Stretcher](src/import.rs#L952) | ✅ |
| Ogre's Mask | ❌ — not yet triaged |
| [Poké Ball](src/import.rs#L421) | ✅ |
| [Poké Pad](src/import.rs#L966) | ✅ |
| [Poké Vital A](src/import.rs#L689) | ✅ |
| [Pokégear 3.0](src/import.rs#L1269) | ✅ |
| [Pokémon Catcher](src/import.rs#L1484) | ✅ |
| [Potion](src/import.rs#L684) | ✅ |
| [Precious Trolley](src/import.rs#L981) | ✅ |
| [Premium Power Pro](src/import.rs#L1370) | ✅ |
| [Prime Catcher](src/import.rs#L1483) | ✅ |
| [Rare Candy](src/import.rs#L1226) | ✅ |
| [Reboot Pod](src/import.rs#L410) | ✅ |
| Redeemable Ticket | ❌ — not yet triaged |
| [Repel](src/import.rs#L580) | ✅ |
| Roto-Stick | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Sacred Ash](src/import.rs#L1079) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1555) | ✅ |
| Scramble Switch | ❌ — not yet triaged |
| [Secret Box](src/import.rs#L1586) | ✅ |
| [Special Red Card](src/import.rs#L1199) | ✅ |
| [Strange Timepiece](src/import.rs#L1502) | ✅ |
| [Super Potion](src/import.rs#L688) | ✅ |
| [Switch](src/import.rs#L1318) | ✅ |
| [TM Machine](src/import.rs#L1023) | ✅ |
| Team Rocket's Bother-Bot | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Team Rocket's Great Ball](src/import.rs#L1526) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1623) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1556) | ✅ |
| [Tera Orb](src/import.rs#L1108) | ✅ |
| [Tool Scrapper](src/import.rs#L920) | ✅ |
| [Transformation Tome](src/import.rs#L1582) | ✅ |
| [Treasure Tracker](src/import.rs#L623) | ✅ |
| [Ultra Ball](src/import.rs#L1093) | ✅ |
| [Unfair Stamp](src/import.rs#L1311) | ✅ |
| [Wondrous Patch](src/import.rs#L1255) | ✅ |

### Tools (26/35 built)

| Card | Status |
| --- | --- |
| [Adversity Policy](src/import.rs#L568) | ✅ |
| [Air Balloon](src/import.rs#L1503) | ✅ |
| Amulet of Hope | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Babiri Berry](src/import.rs#L530) | ✅ |
| Backtrack Badge | ❌ — not yet triaged |
| [Binding Mochi](src/import.rs#L1506) | ✅ |
| [Brave Bangle](src/import.rs#L1505) | ✅ |
| [Colbur Berry](src/import.rs#L534) | ✅ |
| Core Memory | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Counter Gain](src/import.rs#L420) | ✅ |
| [Cynthia's Power Weight](src/import.rs#L494) | ✅ |
| Deluxe Bomb | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Gravity Gemstone](src/import.rs#L412) | ✅ |
| [Haban Berry](src/import.rs#L550) | ✅ |
| [Handheld Fan](src/import.rs#L1510) | ✅ |
| Heavy Baton | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Hero's Cape](src/import.rs#L1504) | ✅ |
| Hop's Choice Band | ❌ — not yet triaged |
| [Light Ball](src/import.rs#L383) | ✅ |
| [Lillie's Pearl](src/import.rs#L1507) | ✅ |
| [Lucky Helmet](src/import.rs#L1509) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Occa Berry](src/import.rs#L538) | ✅ |
| [Passho Berry](src/import.rs#L542) | ✅ |
| [Payapa Berry](src/import.rs#L546) | ✅ |
| [Powerglass](src/import.rs#L1511) | ✅ |
| [Punk Helmet](src/import.rs#L1508) | ✅ |
| [Rescue Board](src/import.rs#L389) | ✅ |
| [Sacred Charm](src/import.rs#L387) | ✅ |
| [Sparkling Crystal](src/import.rs#L416) | ✅ |
| Survival Brace | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Team Rocket's Hypnotizer](src/import.rs#L561) | ✅ |
| Technical Machine: Fluorite | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Thick Scale](src/import.rs#L554) | ✅ |
| Tremendous Bomb | ❌ — not yet triaged |

### Stadiums (25/31 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1564) | ✅ |
| Ange Floette | ❌ — not yet triaged |
| [Area Zero Underdepths](src/import.rs#L941) | ✅ |
| [Battle Cage](src/import.rs#L942) | ✅ |
| Celebratory Fanfare | ❌ — not yet triaged |
| [Community Center](src/import.rs#L1571) | ✅ |
| Dizzying Valley | ❌ — not yet triaged |
| [Festival Grounds](src/import.rs#L1578) | ✅ |
| [Forest of Vitality](src/import.rs#L1577) | ✅ |
| Fossil Quarry | ❌ — not yet triaged |
| [Full Metal Lab](src/import.rs#L526) | ✅ |
| Grand Tree | ❌ — not yet triaged |
| [Granite Cave](src/import.rs#L498) | ✅ |
| [Gravity Mountain](src/import.rs#L1512) | ✅ |
| [Jamming Tower](src/import.rs#L1575) | ✅ |
| [Levincia](src/import.rs#L510) | ✅ |
| [Lively Stadium](src/import.rs#L388) | ✅ |
| [Lumiose City](src/import.rs#L1569) | ✅ |
| [Mystery Garden](src/import.rs#L518) | ✅ |
| [N's Castle](src/import.rs#L1513) | ✅ |
| Neutralization Zone | ❌ — not yet triaged |
| [Nighttime Mine](src/import.rs#L940) | ✅ |
| [Paradise Resort](src/import.rs#L506) | ✅ |
| [Perilous Jungle](src/import.rs#L411) | ✅ |
| [Postwick](src/import.rs#L502) | ✅ |
| [Prism Tower](src/import.rs#L1570) | ✅ |
| [Risky Ruins](src/import.rs#L1576) | ✅ |
| [Spikemuth Gym](src/import.rs#L514) | ✅ |
| [Surfing Beach](src/import.rs#L522) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1565) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L943) | ✅ |

### Special Energy (15/17 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L2028) | ✅ |
| [Bubbly Water Energy](src/import.rs#L2040) | ✅ |
| [Enriching Energy](src/import.rs#L2011) | ✅ |
| [Growing Grass Energy](src/import.rs#L2010) | ✅ |
| [Ignition Energy](src/import.rs#L2052) | ✅ |
| Legacy Energy | ❌ — its Prize-count clause needs the deferred Prize-count mechanic |
| [Magnetic Metal Energy](src/import.rs#L2037) | ✅ |
| [Mist Energy](src/import.rs#L2025) | ✅ |
| [Neo Upper Energy](src/import.rs#L2058) | ✅ |
| [Nitro Fire Energy](src/import.rs#L2043) | ✅ |
| [Prism Energy](src/import.rs#L2031) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L2034) | ✅ |
| [Shadowy Darkness Energy](src/import.rs#L2049) | ✅ |
| [Spiky Energy](src/import.rs#L2022) | ✅ |
| Team Rocket's Energy | ❌ — attaches only to a Team Rocket's Pokémon and discards itself instantly off any other — nothing validates an attach against the carrier's identity yet |
| [Telepathic Psychic Energy](src/import.rs#L2014) | ✅ |
| [Voltaic Lightning Energy](src/import.rs#L2046) | ✅ |

### Pokémon (field) (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2245) | [✅](src/import.rs#L2245) | [✅](src/import.rs#L2127) |
| [Alakazam](src/import.rs#L2400) | [✅](src/import.rs#L2400) | [✅](src/import.rs#L2117) |
| [Annihilape](src/import.rs#L2274) | [✅](src/import.rs#L2274) | [✅](src/import.rs#L2083) |
| [Applin](src/import.rs#L2235) | [✅](src/import.rs#L2235) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2277) | [✅](src/import.rs#L2277) | — |
| [Beldum](src/import.rs#L2257) | [✅](src/import.rs#L2257) | — |
| [Blaziken ex](src/import.rs#L2341) | [✅](src/import.rs#L2341) | [✅](src/import.rs#L2157) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2305) | [✅](src/import.rs#L2305) | [✅](src/import.rs#L2091) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2216) | [✅](src/import.rs#L2216) | — |
| [Budew](src/import.rs#L2282) | [✅](src/import.rs#L2282) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2276) | [✅](src/import.rs#L2276) | — |
| [Carvanha](src/import.rs#L2194) | [✅](src/import.rs#L2194) | — |
| [Celebi](src/import.rs#L2275) | [✅](src/import.rs#L2275) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2373) | [✅](src/import.rs#L2373) | — |
| [Chien-Pao](src/import.rs#L2342) | [✅](src/import.rs#L2342) | [✅](src/import.rs#L2160) |
| [Chikorita](src/import.rs#L2278) | [✅](src/import.rs#L2278) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2323) | [✅](src/import.rs#L2323) | — |
| [Combusken](src/import.rs#L2292) | [✅](src/import.rs#L2292) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2412) | [✅](src/import.rs#L2412) | [✅](src/import.rs#L2072) |
| [Dedenne](src/import.rs#L2232) | [✅](src/import.rs#L2232) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2357) | [✅](src/import.rs#L2357) | [✅](src/import.rs#L2145) |
| [Dragapult ex](src/import.rs#L2239) | [✅](src/import.rs#L2239) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2100) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2246) | [✅](src/import.rs#L2246) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2124) |
| [Dudunsparce ex](src/import.rs#L2207) | [✅](src/import.rs#L2207) | — |
| [Dunsparce](src/import.rs#L2258) | [✅](src/import.rs#L2258) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2128) |
| [Dusknoir](src/import.rs#L2339) | [✅](src/import.rs#L2339) | [✅](src/import.rs#L2129) |
| [Duskull](src/import.rs#L2261) | [✅](src/import.rs#L2261) | — |
| [Dwebble](src/import.rs#L2251) | [✅](src/import.rs#L2251) | — |
| [Elgyem](src/import.rs#L2281) | [✅](src/import.rs#L2281) | — |
| [Enamorus](src/import.rs#L2301) | [✅](src/import.rs#L2301) | — |
| [Fan Rotom](src/import.rs#L2346) | [✅](src/import.rs#L2346) | [✅](src/import.rs#L2170) |
| [Fezandipiti ex](src/import.rs#L2420) | [✅](src/import.rs#L2420) | [✅](src/import.rs#L2118) |
| [Flutter Mane](src/import.rs#L2337) | [✅](src/import.rs#L2337) | [✅](src/import.rs#L2094) |
| [Genesect](src/import.rs#L2392) | [✅](src/import.rs#L2392) | [✅](src/import.rs#L2133) |
| [Genesect ex](src/import.rs#L2340) | [✅](src/import.rs#L2340) | [✅](src/import.rs#L2130) |
| [Goldeen](src/import.rs#L2355) | [✅](src/import.rs#L2355) | [✅](src/import.rs#L2143) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2248) | [✅](src/import.rs#L2248) | [✅](src/import.rs#L2084) |
| [Hydrapple ex](src/import.rs#L2306) | [✅](src/import.rs#L2306) | [✅](src/import.rs#L2085) |
| [Iron Crown ex](src/import.rs#L2266) | [✅](src/import.rs#L2266) | [✅](src/import.rs#L2080) |
| [Iron Leaves ex](src/import.rs#L2345) | [✅](src/import.rs#L2345) | [✅](src/import.rs#L2161) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2113) |
| [Koraidon ex](src/import.rs#L2267) | [✅](src/import.rs#L2267) | — |
| [Kyurem](src/import.rs#L2389) | [✅](src/import.rs#L2389) | [✅](src/import.rs#L2152) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2406) | [✅](src/import.rs#L2406) | [✅](src/import.rs#L2071) |
| [Lillie's Clefairy ex](src/import.rs#L2430) | [✅](src/import.rs#L2430) | [✅](src/import.rs#L2075) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2313) | [✅](src/import.rs#L2313) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2309) | [✅](src/import.rs#L2309) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2416) | [✅](src/import.rs#L2416) | [✅](src/import.rs#L2068) |
| [Mega Lopunny ex](src/import.rs#L2214) | [✅](src/import.rs#L2214) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2249) | [✅](src/import.rs#L2249) | — |
| [Mega Skarmory ex](src/import.rs#L2327) | [✅](src/import.rs#L2327) | — |
| [Mega Slowbro ex](src/import.rs#L2367) | [✅](src/import.rs#L2367) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2139) |
| [Meowth ex](src/import.rs#L2419) | [✅](src/import.rs#L2419) | [✅](src/import.rs#L2112) |
| [Metagross](src/import.rs#L2223) | [✅](src/import.rs#L2223) | — |
| [Metang](src/import.rs#L2407) | [✅](src/import.rs#L2407) | [✅](src/import.rs#L2103) |
| [Moltres](src/import.rs#L2259) | [✅](src/import.rs#L2259) | — |
| [Munkidori](src/import.rs#L2413) | [✅](src/import.rs#L2413) | [✅](src/import.rs#L2106) |
| [N's Darmanitan](src/import.rs#L2204) | [✅](src/import.rs#L2204) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2201) | [✅](src/import.rs#L2201) | — |
| [N's Zekrom](src/import.rs#L2213) | [✅](src/import.rs#L2213) | — |
| [N's Zoroark ex](src/import.rs#L2386) | [✅](src/import.rs#L2386) | [✅](src/import.rs#L2136) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2409) | [✅](src/import.rs#L2409) | [✅](src/import.rs#L2114) |
| [Paldean Tauros](src/import.rs#L2197) | [✅](src/import.rs#L2197) | — |
| [Passimian](src/import.rs#L2210) | [✅](src/import.rs#L2210) | — |
| [Patrat](src/import.rs#L2408) | [✅](src/import.rs#L2408) | [✅](src/import.rs#L2073) |
| [Pecharunt](src/import.rs#L2350) | [✅](src/import.rs#L2350) | [✅](src/import.rs#L2140) |
| [Pecharunt ex](src/import.rs#L2347) | [✅](src/import.rs#L2347) | [✅](src/import.rs#L2177) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2074) |
| [Rabsca](src/import.rs#L2263) | [✅](src/import.rs#L2263) | [✅](src/import.rs#L2079) |
| [Raging Bolt ex](src/import.rs#L2228) | [✅](src/import.rs#L2228) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L2195) | [✅](src/import.rs#L2195) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2356) | [✅](src/import.rs#L2356) | [✅](src/import.rs#L2144) |
| [Shaymin](src/import.rs#L2403) | [✅](src/import.rs#L2403) | [✅](src/import.rs#L2078) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2252) | [✅](src/import.rs#L2252) | — |
| [Slowpoke](src/import.rs#L2260) | [✅](src/import.rs#L2260) | ❌ |
| [Smoochum](src/import.rs#L2320) | [✅](src/import.rs#L2320) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2302) | [✅](src/import.rs#L2302) | — |
| [Tapu Bulu](src/import.rs#L2196) | [✅](src/import.rs#L2196) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2097) |
| [Teal Mask Ogerpon ex](src/import.rs#L2421) | [✅](src/import.rs#L2421) | [✅](src/import.rs#L2121) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2146) |
| [Torchic](src/import.rs#L2262) | [✅](src/import.rs#L2262) | — |
| [Toxel](src/import.rs#L2247) | [✅](src/import.rs#L2247) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2164) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2237) | [✅](src/import.rs#L2237) | — |
| [Yveltal](src/import.rs#L2236) | [✅](src/import.rs#L2236) | — |
| [Zeraora](src/import.rs#L2219) | [✅](src/import.rs#L2219) | — |
| Zoroark | ❌ | — |


## The documents

- [The domain glossary](docs/architecture/glossary.md) — the vocabulary.
- [The effect vocabulary](docs/architecture/effects.md) — what a Trainer has
  to be able to say, counted from the decks in `decks/`.
- [The base rules](docs/architecture/rules.md) — the numbered rules the code
  cites, and the cards that break a naive engine.
- [Card data sources](docs/architecture/sources.md) — where card data comes
  from, and what each source gets wrong.
- [Card data findings](docs/architecture/card-data.md) — what the imported
  artifact holds, the modelling findings already banked, and what the data
  cannot express.
- [The ADR directory](docs/adr/) — the decisions and their reasoning.

