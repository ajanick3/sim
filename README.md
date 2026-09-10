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
| Items | 38 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1022) | ✅ |
| [Black Belt's Training](src/import.rs#L1030) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L643) | ✅ |
| [Brock's Scouting](src/import.rs#L1060) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L962) | ✅ |
| [Crispin](src/import.rs#L868) | ✅ |
| [Cyrano](src/import.rs#L717) | ✅ |
| [Dawn](src/import.rs#L834) | ✅ |
| [Eri](src/import.rs#L1053) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1034) | ✅ |
| [Gwynn](src/import.rs#L731) | ✅ |
| [Hilda](src/import.rs#L788) | ✅ |
| [Janine's Secret Art](src/import.rs#L1084) | ✅ |
| [Judge](src/import.rs#L666) | ✅ |
| [Kieran](src/import.rs#L1038) | ✅ |
| [Lana's Aid](src/import.rs#L988) | ✅ |
| [Lillie's Determination](src/import.rs#L667) | ✅ |
| [Morty's Conviction](src/import.rs#L1048) | ✅ |
| [N's Plan](src/import.rs#L1006) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1008) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1002) | ✅ |
| [Surfer](src/import.rs#L1026) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L892) | ✅ |
| [Wally's Compassion](src/import.rs#L1083) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1052) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L703) | ✅ |
| [Bug Catching Set](src/import.rs#L948) | ✅ |
| [Crushing Hammer](src/import.rs#L702) | ✅ |
| [Dusk Ball](src/import.rs#L1114) | ✅ |
| [Energy Recycler](src/import.rs#L1196) | ✅ |
| [Energy Retrieval](src/import.rs#L1099) | ✅ |
| [Energy Search](src/import.rs#L1085) | ✅ |
| [Energy Switch](src/import.rs#L773) | ✅ |
| [Enhanced Hammer](src/import.rs#L647) | ✅ |
| [Glass Trumpet](src/import.rs#L648) | ✅ |
| [Hand Trimmer](src/import.rs#L1113) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L984) | ✅ |
| [N's PP Up](src/import.rs#L906) | ✅ |
| [Night Stretcher](src/import.rs#L674) | ✅ |
| [Prime Catcher](src/import.rs#L1115) | ✅ |
| [Rare Candy](src/import.rs#L891) | ✅ |
| [Sacred Ash](src/import.rs#L745) | ✅ |
| [Secret Box](src/import.rs#L1145) | ✅ |
| [Special Red Card](src/import.rs#L864) | ✅ |
| [Strange Timepiece](src/import.rs#L1116) | ✅ |
| [Switch](src/import.rs#L983) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1182) | ✅ |
| [Tera Orb](src/import.rs#L774) | ✅ |
| [Tool Scrapper](src/import.rs#L642) | ✅ |
| [Transformation Tome](src/import.rs#L1141) | ✅ |
| [Ultra Ball](src/import.rs#L759) | ✅ |
| [Unfair Stamp](src/import.rs#L976) | ✅ |
| [Wondrous Patch](src/import.rs#L920) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1117) | ✅ |
| [Binding Mochi](src/import.rs#L1120) | ✅ |
| [Brave Bangle](src/import.rs#L1119) | ✅ |
| [Handheld Fan](src/import.rs#L1124) | ✅ |
| [Hero's Cape](src/import.rs#L1118) | ✅ |
| [Lillie's Pearl](src/import.rs#L1121) | ✅ |
| [Lucky Helmet](src/import.rs#L1123) | ✅ |
| [Powerglass](src/import.rs#L1125) | ✅ |
| [Punk Helmet](src/import.rs#L1122) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1128) | ✅ |
| [Area Zero Underdepths](src/import.rs#L663) | ✅ |
| [Battle Cage](src/import.rs#L664) | ✅ |
| [Festival Grounds](src/import.rs#L1137) | ✅ |
| [Forest of Vitality](src/import.rs#L1136) | ✅ |
| [Gravity Mountain](src/import.rs#L1126) | ✅ |
| [Jamming Tower](src/import.rs#L1134) | ✅ |
| [Lumiose City](src/import.rs#L1133) | ✅ |
| [N's Castle](src/import.rs#L1127) | ✅ |
| [Nighttime Mine](src/import.rs#L662) | ✅ |
| [Risky Ruins](src/import.rs#L1135) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1129) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L665) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1575) | ✅ |
| [Enriching Energy](src/import.rs#L1558) | ✅ |
| [Growing Grass Energy](src/import.rs#L1557) | ✅ |
| [Mist Energy](src/import.rs#L1572) | ✅ |
| [Prism Energy](src/import.rs#L1578) | ✅ |
| [Spiky Energy](src/import.rs#L1569) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1561) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1773) | [✅](src/import.rs#L1773) | [✅](src/import.rs#L1655) |
| [Alakazam](src/import.rs#L1928) | [✅](src/import.rs#L1928) | [✅](src/import.rs#L1645) |
| [Annihilape](src/import.rs#L1802) | [✅](src/import.rs#L1802) | [✅](src/import.rs#L1611) |
| [Applin](src/import.rs#L1763) | [✅](src/import.rs#L1763) | — |
| [Bayleef](src/import.rs#L1805) | [✅](src/import.rs#L1805) | — |
| [Beldum](src/import.rs#L1785) | [✅](src/import.rs#L1785) | — |
| [Blaziken ex](src/import.rs#L1869) | [✅](src/import.rs#L1869) | [✅](src/import.rs#L1685) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1833) | [✅](src/import.rs#L1833) | [✅](src/import.rs#L1619) |
| [Brute Bonnet](src/import.rs#L1744) | [✅](src/import.rs#L1744) | — |
| [Budew](src/import.rs#L1810) | [✅](src/import.rs#L1810) | — |
| [Buneary](src/import.rs#L1804) | [✅](src/import.rs#L1804) | — |
| [Carvanha](src/import.rs#L1722) | [✅](src/import.rs#L1722) | — |
| [Celebi](src/import.rs#L1803) | [✅](src/import.rs#L1803) | — |
| [Chi-Yu](src/import.rs#L1901) | [✅](src/import.rs#L1901) | — |
| [Chien-Pao](src/import.rs#L1870) | [✅](src/import.rs#L1870) | [✅](src/import.rs#L1688) |
| [Chikorita](src/import.rs#L1806) | [✅](src/import.rs#L1806) | — |
| [Cofagrigus](src/import.rs#L1851) | [✅](src/import.rs#L1851) | — |
| [Combusken](src/import.rs#L1820) | [✅](src/import.rs#L1820) | — |
| [Crustle](src/import.rs#L1940) | [✅](src/import.rs#L1940) | [✅](src/import.rs#L1600) |
| [Dedenne](src/import.rs#L1760) | [✅](src/import.rs#L1760) | — |
| [Dipplin](src/import.rs#L1885) | [✅](src/import.rs#L1885) | [✅](src/import.rs#L1673) |
| [Dragapult ex](src/import.rs#L1767) | [✅](src/import.rs#L1767) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1628) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1774) | [✅](src/import.rs#L1774) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1652) |
| [Dudunsparce ex](src/import.rs#L1735) | [✅](src/import.rs#L1735) | — |
| [Dunsparce](src/import.rs#L1786) | [✅](src/import.rs#L1786) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1656) |
| [Dusknoir](src/import.rs#L1867) | [✅](src/import.rs#L1867) | [✅](src/import.rs#L1657) |
| [Duskull](src/import.rs#L1789) | [✅](src/import.rs#L1789) | — |
| [Dwebble](src/import.rs#L1779) | [✅](src/import.rs#L1779) | — |
| [Elgyem](src/import.rs#L1809) | [✅](src/import.rs#L1809) | — |
| [Enamorus](src/import.rs#L1829) | [✅](src/import.rs#L1829) | — |
| [Fan Rotom](src/import.rs#L1874) | [✅](src/import.rs#L1874) | [✅](src/import.rs#L1698) |
| [Fezandipiti ex](src/import.rs#L1948) | [✅](src/import.rs#L1948) | [✅](src/import.rs#L1646) |
| [Flutter Mane](src/import.rs#L1865) | [✅](src/import.rs#L1865) | [✅](src/import.rs#L1622) |
| [Genesect](src/import.rs#L1920) | [✅](src/import.rs#L1920) | [✅](src/import.rs#L1661) |
| [Genesect ex](src/import.rs#L1868) | [✅](src/import.rs#L1868) | [✅](src/import.rs#L1658) |
| [Goldeen](src/import.rs#L1883) | [✅](src/import.rs#L1883) | [✅](src/import.rs#L1671) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1776) | [✅](src/import.rs#L1776) | [✅](src/import.rs#L1612) |
| [Hydrapple ex](src/import.rs#L1834) | [✅](src/import.rs#L1834) | [✅](src/import.rs#L1613) |
| [Iron Crown ex](src/import.rs#L1794) | [✅](src/import.rs#L1794) | [✅](src/import.rs#L1608) |
| [Iron Leaves ex](src/import.rs#L1873) | [✅](src/import.rs#L1873) | [✅](src/import.rs#L1689) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1641) |
| [Koraidon ex](src/import.rs#L1795) | [✅](src/import.rs#L1795) | — |
| [Kyurem](src/import.rs#L1917) | [✅](src/import.rs#L1917) | [✅](src/import.rs#L1680) |
| [Latias ex](src/import.rs#L1934) | [✅](src/import.rs#L1934) | [✅](src/import.rs#L1599) |
| [Lillie's Clefairy ex](src/import.rs#L1958) | [✅](src/import.rs#L1958) | [✅](src/import.rs#L1603) |
| [Mega Absol ex](src/import.rs#L1841) | [✅](src/import.rs#L1841) | — |
| [Mega Excadrill ex](src/import.rs#L1837) | [✅](src/import.rs#L1837) | — |
| [Mega Kangaskhan ex](src/import.rs#L1944) | [✅](src/import.rs#L1944) | [✅](src/import.rs#L1596) |
| [Mega Lopunny ex](src/import.rs#L1742) | [✅](src/import.rs#L1742) | — |
| [Mega Sharpedo ex](src/import.rs#L1777) | [✅](src/import.rs#L1777) | — |
| [Mega Skarmory ex](src/import.rs#L1855) | [✅](src/import.rs#L1855) | — |
| [Mega Slowbro ex](src/import.rs#L1895) | [✅](src/import.rs#L1895) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1667) |
| [Meowth ex](src/import.rs#L1947) | [✅](src/import.rs#L1947) | [✅](src/import.rs#L1640) |
| [Metagross](src/import.rs#L1751) | [✅](src/import.rs#L1751) | — |
| [Metang](src/import.rs#L1935) | [✅](src/import.rs#L1935) | [✅](src/import.rs#L1631) |
| [Moltres](src/import.rs#L1787) | [✅](src/import.rs#L1787) | — |
| [Munkidori](src/import.rs#L1941) | [✅](src/import.rs#L1941) | [✅](src/import.rs#L1634) |
| [N's Darmanitan](src/import.rs#L1732) | [✅](src/import.rs#L1732) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1729) | [✅](src/import.rs#L1729) | — |
| [N's Zekrom](src/import.rs#L1741) | [✅](src/import.rs#L1741) | — |
| [N's Zoroark ex](src/import.rs#L1914) | [✅](src/import.rs#L1914) | [✅](src/import.rs#L1664) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1937) | [✅](src/import.rs#L1937) | [✅](src/import.rs#L1642) |
| [Paldean Tauros](src/import.rs#L1725) | [✅](src/import.rs#L1725) | — |
| [Passimian](src/import.rs#L1738) | [✅](src/import.rs#L1738) | — |
| [Patrat](src/import.rs#L1936) | [✅](src/import.rs#L1936) | [✅](src/import.rs#L1601) |
| [Pecharunt](src/import.rs#L1878) | [✅](src/import.rs#L1878) | [✅](src/import.rs#L1668) |
| [Pecharunt ex](src/import.rs#L1875) | [✅](src/import.rs#L1875) | [✅](src/import.rs#L1705) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1602) |
| [Rabsca](src/import.rs#L1791) | [✅](src/import.rs#L1791) | [✅](src/import.rs#L1607) |
| [Raging Bolt ex](src/import.rs#L1756) | [✅](src/import.rs#L1756) | — |
| [Rellor](src/import.rs#L1723) | [✅](src/import.rs#L1723) | — |
| [Seaking](src/import.rs#L1884) | [✅](src/import.rs#L1884) | [✅](src/import.rs#L1672) |
| [Shaymin](src/import.rs#L1931) | [✅](src/import.rs#L1931) | [✅](src/import.rs#L1606) |
| [Slowking](src/import.rs#L1780) | [✅](src/import.rs#L1780) | — |
| [Slowpoke](src/import.rs#L1788) | [✅](src/import.rs#L1788) | ❌ |
| [Smoochum](src/import.rs#L1848) | [✅](src/import.rs#L1848) | — |
| [Stunfisk](src/import.rs#L1830) | [✅](src/import.rs#L1830) | — |
| [Tapu Bulu](src/import.rs#L1724) | [✅](src/import.rs#L1724) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1625) |
| [Teal Mask Ogerpon ex](src/import.rs#L1949) | [✅](src/import.rs#L1949) | [✅](src/import.rs#L1649) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1674) |
| [Torchic](src/import.rs#L1790) | [✅](src/import.rs#L1790) | — |
| [Toxel](src/import.rs#L1775) | [✅](src/import.rs#L1775) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1692) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1765) | [✅](src/import.rs#L1765) | — |
| [Yveltal](src/import.rs#L1764) | [✅](src/import.rs#L1764) | — |
| [Zeraora](src/import.rs#L1747) | [✅](src/import.rs#L1747) | — |

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

