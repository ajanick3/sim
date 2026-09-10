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
| Supporters | 42 | 78 |
| Items | 30 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 7 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L825) | ✅ |
| [Black Belt's Training](src/import.rs#L833) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L446) | ✅ |
| [Brock's Scouting](src/import.rs#L863) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L765) | ✅ |
| [Crispin](src/import.rs#L671) | ✅ |
| [Cyrano](src/import.rs#L520) | ✅ |
| [Dawn](src/import.rs#L637) | ✅ |
| [Eri](src/import.rs#L856) | ✅ |
| [Gladion's Final Battle](src/import.rs#L837) | ✅ |
| [Gwynn](src/import.rs#L534) | ✅ |
| [Hilda](src/import.rs#L591) | ✅ |
| [Janine's Secret Art](src/import.rs#L887) | ✅ |
| [Judge](src/import.rs#L469) | ✅ |
| [Kieran](src/import.rs#L841) | ✅ |
| [Lana's Aid](src/import.rs#L791) | ✅ |
| [Lillie's Determination](src/import.rs#L470) | ✅ |
| [Morty's Conviction](src/import.rs#L851) | ✅ |
| [N's Plan](src/import.rs#L809) | ✅ |
| [Rosa's Encouragement](src/import.rs#L811) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L805) | ✅ |
| [Surfer](src/import.rs#L829) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L695) | ✅ |
| [Wally's Compassion](src/import.rs#L886) | ✅ |
| [Xerosic's Machinations](src/import.rs#L855) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L506) | ✅ |
| [Bug Catching Set](src/import.rs#L751) | ✅ |
| [Crushing Hammer](src/import.rs#L505) | ✅ |
| [Dusk Ball](src/import.rs#L917) | ✅ |
| [Energy Recycler](src/import.rs#L999) | ✅ |
| [Energy Retrieval](src/import.rs#L902) | ✅ |
| [Energy Search](src/import.rs#L888) | ✅ |
| [Energy Switch](src/import.rs#L576) | ✅ |
| [Enhanced Hammer](src/import.rs#L450) | ✅ |
| [Glass Trumpet](src/import.rs#L451) | ✅ |
| [Hand Trimmer](src/import.rs#L916) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L787) | ✅ |
| [N's PP Up](src/import.rs#L709) | ✅ |
| [Night Stretcher](src/import.rs#L477) | ✅ |
| [Prime Catcher](src/import.rs#L918) | ✅ |
| [Rare Candy](src/import.rs#L694) | ✅ |
| [Sacred Ash](src/import.rs#L548) | ✅ |
| [Secret Box](src/import.rs#L948) | ✅ |
| [Special Red Card](src/import.rs#L667) | ✅ |
| [Strange Timepiece](src/import.rs#L919) | ✅ |
| [Switch](src/import.rs#L786) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L985) | ✅ |
| [Tera Orb](src/import.rs#L577) | ✅ |
| [Tool Scrapper](src/import.rs#L445) | ✅ |
| [Transformation Tome](src/import.rs#L944) | ✅ |
| [Ultra Ball](src/import.rs#L562) | ✅ |
| [Unfair Stamp](src/import.rs#L779) | ✅ |
| [Wondrous Patch](src/import.rs#L723) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L920) | ✅ |
| [Binding Mochi](src/import.rs#L923) | ✅ |
| [Brave Bangle](src/import.rs#L922) | ✅ |
| [Handheld Fan](src/import.rs#L927) | ✅ |
| [Hero's Cape](src/import.rs#L921) | ✅ |
| [Lillie's Pearl](src/import.rs#L924) | ✅ |
| [Lucky Helmet](src/import.rs#L926) | ✅ |
| [Powerglass](src/import.rs#L928) | ✅ |
| [Punk Helmet](src/import.rs#L925) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L931) | ✅ |
| [Area Zero Underdepths](src/import.rs#L466) | ✅ |
| [Battle Cage](src/import.rs#L467) | ✅ |
| [Festival Grounds](src/import.rs#L940) | ✅ |
| [Forest of Vitality](src/import.rs#L939) | ✅ |
| [Gravity Mountain](src/import.rs#L929) | ✅ |
| [Jamming Tower](src/import.rs#L937) | ✅ |
| [Lumiose City](src/import.rs#L936) | ✅ |
| [N's Castle](src/import.rs#L930) | ✅ |
| [Nighttime Mine](src/import.rs#L465) | ✅ |
| [Risky Ruins](src/import.rs#L938) | ✅ |
| [Team Rocket's Factory](src/import.rs#L932) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L468) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1378) | ✅ |
| [Enriching Energy](src/import.rs#L1361) | ✅ |
| [Growing Grass Energy](src/import.rs#L1360) | ✅ |
| [Mist Energy](src/import.rs#L1375) | ✅ |
| [Prism Energy](src/import.rs#L1381) | ✅ |
| [Spiky Energy](src/import.rs#L1372) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1364) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1567) | [✅](src/import.rs#L1567) | [✅](src/import.rs#L1449) |
| [Alakazam](src/import.rs#L1722) | [✅](src/import.rs#L1722) | [✅](src/import.rs#L1439) |
| [Annihilape](src/import.rs#L1596) | [✅](src/import.rs#L1596) | [✅](src/import.rs#L1405) |
| [Applin](src/import.rs#L1557) | [✅](src/import.rs#L1557) | — |
| [Bayleef](src/import.rs#L1599) | [✅](src/import.rs#L1599) | — |
| [Beldum](src/import.rs#L1579) | [✅](src/import.rs#L1579) | — |
| [Blaziken ex](src/import.rs#L1663) | [✅](src/import.rs#L1663) | [✅](src/import.rs#L1479) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1627) | [✅](src/import.rs#L1627) | [✅](src/import.rs#L1413) |
| [Brute Bonnet](src/import.rs#L1538) | [✅](src/import.rs#L1538) | — |
| [Budew](src/import.rs#L1604) | [✅](src/import.rs#L1604) | — |
| [Buneary](src/import.rs#L1598) | [✅](src/import.rs#L1598) | — |
| [Carvanha](src/import.rs#L1516) | [✅](src/import.rs#L1516) | — |
| [Celebi](src/import.rs#L1597) | [✅](src/import.rs#L1597) | — |
| [Chi-Yu](src/import.rs#L1695) | [✅](src/import.rs#L1695) | — |
| [Chien-Pao](src/import.rs#L1664) | [✅](src/import.rs#L1664) | [✅](src/import.rs#L1482) |
| [Chikorita](src/import.rs#L1600) | [✅](src/import.rs#L1600) | — |
| [Cofagrigus](src/import.rs#L1645) | [✅](src/import.rs#L1645) | — |
| [Combusken](src/import.rs#L1614) | [✅](src/import.rs#L1614) | — |
| [Crustle](src/import.rs#L1734) | [✅](src/import.rs#L1734) | [✅](src/import.rs#L1394) |
| [Dedenne](src/import.rs#L1554) | [✅](src/import.rs#L1554) | — |
| [Dipplin](src/import.rs#L1679) | [✅](src/import.rs#L1679) | [✅](src/import.rs#L1467) |
| [Dragapult ex](src/import.rs#L1561) | [✅](src/import.rs#L1561) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1422) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1568) | [✅](src/import.rs#L1568) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1446) |
| [Dudunsparce ex](src/import.rs#L1529) | [✅](src/import.rs#L1529) | — |
| [Dunsparce](src/import.rs#L1580) | [✅](src/import.rs#L1580) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1450) |
| [Dusknoir](src/import.rs#L1661) | [✅](src/import.rs#L1661) | [✅](src/import.rs#L1451) |
| [Duskull](src/import.rs#L1583) | [✅](src/import.rs#L1583) | — |
| [Dwebble](src/import.rs#L1573) | [✅](src/import.rs#L1573) | — |
| [Elgyem](src/import.rs#L1603) | [✅](src/import.rs#L1603) | — |
| [Enamorus](src/import.rs#L1623) | [✅](src/import.rs#L1623) | — |
| [Fan Rotom](src/import.rs#L1668) | [✅](src/import.rs#L1668) | [✅](src/import.rs#L1492) |
| [Fezandipiti ex](src/import.rs#L1742) | [✅](src/import.rs#L1742) | [✅](src/import.rs#L1440) |
| [Flutter Mane](src/import.rs#L1659) | [✅](src/import.rs#L1659) | [✅](src/import.rs#L1416) |
| [Genesect](src/import.rs#L1714) | [✅](src/import.rs#L1714) | [✅](src/import.rs#L1455) |
| [Genesect ex](src/import.rs#L1662) | [✅](src/import.rs#L1662) | [✅](src/import.rs#L1452) |
| [Goldeen](src/import.rs#L1677) | [✅](src/import.rs#L1677) | [✅](src/import.rs#L1465) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1570) | [✅](src/import.rs#L1570) | [✅](src/import.rs#L1406) |
| [Hydrapple ex](src/import.rs#L1628) | [✅](src/import.rs#L1628) | [✅](src/import.rs#L1407) |
| [Iron Crown ex](src/import.rs#L1588) | [✅](src/import.rs#L1588) | [✅](src/import.rs#L1402) |
| [Iron Leaves ex](src/import.rs#L1667) | [✅](src/import.rs#L1667) | [✅](src/import.rs#L1483) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1435) |
| [Koraidon ex](src/import.rs#L1589) | [✅](src/import.rs#L1589) | — |
| [Kyurem](src/import.rs#L1711) | [✅](src/import.rs#L1711) | [✅](src/import.rs#L1474) |
| [Latias ex](src/import.rs#L1728) | [✅](src/import.rs#L1728) | [✅](src/import.rs#L1393) |
| [Lillie's Clefairy ex](src/import.rs#L1752) | [✅](src/import.rs#L1752) | [✅](src/import.rs#L1397) |
| [Mega Absol ex](src/import.rs#L1635) | [✅](src/import.rs#L1635) | — |
| [Mega Excadrill ex](src/import.rs#L1631) | [✅](src/import.rs#L1631) | — |
| [Mega Kangaskhan ex](src/import.rs#L1738) | [✅](src/import.rs#L1738) | [✅](src/import.rs#L1390) |
| [Mega Lopunny ex](src/import.rs#L1536) | [✅](src/import.rs#L1536) | — |
| [Mega Sharpedo ex](src/import.rs#L1571) | [✅](src/import.rs#L1571) | — |
| [Mega Skarmory ex](src/import.rs#L1649) | [✅](src/import.rs#L1649) | — |
| [Mega Slowbro ex](src/import.rs#L1689) | [✅](src/import.rs#L1689) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1461) |
| [Meowth ex](src/import.rs#L1741) | [✅](src/import.rs#L1741) | [✅](src/import.rs#L1434) |
| [Metagross](src/import.rs#L1545) | [✅](src/import.rs#L1545) | — |
| [Metang](src/import.rs#L1729) | [✅](src/import.rs#L1729) | [✅](src/import.rs#L1425) |
| [Moltres](src/import.rs#L1581) | [✅](src/import.rs#L1581) | — |
| [Munkidori](src/import.rs#L1735) | [✅](src/import.rs#L1735) | [✅](src/import.rs#L1428) |
| [N's Darmanitan](src/import.rs#L1526) | [✅](src/import.rs#L1526) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1523) | [✅](src/import.rs#L1523) | — |
| [N's Zekrom](src/import.rs#L1535) | [✅](src/import.rs#L1535) | — |
| [N's Zoroark ex](src/import.rs#L1708) | [✅](src/import.rs#L1708) | [✅](src/import.rs#L1458) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1731) | [✅](src/import.rs#L1731) | [✅](src/import.rs#L1436) |
| [Paldean Tauros](src/import.rs#L1519) | [✅](src/import.rs#L1519) | — |
| [Passimian](src/import.rs#L1532) | [✅](src/import.rs#L1532) | — |
| [Patrat](src/import.rs#L1730) | [✅](src/import.rs#L1730) | [✅](src/import.rs#L1395) |
| [Pecharunt](src/import.rs#L1672) | [✅](src/import.rs#L1672) | [✅](src/import.rs#L1462) |
| [Pecharunt ex](src/import.rs#L1669) | [✅](src/import.rs#L1669) | [✅](src/import.rs#L1499) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1396) |
| [Rabsca](src/import.rs#L1585) | [✅](src/import.rs#L1585) | [✅](src/import.rs#L1401) |
| [Raging Bolt ex](src/import.rs#L1550) | [✅](src/import.rs#L1550) | — |
| [Rellor](src/import.rs#L1517) | [✅](src/import.rs#L1517) | — |
| [Seaking](src/import.rs#L1678) | [✅](src/import.rs#L1678) | [✅](src/import.rs#L1466) |
| [Shaymin](src/import.rs#L1725) | [✅](src/import.rs#L1725) | [✅](src/import.rs#L1400) |
| [Slowking](src/import.rs#L1574) | [✅](src/import.rs#L1574) | — |
| [Slowpoke](src/import.rs#L1582) | [✅](src/import.rs#L1582) | ❌ |
| [Smoochum](src/import.rs#L1642) | [✅](src/import.rs#L1642) | — |
| [Stunfisk](src/import.rs#L1624) | [✅](src/import.rs#L1624) | — |
| [Tapu Bulu](src/import.rs#L1518) | [✅](src/import.rs#L1518) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1419) |
| [Teal Mask Ogerpon ex](src/import.rs#L1743) | [✅](src/import.rs#L1743) | [✅](src/import.rs#L1443) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1468) |
| [Torchic](src/import.rs#L1584) | [✅](src/import.rs#L1584) | — |
| [Toxel](src/import.rs#L1569) | [✅](src/import.rs#L1569) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1486) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1559) | [✅](src/import.rs#L1559) | — |
| [Yveltal](src/import.rs#L1558) | [✅](src/import.rs#L1558) | — |
| [Zeraora](src/import.rs#L1541) | [✅](src/import.rs#L1541) | — |

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

