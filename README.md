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
| Stadiums | 15 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1080) | ✅ |
| [Black Belt's Training](src/import.rs#L1088) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L701) | ✅ |
| [Brock's Scouting](src/import.rs#L1118) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1020) | ✅ |
| [Crispin](src/import.rs#L926) | ✅ |
| [Cyrano](src/import.rs#L775) | ✅ |
| [Dawn](src/import.rs#L892) | ✅ |
| [Eri](src/import.rs#L1111) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1092) | ✅ |
| [Gwynn](src/import.rs#L789) | ✅ |
| [Hilda](src/import.rs#L846) | ✅ |
| [Janine's Secret Art](src/import.rs#L1142) | ✅ |
| [Judge](src/import.rs#L724) | ✅ |
| [Kieran](src/import.rs#L1096) | ✅ |
| [Lana's Aid](src/import.rs#L1046) | ✅ |
| [Lillie's Determination](src/import.rs#L725) | ✅ |
| [Morty's Conviction](src/import.rs#L1106) | ✅ |
| [N's Plan](src/import.rs#L1064) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1066) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1060) | ✅ |
| [Surfer](src/import.rs#L1084) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L950) | ✅ |
| [Wally's Compassion](src/import.rs#L1141) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1110) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L761) | ✅ |
| [Bug Catching Set](src/import.rs#L1006) | ✅ |
| [Crushing Hammer](src/import.rs#L760) | ✅ |
| [Dusk Ball](src/import.rs#L1172) | ✅ |
| [Energy Recycler](src/import.rs#L1254) | ✅ |
| [Energy Retrieval](src/import.rs#L1157) | ✅ |
| [Energy Search](src/import.rs#L1143) | ✅ |
| [Energy Switch](src/import.rs#L831) | ✅ |
| [Enhanced Hammer](src/import.rs#L705) | ✅ |
| [Glass Trumpet](src/import.rs#L706) | ✅ |
| [Hand Trimmer](src/import.rs#L1171) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1042) | ✅ |
| [N's PP Up](src/import.rs#L964) | ✅ |
| [Night Stretcher](src/import.rs#L732) | ✅ |
| [Prime Catcher](src/import.rs#L1173) | ✅ |
| [Rare Candy](src/import.rs#L949) | ✅ |
| [Sacred Ash](src/import.rs#L803) | ✅ |
| [Secret Box](src/import.rs#L1203) | ✅ |
| [Special Red Card](src/import.rs#L922) | ✅ |
| [Strange Timepiece](src/import.rs#L1174) | ✅ |
| [Switch](src/import.rs#L1041) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1240) | ✅ |
| [Tera Orb](src/import.rs#L832) | ✅ |
| [Tool Scrapper](src/import.rs#L700) | ✅ |
| [Transformation Tome](src/import.rs#L1199) | ✅ |
| [Ultra Ball](src/import.rs#L817) | ✅ |
| [Unfair Stamp](src/import.rs#L1034) | ✅ |
| [Wondrous Patch](src/import.rs#L978) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1175) | ✅ |
| [Binding Mochi](src/import.rs#L1178) | ✅ |
| [Brave Bangle](src/import.rs#L1177) | ✅ |
| [Handheld Fan](src/import.rs#L1182) | ✅ |
| [Hero's Cape](src/import.rs#L1176) | ✅ |
| [Lillie's Pearl](src/import.rs#L1179) | ✅ |
| [Lucky Helmet](src/import.rs#L1181) | ✅ |
| [Powerglass](src/import.rs#L1183) | ✅ |
| [Punk Helmet](src/import.rs#L1180) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1186) | ✅ |
| [Area Zero Underdepths](src/import.rs#L721) | ✅ |
| [Battle Cage](src/import.rs#L722) | ✅ |
| [Festival Grounds](src/import.rs#L1195) | ✅ |
| [Forest of Vitality](src/import.rs#L1194) | ✅ |
| [Gravity Mountain](src/import.rs#L1184) | ✅ |
| [Jamming Tower](src/import.rs#L1192) | ✅ |
| [Lumiose City](src/import.rs#L1191) | ✅ |
| [N's Castle](src/import.rs#L1185) | ✅ |
| [Nighttime Mine](src/import.rs#L720) | ✅ |
| [Risky Ruins](src/import.rs#L1193) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1187) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L723) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1633) | ✅ |
| [Enriching Energy](src/import.rs#L1616) | ✅ |
| [Growing Grass Energy](src/import.rs#L1615) | ✅ |
| [Mist Energy](src/import.rs#L1630) | ✅ |
| [Prism Energy](src/import.rs#L1636) | ✅ |
| [Spiky Energy](src/import.rs#L1627) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1619) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1831) | [✅](src/import.rs#L1831) | [✅](src/import.rs#L1713) |
| [Alakazam](src/import.rs#L1986) | [✅](src/import.rs#L1986) | [✅](src/import.rs#L1703) |
| [Annihilape](src/import.rs#L1860) | [✅](src/import.rs#L1860) | [✅](src/import.rs#L1669) |
| [Applin](src/import.rs#L1821) | [✅](src/import.rs#L1821) | — |
| [Bayleef](src/import.rs#L1863) | [✅](src/import.rs#L1863) | — |
| [Beldum](src/import.rs#L1843) | [✅](src/import.rs#L1843) | — |
| [Blaziken ex](src/import.rs#L1927) | [✅](src/import.rs#L1927) | [✅](src/import.rs#L1743) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1891) | [✅](src/import.rs#L1891) | [✅](src/import.rs#L1677) |
| [Brute Bonnet](src/import.rs#L1802) | [✅](src/import.rs#L1802) | — |
| [Budew](src/import.rs#L1868) | [✅](src/import.rs#L1868) | — |
| [Buneary](src/import.rs#L1862) | [✅](src/import.rs#L1862) | — |
| [Carvanha](src/import.rs#L1780) | [✅](src/import.rs#L1780) | — |
| [Celebi](src/import.rs#L1861) | [✅](src/import.rs#L1861) | — |
| [Chi-Yu](src/import.rs#L1959) | [✅](src/import.rs#L1959) | — |
| [Chien-Pao](src/import.rs#L1928) | [✅](src/import.rs#L1928) | [✅](src/import.rs#L1746) |
| [Chikorita](src/import.rs#L1864) | [✅](src/import.rs#L1864) | — |
| [Cofagrigus](src/import.rs#L1909) | [✅](src/import.rs#L1909) | — |
| [Combusken](src/import.rs#L1878) | [✅](src/import.rs#L1878) | — |
| [Crustle](src/import.rs#L1998) | [✅](src/import.rs#L1998) | [✅](src/import.rs#L1658) |
| [Dedenne](src/import.rs#L1818) | [✅](src/import.rs#L1818) | — |
| [Dipplin](src/import.rs#L1943) | [✅](src/import.rs#L1943) | [✅](src/import.rs#L1731) |
| [Dragapult ex](src/import.rs#L1825) | [✅](src/import.rs#L1825) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1686) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1832) | [✅](src/import.rs#L1832) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1710) |
| [Dudunsparce ex](src/import.rs#L1793) | [✅](src/import.rs#L1793) | — |
| [Dunsparce](src/import.rs#L1844) | [✅](src/import.rs#L1844) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1714) |
| [Dusknoir](src/import.rs#L1925) | [✅](src/import.rs#L1925) | [✅](src/import.rs#L1715) |
| [Duskull](src/import.rs#L1847) | [✅](src/import.rs#L1847) | — |
| [Dwebble](src/import.rs#L1837) | [✅](src/import.rs#L1837) | — |
| [Elgyem](src/import.rs#L1867) | [✅](src/import.rs#L1867) | — |
| [Enamorus](src/import.rs#L1887) | [✅](src/import.rs#L1887) | — |
| [Fan Rotom](src/import.rs#L1932) | [✅](src/import.rs#L1932) | [✅](src/import.rs#L1756) |
| [Fezandipiti ex](src/import.rs#L2006) | [✅](src/import.rs#L2006) | [✅](src/import.rs#L1704) |
| [Flutter Mane](src/import.rs#L1923) | [✅](src/import.rs#L1923) | [✅](src/import.rs#L1680) |
| [Genesect](src/import.rs#L1978) | [✅](src/import.rs#L1978) | [✅](src/import.rs#L1719) |
| [Genesect ex](src/import.rs#L1926) | [✅](src/import.rs#L1926) | [✅](src/import.rs#L1716) |
| [Goldeen](src/import.rs#L1941) | [✅](src/import.rs#L1941) | [✅](src/import.rs#L1729) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1834) | [✅](src/import.rs#L1834) | [✅](src/import.rs#L1670) |
| [Hydrapple ex](src/import.rs#L1892) | [✅](src/import.rs#L1892) | [✅](src/import.rs#L1671) |
| [Iron Crown ex](src/import.rs#L1852) | [✅](src/import.rs#L1852) | [✅](src/import.rs#L1666) |
| [Iron Leaves ex](src/import.rs#L1931) | [✅](src/import.rs#L1931) | [✅](src/import.rs#L1747) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1699) |
| [Koraidon ex](src/import.rs#L1853) | [✅](src/import.rs#L1853) | — |
| [Kyurem](src/import.rs#L1975) | [✅](src/import.rs#L1975) | [✅](src/import.rs#L1738) |
| [Latias ex](src/import.rs#L1992) | [✅](src/import.rs#L1992) | [✅](src/import.rs#L1657) |
| [Lillie's Clefairy ex](src/import.rs#L2016) | [✅](src/import.rs#L2016) | [✅](src/import.rs#L1661) |
| [Mega Absol ex](src/import.rs#L1899) | [✅](src/import.rs#L1899) | — |
| [Mega Excadrill ex](src/import.rs#L1895) | [✅](src/import.rs#L1895) | — |
| [Mega Kangaskhan ex](src/import.rs#L2002) | [✅](src/import.rs#L2002) | [✅](src/import.rs#L1654) |
| [Mega Lopunny ex](src/import.rs#L1800) | [✅](src/import.rs#L1800) | — |
| [Mega Sharpedo ex](src/import.rs#L1835) | [✅](src/import.rs#L1835) | — |
| [Mega Skarmory ex](src/import.rs#L1913) | [✅](src/import.rs#L1913) | — |
| [Mega Slowbro ex](src/import.rs#L1953) | [✅](src/import.rs#L1953) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1725) |
| [Meowth ex](src/import.rs#L2005) | [✅](src/import.rs#L2005) | [✅](src/import.rs#L1698) |
| [Metagross](src/import.rs#L1809) | [✅](src/import.rs#L1809) | — |
| [Metang](src/import.rs#L1993) | [✅](src/import.rs#L1993) | [✅](src/import.rs#L1689) |
| [Moltres](src/import.rs#L1845) | [✅](src/import.rs#L1845) | — |
| [Munkidori](src/import.rs#L1999) | [✅](src/import.rs#L1999) | [✅](src/import.rs#L1692) |
| [N's Darmanitan](src/import.rs#L1790) | [✅](src/import.rs#L1790) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1787) | [✅](src/import.rs#L1787) | — |
| [N's Zekrom](src/import.rs#L1799) | [✅](src/import.rs#L1799) | — |
| [N's Zoroark ex](src/import.rs#L1972) | [✅](src/import.rs#L1972) | [✅](src/import.rs#L1722) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1995) | [✅](src/import.rs#L1995) | [✅](src/import.rs#L1700) |
| [Paldean Tauros](src/import.rs#L1783) | [✅](src/import.rs#L1783) | — |
| [Passimian](src/import.rs#L1796) | [✅](src/import.rs#L1796) | — |
| [Patrat](src/import.rs#L1994) | [✅](src/import.rs#L1994) | [✅](src/import.rs#L1659) |
| [Pecharunt](src/import.rs#L1936) | [✅](src/import.rs#L1936) | [✅](src/import.rs#L1726) |
| [Pecharunt ex](src/import.rs#L1933) | [✅](src/import.rs#L1933) | [✅](src/import.rs#L1763) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1660) |
| [Rabsca](src/import.rs#L1849) | [✅](src/import.rs#L1849) | [✅](src/import.rs#L1665) |
| [Raging Bolt ex](src/import.rs#L1814) | [✅](src/import.rs#L1814) | — |
| [Rellor](src/import.rs#L1781) | [✅](src/import.rs#L1781) | — |
| [Seaking](src/import.rs#L1942) | [✅](src/import.rs#L1942) | [✅](src/import.rs#L1730) |
| [Shaymin](src/import.rs#L1989) | [✅](src/import.rs#L1989) | [✅](src/import.rs#L1664) |
| [Slowking](src/import.rs#L1838) | [✅](src/import.rs#L1838) | — |
| [Slowpoke](src/import.rs#L1846) | [✅](src/import.rs#L1846) | ❌ |
| [Smoochum](src/import.rs#L1906) | [✅](src/import.rs#L1906) | — |
| [Stunfisk](src/import.rs#L1888) | [✅](src/import.rs#L1888) | — |
| [Tapu Bulu](src/import.rs#L1782) | [✅](src/import.rs#L1782) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1683) |
| [Teal Mask Ogerpon ex](src/import.rs#L2007) | [✅](src/import.rs#L2007) | [✅](src/import.rs#L1707) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1732) |
| [Torchic](src/import.rs#L1848) | [✅](src/import.rs#L1848) | — |
| [Toxel](src/import.rs#L1833) | [✅](src/import.rs#L1833) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1750) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1823) | [✅](src/import.rs#L1823) | — |
| [Yveltal](src/import.rs#L1822) | [✅](src/import.rs#L1822) | — |
| [Zeraora](src/import.rs#L1805) | [✅](src/import.rs#L1805) | — |

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

