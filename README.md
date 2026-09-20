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
| Supporters | 65 | 78 |
| Items | 60 | 85 |
| Tools | 22 | 35 |
| Stadiums | 19 | 31 |
| Special Energy | 15 | 17 |

### Supporters (36/36 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1254) | ✅ |
| [Acerola's Mischief](src/import.rs#L1429) | ✅ |
| [Bianca's Devotion](src/import.rs#L1239) | ✅ |
| [Black Belt's Training](src/import.rs#L1262) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L817) | ✅ |
| [Brock's Scouting](src/import.rs#L1324) | ✅ |
| [Carmine](src/import.rs#L641) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1193) | ✅ |
| [Crispin](src/import.rs#L1099) | ✅ |
| [Cyrano](src/import.rs#L947) | ✅ |
| [Dawn](src/import.rs#L1065) | ✅ |
| [Eri](src/import.rs#L1317) | ✅ |
| [Explorer's Guidance](src/import.rs#L752) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1270) | ✅ |
| [Gwynn](src/import.rs#L961) | ✅ |
| [Hilda](src/import.rs#L1018) | ✅ |
| [Janine's Secret Art](src/import.rs#L1348) | ✅ |
| [Judge](src/import.rs#L840) | ✅ |
| [Kieran](src/import.rs#L1302) | ✅ |
| [Lana's Aid](src/import.rs#L1219) | ✅ |
| [Lillie's Determination](src/import.rs#L841) | ✅ |
| [Lisia's Appeal](src/import.rs#L1423) | ✅ |
| [Morty's Conviction](src/import.rs#L1312) | ✅ |
| [N's Plan](src/import.rs#L1237) | ✅ |
| [Philippe](src/import.rs#L656) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1240) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1233) | ✅ |
| [Salvatore](src/import.rs#L1422) | ✅ |
| [Surfer](src/import.rs#L1258) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1410) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1418) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1123) | ✅ |
| [Team Rocket's Proton](src/import.rs#L431) | ✅ |
| [Wally's Compassion](src/import.rs#L1347) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1316) | ✅ |

### Items (37/37 built)

| Card | Status |
| --- | --- |
| [Brilliant Blender](src/import.rs#L507) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L933) | ✅ |
| [Bug Catching Set](src/import.rs#L1179) | ✅ |
| [Crushing Hammer](src/import.rs#L876) | ✅ |
| [Dark Bell](src/import.rs#L502) | ✅ |
| [Dusk Ball](src/import.rs#L1378) | ✅ |
| [Energy Recycler](src/import.rs#L1506) | ✅ |
| [Energy Retrieval](src/import.rs#L1363) | ✅ |
| [Energy Search](src/import.rs#L1349) | ✅ |
| [Energy Switch](src/import.rs#L1003) | ✅ |
| [Enhanced Hammer](src/import.rs#L821) | ✅ |
| [Fighting Gong](src/import.rs#L1384) | ✅ |
| [Glass Trumpet](src/import.rs#L822) | ✅ |
| [Hand Trimmer](src/import.rs#L1377) | ✅ |
| [Iron Defender](src/import.rs#L390) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1215) | ✅ |
| [Max Rod](src/import.rs#L577) | ✅ |
| [N's PP Up](src/import.rs#L1137) | ✅ |
| [Night Stretcher](src/import.rs#L848) | ✅ |
| [Precious Trolley](src/import.rs#L877) | ✅ |
| [Premium Power Pro](src/import.rs#L1266) | ✅ |
| [Prime Catcher](src/import.rs#L1379) | ✅ |
| [Rare Candy](src/import.rs#L1122) | ✅ |
| [Sacred Ash](src/import.rs#L975) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1424) | ✅ |
| [Secret Box](src/import.rs#L1455) | ✅ |
| [Special Red Card](src/import.rs#L1095) | ✅ |
| [Strange Timepiece](src/import.rs#L1398) | ✅ |
| [Switch](src/import.rs#L1214) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1492) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1425) | ✅ |
| [Tera Orb](src/import.rs#L1004) | ✅ |
| [Tool Scrapper](src/import.rs#L816) | ✅ |
| [Transformation Tome](src/import.rs#L1451) | ✅ |
| [Ultra Ball](src/import.rs#L989) | ✅ |
| [Unfair Stamp](src/import.rs#L1207) | ✅ |
| [Wondrous Patch](src/import.rs#L1151) | ✅ |

### Tools (10/10 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1399) | ✅ |
| [Binding Mochi](src/import.rs#L1402) | ✅ |
| [Brave Bangle](src/import.rs#L1401) | ✅ |
| [Handheld Fan](src/import.rs#L1406) | ✅ |
| [Hero's Cape](src/import.rs#L1400) | ✅ |
| [Lillie's Pearl](src/import.rs#L1403) | ✅ |
| [Lucky Helmet](src/import.rs#L1405) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Powerglass](src/import.rs#L1407) | ✅ |
| [Punk Helmet](src/import.rs#L1404) | ✅ |

### Stadiums (15/15 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1433) | ✅ |
| [Area Zero Underdepths](src/import.rs#L837) | ✅ |
| [Battle Cage](src/import.rs#L838) | ✅ |
| [Community Center](src/import.rs#L1440) | ✅ |
| [Festival Grounds](src/import.rs#L1447) | ✅ |
| [Forest of Vitality](src/import.rs#L1446) | ✅ |
| [Gravity Mountain](src/import.rs#L1408) | ✅ |
| [Jamming Tower](src/import.rs#L1444) | ✅ |
| [Lumiose City](src/import.rs#L1438) | ✅ |
| [N's Castle](src/import.rs#L1409) | ✅ |
| [Nighttime Mine](src/import.rs#L836) | ✅ |
| [Prism Tower](src/import.rs#L1439) | ✅ |
| [Risky Ruins](src/import.rs#L1445) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1434) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L839) | ✅ |

### Special Energy (9/11 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1885) | ✅ |
| [Enriching Energy](src/import.rs#L1868) | ✅ |
| [Growing Grass Energy](src/import.rs#L1867) | ✅ |
| Legacy Energy | ❌ |
| [Mist Energy](src/import.rs#L1882) | ✅ |
| [Neo Upper Energy](src/import.rs#L1915) | ✅ |
| [Prism Energy](src/import.rs#L1888) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L1891) | ✅ |
| [Spiky Energy](src/import.rs#L1879) | ✅ |
| Team Rocket's Energy | ❌ |
| [Telepathic Psychic Energy](src/import.rs#L1871) | ✅ |

### Pokémon (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2102) | [✅](src/import.rs#L2102) | [✅](src/import.rs#L1984) |
| [Alakazam](src/import.rs#L2257) | [✅](src/import.rs#L2257) | [✅](src/import.rs#L1974) |
| [Annihilape](src/import.rs#L2131) | [✅](src/import.rs#L2131) | [✅](src/import.rs#L1940) |
| [Applin](src/import.rs#L2092) | [✅](src/import.rs#L2092) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2134) | [✅](src/import.rs#L2134) | — |
| [Beldum](src/import.rs#L2114) | [✅](src/import.rs#L2114) | — |
| [Blaziken ex](src/import.rs#L2198) | [✅](src/import.rs#L2198) | [✅](src/import.rs#L2014) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2162) | [✅](src/import.rs#L2162) | [✅](src/import.rs#L1948) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2073) | [✅](src/import.rs#L2073) | — |
| [Budew](src/import.rs#L2139) | [✅](src/import.rs#L2139) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2133) | [✅](src/import.rs#L2133) | — |
| [Carvanha](src/import.rs#L2051) | [✅](src/import.rs#L2051) | — |
| [Celebi](src/import.rs#L2132) | [✅](src/import.rs#L2132) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2230) | [✅](src/import.rs#L2230) | — |
| [Chien-Pao](src/import.rs#L2199) | [✅](src/import.rs#L2199) | [✅](src/import.rs#L2017) |
| [Chikorita](src/import.rs#L2135) | [✅](src/import.rs#L2135) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2180) | [✅](src/import.rs#L2180) | — |
| [Combusken](src/import.rs#L2149) | [✅](src/import.rs#L2149) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2269) | [✅](src/import.rs#L2269) | [✅](src/import.rs#L1929) |
| [Dedenne](src/import.rs#L2089) | [✅](src/import.rs#L2089) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2214) | [✅](src/import.rs#L2214) | [✅](src/import.rs#L2002) |
| [Dragapult ex](src/import.rs#L2096) | [✅](src/import.rs#L2096) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1957) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2103) | [✅](src/import.rs#L2103) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1981) |
| [Dudunsparce ex](src/import.rs#L2064) | [✅](src/import.rs#L2064) | — |
| [Dunsparce](src/import.rs#L2115) | [✅](src/import.rs#L2115) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1985) |
| [Dusknoir](src/import.rs#L2196) | [✅](src/import.rs#L2196) | [✅](src/import.rs#L1986) |
| [Duskull](src/import.rs#L2118) | [✅](src/import.rs#L2118) | — |
| [Dwebble](src/import.rs#L2108) | [✅](src/import.rs#L2108) | — |
| [Elgyem](src/import.rs#L2138) | [✅](src/import.rs#L2138) | — |
| [Enamorus](src/import.rs#L2158) | [✅](src/import.rs#L2158) | — |
| [Fan Rotom](src/import.rs#L2203) | [✅](src/import.rs#L2203) | [✅](src/import.rs#L2027) |
| [Fezandipiti ex](src/import.rs#L2277) | [✅](src/import.rs#L2277) | [✅](src/import.rs#L1975) |
| [Flutter Mane](src/import.rs#L2194) | [✅](src/import.rs#L2194) | [✅](src/import.rs#L1951) |
| [Genesect](src/import.rs#L2249) | [✅](src/import.rs#L2249) | [✅](src/import.rs#L1990) |
| [Genesect ex](src/import.rs#L2197) | [✅](src/import.rs#L2197) | [✅](src/import.rs#L1987) |
| [Goldeen](src/import.rs#L2212) | [✅](src/import.rs#L2212) | [✅](src/import.rs#L2000) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2105) | [✅](src/import.rs#L2105) | [✅](src/import.rs#L1941) |
| [Hydrapple ex](src/import.rs#L2163) | [✅](src/import.rs#L2163) | [✅](src/import.rs#L1942) |
| [Iron Crown ex](src/import.rs#L2123) | [✅](src/import.rs#L2123) | [✅](src/import.rs#L1937) |
| [Iron Leaves ex](src/import.rs#L2202) | [✅](src/import.rs#L2202) | [✅](src/import.rs#L2018) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1970) |
| [Koraidon ex](src/import.rs#L2124) | [✅](src/import.rs#L2124) | — |
| [Kyurem](src/import.rs#L2246) | [✅](src/import.rs#L2246) | [✅](src/import.rs#L2009) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2263) | [✅](src/import.rs#L2263) | [✅](src/import.rs#L1928) |
| [Lillie's Clefairy ex](src/import.rs#L2287) | [✅](src/import.rs#L2287) | [✅](src/import.rs#L1932) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2170) | [✅](src/import.rs#L2170) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2166) | [✅](src/import.rs#L2166) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2273) | [✅](src/import.rs#L2273) | [✅](src/import.rs#L1925) |
| [Mega Lopunny ex](src/import.rs#L2071) | [✅](src/import.rs#L2071) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2106) | [✅](src/import.rs#L2106) | — |
| [Mega Skarmory ex](src/import.rs#L2184) | [✅](src/import.rs#L2184) | — |
| [Mega Slowbro ex](src/import.rs#L2224) | [✅](src/import.rs#L2224) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1996) |
| [Meowth ex](src/import.rs#L2276) | [✅](src/import.rs#L2276) | [✅](src/import.rs#L1969) |
| [Metagross](src/import.rs#L2080) | [✅](src/import.rs#L2080) | — |
| [Metang](src/import.rs#L2264) | [✅](src/import.rs#L2264) | [✅](src/import.rs#L1960) |
| [Moltres](src/import.rs#L2116) | [✅](src/import.rs#L2116) | — |
| [Munkidori](src/import.rs#L2270) | [✅](src/import.rs#L2270) | [✅](src/import.rs#L1963) |
| [N's Darmanitan](src/import.rs#L2061) | [✅](src/import.rs#L2061) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2058) | [✅](src/import.rs#L2058) | — |
| [N's Zekrom](src/import.rs#L2070) | [✅](src/import.rs#L2070) | — |
| [N's Zoroark ex](src/import.rs#L2243) | [✅](src/import.rs#L2243) | [✅](src/import.rs#L1993) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2266) | [✅](src/import.rs#L2266) | [✅](src/import.rs#L1971) |
| [Paldean Tauros](src/import.rs#L2054) | [✅](src/import.rs#L2054) | — |
| [Passimian](src/import.rs#L2067) | [✅](src/import.rs#L2067) | — |
| [Patrat](src/import.rs#L2265) | [✅](src/import.rs#L2265) | [✅](src/import.rs#L1930) |
| [Pecharunt](src/import.rs#L2207) | [✅](src/import.rs#L2207) | [✅](src/import.rs#L1997) |
| [Pecharunt ex](src/import.rs#L2204) | [✅](src/import.rs#L2204) | [✅](src/import.rs#L2034) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1931) |
| [Rabsca](src/import.rs#L2120) | [✅](src/import.rs#L2120) | [✅](src/import.rs#L1936) |
| [Raging Bolt ex](src/import.rs#L2085) | [✅](src/import.rs#L2085) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L2052) | [✅](src/import.rs#L2052) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2213) | [✅](src/import.rs#L2213) | [✅](src/import.rs#L2001) |
| [Shaymin](src/import.rs#L2260) | [✅](src/import.rs#L2260) | [✅](src/import.rs#L1935) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2109) | [✅](src/import.rs#L2109) | — |
| [Slowpoke](src/import.rs#L2117) | [✅](src/import.rs#L2117) | ❌ |
| [Smoochum](src/import.rs#L2177) | [✅](src/import.rs#L2177) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2159) | [✅](src/import.rs#L2159) | — |
| [Tapu Bulu](src/import.rs#L2053) | [✅](src/import.rs#L2053) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1954) |
| [Teal Mask Ogerpon ex](src/import.rs#L2278) | [✅](src/import.rs#L2278) | [✅](src/import.rs#L1978) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2003) |
| [Torchic](src/import.rs#L2119) | [✅](src/import.rs#L2119) | — |
| [Toxel](src/import.rs#L2104) | [✅](src/import.rs#L2104) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2021) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2094) | [✅](src/import.rs#L2094) | — |
| [Yveltal](src/import.rs#L2093) | [✅](src/import.rs#L2093) | — |
| [Zeraora](src/import.rs#L2076) | [✅](src/import.rs#L2076) | — |
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

