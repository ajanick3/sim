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
| Items | 41 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1038) | ✅ |
| [Black Belt's Training](src/import.rs#L1046) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L659) | ✅ |
| [Brock's Scouting](src/import.rs#L1076) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L978) | ✅ |
| [Crispin](src/import.rs#L884) | ✅ |
| [Cyrano](src/import.rs#L733) | ✅ |
| [Dawn](src/import.rs#L850) | ✅ |
| [Eri](src/import.rs#L1069) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1050) | ✅ |
| [Gwynn](src/import.rs#L747) | ✅ |
| [Hilda](src/import.rs#L804) | ✅ |
| [Janine's Secret Art](src/import.rs#L1100) | ✅ |
| [Judge](src/import.rs#L682) | ✅ |
| [Kieran](src/import.rs#L1054) | ✅ |
| [Lana's Aid](src/import.rs#L1004) | ✅ |
| [Lillie's Determination](src/import.rs#L683) | ✅ |
| [Morty's Conviction](src/import.rs#L1064) | ✅ |
| [N's Plan](src/import.rs#L1022) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1024) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1018) | ✅ |
| [Surfer](src/import.rs#L1042) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L908) | ✅ |
| [Wally's Compassion](src/import.rs#L1099) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1068) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L719) | ✅ |
| [Bug Catching Set](src/import.rs#L964) | ✅ |
| [Crushing Hammer](src/import.rs#L718) | ✅ |
| [Dusk Ball](src/import.rs#L1130) | ✅ |
| [Energy Recycler](src/import.rs#L1212) | ✅ |
| [Energy Retrieval](src/import.rs#L1115) | ✅ |
| [Energy Search](src/import.rs#L1101) | ✅ |
| [Energy Switch](src/import.rs#L789) | ✅ |
| [Enhanced Hammer](src/import.rs#L663) | ✅ |
| [Glass Trumpet](src/import.rs#L664) | ✅ |
| [Hand Trimmer](src/import.rs#L1129) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1000) | ✅ |
| [N's PP Up](src/import.rs#L922) | ✅ |
| [Night Stretcher](src/import.rs#L690) | ✅ |
| [Prime Catcher](src/import.rs#L1131) | ✅ |
| [Rare Candy](src/import.rs#L907) | ✅ |
| [Sacred Ash](src/import.rs#L761) | ✅ |
| [Secret Box](src/import.rs#L1161) | ✅ |
| [Special Red Card](src/import.rs#L880) | ✅ |
| [Strange Timepiece](src/import.rs#L1132) | ✅ |
| [Switch](src/import.rs#L999) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1198) | ✅ |
| [Tera Orb](src/import.rs#L790) | ✅ |
| [Tool Scrapper](src/import.rs#L658) | ✅ |
| [Transformation Tome](src/import.rs#L1157) | ✅ |
| [Ultra Ball](src/import.rs#L775) | ✅ |
| [Unfair Stamp](src/import.rs#L992) | ✅ |
| [Wondrous Patch](src/import.rs#L936) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1133) | ✅ |
| [Binding Mochi](src/import.rs#L1136) | ✅ |
| [Brave Bangle](src/import.rs#L1135) | ✅ |
| [Handheld Fan](src/import.rs#L1140) | ✅ |
| [Hero's Cape](src/import.rs#L1134) | ✅ |
| [Lillie's Pearl](src/import.rs#L1137) | ✅ |
| [Lucky Helmet](src/import.rs#L1139) | ✅ |
| [Powerglass](src/import.rs#L1141) | ✅ |
| [Punk Helmet](src/import.rs#L1138) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1144) | ✅ |
| [Area Zero Underdepths](src/import.rs#L679) | ✅ |
| [Battle Cage](src/import.rs#L680) | ✅ |
| [Festival Grounds](src/import.rs#L1153) | ✅ |
| [Forest of Vitality](src/import.rs#L1152) | ✅ |
| [Gravity Mountain](src/import.rs#L1142) | ✅ |
| [Jamming Tower](src/import.rs#L1150) | ✅ |
| [Lumiose City](src/import.rs#L1149) | ✅ |
| [N's Castle](src/import.rs#L1143) | ✅ |
| [Nighttime Mine](src/import.rs#L678) | ✅ |
| [Risky Ruins](src/import.rs#L1151) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1145) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L681) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1591) | ✅ |
| [Enriching Energy](src/import.rs#L1574) | ✅ |
| [Growing Grass Energy](src/import.rs#L1573) | ✅ |
| [Mist Energy](src/import.rs#L1588) | ✅ |
| [Prism Energy](src/import.rs#L1594) | ✅ |
| [Spiky Energy](src/import.rs#L1585) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1577) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1789) | [✅](src/import.rs#L1789) | [✅](src/import.rs#L1671) |
| [Alakazam](src/import.rs#L1944) | [✅](src/import.rs#L1944) | [✅](src/import.rs#L1661) |
| [Annihilape](src/import.rs#L1818) | [✅](src/import.rs#L1818) | [✅](src/import.rs#L1627) |
| [Applin](src/import.rs#L1779) | [✅](src/import.rs#L1779) | — |
| [Bayleef](src/import.rs#L1821) | [✅](src/import.rs#L1821) | — |
| [Beldum](src/import.rs#L1801) | [✅](src/import.rs#L1801) | — |
| [Blaziken ex](src/import.rs#L1885) | [✅](src/import.rs#L1885) | [✅](src/import.rs#L1701) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1849) | [✅](src/import.rs#L1849) | [✅](src/import.rs#L1635) |
| [Brute Bonnet](src/import.rs#L1760) | [✅](src/import.rs#L1760) | — |
| [Budew](src/import.rs#L1826) | [✅](src/import.rs#L1826) | — |
| [Buneary](src/import.rs#L1820) | [✅](src/import.rs#L1820) | — |
| [Carvanha](src/import.rs#L1738) | [✅](src/import.rs#L1738) | — |
| [Celebi](src/import.rs#L1819) | [✅](src/import.rs#L1819) | — |
| [Chi-Yu](src/import.rs#L1917) | [✅](src/import.rs#L1917) | — |
| [Chien-Pao](src/import.rs#L1886) | [✅](src/import.rs#L1886) | [✅](src/import.rs#L1704) |
| [Chikorita](src/import.rs#L1822) | [✅](src/import.rs#L1822) | — |
| [Cofagrigus](src/import.rs#L1867) | [✅](src/import.rs#L1867) | — |
| [Combusken](src/import.rs#L1836) | [✅](src/import.rs#L1836) | — |
| [Crustle](src/import.rs#L1956) | [✅](src/import.rs#L1956) | [✅](src/import.rs#L1616) |
| [Dedenne](src/import.rs#L1776) | [✅](src/import.rs#L1776) | — |
| [Dipplin](src/import.rs#L1901) | [✅](src/import.rs#L1901) | [✅](src/import.rs#L1689) |
| [Dragapult ex](src/import.rs#L1783) | [✅](src/import.rs#L1783) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1644) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1790) | [✅](src/import.rs#L1790) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1668) |
| [Dudunsparce ex](src/import.rs#L1751) | [✅](src/import.rs#L1751) | — |
| [Dunsparce](src/import.rs#L1802) | [✅](src/import.rs#L1802) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1672) |
| [Dusknoir](src/import.rs#L1883) | [✅](src/import.rs#L1883) | [✅](src/import.rs#L1673) |
| [Duskull](src/import.rs#L1805) | [✅](src/import.rs#L1805) | — |
| [Dwebble](src/import.rs#L1795) | [✅](src/import.rs#L1795) | — |
| [Elgyem](src/import.rs#L1825) | [✅](src/import.rs#L1825) | — |
| [Enamorus](src/import.rs#L1845) | [✅](src/import.rs#L1845) | — |
| [Fan Rotom](src/import.rs#L1890) | [✅](src/import.rs#L1890) | [✅](src/import.rs#L1714) |
| [Fezandipiti ex](src/import.rs#L1964) | [✅](src/import.rs#L1964) | [✅](src/import.rs#L1662) |
| [Flutter Mane](src/import.rs#L1881) | [✅](src/import.rs#L1881) | [✅](src/import.rs#L1638) |
| [Genesect](src/import.rs#L1936) | [✅](src/import.rs#L1936) | [✅](src/import.rs#L1677) |
| [Genesect ex](src/import.rs#L1884) | [✅](src/import.rs#L1884) | [✅](src/import.rs#L1674) |
| [Goldeen](src/import.rs#L1899) | [✅](src/import.rs#L1899) | [✅](src/import.rs#L1687) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1792) | [✅](src/import.rs#L1792) | [✅](src/import.rs#L1628) |
| [Hydrapple ex](src/import.rs#L1850) | [✅](src/import.rs#L1850) | [✅](src/import.rs#L1629) |
| [Iron Crown ex](src/import.rs#L1810) | [✅](src/import.rs#L1810) | [✅](src/import.rs#L1624) |
| [Iron Leaves ex](src/import.rs#L1889) | [✅](src/import.rs#L1889) | [✅](src/import.rs#L1705) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1657) |
| [Koraidon ex](src/import.rs#L1811) | [✅](src/import.rs#L1811) | — |
| [Kyurem](src/import.rs#L1933) | [✅](src/import.rs#L1933) | [✅](src/import.rs#L1696) |
| [Latias ex](src/import.rs#L1950) | [✅](src/import.rs#L1950) | [✅](src/import.rs#L1615) |
| [Lillie's Clefairy ex](src/import.rs#L1974) | [✅](src/import.rs#L1974) | [✅](src/import.rs#L1619) |
| [Mega Absol ex](src/import.rs#L1857) | [✅](src/import.rs#L1857) | — |
| [Mega Excadrill ex](src/import.rs#L1853) | [✅](src/import.rs#L1853) | — |
| [Mega Kangaskhan ex](src/import.rs#L1960) | [✅](src/import.rs#L1960) | [✅](src/import.rs#L1612) |
| [Mega Lopunny ex](src/import.rs#L1758) | [✅](src/import.rs#L1758) | — |
| [Mega Sharpedo ex](src/import.rs#L1793) | [✅](src/import.rs#L1793) | — |
| [Mega Skarmory ex](src/import.rs#L1871) | [✅](src/import.rs#L1871) | — |
| [Mega Slowbro ex](src/import.rs#L1911) | [✅](src/import.rs#L1911) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1683) |
| [Meowth ex](src/import.rs#L1963) | [✅](src/import.rs#L1963) | [✅](src/import.rs#L1656) |
| [Metagross](src/import.rs#L1767) | [✅](src/import.rs#L1767) | — |
| [Metang](src/import.rs#L1951) | [✅](src/import.rs#L1951) | [✅](src/import.rs#L1647) |
| [Moltres](src/import.rs#L1803) | [✅](src/import.rs#L1803) | — |
| [Munkidori](src/import.rs#L1957) | [✅](src/import.rs#L1957) | [✅](src/import.rs#L1650) |
| [N's Darmanitan](src/import.rs#L1748) | [✅](src/import.rs#L1748) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1745) | [✅](src/import.rs#L1745) | — |
| [N's Zekrom](src/import.rs#L1757) | [✅](src/import.rs#L1757) | — |
| [N's Zoroark ex](src/import.rs#L1930) | [✅](src/import.rs#L1930) | [✅](src/import.rs#L1680) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1953) | [✅](src/import.rs#L1953) | [✅](src/import.rs#L1658) |
| [Paldean Tauros](src/import.rs#L1741) | [✅](src/import.rs#L1741) | — |
| [Passimian](src/import.rs#L1754) | [✅](src/import.rs#L1754) | — |
| [Patrat](src/import.rs#L1952) | [✅](src/import.rs#L1952) | [✅](src/import.rs#L1617) |
| [Pecharunt](src/import.rs#L1894) | [✅](src/import.rs#L1894) | [✅](src/import.rs#L1684) |
| [Pecharunt ex](src/import.rs#L1891) | [✅](src/import.rs#L1891) | [✅](src/import.rs#L1721) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1618) |
| [Rabsca](src/import.rs#L1807) | [✅](src/import.rs#L1807) | [✅](src/import.rs#L1623) |
| [Raging Bolt ex](src/import.rs#L1772) | [✅](src/import.rs#L1772) | — |
| [Rellor](src/import.rs#L1739) | [✅](src/import.rs#L1739) | — |
| [Seaking](src/import.rs#L1900) | [✅](src/import.rs#L1900) | [✅](src/import.rs#L1688) |
| [Shaymin](src/import.rs#L1947) | [✅](src/import.rs#L1947) | [✅](src/import.rs#L1622) |
| [Slowking](src/import.rs#L1796) | [✅](src/import.rs#L1796) | — |
| [Slowpoke](src/import.rs#L1804) | [✅](src/import.rs#L1804) | ❌ |
| [Smoochum](src/import.rs#L1864) | [✅](src/import.rs#L1864) | — |
| [Stunfisk](src/import.rs#L1846) | [✅](src/import.rs#L1846) | — |
| [Tapu Bulu](src/import.rs#L1740) | [✅](src/import.rs#L1740) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1641) |
| [Teal Mask Ogerpon ex](src/import.rs#L1965) | [✅](src/import.rs#L1965) | [✅](src/import.rs#L1665) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1690) |
| [Torchic](src/import.rs#L1806) | [✅](src/import.rs#L1806) | — |
| [Toxel](src/import.rs#L1791) | [✅](src/import.rs#L1791) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1708) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1781) | [✅](src/import.rs#L1781) | — |
| [Yveltal](src/import.rs#L1780) | [✅](src/import.rs#L1780) | — |
| [Zeraora](src/import.rs#L1763) | [✅](src/import.rs#L1763) | — |

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

