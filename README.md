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
| Items | 54 | 85 |
| Tools | 20 | 35 |
| Stadiums | 17 | 31 |
| Special Energy | 13 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1218) | ✅ |
| [Black Belt's Training](src/import.rs#L1226) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L783) | ✅ |
| [Brock's Scouting](src/import.rs#L1256) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1158) | ✅ |
| [Crispin](src/import.rs#L1064) | ✅ |
| [Cyrano](src/import.rs#L913) | ✅ |
| [Dawn](src/import.rs#L1030) | ✅ |
| [Eri](src/import.rs#L1249) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1230) | ✅ |
| [Gwynn](src/import.rs#L927) | ✅ |
| [Hilda](src/import.rs#L984) | ✅ |
| [Janine's Secret Art](src/import.rs#L1280) | ✅ |
| [Judge](src/import.rs#L806) | ✅ |
| [Kieran](src/import.rs#L1234) | ✅ |
| [Lana's Aid](src/import.rs#L1184) | ✅ |
| [Lillie's Determination](src/import.rs#L807) | ✅ |
| [Morty's Conviction](src/import.rs#L1244) | ✅ |
| [N's Plan](src/import.rs#L1202) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1204) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1198) | ✅ |
| [Surfer](src/import.rs#L1222) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1088) | ✅ |
| [Wally's Compassion](src/import.rs#L1279) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1248) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L899) | ✅ |
| [Bug Catching Set](src/import.rs#L1144) | ✅ |
| [Crushing Hammer](src/import.rs#L842) | ✅ |
| [Dusk Ball](src/import.rs#L1310) | ✅ |
| [Energy Recycler](src/import.rs#L1392) | ✅ |
| [Energy Retrieval](src/import.rs#L1295) | ✅ |
| [Energy Search](src/import.rs#L1281) | ✅ |
| [Energy Switch](src/import.rs#L969) | ✅ |
| [Enhanced Hammer](src/import.rs#L787) | ✅ |
| [Glass Trumpet](src/import.rs#L788) | ✅ |
| [Hand Trimmer](src/import.rs#L1309) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1180) | ✅ |
| [N's PP Up](src/import.rs#L1102) | ✅ |
| [Night Stretcher](src/import.rs#L814) | ✅ |
| [Prime Catcher](src/import.rs#L1311) | ✅ |
| [Rare Candy](src/import.rs#L1087) | ✅ |
| [Sacred Ash](src/import.rs#L941) | ✅ |
| [Secret Box](src/import.rs#L1341) | ✅ |
| [Special Red Card](src/import.rs#L1060) | ✅ |
| [Strange Timepiece](src/import.rs#L1312) | ✅ |
| [Switch](src/import.rs#L1179) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1378) | ✅ |
| [Tera Orb](src/import.rs#L970) | ✅ |
| [Tool Scrapper](src/import.rs#L782) | ✅ |
| [Transformation Tome](src/import.rs#L1337) | ✅ |
| [Ultra Ball](src/import.rs#L955) | ✅ |
| [Unfair Stamp](src/import.rs#L1172) | ✅ |
| [Wondrous Patch](src/import.rs#L1116) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1313) | ✅ |
| [Binding Mochi](src/import.rs#L1316) | ✅ |
| [Brave Bangle](src/import.rs#L1315) | ✅ |
| [Handheld Fan](src/import.rs#L1320) | ✅ |
| [Hero's Cape](src/import.rs#L1314) | ✅ |
| [Lillie's Pearl](src/import.rs#L1317) | ✅ |
| [Lucky Helmet](src/import.rs#L1319) | ✅ |
| [Powerglass](src/import.rs#L1321) | ✅ |
| [Punk Helmet](src/import.rs#L1318) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1324) | ✅ |
| [Area Zero Underdepths](src/import.rs#L803) | ✅ |
| [Battle Cage](src/import.rs#L804) | ✅ |
| [Festival Grounds](src/import.rs#L1333) | ✅ |
| [Forest of Vitality](src/import.rs#L1332) | ✅ |
| [Gravity Mountain](src/import.rs#L1322) | ✅ |
| [Jamming Tower](src/import.rs#L1330) | ✅ |
| [Lumiose City](src/import.rs#L1329) | ✅ |
| [N's Castle](src/import.rs#L1323) | ✅ |
| [Nighttime Mine](src/import.rs#L802) | ✅ |
| [Risky Ruins](src/import.rs#L1331) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1325) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L805) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1771) | ✅ |
| [Enriching Energy](src/import.rs#L1754) | ✅ |
| [Growing Grass Energy](src/import.rs#L1753) | ✅ |
| [Mist Energy](src/import.rs#L1768) | ✅ |
| [Prism Energy](src/import.rs#L1774) | ✅ |
| [Spiky Energy](src/import.rs#L1765) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1757) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1978) | [✅](src/import.rs#L1978) | [✅](src/import.rs#L1860) |
| [Alakazam](src/import.rs#L2133) | [✅](src/import.rs#L2133) | [✅](src/import.rs#L1850) |
| [Annihilape](src/import.rs#L2007) | [✅](src/import.rs#L2007) | [✅](src/import.rs#L1816) |
| [Applin](src/import.rs#L1968) | [✅](src/import.rs#L1968) | — |
| [Bayleef](src/import.rs#L2010) | [✅](src/import.rs#L2010) | — |
| [Beldum](src/import.rs#L1990) | [✅](src/import.rs#L1990) | — |
| [Blaziken ex](src/import.rs#L2074) | [✅](src/import.rs#L2074) | [✅](src/import.rs#L1890) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2038) | [✅](src/import.rs#L2038) | [✅](src/import.rs#L1824) |
| [Brute Bonnet](src/import.rs#L1949) | [✅](src/import.rs#L1949) | — |
| [Budew](src/import.rs#L2015) | [✅](src/import.rs#L2015) | — |
| [Buneary](src/import.rs#L2009) | [✅](src/import.rs#L2009) | — |
| [Carvanha](src/import.rs#L1927) | [✅](src/import.rs#L1927) | — |
| [Celebi](src/import.rs#L2008) | [✅](src/import.rs#L2008) | — |
| [Chi-Yu](src/import.rs#L2106) | [✅](src/import.rs#L2106) | — |
| [Chien-Pao](src/import.rs#L2075) | [✅](src/import.rs#L2075) | [✅](src/import.rs#L1893) |
| [Chikorita](src/import.rs#L2011) | [✅](src/import.rs#L2011) | — |
| [Cofagrigus](src/import.rs#L2056) | [✅](src/import.rs#L2056) | — |
| [Combusken](src/import.rs#L2025) | [✅](src/import.rs#L2025) | — |
| [Crustle](src/import.rs#L2145) | [✅](src/import.rs#L2145) | [✅](src/import.rs#L1805) |
| [Dedenne](src/import.rs#L1965) | [✅](src/import.rs#L1965) | — |
| [Dipplin](src/import.rs#L2090) | [✅](src/import.rs#L2090) | [✅](src/import.rs#L1878) |
| [Dragapult ex](src/import.rs#L1972) | [✅](src/import.rs#L1972) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1833) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1979) | [✅](src/import.rs#L1979) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1857) |
| [Dudunsparce ex](src/import.rs#L1940) | [✅](src/import.rs#L1940) | — |
| [Dunsparce](src/import.rs#L1991) | [✅](src/import.rs#L1991) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1861) |
| [Dusknoir](src/import.rs#L2072) | [✅](src/import.rs#L2072) | [✅](src/import.rs#L1862) |
| [Duskull](src/import.rs#L1994) | [✅](src/import.rs#L1994) | — |
| [Dwebble](src/import.rs#L1984) | [✅](src/import.rs#L1984) | — |
| [Elgyem](src/import.rs#L2014) | [✅](src/import.rs#L2014) | — |
| [Enamorus](src/import.rs#L2034) | [✅](src/import.rs#L2034) | — |
| [Fan Rotom](src/import.rs#L2079) | [✅](src/import.rs#L2079) | [✅](src/import.rs#L1903) |
| [Fezandipiti ex](src/import.rs#L2153) | [✅](src/import.rs#L2153) | [✅](src/import.rs#L1851) |
| [Flutter Mane](src/import.rs#L2070) | [✅](src/import.rs#L2070) | [✅](src/import.rs#L1827) |
| [Genesect](src/import.rs#L2125) | [✅](src/import.rs#L2125) | [✅](src/import.rs#L1866) |
| [Genesect ex](src/import.rs#L2073) | [✅](src/import.rs#L2073) | [✅](src/import.rs#L1863) |
| [Goldeen](src/import.rs#L2088) | [✅](src/import.rs#L2088) | [✅](src/import.rs#L1876) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1981) | [✅](src/import.rs#L1981) | [✅](src/import.rs#L1817) |
| [Hydrapple ex](src/import.rs#L2039) | [✅](src/import.rs#L2039) | [✅](src/import.rs#L1818) |
| [Iron Crown ex](src/import.rs#L1999) | [✅](src/import.rs#L1999) | [✅](src/import.rs#L1813) |
| [Iron Leaves ex](src/import.rs#L2078) | [✅](src/import.rs#L2078) | [✅](src/import.rs#L1894) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1846) |
| [Koraidon ex](src/import.rs#L2000) | [✅](src/import.rs#L2000) | — |
| [Kyurem](src/import.rs#L2122) | [✅](src/import.rs#L2122) | [✅](src/import.rs#L1885) |
| [Latias ex](src/import.rs#L2139) | [✅](src/import.rs#L2139) | [✅](src/import.rs#L1804) |
| [Lillie's Clefairy ex](src/import.rs#L2163) | [✅](src/import.rs#L2163) | [✅](src/import.rs#L1808) |
| [Mega Absol ex](src/import.rs#L2046) | [✅](src/import.rs#L2046) | — |
| [Mega Excadrill ex](src/import.rs#L2042) | [✅](src/import.rs#L2042) | — |
| [Mega Kangaskhan ex](src/import.rs#L2149) | [✅](src/import.rs#L2149) | [✅](src/import.rs#L1801) |
| [Mega Lopunny ex](src/import.rs#L1947) | [✅](src/import.rs#L1947) | — |
| [Mega Sharpedo ex](src/import.rs#L1982) | [✅](src/import.rs#L1982) | — |
| [Mega Skarmory ex](src/import.rs#L2060) | [✅](src/import.rs#L2060) | — |
| [Mega Slowbro ex](src/import.rs#L2100) | [✅](src/import.rs#L2100) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1872) |
| [Meowth ex](src/import.rs#L2152) | [✅](src/import.rs#L2152) | [✅](src/import.rs#L1845) |
| [Metagross](src/import.rs#L1956) | [✅](src/import.rs#L1956) | — |
| [Metang](src/import.rs#L2140) | [✅](src/import.rs#L2140) | [✅](src/import.rs#L1836) |
| [Moltres](src/import.rs#L1992) | [✅](src/import.rs#L1992) | — |
| [Munkidori](src/import.rs#L2146) | [✅](src/import.rs#L2146) | [✅](src/import.rs#L1839) |
| [N's Darmanitan](src/import.rs#L1937) | [✅](src/import.rs#L1937) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1934) | [✅](src/import.rs#L1934) | — |
| [N's Zekrom](src/import.rs#L1946) | [✅](src/import.rs#L1946) | — |
| [N's Zoroark ex](src/import.rs#L2119) | [✅](src/import.rs#L2119) | [✅](src/import.rs#L1869) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2142) | [✅](src/import.rs#L2142) | [✅](src/import.rs#L1847) |
| [Paldean Tauros](src/import.rs#L1930) | [✅](src/import.rs#L1930) | — |
| [Passimian](src/import.rs#L1943) | [✅](src/import.rs#L1943) | — |
| [Patrat](src/import.rs#L2141) | [✅](src/import.rs#L2141) | [✅](src/import.rs#L1806) |
| [Pecharunt](src/import.rs#L2083) | [✅](src/import.rs#L2083) | [✅](src/import.rs#L1873) |
| [Pecharunt ex](src/import.rs#L2080) | [✅](src/import.rs#L2080) | [✅](src/import.rs#L1910) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1807) |
| [Rabsca](src/import.rs#L1996) | [✅](src/import.rs#L1996) | [✅](src/import.rs#L1812) |
| [Raging Bolt ex](src/import.rs#L1961) | [✅](src/import.rs#L1961) | — |
| [Rellor](src/import.rs#L1928) | [✅](src/import.rs#L1928) | — |
| [Seaking](src/import.rs#L2089) | [✅](src/import.rs#L2089) | [✅](src/import.rs#L1877) |
| [Shaymin](src/import.rs#L2136) | [✅](src/import.rs#L2136) | [✅](src/import.rs#L1811) |
| [Slowking](src/import.rs#L1985) | [✅](src/import.rs#L1985) | — |
| [Slowpoke](src/import.rs#L1993) | [✅](src/import.rs#L1993) | ❌ |
| [Smoochum](src/import.rs#L2053) | [✅](src/import.rs#L2053) | — |
| [Stunfisk](src/import.rs#L2035) | [✅](src/import.rs#L2035) | — |
| [Tapu Bulu](src/import.rs#L1929) | [✅](src/import.rs#L1929) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1830) |
| [Teal Mask Ogerpon ex](src/import.rs#L2154) | [✅](src/import.rs#L2154) | [✅](src/import.rs#L1854) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1879) |
| [Torchic](src/import.rs#L1995) | [✅](src/import.rs#L1995) | — |
| [Toxel](src/import.rs#L1980) | [✅](src/import.rs#L1980) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1897) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1970) | [✅](src/import.rs#L1970) | — |
| [Yveltal](src/import.rs#L1969) | [✅](src/import.rs#L1969) | — |
| [Zeraora](src/import.rs#L1952) | [✅](src/import.rs#L1952) | — |

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

