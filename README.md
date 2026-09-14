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
| Supporters | 56 | 78 |
| Items | 57 | 85 |
| Tools | 20 | 35 |
| Stadiums | 17 | 31 |
| Special Energy | 13 | 17 |

### Supporters (29/36 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1219) | ✅ |
| Acerola's Mischief | ❌ |
| Bianca's Devotion | ❌ |
| [Black Belt's Training](src/import.rs#L1227) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L783) | ✅ |
| [Brock's Scouting](src/import.rs#L1261) | ✅ |
| [Carmine](src/import.rs#L621) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1159) | ✅ |
| [Crispin](src/import.rs#L1065) | ✅ |
| [Cyrano](src/import.rs#L913) | ✅ |
| [Dawn](src/import.rs#L1031) | ✅ |
| [Eri](src/import.rs#L1254) | ✅ |
| Explorer's Guidance | ❌ |
| [Gladion's Final Battle](src/import.rs#L1235) | ✅ |
| [Gwynn](src/import.rs#L927) | ✅ |
| [Hilda](src/import.rs#L984) | ✅ |
| [Janine's Secret Art](src/import.rs#L1285) | ✅ |
| [Judge](src/import.rs#L806) | ✅ |
| [Kieran](src/import.rs#L1239) | ✅ |
| [Lana's Aid](src/import.rs#L1185) | ✅ |
| [Lillie's Determination](src/import.rs#L807) | ✅ |
| Lisia's Appeal | ❌ |
| [Morty's Conviction](src/import.rs#L1249) | ✅ |
| [N's Plan](src/import.rs#L1203) | ✅ |
| [Philippe](src/import.rs#L636) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1205) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1199) | ✅ |
| Salvatore | ❌ |
| [Surfer](src/import.rs#L1223) | ✅ |
| Team Rocket's Ariana | ❌ |
| Team Rocket's Giovanni | ❌ |
| [Team Rocket's Petrel](src/import.rs#L1089) | ✅ |
| [Team Rocket's Proton](src/import.rs#L426) | ✅ |
| [Wally's Compassion](src/import.rs#L1284) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1253) | ✅ |

### Items (35/37 built)

| Card | Status |
| --- | --- |
| [Brilliant Blender](src/import.rs#L502) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L899) | ✅ |
| [Bug Catching Set](src/import.rs#L1145) | ✅ |
| [Crushing Hammer](src/import.rs#L842) | ✅ |
| [Dark Bell](src/import.rs#L497) | ✅ |
| [Dusk Ball](src/import.rs#L1315) | ✅ |
| [Energy Recycler](src/import.rs#L1415) | ✅ |
| [Energy Retrieval](src/import.rs#L1300) | ✅ |
| [Energy Search](src/import.rs#L1286) | ✅ |
| [Energy Switch](src/import.rs#L969) | ✅ |
| [Enhanced Hammer](src/import.rs#L787) | ✅ |
| [Fighting Gong](src/import.rs#L1321) | ✅ |
| [Glass Trumpet](src/import.rs#L788) | ✅ |
| [Hand Trimmer](src/import.rs#L1314) | ✅ |
| [Iron Defender](src/import.rs#L390) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1181) | ✅ |
| [Max Rod](src/import.rs#L572) | ✅ |
| [N's PP Up](src/import.rs#L1103) | ✅ |
| [Night Stretcher](src/import.rs#L814) | ✅ |
| [Precious Trolley](src/import.rs#L843) | ✅ |
| [Premium Power Pro](src/import.rs#L1231) | ✅ |
| [Prime Catcher](src/import.rs#L1316) | ✅ |
| [Rare Candy](src/import.rs#L1088) | ✅ |
| [Sacred Ash](src/import.rs#L941) | ✅ |
| Scoop Up Cyclone | ❌ |
| [Secret Box](src/import.rs#L1364) | ✅ |
| [Special Red Card](src/import.rs#L1061) | ✅ |
| [Strange Timepiece](src/import.rs#L1335) | ✅ |
| [Switch](src/import.rs#L1180) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1401) | ✅ |
| Team Rocket's Venture Bomb | ❌ |
| [Tera Orb](src/import.rs#L970) | ✅ |
| [Tool Scrapper](src/import.rs#L782) | ✅ |
| [Transformation Tome](src/import.rs#L1360) | ✅ |
| [Ultra Ball](src/import.rs#L955) | ✅ |
| [Unfair Stamp](src/import.rs#L1173) | ✅ |
| [Wondrous Patch](src/import.rs#L1117) | ✅ |

### Tools (10/10 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1336) | ✅ |
| [Binding Mochi](src/import.rs#L1339) | ✅ |
| [Brave Bangle](src/import.rs#L1338) | ✅ |
| [Handheld Fan](src/import.rs#L1343) | ✅ |
| [Hero's Cape](src/import.rs#L1337) | ✅ |
| [Lillie's Pearl](src/import.rs#L1340) | ✅ |
| [Lucky Helmet](src/import.rs#L1342) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Powerglass](src/import.rs#L1344) | ✅ |
| [Punk Helmet](src/import.rs#L1341) | ✅ |

### Stadiums (13/15 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1347) | ✅ |
| [Area Zero Underdepths](src/import.rs#L803) | ✅ |
| [Battle Cage](src/import.rs#L804) | ✅ |
| Community Center | ❌ |
| [Festival Grounds](src/import.rs#L1356) | ✅ |
| [Forest of Vitality](src/import.rs#L1355) | ✅ |
| [Gravity Mountain](src/import.rs#L1345) | ✅ |
| [Jamming Tower](src/import.rs#L1353) | ✅ |
| [Lumiose City](src/import.rs#L1352) | ✅ |
| [N's Castle](src/import.rs#L1346) | ✅ |
| [Nighttime Mine](src/import.rs#L802) | ✅ |
| Prism Tower | ❌ |
| [Risky Ruins](src/import.rs#L1354) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1348) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L805) | ✅ |

### Special Energy (8/11 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1794) | ✅ |
| [Enriching Energy](src/import.rs#L1777) | ✅ |
| [Growing Grass Energy](src/import.rs#L1776) | ✅ |
| Legacy Energy | ❌ |
| [Mist Energy](src/import.rs#L1791) | ✅ |
| Neo Upper Energy | ❌ |
| [Prism Energy](src/import.rs#L1797) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L1800) | ✅ |
| [Spiky Energy](src/import.rs#L1788) | ✅ |
| Team Rocket's Energy | ❌ |
| [Telepathic Psychic Energy](src/import.rs#L1780) | ✅ |

### Pokémon (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2001) | [✅](src/import.rs#L2001) | [✅](src/import.rs#L1883) |
| [Alakazam](src/import.rs#L2156) | [✅](src/import.rs#L2156) | [✅](src/import.rs#L1873) |
| [Annihilape](src/import.rs#L2030) | [✅](src/import.rs#L2030) | [✅](src/import.rs#L1839) |
| [Applin](src/import.rs#L1991) | [✅](src/import.rs#L1991) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2033) | [✅](src/import.rs#L2033) | — |
| [Beldum](src/import.rs#L2013) | [✅](src/import.rs#L2013) | — |
| [Blaziken ex](src/import.rs#L2097) | [✅](src/import.rs#L2097) | [✅](src/import.rs#L1913) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2061) | [✅](src/import.rs#L2061) | [✅](src/import.rs#L1847) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L1972) | [✅](src/import.rs#L1972) | — |
| [Budew](src/import.rs#L2038) | [✅](src/import.rs#L2038) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2032) | [✅](src/import.rs#L2032) | — |
| [Carvanha](src/import.rs#L1950) | [✅](src/import.rs#L1950) | — |
| [Celebi](src/import.rs#L2031) | [✅](src/import.rs#L2031) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2129) | [✅](src/import.rs#L2129) | — |
| [Chien-Pao](src/import.rs#L2098) | [✅](src/import.rs#L2098) | [✅](src/import.rs#L1916) |
| [Chikorita](src/import.rs#L2034) | [✅](src/import.rs#L2034) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2079) | [✅](src/import.rs#L2079) | — |
| [Combusken](src/import.rs#L2048) | [✅](src/import.rs#L2048) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2168) | [✅](src/import.rs#L2168) | [✅](src/import.rs#L1828) |
| [Dedenne](src/import.rs#L1988) | [✅](src/import.rs#L1988) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2113) | [✅](src/import.rs#L2113) | [✅](src/import.rs#L1901) |
| [Dragapult ex](src/import.rs#L1995) | [✅](src/import.rs#L1995) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1856) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2002) | [✅](src/import.rs#L2002) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1880) |
| [Dudunsparce ex](src/import.rs#L1963) | [✅](src/import.rs#L1963) | — |
| [Dunsparce](src/import.rs#L2014) | [✅](src/import.rs#L2014) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1884) |
| [Dusknoir](src/import.rs#L2095) | [✅](src/import.rs#L2095) | [✅](src/import.rs#L1885) |
| [Duskull](src/import.rs#L2017) | [✅](src/import.rs#L2017) | — |
| [Dwebble](src/import.rs#L2007) | [✅](src/import.rs#L2007) | — |
| [Elgyem](src/import.rs#L2037) | [✅](src/import.rs#L2037) | — |
| [Enamorus](src/import.rs#L2057) | [✅](src/import.rs#L2057) | — |
| [Fan Rotom](src/import.rs#L2102) | [✅](src/import.rs#L2102) | [✅](src/import.rs#L1926) |
| [Fezandipiti ex](src/import.rs#L2176) | [✅](src/import.rs#L2176) | [✅](src/import.rs#L1874) |
| [Flutter Mane](src/import.rs#L2093) | [✅](src/import.rs#L2093) | [✅](src/import.rs#L1850) |
| [Genesect](src/import.rs#L2148) | [✅](src/import.rs#L2148) | [✅](src/import.rs#L1889) |
| [Genesect ex](src/import.rs#L2096) | [✅](src/import.rs#L2096) | [✅](src/import.rs#L1886) |
| [Goldeen](src/import.rs#L2111) | [✅](src/import.rs#L2111) | [✅](src/import.rs#L1899) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2004) | [✅](src/import.rs#L2004) | [✅](src/import.rs#L1840) |
| [Hydrapple ex](src/import.rs#L2062) | [✅](src/import.rs#L2062) | [✅](src/import.rs#L1841) |
| [Iron Crown ex](src/import.rs#L2022) | [✅](src/import.rs#L2022) | [✅](src/import.rs#L1836) |
| [Iron Leaves ex](src/import.rs#L2101) | [✅](src/import.rs#L2101) | [✅](src/import.rs#L1917) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1869) |
| [Koraidon ex](src/import.rs#L2023) | [✅](src/import.rs#L2023) | — |
| [Kyurem](src/import.rs#L2145) | [✅](src/import.rs#L2145) | [✅](src/import.rs#L1908) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2162) | [✅](src/import.rs#L2162) | [✅](src/import.rs#L1827) |
| [Lillie's Clefairy ex](src/import.rs#L2186) | [✅](src/import.rs#L2186) | [✅](src/import.rs#L1831) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2069) | [✅](src/import.rs#L2069) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2065) | [✅](src/import.rs#L2065) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2172) | [✅](src/import.rs#L2172) | [✅](src/import.rs#L1824) |
| [Mega Lopunny ex](src/import.rs#L1970) | [✅](src/import.rs#L1970) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2005) | [✅](src/import.rs#L2005) | — |
| [Mega Skarmory ex](src/import.rs#L2083) | [✅](src/import.rs#L2083) | — |
| [Mega Slowbro ex](src/import.rs#L2123) | [✅](src/import.rs#L2123) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1895) |
| [Meowth ex](src/import.rs#L2175) | [✅](src/import.rs#L2175) | [✅](src/import.rs#L1868) |
| [Metagross](src/import.rs#L1979) | [✅](src/import.rs#L1979) | — |
| [Metang](src/import.rs#L2163) | [✅](src/import.rs#L2163) | [✅](src/import.rs#L1859) |
| [Moltres](src/import.rs#L2015) | [✅](src/import.rs#L2015) | — |
| [Munkidori](src/import.rs#L2169) | [✅](src/import.rs#L2169) | [✅](src/import.rs#L1862) |
| [N's Darmanitan](src/import.rs#L1960) | [✅](src/import.rs#L1960) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1957) | [✅](src/import.rs#L1957) | — |
| [N's Zekrom](src/import.rs#L1969) | [✅](src/import.rs#L1969) | — |
| [N's Zoroark ex](src/import.rs#L2142) | [✅](src/import.rs#L2142) | [✅](src/import.rs#L1892) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2165) | [✅](src/import.rs#L2165) | [✅](src/import.rs#L1870) |
| [Paldean Tauros](src/import.rs#L1953) | [✅](src/import.rs#L1953) | — |
| [Passimian](src/import.rs#L1966) | [✅](src/import.rs#L1966) | — |
| [Patrat](src/import.rs#L2164) | [✅](src/import.rs#L2164) | [✅](src/import.rs#L1829) |
| [Pecharunt](src/import.rs#L2106) | [✅](src/import.rs#L2106) | [✅](src/import.rs#L1896) |
| [Pecharunt ex](src/import.rs#L2103) | [✅](src/import.rs#L2103) | [✅](src/import.rs#L1933) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1830) |
| [Rabsca](src/import.rs#L2019) | [✅](src/import.rs#L2019) | [✅](src/import.rs#L1835) |
| [Raging Bolt ex](src/import.rs#L1984) | [✅](src/import.rs#L1984) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L1951) | [✅](src/import.rs#L1951) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2112) | [✅](src/import.rs#L2112) | [✅](src/import.rs#L1900) |
| [Shaymin](src/import.rs#L2159) | [✅](src/import.rs#L2159) | [✅](src/import.rs#L1834) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2008) | [✅](src/import.rs#L2008) | — |
| [Slowpoke](src/import.rs#L2016) | [✅](src/import.rs#L2016) | ❌ |
| [Smoochum](src/import.rs#L2076) | [✅](src/import.rs#L2076) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2058) | [✅](src/import.rs#L2058) | — |
| [Tapu Bulu](src/import.rs#L1952) | [✅](src/import.rs#L1952) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1853) |
| [Teal Mask Ogerpon ex](src/import.rs#L2177) | [✅](src/import.rs#L2177) | [✅](src/import.rs#L1877) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1902) |
| [Torchic](src/import.rs#L2018) | [✅](src/import.rs#L2018) | — |
| [Toxel](src/import.rs#L2003) | [✅](src/import.rs#L2003) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1920) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1993) | [✅](src/import.rs#L1993) | — |
| [Yveltal](src/import.rs#L1992) | [✅](src/import.rs#L1992) | — |
| [Zeraora](src/import.rs#L1975) | [✅](src/import.rs#L1975) | — |
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

