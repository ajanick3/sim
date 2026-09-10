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
| Supporters | 34 | 78 |
| Items | 30 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 7 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L772) | ✅ |
| [Black Belt's Training](src/import.rs#L780) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L393) | ✅ |
| [Brock's Scouting](src/import.rs#L810) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L712) | ✅ |
| [Crispin](src/import.rs#L618) | ✅ |
| [Cyrano](src/import.rs#L467) | ✅ |
| [Dawn](src/import.rs#L584) | ✅ |
| [Eri](src/import.rs#L803) | ✅ |
| [Gladion's Final Battle](src/import.rs#L784) | ✅ |
| [Gwynn](src/import.rs#L481) | ✅ |
| [Hilda](src/import.rs#L538) | ✅ |
| [Janine's Secret Art](src/import.rs#L834) | ✅ |
| [Judge](src/import.rs#L416) | ✅ |
| [Kieran](src/import.rs#L788) | ✅ |
| [Lana's Aid](src/import.rs#L738) | ✅ |
| [Lillie's Determination](src/import.rs#L417) | ✅ |
| [Morty's Conviction](src/import.rs#L798) | ✅ |
| [N's Plan](src/import.rs#L756) | ✅ |
| [Rosa's Encouragement](src/import.rs#L758) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L752) | ✅ |
| [Surfer](src/import.rs#L776) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L642) | ✅ |
| [Wally's Compassion](src/import.rs#L833) | ✅ |
| [Xerosic's Machinations](src/import.rs#L802) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L453) | ✅ |
| [Bug Catching Set](src/import.rs#L698) | ✅ |
| [Crushing Hammer](src/import.rs#L452) | ✅ |
| [Dusk Ball](src/import.rs#L864) | ✅ |
| [Energy Recycler](src/import.rs#L946) | ✅ |
| [Energy Retrieval](src/import.rs#L849) | ✅ |
| [Energy Search](src/import.rs#L835) | ✅ |
| [Energy Switch](src/import.rs#L523) | ✅ |
| [Enhanced Hammer](src/import.rs#L397) | ✅ |
| [Glass Trumpet](src/import.rs#L398) | ✅ |
| [Hand Trimmer](src/import.rs#L863) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L734) | ✅ |
| [N's PP Up](src/import.rs#L656) | ✅ |
| [Night Stretcher](src/import.rs#L424) | ✅ |
| [Prime Catcher](src/import.rs#L865) | ✅ |
| [Rare Candy](src/import.rs#L641) | ✅ |
| [Sacred Ash](src/import.rs#L495) | ✅ |
| [Secret Box](src/import.rs#L895) | ✅ |
| [Special Red Card](src/import.rs#L614) | ✅ |
| [Strange Timepiece](src/import.rs#L866) | ✅ |
| [Switch](src/import.rs#L733) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L932) | ✅ |
| [Tera Orb](src/import.rs#L524) | ✅ |
| [Tool Scrapper](src/import.rs#L392) | ✅ |
| [Transformation Tome](src/import.rs#L891) | ✅ |
| [Ultra Ball](src/import.rs#L509) | ✅ |
| [Unfair Stamp](src/import.rs#L726) | ✅ |
| [Wondrous Patch](src/import.rs#L670) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L867) | ✅ |
| [Binding Mochi](src/import.rs#L870) | ✅ |
| [Brave Bangle](src/import.rs#L869) | ✅ |
| [Handheld Fan](src/import.rs#L874) | ✅ |
| [Hero's Cape](src/import.rs#L868) | ✅ |
| [Lillie's Pearl](src/import.rs#L871) | ✅ |
| [Lucky Helmet](src/import.rs#L873) | ✅ |
| [Powerglass](src/import.rs#L875) | ✅ |
| [Punk Helmet](src/import.rs#L872) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L878) | ✅ |
| [Area Zero Underdepths](src/import.rs#L413) | ✅ |
| [Battle Cage](src/import.rs#L414) | ✅ |
| [Festival Grounds](src/import.rs#L887) | ✅ |
| [Forest of Vitality](src/import.rs#L886) | ✅ |
| [Gravity Mountain](src/import.rs#L876) | ✅ |
| [Jamming Tower](src/import.rs#L884) | ✅ |
| [Lumiose City](src/import.rs#L883) | ✅ |
| [N's Castle](src/import.rs#L877) | ✅ |
| [Nighttime Mine](src/import.rs#L412) | ✅ |
| [Risky Ruins](src/import.rs#L885) | ✅ |
| [Team Rocket's Factory](src/import.rs#L879) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L415) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1325) | ✅ |
| [Enriching Energy](src/import.rs#L1308) | ✅ |
| [Growing Grass Energy](src/import.rs#L1307) | ✅ |
| [Mist Energy](src/import.rs#L1322) | ✅ |
| [Prism Energy](src/import.rs#L1328) | ✅ |
| [Spiky Energy](src/import.rs#L1319) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1311) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1514) | [✅](src/import.rs#L1514) | [✅](src/import.rs#L1396) |
| [Alakazam](src/import.rs#L1669) | [✅](src/import.rs#L1669) | [✅](src/import.rs#L1386) |
| [Annihilape](src/import.rs#L1543) | [✅](src/import.rs#L1543) | [✅](src/import.rs#L1352) |
| [Applin](src/import.rs#L1504) | [✅](src/import.rs#L1504) | — |
| [Bayleef](src/import.rs#L1546) | [✅](src/import.rs#L1546) | — |
| [Beldum](src/import.rs#L1526) | [✅](src/import.rs#L1526) | — |
| [Blaziken ex](src/import.rs#L1610) | [✅](src/import.rs#L1610) | [✅](src/import.rs#L1426) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1574) | [✅](src/import.rs#L1574) | [✅](src/import.rs#L1360) |
| [Brute Bonnet](src/import.rs#L1485) | [✅](src/import.rs#L1485) | — |
| [Budew](src/import.rs#L1551) | [✅](src/import.rs#L1551) | — |
| [Buneary](src/import.rs#L1545) | [✅](src/import.rs#L1545) | — |
| [Carvanha](src/import.rs#L1463) | [✅](src/import.rs#L1463) | — |
| [Celebi](src/import.rs#L1544) | [✅](src/import.rs#L1544) | — |
| [Chi-Yu](src/import.rs#L1642) | [✅](src/import.rs#L1642) | — |
| [Chien-Pao](src/import.rs#L1611) | [✅](src/import.rs#L1611) | [✅](src/import.rs#L1429) |
| [Chikorita](src/import.rs#L1547) | [✅](src/import.rs#L1547) | — |
| [Cofagrigus](src/import.rs#L1592) | [✅](src/import.rs#L1592) | — |
| [Combusken](src/import.rs#L1561) | [✅](src/import.rs#L1561) | — |
| [Crustle](src/import.rs#L1681) | [✅](src/import.rs#L1681) | [✅](src/import.rs#L1341) |
| [Dedenne](src/import.rs#L1501) | [✅](src/import.rs#L1501) | — |
| [Dipplin](src/import.rs#L1626) | [✅](src/import.rs#L1626) | [✅](src/import.rs#L1414) |
| [Dragapult ex](src/import.rs#L1508) | [✅](src/import.rs#L1508) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1369) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1515) | [✅](src/import.rs#L1515) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1393) |
| [Dudunsparce ex](src/import.rs#L1476) | [✅](src/import.rs#L1476) | — |
| [Dunsparce](src/import.rs#L1527) | [✅](src/import.rs#L1527) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1397) |
| [Dusknoir](src/import.rs#L1608) | [✅](src/import.rs#L1608) | [✅](src/import.rs#L1398) |
| [Duskull](src/import.rs#L1530) | [✅](src/import.rs#L1530) | — |
| [Dwebble](src/import.rs#L1520) | [✅](src/import.rs#L1520) | — |
| [Elgyem](src/import.rs#L1550) | [✅](src/import.rs#L1550) | — |
| [Enamorus](src/import.rs#L1570) | [✅](src/import.rs#L1570) | — |
| [Fan Rotom](src/import.rs#L1615) | [✅](src/import.rs#L1615) | [✅](src/import.rs#L1439) |
| [Fezandipiti ex](src/import.rs#L1689) | [✅](src/import.rs#L1689) | [✅](src/import.rs#L1387) |
| [Flutter Mane](src/import.rs#L1606) | [✅](src/import.rs#L1606) | [✅](src/import.rs#L1363) |
| [Genesect](src/import.rs#L1661) | [✅](src/import.rs#L1661) | [✅](src/import.rs#L1402) |
| [Genesect ex](src/import.rs#L1609) | [✅](src/import.rs#L1609) | [✅](src/import.rs#L1399) |
| [Goldeen](src/import.rs#L1624) | [✅](src/import.rs#L1624) | [✅](src/import.rs#L1412) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1517) | [✅](src/import.rs#L1517) | [✅](src/import.rs#L1353) |
| [Hydrapple ex](src/import.rs#L1575) | [✅](src/import.rs#L1575) | [✅](src/import.rs#L1354) |
| [Iron Crown ex](src/import.rs#L1535) | [✅](src/import.rs#L1535) | [✅](src/import.rs#L1349) |
| [Iron Leaves ex](src/import.rs#L1614) | [✅](src/import.rs#L1614) | [✅](src/import.rs#L1430) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1382) |
| [Koraidon ex](src/import.rs#L1536) | [✅](src/import.rs#L1536) | — |
| [Kyurem](src/import.rs#L1658) | [✅](src/import.rs#L1658) | [✅](src/import.rs#L1421) |
| [Latias ex](src/import.rs#L1675) | [✅](src/import.rs#L1675) | [✅](src/import.rs#L1340) |
| [Lillie's Clefairy ex](src/import.rs#L1699) | [✅](src/import.rs#L1699) | [✅](src/import.rs#L1344) |
| [Mega Absol ex](src/import.rs#L1582) | [✅](src/import.rs#L1582) | — |
| [Mega Excadrill ex](src/import.rs#L1578) | [✅](src/import.rs#L1578) | — |
| [Mega Kangaskhan ex](src/import.rs#L1685) | [✅](src/import.rs#L1685) | [✅](src/import.rs#L1337) |
| [Mega Lopunny ex](src/import.rs#L1483) | [✅](src/import.rs#L1483) | — |
| [Mega Sharpedo ex](src/import.rs#L1518) | [✅](src/import.rs#L1518) | — |
| [Mega Skarmory ex](src/import.rs#L1596) | [✅](src/import.rs#L1596) | — |
| [Mega Slowbro ex](src/import.rs#L1636) | [✅](src/import.rs#L1636) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1408) |
| [Meowth ex](src/import.rs#L1688) | [✅](src/import.rs#L1688) | [✅](src/import.rs#L1381) |
| [Metagross](src/import.rs#L1492) | [✅](src/import.rs#L1492) | — |
| [Metang](src/import.rs#L1676) | [✅](src/import.rs#L1676) | [✅](src/import.rs#L1372) |
| [Moltres](src/import.rs#L1528) | [✅](src/import.rs#L1528) | — |
| [Munkidori](src/import.rs#L1682) | [✅](src/import.rs#L1682) | [✅](src/import.rs#L1375) |
| [N's Darmanitan](src/import.rs#L1473) | [✅](src/import.rs#L1473) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1470) | [✅](src/import.rs#L1470) | — |
| [N's Zekrom](src/import.rs#L1482) | [✅](src/import.rs#L1482) | — |
| [N's Zoroark ex](src/import.rs#L1655) | [✅](src/import.rs#L1655) | [✅](src/import.rs#L1405) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1678) | [✅](src/import.rs#L1678) | [✅](src/import.rs#L1383) |
| [Paldean Tauros](src/import.rs#L1466) | [✅](src/import.rs#L1466) | — |
| [Passimian](src/import.rs#L1479) | [✅](src/import.rs#L1479) | — |
| [Patrat](src/import.rs#L1677) | [✅](src/import.rs#L1677) | [✅](src/import.rs#L1342) |
| [Pecharunt](src/import.rs#L1619) | [✅](src/import.rs#L1619) | [✅](src/import.rs#L1409) |
| [Pecharunt ex](src/import.rs#L1616) | [✅](src/import.rs#L1616) | [✅](src/import.rs#L1446) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1343) |
| [Rabsca](src/import.rs#L1532) | [✅](src/import.rs#L1532) | [✅](src/import.rs#L1348) |
| [Raging Bolt ex](src/import.rs#L1497) | [✅](src/import.rs#L1497) | — |
| [Rellor](src/import.rs#L1464) | [✅](src/import.rs#L1464) | — |
| [Seaking](src/import.rs#L1625) | [✅](src/import.rs#L1625) | [✅](src/import.rs#L1413) |
| [Shaymin](src/import.rs#L1672) | [✅](src/import.rs#L1672) | [✅](src/import.rs#L1347) |
| [Slowking](src/import.rs#L1521) | [✅](src/import.rs#L1521) | — |
| [Slowpoke](src/import.rs#L1529) | [✅](src/import.rs#L1529) | ❌ |
| [Smoochum](src/import.rs#L1589) | [✅](src/import.rs#L1589) | — |
| [Stunfisk](src/import.rs#L1571) | [✅](src/import.rs#L1571) | — |
| [Tapu Bulu](src/import.rs#L1465) | [✅](src/import.rs#L1465) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1366) |
| [Teal Mask Ogerpon ex](src/import.rs#L1690) | [✅](src/import.rs#L1690) | [✅](src/import.rs#L1390) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1415) |
| [Torchic](src/import.rs#L1531) | [✅](src/import.rs#L1531) | — |
| [Toxel](src/import.rs#L1516) | [✅](src/import.rs#L1516) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1433) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1506) | [✅](src/import.rs#L1506) | — |
| [Yveltal](src/import.rs#L1505) | [✅](src/import.rs#L1505) | — |
| [Zeraora](src/import.rs#L1488) | [✅](src/import.rs#L1488) | — |

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

