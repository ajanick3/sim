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
| Supporters | 37 | 78 |
| Items | 30 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 7 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L778) | ✅ |
| [Black Belt's Training](src/import.rs#L786) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L399) | ✅ |
| [Brock's Scouting](src/import.rs#L816) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L718) | ✅ |
| [Crispin](src/import.rs#L624) | ✅ |
| [Cyrano](src/import.rs#L473) | ✅ |
| [Dawn](src/import.rs#L590) | ✅ |
| [Eri](src/import.rs#L809) | ✅ |
| [Gladion's Final Battle](src/import.rs#L790) | ✅ |
| [Gwynn](src/import.rs#L487) | ✅ |
| [Hilda](src/import.rs#L544) | ✅ |
| [Janine's Secret Art](src/import.rs#L840) | ✅ |
| [Judge](src/import.rs#L422) | ✅ |
| [Kieran](src/import.rs#L794) | ✅ |
| [Lana's Aid](src/import.rs#L744) | ✅ |
| [Lillie's Determination](src/import.rs#L423) | ✅ |
| [Morty's Conviction](src/import.rs#L804) | ✅ |
| [N's Plan](src/import.rs#L762) | ✅ |
| [Rosa's Encouragement](src/import.rs#L764) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L758) | ✅ |
| [Surfer](src/import.rs#L782) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L648) | ✅ |
| [Wally's Compassion](src/import.rs#L839) | ✅ |
| [Xerosic's Machinations](src/import.rs#L808) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L459) | ✅ |
| [Bug Catching Set](src/import.rs#L704) | ✅ |
| [Crushing Hammer](src/import.rs#L458) | ✅ |
| [Dusk Ball](src/import.rs#L870) | ✅ |
| [Energy Recycler](src/import.rs#L952) | ✅ |
| [Energy Retrieval](src/import.rs#L855) | ✅ |
| [Energy Search](src/import.rs#L841) | ✅ |
| [Energy Switch](src/import.rs#L529) | ✅ |
| [Enhanced Hammer](src/import.rs#L403) | ✅ |
| [Glass Trumpet](src/import.rs#L404) | ✅ |
| [Hand Trimmer](src/import.rs#L869) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L740) | ✅ |
| [N's PP Up](src/import.rs#L662) | ✅ |
| [Night Stretcher](src/import.rs#L430) | ✅ |
| [Prime Catcher](src/import.rs#L871) | ✅ |
| [Rare Candy](src/import.rs#L647) | ✅ |
| [Sacred Ash](src/import.rs#L501) | ✅ |
| [Secret Box](src/import.rs#L901) | ✅ |
| [Special Red Card](src/import.rs#L620) | ✅ |
| [Strange Timepiece](src/import.rs#L872) | ✅ |
| [Switch](src/import.rs#L739) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L938) | ✅ |
| [Tera Orb](src/import.rs#L530) | ✅ |
| [Tool Scrapper](src/import.rs#L398) | ✅ |
| [Transformation Tome](src/import.rs#L897) | ✅ |
| [Ultra Ball](src/import.rs#L515) | ✅ |
| [Unfair Stamp](src/import.rs#L732) | ✅ |
| [Wondrous Patch](src/import.rs#L676) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L873) | ✅ |
| [Binding Mochi](src/import.rs#L876) | ✅ |
| [Brave Bangle](src/import.rs#L875) | ✅ |
| [Handheld Fan](src/import.rs#L880) | ✅ |
| [Hero's Cape](src/import.rs#L874) | ✅ |
| [Lillie's Pearl](src/import.rs#L877) | ✅ |
| [Lucky Helmet](src/import.rs#L879) | ✅ |
| [Powerglass](src/import.rs#L881) | ✅ |
| [Punk Helmet](src/import.rs#L878) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L884) | ✅ |
| [Area Zero Underdepths](src/import.rs#L419) | ✅ |
| [Battle Cage](src/import.rs#L420) | ✅ |
| [Festival Grounds](src/import.rs#L893) | ✅ |
| [Forest of Vitality](src/import.rs#L892) | ✅ |
| [Gravity Mountain](src/import.rs#L882) | ✅ |
| [Jamming Tower](src/import.rs#L890) | ✅ |
| [Lumiose City](src/import.rs#L889) | ✅ |
| [N's Castle](src/import.rs#L883) | ✅ |
| [Nighttime Mine](src/import.rs#L418) | ✅ |
| [Risky Ruins](src/import.rs#L891) | ✅ |
| [Team Rocket's Factory](src/import.rs#L885) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L421) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1331) | ✅ |
| [Enriching Energy](src/import.rs#L1314) | ✅ |
| [Growing Grass Energy](src/import.rs#L1313) | ✅ |
| [Mist Energy](src/import.rs#L1328) | ✅ |
| [Prism Energy](src/import.rs#L1334) | ✅ |
| [Spiky Energy](src/import.rs#L1325) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1317) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1520) | [✅](src/import.rs#L1520) | [✅](src/import.rs#L1402) |
| [Alakazam](src/import.rs#L1675) | [✅](src/import.rs#L1675) | [✅](src/import.rs#L1392) |
| [Annihilape](src/import.rs#L1549) | [✅](src/import.rs#L1549) | [✅](src/import.rs#L1358) |
| [Applin](src/import.rs#L1510) | [✅](src/import.rs#L1510) | — |
| [Bayleef](src/import.rs#L1552) | [✅](src/import.rs#L1552) | — |
| [Beldum](src/import.rs#L1532) | [✅](src/import.rs#L1532) | — |
| [Blaziken ex](src/import.rs#L1616) | [✅](src/import.rs#L1616) | [✅](src/import.rs#L1432) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1580) | [✅](src/import.rs#L1580) | [✅](src/import.rs#L1366) |
| [Brute Bonnet](src/import.rs#L1491) | [✅](src/import.rs#L1491) | — |
| [Budew](src/import.rs#L1557) | [✅](src/import.rs#L1557) | — |
| [Buneary](src/import.rs#L1551) | [✅](src/import.rs#L1551) | — |
| [Carvanha](src/import.rs#L1469) | [✅](src/import.rs#L1469) | — |
| [Celebi](src/import.rs#L1550) | [✅](src/import.rs#L1550) | — |
| [Chi-Yu](src/import.rs#L1648) | [✅](src/import.rs#L1648) | — |
| [Chien-Pao](src/import.rs#L1617) | [✅](src/import.rs#L1617) | [✅](src/import.rs#L1435) |
| [Chikorita](src/import.rs#L1553) | [✅](src/import.rs#L1553) | — |
| [Cofagrigus](src/import.rs#L1598) | [✅](src/import.rs#L1598) | — |
| [Combusken](src/import.rs#L1567) | [✅](src/import.rs#L1567) | — |
| [Crustle](src/import.rs#L1687) | [✅](src/import.rs#L1687) | [✅](src/import.rs#L1347) |
| [Dedenne](src/import.rs#L1507) | [✅](src/import.rs#L1507) | — |
| [Dipplin](src/import.rs#L1632) | [✅](src/import.rs#L1632) | [✅](src/import.rs#L1420) |
| [Dragapult ex](src/import.rs#L1514) | [✅](src/import.rs#L1514) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1375) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1521) | [✅](src/import.rs#L1521) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1399) |
| [Dudunsparce ex](src/import.rs#L1482) | [✅](src/import.rs#L1482) | — |
| [Dunsparce](src/import.rs#L1533) | [✅](src/import.rs#L1533) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1403) |
| [Dusknoir](src/import.rs#L1614) | [✅](src/import.rs#L1614) | [✅](src/import.rs#L1404) |
| [Duskull](src/import.rs#L1536) | [✅](src/import.rs#L1536) | — |
| [Dwebble](src/import.rs#L1526) | [✅](src/import.rs#L1526) | — |
| [Elgyem](src/import.rs#L1556) | [✅](src/import.rs#L1556) | — |
| [Enamorus](src/import.rs#L1576) | [✅](src/import.rs#L1576) | — |
| [Fan Rotom](src/import.rs#L1621) | [✅](src/import.rs#L1621) | [✅](src/import.rs#L1445) |
| [Fezandipiti ex](src/import.rs#L1695) | [✅](src/import.rs#L1695) | [✅](src/import.rs#L1393) |
| [Flutter Mane](src/import.rs#L1612) | [✅](src/import.rs#L1612) | [✅](src/import.rs#L1369) |
| [Genesect](src/import.rs#L1667) | [✅](src/import.rs#L1667) | [✅](src/import.rs#L1408) |
| [Genesect ex](src/import.rs#L1615) | [✅](src/import.rs#L1615) | [✅](src/import.rs#L1405) |
| [Goldeen](src/import.rs#L1630) | [✅](src/import.rs#L1630) | [✅](src/import.rs#L1418) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1523) | [✅](src/import.rs#L1523) | [✅](src/import.rs#L1359) |
| [Hydrapple ex](src/import.rs#L1581) | [✅](src/import.rs#L1581) | [✅](src/import.rs#L1360) |
| [Iron Crown ex](src/import.rs#L1541) | [✅](src/import.rs#L1541) | [✅](src/import.rs#L1355) |
| [Iron Leaves ex](src/import.rs#L1620) | [✅](src/import.rs#L1620) | [✅](src/import.rs#L1436) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1388) |
| [Koraidon ex](src/import.rs#L1542) | [✅](src/import.rs#L1542) | — |
| [Kyurem](src/import.rs#L1664) | [✅](src/import.rs#L1664) | [✅](src/import.rs#L1427) |
| [Latias ex](src/import.rs#L1681) | [✅](src/import.rs#L1681) | [✅](src/import.rs#L1346) |
| [Lillie's Clefairy ex](src/import.rs#L1705) | [✅](src/import.rs#L1705) | [✅](src/import.rs#L1350) |
| [Mega Absol ex](src/import.rs#L1588) | [✅](src/import.rs#L1588) | — |
| [Mega Excadrill ex](src/import.rs#L1584) | [✅](src/import.rs#L1584) | — |
| [Mega Kangaskhan ex](src/import.rs#L1691) | [✅](src/import.rs#L1691) | [✅](src/import.rs#L1343) |
| [Mega Lopunny ex](src/import.rs#L1489) | [✅](src/import.rs#L1489) | — |
| [Mega Sharpedo ex](src/import.rs#L1524) | [✅](src/import.rs#L1524) | — |
| [Mega Skarmory ex](src/import.rs#L1602) | [✅](src/import.rs#L1602) | — |
| [Mega Slowbro ex](src/import.rs#L1642) | [✅](src/import.rs#L1642) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1414) |
| [Meowth ex](src/import.rs#L1694) | [✅](src/import.rs#L1694) | [✅](src/import.rs#L1387) |
| [Metagross](src/import.rs#L1498) | [✅](src/import.rs#L1498) | — |
| [Metang](src/import.rs#L1682) | [✅](src/import.rs#L1682) | [✅](src/import.rs#L1378) |
| [Moltres](src/import.rs#L1534) | [✅](src/import.rs#L1534) | — |
| [Munkidori](src/import.rs#L1688) | [✅](src/import.rs#L1688) | [✅](src/import.rs#L1381) |
| [N's Darmanitan](src/import.rs#L1479) | [✅](src/import.rs#L1479) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1476) | [✅](src/import.rs#L1476) | — |
| [N's Zekrom](src/import.rs#L1488) | [✅](src/import.rs#L1488) | — |
| [N's Zoroark ex](src/import.rs#L1661) | [✅](src/import.rs#L1661) | [✅](src/import.rs#L1411) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1684) | [✅](src/import.rs#L1684) | [✅](src/import.rs#L1389) |
| [Paldean Tauros](src/import.rs#L1472) | [✅](src/import.rs#L1472) | — |
| [Passimian](src/import.rs#L1485) | [✅](src/import.rs#L1485) | — |
| [Patrat](src/import.rs#L1683) | [✅](src/import.rs#L1683) | [✅](src/import.rs#L1348) |
| [Pecharunt](src/import.rs#L1625) | [✅](src/import.rs#L1625) | [✅](src/import.rs#L1415) |
| [Pecharunt ex](src/import.rs#L1622) | [✅](src/import.rs#L1622) | [✅](src/import.rs#L1452) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1349) |
| [Rabsca](src/import.rs#L1538) | [✅](src/import.rs#L1538) | [✅](src/import.rs#L1354) |
| [Raging Bolt ex](src/import.rs#L1503) | [✅](src/import.rs#L1503) | — |
| [Rellor](src/import.rs#L1470) | [✅](src/import.rs#L1470) | — |
| [Seaking](src/import.rs#L1631) | [✅](src/import.rs#L1631) | [✅](src/import.rs#L1419) |
| [Shaymin](src/import.rs#L1678) | [✅](src/import.rs#L1678) | [✅](src/import.rs#L1353) |
| [Slowking](src/import.rs#L1527) | [✅](src/import.rs#L1527) | — |
| [Slowpoke](src/import.rs#L1535) | [✅](src/import.rs#L1535) | ❌ |
| [Smoochum](src/import.rs#L1595) | [✅](src/import.rs#L1595) | — |
| [Stunfisk](src/import.rs#L1577) | [✅](src/import.rs#L1577) | — |
| [Tapu Bulu](src/import.rs#L1471) | [✅](src/import.rs#L1471) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1372) |
| [Teal Mask Ogerpon ex](src/import.rs#L1696) | [✅](src/import.rs#L1696) | [✅](src/import.rs#L1396) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1421) |
| [Torchic](src/import.rs#L1537) | [✅](src/import.rs#L1537) | — |
| [Toxel](src/import.rs#L1522) | [✅](src/import.rs#L1522) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1439) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1512) | [✅](src/import.rs#L1512) | — |
| [Yveltal](src/import.rs#L1511) | [✅](src/import.rs#L1511) | — |
| [Zeraora](src/import.rs#L1494) | [✅](src/import.rs#L1494) | — |

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

