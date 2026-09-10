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
| Items | 32 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L938) | ✅ |
| [Black Belt's Training](src/import.rs#L946) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L559) | ✅ |
| [Brock's Scouting](src/import.rs#L976) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L878) | ✅ |
| [Crispin](src/import.rs#L784) | ✅ |
| [Cyrano](src/import.rs#L633) | ✅ |
| [Dawn](src/import.rs#L750) | ✅ |
| [Eri](src/import.rs#L969) | ✅ |
| [Gladion's Final Battle](src/import.rs#L950) | ✅ |
| [Gwynn](src/import.rs#L647) | ✅ |
| [Hilda](src/import.rs#L704) | ✅ |
| [Janine's Secret Art](src/import.rs#L1000) | ✅ |
| [Judge](src/import.rs#L582) | ✅ |
| [Kieran](src/import.rs#L954) | ✅ |
| [Lana's Aid](src/import.rs#L904) | ✅ |
| [Lillie's Determination](src/import.rs#L583) | ✅ |
| [Morty's Conviction](src/import.rs#L964) | ✅ |
| [N's Plan](src/import.rs#L922) | ✅ |
| [Rosa's Encouragement](src/import.rs#L924) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L918) | ✅ |
| [Surfer](src/import.rs#L942) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L808) | ✅ |
| [Wally's Compassion](src/import.rs#L999) | ✅ |
| [Xerosic's Machinations](src/import.rs#L968) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L619) | ✅ |
| [Bug Catching Set](src/import.rs#L864) | ✅ |
| [Crushing Hammer](src/import.rs#L618) | ✅ |
| [Dusk Ball](src/import.rs#L1030) | ✅ |
| [Energy Recycler](src/import.rs#L1112) | ✅ |
| [Energy Retrieval](src/import.rs#L1015) | ✅ |
| [Energy Search](src/import.rs#L1001) | ✅ |
| [Energy Switch](src/import.rs#L689) | ✅ |
| [Enhanced Hammer](src/import.rs#L563) | ✅ |
| [Glass Trumpet](src/import.rs#L564) | ✅ |
| [Hand Trimmer](src/import.rs#L1029) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L900) | ✅ |
| [N's PP Up](src/import.rs#L822) | ✅ |
| [Night Stretcher](src/import.rs#L590) | ✅ |
| [Prime Catcher](src/import.rs#L1031) | ✅ |
| [Rare Candy](src/import.rs#L807) | ✅ |
| [Sacred Ash](src/import.rs#L661) | ✅ |
| [Secret Box](src/import.rs#L1061) | ✅ |
| [Special Red Card](src/import.rs#L780) | ✅ |
| [Strange Timepiece](src/import.rs#L1032) | ✅ |
| [Switch](src/import.rs#L899) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1098) | ✅ |
| [Tera Orb](src/import.rs#L690) | ✅ |
| [Tool Scrapper](src/import.rs#L558) | ✅ |
| [Transformation Tome](src/import.rs#L1057) | ✅ |
| [Ultra Ball](src/import.rs#L675) | ✅ |
| [Unfair Stamp](src/import.rs#L892) | ✅ |
| [Wondrous Patch](src/import.rs#L836) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1033) | ✅ |
| [Binding Mochi](src/import.rs#L1036) | ✅ |
| [Brave Bangle](src/import.rs#L1035) | ✅ |
| [Handheld Fan](src/import.rs#L1040) | ✅ |
| [Hero's Cape](src/import.rs#L1034) | ✅ |
| [Lillie's Pearl](src/import.rs#L1037) | ✅ |
| [Lucky Helmet](src/import.rs#L1039) | ✅ |
| [Powerglass](src/import.rs#L1041) | ✅ |
| [Punk Helmet](src/import.rs#L1038) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1044) | ✅ |
| [Area Zero Underdepths](src/import.rs#L579) | ✅ |
| [Battle Cage](src/import.rs#L580) | ✅ |
| [Festival Grounds](src/import.rs#L1053) | ✅ |
| [Forest of Vitality](src/import.rs#L1052) | ✅ |
| [Gravity Mountain](src/import.rs#L1042) | ✅ |
| [Jamming Tower](src/import.rs#L1050) | ✅ |
| [Lumiose City](src/import.rs#L1049) | ✅ |
| [N's Castle](src/import.rs#L1043) | ✅ |
| [Nighttime Mine](src/import.rs#L578) | ✅ |
| [Risky Ruins](src/import.rs#L1051) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1045) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L581) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1491) | ✅ |
| [Enriching Energy](src/import.rs#L1474) | ✅ |
| [Growing Grass Energy](src/import.rs#L1473) | ✅ |
| [Mist Energy](src/import.rs#L1488) | ✅ |
| [Prism Energy](src/import.rs#L1494) | ✅ |
| [Spiky Energy](src/import.rs#L1485) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1477) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1689) | [✅](src/import.rs#L1689) | [✅](src/import.rs#L1571) |
| [Alakazam](src/import.rs#L1844) | [✅](src/import.rs#L1844) | [✅](src/import.rs#L1561) |
| [Annihilape](src/import.rs#L1718) | [✅](src/import.rs#L1718) | [✅](src/import.rs#L1527) |
| [Applin](src/import.rs#L1679) | [✅](src/import.rs#L1679) | — |
| [Bayleef](src/import.rs#L1721) | [✅](src/import.rs#L1721) | — |
| [Beldum](src/import.rs#L1701) | [✅](src/import.rs#L1701) | — |
| [Blaziken ex](src/import.rs#L1785) | [✅](src/import.rs#L1785) | [✅](src/import.rs#L1601) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1749) | [✅](src/import.rs#L1749) | [✅](src/import.rs#L1535) |
| [Brute Bonnet](src/import.rs#L1660) | [✅](src/import.rs#L1660) | — |
| [Budew](src/import.rs#L1726) | [✅](src/import.rs#L1726) | — |
| [Buneary](src/import.rs#L1720) | [✅](src/import.rs#L1720) | — |
| [Carvanha](src/import.rs#L1638) | [✅](src/import.rs#L1638) | — |
| [Celebi](src/import.rs#L1719) | [✅](src/import.rs#L1719) | — |
| [Chi-Yu](src/import.rs#L1817) | [✅](src/import.rs#L1817) | — |
| [Chien-Pao](src/import.rs#L1786) | [✅](src/import.rs#L1786) | [✅](src/import.rs#L1604) |
| [Chikorita](src/import.rs#L1722) | [✅](src/import.rs#L1722) | — |
| [Cofagrigus](src/import.rs#L1767) | [✅](src/import.rs#L1767) | — |
| [Combusken](src/import.rs#L1736) | [✅](src/import.rs#L1736) | — |
| [Crustle](src/import.rs#L1856) | [✅](src/import.rs#L1856) | [✅](src/import.rs#L1516) |
| [Dedenne](src/import.rs#L1676) | [✅](src/import.rs#L1676) | — |
| [Dipplin](src/import.rs#L1801) | [✅](src/import.rs#L1801) | [✅](src/import.rs#L1589) |
| [Dragapult ex](src/import.rs#L1683) | [✅](src/import.rs#L1683) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1544) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1690) | [✅](src/import.rs#L1690) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1568) |
| [Dudunsparce ex](src/import.rs#L1651) | [✅](src/import.rs#L1651) | — |
| [Dunsparce](src/import.rs#L1702) | [✅](src/import.rs#L1702) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1572) |
| [Dusknoir](src/import.rs#L1783) | [✅](src/import.rs#L1783) | [✅](src/import.rs#L1573) |
| [Duskull](src/import.rs#L1705) | [✅](src/import.rs#L1705) | — |
| [Dwebble](src/import.rs#L1695) | [✅](src/import.rs#L1695) | — |
| [Elgyem](src/import.rs#L1725) | [✅](src/import.rs#L1725) | — |
| [Enamorus](src/import.rs#L1745) | [✅](src/import.rs#L1745) | — |
| [Fan Rotom](src/import.rs#L1790) | [✅](src/import.rs#L1790) | [✅](src/import.rs#L1614) |
| [Fezandipiti ex](src/import.rs#L1864) | [✅](src/import.rs#L1864) | [✅](src/import.rs#L1562) |
| [Flutter Mane](src/import.rs#L1781) | [✅](src/import.rs#L1781) | [✅](src/import.rs#L1538) |
| [Genesect](src/import.rs#L1836) | [✅](src/import.rs#L1836) | [✅](src/import.rs#L1577) |
| [Genesect ex](src/import.rs#L1784) | [✅](src/import.rs#L1784) | [✅](src/import.rs#L1574) |
| [Goldeen](src/import.rs#L1799) | [✅](src/import.rs#L1799) | [✅](src/import.rs#L1587) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1692) | [✅](src/import.rs#L1692) | [✅](src/import.rs#L1528) |
| [Hydrapple ex](src/import.rs#L1750) | [✅](src/import.rs#L1750) | [✅](src/import.rs#L1529) |
| [Iron Crown ex](src/import.rs#L1710) | [✅](src/import.rs#L1710) | [✅](src/import.rs#L1524) |
| [Iron Leaves ex](src/import.rs#L1789) | [✅](src/import.rs#L1789) | [✅](src/import.rs#L1605) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1557) |
| [Koraidon ex](src/import.rs#L1711) | [✅](src/import.rs#L1711) | — |
| [Kyurem](src/import.rs#L1833) | [✅](src/import.rs#L1833) | [✅](src/import.rs#L1596) |
| [Latias ex](src/import.rs#L1850) | [✅](src/import.rs#L1850) | [✅](src/import.rs#L1515) |
| [Lillie's Clefairy ex](src/import.rs#L1874) | [✅](src/import.rs#L1874) | [✅](src/import.rs#L1519) |
| [Mega Absol ex](src/import.rs#L1757) | [✅](src/import.rs#L1757) | — |
| [Mega Excadrill ex](src/import.rs#L1753) | [✅](src/import.rs#L1753) | — |
| [Mega Kangaskhan ex](src/import.rs#L1860) | [✅](src/import.rs#L1860) | [✅](src/import.rs#L1512) |
| [Mega Lopunny ex](src/import.rs#L1658) | [✅](src/import.rs#L1658) | — |
| [Mega Sharpedo ex](src/import.rs#L1693) | [✅](src/import.rs#L1693) | — |
| [Mega Skarmory ex](src/import.rs#L1771) | [✅](src/import.rs#L1771) | — |
| [Mega Slowbro ex](src/import.rs#L1811) | [✅](src/import.rs#L1811) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1583) |
| [Meowth ex](src/import.rs#L1863) | [✅](src/import.rs#L1863) | [✅](src/import.rs#L1556) |
| [Metagross](src/import.rs#L1667) | [✅](src/import.rs#L1667) | — |
| [Metang](src/import.rs#L1851) | [✅](src/import.rs#L1851) | [✅](src/import.rs#L1547) |
| [Moltres](src/import.rs#L1703) | [✅](src/import.rs#L1703) | — |
| [Munkidori](src/import.rs#L1857) | [✅](src/import.rs#L1857) | [✅](src/import.rs#L1550) |
| [N's Darmanitan](src/import.rs#L1648) | [✅](src/import.rs#L1648) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1645) | [✅](src/import.rs#L1645) | — |
| [N's Zekrom](src/import.rs#L1657) | [✅](src/import.rs#L1657) | — |
| [N's Zoroark ex](src/import.rs#L1830) | [✅](src/import.rs#L1830) | [✅](src/import.rs#L1580) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1853) | [✅](src/import.rs#L1853) | [✅](src/import.rs#L1558) |
| [Paldean Tauros](src/import.rs#L1641) | [✅](src/import.rs#L1641) | — |
| [Passimian](src/import.rs#L1654) | [✅](src/import.rs#L1654) | — |
| [Patrat](src/import.rs#L1852) | [✅](src/import.rs#L1852) | [✅](src/import.rs#L1517) |
| [Pecharunt](src/import.rs#L1794) | [✅](src/import.rs#L1794) | [✅](src/import.rs#L1584) |
| [Pecharunt ex](src/import.rs#L1791) | [✅](src/import.rs#L1791) | [✅](src/import.rs#L1621) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1518) |
| [Rabsca](src/import.rs#L1707) | [✅](src/import.rs#L1707) | [✅](src/import.rs#L1523) |
| [Raging Bolt ex](src/import.rs#L1672) | [✅](src/import.rs#L1672) | — |
| [Rellor](src/import.rs#L1639) | [✅](src/import.rs#L1639) | — |
| [Seaking](src/import.rs#L1800) | [✅](src/import.rs#L1800) | [✅](src/import.rs#L1588) |
| [Shaymin](src/import.rs#L1847) | [✅](src/import.rs#L1847) | [✅](src/import.rs#L1522) |
| [Slowking](src/import.rs#L1696) | [✅](src/import.rs#L1696) | — |
| [Slowpoke](src/import.rs#L1704) | [✅](src/import.rs#L1704) | ❌ |
| [Smoochum](src/import.rs#L1764) | [✅](src/import.rs#L1764) | — |
| [Stunfisk](src/import.rs#L1746) | [✅](src/import.rs#L1746) | — |
| [Tapu Bulu](src/import.rs#L1640) | [✅](src/import.rs#L1640) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1541) |
| [Teal Mask Ogerpon ex](src/import.rs#L1865) | [✅](src/import.rs#L1865) | [✅](src/import.rs#L1565) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1590) |
| [Torchic](src/import.rs#L1706) | [✅](src/import.rs#L1706) | — |
| [Toxel](src/import.rs#L1691) | [✅](src/import.rs#L1691) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1608) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1681) | [✅](src/import.rs#L1681) | — |
| [Yveltal](src/import.rs#L1680) | [✅](src/import.rs#L1680) | — |
| [Zeraora](src/import.rs#L1663) | [✅](src/import.rs#L1663) | — |

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

