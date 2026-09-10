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
| Supporters | 50 | 78 |
| Items | 30 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 7 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L886) | ✅ |
| [Black Belt's Training](src/import.rs#L894) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L507) | ✅ |
| [Brock's Scouting](src/import.rs#L924) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L826) | ✅ |
| [Crispin](src/import.rs#L732) | ✅ |
| [Cyrano](src/import.rs#L581) | ✅ |
| [Dawn](src/import.rs#L698) | ✅ |
| [Eri](src/import.rs#L917) | ✅ |
| [Gladion's Final Battle](src/import.rs#L898) | ✅ |
| [Gwynn](src/import.rs#L595) | ✅ |
| [Hilda](src/import.rs#L652) | ✅ |
| [Janine's Secret Art](src/import.rs#L948) | ✅ |
| [Judge](src/import.rs#L530) | ✅ |
| [Kieran](src/import.rs#L902) | ✅ |
| [Lana's Aid](src/import.rs#L852) | ✅ |
| [Lillie's Determination](src/import.rs#L531) | ✅ |
| [Morty's Conviction](src/import.rs#L912) | ✅ |
| [N's Plan](src/import.rs#L870) | ✅ |
| [Rosa's Encouragement](src/import.rs#L872) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L866) | ✅ |
| [Surfer](src/import.rs#L890) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L756) | ✅ |
| [Wally's Compassion](src/import.rs#L947) | ✅ |
| [Xerosic's Machinations](src/import.rs#L916) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L567) | ✅ |
| [Bug Catching Set](src/import.rs#L812) | ✅ |
| [Crushing Hammer](src/import.rs#L566) | ✅ |
| [Dusk Ball](src/import.rs#L978) | ✅ |
| [Energy Recycler](src/import.rs#L1060) | ✅ |
| [Energy Retrieval](src/import.rs#L963) | ✅ |
| [Energy Search](src/import.rs#L949) | ✅ |
| [Energy Switch](src/import.rs#L637) | ✅ |
| [Enhanced Hammer](src/import.rs#L511) | ✅ |
| [Glass Trumpet](src/import.rs#L512) | ✅ |
| [Hand Trimmer](src/import.rs#L977) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L848) | ✅ |
| [N's PP Up](src/import.rs#L770) | ✅ |
| [Night Stretcher](src/import.rs#L538) | ✅ |
| [Prime Catcher](src/import.rs#L979) | ✅ |
| [Rare Candy](src/import.rs#L755) | ✅ |
| [Sacred Ash](src/import.rs#L609) | ✅ |
| [Secret Box](src/import.rs#L1009) | ✅ |
| [Special Red Card](src/import.rs#L728) | ✅ |
| [Strange Timepiece](src/import.rs#L980) | ✅ |
| [Switch](src/import.rs#L847) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1046) | ✅ |
| [Tera Orb](src/import.rs#L638) | ✅ |
| [Tool Scrapper](src/import.rs#L506) | ✅ |
| [Transformation Tome](src/import.rs#L1005) | ✅ |
| [Ultra Ball](src/import.rs#L623) | ✅ |
| [Unfair Stamp](src/import.rs#L840) | ✅ |
| [Wondrous Patch](src/import.rs#L784) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L981) | ✅ |
| [Binding Mochi](src/import.rs#L984) | ✅ |
| [Brave Bangle](src/import.rs#L983) | ✅ |
| [Handheld Fan](src/import.rs#L988) | ✅ |
| [Hero's Cape](src/import.rs#L982) | ✅ |
| [Lillie's Pearl](src/import.rs#L985) | ✅ |
| [Lucky Helmet](src/import.rs#L987) | ✅ |
| [Powerglass](src/import.rs#L989) | ✅ |
| [Punk Helmet](src/import.rs#L986) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L992) | ✅ |
| [Area Zero Underdepths](src/import.rs#L527) | ✅ |
| [Battle Cage](src/import.rs#L528) | ✅ |
| [Festival Grounds](src/import.rs#L1001) | ✅ |
| [Forest of Vitality](src/import.rs#L1000) | ✅ |
| [Gravity Mountain](src/import.rs#L990) | ✅ |
| [Jamming Tower](src/import.rs#L998) | ✅ |
| [Lumiose City](src/import.rs#L997) | ✅ |
| [N's Castle](src/import.rs#L991) | ✅ |
| [Nighttime Mine](src/import.rs#L526) | ✅ |
| [Risky Ruins](src/import.rs#L999) | ✅ |
| [Team Rocket's Factory](src/import.rs#L993) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L529) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1439) | ✅ |
| [Enriching Energy](src/import.rs#L1422) | ✅ |
| [Growing Grass Energy](src/import.rs#L1421) | ✅ |
| [Mist Energy](src/import.rs#L1436) | ✅ |
| [Prism Energy](src/import.rs#L1442) | ✅ |
| [Spiky Energy](src/import.rs#L1433) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1425) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1628) | [✅](src/import.rs#L1628) | [✅](src/import.rs#L1510) |
| [Alakazam](src/import.rs#L1783) | [✅](src/import.rs#L1783) | [✅](src/import.rs#L1500) |
| [Annihilape](src/import.rs#L1657) | [✅](src/import.rs#L1657) | [✅](src/import.rs#L1466) |
| [Applin](src/import.rs#L1618) | [✅](src/import.rs#L1618) | — |
| [Bayleef](src/import.rs#L1660) | [✅](src/import.rs#L1660) | — |
| [Beldum](src/import.rs#L1640) | [✅](src/import.rs#L1640) | — |
| [Blaziken ex](src/import.rs#L1724) | [✅](src/import.rs#L1724) | [✅](src/import.rs#L1540) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1688) | [✅](src/import.rs#L1688) | [✅](src/import.rs#L1474) |
| [Brute Bonnet](src/import.rs#L1599) | [✅](src/import.rs#L1599) | — |
| [Budew](src/import.rs#L1665) | [✅](src/import.rs#L1665) | — |
| [Buneary](src/import.rs#L1659) | [✅](src/import.rs#L1659) | — |
| [Carvanha](src/import.rs#L1577) | [✅](src/import.rs#L1577) | — |
| [Celebi](src/import.rs#L1658) | [✅](src/import.rs#L1658) | — |
| [Chi-Yu](src/import.rs#L1756) | [✅](src/import.rs#L1756) | — |
| [Chien-Pao](src/import.rs#L1725) | [✅](src/import.rs#L1725) | [✅](src/import.rs#L1543) |
| [Chikorita](src/import.rs#L1661) | [✅](src/import.rs#L1661) | — |
| [Cofagrigus](src/import.rs#L1706) | [✅](src/import.rs#L1706) | — |
| [Combusken](src/import.rs#L1675) | [✅](src/import.rs#L1675) | — |
| [Crustle](src/import.rs#L1795) | [✅](src/import.rs#L1795) | [✅](src/import.rs#L1455) |
| [Dedenne](src/import.rs#L1615) | [✅](src/import.rs#L1615) | — |
| [Dipplin](src/import.rs#L1740) | [✅](src/import.rs#L1740) | [✅](src/import.rs#L1528) |
| [Dragapult ex](src/import.rs#L1622) | [✅](src/import.rs#L1622) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1483) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1629) | [✅](src/import.rs#L1629) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1507) |
| [Dudunsparce ex](src/import.rs#L1590) | [✅](src/import.rs#L1590) | — |
| [Dunsparce](src/import.rs#L1641) | [✅](src/import.rs#L1641) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1511) |
| [Dusknoir](src/import.rs#L1722) | [✅](src/import.rs#L1722) | [✅](src/import.rs#L1512) |
| [Duskull](src/import.rs#L1644) | [✅](src/import.rs#L1644) | — |
| [Dwebble](src/import.rs#L1634) | [✅](src/import.rs#L1634) | — |
| [Elgyem](src/import.rs#L1664) | [✅](src/import.rs#L1664) | — |
| [Enamorus](src/import.rs#L1684) | [✅](src/import.rs#L1684) | — |
| [Fan Rotom](src/import.rs#L1729) | [✅](src/import.rs#L1729) | [✅](src/import.rs#L1553) |
| [Fezandipiti ex](src/import.rs#L1803) | [✅](src/import.rs#L1803) | [✅](src/import.rs#L1501) |
| [Flutter Mane](src/import.rs#L1720) | [✅](src/import.rs#L1720) | [✅](src/import.rs#L1477) |
| [Genesect](src/import.rs#L1775) | [✅](src/import.rs#L1775) | [✅](src/import.rs#L1516) |
| [Genesect ex](src/import.rs#L1723) | [✅](src/import.rs#L1723) | [✅](src/import.rs#L1513) |
| [Goldeen](src/import.rs#L1738) | [✅](src/import.rs#L1738) | [✅](src/import.rs#L1526) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1631) | [✅](src/import.rs#L1631) | [✅](src/import.rs#L1467) |
| [Hydrapple ex](src/import.rs#L1689) | [✅](src/import.rs#L1689) | [✅](src/import.rs#L1468) |
| [Iron Crown ex](src/import.rs#L1649) | [✅](src/import.rs#L1649) | [✅](src/import.rs#L1463) |
| [Iron Leaves ex](src/import.rs#L1728) | [✅](src/import.rs#L1728) | [✅](src/import.rs#L1544) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1496) |
| [Koraidon ex](src/import.rs#L1650) | [✅](src/import.rs#L1650) | — |
| [Kyurem](src/import.rs#L1772) | [✅](src/import.rs#L1772) | [✅](src/import.rs#L1535) |
| [Latias ex](src/import.rs#L1789) | [✅](src/import.rs#L1789) | [✅](src/import.rs#L1454) |
| [Lillie's Clefairy ex](src/import.rs#L1813) | [✅](src/import.rs#L1813) | [✅](src/import.rs#L1458) |
| [Mega Absol ex](src/import.rs#L1696) | [✅](src/import.rs#L1696) | — |
| [Mega Excadrill ex](src/import.rs#L1692) | [✅](src/import.rs#L1692) | — |
| [Mega Kangaskhan ex](src/import.rs#L1799) | [✅](src/import.rs#L1799) | [✅](src/import.rs#L1451) |
| [Mega Lopunny ex](src/import.rs#L1597) | [✅](src/import.rs#L1597) | — |
| [Mega Sharpedo ex](src/import.rs#L1632) | [✅](src/import.rs#L1632) | — |
| [Mega Skarmory ex](src/import.rs#L1710) | [✅](src/import.rs#L1710) | — |
| [Mega Slowbro ex](src/import.rs#L1750) | [✅](src/import.rs#L1750) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1522) |
| [Meowth ex](src/import.rs#L1802) | [✅](src/import.rs#L1802) | [✅](src/import.rs#L1495) |
| [Metagross](src/import.rs#L1606) | [✅](src/import.rs#L1606) | — |
| [Metang](src/import.rs#L1790) | [✅](src/import.rs#L1790) | [✅](src/import.rs#L1486) |
| [Moltres](src/import.rs#L1642) | [✅](src/import.rs#L1642) | — |
| [Munkidori](src/import.rs#L1796) | [✅](src/import.rs#L1796) | [✅](src/import.rs#L1489) |
| [N's Darmanitan](src/import.rs#L1587) | [✅](src/import.rs#L1587) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1584) | [✅](src/import.rs#L1584) | — |
| [N's Zekrom](src/import.rs#L1596) | [✅](src/import.rs#L1596) | — |
| [N's Zoroark ex](src/import.rs#L1769) | [✅](src/import.rs#L1769) | [✅](src/import.rs#L1519) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1792) | [✅](src/import.rs#L1792) | [✅](src/import.rs#L1497) |
| [Paldean Tauros](src/import.rs#L1580) | [✅](src/import.rs#L1580) | — |
| [Passimian](src/import.rs#L1593) | [✅](src/import.rs#L1593) | — |
| [Patrat](src/import.rs#L1791) | [✅](src/import.rs#L1791) | [✅](src/import.rs#L1456) |
| [Pecharunt](src/import.rs#L1733) | [✅](src/import.rs#L1733) | [✅](src/import.rs#L1523) |
| [Pecharunt ex](src/import.rs#L1730) | [✅](src/import.rs#L1730) | [✅](src/import.rs#L1560) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1457) |
| [Rabsca](src/import.rs#L1646) | [✅](src/import.rs#L1646) | [✅](src/import.rs#L1462) |
| [Raging Bolt ex](src/import.rs#L1611) | [✅](src/import.rs#L1611) | — |
| [Rellor](src/import.rs#L1578) | [✅](src/import.rs#L1578) | — |
| [Seaking](src/import.rs#L1739) | [✅](src/import.rs#L1739) | [✅](src/import.rs#L1527) |
| [Shaymin](src/import.rs#L1786) | [✅](src/import.rs#L1786) | [✅](src/import.rs#L1461) |
| [Slowking](src/import.rs#L1635) | [✅](src/import.rs#L1635) | — |
| [Slowpoke](src/import.rs#L1643) | [✅](src/import.rs#L1643) | ❌ |
| [Smoochum](src/import.rs#L1703) | [✅](src/import.rs#L1703) | — |
| [Stunfisk](src/import.rs#L1685) | [✅](src/import.rs#L1685) | — |
| [Tapu Bulu](src/import.rs#L1579) | [✅](src/import.rs#L1579) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1480) |
| [Teal Mask Ogerpon ex](src/import.rs#L1804) | [✅](src/import.rs#L1804) | [✅](src/import.rs#L1504) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1529) |
| [Torchic](src/import.rs#L1645) | [✅](src/import.rs#L1645) | — |
| [Toxel](src/import.rs#L1630) | [✅](src/import.rs#L1630) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1547) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1620) | [✅](src/import.rs#L1620) | — |
| [Yveltal](src/import.rs#L1619) | [✅](src/import.rs#L1619) | — |
| [Zeraora](src/import.rs#L1602) | [✅](src/import.rs#L1602) | — |

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

