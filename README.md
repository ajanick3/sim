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
| Supporters | 66 | 78 |
| Items | 60 | 85 |
| Tools | 22 | 35 |
| Stadiums | 19 | 31 |
| Special Energy | 15 | 17 |

### Supporters (36/36 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1268) | ✅ |
| [Acerola's Mischief](src/import.rs#L1443) | ✅ |
| [Bianca's Devotion](src/import.rs#L1253) | ✅ |
| [Black Belt's Training](src/import.rs#L1276) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L831) | ✅ |
| [Brock's Scouting](src/import.rs#L1338) | ✅ |
| [Carmine](src/import.rs#L655) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1207) | ✅ |
| [Crispin](src/import.rs#L1113) | ✅ |
| [Cyrano](src/import.rs#L961) | ✅ |
| [Dawn](src/import.rs#L1079) | ✅ |
| [Eri](src/import.rs#L1331) | ✅ |
| [Explorer's Guidance](src/import.rs#L766) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1284) | ✅ |
| [Gwynn](src/import.rs#L975) | ✅ |
| [Hilda](src/import.rs#L1032) | ✅ |
| [Janine's Secret Art](src/import.rs#L1362) | ✅ |
| [Judge](src/import.rs#L854) | ✅ |
| [Kieran](src/import.rs#L1316) | ✅ |
| [Lana's Aid](src/import.rs#L1233) | ✅ |
| [Lillie's Determination](src/import.rs#L855) | ✅ |
| [Lisia's Appeal](src/import.rs#L1437) | ✅ |
| [Morty's Conviction](src/import.rs#L1326) | ✅ |
| [N's Plan](src/import.rs#L1251) | ✅ |
| [Philippe](src/import.rs#L670) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1254) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1247) | ✅ |
| [Salvatore](src/import.rs#L1436) | ✅ |
| [Surfer](src/import.rs#L1272) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1424) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1432) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1137) | ✅ |
| [Team Rocket's Proton](src/import.rs#L445) | ✅ |
| [Wally's Compassion](src/import.rs#L1361) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1330) | ✅ |

### Items (37/37 built)

| Card | Status |
| --- | --- |
| [Brilliant Blender](src/import.rs#L521) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L947) | ✅ |
| [Bug Catching Set](src/import.rs#L1193) | ✅ |
| [Crushing Hammer](src/import.rs#L890) | ✅ |
| [Dark Bell](src/import.rs#L516) | ✅ |
| [Dusk Ball](src/import.rs#L1392) | ✅ |
| [Energy Recycler](src/import.rs#L1520) | ✅ |
| [Energy Retrieval](src/import.rs#L1377) | ✅ |
| [Energy Search](src/import.rs#L1363) | ✅ |
| [Energy Switch](src/import.rs#L1017) | ✅ |
| [Enhanced Hammer](src/import.rs#L835) | ✅ |
| [Fighting Gong](src/import.rs#L1398) | ✅ |
| [Glass Trumpet](src/import.rs#L836) | ✅ |
| [Hand Trimmer](src/import.rs#L1391) | ✅ |
| [Iron Defender](src/import.rs#L390) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1229) | ✅ |
| [Max Rod](src/import.rs#L591) | ✅ |
| [N's PP Up](src/import.rs#L1151) | ✅ |
| [Night Stretcher](src/import.rs#L862) | ✅ |
| [Precious Trolley](src/import.rs#L891) | ✅ |
| [Premium Power Pro](src/import.rs#L1280) | ✅ |
| [Prime Catcher](src/import.rs#L1393) | ✅ |
| [Rare Candy](src/import.rs#L1136) | ✅ |
| [Sacred Ash](src/import.rs#L989) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1438) | ✅ |
| [Secret Box](src/import.rs#L1469) | ✅ |
| [Special Red Card](src/import.rs#L1109) | ✅ |
| [Strange Timepiece](src/import.rs#L1412) | ✅ |
| [Switch](src/import.rs#L1228) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1506) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1439) | ✅ |
| [Tera Orb](src/import.rs#L1018) | ✅ |
| [Tool Scrapper](src/import.rs#L830) | ✅ |
| [Transformation Tome](src/import.rs#L1465) | ✅ |
| [Ultra Ball](src/import.rs#L1003) | ✅ |
| [Unfair Stamp](src/import.rs#L1221) | ✅ |
| [Wondrous Patch](src/import.rs#L1165) | ✅ |

### Tools (10/10 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1413) | ✅ |
| [Binding Mochi](src/import.rs#L1416) | ✅ |
| [Brave Bangle](src/import.rs#L1415) | ✅ |
| [Handheld Fan](src/import.rs#L1420) | ✅ |
| [Hero's Cape](src/import.rs#L1414) | ✅ |
| [Lillie's Pearl](src/import.rs#L1417) | ✅ |
| [Lucky Helmet](src/import.rs#L1419) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Powerglass](src/import.rs#L1421) | ✅ |
| [Punk Helmet](src/import.rs#L1418) | ✅ |

### Stadiums (15/15 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1447) | ✅ |
| [Area Zero Underdepths](src/import.rs#L851) | ✅ |
| [Battle Cage](src/import.rs#L852) | ✅ |
| [Community Center](src/import.rs#L1454) | ✅ |
| [Festival Grounds](src/import.rs#L1461) | ✅ |
| [Forest of Vitality](src/import.rs#L1460) | ✅ |
| [Gravity Mountain](src/import.rs#L1422) | ✅ |
| [Jamming Tower](src/import.rs#L1458) | ✅ |
| [Lumiose City](src/import.rs#L1452) | ✅ |
| [N's Castle](src/import.rs#L1423) | ✅ |
| [Nighttime Mine](src/import.rs#L850) | ✅ |
| [Prism Tower](src/import.rs#L1453) | ✅ |
| [Risky Ruins](src/import.rs#L1459) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1448) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L853) | ✅ |

### Special Energy (9/11 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1899) | ✅ |
| [Enriching Energy](src/import.rs#L1882) | ✅ |
| [Growing Grass Energy](src/import.rs#L1881) | ✅ |
| Legacy Energy | ❌ |
| [Mist Energy](src/import.rs#L1896) | ✅ |
| [Neo Upper Energy](src/import.rs#L1929) | ✅ |
| [Prism Energy](src/import.rs#L1902) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L1905) | ✅ |
| [Spiky Energy](src/import.rs#L1893) | ✅ |
| Team Rocket's Energy | ❌ |
| [Telepathic Psychic Energy](src/import.rs#L1885) | ✅ |

### Pokémon (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2116) | [✅](src/import.rs#L2116) | [✅](src/import.rs#L1998) |
| [Alakazam](src/import.rs#L2271) | [✅](src/import.rs#L2271) | [✅](src/import.rs#L1988) |
| [Annihilape](src/import.rs#L2145) | [✅](src/import.rs#L2145) | [✅](src/import.rs#L1954) |
| [Applin](src/import.rs#L2106) | [✅](src/import.rs#L2106) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2148) | [✅](src/import.rs#L2148) | — |
| [Beldum](src/import.rs#L2128) | [✅](src/import.rs#L2128) | — |
| [Blaziken ex](src/import.rs#L2212) | [✅](src/import.rs#L2212) | [✅](src/import.rs#L2028) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2176) | [✅](src/import.rs#L2176) | [✅](src/import.rs#L1962) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2087) | [✅](src/import.rs#L2087) | — |
| [Budew](src/import.rs#L2153) | [✅](src/import.rs#L2153) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2147) | [✅](src/import.rs#L2147) | — |
| [Carvanha](src/import.rs#L2065) | [✅](src/import.rs#L2065) | — |
| [Celebi](src/import.rs#L2146) | [✅](src/import.rs#L2146) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2244) | [✅](src/import.rs#L2244) | — |
| [Chien-Pao](src/import.rs#L2213) | [✅](src/import.rs#L2213) | [✅](src/import.rs#L2031) |
| [Chikorita](src/import.rs#L2149) | [✅](src/import.rs#L2149) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2194) | [✅](src/import.rs#L2194) | — |
| [Combusken](src/import.rs#L2163) | [✅](src/import.rs#L2163) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2283) | [✅](src/import.rs#L2283) | [✅](src/import.rs#L1943) |
| [Dedenne](src/import.rs#L2103) | [✅](src/import.rs#L2103) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2228) | [✅](src/import.rs#L2228) | [✅](src/import.rs#L2016) |
| [Dragapult ex](src/import.rs#L2110) | [✅](src/import.rs#L2110) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1971) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2117) | [✅](src/import.rs#L2117) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1995) |
| [Dudunsparce ex](src/import.rs#L2078) | [✅](src/import.rs#L2078) | — |
| [Dunsparce](src/import.rs#L2129) | [✅](src/import.rs#L2129) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1999) |
| [Dusknoir](src/import.rs#L2210) | [✅](src/import.rs#L2210) | [✅](src/import.rs#L2000) |
| [Duskull](src/import.rs#L2132) | [✅](src/import.rs#L2132) | — |
| [Dwebble](src/import.rs#L2122) | [✅](src/import.rs#L2122) | — |
| [Elgyem](src/import.rs#L2152) | [✅](src/import.rs#L2152) | — |
| [Enamorus](src/import.rs#L2172) | [✅](src/import.rs#L2172) | — |
| [Fan Rotom](src/import.rs#L2217) | [✅](src/import.rs#L2217) | [✅](src/import.rs#L2041) |
| [Fezandipiti ex](src/import.rs#L2291) | [✅](src/import.rs#L2291) | [✅](src/import.rs#L1989) |
| [Flutter Mane](src/import.rs#L2208) | [✅](src/import.rs#L2208) | [✅](src/import.rs#L1965) |
| [Genesect](src/import.rs#L2263) | [✅](src/import.rs#L2263) | [✅](src/import.rs#L2004) |
| [Genesect ex](src/import.rs#L2211) | [✅](src/import.rs#L2211) | [✅](src/import.rs#L2001) |
| [Goldeen](src/import.rs#L2226) | [✅](src/import.rs#L2226) | [✅](src/import.rs#L2014) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2119) | [✅](src/import.rs#L2119) | [✅](src/import.rs#L1955) |
| [Hydrapple ex](src/import.rs#L2177) | [✅](src/import.rs#L2177) | [✅](src/import.rs#L1956) |
| [Iron Crown ex](src/import.rs#L2137) | [✅](src/import.rs#L2137) | [✅](src/import.rs#L1951) |
| [Iron Leaves ex](src/import.rs#L2216) | [✅](src/import.rs#L2216) | [✅](src/import.rs#L2032) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1984) |
| [Koraidon ex](src/import.rs#L2138) | [✅](src/import.rs#L2138) | — |
| [Kyurem](src/import.rs#L2260) | [✅](src/import.rs#L2260) | [✅](src/import.rs#L2023) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2277) | [✅](src/import.rs#L2277) | [✅](src/import.rs#L1942) |
| [Lillie's Clefairy ex](src/import.rs#L2301) | [✅](src/import.rs#L2301) | [✅](src/import.rs#L1946) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2184) | [✅](src/import.rs#L2184) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2180) | [✅](src/import.rs#L2180) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2287) | [✅](src/import.rs#L2287) | [✅](src/import.rs#L1939) |
| [Mega Lopunny ex](src/import.rs#L2085) | [✅](src/import.rs#L2085) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2120) | [✅](src/import.rs#L2120) | — |
| [Mega Skarmory ex](src/import.rs#L2198) | [✅](src/import.rs#L2198) | — |
| [Mega Slowbro ex](src/import.rs#L2238) | [✅](src/import.rs#L2238) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2010) |
| [Meowth ex](src/import.rs#L2290) | [✅](src/import.rs#L2290) | [✅](src/import.rs#L1983) |
| [Metagross](src/import.rs#L2094) | [✅](src/import.rs#L2094) | — |
| [Metang](src/import.rs#L2278) | [✅](src/import.rs#L2278) | [✅](src/import.rs#L1974) |
| [Moltres](src/import.rs#L2130) | [✅](src/import.rs#L2130) | — |
| [Munkidori](src/import.rs#L2284) | [✅](src/import.rs#L2284) | [✅](src/import.rs#L1977) |
| [N's Darmanitan](src/import.rs#L2075) | [✅](src/import.rs#L2075) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2072) | [✅](src/import.rs#L2072) | — |
| [N's Zekrom](src/import.rs#L2084) | [✅](src/import.rs#L2084) | — |
| [N's Zoroark ex](src/import.rs#L2257) | [✅](src/import.rs#L2257) | [✅](src/import.rs#L2007) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2280) | [✅](src/import.rs#L2280) | [✅](src/import.rs#L1985) |
| [Paldean Tauros](src/import.rs#L2068) | [✅](src/import.rs#L2068) | — |
| [Passimian](src/import.rs#L2081) | [✅](src/import.rs#L2081) | — |
| [Patrat](src/import.rs#L2279) | [✅](src/import.rs#L2279) | [✅](src/import.rs#L1944) |
| [Pecharunt](src/import.rs#L2221) | [✅](src/import.rs#L2221) | [✅](src/import.rs#L2011) |
| [Pecharunt ex](src/import.rs#L2218) | [✅](src/import.rs#L2218) | [✅](src/import.rs#L2048) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1945) |
| [Rabsca](src/import.rs#L2134) | [✅](src/import.rs#L2134) | [✅](src/import.rs#L1950) |
| [Raging Bolt ex](src/import.rs#L2099) | [✅](src/import.rs#L2099) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L2066) | [✅](src/import.rs#L2066) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2227) | [✅](src/import.rs#L2227) | [✅](src/import.rs#L2015) |
| [Shaymin](src/import.rs#L2274) | [✅](src/import.rs#L2274) | [✅](src/import.rs#L1949) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2123) | [✅](src/import.rs#L2123) | — |
| [Slowpoke](src/import.rs#L2131) | [✅](src/import.rs#L2131) | ❌ |
| [Smoochum](src/import.rs#L2191) | [✅](src/import.rs#L2191) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2173) | [✅](src/import.rs#L2173) | — |
| [Tapu Bulu](src/import.rs#L2067) | [✅](src/import.rs#L2067) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1968) |
| [Teal Mask Ogerpon ex](src/import.rs#L2292) | [✅](src/import.rs#L2292) | [✅](src/import.rs#L1992) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2017) |
| [Torchic](src/import.rs#L2133) | [✅](src/import.rs#L2133) | — |
| [Toxel](src/import.rs#L2118) | [✅](src/import.rs#L2118) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2035) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2108) | [✅](src/import.rs#L2108) | — |
| [Yveltal](src/import.rs#L2107) | [✅](src/import.rs#L2107) | — |
| [Zeraora](src/import.rs#L2090) | [✅](src/import.rs#L2090) | — |
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

