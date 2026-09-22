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
cargo run --bin coverage             # 978 of 3051 Standard cards (32.1%)
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
| Supporters | 70 | 78 |
| Items | 61 | 85 |
| Tools | 23 | 35 |
| Stadiums | 19 | 31 |
| Special Energy | 15 | 17 |

### Supporters (70/78 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1302) | ✅ |
| [Acerola's Mischief](src/import.rs#L1504) | ✅ |
| [Amarys](src/import.rs#L756) | ✅ |
| Anthea & Concordia | ❌ — not yet triaged |
| [Bianca's Devotion](src/import.rs#L1287) | ✅ |
| [Billy & O'Nare](src/import.rs#L743) | ✅ |
| [Black Belt's Training](src/import.rs#L1310) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L865) | ✅ |
| [Brock's Scouting](src/import.rs#L1372) | ✅ |
| [Canari](src/import.rs#L702) | ✅ |
| Caretaker | ❌ — not yet triaged |
| [Carmine](src/import.rs#L659) | ✅ |
| [Cassiopeia](src/import.rs#L720) | ✅ |
| [Cheren](src/import.rs#L234) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1241) | ✅ |
| [Clemont's Quick Wit](src/import.rs#L857) | ✅ |
| [Colress's Tenacity](src/import.rs#L1090) | ✅ |
| [Cook](src/import.rs#L380) | ✅ |
| [Crispin](src/import.rs#L1147) | ✅ |
| [Cyrano](src/import.rs#L995) | ✅ |
| [Dawn](src/import.rs#L1113) | ✅ |
| [Drasna](src/import.rs#L752) | ✅ |
| [Drayton](src/import.rs#L802) | ✅ |
| [Emcee's Hype](src/import.rs#L735) | ✅ |
| [Emma](src/import.rs#L751) | ✅ |
| [Eri](src/import.rs#L1365) | ✅ |
| [Ethan's Adventure](src/import.rs#L435) | ✅ |
| [Explorer's Guidance](src/import.rs#L788) | ✅ |
| [Fennel](src/import.rs#L853) | ✅ |
| [Firebreather](src/import.rs#L688) | ✅ |
| [Friends in Paldea](src/import.rs#L234) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1318) | ✅ |
| Grimsley's Move | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Gwynn](src/import.rs#L1009) | ✅ |
| [Harlequin](src/import.rs#L837) | ✅ |
| [Hassel](src/import.rs#L774) | ✅ |
| [Hilda](src/import.rs#L1066) | ✅ |
| [Iris's Fighting Spirit](src/import.rs#L849) | ✅ |
| [Jacinthe](src/import.rs#L637) | ✅ |
| [Janine's Secret Art](src/import.rs#L1396) | ✅ |
| [Jasmine's Gaze](src/import.rs#L390) | ✅ |
| [Jett](src/import.rs#L734) | ✅ |
| [Judge](src/import.rs#L888) | ✅ |
| [Kieran](src/import.rs#L1350) | ✅ |
| Kofu | ❌ — not yet triaged |
| [Lacey](src/import.rs#L829) | ✅ |
| [Lana's Aid](src/import.rs#L1267) | ✅ |
| [Larry's Skill](src/import.rs#L1322) | ✅ |
| [Lillie's Determination](src/import.rs#L889) | ✅ |
| [Lisia's Appeal](src/import.rs#L1498) | ✅ |
| Lt. Surge's Bargain | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Lucian](src/import.rs#L825) | ✅ |
| [Misty's Vitality](src/import.rs#L644) | ✅ |
| [Morty's Conviction](src/import.rs#L1360) | ✅ |
| [N's Plan](src/import.rs#L1285) | ✅ |
| [Naveen](src/import.rs#L848) | ✅ |
| Perrin | ❌ — not yet triaged |
| [Philippe](src/import.rs#L674) | ✅ |
| [Picnicker](src/import.rs#L658) | ✅ |
| [Pokémon Center Lady](src/import.rs#L1286) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1288) | ✅ |
| [Roxie's Performance](src/import.rs#L400) | ✅ |
| Ruffian | ❌ — needs a target-then-discard-both phase, scoped to one Pokémon chosen up front |
| [Rust Syndicate Grunt](src/import.rs#L1281) | ✅ |
| [Salvatore](src/import.rs#L1497) | ✅ |
| [Surfer](src/import.rs#L1306) | ✅ |
| [Tarragon](src/import.rs#L660) | ✅ |
| [Team Rocket's Archer](src/import.rs#L716) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1458) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1466) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1171) | ✅ |
| [Team Rocket's Proton](src/import.rs#L449) | ✅ |
| Tyme | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Urbain](src/import.rs#L379) | ✅ |
| [Waitress](src/import.rs#L760) | ✅ |
| [Wally's Compassion](src/import.rs#L1395) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1364) | ✅ |

### Items (61/85 built)

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
| [Boxed Order](src/import.rs#L581) | ✅ |
| [Brilliant Blender](src/import.rs#L525) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L981) | ✅ |
| [Bug Catching Set](src/import.rs#L1227) | ✅ |
| Call Bell | ❌ — needs the "playable on the first turn" allowance |
| Chill Teaser Toy | ❌ — needs the "playable on the first turn" allowance |
| [Crushing Hammer](src/import.rs#L924) | ✅ |
| [Dangerous Laser](src/import.rs#L513) | ✅ |
| [Dark Bell](src/import.rs#L520) | ✅ |
| Deduction Kit | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Dragon Elixir](src/import.rs#L624) | ✅ |
| [Dusk Ball](src/import.rs#L1426) | ✅ |
| Energy Coin | ❌ — not yet triaged |
| [Energy Recycler](src/import.rs#L1581) | ✅ |
| [Energy Retrieval](src/import.rs#L1411) | ✅ |
| [Energy Search](src/import.rs#L1397) | ✅ |
| [Energy Search Pro](src/import.rs#L939) | ✅ |
| Energy Swatter | ❌ — not yet triaged |
| [Energy Switch](src/import.rs#L1051) | ✅ |
| [Enhanced Hammer](src/import.rs#L869) | ✅ |
| [Fighting Gong](src/import.rs#L1432) | ✅ |
| [Glass Trumpet](src/import.rs#L870) | ✅ |
| Great Haul Net | ❌ — not yet triaged |
| [Hand Trimmer](src/import.rs#L1425) | ✅ |
| [Hole-Digging Shovel](src/import.rs#L381) | ✅ |
| [Hop's Bag](src/import.rs#L463) | ✅ |
| [Hyper Aroma](src/import.rs#L553) | ✅ |
| [Iron Defender](src/import.rs#L394) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1263) | ✅ |
| Love Ball | ❌ — not yet triaged |
| [Lumiose Galette](src/import.rs#L623) | ✅ |
| [Master Ball](src/import.rs#L539) | ✅ |
| [Max Rod](src/import.rs#L595) | ✅ |
| Meddling Memo | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Mega Signal](src/import.rs#L953) | ✅ |
| Megaton Blower | ❌ — not yet triaged |
| [Miracle Headset](src/import.rs#L609) | ✅ |
| [N's PP Up](src/import.rs#L1185) | ✅ |
| [Night Stretcher](src/import.rs#L896) | ✅ |
| Ogre's Mask | ❌ — not yet triaged |
| [Poké Ball](src/import.rs#L421) | ✅ |
| [Poké Pad](src/import.rs#L910) | ✅ |
| [Poké Vital A](src/import.rs#L633) | ✅ |
| [Pokégear 3.0](src/import.rs#L1213) | ✅ |
| [Pokémon Catcher](src/import.rs#L1428) | ✅ |
| [Potion](src/import.rs#L628) | ✅ |
| [Precious Trolley](src/import.rs#L925) | ✅ |
| [Premium Power Pro](src/import.rs#L1314) | ✅ |
| [Prime Catcher](src/import.rs#L1427) | ✅ |
| [Rare Candy](src/import.rs#L1170) | ✅ |
| [Reboot Pod](src/import.rs#L410) | ✅ |
| Redeemable Ticket | ❌ — not yet triaged |
| [Repel](src/import.rs#L524) | ✅ |
| Roto-Stick | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Sacred Ash](src/import.rs#L1023) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1499) | ✅ |
| Scramble Switch | ❌ — not yet triaged |
| [Secret Box](src/import.rs#L1530) | ✅ |
| [Special Red Card](src/import.rs#L1143) | ✅ |
| [Strange Timepiece](src/import.rs#L1446) | ✅ |
| [Super Potion](src/import.rs#L632) | ✅ |
| [Switch](src/import.rs#L1262) | ✅ |
| [TM Machine](src/import.rs#L967) | ✅ |
| Team Rocket's Bother-Bot | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Team Rocket's Great Ball](src/import.rs#L1470) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1567) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1500) | ✅ |
| [Tera Orb](src/import.rs#L1052) | ✅ |
| [Tool Scrapper](src/import.rs#L864) | ✅ |
| [Transformation Tome](src/import.rs#L1526) | ✅ |
| [Treasure Tracker](src/import.rs#L567) | ✅ |
| [Ultra Ball](src/import.rs#L1037) | ✅ |
| [Unfair Stamp](src/import.rs#L1255) | ✅ |
| [Wondrous Patch](src/import.rs#L1199) | ✅ |

### Tools (23/35 built)

| Card | Status |
| --- | --- |
| Adversity Policy | ❌ — not yet triaged |
| [Air Balloon](src/import.rs#L1447) | ✅ |
| Amulet of Hope | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Babiri Berry](src/import.rs#L489) | ✅ |
| Backtrack Badge | ❌ — not yet triaged |
| [Binding Mochi](src/import.rs#L1450) | ✅ |
| [Brave Bangle](src/import.rs#L1449) | ✅ |
| [Colbur Berry](src/import.rs#L493) | ✅ |
| Core Memory | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Counter Gain](src/import.rs#L420) | ✅ |
| [Cynthia's Power Weight](src/import.rs#L477) | ✅ |
| Deluxe Bomb | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Gravity Gemstone](src/import.rs#L412) | ✅ |
| [Haban Berry](src/import.rs#L509) | ✅ |
| [Handheld Fan](src/import.rs#L1454) | ✅ |
| Heavy Baton | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Hero's Cape](src/import.rs#L1448) | ✅ |
| Hop's Choice Band | ❌ — not yet triaged |
| [Light Ball](src/import.rs#L383) | ✅ |
| [Lillie's Pearl](src/import.rs#L1451) | ✅ |
| [Lucky Helmet](src/import.rs#L1453) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Occa Berry](src/import.rs#L497) | ✅ |
| [Passho Berry](src/import.rs#L501) | ✅ |
| [Payapa Berry](src/import.rs#L505) | ✅ |
| [Powerglass](src/import.rs#L1455) | ✅ |
| [Punk Helmet](src/import.rs#L1452) | ✅ |
| [Rescue Board](src/import.rs#L389) | ✅ |
| [Sacred Charm](src/import.rs#L387) | ✅ |
| [Sparkling Crystal](src/import.rs#L416) | ✅ |
| Survival Brace | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| Team Rocket's Hypnotizer | ❌ — not yet triaged |
| Technical Machine: Fluorite | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| Thick Scale | ❌ — not yet triaged |
| Tremendous Bomb | ❌ — not yet triaged |

### Stadiums (19/31 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1508) | ✅ |
| Ange Floette | ❌ — not yet triaged |
| [Area Zero Underdepths](src/import.rs#L885) | ✅ |
| [Battle Cage](src/import.rs#L886) | ✅ |
| Celebratory Fanfare | ❌ — not yet triaged |
| [Community Center](src/import.rs#L1515) | ✅ |
| Dizzying Valley | ❌ — not yet triaged |
| [Festival Grounds](src/import.rs#L1522) | ✅ |
| [Forest of Vitality](src/import.rs#L1521) | ✅ |
| Fossil Quarry | ❌ — not yet triaged |
| [Full Metal Lab](src/import.rs#L485) | ✅ |
| Grand Tree | ❌ — not yet triaged |
| [Granite Cave](src/import.rs#L481) | ✅ |
| [Gravity Mountain](src/import.rs#L1456) | ✅ |
| [Jamming Tower](src/import.rs#L1519) | ✅ |
| Levincia | ❌ — not yet triaged |
| [Lively Stadium](src/import.rs#L388) | ✅ |
| [Lumiose City](src/import.rs#L1513) | ✅ |
| Mystery Garden | ❌ — not yet triaged |
| [N's Castle](src/import.rs#L1457) | ✅ |
| Neutralization Zone | ❌ — not yet triaged |
| [Nighttime Mine](src/import.rs#L884) | ✅ |
| Paradise Resort | ❌ — not yet triaged |
| [Perilous Jungle](src/import.rs#L411) | ✅ |
| Postwick | ❌ — not yet triaged |
| [Prism Tower](src/import.rs#L1514) | ✅ |
| [Risky Ruins](src/import.rs#L1520) | ✅ |
| Spikemuth Gym | ❌ — not yet triaged |
| Surfing Beach | ❌ — not yet triaged |
| [Team Rocket's Factory](src/import.rs#L1509) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L887) | ✅ |

### Special Energy (15/17 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1960) | ✅ |
| [Bubbly Water Energy](src/import.rs#L1972) | ✅ |
| [Enriching Energy](src/import.rs#L1943) | ✅ |
| [Growing Grass Energy](src/import.rs#L1942) | ✅ |
| [Ignition Energy](src/import.rs#L1984) | ✅ |
| Legacy Energy | ❌ — its Prize-count clause needs the deferred Prize-count mechanic |
| [Magnetic Metal Energy](src/import.rs#L1969) | ✅ |
| [Mist Energy](src/import.rs#L1957) | ✅ |
| [Neo Upper Energy](src/import.rs#L1990) | ✅ |
| [Nitro Fire Energy](src/import.rs#L1975) | ✅ |
| [Prism Energy](src/import.rs#L1963) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L1966) | ✅ |
| [Shadowy Darkness Energy](src/import.rs#L1981) | ✅ |
| [Spiky Energy](src/import.rs#L1954) | ✅ |
| Team Rocket's Energy | ❌ — attaches only to a Team Rocket's Pokémon and discards itself instantly off any other — nothing validates an attach against the carrier's identity yet |
| [Telepathic Psychic Energy](src/import.rs#L1946) | ✅ |
| [Voltaic Lightning Energy](src/import.rs#L1978) | ✅ |

### Pokémon (field) (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2177) | [✅](src/import.rs#L2177) | [✅](src/import.rs#L2059) |
| [Alakazam](src/import.rs#L2332) | [✅](src/import.rs#L2332) | [✅](src/import.rs#L2049) |
| [Annihilape](src/import.rs#L2206) | [✅](src/import.rs#L2206) | [✅](src/import.rs#L2015) |
| [Applin](src/import.rs#L2167) | [✅](src/import.rs#L2167) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2209) | [✅](src/import.rs#L2209) | — |
| [Beldum](src/import.rs#L2189) | [✅](src/import.rs#L2189) | — |
| [Blaziken ex](src/import.rs#L2273) | [✅](src/import.rs#L2273) | [✅](src/import.rs#L2089) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2237) | [✅](src/import.rs#L2237) | [✅](src/import.rs#L2023) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2148) | [✅](src/import.rs#L2148) | — |
| [Budew](src/import.rs#L2214) | [✅](src/import.rs#L2214) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2208) | [✅](src/import.rs#L2208) | — |
| [Carvanha](src/import.rs#L2126) | [✅](src/import.rs#L2126) | — |
| [Celebi](src/import.rs#L2207) | [✅](src/import.rs#L2207) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2305) | [✅](src/import.rs#L2305) | — |
| [Chien-Pao](src/import.rs#L2274) | [✅](src/import.rs#L2274) | [✅](src/import.rs#L2092) |
| [Chikorita](src/import.rs#L2210) | [✅](src/import.rs#L2210) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2255) | [✅](src/import.rs#L2255) | — |
| [Combusken](src/import.rs#L2224) | [✅](src/import.rs#L2224) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2344) | [✅](src/import.rs#L2344) | [✅](src/import.rs#L2004) |
| [Dedenne](src/import.rs#L2164) | [✅](src/import.rs#L2164) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2289) | [✅](src/import.rs#L2289) | [✅](src/import.rs#L2077) |
| [Dragapult ex](src/import.rs#L2171) | [✅](src/import.rs#L2171) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2032) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2178) | [✅](src/import.rs#L2178) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2056) |
| [Dudunsparce ex](src/import.rs#L2139) | [✅](src/import.rs#L2139) | — |
| [Dunsparce](src/import.rs#L2190) | [✅](src/import.rs#L2190) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2060) |
| [Dusknoir](src/import.rs#L2271) | [✅](src/import.rs#L2271) | [✅](src/import.rs#L2061) |
| [Duskull](src/import.rs#L2193) | [✅](src/import.rs#L2193) | — |
| [Dwebble](src/import.rs#L2183) | [✅](src/import.rs#L2183) | — |
| [Elgyem](src/import.rs#L2213) | [✅](src/import.rs#L2213) | — |
| [Enamorus](src/import.rs#L2233) | [✅](src/import.rs#L2233) | — |
| [Fan Rotom](src/import.rs#L2278) | [✅](src/import.rs#L2278) | [✅](src/import.rs#L2102) |
| [Fezandipiti ex](src/import.rs#L2352) | [✅](src/import.rs#L2352) | [✅](src/import.rs#L2050) |
| [Flutter Mane](src/import.rs#L2269) | [✅](src/import.rs#L2269) | [✅](src/import.rs#L2026) |
| [Genesect](src/import.rs#L2324) | [✅](src/import.rs#L2324) | [✅](src/import.rs#L2065) |
| [Genesect ex](src/import.rs#L2272) | [✅](src/import.rs#L2272) | [✅](src/import.rs#L2062) |
| [Goldeen](src/import.rs#L2287) | [✅](src/import.rs#L2287) | [✅](src/import.rs#L2075) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2180) | [✅](src/import.rs#L2180) | [✅](src/import.rs#L2016) |
| [Hydrapple ex](src/import.rs#L2238) | [✅](src/import.rs#L2238) | [✅](src/import.rs#L2017) |
| [Iron Crown ex](src/import.rs#L2198) | [✅](src/import.rs#L2198) | [✅](src/import.rs#L2012) |
| [Iron Leaves ex](src/import.rs#L2277) | [✅](src/import.rs#L2277) | [✅](src/import.rs#L2093) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2045) |
| [Koraidon ex](src/import.rs#L2199) | [✅](src/import.rs#L2199) | — |
| [Kyurem](src/import.rs#L2321) | [✅](src/import.rs#L2321) | [✅](src/import.rs#L2084) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2338) | [✅](src/import.rs#L2338) | [✅](src/import.rs#L2003) |
| [Lillie's Clefairy ex](src/import.rs#L2362) | [✅](src/import.rs#L2362) | [✅](src/import.rs#L2007) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2245) | [✅](src/import.rs#L2245) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2241) | [✅](src/import.rs#L2241) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2348) | [✅](src/import.rs#L2348) | [✅](src/import.rs#L2000) |
| [Mega Lopunny ex](src/import.rs#L2146) | [✅](src/import.rs#L2146) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2181) | [✅](src/import.rs#L2181) | — |
| [Mega Skarmory ex](src/import.rs#L2259) | [✅](src/import.rs#L2259) | — |
| [Mega Slowbro ex](src/import.rs#L2299) | [✅](src/import.rs#L2299) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2071) |
| [Meowth ex](src/import.rs#L2351) | [✅](src/import.rs#L2351) | [✅](src/import.rs#L2044) |
| [Metagross](src/import.rs#L2155) | [✅](src/import.rs#L2155) | — |
| [Metang](src/import.rs#L2339) | [✅](src/import.rs#L2339) | [✅](src/import.rs#L2035) |
| [Moltres](src/import.rs#L2191) | [✅](src/import.rs#L2191) | — |
| [Munkidori](src/import.rs#L2345) | [✅](src/import.rs#L2345) | [✅](src/import.rs#L2038) |
| [N's Darmanitan](src/import.rs#L2136) | [✅](src/import.rs#L2136) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2133) | [✅](src/import.rs#L2133) | — |
| [N's Zekrom](src/import.rs#L2145) | [✅](src/import.rs#L2145) | — |
| [N's Zoroark ex](src/import.rs#L2318) | [✅](src/import.rs#L2318) | [✅](src/import.rs#L2068) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2341) | [✅](src/import.rs#L2341) | [✅](src/import.rs#L2046) |
| [Paldean Tauros](src/import.rs#L2129) | [✅](src/import.rs#L2129) | — |
| [Passimian](src/import.rs#L2142) | [✅](src/import.rs#L2142) | — |
| [Patrat](src/import.rs#L2340) | [✅](src/import.rs#L2340) | [✅](src/import.rs#L2005) |
| [Pecharunt](src/import.rs#L2282) | [✅](src/import.rs#L2282) | [✅](src/import.rs#L2072) |
| [Pecharunt ex](src/import.rs#L2279) | [✅](src/import.rs#L2279) | [✅](src/import.rs#L2109) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2006) |
| [Rabsca](src/import.rs#L2195) | [✅](src/import.rs#L2195) | [✅](src/import.rs#L2011) |
| [Raging Bolt ex](src/import.rs#L2160) | [✅](src/import.rs#L2160) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L2127) | [✅](src/import.rs#L2127) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2288) | [✅](src/import.rs#L2288) | [✅](src/import.rs#L2076) |
| [Shaymin](src/import.rs#L2335) | [✅](src/import.rs#L2335) | [✅](src/import.rs#L2010) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2184) | [✅](src/import.rs#L2184) | — |
| [Slowpoke](src/import.rs#L2192) | [✅](src/import.rs#L2192) | ❌ |
| [Smoochum](src/import.rs#L2252) | [✅](src/import.rs#L2252) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2234) | [✅](src/import.rs#L2234) | — |
| [Tapu Bulu](src/import.rs#L2128) | [✅](src/import.rs#L2128) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2029) |
| [Teal Mask Ogerpon ex](src/import.rs#L2353) | [✅](src/import.rs#L2353) | [✅](src/import.rs#L2053) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2078) |
| [Torchic](src/import.rs#L2194) | [✅](src/import.rs#L2194) | — |
| [Toxel](src/import.rs#L2179) | [✅](src/import.rs#L2179) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2096) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2169) | [✅](src/import.rs#L2169) | — |
| [Yveltal](src/import.rs#L2168) | [✅](src/import.rs#L2168) | — |
| [Zeraora](src/import.rs#L2151) | [✅](src/import.rs#L2151) | — |
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

