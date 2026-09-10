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
| Items | 48 | 85 |
| Tools | 20 | 35 |
| Stadiums | 17 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1157) | ✅ |
| [Black Belt's Training](src/import.rs#L1165) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L778) | ✅ |
| [Brock's Scouting](src/import.rs#L1195) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1097) | ✅ |
| [Crispin](src/import.rs#L1003) | ✅ |
| [Cyrano](src/import.rs#L852) | ✅ |
| [Dawn](src/import.rs#L969) | ✅ |
| [Eri](src/import.rs#L1188) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1169) | ✅ |
| [Gwynn](src/import.rs#L866) | ✅ |
| [Hilda](src/import.rs#L923) | ✅ |
| [Janine's Secret Art](src/import.rs#L1219) | ✅ |
| [Judge](src/import.rs#L801) | ✅ |
| [Kieran](src/import.rs#L1173) | ✅ |
| [Lana's Aid](src/import.rs#L1123) | ✅ |
| [Lillie's Determination](src/import.rs#L802) | ✅ |
| [Morty's Conviction](src/import.rs#L1183) | ✅ |
| [N's Plan](src/import.rs#L1141) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1143) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1137) | ✅ |
| [Surfer](src/import.rs#L1161) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1027) | ✅ |
| [Wally's Compassion](src/import.rs#L1218) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1187) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L838) | ✅ |
| [Bug Catching Set](src/import.rs#L1083) | ✅ |
| [Crushing Hammer](src/import.rs#L837) | ✅ |
| [Dusk Ball](src/import.rs#L1249) | ✅ |
| [Energy Recycler](src/import.rs#L1331) | ✅ |
| [Energy Retrieval](src/import.rs#L1234) | ✅ |
| [Energy Search](src/import.rs#L1220) | ✅ |
| [Energy Switch](src/import.rs#L908) | ✅ |
| [Enhanced Hammer](src/import.rs#L782) | ✅ |
| [Glass Trumpet](src/import.rs#L783) | ✅ |
| [Hand Trimmer](src/import.rs#L1248) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1119) | ✅ |
| [N's PP Up](src/import.rs#L1041) | ✅ |
| [Night Stretcher](src/import.rs#L809) | ✅ |
| [Prime Catcher](src/import.rs#L1250) | ✅ |
| [Rare Candy](src/import.rs#L1026) | ✅ |
| [Sacred Ash](src/import.rs#L880) | ✅ |
| [Secret Box](src/import.rs#L1280) | ✅ |
| [Special Red Card](src/import.rs#L999) | ✅ |
| [Strange Timepiece](src/import.rs#L1251) | ✅ |
| [Switch](src/import.rs#L1118) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1317) | ✅ |
| [Tera Orb](src/import.rs#L909) | ✅ |
| [Tool Scrapper](src/import.rs#L777) | ✅ |
| [Transformation Tome](src/import.rs#L1276) | ✅ |
| [Ultra Ball](src/import.rs#L894) | ✅ |
| [Unfair Stamp](src/import.rs#L1111) | ✅ |
| [Wondrous Patch](src/import.rs#L1055) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1252) | ✅ |
| [Binding Mochi](src/import.rs#L1255) | ✅ |
| [Brave Bangle](src/import.rs#L1254) | ✅ |
| [Handheld Fan](src/import.rs#L1259) | ✅ |
| [Hero's Cape](src/import.rs#L1253) | ✅ |
| [Lillie's Pearl](src/import.rs#L1256) | ✅ |
| [Lucky Helmet](src/import.rs#L1258) | ✅ |
| [Powerglass](src/import.rs#L1260) | ✅ |
| [Punk Helmet](src/import.rs#L1257) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1263) | ✅ |
| [Area Zero Underdepths](src/import.rs#L798) | ✅ |
| [Battle Cage](src/import.rs#L799) | ✅ |
| [Festival Grounds](src/import.rs#L1272) | ✅ |
| [Forest of Vitality](src/import.rs#L1271) | ✅ |
| [Gravity Mountain](src/import.rs#L1261) | ✅ |
| [Jamming Tower](src/import.rs#L1269) | ✅ |
| [Lumiose City](src/import.rs#L1268) | ✅ |
| [N's Castle](src/import.rs#L1262) | ✅ |
| [Nighttime Mine](src/import.rs#L797) | ✅ |
| [Risky Ruins](src/import.rs#L1270) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1264) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L800) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1710) | ✅ |
| [Enriching Energy](src/import.rs#L1693) | ✅ |
| [Growing Grass Energy](src/import.rs#L1692) | ✅ |
| [Mist Energy](src/import.rs#L1707) | ✅ |
| [Prism Energy](src/import.rs#L1713) | ✅ |
| [Spiky Energy](src/import.rs#L1704) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1696) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1908) | [✅](src/import.rs#L1908) | [✅](src/import.rs#L1790) |
| [Alakazam](src/import.rs#L2063) | [✅](src/import.rs#L2063) | [✅](src/import.rs#L1780) |
| [Annihilape](src/import.rs#L1937) | [✅](src/import.rs#L1937) | [✅](src/import.rs#L1746) |
| [Applin](src/import.rs#L1898) | [✅](src/import.rs#L1898) | — |
| [Bayleef](src/import.rs#L1940) | [✅](src/import.rs#L1940) | — |
| [Beldum](src/import.rs#L1920) | [✅](src/import.rs#L1920) | — |
| [Blaziken ex](src/import.rs#L2004) | [✅](src/import.rs#L2004) | [✅](src/import.rs#L1820) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1968) | [✅](src/import.rs#L1968) | [✅](src/import.rs#L1754) |
| [Brute Bonnet](src/import.rs#L1879) | [✅](src/import.rs#L1879) | — |
| [Budew](src/import.rs#L1945) | [✅](src/import.rs#L1945) | — |
| [Buneary](src/import.rs#L1939) | [✅](src/import.rs#L1939) | — |
| [Carvanha](src/import.rs#L1857) | [✅](src/import.rs#L1857) | — |
| [Celebi](src/import.rs#L1938) | [✅](src/import.rs#L1938) | — |
| [Chi-Yu](src/import.rs#L2036) | [✅](src/import.rs#L2036) | — |
| [Chien-Pao](src/import.rs#L2005) | [✅](src/import.rs#L2005) | [✅](src/import.rs#L1823) |
| [Chikorita](src/import.rs#L1941) | [✅](src/import.rs#L1941) | — |
| [Cofagrigus](src/import.rs#L1986) | [✅](src/import.rs#L1986) | — |
| [Combusken](src/import.rs#L1955) | [✅](src/import.rs#L1955) | — |
| [Crustle](src/import.rs#L2075) | [✅](src/import.rs#L2075) | [✅](src/import.rs#L1735) |
| [Dedenne](src/import.rs#L1895) | [✅](src/import.rs#L1895) | — |
| [Dipplin](src/import.rs#L2020) | [✅](src/import.rs#L2020) | [✅](src/import.rs#L1808) |
| [Dragapult ex](src/import.rs#L1902) | [✅](src/import.rs#L1902) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1763) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1909) | [✅](src/import.rs#L1909) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1787) |
| [Dudunsparce ex](src/import.rs#L1870) | [✅](src/import.rs#L1870) | — |
| [Dunsparce](src/import.rs#L1921) | [✅](src/import.rs#L1921) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1791) |
| [Dusknoir](src/import.rs#L2002) | [✅](src/import.rs#L2002) | [✅](src/import.rs#L1792) |
| [Duskull](src/import.rs#L1924) | [✅](src/import.rs#L1924) | — |
| [Dwebble](src/import.rs#L1914) | [✅](src/import.rs#L1914) | — |
| [Elgyem](src/import.rs#L1944) | [✅](src/import.rs#L1944) | — |
| [Enamorus](src/import.rs#L1964) | [✅](src/import.rs#L1964) | — |
| [Fan Rotom](src/import.rs#L2009) | [✅](src/import.rs#L2009) | [✅](src/import.rs#L1833) |
| [Fezandipiti ex](src/import.rs#L2083) | [✅](src/import.rs#L2083) | [✅](src/import.rs#L1781) |
| [Flutter Mane](src/import.rs#L2000) | [✅](src/import.rs#L2000) | [✅](src/import.rs#L1757) |
| [Genesect](src/import.rs#L2055) | [✅](src/import.rs#L2055) | [✅](src/import.rs#L1796) |
| [Genesect ex](src/import.rs#L2003) | [✅](src/import.rs#L2003) | [✅](src/import.rs#L1793) |
| [Goldeen](src/import.rs#L2018) | [✅](src/import.rs#L2018) | [✅](src/import.rs#L1806) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1911) | [✅](src/import.rs#L1911) | [✅](src/import.rs#L1747) |
| [Hydrapple ex](src/import.rs#L1969) | [✅](src/import.rs#L1969) | [✅](src/import.rs#L1748) |
| [Iron Crown ex](src/import.rs#L1929) | [✅](src/import.rs#L1929) | [✅](src/import.rs#L1743) |
| [Iron Leaves ex](src/import.rs#L2008) | [✅](src/import.rs#L2008) | [✅](src/import.rs#L1824) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1776) |
| [Koraidon ex](src/import.rs#L1930) | [✅](src/import.rs#L1930) | — |
| [Kyurem](src/import.rs#L2052) | [✅](src/import.rs#L2052) | [✅](src/import.rs#L1815) |
| [Latias ex](src/import.rs#L2069) | [✅](src/import.rs#L2069) | [✅](src/import.rs#L1734) |
| [Lillie's Clefairy ex](src/import.rs#L2093) | [✅](src/import.rs#L2093) | [✅](src/import.rs#L1738) |
| [Mega Absol ex](src/import.rs#L1976) | [✅](src/import.rs#L1976) | — |
| [Mega Excadrill ex](src/import.rs#L1972) | [✅](src/import.rs#L1972) | — |
| [Mega Kangaskhan ex](src/import.rs#L2079) | [✅](src/import.rs#L2079) | [✅](src/import.rs#L1731) |
| [Mega Lopunny ex](src/import.rs#L1877) | [✅](src/import.rs#L1877) | — |
| [Mega Sharpedo ex](src/import.rs#L1912) | [✅](src/import.rs#L1912) | — |
| [Mega Skarmory ex](src/import.rs#L1990) | [✅](src/import.rs#L1990) | — |
| [Mega Slowbro ex](src/import.rs#L2030) | [✅](src/import.rs#L2030) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1802) |
| [Meowth ex](src/import.rs#L2082) | [✅](src/import.rs#L2082) | [✅](src/import.rs#L1775) |
| [Metagross](src/import.rs#L1886) | [✅](src/import.rs#L1886) | — |
| [Metang](src/import.rs#L2070) | [✅](src/import.rs#L2070) | [✅](src/import.rs#L1766) |
| [Moltres](src/import.rs#L1922) | [✅](src/import.rs#L1922) | — |
| [Munkidori](src/import.rs#L2076) | [✅](src/import.rs#L2076) | [✅](src/import.rs#L1769) |
| [N's Darmanitan](src/import.rs#L1867) | [✅](src/import.rs#L1867) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1864) | [✅](src/import.rs#L1864) | — |
| [N's Zekrom](src/import.rs#L1876) | [✅](src/import.rs#L1876) | — |
| [N's Zoroark ex](src/import.rs#L2049) | [✅](src/import.rs#L2049) | [✅](src/import.rs#L1799) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2072) | [✅](src/import.rs#L2072) | [✅](src/import.rs#L1777) |
| [Paldean Tauros](src/import.rs#L1860) | [✅](src/import.rs#L1860) | — |
| [Passimian](src/import.rs#L1873) | [✅](src/import.rs#L1873) | — |
| [Patrat](src/import.rs#L2071) | [✅](src/import.rs#L2071) | [✅](src/import.rs#L1736) |
| [Pecharunt](src/import.rs#L2013) | [✅](src/import.rs#L2013) | [✅](src/import.rs#L1803) |
| [Pecharunt ex](src/import.rs#L2010) | [✅](src/import.rs#L2010) | [✅](src/import.rs#L1840) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1737) |
| [Rabsca](src/import.rs#L1926) | [✅](src/import.rs#L1926) | [✅](src/import.rs#L1742) |
| [Raging Bolt ex](src/import.rs#L1891) | [✅](src/import.rs#L1891) | — |
| [Rellor](src/import.rs#L1858) | [✅](src/import.rs#L1858) | — |
| [Seaking](src/import.rs#L2019) | [✅](src/import.rs#L2019) | [✅](src/import.rs#L1807) |
| [Shaymin](src/import.rs#L2066) | [✅](src/import.rs#L2066) | [✅](src/import.rs#L1741) |
| [Slowking](src/import.rs#L1915) | [✅](src/import.rs#L1915) | — |
| [Slowpoke](src/import.rs#L1923) | [✅](src/import.rs#L1923) | ❌ |
| [Smoochum](src/import.rs#L1983) | [✅](src/import.rs#L1983) | — |
| [Stunfisk](src/import.rs#L1965) | [✅](src/import.rs#L1965) | — |
| [Tapu Bulu](src/import.rs#L1859) | [✅](src/import.rs#L1859) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1760) |
| [Teal Mask Ogerpon ex](src/import.rs#L2084) | [✅](src/import.rs#L2084) | [✅](src/import.rs#L1784) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1809) |
| [Torchic](src/import.rs#L1925) | [✅](src/import.rs#L1925) | — |
| [Toxel](src/import.rs#L1910) | [✅](src/import.rs#L1910) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1827) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1900) | [✅](src/import.rs#L1900) | — |
| [Yveltal](src/import.rs#L1899) | [✅](src/import.rs#L1899) | — |
| [Zeraora](src/import.rs#L1882) | [✅](src/import.rs#L1882) | — |

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

