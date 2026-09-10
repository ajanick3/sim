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
| Supporters | 40 | 78 |
| Items | 30 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 7 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L797) | ✅ |
| [Black Belt's Training](src/import.rs#L805) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L418) | ✅ |
| [Brock's Scouting](src/import.rs#L835) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L737) | ✅ |
| [Crispin](src/import.rs#L643) | ✅ |
| [Cyrano](src/import.rs#L492) | ✅ |
| [Dawn](src/import.rs#L609) | ✅ |
| [Eri](src/import.rs#L828) | ✅ |
| [Gladion's Final Battle](src/import.rs#L809) | ✅ |
| [Gwynn](src/import.rs#L506) | ✅ |
| [Hilda](src/import.rs#L563) | ✅ |
| [Janine's Secret Art](src/import.rs#L859) | ✅ |
| [Judge](src/import.rs#L441) | ✅ |
| [Kieran](src/import.rs#L813) | ✅ |
| [Lana's Aid](src/import.rs#L763) | ✅ |
| [Lillie's Determination](src/import.rs#L442) | ✅ |
| [Morty's Conviction](src/import.rs#L823) | ✅ |
| [N's Plan](src/import.rs#L781) | ✅ |
| [Rosa's Encouragement](src/import.rs#L783) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L777) | ✅ |
| [Surfer](src/import.rs#L801) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L667) | ✅ |
| [Wally's Compassion](src/import.rs#L858) | ✅ |
| [Xerosic's Machinations](src/import.rs#L827) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L478) | ✅ |
| [Bug Catching Set](src/import.rs#L723) | ✅ |
| [Crushing Hammer](src/import.rs#L477) | ✅ |
| [Dusk Ball](src/import.rs#L889) | ✅ |
| [Energy Recycler](src/import.rs#L971) | ✅ |
| [Energy Retrieval](src/import.rs#L874) | ✅ |
| [Energy Search](src/import.rs#L860) | ✅ |
| [Energy Switch](src/import.rs#L548) | ✅ |
| [Enhanced Hammer](src/import.rs#L422) | ✅ |
| [Glass Trumpet](src/import.rs#L423) | ✅ |
| [Hand Trimmer](src/import.rs#L888) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L759) | ✅ |
| [N's PP Up](src/import.rs#L681) | ✅ |
| [Night Stretcher](src/import.rs#L449) | ✅ |
| [Prime Catcher](src/import.rs#L890) | ✅ |
| [Rare Candy](src/import.rs#L666) | ✅ |
| [Sacred Ash](src/import.rs#L520) | ✅ |
| [Secret Box](src/import.rs#L920) | ✅ |
| [Special Red Card](src/import.rs#L639) | ✅ |
| [Strange Timepiece](src/import.rs#L891) | ✅ |
| [Switch](src/import.rs#L758) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L957) | ✅ |
| [Tera Orb](src/import.rs#L549) | ✅ |
| [Tool Scrapper](src/import.rs#L417) | ✅ |
| [Transformation Tome](src/import.rs#L916) | ✅ |
| [Ultra Ball](src/import.rs#L534) | ✅ |
| [Unfair Stamp](src/import.rs#L751) | ✅ |
| [Wondrous Patch](src/import.rs#L695) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L892) | ✅ |
| [Binding Mochi](src/import.rs#L895) | ✅ |
| [Brave Bangle](src/import.rs#L894) | ✅ |
| [Handheld Fan](src/import.rs#L899) | ✅ |
| [Hero's Cape](src/import.rs#L893) | ✅ |
| [Lillie's Pearl](src/import.rs#L896) | ✅ |
| [Lucky Helmet](src/import.rs#L898) | ✅ |
| [Powerglass](src/import.rs#L900) | ✅ |
| [Punk Helmet](src/import.rs#L897) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L903) | ✅ |
| [Area Zero Underdepths](src/import.rs#L438) | ✅ |
| [Battle Cage](src/import.rs#L439) | ✅ |
| [Festival Grounds](src/import.rs#L912) | ✅ |
| [Forest of Vitality](src/import.rs#L911) | ✅ |
| [Gravity Mountain](src/import.rs#L901) | ✅ |
| [Jamming Tower](src/import.rs#L909) | ✅ |
| [Lumiose City](src/import.rs#L908) | ✅ |
| [N's Castle](src/import.rs#L902) | ✅ |
| [Nighttime Mine](src/import.rs#L437) | ✅ |
| [Risky Ruins](src/import.rs#L910) | ✅ |
| [Team Rocket's Factory](src/import.rs#L904) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L440) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1350) | ✅ |
| [Enriching Energy](src/import.rs#L1333) | ✅ |
| [Growing Grass Energy](src/import.rs#L1332) | ✅ |
| [Mist Energy](src/import.rs#L1347) | ✅ |
| [Prism Energy](src/import.rs#L1353) | ✅ |
| [Spiky Energy](src/import.rs#L1344) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1336) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1539) | [✅](src/import.rs#L1539) | [✅](src/import.rs#L1421) |
| [Alakazam](src/import.rs#L1694) | [✅](src/import.rs#L1694) | [✅](src/import.rs#L1411) |
| [Annihilape](src/import.rs#L1568) | [✅](src/import.rs#L1568) | [✅](src/import.rs#L1377) |
| [Applin](src/import.rs#L1529) | [✅](src/import.rs#L1529) | — |
| [Bayleef](src/import.rs#L1571) | [✅](src/import.rs#L1571) | — |
| [Beldum](src/import.rs#L1551) | [✅](src/import.rs#L1551) | — |
| [Blaziken ex](src/import.rs#L1635) | [✅](src/import.rs#L1635) | [✅](src/import.rs#L1451) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1599) | [✅](src/import.rs#L1599) | [✅](src/import.rs#L1385) |
| [Brute Bonnet](src/import.rs#L1510) | [✅](src/import.rs#L1510) | — |
| [Budew](src/import.rs#L1576) | [✅](src/import.rs#L1576) | — |
| [Buneary](src/import.rs#L1570) | [✅](src/import.rs#L1570) | — |
| [Carvanha](src/import.rs#L1488) | [✅](src/import.rs#L1488) | — |
| [Celebi](src/import.rs#L1569) | [✅](src/import.rs#L1569) | — |
| [Chi-Yu](src/import.rs#L1667) | [✅](src/import.rs#L1667) | — |
| [Chien-Pao](src/import.rs#L1636) | [✅](src/import.rs#L1636) | [✅](src/import.rs#L1454) |
| [Chikorita](src/import.rs#L1572) | [✅](src/import.rs#L1572) | — |
| [Cofagrigus](src/import.rs#L1617) | [✅](src/import.rs#L1617) | — |
| [Combusken](src/import.rs#L1586) | [✅](src/import.rs#L1586) | — |
| [Crustle](src/import.rs#L1706) | [✅](src/import.rs#L1706) | [✅](src/import.rs#L1366) |
| [Dedenne](src/import.rs#L1526) | [✅](src/import.rs#L1526) | — |
| [Dipplin](src/import.rs#L1651) | [✅](src/import.rs#L1651) | [✅](src/import.rs#L1439) |
| [Dragapult ex](src/import.rs#L1533) | [✅](src/import.rs#L1533) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1394) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1540) | [✅](src/import.rs#L1540) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1418) |
| [Dudunsparce ex](src/import.rs#L1501) | [✅](src/import.rs#L1501) | — |
| [Dunsparce](src/import.rs#L1552) | [✅](src/import.rs#L1552) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1422) |
| [Dusknoir](src/import.rs#L1633) | [✅](src/import.rs#L1633) | [✅](src/import.rs#L1423) |
| [Duskull](src/import.rs#L1555) | [✅](src/import.rs#L1555) | — |
| [Dwebble](src/import.rs#L1545) | [✅](src/import.rs#L1545) | — |
| [Elgyem](src/import.rs#L1575) | [✅](src/import.rs#L1575) | — |
| [Enamorus](src/import.rs#L1595) | [✅](src/import.rs#L1595) | — |
| [Fan Rotom](src/import.rs#L1640) | [✅](src/import.rs#L1640) | [✅](src/import.rs#L1464) |
| [Fezandipiti ex](src/import.rs#L1714) | [✅](src/import.rs#L1714) | [✅](src/import.rs#L1412) |
| [Flutter Mane](src/import.rs#L1631) | [✅](src/import.rs#L1631) | [✅](src/import.rs#L1388) |
| [Genesect](src/import.rs#L1686) | [✅](src/import.rs#L1686) | [✅](src/import.rs#L1427) |
| [Genesect ex](src/import.rs#L1634) | [✅](src/import.rs#L1634) | [✅](src/import.rs#L1424) |
| [Goldeen](src/import.rs#L1649) | [✅](src/import.rs#L1649) | [✅](src/import.rs#L1437) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1542) | [✅](src/import.rs#L1542) | [✅](src/import.rs#L1378) |
| [Hydrapple ex](src/import.rs#L1600) | [✅](src/import.rs#L1600) | [✅](src/import.rs#L1379) |
| [Iron Crown ex](src/import.rs#L1560) | [✅](src/import.rs#L1560) | [✅](src/import.rs#L1374) |
| [Iron Leaves ex](src/import.rs#L1639) | [✅](src/import.rs#L1639) | [✅](src/import.rs#L1455) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1407) |
| [Koraidon ex](src/import.rs#L1561) | [✅](src/import.rs#L1561) | — |
| [Kyurem](src/import.rs#L1683) | [✅](src/import.rs#L1683) | [✅](src/import.rs#L1446) |
| [Latias ex](src/import.rs#L1700) | [✅](src/import.rs#L1700) | [✅](src/import.rs#L1365) |
| [Lillie's Clefairy ex](src/import.rs#L1724) | [✅](src/import.rs#L1724) | [✅](src/import.rs#L1369) |
| [Mega Absol ex](src/import.rs#L1607) | [✅](src/import.rs#L1607) | — |
| [Mega Excadrill ex](src/import.rs#L1603) | [✅](src/import.rs#L1603) | — |
| [Mega Kangaskhan ex](src/import.rs#L1710) | [✅](src/import.rs#L1710) | [✅](src/import.rs#L1362) |
| [Mega Lopunny ex](src/import.rs#L1508) | [✅](src/import.rs#L1508) | — |
| [Mega Sharpedo ex](src/import.rs#L1543) | [✅](src/import.rs#L1543) | — |
| [Mega Skarmory ex](src/import.rs#L1621) | [✅](src/import.rs#L1621) | — |
| [Mega Slowbro ex](src/import.rs#L1661) | [✅](src/import.rs#L1661) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1433) |
| [Meowth ex](src/import.rs#L1713) | [✅](src/import.rs#L1713) | [✅](src/import.rs#L1406) |
| [Metagross](src/import.rs#L1517) | [✅](src/import.rs#L1517) | — |
| [Metang](src/import.rs#L1701) | [✅](src/import.rs#L1701) | [✅](src/import.rs#L1397) |
| [Moltres](src/import.rs#L1553) | [✅](src/import.rs#L1553) | — |
| [Munkidori](src/import.rs#L1707) | [✅](src/import.rs#L1707) | [✅](src/import.rs#L1400) |
| [N's Darmanitan](src/import.rs#L1498) | [✅](src/import.rs#L1498) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1495) | [✅](src/import.rs#L1495) | — |
| [N's Zekrom](src/import.rs#L1507) | [✅](src/import.rs#L1507) | — |
| [N's Zoroark ex](src/import.rs#L1680) | [✅](src/import.rs#L1680) | [✅](src/import.rs#L1430) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1703) | [✅](src/import.rs#L1703) | [✅](src/import.rs#L1408) |
| [Paldean Tauros](src/import.rs#L1491) | [✅](src/import.rs#L1491) | — |
| [Passimian](src/import.rs#L1504) | [✅](src/import.rs#L1504) | — |
| [Patrat](src/import.rs#L1702) | [✅](src/import.rs#L1702) | [✅](src/import.rs#L1367) |
| [Pecharunt](src/import.rs#L1644) | [✅](src/import.rs#L1644) | [✅](src/import.rs#L1434) |
| [Pecharunt ex](src/import.rs#L1641) | [✅](src/import.rs#L1641) | [✅](src/import.rs#L1471) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1368) |
| [Rabsca](src/import.rs#L1557) | [✅](src/import.rs#L1557) | [✅](src/import.rs#L1373) |
| [Raging Bolt ex](src/import.rs#L1522) | [✅](src/import.rs#L1522) | — |
| [Rellor](src/import.rs#L1489) | [✅](src/import.rs#L1489) | — |
| [Seaking](src/import.rs#L1650) | [✅](src/import.rs#L1650) | [✅](src/import.rs#L1438) |
| [Shaymin](src/import.rs#L1697) | [✅](src/import.rs#L1697) | [✅](src/import.rs#L1372) |
| [Slowking](src/import.rs#L1546) | [✅](src/import.rs#L1546) | — |
| [Slowpoke](src/import.rs#L1554) | [✅](src/import.rs#L1554) | ❌ |
| [Smoochum](src/import.rs#L1614) | [✅](src/import.rs#L1614) | — |
| [Stunfisk](src/import.rs#L1596) | [✅](src/import.rs#L1596) | — |
| [Tapu Bulu](src/import.rs#L1490) | [✅](src/import.rs#L1490) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1391) |
| [Teal Mask Ogerpon ex](src/import.rs#L1715) | [✅](src/import.rs#L1715) | [✅](src/import.rs#L1415) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1440) |
| [Torchic](src/import.rs#L1556) | [✅](src/import.rs#L1556) | — |
| [Toxel](src/import.rs#L1541) | [✅](src/import.rs#L1541) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1458) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1531) | [✅](src/import.rs#L1531) | — |
| [Yveltal](src/import.rs#L1530) | [✅](src/import.rs#L1530) | — |
| [Zeraora](src/import.rs#L1513) | [✅](src/import.rs#L1513) | — |

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

