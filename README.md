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
| Supporters | 53 | 78 |
| Items | 43 | 85 |
| Tools | 15 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1073) | ✅ |
| [Black Belt's Training](src/import.rs#L1081) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L694) | ✅ |
| [Brock's Scouting](src/import.rs#L1111) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1013) | ✅ |
| [Crispin](src/import.rs#L919) | ✅ |
| [Cyrano](src/import.rs#L768) | ✅ |
| [Dawn](src/import.rs#L885) | ✅ |
| [Eri](src/import.rs#L1104) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1085) | ✅ |
| [Gwynn](src/import.rs#L782) | ✅ |
| [Hilda](src/import.rs#L839) | ✅ |
| [Janine's Secret Art](src/import.rs#L1135) | ✅ |
| [Judge](src/import.rs#L717) | ✅ |
| [Kieran](src/import.rs#L1089) | ✅ |
| [Lana's Aid](src/import.rs#L1039) | ✅ |
| [Lillie's Determination](src/import.rs#L718) | ✅ |
| [Morty's Conviction](src/import.rs#L1099) | ✅ |
| [N's Plan](src/import.rs#L1057) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1059) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1053) | ✅ |
| [Surfer](src/import.rs#L1077) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L943) | ✅ |
| [Wally's Compassion](src/import.rs#L1134) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1103) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L754) | ✅ |
| [Bug Catching Set](src/import.rs#L999) | ✅ |
| [Crushing Hammer](src/import.rs#L753) | ✅ |
| [Dusk Ball](src/import.rs#L1165) | ✅ |
| [Energy Recycler](src/import.rs#L1247) | ✅ |
| [Energy Retrieval](src/import.rs#L1150) | ✅ |
| [Energy Search](src/import.rs#L1136) | ✅ |
| [Energy Switch](src/import.rs#L824) | ✅ |
| [Enhanced Hammer](src/import.rs#L698) | ✅ |
| [Glass Trumpet](src/import.rs#L699) | ✅ |
| [Hand Trimmer](src/import.rs#L1164) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1035) | ✅ |
| [N's PP Up](src/import.rs#L957) | ✅ |
| [Night Stretcher](src/import.rs#L725) | ✅ |
| [Prime Catcher](src/import.rs#L1166) | ✅ |
| [Rare Candy](src/import.rs#L942) | ✅ |
| [Sacred Ash](src/import.rs#L796) | ✅ |
| [Secret Box](src/import.rs#L1196) | ✅ |
| [Special Red Card](src/import.rs#L915) | ✅ |
| [Strange Timepiece](src/import.rs#L1167) | ✅ |
| [Switch](src/import.rs#L1034) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1233) | ✅ |
| [Tera Orb](src/import.rs#L825) | ✅ |
| [Tool Scrapper](src/import.rs#L693) | ✅ |
| [Transformation Tome](src/import.rs#L1192) | ✅ |
| [Ultra Ball](src/import.rs#L810) | ✅ |
| [Unfair Stamp](src/import.rs#L1027) | ✅ |
| [Wondrous Patch](src/import.rs#L971) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1168) | ✅ |
| [Binding Mochi](src/import.rs#L1171) | ✅ |
| [Brave Bangle](src/import.rs#L1170) | ✅ |
| [Handheld Fan](src/import.rs#L1175) | ✅ |
| [Hero's Cape](src/import.rs#L1169) | ✅ |
| [Lillie's Pearl](src/import.rs#L1172) | ✅ |
| [Lucky Helmet](src/import.rs#L1174) | ✅ |
| [Powerglass](src/import.rs#L1176) | ✅ |
| [Punk Helmet](src/import.rs#L1173) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1179) | ✅ |
| [Area Zero Underdepths](src/import.rs#L714) | ✅ |
| [Battle Cage](src/import.rs#L715) | ✅ |
| [Festival Grounds](src/import.rs#L1188) | ✅ |
| [Forest of Vitality](src/import.rs#L1187) | ✅ |
| [Gravity Mountain](src/import.rs#L1177) | ✅ |
| [Jamming Tower](src/import.rs#L1185) | ✅ |
| [Lumiose City](src/import.rs#L1184) | ✅ |
| [N's Castle](src/import.rs#L1178) | ✅ |
| [Nighttime Mine](src/import.rs#L713) | ✅ |
| [Risky Ruins](src/import.rs#L1186) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1180) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L716) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1626) | ✅ |
| [Enriching Energy](src/import.rs#L1609) | ✅ |
| [Growing Grass Energy](src/import.rs#L1608) | ✅ |
| [Mist Energy](src/import.rs#L1623) | ✅ |
| [Prism Energy](src/import.rs#L1629) | ✅ |
| [Spiky Energy](src/import.rs#L1620) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1612) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1824) | [✅](src/import.rs#L1824) | [✅](src/import.rs#L1706) |
| [Alakazam](src/import.rs#L1979) | [✅](src/import.rs#L1979) | [✅](src/import.rs#L1696) |
| [Annihilape](src/import.rs#L1853) | [✅](src/import.rs#L1853) | [✅](src/import.rs#L1662) |
| [Applin](src/import.rs#L1814) | [✅](src/import.rs#L1814) | — |
| [Bayleef](src/import.rs#L1856) | [✅](src/import.rs#L1856) | — |
| [Beldum](src/import.rs#L1836) | [✅](src/import.rs#L1836) | — |
| [Blaziken ex](src/import.rs#L1920) | [✅](src/import.rs#L1920) | [✅](src/import.rs#L1736) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1884) | [✅](src/import.rs#L1884) | [✅](src/import.rs#L1670) |
| [Brute Bonnet](src/import.rs#L1795) | [✅](src/import.rs#L1795) | — |
| [Budew](src/import.rs#L1861) | [✅](src/import.rs#L1861) | — |
| [Buneary](src/import.rs#L1855) | [✅](src/import.rs#L1855) | — |
| [Carvanha](src/import.rs#L1773) | [✅](src/import.rs#L1773) | — |
| [Celebi](src/import.rs#L1854) | [✅](src/import.rs#L1854) | — |
| [Chi-Yu](src/import.rs#L1952) | [✅](src/import.rs#L1952) | — |
| [Chien-Pao](src/import.rs#L1921) | [✅](src/import.rs#L1921) | [✅](src/import.rs#L1739) |
| [Chikorita](src/import.rs#L1857) | [✅](src/import.rs#L1857) | — |
| [Cofagrigus](src/import.rs#L1902) | [✅](src/import.rs#L1902) | — |
| [Combusken](src/import.rs#L1871) | [✅](src/import.rs#L1871) | — |
| [Crustle](src/import.rs#L1991) | [✅](src/import.rs#L1991) | [✅](src/import.rs#L1651) |
| [Dedenne](src/import.rs#L1811) | [✅](src/import.rs#L1811) | — |
| [Dipplin](src/import.rs#L1936) | [✅](src/import.rs#L1936) | [✅](src/import.rs#L1724) |
| [Dragapult ex](src/import.rs#L1818) | [✅](src/import.rs#L1818) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1679) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1825) | [✅](src/import.rs#L1825) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1703) |
| [Dudunsparce ex](src/import.rs#L1786) | [✅](src/import.rs#L1786) | — |
| [Dunsparce](src/import.rs#L1837) | [✅](src/import.rs#L1837) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1707) |
| [Dusknoir](src/import.rs#L1918) | [✅](src/import.rs#L1918) | [✅](src/import.rs#L1708) |
| [Duskull](src/import.rs#L1840) | [✅](src/import.rs#L1840) | — |
| [Dwebble](src/import.rs#L1830) | [✅](src/import.rs#L1830) | — |
| [Elgyem](src/import.rs#L1860) | [✅](src/import.rs#L1860) | — |
| [Enamorus](src/import.rs#L1880) | [✅](src/import.rs#L1880) | — |
| [Fan Rotom](src/import.rs#L1925) | [✅](src/import.rs#L1925) | [✅](src/import.rs#L1749) |
| [Fezandipiti ex](src/import.rs#L1999) | [✅](src/import.rs#L1999) | [✅](src/import.rs#L1697) |
| [Flutter Mane](src/import.rs#L1916) | [✅](src/import.rs#L1916) | [✅](src/import.rs#L1673) |
| [Genesect](src/import.rs#L1971) | [✅](src/import.rs#L1971) | [✅](src/import.rs#L1712) |
| [Genesect ex](src/import.rs#L1919) | [✅](src/import.rs#L1919) | [✅](src/import.rs#L1709) |
| [Goldeen](src/import.rs#L1934) | [✅](src/import.rs#L1934) | [✅](src/import.rs#L1722) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1827) | [✅](src/import.rs#L1827) | [✅](src/import.rs#L1663) |
| [Hydrapple ex](src/import.rs#L1885) | [✅](src/import.rs#L1885) | [✅](src/import.rs#L1664) |
| [Iron Crown ex](src/import.rs#L1845) | [✅](src/import.rs#L1845) | [✅](src/import.rs#L1659) |
| [Iron Leaves ex](src/import.rs#L1924) | [✅](src/import.rs#L1924) | [✅](src/import.rs#L1740) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1692) |
| [Koraidon ex](src/import.rs#L1846) | [✅](src/import.rs#L1846) | — |
| [Kyurem](src/import.rs#L1968) | [✅](src/import.rs#L1968) | [✅](src/import.rs#L1731) |
| [Latias ex](src/import.rs#L1985) | [✅](src/import.rs#L1985) | [✅](src/import.rs#L1650) |
| [Lillie's Clefairy ex](src/import.rs#L2009) | [✅](src/import.rs#L2009) | [✅](src/import.rs#L1654) |
| [Mega Absol ex](src/import.rs#L1892) | [✅](src/import.rs#L1892) | — |
| [Mega Excadrill ex](src/import.rs#L1888) | [✅](src/import.rs#L1888) | — |
| [Mega Kangaskhan ex](src/import.rs#L1995) | [✅](src/import.rs#L1995) | [✅](src/import.rs#L1647) |
| [Mega Lopunny ex](src/import.rs#L1793) | [✅](src/import.rs#L1793) | — |
| [Mega Sharpedo ex](src/import.rs#L1828) | [✅](src/import.rs#L1828) | — |
| [Mega Skarmory ex](src/import.rs#L1906) | [✅](src/import.rs#L1906) | — |
| [Mega Slowbro ex](src/import.rs#L1946) | [✅](src/import.rs#L1946) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1718) |
| [Meowth ex](src/import.rs#L1998) | [✅](src/import.rs#L1998) | [✅](src/import.rs#L1691) |
| [Metagross](src/import.rs#L1802) | [✅](src/import.rs#L1802) | — |
| [Metang](src/import.rs#L1986) | [✅](src/import.rs#L1986) | [✅](src/import.rs#L1682) |
| [Moltres](src/import.rs#L1838) | [✅](src/import.rs#L1838) | — |
| [Munkidori](src/import.rs#L1992) | [✅](src/import.rs#L1992) | [✅](src/import.rs#L1685) |
| [N's Darmanitan](src/import.rs#L1783) | [✅](src/import.rs#L1783) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1780) | [✅](src/import.rs#L1780) | — |
| [N's Zekrom](src/import.rs#L1792) | [✅](src/import.rs#L1792) | — |
| [N's Zoroark ex](src/import.rs#L1965) | [✅](src/import.rs#L1965) | [✅](src/import.rs#L1715) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1988) | [✅](src/import.rs#L1988) | [✅](src/import.rs#L1693) |
| [Paldean Tauros](src/import.rs#L1776) | [✅](src/import.rs#L1776) | — |
| [Passimian](src/import.rs#L1789) | [✅](src/import.rs#L1789) | — |
| [Patrat](src/import.rs#L1987) | [✅](src/import.rs#L1987) | [✅](src/import.rs#L1652) |
| [Pecharunt](src/import.rs#L1929) | [✅](src/import.rs#L1929) | [✅](src/import.rs#L1719) |
| [Pecharunt ex](src/import.rs#L1926) | [✅](src/import.rs#L1926) | [✅](src/import.rs#L1756) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1653) |
| [Rabsca](src/import.rs#L1842) | [✅](src/import.rs#L1842) | [✅](src/import.rs#L1658) |
| [Raging Bolt ex](src/import.rs#L1807) | [✅](src/import.rs#L1807) | — |
| [Rellor](src/import.rs#L1774) | [✅](src/import.rs#L1774) | — |
| [Seaking](src/import.rs#L1935) | [✅](src/import.rs#L1935) | [✅](src/import.rs#L1723) |
| [Shaymin](src/import.rs#L1982) | [✅](src/import.rs#L1982) | [✅](src/import.rs#L1657) |
| [Slowking](src/import.rs#L1831) | [✅](src/import.rs#L1831) | — |
| [Slowpoke](src/import.rs#L1839) | [✅](src/import.rs#L1839) | ❌ |
| [Smoochum](src/import.rs#L1899) | [✅](src/import.rs#L1899) | — |
| [Stunfisk](src/import.rs#L1881) | [✅](src/import.rs#L1881) | — |
| [Tapu Bulu](src/import.rs#L1775) | [✅](src/import.rs#L1775) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1676) |
| [Teal Mask Ogerpon ex](src/import.rs#L2000) | [✅](src/import.rs#L2000) | [✅](src/import.rs#L1700) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1725) |
| [Torchic](src/import.rs#L1841) | [✅](src/import.rs#L1841) | — |
| [Toxel](src/import.rs#L1826) | [✅](src/import.rs#L1826) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1743) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1816) | [✅](src/import.rs#L1816) | — |
| [Yveltal](src/import.rs#L1815) | [✅](src/import.rs#L1815) | — |
| [Zeraora](src/import.rs#L1798) | [✅](src/import.rs#L1798) | — |

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

