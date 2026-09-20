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
| Supporters | 64 | 78 |
| Items | 59 | 85 |
| Tools | 22 | 35 |
| Stadiums | 19 | 31 |
| Special Energy | 15 | 17 |

### Supporters (36/36 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1239) | ✅ |
| [Acerola's Mischief](src/import.rs#L1414) | ✅ |
| [Bianca's Devotion](src/import.rs#L1224) | ✅ |
| [Black Belt's Training](src/import.rs#L1247) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L802) | ✅ |
| [Brock's Scouting](src/import.rs#L1309) | ✅ |
| [Carmine](src/import.rs#L626) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1178) | ✅ |
| [Crispin](src/import.rs#L1084) | ✅ |
| [Cyrano](src/import.rs#L932) | ✅ |
| [Dawn](src/import.rs#L1050) | ✅ |
| [Eri](src/import.rs#L1302) | ✅ |
| [Explorer's Guidance](src/import.rs#L737) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1255) | ✅ |
| [Gwynn](src/import.rs#L946) | ✅ |
| [Hilda](src/import.rs#L1003) | ✅ |
| [Janine's Secret Art](src/import.rs#L1333) | ✅ |
| [Judge](src/import.rs#L825) | ✅ |
| [Kieran](src/import.rs#L1287) | ✅ |
| [Lana's Aid](src/import.rs#L1204) | ✅ |
| [Lillie's Determination](src/import.rs#L826) | ✅ |
| [Lisia's Appeal](src/import.rs#L1408) | ✅ |
| [Morty's Conviction](src/import.rs#L1297) | ✅ |
| [N's Plan](src/import.rs#L1222) | ✅ |
| [Philippe](src/import.rs#L641) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1225) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1218) | ✅ |
| [Salvatore](src/import.rs#L1407) | ✅ |
| [Surfer](src/import.rs#L1243) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1395) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1403) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1108) | ✅ |
| [Team Rocket's Proton](src/import.rs#L431) | ✅ |
| [Wally's Compassion](src/import.rs#L1332) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1301) | ✅ |

### Items (37/37 built)

| Card | Status |
| --- | --- |
| [Brilliant Blender](src/import.rs#L507) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L918) | ✅ |
| [Bug Catching Set](src/import.rs#L1164) | ✅ |
| [Crushing Hammer](src/import.rs#L861) | ✅ |
| [Dark Bell](src/import.rs#L502) | ✅ |
| [Dusk Ball](src/import.rs#L1363) | ✅ |
| [Energy Recycler](src/import.rs#L1491) | ✅ |
| [Energy Retrieval](src/import.rs#L1348) | ✅ |
| [Energy Search](src/import.rs#L1334) | ✅ |
| [Energy Switch](src/import.rs#L988) | ✅ |
| [Enhanced Hammer](src/import.rs#L806) | ✅ |
| [Fighting Gong](src/import.rs#L1369) | ✅ |
| [Glass Trumpet](src/import.rs#L807) | ✅ |
| [Hand Trimmer](src/import.rs#L1362) | ✅ |
| [Iron Defender](src/import.rs#L390) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1200) | ✅ |
| [Max Rod](src/import.rs#L577) | ✅ |
| [N's PP Up](src/import.rs#L1122) | ✅ |
| [Night Stretcher](src/import.rs#L833) | ✅ |
| [Precious Trolley](src/import.rs#L862) | ✅ |
| [Premium Power Pro](src/import.rs#L1251) | ✅ |
| [Prime Catcher](src/import.rs#L1364) | ✅ |
| [Rare Candy](src/import.rs#L1107) | ✅ |
| [Sacred Ash](src/import.rs#L960) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1409) | ✅ |
| [Secret Box](src/import.rs#L1440) | ✅ |
| [Special Red Card](src/import.rs#L1080) | ✅ |
| [Strange Timepiece](src/import.rs#L1383) | ✅ |
| [Switch](src/import.rs#L1199) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1477) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1410) | ✅ |
| [Tera Orb](src/import.rs#L989) | ✅ |
| [Tool Scrapper](src/import.rs#L801) | ✅ |
| [Transformation Tome](src/import.rs#L1436) | ✅ |
| [Ultra Ball](src/import.rs#L974) | ✅ |
| [Unfair Stamp](src/import.rs#L1192) | ✅ |
| [Wondrous Patch](src/import.rs#L1136) | ✅ |

### Tools (10/10 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1384) | ✅ |
| [Binding Mochi](src/import.rs#L1387) | ✅ |
| [Brave Bangle](src/import.rs#L1386) | ✅ |
| [Handheld Fan](src/import.rs#L1391) | ✅ |
| [Hero's Cape](src/import.rs#L1385) | ✅ |
| [Lillie's Pearl](src/import.rs#L1388) | ✅ |
| [Lucky Helmet](src/import.rs#L1390) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Powerglass](src/import.rs#L1392) | ✅ |
| [Punk Helmet](src/import.rs#L1389) | ✅ |

### Stadiums (15/15 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1418) | ✅ |
| [Area Zero Underdepths](src/import.rs#L822) | ✅ |
| [Battle Cage](src/import.rs#L823) | ✅ |
| [Community Center](src/import.rs#L1425) | ✅ |
| [Festival Grounds](src/import.rs#L1432) | ✅ |
| [Forest of Vitality](src/import.rs#L1431) | ✅ |
| [Gravity Mountain](src/import.rs#L1393) | ✅ |
| [Jamming Tower](src/import.rs#L1429) | ✅ |
| [Lumiose City](src/import.rs#L1423) | ✅ |
| [N's Castle](src/import.rs#L1394) | ✅ |
| [Nighttime Mine](src/import.rs#L821) | ✅ |
| [Prism Tower](src/import.rs#L1424) | ✅ |
| [Risky Ruins](src/import.rs#L1430) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1419) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L824) | ✅ |

### Special Energy (9/11 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1870) | ✅ |
| [Enriching Energy](src/import.rs#L1853) | ✅ |
| [Growing Grass Energy](src/import.rs#L1852) | ✅ |
| Legacy Energy | ❌ |
| [Mist Energy](src/import.rs#L1867) | ✅ |
| [Neo Upper Energy](src/import.rs#L1900) | ✅ |
| [Prism Energy](src/import.rs#L1873) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L1876) | ✅ |
| [Spiky Energy](src/import.rs#L1864) | ✅ |
| Team Rocket's Energy | ❌ |
| [Telepathic Psychic Energy](src/import.rs#L1856) | ✅ |

### Pokémon (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2087) | [✅](src/import.rs#L2087) | [✅](src/import.rs#L1969) |
| [Alakazam](src/import.rs#L2242) | [✅](src/import.rs#L2242) | [✅](src/import.rs#L1959) |
| [Annihilape](src/import.rs#L2116) | [✅](src/import.rs#L2116) | [✅](src/import.rs#L1925) |
| [Applin](src/import.rs#L2077) | [✅](src/import.rs#L2077) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2119) | [✅](src/import.rs#L2119) | — |
| [Beldum](src/import.rs#L2099) | [✅](src/import.rs#L2099) | — |
| [Blaziken ex](src/import.rs#L2183) | [✅](src/import.rs#L2183) | [✅](src/import.rs#L1999) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2147) | [✅](src/import.rs#L2147) | [✅](src/import.rs#L1933) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2058) | [✅](src/import.rs#L2058) | — |
| [Budew](src/import.rs#L2124) | [✅](src/import.rs#L2124) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2118) | [✅](src/import.rs#L2118) | — |
| [Carvanha](src/import.rs#L2036) | [✅](src/import.rs#L2036) | — |
| [Celebi](src/import.rs#L2117) | [✅](src/import.rs#L2117) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2215) | [✅](src/import.rs#L2215) | — |
| [Chien-Pao](src/import.rs#L2184) | [✅](src/import.rs#L2184) | [✅](src/import.rs#L2002) |
| [Chikorita](src/import.rs#L2120) | [✅](src/import.rs#L2120) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2165) | [✅](src/import.rs#L2165) | — |
| [Combusken](src/import.rs#L2134) | [✅](src/import.rs#L2134) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2254) | [✅](src/import.rs#L2254) | [✅](src/import.rs#L1914) |
| [Dedenne](src/import.rs#L2074) | [✅](src/import.rs#L2074) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2199) | [✅](src/import.rs#L2199) | [✅](src/import.rs#L1987) |
| [Dragapult ex](src/import.rs#L2081) | [✅](src/import.rs#L2081) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1942) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2088) | [✅](src/import.rs#L2088) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1966) |
| [Dudunsparce ex](src/import.rs#L2049) | [✅](src/import.rs#L2049) | — |
| [Dunsparce](src/import.rs#L2100) | [✅](src/import.rs#L2100) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1970) |
| [Dusknoir](src/import.rs#L2181) | [✅](src/import.rs#L2181) | [✅](src/import.rs#L1971) |
| [Duskull](src/import.rs#L2103) | [✅](src/import.rs#L2103) | — |
| [Dwebble](src/import.rs#L2093) | [✅](src/import.rs#L2093) | — |
| [Elgyem](src/import.rs#L2123) | [✅](src/import.rs#L2123) | — |
| [Enamorus](src/import.rs#L2143) | [✅](src/import.rs#L2143) | — |
| [Fan Rotom](src/import.rs#L2188) | [✅](src/import.rs#L2188) | [✅](src/import.rs#L2012) |
| [Fezandipiti ex](src/import.rs#L2262) | [✅](src/import.rs#L2262) | [✅](src/import.rs#L1960) |
| [Flutter Mane](src/import.rs#L2179) | [✅](src/import.rs#L2179) | [✅](src/import.rs#L1936) |
| [Genesect](src/import.rs#L2234) | [✅](src/import.rs#L2234) | [✅](src/import.rs#L1975) |
| [Genesect ex](src/import.rs#L2182) | [✅](src/import.rs#L2182) | [✅](src/import.rs#L1972) |
| [Goldeen](src/import.rs#L2197) | [✅](src/import.rs#L2197) | [✅](src/import.rs#L1985) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2090) | [✅](src/import.rs#L2090) | [✅](src/import.rs#L1926) |
| [Hydrapple ex](src/import.rs#L2148) | [✅](src/import.rs#L2148) | [✅](src/import.rs#L1927) |
| [Iron Crown ex](src/import.rs#L2108) | [✅](src/import.rs#L2108) | [✅](src/import.rs#L1922) |
| [Iron Leaves ex](src/import.rs#L2187) | [✅](src/import.rs#L2187) | [✅](src/import.rs#L2003) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1955) |
| [Koraidon ex](src/import.rs#L2109) | [✅](src/import.rs#L2109) | — |
| [Kyurem](src/import.rs#L2231) | [✅](src/import.rs#L2231) | [✅](src/import.rs#L1994) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2248) | [✅](src/import.rs#L2248) | [✅](src/import.rs#L1913) |
| [Lillie's Clefairy ex](src/import.rs#L2272) | [✅](src/import.rs#L2272) | [✅](src/import.rs#L1917) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2155) | [✅](src/import.rs#L2155) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2151) | [✅](src/import.rs#L2151) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2258) | [✅](src/import.rs#L2258) | [✅](src/import.rs#L1910) |
| [Mega Lopunny ex](src/import.rs#L2056) | [✅](src/import.rs#L2056) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2091) | [✅](src/import.rs#L2091) | — |
| [Mega Skarmory ex](src/import.rs#L2169) | [✅](src/import.rs#L2169) | — |
| [Mega Slowbro ex](src/import.rs#L2209) | [✅](src/import.rs#L2209) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1981) |
| [Meowth ex](src/import.rs#L2261) | [✅](src/import.rs#L2261) | [✅](src/import.rs#L1954) |
| [Metagross](src/import.rs#L2065) | [✅](src/import.rs#L2065) | — |
| [Metang](src/import.rs#L2249) | [✅](src/import.rs#L2249) | [✅](src/import.rs#L1945) |
| [Moltres](src/import.rs#L2101) | [✅](src/import.rs#L2101) | — |
| [Munkidori](src/import.rs#L2255) | [✅](src/import.rs#L2255) | [✅](src/import.rs#L1948) |
| [N's Darmanitan](src/import.rs#L2046) | [✅](src/import.rs#L2046) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2043) | [✅](src/import.rs#L2043) | — |
| [N's Zekrom](src/import.rs#L2055) | [✅](src/import.rs#L2055) | — |
| [N's Zoroark ex](src/import.rs#L2228) | [✅](src/import.rs#L2228) | [✅](src/import.rs#L1978) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2251) | [✅](src/import.rs#L2251) | [✅](src/import.rs#L1956) |
| [Paldean Tauros](src/import.rs#L2039) | [✅](src/import.rs#L2039) | — |
| [Passimian](src/import.rs#L2052) | [✅](src/import.rs#L2052) | — |
| [Patrat](src/import.rs#L2250) | [✅](src/import.rs#L2250) | [✅](src/import.rs#L1915) |
| [Pecharunt](src/import.rs#L2192) | [✅](src/import.rs#L2192) | [✅](src/import.rs#L1982) |
| [Pecharunt ex](src/import.rs#L2189) | [✅](src/import.rs#L2189) | [✅](src/import.rs#L2019) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1916) |
| [Rabsca](src/import.rs#L2105) | [✅](src/import.rs#L2105) | [✅](src/import.rs#L1921) |
| [Raging Bolt ex](src/import.rs#L2070) | [✅](src/import.rs#L2070) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L2037) | [✅](src/import.rs#L2037) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2198) | [✅](src/import.rs#L2198) | [✅](src/import.rs#L1986) |
| [Shaymin](src/import.rs#L2245) | [✅](src/import.rs#L2245) | [✅](src/import.rs#L1920) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2094) | [✅](src/import.rs#L2094) | — |
| [Slowpoke](src/import.rs#L2102) | [✅](src/import.rs#L2102) | ❌ |
| [Smoochum](src/import.rs#L2162) | [✅](src/import.rs#L2162) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2144) | [✅](src/import.rs#L2144) | — |
| [Tapu Bulu](src/import.rs#L2038) | [✅](src/import.rs#L2038) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1939) |
| [Teal Mask Ogerpon ex](src/import.rs#L2263) | [✅](src/import.rs#L2263) | [✅](src/import.rs#L1963) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1988) |
| [Torchic](src/import.rs#L2104) | [✅](src/import.rs#L2104) | — |
| [Toxel](src/import.rs#L2089) | [✅](src/import.rs#L2089) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2006) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2079) | [✅](src/import.rs#L2079) | — |
| [Yveltal](src/import.rs#L2078) | [✅](src/import.rs#L2078) | — |
| [Zeraora](src/import.rs#L2061) | [✅](src/import.rs#L2061) | — |
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

