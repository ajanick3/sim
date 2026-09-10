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
| Supporters | 44 | 78 |
| Items | 30 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 7 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L853) | ✅ |
| [Black Belt's Training](src/import.rs#L861) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L474) | ✅ |
| [Brock's Scouting](src/import.rs#L891) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L793) | ✅ |
| [Crispin](src/import.rs#L699) | ✅ |
| [Cyrano](src/import.rs#L548) | ✅ |
| [Dawn](src/import.rs#L665) | ✅ |
| [Eri](src/import.rs#L884) | ✅ |
| [Gladion's Final Battle](src/import.rs#L865) | ✅ |
| [Gwynn](src/import.rs#L562) | ✅ |
| [Hilda](src/import.rs#L619) | ✅ |
| [Janine's Secret Art](src/import.rs#L915) | ✅ |
| [Judge](src/import.rs#L497) | ✅ |
| [Kieran](src/import.rs#L869) | ✅ |
| [Lana's Aid](src/import.rs#L819) | ✅ |
| [Lillie's Determination](src/import.rs#L498) | ✅ |
| [Morty's Conviction](src/import.rs#L879) | ✅ |
| [N's Plan](src/import.rs#L837) | ✅ |
| [Rosa's Encouragement](src/import.rs#L839) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L833) | ✅ |
| [Surfer](src/import.rs#L857) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L723) | ✅ |
| [Wally's Compassion](src/import.rs#L914) | ✅ |
| [Xerosic's Machinations](src/import.rs#L883) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L534) | ✅ |
| [Bug Catching Set](src/import.rs#L779) | ✅ |
| [Crushing Hammer](src/import.rs#L533) | ✅ |
| [Dusk Ball](src/import.rs#L945) | ✅ |
| [Energy Recycler](src/import.rs#L1027) | ✅ |
| [Energy Retrieval](src/import.rs#L930) | ✅ |
| [Energy Search](src/import.rs#L916) | ✅ |
| [Energy Switch](src/import.rs#L604) | ✅ |
| [Enhanced Hammer](src/import.rs#L478) | ✅ |
| [Glass Trumpet](src/import.rs#L479) | ✅ |
| [Hand Trimmer](src/import.rs#L944) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L815) | ✅ |
| [N's PP Up](src/import.rs#L737) | ✅ |
| [Night Stretcher](src/import.rs#L505) | ✅ |
| [Prime Catcher](src/import.rs#L946) | ✅ |
| [Rare Candy](src/import.rs#L722) | ✅ |
| [Sacred Ash](src/import.rs#L576) | ✅ |
| [Secret Box](src/import.rs#L976) | ✅ |
| [Special Red Card](src/import.rs#L695) | ✅ |
| [Strange Timepiece](src/import.rs#L947) | ✅ |
| [Switch](src/import.rs#L814) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1013) | ✅ |
| [Tera Orb](src/import.rs#L605) | ✅ |
| [Tool Scrapper](src/import.rs#L473) | ✅ |
| [Transformation Tome](src/import.rs#L972) | ✅ |
| [Ultra Ball](src/import.rs#L590) | ✅ |
| [Unfair Stamp](src/import.rs#L807) | ✅ |
| [Wondrous Patch](src/import.rs#L751) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L948) | ✅ |
| [Binding Mochi](src/import.rs#L951) | ✅ |
| [Brave Bangle](src/import.rs#L950) | ✅ |
| [Handheld Fan](src/import.rs#L955) | ✅ |
| [Hero's Cape](src/import.rs#L949) | ✅ |
| [Lillie's Pearl](src/import.rs#L952) | ✅ |
| [Lucky Helmet](src/import.rs#L954) | ✅ |
| [Powerglass](src/import.rs#L956) | ✅ |
| [Punk Helmet](src/import.rs#L953) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L959) | ✅ |
| [Area Zero Underdepths](src/import.rs#L494) | ✅ |
| [Battle Cage](src/import.rs#L495) | ✅ |
| [Festival Grounds](src/import.rs#L968) | ✅ |
| [Forest of Vitality](src/import.rs#L967) | ✅ |
| [Gravity Mountain](src/import.rs#L957) | ✅ |
| [Jamming Tower](src/import.rs#L965) | ✅ |
| [Lumiose City](src/import.rs#L964) | ✅ |
| [N's Castle](src/import.rs#L958) | ✅ |
| [Nighttime Mine](src/import.rs#L493) | ✅ |
| [Risky Ruins](src/import.rs#L966) | ✅ |
| [Team Rocket's Factory](src/import.rs#L960) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L496) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1406) | ✅ |
| [Enriching Energy](src/import.rs#L1389) | ✅ |
| [Growing Grass Energy](src/import.rs#L1388) | ✅ |
| [Mist Energy](src/import.rs#L1403) | ✅ |
| [Prism Energy](src/import.rs#L1409) | ✅ |
| [Spiky Energy](src/import.rs#L1400) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1392) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1595) | [✅](src/import.rs#L1595) | [✅](src/import.rs#L1477) |
| [Alakazam](src/import.rs#L1750) | [✅](src/import.rs#L1750) | [✅](src/import.rs#L1467) |
| [Annihilape](src/import.rs#L1624) | [✅](src/import.rs#L1624) | [✅](src/import.rs#L1433) |
| [Applin](src/import.rs#L1585) | [✅](src/import.rs#L1585) | — |
| [Bayleef](src/import.rs#L1627) | [✅](src/import.rs#L1627) | — |
| [Beldum](src/import.rs#L1607) | [✅](src/import.rs#L1607) | — |
| [Blaziken ex](src/import.rs#L1691) | [✅](src/import.rs#L1691) | [✅](src/import.rs#L1507) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1655) | [✅](src/import.rs#L1655) | [✅](src/import.rs#L1441) |
| [Brute Bonnet](src/import.rs#L1566) | [✅](src/import.rs#L1566) | — |
| [Budew](src/import.rs#L1632) | [✅](src/import.rs#L1632) | — |
| [Buneary](src/import.rs#L1626) | [✅](src/import.rs#L1626) | — |
| [Carvanha](src/import.rs#L1544) | [✅](src/import.rs#L1544) | — |
| [Celebi](src/import.rs#L1625) | [✅](src/import.rs#L1625) | — |
| [Chi-Yu](src/import.rs#L1723) | [✅](src/import.rs#L1723) | — |
| [Chien-Pao](src/import.rs#L1692) | [✅](src/import.rs#L1692) | [✅](src/import.rs#L1510) |
| [Chikorita](src/import.rs#L1628) | [✅](src/import.rs#L1628) | — |
| [Cofagrigus](src/import.rs#L1673) | [✅](src/import.rs#L1673) | — |
| [Combusken](src/import.rs#L1642) | [✅](src/import.rs#L1642) | — |
| [Crustle](src/import.rs#L1762) | [✅](src/import.rs#L1762) | [✅](src/import.rs#L1422) |
| [Dedenne](src/import.rs#L1582) | [✅](src/import.rs#L1582) | — |
| [Dipplin](src/import.rs#L1707) | [✅](src/import.rs#L1707) | [✅](src/import.rs#L1495) |
| [Dragapult ex](src/import.rs#L1589) | [✅](src/import.rs#L1589) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1450) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1596) | [✅](src/import.rs#L1596) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1474) |
| [Dudunsparce ex](src/import.rs#L1557) | [✅](src/import.rs#L1557) | — |
| [Dunsparce](src/import.rs#L1608) | [✅](src/import.rs#L1608) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1478) |
| [Dusknoir](src/import.rs#L1689) | [✅](src/import.rs#L1689) | [✅](src/import.rs#L1479) |
| [Duskull](src/import.rs#L1611) | [✅](src/import.rs#L1611) | — |
| [Dwebble](src/import.rs#L1601) | [✅](src/import.rs#L1601) | — |
| [Elgyem](src/import.rs#L1631) | [✅](src/import.rs#L1631) | — |
| [Enamorus](src/import.rs#L1651) | [✅](src/import.rs#L1651) | — |
| [Fan Rotom](src/import.rs#L1696) | [✅](src/import.rs#L1696) | [✅](src/import.rs#L1520) |
| [Fezandipiti ex](src/import.rs#L1770) | [✅](src/import.rs#L1770) | [✅](src/import.rs#L1468) |
| [Flutter Mane](src/import.rs#L1687) | [✅](src/import.rs#L1687) | [✅](src/import.rs#L1444) |
| [Genesect](src/import.rs#L1742) | [✅](src/import.rs#L1742) | [✅](src/import.rs#L1483) |
| [Genesect ex](src/import.rs#L1690) | [✅](src/import.rs#L1690) | [✅](src/import.rs#L1480) |
| [Goldeen](src/import.rs#L1705) | [✅](src/import.rs#L1705) | [✅](src/import.rs#L1493) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1598) | [✅](src/import.rs#L1598) | [✅](src/import.rs#L1434) |
| [Hydrapple ex](src/import.rs#L1656) | [✅](src/import.rs#L1656) | [✅](src/import.rs#L1435) |
| [Iron Crown ex](src/import.rs#L1616) | [✅](src/import.rs#L1616) | [✅](src/import.rs#L1430) |
| [Iron Leaves ex](src/import.rs#L1695) | [✅](src/import.rs#L1695) | [✅](src/import.rs#L1511) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1463) |
| [Koraidon ex](src/import.rs#L1617) | [✅](src/import.rs#L1617) | — |
| [Kyurem](src/import.rs#L1739) | [✅](src/import.rs#L1739) | [✅](src/import.rs#L1502) |
| [Latias ex](src/import.rs#L1756) | [✅](src/import.rs#L1756) | [✅](src/import.rs#L1421) |
| [Lillie's Clefairy ex](src/import.rs#L1780) | [✅](src/import.rs#L1780) | [✅](src/import.rs#L1425) |
| [Mega Absol ex](src/import.rs#L1663) | [✅](src/import.rs#L1663) | — |
| [Mega Excadrill ex](src/import.rs#L1659) | [✅](src/import.rs#L1659) | — |
| [Mega Kangaskhan ex](src/import.rs#L1766) | [✅](src/import.rs#L1766) | [✅](src/import.rs#L1418) |
| [Mega Lopunny ex](src/import.rs#L1564) | [✅](src/import.rs#L1564) | — |
| [Mega Sharpedo ex](src/import.rs#L1599) | [✅](src/import.rs#L1599) | — |
| [Mega Skarmory ex](src/import.rs#L1677) | [✅](src/import.rs#L1677) | — |
| [Mega Slowbro ex](src/import.rs#L1717) | [✅](src/import.rs#L1717) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1489) |
| [Meowth ex](src/import.rs#L1769) | [✅](src/import.rs#L1769) | [✅](src/import.rs#L1462) |
| [Metagross](src/import.rs#L1573) | [✅](src/import.rs#L1573) | — |
| [Metang](src/import.rs#L1757) | [✅](src/import.rs#L1757) | [✅](src/import.rs#L1453) |
| [Moltres](src/import.rs#L1609) | [✅](src/import.rs#L1609) | — |
| [Munkidori](src/import.rs#L1763) | [✅](src/import.rs#L1763) | [✅](src/import.rs#L1456) |
| [N's Darmanitan](src/import.rs#L1554) | [✅](src/import.rs#L1554) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1551) | [✅](src/import.rs#L1551) | — |
| [N's Zekrom](src/import.rs#L1563) | [✅](src/import.rs#L1563) | — |
| [N's Zoroark ex](src/import.rs#L1736) | [✅](src/import.rs#L1736) | [✅](src/import.rs#L1486) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1759) | [✅](src/import.rs#L1759) | [✅](src/import.rs#L1464) |
| [Paldean Tauros](src/import.rs#L1547) | [✅](src/import.rs#L1547) | — |
| [Passimian](src/import.rs#L1560) | [✅](src/import.rs#L1560) | — |
| [Patrat](src/import.rs#L1758) | [✅](src/import.rs#L1758) | [✅](src/import.rs#L1423) |
| [Pecharunt](src/import.rs#L1700) | [✅](src/import.rs#L1700) | [✅](src/import.rs#L1490) |
| [Pecharunt ex](src/import.rs#L1697) | [✅](src/import.rs#L1697) | [✅](src/import.rs#L1527) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1424) |
| [Rabsca](src/import.rs#L1613) | [✅](src/import.rs#L1613) | [✅](src/import.rs#L1429) |
| [Raging Bolt ex](src/import.rs#L1578) | [✅](src/import.rs#L1578) | — |
| [Rellor](src/import.rs#L1545) | [✅](src/import.rs#L1545) | — |
| [Seaking](src/import.rs#L1706) | [✅](src/import.rs#L1706) | [✅](src/import.rs#L1494) |
| [Shaymin](src/import.rs#L1753) | [✅](src/import.rs#L1753) | [✅](src/import.rs#L1428) |
| [Slowking](src/import.rs#L1602) | [✅](src/import.rs#L1602) | — |
| [Slowpoke](src/import.rs#L1610) | [✅](src/import.rs#L1610) | ❌ |
| [Smoochum](src/import.rs#L1670) | [✅](src/import.rs#L1670) | — |
| [Stunfisk](src/import.rs#L1652) | [✅](src/import.rs#L1652) | — |
| [Tapu Bulu](src/import.rs#L1546) | [✅](src/import.rs#L1546) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1447) |
| [Teal Mask Ogerpon ex](src/import.rs#L1771) | [✅](src/import.rs#L1771) | [✅](src/import.rs#L1471) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1496) |
| [Torchic](src/import.rs#L1612) | [✅](src/import.rs#L1612) | — |
| [Toxel](src/import.rs#L1597) | [✅](src/import.rs#L1597) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1514) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1587) | [✅](src/import.rs#L1587) | — |
| [Yveltal](src/import.rs#L1586) | [✅](src/import.rs#L1586) | — |
| [Zeraora](src/import.rs#L1569) | [✅](src/import.rs#L1569) | — |

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

