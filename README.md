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
| Supporters | 54 | 78 |
| Items | 44 | 85 |
| Tools | 18 | 35 |
| Stadiums | 16 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1116) | ✅ |
| [Black Belt's Training](src/import.rs#L1124) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L737) | ✅ |
| [Brock's Scouting](src/import.rs#L1154) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1056) | ✅ |
| [Crispin](src/import.rs#L962) | ✅ |
| [Cyrano](src/import.rs#L811) | ✅ |
| [Dawn](src/import.rs#L928) | ✅ |
| [Eri](src/import.rs#L1147) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1128) | ✅ |
| [Gwynn](src/import.rs#L825) | ✅ |
| [Hilda](src/import.rs#L882) | ✅ |
| [Janine's Secret Art](src/import.rs#L1178) | ✅ |
| [Judge](src/import.rs#L760) | ✅ |
| [Kieran](src/import.rs#L1132) | ✅ |
| [Lana's Aid](src/import.rs#L1082) | ✅ |
| [Lillie's Determination](src/import.rs#L761) | ✅ |
| [Morty's Conviction](src/import.rs#L1142) | ✅ |
| [N's Plan](src/import.rs#L1100) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1102) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1096) | ✅ |
| [Surfer](src/import.rs#L1120) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L986) | ✅ |
| [Wally's Compassion](src/import.rs#L1177) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1146) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L797) | ✅ |
| [Bug Catching Set](src/import.rs#L1042) | ✅ |
| [Crushing Hammer](src/import.rs#L796) | ✅ |
| [Dusk Ball](src/import.rs#L1208) | ✅ |
| [Energy Recycler](src/import.rs#L1290) | ✅ |
| [Energy Retrieval](src/import.rs#L1193) | ✅ |
| [Energy Search](src/import.rs#L1179) | ✅ |
| [Energy Switch](src/import.rs#L867) | ✅ |
| [Enhanced Hammer](src/import.rs#L741) | ✅ |
| [Glass Trumpet](src/import.rs#L742) | ✅ |
| [Hand Trimmer](src/import.rs#L1207) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1078) | ✅ |
| [N's PP Up](src/import.rs#L1000) | ✅ |
| [Night Stretcher](src/import.rs#L768) | ✅ |
| [Prime Catcher](src/import.rs#L1209) | ✅ |
| [Rare Candy](src/import.rs#L985) | ✅ |
| [Sacred Ash](src/import.rs#L839) | ✅ |
| [Secret Box](src/import.rs#L1239) | ✅ |
| [Special Red Card](src/import.rs#L958) | ✅ |
| [Strange Timepiece](src/import.rs#L1210) | ✅ |
| [Switch](src/import.rs#L1077) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1276) | ✅ |
| [Tera Orb](src/import.rs#L868) | ✅ |
| [Tool Scrapper](src/import.rs#L736) | ✅ |
| [Transformation Tome](src/import.rs#L1235) | ✅ |
| [Ultra Ball](src/import.rs#L853) | ✅ |
| [Unfair Stamp](src/import.rs#L1070) | ✅ |
| [Wondrous Patch](src/import.rs#L1014) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1211) | ✅ |
| [Binding Mochi](src/import.rs#L1214) | ✅ |
| [Brave Bangle](src/import.rs#L1213) | ✅ |
| [Handheld Fan](src/import.rs#L1218) | ✅ |
| [Hero's Cape](src/import.rs#L1212) | ✅ |
| [Lillie's Pearl](src/import.rs#L1215) | ✅ |
| [Lucky Helmet](src/import.rs#L1217) | ✅ |
| [Powerglass](src/import.rs#L1219) | ✅ |
| [Punk Helmet](src/import.rs#L1216) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1222) | ✅ |
| [Area Zero Underdepths](src/import.rs#L757) | ✅ |
| [Battle Cage](src/import.rs#L758) | ✅ |
| [Festival Grounds](src/import.rs#L1231) | ✅ |
| [Forest of Vitality](src/import.rs#L1230) | ✅ |
| [Gravity Mountain](src/import.rs#L1220) | ✅ |
| [Jamming Tower](src/import.rs#L1228) | ✅ |
| [Lumiose City](src/import.rs#L1227) | ✅ |
| [N's Castle](src/import.rs#L1221) | ✅ |
| [Nighttime Mine](src/import.rs#L756) | ✅ |
| [Risky Ruins](src/import.rs#L1229) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1223) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L759) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1669) | ✅ |
| [Enriching Energy](src/import.rs#L1652) | ✅ |
| [Growing Grass Energy](src/import.rs#L1651) | ✅ |
| [Mist Energy](src/import.rs#L1666) | ✅ |
| [Prism Energy](src/import.rs#L1672) | ✅ |
| [Spiky Energy](src/import.rs#L1663) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1655) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1867) | [✅](src/import.rs#L1867) | [✅](src/import.rs#L1749) |
| [Alakazam](src/import.rs#L2022) | [✅](src/import.rs#L2022) | [✅](src/import.rs#L1739) |
| [Annihilape](src/import.rs#L1896) | [✅](src/import.rs#L1896) | [✅](src/import.rs#L1705) |
| [Applin](src/import.rs#L1857) | [✅](src/import.rs#L1857) | — |
| [Bayleef](src/import.rs#L1899) | [✅](src/import.rs#L1899) | — |
| [Beldum](src/import.rs#L1879) | [✅](src/import.rs#L1879) | — |
| [Blaziken ex](src/import.rs#L1963) | [✅](src/import.rs#L1963) | [✅](src/import.rs#L1779) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1927) | [✅](src/import.rs#L1927) | [✅](src/import.rs#L1713) |
| [Brute Bonnet](src/import.rs#L1838) | [✅](src/import.rs#L1838) | — |
| [Budew](src/import.rs#L1904) | [✅](src/import.rs#L1904) | — |
| [Buneary](src/import.rs#L1898) | [✅](src/import.rs#L1898) | — |
| [Carvanha](src/import.rs#L1816) | [✅](src/import.rs#L1816) | — |
| [Celebi](src/import.rs#L1897) | [✅](src/import.rs#L1897) | — |
| [Chi-Yu](src/import.rs#L1995) | [✅](src/import.rs#L1995) | — |
| [Chien-Pao](src/import.rs#L1964) | [✅](src/import.rs#L1964) | [✅](src/import.rs#L1782) |
| [Chikorita](src/import.rs#L1900) | [✅](src/import.rs#L1900) | — |
| [Cofagrigus](src/import.rs#L1945) | [✅](src/import.rs#L1945) | — |
| [Combusken](src/import.rs#L1914) | [✅](src/import.rs#L1914) | — |
| [Crustle](src/import.rs#L2034) | [✅](src/import.rs#L2034) | [✅](src/import.rs#L1694) |
| [Dedenne](src/import.rs#L1854) | [✅](src/import.rs#L1854) | — |
| [Dipplin](src/import.rs#L1979) | [✅](src/import.rs#L1979) | [✅](src/import.rs#L1767) |
| [Dragapult ex](src/import.rs#L1861) | [✅](src/import.rs#L1861) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1722) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1868) | [✅](src/import.rs#L1868) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1746) |
| [Dudunsparce ex](src/import.rs#L1829) | [✅](src/import.rs#L1829) | — |
| [Dunsparce](src/import.rs#L1880) | [✅](src/import.rs#L1880) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1750) |
| [Dusknoir](src/import.rs#L1961) | [✅](src/import.rs#L1961) | [✅](src/import.rs#L1751) |
| [Duskull](src/import.rs#L1883) | [✅](src/import.rs#L1883) | — |
| [Dwebble](src/import.rs#L1873) | [✅](src/import.rs#L1873) | — |
| [Elgyem](src/import.rs#L1903) | [✅](src/import.rs#L1903) | — |
| [Enamorus](src/import.rs#L1923) | [✅](src/import.rs#L1923) | — |
| [Fan Rotom](src/import.rs#L1968) | [✅](src/import.rs#L1968) | [✅](src/import.rs#L1792) |
| [Fezandipiti ex](src/import.rs#L2042) | [✅](src/import.rs#L2042) | [✅](src/import.rs#L1740) |
| [Flutter Mane](src/import.rs#L1959) | [✅](src/import.rs#L1959) | [✅](src/import.rs#L1716) |
| [Genesect](src/import.rs#L2014) | [✅](src/import.rs#L2014) | [✅](src/import.rs#L1755) |
| [Genesect ex](src/import.rs#L1962) | [✅](src/import.rs#L1962) | [✅](src/import.rs#L1752) |
| [Goldeen](src/import.rs#L1977) | [✅](src/import.rs#L1977) | [✅](src/import.rs#L1765) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1870) | [✅](src/import.rs#L1870) | [✅](src/import.rs#L1706) |
| [Hydrapple ex](src/import.rs#L1928) | [✅](src/import.rs#L1928) | [✅](src/import.rs#L1707) |
| [Iron Crown ex](src/import.rs#L1888) | [✅](src/import.rs#L1888) | [✅](src/import.rs#L1702) |
| [Iron Leaves ex](src/import.rs#L1967) | [✅](src/import.rs#L1967) | [✅](src/import.rs#L1783) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1735) |
| [Koraidon ex](src/import.rs#L1889) | [✅](src/import.rs#L1889) | — |
| [Kyurem](src/import.rs#L2011) | [✅](src/import.rs#L2011) | [✅](src/import.rs#L1774) |
| [Latias ex](src/import.rs#L2028) | [✅](src/import.rs#L2028) | [✅](src/import.rs#L1693) |
| [Lillie's Clefairy ex](src/import.rs#L2052) | [✅](src/import.rs#L2052) | [✅](src/import.rs#L1697) |
| [Mega Absol ex](src/import.rs#L1935) | [✅](src/import.rs#L1935) | — |
| [Mega Excadrill ex](src/import.rs#L1931) | [✅](src/import.rs#L1931) | — |
| [Mega Kangaskhan ex](src/import.rs#L2038) | [✅](src/import.rs#L2038) | [✅](src/import.rs#L1690) |
| [Mega Lopunny ex](src/import.rs#L1836) | [✅](src/import.rs#L1836) | — |
| [Mega Sharpedo ex](src/import.rs#L1871) | [✅](src/import.rs#L1871) | — |
| [Mega Skarmory ex](src/import.rs#L1949) | [✅](src/import.rs#L1949) | — |
| [Mega Slowbro ex](src/import.rs#L1989) | [✅](src/import.rs#L1989) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1761) |
| [Meowth ex](src/import.rs#L2041) | [✅](src/import.rs#L2041) | [✅](src/import.rs#L1734) |
| [Metagross](src/import.rs#L1845) | [✅](src/import.rs#L1845) | — |
| [Metang](src/import.rs#L2029) | [✅](src/import.rs#L2029) | [✅](src/import.rs#L1725) |
| [Moltres](src/import.rs#L1881) | [✅](src/import.rs#L1881) | — |
| [Munkidori](src/import.rs#L2035) | [✅](src/import.rs#L2035) | [✅](src/import.rs#L1728) |
| [N's Darmanitan](src/import.rs#L1826) | [✅](src/import.rs#L1826) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1823) | [✅](src/import.rs#L1823) | — |
| [N's Zekrom](src/import.rs#L1835) | [✅](src/import.rs#L1835) | — |
| [N's Zoroark ex](src/import.rs#L2008) | [✅](src/import.rs#L2008) | [✅](src/import.rs#L1758) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2031) | [✅](src/import.rs#L2031) | [✅](src/import.rs#L1736) |
| [Paldean Tauros](src/import.rs#L1819) | [✅](src/import.rs#L1819) | — |
| [Passimian](src/import.rs#L1832) | [✅](src/import.rs#L1832) | — |
| [Patrat](src/import.rs#L2030) | [✅](src/import.rs#L2030) | [✅](src/import.rs#L1695) |
| [Pecharunt](src/import.rs#L1972) | [✅](src/import.rs#L1972) | [✅](src/import.rs#L1762) |
| [Pecharunt ex](src/import.rs#L1969) | [✅](src/import.rs#L1969) | [✅](src/import.rs#L1799) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1696) |
| [Rabsca](src/import.rs#L1885) | [✅](src/import.rs#L1885) | [✅](src/import.rs#L1701) |
| [Raging Bolt ex](src/import.rs#L1850) | [✅](src/import.rs#L1850) | — |
| [Rellor](src/import.rs#L1817) | [✅](src/import.rs#L1817) | — |
| [Seaking](src/import.rs#L1978) | [✅](src/import.rs#L1978) | [✅](src/import.rs#L1766) |
| [Shaymin](src/import.rs#L2025) | [✅](src/import.rs#L2025) | [✅](src/import.rs#L1700) |
| [Slowking](src/import.rs#L1874) | [✅](src/import.rs#L1874) | — |
| [Slowpoke](src/import.rs#L1882) | [✅](src/import.rs#L1882) | ❌ |
| [Smoochum](src/import.rs#L1942) | [✅](src/import.rs#L1942) | — |
| [Stunfisk](src/import.rs#L1924) | [✅](src/import.rs#L1924) | — |
| [Tapu Bulu](src/import.rs#L1818) | [✅](src/import.rs#L1818) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1719) |
| [Teal Mask Ogerpon ex](src/import.rs#L2043) | [✅](src/import.rs#L2043) | [✅](src/import.rs#L1743) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1768) |
| [Torchic](src/import.rs#L1884) | [✅](src/import.rs#L1884) | — |
| [Toxel](src/import.rs#L1869) | [✅](src/import.rs#L1869) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1786) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1859) | [✅](src/import.rs#L1859) | — |
| [Yveltal](src/import.rs#L1858) | [✅](src/import.rs#L1858) | — |
| [Zeraora](src/import.rs#L1841) | [✅](src/import.rs#L1841) | — |

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

