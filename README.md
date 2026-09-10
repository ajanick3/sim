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
| Supporters | 47 | 78 |
| Items | 30 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 7 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L870) | ✅ |
| [Black Belt's Training](src/import.rs#L878) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L491) | ✅ |
| [Brock's Scouting](src/import.rs#L908) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L810) | ✅ |
| [Crispin](src/import.rs#L716) | ✅ |
| [Cyrano](src/import.rs#L565) | ✅ |
| [Dawn](src/import.rs#L682) | ✅ |
| [Eri](src/import.rs#L901) | ✅ |
| [Gladion's Final Battle](src/import.rs#L882) | ✅ |
| [Gwynn](src/import.rs#L579) | ✅ |
| [Hilda](src/import.rs#L636) | ✅ |
| [Janine's Secret Art](src/import.rs#L932) | ✅ |
| [Judge](src/import.rs#L514) | ✅ |
| [Kieran](src/import.rs#L886) | ✅ |
| [Lana's Aid](src/import.rs#L836) | ✅ |
| [Lillie's Determination](src/import.rs#L515) | ✅ |
| [Morty's Conviction](src/import.rs#L896) | ✅ |
| [N's Plan](src/import.rs#L854) | ✅ |
| [Rosa's Encouragement](src/import.rs#L856) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L850) | ✅ |
| [Surfer](src/import.rs#L874) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L740) | ✅ |
| [Wally's Compassion](src/import.rs#L931) | ✅ |
| [Xerosic's Machinations](src/import.rs#L900) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L551) | ✅ |
| [Bug Catching Set](src/import.rs#L796) | ✅ |
| [Crushing Hammer](src/import.rs#L550) | ✅ |
| [Dusk Ball](src/import.rs#L962) | ✅ |
| [Energy Recycler](src/import.rs#L1044) | ✅ |
| [Energy Retrieval](src/import.rs#L947) | ✅ |
| [Energy Search](src/import.rs#L933) | ✅ |
| [Energy Switch](src/import.rs#L621) | ✅ |
| [Enhanced Hammer](src/import.rs#L495) | ✅ |
| [Glass Trumpet](src/import.rs#L496) | ✅ |
| [Hand Trimmer](src/import.rs#L961) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L832) | ✅ |
| [N's PP Up](src/import.rs#L754) | ✅ |
| [Night Stretcher](src/import.rs#L522) | ✅ |
| [Prime Catcher](src/import.rs#L963) | ✅ |
| [Rare Candy](src/import.rs#L739) | ✅ |
| [Sacred Ash](src/import.rs#L593) | ✅ |
| [Secret Box](src/import.rs#L993) | ✅ |
| [Special Red Card](src/import.rs#L712) | ✅ |
| [Strange Timepiece](src/import.rs#L964) | ✅ |
| [Switch](src/import.rs#L831) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1030) | ✅ |
| [Tera Orb](src/import.rs#L622) | ✅ |
| [Tool Scrapper](src/import.rs#L490) | ✅ |
| [Transformation Tome](src/import.rs#L989) | ✅ |
| [Ultra Ball](src/import.rs#L607) | ✅ |
| [Unfair Stamp](src/import.rs#L824) | ✅ |
| [Wondrous Patch](src/import.rs#L768) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L965) | ✅ |
| [Binding Mochi](src/import.rs#L968) | ✅ |
| [Brave Bangle](src/import.rs#L967) | ✅ |
| [Handheld Fan](src/import.rs#L972) | ✅ |
| [Hero's Cape](src/import.rs#L966) | ✅ |
| [Lillie's Pearl](src/import.rs#L969) | ✅ |
| [Lucky Helmet](src/import.rs#L971) | ✅ |
| [Powerglass](src/import.rs#L973) | ✅ |
| [Punk Helmet](src/import.rs#L970) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L976) | ✅ |
| [Area Zero Underdepths](src/import.rs#L511) | ✅ |
| [Battle Cage](src/import.rs#L512) | ✅ |
| [Festival Grounds](src/import.rs#L985) | ✅ |
| [Forest of Vitality](src/import.rs#L984) | ✅ |
| [Gravity Mountain](src/import.rs#L974) | ✅ |
| [Jamming Tower](src/import.rs#L982) | ✅ |
| [Lumiose City](src/import.rs#L981) | ✅ |
| [N's Castle](src/import.rs#L975) | ✅ |
| [Nighttime Mine](src/import.rs#L510) | ✅ |
| [Risky Ruins](src/import.rs#L983) | ✅ |
| [Team Rocket's Factory](src/import.rs#L977) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L513) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1423) | ✅ |
| [Enriching Energy](src/import.rs#L1406) | ✅ |
| [Growing Grass Energy](src/import.rs#L1405) | ✅ |
| [Mist Energy](src/import.rs#L1420) | ✅ |
| [Prism Energy](src/import.rs#L1426) | ✅ |
| [Spiky Energy](src/import.rs#L1417) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1409) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1612) | [✅](src/import.rs#L1612) | [✅](src/import.rs#L1494) |
| [Alakazam](src/import.rs#L1767) | [✅](src/import.rs#L1767) | [✅](src/import.rs#L1484) |
| [Annihilape](src/import.rs#L1641) | [✅](src/import.rs#L1641) | [✅](src/import.rs#L1450) |
| [Applin](src/import.rs#L1602) | [✅](src/import.rs#L1602) | — |
| [Bayleef](src/import.rs#L1644) | [✅](src/import.rs#L1644) | — |
| [Beldum](src/import.rs#L1624) | [✅](src/import.rs#L1624) | — |
| [Blaziken ex](src/import.rs#L1708) | [✅](src/import.rs#L1708) | [✅](src/import.rs#L1524) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1672) | [✅](src/import.rs#L1672) | [✅](src/import.rs#L1458) |
| [Brute Bonnet](src/import.rs#L1583) | [✅](src/import.rs#L1583) | — |
| [Budew](src/import.rs#L1649) | [✅](src/import.rs#L1649) | — |
| [Buneary](src/import.rs#L1643) | [✅](src/import.rs#L1643) | — |
| [Carvanha](src/import.rs#L1561) | [✅](src/import.rs#L1561) | — |
| [Celebi](src/import.rs#L1642) | [✅](src/import.rs#L1642) | — |
| [Chi-Yu](src/import.rs#L1740) | [✅](src/import.rs#L1740) | — |
| [Chien-Pao](src/import.rs#L1709) | [✅](src/import.rs#L1709) | [✅](src/import.rs#L1527) |
| [Chikorita](src/import.rs#L1645) | [✅](src/import.rs#L1645) | — |
| [Cofagrigus](src/import.rs#L1690) | [✅](src/import.rs#L1690) | — |
| [Combusken](src/import.rs#L1659) | [✅](src/import.rs#L1659) | — |
| [Crustle](src/import.rs#L1779) | [✅](src/import.rs#L1779) | [✅](src/import.rs#L1439) |
| [Dedenne](src/import.rs#L1599) | [✅](src/import.rs#L1599) | — |
| [Dipplin](src/import.rs#L1724) | [✅](src/import.rs#L1724) | [✅](src/import.rs#L1512) |
| [Dragapult ex](src/import.rs#L1606) | [✅](src/import.rs#L1606) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1467) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1613) | [✅](src/import.rs#L1613) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1491) |
| [Dudunsparce ex](src/import.rs#L1574) | [✅](src/import.rs#L1574) | — |
| [Dunsparce](src/import.rs#L1625) | [✅](src/import.rs#L1625) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1495) |
| [Dusknoir](src/import.rs#L1706) | [✅](src/import.rs#L1706) | [✅](src/import.rs#L1496) |
| [Duskull](src/import.rs#L1628) | [✅](src/import.rs#L1628) | — |
| [Dwebble](src/import.rs#L1618) | [✅](src/import.rs#L1618) | — |
| [Elgyem](src/import.rs#L1648) | [✅](src/import.rs#L1648) | — |
| [Enamorus](src/import.rs#L1668) | [✅](src/import.rs#L1668) | — |
| [Fan Rotom](src/import.rs#L1713) | [✅](src/import.rs#L1713) | [✅](src/import.rs#L1537) |
| [Fezandipiti ex](src/import.rs#L1787) | [✅](src/import.rs#L1787) | [✅](src/import.rs#L1485) |
| [Flutter Mane](src/import.rs#L1704) | [✅](src/import.rs#L1704) | [✅](src/import.rs#L1461) |
| [Genesect](src/import.rs#L1759) | [✅](src/import.rs#L1759) | [✅](src/import.rs#L1500) |
| [Genesect ex](src/import.rs#L1707) | [✅](src/import.rs#L1707) | [✅](src/import.rs#L1497) |
| [Goldeen](src/import.rs#L1722) | [✅](src/import.rs#L1722) | [✅](src/import.rs#L1510) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1615) | [✅](src/import.rs#L1615) | [✅](src/import.rs#L1451) |
| [Hydrapple ex](src/import.rs#L1673) | [✅](src/import.rs#L1673) | [✅](src/import.rs#L1452) |
| [Iron Crown ex](src/import.rs#L1633) | [✅](src/import.rs#L1633) | [✅](src/import.rs#L1447) |
| [Iron Leaves ex](src/import.rs#L1712) | [✅](src/import.rs#L1712) | [✅](src/import.rs#L1528) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1480) |
| [Koraidon ex](src/import.rs#L1634) | [✅](src/import.rs#L1634) | — |
| [Kyurem](src/import.rs#L1756) | [✅](src/import.rs#L1756) | [✅](src/import.rs#L1519) |
| [Latias ex](src/import.rs#L1773) | [✅](src/import.rs#L1773) | [✅](src/import.rs#L1438) |
| [Lillie's Clefairy ex](src/import.rs#L1797) | [✅](src/import.rs#L1797) | [✅](src/import.rs#L1442) |
| [Mega Absol ex](src/import.rs#L1680) | [✅](src/import.rs#L1680) | — |
| [Mega Excadrill ex](src/import.rs#L1676) | [✅](src/import.rs#L1676) | — |
| [Mega Kangaskhan ex](src/import.rs#L1783) | [✅](src/import.rs#L1783) | [✅](src/import.rs#L1435) |
| [Mega Lopunny ex](src/import.rs#L1581) | [✅](src/import.rs#L1581) | — |
| [Mega Sharpedo ex](src/import.rs#L1616) | [✅](src/import.rs#L1616) | — |
| [Mega Skarmory ex](src/import.rs#L1694) | [✅](src/import.rs#L1694) | — |
| [Mega Slowbro ex](src/import.rs#L1734) | [✅](src/import.rs#L1734) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1506) |
| [Meowth ex](src/import.rs#L1786) | [✅](src/import.rs#L1786) | [✅](src/import.rs#L1479) |
| [Metagross](src/import.rs#L1590) | [✅](src/import.rs#L1590) | — |
| [Metang](src/import.rs#L1774) | [✅](src/import.rs#L1774) | [✅](src/import.rs#L1470) |
| [Moltres](src/import.rs#L1626) | [✅](src/import.rs#L1626) | — |
| [Munkidori](src/import.rs#L1780) | [✅](src/import.rs#L1780) | [✅](src/import.rs#L1473) |
| [N's Darmanitan](src/import.rs#L1571) | [✅](src/import.rs#L1571) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1568) | [✅](src/import.rs#L1568) | — |
| [N's Zekrom](src/import.rs#L1580) | [✅](src/import.rs#L1580) | — |
| [N's Zoroark ex](src/import.rs#L1753) | [✅](src/import.rs#L1753) | [✅](src/import.rs#L1503) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1776) | [✅](src/import.rs#L1776) | [✅](src/import.rs#L1481) |
| [Paldean Tauros](src/import.rs#L1564) | [✅](src/import.rs#L1564) | — |
| [Passimian](src/import.rs#L1577) | [✅](src/import.rs#L1577) | — |
| [Patrat](src/import.rs#L1775) | [✅](src/import.rs#L1775) | [✅](src/import.rs#L1440) |
| [Pecharunt](src/import.rs#L1717) | [✅](src/import.rs#L1717) | [✅](src/import.rs#L1507) |
| [Pecharunt ex](src/import.rs#L1714) | [✅](src/import.rs#L1714) | [✅](src/import.rs#L1544) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1441) |
| [Rabsca](src/import.rs#L1630) | [✅](src/import.rs#L1630) | [✅](src/import.rs#L1446) |
| [Raging Bolt ex](src/import.rs#L1595) | [✅](src/import.rs#L1595) | — |
| [Rellor](src/import.rs#L1562) | [✅](src/import.rs#L1562) | — |
| [Seaking](src/import.rs#L1723) | [✅](src/import.rs#L1723) | [✅](src/import.rs#L1511) |
| [Shaymin](src/import.rs#L1770) | [✅](src/import.rs#L1770) | [✅](src/import.rs#L1445) |
| [Slowking](src/import.rs#L1619) | [✅](src/import.rs#L1619) | — |
| [Slowpoke](src/import.rs#L1627) | [✅](src/import.rs#L1627) | ❌ |
| [Smoochum](src/import.rs#L1687) | [✅](src/import.rs#L1687) | — |
| [Stunfisk](src/import.rs#L1669) | [✅](src/import.rs#L1669) | — |
| [Tapu Bulu](src/import.rs#L1563) | [✅](src/import.rs#L1563) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1464) |
| [Teal Mask Ogerpon ex](src/import.rs#L1788) | [✅](src/import.rs#L1788) | [✅](src/import.rs#L1488) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1513) |
| [Torchic](src/import.rs#L1629) | [✅](src/import.rs#L1629) | — |
| [Toxel](src/import.rs#L1614) | [✅](src/import.rs#L1614) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1531) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1604) | [✅](src/import.rs#L1604) | — |
| [Yveltal](src/import.rs#L1603) | [✅](src/import.rs#L1603) | — |
| [Zeraora](src/import.rs#L1586) | [✅](src/import.rs#L1586) | — |

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

