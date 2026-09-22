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
| Tools | 26 | 35 |
| Stadiums | 19 | 31 |
| Special Energy | 15 | 17 |

### Supporters (70/78 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1317) | ✅ |
| [Acerola's Mischief](src/import.rs#L1519) | ✅ |
| [Amarys](src/import.rs#L771) | ✅ |
| Anthea & Concordia | ❌ — not yet triaged |
| [Bianca's Devotion](src/import.rs#L1302) | ✅ |
| [Billy & O'Nare](src/import.rs#L758) | ✅ |
| [Black Belt's Training](src/import.rs#L1325) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L880) | ✅ |
| [Brock's Scouting](src/import.rs#L1387) | ✅ |
| [Canari](src/import.rs#L717) | ✅ |
| Caretaker | ❌ — not yet triaged |
| [Carmine](src/import.rs#L674) | ✅ |
| [Cassiopeia](src/import.rs#L735) | ✅ |
| [Cheren](src/import.rs#L234) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1256) | ✅ |
| [Clemont's Quick Wit](src/import.rs#L872) | ✅ |
| [Colress's Tenacity](src/import.rs#L1105) | ✅ |
| [Cook](src/import.rs#L380) | ✅ |
| [Crispin](src/import.rs#L1162) | ✅ |
| [Cyrano](src/import.rs#L1010) | ✅ |
| [Dawn](src/import.rs#L1128) | ✅ |
| [Drasna](src/import.rs#L767) | ✅ |
| [Drayton](src/import.rs#L817) | ✅ |
| [Emcee's Hype](src/import.rs#L750) | ✅ |
| [Emma](src/import.rs#L766) | ✅ |
| [Eri](src/import.rs#L1380) | ✅ |
| [Ethan's Adventure](src/import.rs#L435) | ✅ |
| [Explorer's Guidance](src/import.rs#L803) | ✅ |
| [Fennel](src/import.rs#L868) | ✅ |
| [Firebreather](src/import.rs#L703) | ✅ |
| [Friends in Paldea](src/import.rs#L234) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1333) | ✅ |
| Grimsley's Move | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Gwynn](src/import.rs#L1024) | ✅ |
| [Harlequin](src/import.rs#L852) | ✅ |
| [Hassel](src/import.rs#L789) | ✅ |
| [Hilda](src/import.rs#L1081) | ✅ |
| [Iris's Fighting Spirit](src/import.rs#L864) | ✅ |
| [Jacinthe](src/import.rs#L652) | ✅ |
| [Janine's Secret Art](src/import.rs#L1411) | ✅ |
| [Jasmine's Gaze](src/import.rs#L390) | ✅ |
| [Jett](src/import.rs#L749) | ✅ |
| [Judge](src/import.rs#L903) | ✅ |
| [Kieran](src/import.rs#L1365) | ✅ |
| Kofu | ❌ — not yet triaged |
| [Lacey](src/import.rs#L844) | ✅ |
| [Lana's Aid](src/import.rs#L1282) | ✅ |
| [Larry's Skill](src/import.rs#L1337) | ✅ |
| [Lillie's Determination](src/import.rs#L904) | ✅ |
| [Lisia's Appeal](src/import.rs#L1513) | ✅ |
| Lt. Surge's Bargain | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Lucian](src/import.rs#L840) | ✅ |
| [Misty's Vitality](src/import.rs#L659) | ✅ |
| [Morty's Conviction](src/import.rs#L1375) | ✅ |
| [N's Plan](src/import.rs#L1300) | ✅ |
| [Naveen](src/import.rs#L863) | ✅ |
| Perrin | ❌ — not yet triaged |
| [Philippe](src/import.rs#L689) | ✅ |
| [Picnicker](src/import.rs#L673) | ✅ |
| [Pokémon Center Lady](src/import.rs#L1301) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1303) | ✅ |
| [Roxie's Performance](src/import.rs#L400) | ✅ |
| Ruffian | ❌ — needs a target-then-discard-both phase, scoped to one Pokémon chosen up front |
| [Rust Syndicate Grunt](src/import.rs#L1296) | ✅ |
| [Salvatore](src/import.rs#L1512) | ✅ |
| [Surfer](src/import.rs#L1321) | ✅ |
| [Tarragon](src/import.rs#L675) | ✅ |
| [Team Rocket's Archer](src/import.rs#L731) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1473) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1481) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1186) | ✅ |
| [Team Rocket's Proton](src/import.rs#L449) | ✅ |
| Tyme | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Urbain](src/import.rs#L379) | ✅ |
| [Waitress](src/import.rs#L775) | ✅ |
| [Wally's Compassion](src/import.rs#L1410) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1379) | ✅ |

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
| [Boxed Order](src/import.rs#L596) | ✅ |
| [Brilliant Blender](src/import.rs#L540) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L996) | ✅ |
| [Bug Catching Set](src/import.rs#L1242) | ✅ |
| Call Bell | ❌ — needs the "playable on the first turn" allowance |
| Chill Teaser Toy | ❌ — needs the "playable on the first turn" allowance |
| [Crushing Hammer](src/import.rs#L939) | ✅ |
| [Dangerous Laser](src/import.rs#L528) | ✅ |
| [Dark Bell](src/import.rs#L535) | ✅ |
| Deduction Kit | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Dragon Elixir](src/import.rs#L639) | ✅ |
| [Dusk Ball](src/import.rs#L1441) | ✅ |
| Energy Coin | ❌ — not yet triaged |
| [Energy Recycler](src/import.rs#L1596) | ✅ |
| [Energy Retrieval](src/import.rs#L1426) | ✅ |
| [Energy Search](src/import.rs#L1412) | ✅ |
| [Energy Search Pro](src/import.rs#L954) | ✅ |
| Energy Swatter | ❌ — not yet triaged |
| [Energy Switch](src/import.rs#L1066) | ✅ |
| [Enhanced Hammer](src/import.rs#L884) | ✅ |
| [Fighting Gong](src/import.rs#L1447) | ✅ |
| [Glass Trumpet](src/import.rs#L885) | ✅ |
| Great Haul Net | ❌ — not yet triaged |
| [Hand Trimmer](src/import.rs#L1440) | ✅ |
| [Hole-Digging Shovel](src/import.rs#L381) | ✅ |
| [Hop's Bag](src/import.rs#L463) | ✅ |
| [Hyper Aroma](src/import.rs#L568) | ✅ |
| [Iron Defender](src/import.rs#L394) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1278) | ✅ |
| Love Ball | ❌ — not yet triaged |
| [Lumiose Galette](src/import.rs#L638) | ✅ |
| [Master Ball](src/import.rs#L554) | ✅ |
| [Max Rod](src/import.rs#L610) | ✅ |
| Meddling Memo | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Mega Signal](src/import.rs#L968) | ✅ |
| Megaton Blower | ❌ — not yet triaged |
| [Miracle Headset](src/import.rs#L624) | ✅ |
| [N's PP Up](src/import.rs#L1200) | ✅ |
| [Night Stretcher](src/import.rs#L911) | ✅ |
| Ogre's Mask | ❌ — not yet triaged |
| [Poké Ball](src/import.rs#L421) | ✅ |
| [Poké Pad](src/import.rs#L925) | ✅ |
| [Poké Vital A](src/import.rs#L648) | ✅ |
| [Pokégear 3.0](src/import.rs#L1228) | ✅ |
| [Pokémon Catcher](src/import.rs#L1443) | ✅ |
| [Potion](src/import.rs#L643) | ✅ |
| [Precious Trolley](src/import.rs#L940) | ✅ |
| [Premium Power Pro](src/import.rs#L1329) | ✅ |
| [Prime Catcher](src/import.rs#L1442) | ✅ |
| [Rare Candy](src/import.rs#L1185) | ✅ |
| [Reboot Pod](src/import.rs#L410) | ✅ |
| Redeemable Ticket | ❌ — not yet triaged |
| [Repel](src/import.rs#L539) | ✅ |
| Roto-Stick | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Sacred Ash](src/import.rs#L1038) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1514) | ✅ |
| Scramble Switch | ❌ — not yet triaged |
| [Secret Box](src/import.rs#L1545) | ✅ |
| [Special Red Card](src/import.rs#L1158) | ✅ |
| [Strange Timepiece](src/import.rs#L1461) | ✅ |
| [Super Potion](src/import.rs#L647) | ✅ |
| [Switch](src/import.rs#L1277) | ✅ |
| [TM Machine](src/import.rs#L982) | ✅ |
| Team Rocket's Bother-Bot | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Team Rocket's Great Ball](src/import.rs#L1485) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1582) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1515) | ✅ |
| [Tera Orb](src/import.rs#L1067) | ✅ |
| [Tool Scrapper](src/import.rs#L879) | ✅ |
| [Transformation Tome](src/import.rs#L1541) | ✅ |
| [Treasure Tracker](src/import.rs#L582) | ✅ |
| [Ultra Ball](src/import.rs#L1052) | ✅ |
| [Unfair Stamp](src/import.rs#L1270) | ✅ |
| [Wondrous Patch](src/import.rs#L1214) | ✅ |

### Tools (26/35 built)

| Card | Status |
| --- | --- |
| [Adversity Policy](src/import.rs#L527) | ✅ |
| [Air Balloon](src/import.rs#L1462) | ✅ |
| Amulet of Hope | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Babiri Berry](src/import.rs#L489) | ✅ |
| Backtrack Badge | ❌ — not yet triaged |
| [Binding Mochi](src/import.rs#L1465) | ✅ |
| [Brave Bangle](src/import.rs#L1464) | ✅ |
| [Colbur Berry](src/import.rs#L493) | ✅ |
| Core Memory | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Counter Gain](src/import.rs#L420) | ✅ |
| [Cynthia's Power Weight](src/import.rs#L477) | ✅ |
| Deluxe Bomb | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Gravity Gemstone](src/import.rs#L412) | ✅ |
| [Haban Berry](src/import.rs#L509) | ✅ |
| [Handheld Fan](src/import.rs#L1469) | ✅ |
| Heavy Baton | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Hero's Cape](src/import.rs#L1463) | ✅ |
| Hop's Choice Band | ❌ — not yet triaged |
| [Light Ball](src/import.rs#L383) | ✅ |
| [Lillie's Pearl](src/import.rs#L1466) | ✅ |
| [Lucky Helmet](src/import.rs#L1468) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Occa Berry](src/import.rs#L497) | ✅ |
| [Passho Berry](src/import.rs#L501) | ✅ |
| [Payapa Berry](src/import.rs#L505) | ✅ |
| [Powerglass](src/import.rs#L1470) | ✅ |
| [Punk Helmet](src/import.rs#L1467) | ✅ |
| [Rescue Board](src/import.rs#L389) | ✅ |
| [Sacred Charm](src/import.rs#L387) | ✅ |
| [Sparkling Crystal](src/import.rs#L416) | ✅ |
| Survival Brace | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Team Rocket's Hypnotizer](src/import.rs#L520) | ✅ |
| Technical Machine: Fluorite | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Thick Scale](src/import.rs#L513) | ✅ |
| Tremendous Bomb | ❌ — not yet triaged |

### Stadiums (19/31 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1523) | ✅ |
| Ange Floette | ❌ — not yet triaged |
| [Area Zero Underdepths](src/import.rs#L900) | ✅ |
| [Battle Cage](src/import.rs#L901) | ✅ |
| Celebratory Fanfare | ❌ — not yet triaged |
| [Community Center](src/import.rs#L1530) | ✅ |
| Dizzying Valley | ❌ — not yet triaged |
| [Festival Grounds](src/import.rs#L1537) | ✅ |
| [Forest of Vitality](src/import.rs#L1536) | ✅ |
| Fossil Quarry | ❌ — not yet triaged |
| [Full Metal Lab](src/import.rs#L485) | ✅ |
| Grand Tree | ❌ — not yet triaged |
| [Granite Cave](src/import.rs#L481) | ✅ |
| [Gravity Mountain](src/import.rs#L1471) | ✅ |
| [Jamming Tower](src/import.rs#L1534) | ✅ |
| Levincia | ❌ — not yet triaged |
| [Lively Stadium](src/import.rs#L388) | ✅ |
| [Lumiose City](src/import.rs#L1528) | ✅ |
| Mystery Garden | ❌ — not yet triaged |
| [N's Castle](src/import.rs#L1472) | ✅ |
| Neutralization Zone | ❌ — not yet triaged |
| [Nighttime Mine](src/import.rs#L899) | ✅ |
| Paradise Resort | ❌ — not yet triaged |
| [Perilous Jungle](src/import.rs#L411) | ✅ |
| Postwick | ❌ — not yet triaged |
| [Prism Tower](src/import.rs#L1529) | ✅ |
| [Risky Ruins](src/import.rs#L1535) | ✅ |
| Spikemuth Gym | ❌ — not yet triaged |
| Surfing Beach | ❌ — not yet triaged |
| [Team Rocket's Factory](src/import.rs#L1524) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L902) | ✅ |

### Special Energy (15/17 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1975) | ✅ |
| [Bubbly Water Energy](src/import.rs#L1987) | ✅ |
| [Enriching Energy](src/import.rs#L1958) | ✅ |
| [Growing Grass Energy](src/import.rs#L1957) | ✅ |
| [Ignition Energy](src/import.rs#L1999) | ✅ |
| Legacy Energy | ❌ — its Prize-count clause needs the deferred Prize-count mechanic |
| [Magnetic Metal Energy](src/import.rs#L1984) | ✅ |
| [Mist Energy](src/import.rs#L1972) | ✅ |
| [Neo Upper Energy](src/import.rs#L2005) | ✅ |
| [Nitro Fire Energy](src/import.rs#L1990) | ✅ |
| [Prism Energy](src/import.rs#L1978) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L1981) | ✅ |
| [Shadowy Darkness Energy](src/import.rs#L1996) | ✅ |
| [Spiky Energy](src/import.rs#L1969) | ✅ |
| Team Rocket's Energy | ❌ — attaches only to a Team Rocket's Pokémon and discards itself instantly off any other — nothing validates an attach against the carrier's identity yet |
| [Telepathic Psychic Energy](src/import.rs#L1961) | ✅ |
| [Voltaic Lightning Energy](src/import.rs#L1993) | ✅ |

### Pokémon (field) (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2192) | [✅](src/import.rs#L2192) | [✅](src/import.rs#L2074) |
| [Alakazam](src/import.rs#L2347) | [✅](src/import.rs#L2347) | [✅](src/import.rs#L2064) |
| [Annihilape](src/import.rs#L2221) | [✅](src/import.rs#L2221) | [✅](src/import.rs#L2030) |
| [Applin](src/import.rs#L2182) | [✅](src/import.rs#L2182) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2224) | [✅](src/import.rs#L2224) | — |
| [Beldum](src/import.rs#L2204) | [✅](src/import.rs#L2204) | — |
| [Blaziken ex](src/import.rs#L2288) | [✅](src/import.rs#L2288) | [✅](src/import.rs#L2104) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2252) | [✅](src/import.rs#L2252) | [✅](src/import.rs#L2038) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2163) | [✅](src/import.rs#L2163) | — |
| [Budew](src/import.rs#L2229) | [✅](src/import.rs#L2229) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2223) | [✅](src/import.rs#L2223) | — |
| [Carvanha](src/import.rs#L2141) | [✅](src/import.rs#L2141) | — |
| [Celebi](src/import.rs#L2222) | [✅](src/import.rs#L2222) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2320) | [✅](src/import.rs#L2320) | — |
| [Chien-Pao](src/import.rs#L2289) | [✅](src/import.rs#L2289) | [✅](src/import.rs#L2107) |
| [Chikorita](src/import.rs#L2225) | [✅](src/import.rs#L2225) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2270) | [✅](src/import.rs#L2270) | — |
| [Combusken](src/import.rs#L2239) | [✅](src/import.rs#L2239) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2359) | [✅](src/import.rs#L2359) | [✅](src/import.rs#L2019) |
| [Dedenne](src/import.rs#L2179) | [✅](src/import.rs#L2179) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2304) | [✅](src/import.rs#L2304) | [✅](src/import.rs#L2092) |
| [Dragapult ex](src/import.rs#L2186) | [✅](src/import.rs#L2186) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2047) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2193) | [✅](src/import.rs#L2193) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2071) |
| [Dudunsparce ex](src/import.rs#L2154) | [✅](src/import.rs#L2154) | — |
| [Dunsparce](src/import.rs#L2205) | [✅](src/import.rs#L2205) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2075) |
| [Dusknoir](src/import.rs#L2286) | [✅](src/import.rs#L2286) | [✅](src/import.rs#L2076) |
| [Duskull](src/import.rs#L2208) | [✅](src/import.rs#L2208) | — |
| [Dwebble](src/import.rs#L2198) | [✅](src/import.rs#L2198) | — |
| [Elgyem](src/import.rs#L2228) | [✅](src/import.rs#L2228) | — |
| [Enamorus](src/import.rs#L2248) | [✅](src/import.rs#L2248) | — |
| [Fan Rotom](src/import.rs#L2293) | [✅](src/import.rs#L2293) | [✅](src/import.rs#L2117) |
| [Fezandipiti ex](src/import.rs#L2367) | [✅](src/import.rs#L2367) | [✅](src/import.rs#L2065) |
| [Flutter Mane](src/import.rs#L2284) | [✅](src/import.rs#L2284) | [✅](src/import.rs#L2041) |
| [Genesect](src/import.rs#L2339) | [✅](src/import.rs#L2339) | [✅](src/import.rs#L2080) |
| [Genesect ex](src/import.rs#L2287) | [✅](src/import.rs#L2287) | [✅](src/import.rs#L2077) |
| [Goldeen](src/import.rs#L2302) | [✅](src/import.rs#L2302) | [✅](src/import.rs#L2090) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2195) | [✅](src/import.rs#L2195) | [✅](src/import.rs#L2031) |
| [Hydrapple ex](src/import.rs#L2253) | [✅](src/import.rs#L2253) | [✅](src/import.rs#L2032) |
| [Iron Crown ex](src/import.rs#L2213) | [✅](src/import.rs#L2213) | [✅](src/import.rs#L2027) |
| [Iron Leaves ex](src/import.rs#L2292) | [✅](src/import.rs#L2292) | [✅](src/import.rs#L2108) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2060) |
| [Koraidon ex](src/import.rs#L2214) | [✅](src/import.rs#L2214) | — |
| [Kyurem](src/import.rs#L2336) | [✅](src/import.rs#L2336) | [✅](src/import.rs#L2099) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2353) | [✅](src/import.rs#L2353) | [✅](src/import.rs#L2018) |
| [Lillie's Clefairy ex](src/import.rs#L2377) | [✅](src/import.rs#L2377) | [✅](src/import.rs#L2022) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2260) | [✅](src/import.rs#L2260) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2256) | [✅](src/import.rs#L2256) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2363) | [✅](src/import.rs#L2363) | [✅](src/import.rs#L2015) |
| [Mega Lopunny ex](src/import.rs#L2161) | [✅](src/import.rs#L2161) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2196) | [✅](src/import.rs#L2196) | — |
| [Mega Skarmory ex](src/import.rs#L2274) | [✅](src/import.rs#L2274) | — |
| [Mega Slowbro ex](src/import.rs#L2314) | [✅](src/import.rs#L2314) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2086) |
| [Meowth ex](src/import.rs#L2366) | [✅](src/import.rs#L2366) | [✅](src/import.rs#L2059) |
| [Metagross](src/import.rs#L2170) | [✅](src/import.rs#L2170) | — |
| [Metang](src/import.rs#L2354) | [✅](src/import.rs#L2354) | [✅](src/import.rs#L2050) |
| [Moltres](src/import.rs#L2206) | [✅](src/import.rs#L2206) | — |
| [Munkidori](src/import.rs#L2360) | [✅](src/import.rs#L2360) | [✅](src/import.rs#L2053) |
| [N's Darmanitan](src/import.rs#L2151) | [✅](src/import.rs#L2151) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2148) | [✅](src/import.rs#L2148) | — |
| [N's Zekrom](src/import.rs#L2160) | [✅](src/import.rs#L2160) | — |
| [N's Zoroark ex](src/import.rs#L2333) | [✅](src/import.rs#L2333) | [✅](src/import.rs#L2083) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2356) | [✅](src/import.rs#L2356) | [✅](src/import.rs#L2061) |
| [Paldean Tauros](src/import.rs#L2144) | [✅](src/import.rs#L2144) | — |
| [Passimian](src/import.rs#L2157) | [✅](src/import.rs#L2157) | — |
| [Patrat](src/import.rs#L2355) | [✅](src/import.rs#L2355) | [✅](src/import.rs#L2020) |
| [Pecharunt](src/import.rs#L2297) | [✅](src/import.rs#L2297) | [✅](src/import.rs#L2087) |
| [Pecharunt ex](src/import.rs#L2294) | [✅](src/import.rs#L2294) | [✅](src/import.rs#L2124) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2021) |
| [Rabsca](src/import.rs#L2210) | [✅](src/import.rs#L2210) | [✅](src/import.rs#L2026) |
| [Raging Bolt ex](src/import.rs#L2175) | [✅](src/import.rs#L2175) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L2142) | [✅](src/import.rs#L2142) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2303) | [✅](src/import.rs#L2303) | [✅](src/import.rs#L2091) |
| [Shaymin](src/import.rs#L2350) | [✅](src/import.rs#L2350) | [✅](src/import.rs#L2025) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2199) | [✅](src/import.rs#L2199) | — |
| [Slowpoke](src/import.rs#L2207) | [✅](src/import.rs#L2207) | ❌ |
| [Smoochum](src/import.rs#L2267) | [✅](src/import.rs#L2267) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2249) | [✅](src/import.rs#L2249) | — |
| [Tapu Bulu](src/import.rs#L2143) | [✅](src/import.rs#L2143) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2044) |
| [Teal Mask Ogerpon ex](src/import.rs#L2368) | [✅](src/import.rs#L2368) | [✅](src/import.rs#L2068) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2093) |
| [Torchic](src/import.rs#L2209) | [✅](src/import.rs#L2209) | — |
| [Toxel](src/import.rs#L2194) | [✅](src/import.rs#L2194) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2111) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2184) | [✅](src/import.rs#L2184) | — |
| [Yveltal](src/import.rs#L2183) | [✅](src/import.rs#L2183) | — |
| [Zeraora](src/import.rs#L2166) | [✅](src/import.rs#L2166) | — |
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

