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
of Trainer, Pokémon Abilities, Pokémon attacks with their own effects, and
the first Special Energy.

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
cargo run --bin coverage             # 368 of 3051 Standard cards (12.1%)
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

Every card in the artifact, by name; the tables below track the field.

| Kind | Built | Total |
| --- | --- | --- |
| Supporters | 63 | 78 |
| Items | 59 | 85 |
| Tools | 20 | 35 |
| Stadiums | 19 | 31 |
| Special Energy | 13 | 17 |

### Supporters (36/36 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1234) | ✅ |
| [Acerola's Mischief](src/import.rs#L1381) | ✅ |
| [Bianca's Devotion](src/import.rs#L1219) | ✅ |
| [Black Belt's Training](src/import.rs#L1242) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L797) | ✅ |
| [Brock's Scouting](src/import.rs#L1276) | ✅ |
| [Carmine](src/import.rs#L621) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1173) | ✅ |
| [Crispin](src/import.rs#L1079) | ✅ |
| [Cyrano](src/import.rs#L927) | ✅ |
| [Dawn](src/import.rs#L1045) | ✅ |
| [Eri](src/import.rs#L1269) | ✅ |
| [Explorer's Guidance](src/import.rs#L732) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1250) | ✅ |
| [Gwynn](src/import.rs#L941) | ✅ |
| [Hilda](src/import.rs#L998) | ✅ |
| [Janine's Secret Art](src/import.rs#L1300) | ✅ |
| [Judge](src/import.rs#L820) | ✅ |
| [Kieran](src/import.rs#L1254) | ✅ |
| [Lana's Aid](src/import.rs#L1199) | ✅ |
| [Lillie's Determination](src/import.rs#L821) | ✅ |
| [Lisia's Appeal](src/import.rs#L1375) | ✅ |
| [Morty's Conviction](src/import.rs#L1264) | ✅ |
| [N's Plan](src/import.rs#L1217) | ✅ |
| [Philippe](src/import.rs#L636) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1220) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1213) | ✅ |
| [Salvatore](src/import.rs#L1374) | ✅ |
| [Surfer](src/import.rs#L1238) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1362) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1370) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1103) | ✅ |
| [Team Rocket's Proton](src/import.rs#L426) | ✅ |
| [Wally's Compassion](src/import.rs#L1299) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1268) | ✅ |

### Items (37/37 built)

| Card | Status |
| --- | --- |
| [Brilliant Blender](src/import.rs#L502) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L913) | ✅ |
| [Bug Catching Set](src/import.rs#L1159) | ✅ |
| [Crushing Hammer](src/import.rs#L856) | ✅ |
| [Dark Bell](src/import.rs#L497) | ✅ |
| [Dusk Ball](src/import.rs#L1330) | ✅ |
| [Energy Recycler](src/import.rs#L1458) | ✅ |
| [Energy Retrieval](src/import.rs#L1315) | ✅ |
| [Energy Search](src/import.rs#L1301) | ✅ |
| [Energy Switch](src/import.rs#L983) | ✅ |
| [Enhanced Hammer](src/import.rs#L801) | ✅ |
| [Fighting Gong](src/import.rs#L1336) | ✅ |
| [Glass Trumpet](src/import.rs#L802) | ✅ |
| [Hand Trimmer](src/import.rs#L1329) | ✅ |
| [Iron Defender](src/import.rs#L390) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1195) | ✅ |
| [Max Rod](src/import.rs#L572) | ✅ |
| [N's PP Up](src/import.rs#L1117) | ✅ |
| [Night Stretcher](src/import.rs#L828) | ✅ |
| [Precious Trolley](src/import.rs#L857) | ✅ |
| [Premium Power Pro](src/import.rs#L1246) | ✅ |
| [Prime Catcher](src/import.rs#L1331) | ✅ |
| [Rare Candy](src/import.rs#L1102) | ✅ |
| [Sacred Ash](src/import.rs#L955) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1376) | ✅ |
| [Secret Box](src/import.rs#L1407) | ✅ |
| [Special Red Card](src/import.rs#L1075) | ✅ |
| [Strange Timepiece](src/import.rs#L1350) | ✅ |
| [Switch](src/import.rs#L1194) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1444) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1377) | ✅ |
| [Tera Orb](src/import.rs#L984) | ✅ |
| [Tool Scrapper](src/import.rs#L796) | ✅ |
| [Transformation Tome](src/import.rs#L1403) | ✅ |
| [Ultra Ball](src/import.rs#L969) | ✅ |
| [Unfair Stamp](src/import.rs#L1187) | ✅ |
| [Wondrous Patch](src/import.rs#L1131) | ✅ |

### Tools (10/10 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1351) | ✅ |
| [Binding Mochi](src/import.rs#L1354) | ✅ |
| [Brave Bangle](src/import.rs#L1353) | ✅ |
| [Handheld Fan](src/import.rs#L1358) | ✅ |
| [Hero's Cape](src/import.rs#L1352) | ✅ |
| [Lillie's Pearl](src/import.rs#L1355) | ✅ |
| [Lucky Helmet](src/import.rs#L1357) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Powerglass](src/import.rs#L1359) | ✅ |
| [Punk Helmet](src/import.rs#L1356) | ✅ |

### Stadiums (15/15 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1385) | ✅ |
| [Area Zero Underdepths](src/import.rs#L817) | ✅ |
| [Battle Cage](src/import.rs#L818) | ✅ |
| [Community Center](src/import.rs#L1392) | ✅ |
| [Festival Grounds](src/import.rs#L1399) | ✅ |
| [Forest of Vitality](src/import.rs#L1398) | ✅ |
| [Gravity Mountain](src/import.rs#L1360) | ✅ |
| [Jamming Tower](src/import.rs#L1396) | ✅ |
| [Lumiose City](src/import.rs#L1390) | ✅ |
| [N's Castle](src/import.rs#L1361) | ✅ |
| [Nighttime Mine](src/import.rs#L816) | ✅ |
| [Prism Tower](src/import.rs#L1391) | ✅ |
| [Risky Ruins](src/import.rs#L1397) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1386) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L819) | ✅ |

### Special Energy (8/11 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1837) | ✅ |
| [Enriching Energy](src/import.rs#L1820) | ✅ |
| [Growing Grass Energy](src/import.rs#L1819) | ✅ |
| Legacy Energy | ❌ |
| [Mist Energy](src/import.rs#L1834) | ✅ |
| Neo Upper Energy | ❌ |
| [Prism Energy](src/import.rs#L1840) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L1843) | ✅ |
| [Spiky Energy](src/import.rs#L1831) | ✅ |
| Team Rocket's Energy | ❌ |
| [Telepathic Psychic Energy](src/import.rs#L1823) | ✅ |

### Pokémon (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2044) | [✅](src/import.rs#L2044) | [✅](src/import.rs#L1926) |
| [Alakazam](src/import.rs#L2199) | [✅](src/import.rs#L2199) | [✅](src/import.rs#L1916) |
| [Annihilape](src/import.rs#L2073) | [✅](src/import.rs#L2073) | [✅](src/import.rs#L1882) |
| [Applin](src/import.rs#L2034) | [✅](src/import.rs#L2034) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2076) | [✅](src/import.rs#L2076) | — |
| [Beldum](src/import.rs#L2056) | [✅](src/import.rs#L2056) | — |
| [Blaziken ex](src/import.rs#L2140) | [✅](src/import.rs#L2140) | [✅](src/import.rs#L1956) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2104) | [✅](src/import.rs#L2104) | [✅](src/import.rs#L1890) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2015) | [✅](src/import.rs#L2015) | — |
| [Budew](src/import.rs#L2081) | [✅](src/import.rs#L2081) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2075) | [✅](src/import.rs#L2075) | — |
| [Carvanha](src/import.rs#L1993) | [✅](src/import.rs#L1993) | — |
| [Celebi](src/import.rs#L2074) | [✅](src/import.rs#L2074) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2172) | [✅](src/import.rs#L2172) | — |
| [Chien-Pao](src/import.rs#L2141) | [✅](src/import.rs#L2141) | [✅](src/import.rs#L1959) |
| [Chikorita](src/import.rs#L2077) | [✅](src/import.rs#L2077) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2122) | [✅](src/import.rs#L2122) | — |
| [Combusken](src/import.rs#L2091) | [✅](src/import.rs#L2091) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2211) | [✅](src/import.rs#L2211) | [✅](src/import.rs#L1871) |
| [Dedenne](src/import.rs#L2031) | [✅](src/import.rs#L2031) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2156) | [✅](src/import.rs#L2156) | [✅](src/import.rs#L1944) |
| [Dragapult ex](src/import.rs#L2038) | [✅](src/import.rs#L2038) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1899) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2045) | [✅](src/import.rs#L2045) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1923) |
| [Dudunsparce ex](src/import.rs#L2006) | [✅](src/import.rs#L2006) | — |
| [Dunsparce](src/import.rs#L2057) | [✅](src/import.rs#L2057) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1927) |
| [Dusknoir](src/import.rs#L2138) | [✅](src/import.rs#L2138) | [✅](src/import.rs#L1928) |
| [Duskull](src/import.rs#L2060) | [✅](src/import.rs#L2060) | — |
| [Dwebble](src/import.rs#L2050) | [✅](src/import.rs#L2050) | — |
| [Elgyem](src/import.rs#L2080) | [✅](src/import.rs#L2080) | — |
| [Enamorus](src/import.rs#L2100) | [✅](src/import.rs#L2100) | — |
| [Fan Rotom](src/import.rs#L2145) | [✅](src/import.rs#L2145) | [✅](src/import.rs#L1969) |
| [Fezandipiti ex](src/import.rs#L2219) | [✅](src/import.rs#L2219) | [✅](src/import.rs#L1917) |
| [Flutter Mane](src/import.rs#L2136) | [✅](src/import.rs#L2136) | [✅](src/import.rs#L1893) |
| [Genesect](src/import.rs#L2191) | [✅](src/import.rs#L2191) | [✅](src/import.rs#L1932) |
| [Genesect ex](src/import.rs#L2139) | [✅](src/import.rs#L2139) | [✅](src/import.rs#L1929) |
| [Goldeen](src/import.rs#L2154) | [✅](src/import.rs#L2154) | [✅](src/import.rs#L1942) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2047) | [✅](src/import.rs#L2047) | [✅](src/import.rs#L1883) |
| [Hydrapple ex](src/import.rs#L2105) | [✅](src/import.rs#L2105) | [✅](src/import.rs#L1884) |
| [Iron Crown ex](src/import.rs#L2065) | [✅](src/import.rs#L2065) | [✅](src/import.rs#L1879) |
| [Iron Leaves ex](src/import.rs#L2144) | [✅](src/import.rs#L2144) | [✅](src/import.rs#L1960) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1912) |
| [Koraidon ex](src/import.rs#L2066) | [✅](src/import.rs#L2066) | — |
| [Kyurem](src/import.rs#L2188) | [✅](src/import.rs#L2188) | [✅](src/import.rs#L1951) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2205) | [✅](src/import.rs#L2205) | [✅](src/import.rs#L1870) |
| [Lillie's Clefairy ex](src/import.rs#L2229) | [✅](src/import.rs#L2229) | [✅](src/import.rs#L1874) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2112) | [✅](src/import.rs#L2112) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2108) | [✅](src/import.rs#L2108) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2215) | [✅](src/import.rs#L2215) | [✅](src/import.rs#L1867) |
| [Mega Lopunny ex](src/import.rs#L2013) | [✅](src/import.rs#L2013) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2048) | [✅](src/import.rs#L2048) | — |
| [Mega Skarmory ex](src/import.rs#L2126) | [✅](src/import.rs#L2126) | — |
| [Mega Slowbro ex](src/import.rs#L2166) | [✅](src/import.rs#L2166) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1938) |
| [Meowth ex](src/import.rs#L2218) | [✅](src/import.rs#L2218) | [✅](src/import.rs#L1911) |
| [Metagross](src/import.rs#L2022) | [✅](src/import.rs#L2022) | — |
| [Metang](src/import.rs#L2206) | [✅](src/import.rs#L2206) | [✅](src/import.rs#L1902) |
| [Moltres](src/import.rs#L2058) | [✅](src/import.rs#L2058) | — |
| [Munkidori](src/import.rs#L2212) | [✅](src/import.rs#L2212) | [✅](src/import.rs#L1905) |
| [N's Darmanitan](src/import.rs#L2003) | [✅](src/import.rs#L2003) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2000) | [✅](src/import.rs#L2000) | — |
| [N's Zekrom](src/import.rs#L2012) | [✅](src/import.rs#L2012) | — |
| [N's Zoroark ex](src/import.rs#L2185) | [✅](src/import.rs#L2185) | [✅](src/import.rs#L1935) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2208) | [✅](src/import.rs#L2208) | [✅](src/import.rs#L1913) |
| [Paldean Tauros](src/import.rs#L1996) | [✅](src/import.rs#L1996) | — |
| [Passimian](src/import.rs#L2009) | [✅](src/import.rs#L2009) | — |
| [Patrat](src/import.rs#L2207) | [✅](src/import.rs#L2207) | [✅](src/import.rs#L1872) |
| [Pecharunt](src/import.rs#L2149) | [✅](src/import.rs#L2149) | [✅](src/import.rs#L1939) |
| [Pecharunt ex](src/import.rs#L2146) | [✅](src/import.rs#L2146) | [✅](src/import.rs#L1976) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1873) |
| [Rabsca](src/import.rs#L2062) | [✅](src/import.rs#L2062) | [✅](src/import.rs#L1878) |
| [Raging Bolt ex](src/import.rs#L2027) | [✅](src/import.rs#L2027) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L1994) | [✅](src/import.rs#L1994) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2155) | [✅](src/import.rs#L2155) | [✅](src/import.rs#L1943) |
| [Shaymin](src/import.rs#L2202) | [✅](src/import.rs#L2202) | [✅](src/import.rs#L1877) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2051) | [✅](src/import.rs#L2051) | — |
| [Slowpoke](src/import.rs#L2059) | [✅](src/import.rs#L2059) | ❌ |
| [Smoochum](src/import.rs#L2119) | [✅](src/import.rs#L2119) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2101) | [✅](src/import.rs#L2101) | — |
| [Tapu Bulu](src/import.rs#L1995) | [✅](src/import.rs#L1995) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1896) |
| [Teal Mask Ogerpon ex](src/import.rs#L2220) | [✅](src/import.rs#L2220) | [✅](src/import.rs#L1920) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1945) |
| [Torchic](src/import.rs#L2061) | [✅](src/import.rs#L2061) | — |
| [Toxel](src/import.rs#L2046) | [✅](src/import.rs#L2046) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1963) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2036) | [✅](src/import.rs#L2036) | — |
| [Yveltal](src/import.rs#L2035) | [✅](src/import.rs#L2035) | — |
| [Zeraora](src/import.rs#L2018) | [✅](src/import.rs#L2018) | — |
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

