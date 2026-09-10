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
| Supporters | 31 | 78 |
| Items | 30 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 7 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L760) | ✅ |
| [Black Belt's Training](src/import.rs#L768) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L381) | ✅ |
| [Brock's Scouting](src/import.rs#L798) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L700) | ✅ |
| [Crispin](src/import.rs#L606) | ✅ |
| [Cyrano](src/import.rs#L455) | ✅ |
| [Dawn](src/import.rs#L572) | ✅ |
| [Eri](src/import.rs#L791) | ✅ |
| [Gladion's Final Battle](src/import.rs#L772) | ✅ |
| [Gwynn](src/import.rs#L469) | ✅ |
| [Hilda](src/import.rs#L526) | ✅ |
| [Janine's Secret Art](src/import.rs#L822) | ✅ |
| [Judge](src/import.rs#L404) | ✅ |
| [Kieran](src/import.rs#L776) | ✅ |
| [Lana's Aid](src/import.rs#L726) | ✅ |
| [Lillie's Determination](src/import.rs#L405) | ✅ |
| [Morty's Conviction](src/import.rs#L786) | ✅ |
| [N's Plan](src/import.rs#L744) | ✅ |
| [Rosa's Encouragement](src/import.rs#L746) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L740) | ✅ |
| [Surfer](src/import.rs#L764) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L630) | ✅ |
| [Wally's Compassion](src/import.rs#L821) | ✅ |
| [Xerosic's Machinations](src/import.rs#L790) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L441) | ✅ |
| [Bug Catching Set](src/import.rs#L686) | ✅ |
| [Crushing Hammer](src/import.rs#L440) | ✅ |
| [Dusk Ball](src/import.rs#L852) | ✅ |
| [Energy Recycler](src/import.rs#L934) | ✅ |
| [Energy Retrieval](src/import.rs#L837) | ✅ |
| [Energy Search](src/import.rs#L823) | ✅ |
| [Energy Switch](src/import.rs#L511) | ✅ |
| [Enhanced Hammer](src/import.rs#L385) | ✅ |
| [Glass Trumpet](src/import.rs#L386) | ✅ |
| [Hand Trimmer](src/import.rs#L851) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L722) | ✅ |
| [N's PP Up](src/import.rs#L644) | ✅ |
| [Night Stretcher](src/import.rs#L412) | ✅ |
| [Prime Catcher](src/import.rs#L853) | ✅ |
| [Rare Candy](src/import.rs#L629) | ✅ |
| [Sacred Ash](src/import.rs#L483) | ✅ |
| [Secret Box](src/import.rs#L883) | ✅ |
| [Special Red Card](src/import.rs#L602) | ✅ |
| [Strange Timepiece](src/import.rs#L854) | ✅ |
| [Switch](src/import.rs#L721) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L920) | ✅ |
| [Tera Orb](src/import.rs#L512) | ✅ |
| [Tool Scrapper](src/import.rs#L380) | ✅ |
| [Transformation Tome](src/import.rs#L879) | ✅ |
| [Ultra Ball](src/import.rs#L497) | ✅ |
| [Unfair Stamp](src/import.rs#L714) | ✅ |
| [Wondrous Patch](src/import.rs#L658) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L855) | ✅ |
| [Binding Mochi](src/import.rs#L858) | ✅ |
| [Brave Bangle](src/import.rs#L857) | ✅ |
| [Handheld Fan](src/import.rs#L862) | ✅ |
| [Hero's Cape](src/import.rs#L856) | ✅ |
| [Lillie's Pearl](src/import.rs#L859) | ✅ |
| [Lucky Helmet](src/import.rs#L861) | ✅ |
| [Powerglass](src/import.rs#L863) | ✅ |
| [Punk Helmet](src/import.rs#L860) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L866) | ✅ |
| [Area Zero Underdepths](src/import.rs#L401) | ✅ |
| [Battle Cage](src/import.rs#L402) | ✅ |
| [Festival Grounds](src/import.rs#L875) | ✅ |
| [Forest of Vitality](src/import.rs#L874) | ✅ |
| [Gravity Mountain](src/import.rs#L864) | ✅ |
| [Jamming Tower](src/import.rs#L872) | ✅ |
| [Lumiose City](src/import.rs#L871) | ✅ |
| [N's Castle](src/import.rs#L865) | ✅ |
| [Nighttime Mine](src/import.rs#L400) | ✅ |
| [Risky Ruins](src/import.rs#L873) | ✅ |
| [Team Rocket's Factory](src/import.rs#L867) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L403) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1313) | ✅ |
| [Enriching Energy](src/import.rs#L1296) | ✅ |
| [Growing Grass Energy](src/import.rs#L1295) | ✅ |
| [Mist Energy](src/import.rs#L1310) | ✅ |
| [Prism Energy](src/import.rs#L1316) | ✅ |
| [Spiky Energy](src/import.rs#L1307) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1299) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1502) | [✅](src/import.rs#L1502) | [✅](src/import.rs#L1384) |
| [Alakazam](src/import.rs#L1657) | [✅](src/import.rs#L1657) | [✅](src/import.rs#L1374) |
| [Annihilape](src/import.rs#L1531) | [✅](src/import.rs#L1531) | [✅](src/import.rs#L1340) |
| [Applin](src/import.rs#L1492) | [✅](src/import.rs#L1492) | — |
| [Bayleef](src/import.rs#L1534) | [✅](src/import.rs#L1534) | — |
| [Beldum](src/import.rs#L1514) | [✅](src/import.rs#L1514) | — |
| [Blaziken ex](src/import.rs#L1598) | [✅](src/import.rs#L1598) | [✅](src/import.rs#L1414) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1562) | [✅](src/import.rs#L1562) | [✅](src/import.rs#L1348) |
| [Brute Bonnet](src/import.rs#L1473) | [✅](src/import.rs#L1473) | — |
| [Budew](src/import.rs#L1539) | [✅](src/import.rs#L1539) | — |
| [Buneary](src/import.rs#L1533) | [✅](src/import.rs#L1533) | — |
| [Carvanha](src/import.rs#L1451) | [✅](src/import.rs#L1451) | — |
| [Celebi](src/import.rs#L1532) | [✅](src/import.rs#L1532) | — |
| [Chi-Yu](src/import.rs#L1630) | [✅](src/import.rs#L1630) | — |
| [Chien-Pao](src/import.rs#L1599) | [✅](src/import.rs#L1599) | [✅](src/import.rs#L1417) |
| [Chikorita](src/import.rs#L1535) | [✅](src/import.rs#L1535) | — |
| [Cofagrigus](src/import.rs#L1580) | [✅](src/import.rs#L1580) | — |
| [Combusken](src/import.rs#L1549) | [✅](src/import.rs#L1549) | — |
| [Crustle](src/import.rs#L1669) | [✅](src/import.rs#L1669) | [✅](src/import.rs#L1329) |
| [Dedenne](src/import.rs#L1489) | [✅](src/import.rs#L1489) | — |
| [Dipplin](src/import.rs#L1614) | [✅](src/import.rs#L1614) | [✅](src/import.rs#L1402) |
| [Dragapult ex](src/import.rs#L1496) | [✅](src/import.rs#L1496) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1357) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1503) | [✅](src/import.rs#L1503) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1381) |
| [Dudunsparce ex](src/import.rs#L1464) | [✅](src/import.rs#L1464) | — |
| [Dunsparce](src/import.rs#L1515) | [✅](src/import.rs#L1515) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1385) |
| [Dusknoir](src/import.rs#L1596) | [✅](src/import.rs#L1596) | [✅](src/import.rs#L1386) |
| [Duskull](src/import.rs#L1518) | [✅](src/import.rs#L1518) | — |
| [Dwebble](src/import.rs#L1508) | [✅](src/import.rs#L1508) | — |
| [Elgyem](src/import.rs#L1538) | [✅](src/import.rs#L1538) | — |
| [Enamorus](src/import.rs#L1558) | [✅](src/import.rs#L1558) | — |
| [Fan Rotom](src/import.rs#L1603) | [✅](src/import.rs#L1603) | [✅](src/import.rs#L1427) |
| [Fezandipiti ex](src/import.rs#L1677) | [✅](src/import.rs#L1677) | [✅](src/import.rs#L1375) |
| [Flutter Mane](src/import.rs#L1594) | [✅](src/import.rs#L1594) | [✅](src/import.rs#L1351) |
| [Genesect](src/import.rs#L1649) | [✅](src/import.rs#L1649) | [✅](src/import.rs#L1390) |
| [Genesect ex](src/import.rs#L1597) | [✅](src/import.rs#L1597) | [✅](src/import.rs#L1387) |
| [Goldeen](src/import.rs#L1612) | [✅](src/import.rs#L1612) | [✅](src/import.rs#L1400) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1505) | [✅](src/import.rs#L1505) | [✅](src/import.rs#L1341) |
| [Hydrapple ex](src/import.rs#L1563) | [✅](src/import.rs#L1563) | [✅](src/import.rs#L1342) |
| [Iron Crown ex](src/import.rs#L1523) | [✅](src/import.rs#L1523) | [✅](src/import.rs#L1337) |
| [Iron Leaves ex](src/import.rs#L1602) | [✅](src/import.rs#L1602) | [✅](src/import.rs#L1418) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1370) |
| [Koraidon ex](src/import.rs#L1524) | [✅](src/import.rs#L1524) | — |
| [Kyurem](src/import.rs#L1646) | [✅](src/import.rs#L1646) | [✅](src/import.rs#L1409) |
| [Latias ex](src/import.rs#L1663) | [✅](src/import.rs#L1663) | [✅](src/import.rs#L1328) |
| [Lillie's Clefairy ex](src/import.rs#L1687) | [✅](src/import.rs#L1687) | [✅](src/import.rs#L1332) |
| [Mega Absol ex](src/import.rs#L1570) | [✅](src/import.rs#L1570) | — |
| [Mega Excadrill ex](src/import.rs#L1566) | [✅](src/import.rs#L1566) | — |
| [Mega Kangaskhan ex](src/import.rs#L1673) | [✅](src/import.rs#L1673) | [✅](src/import.rs#L1325) |
| [Mega Lopunny ex](src/import.rs#L1471) | [✅](src/import.rs#L1471) | — |
| [Mega Sharpedo ex](src/import.rs#L1506) | [✅](src/import.rs#L1506) | — |
| [Mega Skarmory ex](src/import.rs#L1584) | [✅](src/import.rs#L1584) | — |
| [Mega Slowbro ex](src/import.rs#L1624) | [✅](src/import.rs#L1624) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1396) |
| [Meowth ex](src/import.rs#L1676) | [✅](src/import.rs#L1676) | [✅](src/import.rs#L1369) |
| [Metagross](src/import.rs#L1480) | [✅](src/import.rs#L1480) | — |
| [Metang](src/import.rs#L1664) | [✅](src/import.rs#L1664) | [✅](src/import.rs#L1360) |
| [Moltres](src/import.rs#L1516) | [✅](src/import.rs#L1516) | — |
| [Munkidori](src/import.rs#L1670) | [✅](src/import.rs#L1670) | [✅](src/import.rs#L1363) |
| [N's Darmanitan](src/import.rs#L1461) | [✅](src/import.rs#L1461) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1458) | [✅](src/import.rs#L1458) | — |
| [N's Zekrom](src/import.rs#L1470) | [✅](src/import.rs#L1470) | — |
| [N's Zoroark ex](src/import.rs#L1643) | [✅](src/import.rs#L1643) | [✅](src/import.rs#L1393) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1666) | [✅](src/import.rs#L1666) | [✅](src/import.rs#L1371) |
| [Paldean Tauros](src/import.rs#L1454) | [✅](src/import.rs#L1454) | — |
| [Passimian](src/import.rs#L1467) | [✅](src/import.rs#L1467) | — |
| [Patrat](src/import.rs#L1665) | [✅](src/import.rs#L1665) | [✅](src/import.rs#L1330) |
| [Pecharunt](src/import.rs#L1607) | [✅](src/import.rs#L1607) | [✅](src/import.rs#L1397) |
| [Pecharunt ex](src/import.rs#L1604) | [✅](src/import.rs#L1604) | [✅](src/import.rs#L1434) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1331) |
| [Rabsca](src/import.rs#L1520) | [✅](src/import.rs#L1520) | [✅](src/import.rs#L1336) |
| [Raging Bolt ex](src/import.rs#L1485) | [✅](src/import.rs#L1485) | — |
| [Rellor](src/import.rs#L1452) | [✅](src/import.rs#L1452) | — |
| [Seaking](src/import.rs#L1613) | [✅](src/import.rs#L1613) | [✅](src/import.rs#L1401) |
| [Shaymin](src/import.rs#L1660) | [✅](src/import.rs#L1660) | [✅](src/import.rs#L1335) |
| [Slowking](src/import.rs#L1509) | [✅](src/import.rs#L1509) | — |
| [Slowpoke](src/import.rs#L1517) | [✅](src/import.rs#L1517) | ❌ |
| [Smoochum](src/import.rs#L1577) | [✅](src/import.rs#L1577) | — |
| [Stunfisk](src/import.rs#L1559) | [✅](src/import.rs#L1559) | — |
| [Tapu Bulu](src/import.rs#L1453) | [✅](src/import.rs#L1453) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1354) |
| [Teal Mask Ogerpon ex](src/import.rs#L1678) | [✅](src/import.rs#L1678) | [✅](src/import.rs#L1378) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1403) |
| [Torchic](src/import.rs#L1519) | [✅](src/import.rs#L1519) | — |
| [Toxel](src/import.rs#L1504) | [✅](src/import.rs#L1504) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1421) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1494) | [✅](src/import.rs#L1494) | — |
| [Yveltal](src/import.rs#L1493) | [✅](src/import.rs#L1493) | — |
| [Zeraora](src/import.rs#L1476) | [✅](src/import.rs#L1476) | — |

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

