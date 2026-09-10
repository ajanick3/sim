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
| Tools | 17 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1075) | ✅ |
| [Black Belt's Training](src/import.rs#L1083) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L696) | ✅ |
| [Brock's Scouting](src/import.rs#L1113) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1015) | ✅ |
| [Crispin](src/import.rs#L921) | ✅ |
| [Cyrano](src/import.rs#L770) | ✅ |
| [Dawn](src/import.rs#L887) | ✅ |
| [Eri](src/import.rs#L1106) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1087) | ✅ |
| [Gwynn](src/import.rs#L784) | ✅ |
| [Hilda](src/import.rs#L841) | ✅ |
| [Janine's Secret Art](src/import.rs#L1137) | ✅ |
| [Judge](src/import.rs#L719) | ✅ |
| [Kieran](src/import.rs#L1091) | ✅ |
| [Lana's Aid](src/import.rs#L1041) | ✅ |
| [Lillie's Determination](src/import.rs#L720) | ✅ |
| [Morty's Conviction](src/import.rs#L1101) | ✅ |
| [N's Plan](src/import.rs#L1059) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1061) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1055) | ✅ |
| [Surfer](src/import.rs#L1079) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L945) | ✅ |
| [Wally's Compassion](src/import.rs#L1136) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1105) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L756) | ✅ |
| [Bug Catching Set](src/import.rs#L1001) | ✅ |
| [Crushing Hammer](src/import.rs#L755) | ✅ |
| [Dusk Ball](src/import.rs#L1167) | ✅ |
| [Energy Recycler](src/import.rs#L1249) | ✅ |
| [Energy Retrieval](src/import.rs#L1152) | ✅ |
| [Energy Search](src/import.rs#L1138) | ✅ |
| [Energy Switch](src/import.rs#L826) | ✅ |
| [Enhanced Hammer](src/import.rs#L700) | ✅ |
| [Glass Trumpet](src/import.rs#L701) | ✅ |
| [Hand Trimmer](src/import.rs#L1166) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1037) | ✅ |
| [N's PP Up](src/import.rs#L959) | ✅ |
| [Night Stretcher](src/import.rs#L727) | ✅ |
| [Prime Catcher](src/import.rs#L1168) | ✅ |
| [Rare Candy](src/import.rs#L944) | ✅ |
| [Sacred Ash](src/import.rs#L798) | ✅ |
| [Secret Box](src/import.rs#L1198) | ✅ |
| [Special Red Card](src/import.rs#L917) | ✅ |
| [Strange Timepiece](src/import.rs#L1169) | ✅ |
| [Switch](src/import.rs#L1036) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1235) | ✅ |
| [Tera Orb](src/import.rs#L827) | ✅ |
| [Tool Scrapper](src/import.rs#L695) | ✅ |
| [Transformation Tome](src/import.rs#L1194) | ✅ |
| [Ultra Ball](src/import.rs#L812) | ✅ |
| [Unfair Stamp](src/import.rs#L1029) | ✅ |
| [Wondrous Patch](src/import.rs#L973) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1170) | ✅ |
| [Binding Mochi](src/import.rs#L1173) | ✅ |
| [Brave Bangle](src/import.rs#L1172) | ✅ |
| [Handheld Fan](src/import.rs#L1177) | ✅ |
| [Hero's Cape](src/import.rs#L1171) | ✅ |
| [Lillie's Pearl](src/import.rs#L1174) | ✅ |
| [Lucky Helmet](src/import.rs#L1176) | ✅ |
| [Powerglass](src/import.rs#L1178) | ✅ |
| [Punk Helmet](src/import.rs#L1175) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1181) | ✅ |
| [Area Zero Underdepths](src/import.rs#L716) | ✅ |
| [Battle Cage](src/import.rs#L717) | ✅ |
| [Festival Grounds](src/import.rs#L1190) | ✅ |
| [Forest of Vitality](src/import.rs#L1189) | ✅ |
| [Gravity Mountain](src/import.rs#L1179) | ✅ |
| [Jamming Tower](src/import.rs#L1187) | ✅ |
| [Lumiose City](src/import.rs#L1186) | ✅ |
| [N's Castle](src/import.rs#L1180) | ✅ |
| [Nighttime Mine](src/import.rs#L715) | ✅ |
| [Risky Ruins](src/import.rs#L1188) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1182) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L718) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1628) | ✅ |
| [Enriching Energy](src/import.rs#L1611) | ✅ |
| [Growing Grass Energy](src/import.rs#L1610) | ✅ |
| [Mist Energy](src/import.rs#L1625) | ✅ |
| [Prism Energy](src/import.rs#L1631) | ✅ |
| [Spiky Energy](src/import.rs#L1622) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1614) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1826) | [✅](src/import.rs#L1826) | [✅](src/import.rs#L1708) |
| [Alakazam](src/import.rs#L1981) | [✅](src/import.rs#L1981) | [✅](src/import.rs#L1698) |
| [Annihilape](src/import.rs#L1855) | [✅](src/import.rs#L1855) | [✅](src/import.rs#L1664) |
| [Applin](src/import.rs#L1816) | [✅](src/import.rs#L1816) | — |
| [Bayleef](src/import.rs#L1858) | [✅](src/import.rs#L1858) | — |
| [Beldum](src/import.rs#L1838) | [✅](src/import.rs#L1838) | — |
| [Blaziken ex](src/import.rs#L1922) | [✅](src/import.rs#L1922) | [✅](src/import.rs#L1738) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1886) | [✅](src/import.rs#L1886) | [✅](src/import.rs#L1672) |
| [Brute Bonnet](src/import.rs#L1797) | [✅](src/import.rs#L1797) | — |
| [Budew](src/import.rs#L1863) | [✅](src/import.rs#L1863) | — |
| [Buneary](src/import.rs#L1857) | [✅](src/import.rs#L1857) | — |
| [Carvanha](src/import.rs#L1775) | [✅](src/import.rs#L1775) | — |
| [Celebi](src/import.rs#L1856) | [✅](src/import.rs#L1856) | — |
| [Chi-Yu](src/import.rs#L1954) | [✅](src/import.rs#L1954) | — |
| [Chien-Pao](src/import.rs#L1923) | [✅](src/import.rs#L1923) | [✅](src/import.rs#L1741) |
| [Chikorita](src/import.rs#L1859) | [✅](src/import.rs#L1859) | — |
| [Cofagrigus](src/import.rs#L1904) | [✅](src/import.rs#L1904) | — |
| [Combusken](src/import.rs#L1873) | [✅](src/import.rs#L1873) | — |
| [Crustle](src/import.rs#L1993) | [✅](src/import.rs#L1993) | [✅](src/import.rs#L1653) |
| [Dedenne](src/import.rs#L1813) | [✅](src/import.rs#L1813) | — |
| [Dipplin](src/import.rs#L1938) | [✅](src/import.rs#L1938) | [✅](src/import.rs#L1726) |
| [Dragapult ex](src/import.rs#L1820) | [✅](src/import.rs#L1820) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1681) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1827) | [✅](src/import.rs#L1827) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1705) |
| [Dudunsparce ex](src/import.rs#L1788) | [✅](src/import.rs#L1788) | — |
| [Dunsparce](src/import.rs#L1839) | [✅](src/import.rs#L1839) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1709) |
| [Dusknoir](src/import.rs#L1920) | [✅](src/import.rs#L1920) | [✅](src/import.rs#L1710) |
| [Duskull](src/import.rs#L1842) | [✅](src/import.rs#L1842) | — |
| [Dwebble](src/import.rs#L1832) | [✅](src/import.rs#L1832) | — |
| [Elgyem](src/import.rs#L1862) | [✅](src/import.rs#L1862) | — |
| [Enamorus](src/import.rs#L1882) | [✅](src/import.rs#L1882) | — |
| [Fan Rotom](src/import.rs#L1927) | [✅](src/import.rs#L1927) | [✅](src/import.rs#L1751) |
| [Fezandipiti ex](src/import.rs#L2001) | [✅](src/import.rs#L2001) | [✅](src/import.rs#L1699) |
| [Flutter Mane](src/import.rs#L1918) | [✅](src/import.rs#L1918) | [✅](src/import.rs#L1675) |
| [Genesect](src/import.rs#L1973) | [✅](src/import.rs#L1973) | [✅](src/import.rs#L1714) |
| [Genesect ex](src/import.rs#L1921) | [✅](src/import.rs#L1921) | [✅](src/import.rs#L1711) |
| [Goldeen](src/import.rs#L1936) | [✅](src/import.rs#L1936) | [✅](src/import.rs#L1724) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1829) | [✅](src/import.rs#L1829) | [✅](src/import.rs#L1665) |
| [Hydrapple ex](src/import.rs#L1887) | [✅](src/import.rs#L1887) | [✅](src/import.rs#L1666) |
| [Iron Crown ex](src/import.rs#L1847) | [✅](src/import.rs#L1847) | [✅](src/import.rs#L1661) |
| [Iron Leaves ex](src/import.rs#L1926) | [✅](src/import.rs#L1926) | [✅](src/import.rs#L1742) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1694) |
| [Koraidon ex](src/import.rs#L1848) | [✅](src/import.rs#L1848) | — |
| [Kyurem](src/import.rs#L1970) | [✅](src/import.rs#L1970) | [✅](src/import.rs#L1733) |
| [Latias ex](src/import.rs#L1987) | [✅](src/import.rs#L1987) | [✅](src/import.rs#L1652) |
| [Lillie's Clefairy ex](src/import.rs#L2011) | [✅](src/import.rs#L2011) | [✅](src/import.rs#L1656) |
| [Mega Absol ex](src/import.rs#L1894) | [✅](src/import.rs#L1894) | — |
| [Mega Excadrill ex](src/import.rs#L1890) | [✅](src/import.rs#L1890) | — |
| [Mega Kangaskhan ex](src/import.rs#L1997) | [✅](src/import.rs#L1997) | [✅](src/import.rs#L1649) |
| [Mega Lopunny ex](src/import.rs#L1795) | [✅](src/import.rs#L1795) | — |
| [Mega Sharpedo ex](src/import.rs#L1830) | [✅](src/import.rs#L1830) | — |
| [Mega Skarmory ex](src/import.rs#L1908) | [✅](src/import.rs#L1908) | — |
| [Mega Slowbro ex](src/import.rs#L1948) | [✅](src/import.rs#L1948) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1720) |
| [Meowth ex](src/import.rs#L2000) | [✅](src/import.rs#L2000) | [✅](src/import.rs#L1693) |
| [Metagross](src/import.rs#L1804) | [✅](src/import.rs#L1804) | — |
| [Metang](src/import.rs#L1988) | [✅](src/import.rs#L1988) | [✅](src/import.rs#L1684) |
| [Moltres](src/import.rs#L1840) | [✅](src/import.rs#L1840) | — |
| [Munkidori](src/import.rs#L1994) | [✅](src/import.rs#L1994) | [✅](src/import.rs#L1687) |
| [N's Darmanitan](src/import.rs#L1785) | [✅](src/import.rs#L1785) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1782) | [✅](src/import.rs#L1782) | — |
| [N's Zekrom](src/import.rs#L1794) | [✅](src/import.rs#L1794) | — |
| [N's Zoroark ex](src/import.rs#L1967) | [✅](src/import.rs#L1967) | [✅](src/import.rs#L1717) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1990) | [✅](src/import.rs#L1990) | [✅](src/import.rs#L1695) |
| [Paldean Tauros](src/import.rs#L1778) | [✅](src/import.rs#L1778) | — |
| [Passimian](src/import.rs#L1791) | [✅](src/import.rs#L1791) | — |
| [Patrat](src/import.rs#L1989) | [✅](src/import.rs#L1989) | [✅](src/import.rs#L1654) |
| [Pecharunt](src/import.rs#L1931) | [✅](src/import.rs#L1931) | [✅](src/import.rs#L1721) |
| [Pecharunt ex](src/import.rs#L1928) | [✅](src/import.rs#L1928) | [✅](src/import.rs#L1758) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1655) |
| [Rabsca](src/import.rs#L1844) | [✅](src/import.rs#L1844) | [✅](src/import.rs#L1660) |
| [Raging Bolt ex](src/import.rs#L1809) | [✅](src/import.rs#L1809) | — |
| [Rellor](src/import.rs#L1776) | [✅](src/import.rs#L1776) | — |
| [Seaking](src/import.rs#L1937) | [✅](src/import.rs#L1937) | [✅](src/import.rs#L1725) |
| [Shaymin](src/import.rs#L1984) | [✅](src/import.rs#L1984) | [✅](src/import.rs#L1659) |
| [Slowking](src/import.rs#L1833) | [✅](src/import.rs#L1833) | — |
| [Slowpoke](src/import.rs#L1841) | [✅](src/import.rs#L1841) | ❌ |
| [Smoochum](src/import.rs#L1901) | [✅](src/import.rs#L1901) | — |
| [Stunfisk](src/import.rs#L1883) | [✅](src/import.rs#L1883) | — |
| [Tapu Bulu](src/import.rs#L1777) | [✅](src/import.rs#L1777) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1678) |
| [Teal Mask Ogerpon ex](src/import.rs#L2002) | [✅](src/import.rs#L2002) | [✅](src/import.rs#L1702) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1727) |
| [Torchic](src/import.rs#L1843) | [✅](src/import.rs#L1843) | — |
| [Toxel](src/import.rs#L1828) | [✅](src/import.rs#L1828) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1745) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1818) | [✅](src/import.rs#L1818) | — |
| [Yveltal](src/import.rs#L1817) | [✅](src/import.rs#L1817) | — |
| [Zeraora](src/import.rs#L1800) | [✅](src/import.rs#L1800) | — |

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

