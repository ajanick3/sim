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
| Items | 52 | 85 |
| Tools | 20 | 35 |
| Stadiums | 17 | 31 |
| Special Energy | 13 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1190) | ✅ |
| [Black Belt's Training](src/import.rs#L1198) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L783) | ✅ |
| [Brock's Scouting](src/import.rs#L1228) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1130) | ✅ |
| [Crispin](src/import.rs#L1036) | ✅ |
| [Cyrano](src/import.rs#L885) | ✅ |
| [Dawn](src/import.rs#L1002) | ✅ |
| [Eri](src/import.rs#L1221) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1202) | ✅ |
| [Gwynn](src/import.rs#L899) | ✅ |
| [Hilda](src/import.rs#L956) | ✅ |
| [Janine's Secret Art](src/import.rs#L1252) | ✅ |
| [Judge](src/import.rs#L806) | ✅ |
| [Kieran](src/import.rs#L1206) | ✅ |
| [Lana's Aid](src/import.rs#L1156) | ✅ |
| [Lillie's Determination](src/import.rs#L807) | ✅ |
| [Morty's Conviction](src/import.rs#L1216) | ✅ |
| [N's Plan](src/import.rs#L1174) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1176) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1170) | ✅ |
| [Surfer](src/import.rs#L1194) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1060) | ✅ |
| [Wally's Compassion](src/import.rs#L1251) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1220) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L871) | ✅ |
| [Bug Catching Set](src/import.rs#L1116) | ✅ |
| [Crushing Hammer](src/import.rs#L842) | ✅ |
| [Dusk Ball](src/import.rs#L1282) | ✅ |
| [Energy Recycler](src/import.rs#L1364) | ✅ |
| [Energy Retrieval](src/import.rs#L1267) | ✅ |
| [Energy Search](src/import.rs#L1253) | ✅ |
| [Energy Switch](src/import.rs#L941) | ✅ |
| [Enhanced Hammer](src/import.rs#L787) | ✅ |
| [Glass Trumpet](src/import.rs#L788) | ✅ |
| [Hand Trimmer](src/import.rs#L1281) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1152) | ✅ |
| [N's PP Up](src/import.rs#L1074) | ✅ |
| [Night Stretcher](src/import.rs#L814) | ✅ |
| [Prime Catcher](src/import.rs#L1283) | ✅ |
| [Rare Candy](src/import.rs#L1059) | ✅ |
| [Sacred Ash](src/import.rs#L913) | ✅ |
| [Secret Box](src/import.rs#L1313) | ✅ |
| [Special Red Card](src/import.rs#L1032) | ✅ |
| [Strange Timepiece](src/import.rs#L1284) | ✅ |
| [Switch](src/import.rs#L1151) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1350) | ✅ |
| [Tera Orb](src/import.rs#L942) | ✅ |
| [Tool Scrapper](src/import.rs#L782) | ✅ |
| [Transformation Tome](src/import.rs#L1309) | ✅ |
| [Ultra Ball](src/import.rs#L927) | ✅ |
| [Unfair Stamp](src/import.rs#L1144) | ✅ |
| [Wondrous Patch](src/import.rs#L1088) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1285) | ✅ |
| [Binding Mochi](src/import.rs#L1288) | ✅ |
| [Brave Bangle](src/import.rs#L1287) | ✅ |
| [Handheld Fan](src/import.rs#L1292) | ✅ |
| [Hero's Cape](src/import.rs#L1286) | ✅ |
| [Lillie's Pearl](src/import.rs#L1289) | ✅ |
| [Lucky Helmet](src/import.rs#L1291) | ✅ |
| [Powerglass](src/import.rs#L1293) | ✅ |
| [Punk Helmet](src/import.rs#L1290) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1296) | ✅ |
| [Area Zero Underdepths](src/import.rs#L803) | ✅ |
| [Battle Cage](src/import.rs#L804) | ✅ |
| [Festival Grounds](src/import.rs#L1305) | ✅ |
| [Forest of Vitality](src/import.rs#L1304) | ✅ |
| [Gravity Mountain](src/import.rs#L1294) | ✅ |
| [Jamming Tower](src/import.rs#L1302) | ✅ |
| [Lumiose City](src/import.rs#L1301) | ✅ |
| [N's Castle](src/import.rs#L1295) | ✅ |
| [Nighttime Mine](src/import.rs#L802) | ✅ |
| [Risky Ruins](src/import.rs#L1303) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1297) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L805) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1743) | ✅ |
| [Enriching Energy](src/import.rs#L1726) | ✅ |
| [Growing Grass Energy](src/import.rs#L1725) | ✅ |
| [Mist Energy](src/import.rs#L1740) | ✅ |
| [Prism Energy](src/import.rs#L1746) | ✅ |
| [Spiky Energy](src/import.rs#L1737) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1729) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1950) | [✅](src/import.rs#L1950) | [✅](src/import.rs#L1832) |
| [Alakazam](src/import.rs#L2105) | [✅](src/import.rs#L2105) | [✅](src/import.rs#L1822) |
| [Annihilape](src/import.rs#L1979) | [✅](src/import.rs#L1979) | [✅](src/import.rs#L1788) |
| [Applin](src/import.rs#L1940) | [✅](src/import.rs#L1940) | — |
| [Bayleef](src/import.rs#L1982) | [✅](src/import.rs#L1982) | — |
| [Beldum](src/import.rs#L1962) | [✅](src/import.rs#L1962) | — |
| [Blaziken ex](src/import.rs#L2046) | [✅](src/import.rs#L2046) | [✅](src/import.rs#L1862) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2010) | [✅](src/import.rs#L2010) | [✅](src/import.rs#L1796) |
| [Brute Bonnet](src/import.rs#L1921) | [✅](src/import.rs#L1921) | — |
| [Budew](src/import.rs#L1987) | [✅](src/import.rs#L1987) | — |
| [Buneary](src/import.rs#L1981) | [✅](src/import.rs#L1981) | — |
| [Carvanha](src/import.rs#L1899) | [✅](src/import.rs#L1899) | — |
| [Celebi](src/import.rs#L1980) | [✅](src/import.rs#L1980) | — |
| [Chi-Yu](src/import.rs#L2078) | [✅](src/import.rs#L2078) | — |
| [Chien-Pao](src/import.rs#L2047) | [✅](src/import.rs#L2047) | [✅](src/import.rs#L1865) |
| [Chikorita](src/import.rs#L1983) | [✅](src/import.rs#L1983) | — |
| [Cofagrigus](src/import.rs#L2028) | [✅](src/import.rs#L2028) | — |
| [Combusken](src/import.rs#L1997) | [✅](src/import.rs#L1997) | — |
| [Crustle](src/import.rs#L2117) | [✅](src/import.rs#L2117) | [✅](src/import.rs#L1777) |
| [Dedenne](src/import.rs#L1937) | [✅](src/import.rs#L1937) | — |
| [Dipplin](src/import.rs#L2062) | [✅](src/import.rs#L2062) | [✅](src/import.rs#L1850) |
| [Dragapult ex](src/import.rs#L1944) | [✅](src/import.rs#L1944) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1805) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1951) | [✅](src/import.rs#L1951) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1829) |
| [Dudunsparce ex](src/import.rs#L1912) | [✅](src/import.rs#L1912) | — |
| [Dunsparce](src/import.rs#L1963) | [✅](src/import.rs#L1963) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1833) |
| [Dusknoir](src/import.rs#L2044) | [✅](src/import.rs#L2044) | [✅](src/import.rs#L1834) |
| [Duskull](src/import.rs#L1966) | [✅](src/import.rs#L1966) | — |
| [Dwebble](src/import.rs#L1956) | [✅](src/import.rs#L1956) | — |
| [Elgyem](src/import.rs#L1986) | [✅](src/import.rs#L1986) | — |
| [Enamorus](src/import.rs#L2006) | [✅](src/import.rs#L2006) | — |
| [Fan Rotom](src/import.rs#L2051) | [✅](src/import.rs#L2051) | [✅](src/import.rs#L1875) |
| [Fezandipiti ex](src/import.rs#L2125) | [✅](src/import.rs#L2125) | [✅](src/import.rs#L1823) |
| [Flutter Mane](src/import.rs#L2042) | [✅](src/import.rs#L2042) | [✅](src/import.rs#L1799) |
| [Genesect](src/import.rs#L2097) | [✅](src/import.rs#L2097) | [✅](src/import.rs#L1838) |
| [Genesect ex](src/import.rs#L2045) | [✅](src/import.rs#L2045) | [✅](src/import.rs#L1835) |
| [Goldeen](src/import.rs#L2060) | [✅](src/import.rs#L2060) | [✅](src/import.rs#L1848) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1953) | [✅](src/import.rs#L1953) | [✅](src/import.rs#L1789) |
| [Hydrapple ex](src/import.rs#L2011) | [✅](src/import.rs#L2011) | [✅](src/import.rs#L1790) |
| [Iron Crown ex](src/import.rs#L1971) | [✅](src/import.rs#L1971) | [✅](src/import.rs#L1785) |
| [Iron Leaves ex](src/import.rs#L2050) | [✅](src/import.rs#L2050) | [✅](src/import.rs#L1866) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1818) |
| [Koraidon ex](src/import.rs#L1972) | [✅](src/import.rs#L1972) | — |
| [Kyurem](src/import.rs#L2094) | [✅](src/import.rs#L2094) | [✅](src/import.rs#L1857) |
| [Latias ex](src/import.rs#L2111) | [✅](src/import.rs#L2111) | [✅](src/import.rs#L1776) |
| [Lillie's Clefairy ex](src/import.rs#L2135) | [✅](src/import.rs#L2135) | [✅](src/import.rs#L1780) |
| [Mega Absol ex](src/import.rs#L2018) | [✅](src/import.rs#L2018) | — |
| [Mega Excadrill ex](src/import.rs#L2014) | [✅](src/import.rs#L2014) | — |
| [Mega Kangaskhan ex](src/import.rs#L2121) | [✅](src/import.rs#L2121) | [✅](src/import.rs#L1773) |
| [Mega Lopunny ex](src/import.rs#L1919) | [✅](src/import.rs#L1919) | — |
| [Mega Sharpedo ex](src/import.rs#L1954) | [✅](src/import.rs#L1954) | — |
| [Mega Skarmory ex](src/import.rs#L2032) | [✅](src/import.rs#L2032) | — |
| [Mega Slowbro ex](src/import.rs#L2072) | [✅](src/import.rs#L2072) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1844) |
| [Meowth ex](src/import.rs#L2124) | [✅](src/import.rs#L2124) | [✅](src/import.rs#L1817) |
| [Metagross](src/import.rs#L1928) | [✅](src/import.rs#L1928) | — |
| [Metang](src/import.rs#L2112) | [✅](src/import.rs#L2112) | [✅](src/import.rs#L1808) |
| [Moltres](src/import.rs#L1964) | [✅](src/import.rs#L1964) | — |
| [Munkidori](src/import.rs#L2118) | [✅](src/import.rs#L2118) | [✅](src/import.rs#L1811) |
| [N's Darmanitan](src/import.rs#L1909) | [✅](src/import.rs#L1909) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1906) | [✅](src/import.rs#L1906) | — |
| [N's Zekrom](src/import.rs#L1918) | [✅](src/import.rs#L1918) | — |
| [N's Zoroark ex](src/import.rs#L2091) | [✅](src/import.rs#L2091) | [✅](src/import.rs#L1841) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2114) | [✅](src/import.rs#L2114) | [✅](src/import.rs#L1819) |
| [Paldean Tauros](src/import.rs#L1902) | [✅](src/import.rs#L1902) | — |
| [Passimian](src/import.rs#L1915) | [✅](src/import.rs#L1915) | — |
| [Patrat](src/import.rs#L2113) | [✅](src/import.rs#L2113) | [✅](src/import.rs#L1778) |
| [Pecharunt](src/import.rs#L2055) | [✅](src/import.rs#L2055) | [✅](src/import.rs#L1845) |
| [Pecharunt ex](src/import.rs#L2052) | [✅](src/import.rs#L2052) | [✅](src/import.rs#L1882) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1779) |
| [Rabsca](src/import.rs#L1968) | [✅](src/import.rs#L1968) | [✅](src/import.rs#L1784) |
| [Raging Bolt ex](src/import.rs#L1933) | [✅](src/import.rs#L1933) | — |
| [Rellor](src/import.rs#L1900) | [✅](src/import.rs#L1900) | — |
| [Seaking](src/import.rs#L2061) | [✅](src/import.rs#L2061) | [✅](src/import.rs#L1849) |
| [Shaymin](src/import.rs#L2108) | [✅](src/import.rs#L2108) | [✅](src/import.rs#L1783) |
| [Slowking](src/import.rs#L1957) | [✅](src/import.rs#L1957) | — |
| [Slowpoke](src/import.rs#L1965) | [✅](src/import.rs#L1965) | ❌ |
| [Smoochum](src/import.rs#L2025) | [✅](src/import.rs#L2025) | — |
| [Stunfisk](src/import.rs#L2007) | [✅](src/import.rs#L2007) | — |
| [Tapu Bulu](src/import.rs#L1901) | [✅](src/import.rs#L1901) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1802) |
| [Teal Mask Ogerpon ex](src/import.rs#L2126) | [✅](src/import.rs#L2126) | [✅](src/import.rs#L1826) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1851) |
| [Torchic](src/import.rs#L1967) | [✅](src/import.rs#L1967) | — |
| [Toxel](src/import.rs#L1952) | [✅](src/import.rs#L1952) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1869) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1942) | [✅](src/import.rs#L1942) | — |
| [Yveltal](src/import.rs#L1941) | [✅](src/import.rs#L1941) | — |
| [Zeraora](src/import.rs#L1924) | [✅](src/import.rs#L1924) | — |

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

