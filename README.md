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
per player, named `<placement>-<player-slug>.txt`.
`tools/fetch_worlds_decks.py` fetches them from the operator's own tournament
site. Sixty-one of the sixty-four are kept: three named their cards by
Japanese-region set codes this artifact does not hold, and were dropped rather
than guessed at, so the placement numbers have three gaps.

### Standard coverage

Every card in the artifact, by name; the tables below track the field.

| Kind | Built | Total |
| --- | --- | --- |
| Supporters | 56 | 78 |
| Items | 50 | 85 |
| Tools | 20 | 35 |
| Stadiums | 17 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1162) | ✅ |
| [Black Belt's Training](src/import.rs#L1170) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L783) | ✅ |
| [Brock's Scouting](src/import.rs#L1200) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1102) | ✅ |
| [Crispin](src/import.rs#L1008) | ✅ |
| [Cyrano](src/import.rs#L857) | ✅ |
| [Dawn](src/import.rs#L974) | ✅ |
| [Eri](src/import.rs#L1193) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1174) | ✅ |
| [Gwynn](src/import.rs#L871) | ✅ |
| [Hilda](src/import.rs#L928) | ✅ |
| [Janine's Secret Art](src/import.rs#L1224) | ✅ |
| [Judge](src/import.rs#L806) | ✅ |
| [Kieran](src/import.rs#L1178) | ✅ |
| [Lana's Aid](src/import.rs#L1128) | ✅ |
| [Lillie's Determination](src/import.rs#L807) | ✅ |
| [Morty's Conviction](src/import.rs#L1188) | ✅ |
| [N's Plan](src/import.rs#L1146) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1148) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1142) | ✅ |
| [Surfer](src/import.rs#L1166) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1032) | ✅ |
| [Wally's Compassion](src/import.rs#L1223) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1192) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L843) | ✅ |
| [Bug Catching Set](src/import.rs#L1088) | ✅ |
| [Crushing Hammer](src/import.rs#L842) | ✅ |
| [Dusk Ball](src/import.rs#L1254) | ✅ |
| [Energy Recycler](src/import.rs#L1336) | ✅ |
| [Energy Retrieval](src/import.rs#L1239) | ✅ |
| [Energy Search](src/import.rs#L1225) | ✅ |
| [Energy Switch](src/import.rs#L913) | ✅ |
| [Enhanced Hammer](src/import.rs#L787) | ✅ |
| [Glass Trumpet](src/import.rs#L788) | ✅ |
| [Hand Trimmer](src/import.rs#L1253) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1124) | ✅ |
| [N's PP Up](src/import.rs#L1046) | ✅ |
| [Night Stretcher](src/import.rs#L814) | ✅ |
| [Prime Catcher](src/import.rs#L1255) | ✅ |
| [Rare Candy](src/import.rs#L1031) | ✅ |
| [Sacred Ash](src/import.rs#L885) | ✅ |
| [Secret Box](src/import.rs#L1285) | ✅ |
| [Special Red Card](src/import.rs#L1004) | ✅ |
| [Strange Timepiece](src/import.rs#L1256) | ✅ |
| [Switch](src/import.rs#L1123) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1322) | ✅ |
| [Tera Orb](src/import.rs#L914) | ✅ |
| [Tool Scrapper](src/import.rs#L782) | ✅ |
| [Transformation Tome](src/import.rs#L1281) | ✅ |
| [Ultra Ball](src/import.rs#L899) | ✅ |
| [Unfair Stamp](src/import.rs#L1116) | ✅ |
| [Wondrous Patch](src/import.rs#L1060) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1257) | ✅ |
| [Binding Mochi](src/import.rs#L1260) | ✅ |
| [Brave Bangle](src/import.rs#L1259) | ✅ |
| [Handheld Fan](src/import.rs#L1264) | ✅ |
| [Hero's Cape](src/import.rs#L1258) | ✅ |
| [Lillie's Pearl](src/import.rs#L1261) | ✅ |
| [Lucky Helmet](src/import.rs#L1263) | ✅ |
| [Powerglass](src/import.rs#L1265) | ✅ |
| [Punk Helmet](src/import.rs#L1262) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1268) | ✅ |
| [Area Zero Underdepths](src/import.rs#L803) | ✅ |
| [Battle Cage](src/import.rs#L804) | ✅ |
| [Festival Grounds](src/import.rs#L1277) | ✅ |
| [Forest of Vitality](src/import.rs#L1276) | ✅ |
| [Gravity Mountain](src/import.rs#L1266) | ✅ |
| [Jamming Tower](src/import.rs#L1274) | ✅ |
| [Lumiose City](src/import.rs#L1273) | ✅ |
| [N's Castle](src/import.rs#L1267) | ✅ |
| [Nighttime Mine](src/import.rs#L802) | ✅ |
| [Risky Ruins](src/import.rs#L1275) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1269) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L805) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1715) | ✅ |
| [Enriching Energy](src/import.rs#L1698) | ✅ |
| [Growing Grass Energy](src/import.rs#L1697) | ✅ |
| [Mist Energy](src/import.rs#L1712) | ✅ |
| [Prism Energy](src/import.rs#L1718) | ✅ |
| [Spiky Energy](src/import.rs#L1709) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1701) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1913) | [✅](src/import.rs#L1913) | [✅](src/import.rs#L1795) |
| [Alakazam](src/import.rs#L2068) | [✅](src/import.rs#L2068) | [✅](src/import.rs#L1785) |
| [Annihilape](src/import.rs#L1942) | [✅](src/import.rs#L1942) | [✅](src/import.rs#L1751) |
| [Applin](src/import.rs#L1903) | [✅](src/import.rs#L1903) | — |
| [Bayleef](src/import.rs#L1945) | [✅](src/import.rs#L1945) | — |
| [Beldum](src/import.rs#L1925) | [✅](src/import.rs#L1925) | — |
| [Blaziken ex](src/import.rs#L2009) | [✅](src/import.rs#L2009) | [✅](src/import.rs#L1825) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1973) | [✅](src/import.rs#L1973) | [✅](src/import.rs#L1759) |
| [Brute Bonnet](src/import.rs#L1884) | [✅](src/import.rs#L1884) | — |
| [Budew](src/import.rs#L1950) | [✅](src/import.rs#L1950) | — |
| [Buneary](src/import.rs#L1944) | [✅](src/import.rs#L1944) | — |
| [Carvanha](src/import.rs#L1862) | [✅](src/import.rs#L1862) | — |
| [Celebi](src/import.rs#L1943) | [✅](src/import.rs#L1943) | — |
| [Chi-Yu](src/import.rs#L2041) | [✅](src/import.rs#L2041) | — |
| [Chien-Pao](src/import.rs#L2010) | [✅](src/import.rs#L2010) | [✅](src/import.rs#L1828) |
| [Chikorita](src/import.rs#L1946) | [✅](src/import.rs#L1946) | — |
| [Cofagrigus](src/import.rs#L1991) | [✅](src/import.rs#L1991) | — |
| [Combusken](src/import.rs#L1960) | [✅](src/import.rs#L1960) | — |
| [Crustle](src/import.rs#L2080) | [✅](src/import.rs#L2080) | [✅](src/import.rs#L1740) |
| [Dedenne](src/import.rs#L1900) | [✅](src/import.rs#L1900) | — |
| [Dipplin](src/import.rs#L2025) | [✅](src/import.rs#L2025) | [✅](src/import.rs#L1813) |
| [Dragapult ex](src/import.rs#L1907) | [✅](src/import.rs#L1907) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1768) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1914) | [✅](src/import.rs#L1914) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1792) |
| [Dudunsparce ex](src/import.rs#L1875) | [✅](src/import.rs#L1875) | — |
| [Dunsparce](src/import.rs#L1926) | [✅](src/import.rs#L1926) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1796) |
| [Dusknoir](src/import.rs#L2007) | [✅](src/import.rs#L2007) | [✅](src/import.rs#L1797) |
| [Duskull](src/import.rs#L1929) | [✅](src/import.rs#L1929) | — |
| [Dwebble](src/import.rs#L1919) | [✅](src/import.rs#L1919) | — |
| [Elgyem](src/import.rs#L1949) | [✅](src/import.rs#L1949) | — |
| [Enamorus](src/import.rs#L1969) | [✅](src/import.rs#L1969) | — |
| [Fan Rotom](src/import.rs#L2014) | [✅](src/import.rs#L2014) | [✅](src/import.rs#L1838) |
| [Fezandipiti ex](src/import.rs#L2088) | [✅](src/import.rs#L2088) | [✅](src/import.rs#L1786) |
| [Flutter Mane](src/import.rs#L2005) | [✅](src/import.rs#L2005) | [✅](src/import.rs#L1762) |
| [Genesect](src/import.rs#L2060) | [✅](src/import.rs#L2060) | [✅](src/import.rs#L1801) |
| [Genesect ex](src/import.rs#L2008) | [✅](src/import.rs#L2008) | [✅](src/import.rs#L1798) |
| [Goldeen](src/import.rs#L2023) | [✅](src/import.rs#L2023) | [✅](src/import.rs#L1811) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1916) | [✅](src/import.rs#L1916) | [✅](src/import.rs#L1752) |
| [Hydrapple ex](src/import.rs#L1974) | [✅](src/import.rs#L1974) | [✅](src/import.rs#L1753) |
| [Iron Crown ex](src/import.rs#L1934) | [✅](src/import.rs#L1934) | [✅](src/import.rs#L1748) |
| [Iron Leaves ex](src/import.rs#L2013) | [✅](src/import.rs#L2013) | [✅](src/import.rs#L1829) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1781) |
| [Koraidon ex](src/import.rs#L1935) | [✅](src/import.rs#L1935) | — |
| [Kyurem](src/import.rs#L2057) | [✅](src/import.rs#L2057) | [✅](src/import.rs#L1820) |
| [Latias ex](src/import.rs#L2074) | [✅](src/import.rs#L2074) | [✅](src/import.rs#L1739) |
| [Lillie's Clefairy ex](src/import.rs#L2098) | [✅](src/import.rs#L2098) | [✅](src/import.rs#L1743) |
| [Mega Absol ex](src/import.rs#L1981) | [✅](src/import.rs#L1981) | — |
| [Mega Excadrill ex](src/import.rs#L1977) | [✅](src/import.rs#L1977) | — |
| [Mega Kangaskhan ex](src/import.rs#L2084) | [✅](src/import.rs#L2084) | [✅](src/import.rs#L1736) |
| [Mega Lopunny ex](src/import.rs#L1882) | [✅](src/import.rs#L1882) | — |
| [Mega Sharpedo ex](src/import.rs#L1917) | [✅](src/import.rs#L1917) | — |
| [Mega Skarmory ex](src/import.rs#L1995) | [✅](src/import.rs#L1995) | — |
| [Mega Slowbro ex](src/import.rs#L2035) | [✅](src/import.rs#L2035) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1807) |
| [Meowth ex](src/import.rs#L2087) | [✅](src/import.rs#L2087) | [✅](src/import.rs#L1780) |
| [Metagross](src/import.rs#L1891) | [✅](src/import.rs#L1891) | — |
| [Metang](src/import.rs#L2075) | [✅](src/import.rs#L2075) | [✅](src/import.rs#L1771) |
| [Moltres](src/import.rs#L1927) | [✅](src/import.rs#L1927) | — |
| [Munkidori](src/import.rs#L2081) | [✅](src/import.rs#L2081) | [✅](src/import.rs#L1774) |
| [N's Darmanitan](src/import.rs#L1872) | [✅](src/import.rs#L1872) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1869) | [✅](src/import.rs#L1869) | — |
| [N's Zekrom](src/import.rs#L1881) | [✅](src/import.rs#L1881) | — |
| [N's Zoroark ex](src/import.rs#L2054) | [✅](src/import.rs#L2054) | [✅](src/import.rs#L1804) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2077) | [✅](src/import.rs#L2077) | [✅](src/import.rs#L1782) |
| [Paldean Tauros](src/import.rs#L1865) | [✅](src/import.rs#L1865) | — |
| [Passimian](src/import.rs#L1878) | [✅](src/import.rs#L1878) | — |
| [Patrat](src/import.rs#L2076) | [✅](src/import.rs#L2076) | [✅](src/import.rs#L1741) |
| [Pecharunt](src/import.rs#L2018) | [✅](src/import.rs#L2018) | [✅](src/import.rs#L1808) |
| [Pecharunt ex](src/import.rs#L2015) | [✅](src/import.rs#L2015) | [✅](src/import.rs#L1845) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1742) |
| [Rabsca](src/import.rs#L1931) | [✅](src/import.rs#L1931) | [✅](src/import.rs#L1747) |
| [Raging Bolt ex](src/import.rs#L1896) | [✅](src/import.rs#L1896) | — |
| [Rellor](src/import.rs#L1863) | [✅](src/import.rs#L1863) | — |
| [Seaking](src/import.rs#L2024) | [✅](src/import.rs#L2024) | [✅](src/import.rs#L1812) |
| [Shaymin](src/import.rs#L2071) | [✅](src/import.rs#L2071) | [✅](src/import.rs#L1746) |
| [Slowking](src/import.rs#L1920) | [✅](src/import.rs#L1920) | — |
| [Slowpoke](src/import.rs#L1928) | [✅](src/import.rs#L1928) | ❌ |
| [Smoochum](src/import.rs#L1988) | [✅](src/import.rs#L1988) | — |
| [Stunfisk](src/import.rs#L1970) | [✅](src/import.rs#L1970) | — |
| [Tapu Bulu](src/import.rs#L1864) | [✅](src/import.rs#L1864) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1765) |
| [Teal Mask Ogerpon ex](src/import.rs#L2089) | [✅](src/import.rs#L2089) | [✅](src/import.rs#L1789) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1814) |
| [Torchic](src/import.rs#L1930) | [✅](src/import.rs#L1930) | — |
| [Toxel](src/import.rs#L1915) | [✅](src/import.rs#L1915) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1832) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1905) | [✅](src/import.rs#L1905) | — |
| [Yveltal](src/import.rs#L1904) | [✅](src/import.rs#L1904) | — |
| [Zeraora](src/import.rs#L1887) | [✅](src/import.rs#L1887) | — |

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

