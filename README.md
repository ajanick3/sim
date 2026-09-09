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

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L759) | ✅ |
| [Black Belt's Training](src/import.rs#L767) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L380) | ✅ |
| [Brock's Scouting](src/import.rs#L797) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L699) | ✅ |
| [Crispin](src/import.rs#L605) | ✅ |
| [Cyrano](src/import.rs#L454) | ✅ |
| [Dawn](src/import.rs#L571) | ✅ |
| [Eri](src/import.rs#L790) | ✅ |
| [Gladion's Final Battle](src/import.rs#L771) | ✅ |
| [Gwynn](src/import.rs#L468) | ✅ |
| [Hilda](src/import.rs#L525) | ✅ |
| [Janine's Secret Art](src/import.rs#L821) | ✅ |
| [Judge](src/import.rs#L403) | ✅ |
| [Kieran](src/import.rs#L775) | ✅ |
| [Lana's Aid](src/import.rs#L725) | ✅ |
| [Lillie's Determination](src/import.rs#L404) | ✅ |
| [Morty's Conviction](src/import.rs#L785) | ✅ |
| [N's Plan](src/import.rs#L743) | ✅ |
| [Rosa's Encouragement](src/import.rs#L745) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L739) | ✅ |
| [Surfer](src/import.rs#L763) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L629) | ✅ |
| [Wally's Compassion](src/import.rs#L820) | ✅ |
| [Xerosic's Machinations](src/import.rs#L789) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L440) | ✅ |
| [Bug Catching Set](src/import.rs#L685) | ✅ |
| [Crushing Hammer](src/import.rs#L439) | ✅ |
| [Dusk Ball](src/import.rs#L851) | ✅ |
| [Energy Recycler](src/import.rs#L933) | ✅ |
| [Energy Retrieval](src/import.rs#L836) | ✅ |
| [Energy Search](src/import.rs#L822) | ✅ |
| [Energy Switch](src/import.rs#L510) | ✅ |
| [Enhanced Hammer](src/import.rs#L384) | ✅ |
| [Glass Trumpet](src/import.rs#L385) | ✅ |
| [Hand Trimmer](src/import.rs#L850) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L721) | ✅ |
| [N's PP Up](src/import.rs#L643) | ✅ |
| [Night Stretcher](src/import.rs#L411) | ✅ |
| [Prime Catcher](src/import.rs#L852) | ✅ |
| [Rare Candy](src/import.rs#L628) | ✅ |
| [Sacred Ash](src/import.rs#L482) | ✅ |
| [Secret Box](src/import.rs#L882) | ✅ |
| [Special Red Card](src/import.rs#L601) | ✅ |
| [Strange Timepiece](src/import.rs#L853) | ✅ |
| [Switch](src/import.rs#L720) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L919) | ✅ |
| [Tera Orb](src/import.rs#L511) | ✅ |
| [Tool Scrapper](src/import.rs#L379) | ✅ |
| [Transformation Tome](src/import.rs#L878) | ✅ |
| [Ultra Ball](src/import.rs#L496) | ✅ |
| [Unfair Stamp](src/import.rs#L713) | ✅ |
| [Wondrous Patch](src/import.rs#L657) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L854) | ✅ |
| [Binding Mochi](src/import.rs#L857) | ✅ |
| [Brave Bangle](src/import.rs#L856) | ✅ |
| [Handheld Fan](src/import.rs#L861) | ✅ |
| [Hero's Cape](src/import.rs#L855) | ✅ |
| [Lillie's Pearl](src/import.rs#L858) | ✅ |
| [Lucky Helmet](src/import.rs#L860) | ✅ |
| [Powerglass](src/import.rs#L862) | ✅ |
| [Punk Helmet](src/import.rs#L859) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L865) | ✅ |
| [Area Zero Underdepths](src/import.rs#L400) | ✅ |
| [Battle Cage](src/import.rs#L401) | ✅ |
| [Festival Grounds](src/import.rs#L874) | ✅ |
| [Forest of Vitality](src/import.rs#L873) | ✅ |
| [Gravity Mountain](src/import.rs#L863) | ✅ |
| [Jamming Tower](src/import.rs#L871) | ✅ |
| [Lumiose City](src/import.rs#L870) | ✅ |
| [N's Castle](src/import.rs#L864) | ✅ |
| [Nighttime Mine](src/import.rs#L399) | ✅ |
| [Risky Ruins](src/import.rs#L872) | ✅ |
| [Team Rocket's Factory](src/import.rs#L866) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L402) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1312) | ✅ |
| [Enriching Energy](src/import.rs#L1295) | ✅ |
| [Growing Grass Energy](src/import.rs#L1294) | ✅ |
| [Mist Energy](src/import.rs#L1309) | ✅ |
| [Prism Energy](src/import.rs#L1315) | ✅ |
| [Spiky Energy](src/import.rs#L1306) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1298) | ✅ |

### Pokémon (93/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1497) | [✅](src/import.rs#L1497) | [✅](src/import.rs#L1383) |
| [Alakazam](src/import.rs#L1647) | [✅](src/import.rs#L1647) | [✅](src/import.rs#L1373) |
| [Annihilape](src/import.rs#L1526) | [✅](src/import.rs#L1526) | [✅](src/import.rs#L1339) |
| [Applin](src/import.rs#L1487) | [✅](src/import.rs#L1487) | — |
| [Bayleef](src/import.rs#L1529) | [✅](src/import.rs#L1529) | — |
| [Beldum](src/import.rs#L1509) | [✅](src/import.rs#L1509) | — |
| [Blaziken ex](src/import.rs#L1593) | [✅](src/import.rs#L1593) | [✅](src/import.rs#L1409) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1557) | [✅](src/import.rs#L1557) | [✅](src/import.rs#L1347) |
| [Brute Bonnet](src/import.rs#L1468) | [✅](src/import.rs#L1468) | — |
| [Budew](src/import.rs#L1534) | [✅](src/import.rs#L1534) | — |
| [Buneary](src/import.rs#L1528) | [✅](src/import.rs#L1528) | — |
| [Carvanha](src/import.rs#L1446) | [✅](src/import.rs#L1446) | — |
| [Celebi](src/import.rs#L1527) | [✅](src/import.rs#L1527) | — |
| [Chi-Yu](src/import.rs#L1620) | [✅](src/import.rs#L1620) | — |
| [Chien-Pao](src/import.rs#L1594) | [✅](src/import.rs#L1594) | [✅](src/import.rs#L1412) |
| [Chikorita](src/import.rs#L1530) | [✅](src/import.rs#L1530) | — |
| [Cofagrigus](src/import.rs#L1575) | [✅](src/import.rs#L1575) | — |
| [Combusken](src/import.rs#L1544) | [✅](src/import.rs#L1544) | — |
| [Crustle](src/import.rs#L1659) | [✅](src/import.rs#L1659) | [✅](src/import.rs#L1328) |
| [Dedenne](src/import.rs#L1484) | [✅](src/import.rs#L1484) | — |
| [Dipplin](src/import.rs#L1604) | [✅](src/import.rs#L1604) | [✅](src/import.rs#L1397) |
| [Dragapult ex](src/import.rs#L1491) | [✅](src/import.rs#L1491) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1356) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1498) | [✅](src/import.rs#L1498) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1380) |
| [Dudunsparce ex](src/import.rs#L1459) | [✅](src/import.rs#L1459) | — |
| [Dunsparce](src/import.rs#L1510) | [✅](src/import.rs#L1510) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1384) |
| [Dusknoir](src/import.rs#L1591) | [✅](src/import.rs#L1591) | [✅](src/import.rs#L1385) |
| [Duskull](src/import.rs#L1513) | [✅](src/import.rs#L1513) | — |
| [Dwebble](src/import.rs#L1503) | [✅](src/import.rs#L1503) | — |
| [Elgyem](src/import.rs#L1533) | [✅](src/import.rs#L1533) | — |
| [Enamorus](src/import.rs#L1553) | [✅](src/import.rs#L1553) | — |
| [Fan Rotom](src/import.rs#L1598) | [✅](src/import.rs#L1598) | [✅](src/import.rs#L1422) |
| [Fezandipiti ex](src/import.rs#L1667) | [✅](src/import.rs#L1667) | [✅](src/import.rs#L1374) |
| [Flutter Mane](src/import.rs#L1589) | [✅](src/import.rs#L1589) | [✅](src/import.rs#L1350) |
| [Genesect](src/import.rs#L1639) | [✅](src/import.rs#L1639) | [✅](src/import.rs#L1389) |
| [Genesect ex](src/import.rs#L1592) | [✅](src/import.rs#L1592) | [✅](src/import.rs#L1386) |
| [Goldeen](src/import.rs#L1602) | [✅](src/import.rs#L1602) | [✅](src/import.rs#L1395) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1500) | [✅](src/import.rs#L1500) | [✅](src/import.rs#L1340) |
| [Hydrapple ex](src/import.rs#L1558) | [✅](src/import.rs#L1558) | [✅](src/import.rs#L1341) |
| [Iron Crown ex](src/import.rs#L1518) | [✅](src/import.rs#L1518) | [✅](src/import.rs#L1336) |
| [Iron Leaves ex](src/import.rs#L1597) | [✅](src/import.rs#L1597) | [✅](src/import.rs#L1413) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1369) |
| [Koraidon ex](src/import.rs#L1519) | [✅](src/import.rs#L1519) | — |
| [Kyurem](src/import.rs#L1636) | [✅](src/import.rs#L1636) | [✅](src/import.rs#L1404) |
| [Latias ex](src/import.rs#L1653) | [✅](src/import.rs#L1653) | [✅](src/import.rs#L1327) |
| [Lillie's Clefairy ex](src/import.rs#L1677) | [✅](src/import.rs#L1677) | [✅](src/import.rs#L1331) |
| [Mega Absol ex](src/import.rs#L1565) | [✅](src/import.rs#L1565) | — |
| [Mega Excadrill ex](src/import.rs#L1561) | [✅](src/import.rs#L1561) | — |
| [Mega Kangaskhan ex](src/import.rs#L1663) | [✅](src/import.rs#L1663) | [✅](src/import.rs#L1324) |
| [Mega Lopunny ex](src/import.rs#L1466) | [✅](src/import.rs#L1466) | — |
| [Mega Sharpedo ex](src/import.rs#L1501) | [✅](src/import.rs#L1501) | — |
| [Mega Skarmory ex](src/import.rs#L1579) | [✅](src/import.rs#L1579) | — |
| [Mega Slowbro ex](src/import.rs#L1614) | [✅](src/import.rs#L1614) | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| [Meowth ex](src/import.rs#L1666) | [✅](src/import.rs#L1666) | [✅](src/import.rs#L1368) |
| [Metagross](src/import.rs#L1475) | [✅](src/import.rs#L1475) | — |
| [Metang](src/import.rs#L1654) | [✅](src/import.rs#L1654) | [✅](src/import.rs#L1359) |
| [Moltres](src/import.rs#L1511) | [✅](src/import.rs#L1511) | — |
| [Munkidori](src/import.rs#L1660) | [✅](src/import.rs#L1660) | [✅](src/import.rs#L1362) |
| [N's Darmanitan](src/import.rs#L1456) | [✅](src/import.rs#L1456) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1453) | [✅](src/import.rs#L1453) | — |
| [N's Zekrom](src/import.rs#L1465) | [✅](src/import.rs#L1465) | — |
| [N's Zoroark ex](src/import.rs#L1633) | [✅](src/import.rs#L1633) | [✅](src/import.rs#L1392) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1656) | [✅](src/import.rs#L1656) | [✅](src/import.rs#L1370) |
| [Paldean Tauros](src/import.rs#L1449) | [✅](src/import.rs#L1449) | — |
| [Passimian](src/import.rs#L1462) | [✅](src/import.rs#L1462) | — |
| [Patrat](src/import.rs#L1655) | [✅](src/import.rs#L1655) | [✅](src/import.rs#L1329) |
| Pecharunt | ❌ | ❌ |
| [Pecharunt ex](src/import.rs#L1599) | [✅](src/import.rs#L1599) | [✅](src/import.rs#L1429) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1330) |
| [Rabsca](src/import.rs#L1515) | [✅](src/import.rs#L1515) | [✅](src/import.rs#L1335) |
| [Raging Bolt ex](src/import.rs#L1480) | [✅](src/import.rs#L1480) | — |
| [Rellor](src/import.rs#L1447) | [✅](src/import.rs#L1447) | — |
| [Seaking](src/import.rs#L1603) | [✅](src/import.rs#L1603) | [✅](src/import.rs#L1396) |
| [Shaymin](src/import.rs#L1650) | [✅](src/import.rs#L1650) | [✅](src/import.rs#L1334) |
| [Slowking](src/import.rs#L1504) | [✅](src/import.rs#L1504) | — |
| [Slowpoke](src/import.rs#L1512) | [✅](src/import.rs#L1512) | ❌ |
| [Smoochum](src/import.rs#L1572) | [✅](src/import.rs#L1572) | — |
| [Stunfisk](src/import.rs#L1554) | [✅](src/import.rs#L1554) | — |
| [Tapu Bulu](src/import.rs#L1448) | [✅](src/import.rs#L1448) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1353) |
| [Teal Mask Ogerpon ex](src/import.rs#L1668) | [✅](src/import.rs#L1668) | [✅](src/import.rs#L1377) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1398) |
| [Torchic](src/import.rs#L1514) | [✅](src/import.rs#L1514) | — |
| [Toxel](src/import.rs#L1499) | [✅](src/import.rs#L1499) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1416) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1489) | [✅](src/import.rs#L1489) | — |
| [Yveltal](src/import.rs#L1488) | [✅](src/import.rs#L1488) | — |
| [Zeraora](src/import.rs#L1471) | [✅](src/import.rs#L1471) | — |

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





























