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
| [AZ's Tranquility](src/import.rs#L736) | ✅ |
| [Black Belt's Training](src/import.rs#L744) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L380) | ✅ |
| [Brock's Scouting](src/import.rs#L774) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L676) | ✅ |
| [Crispin](src/import.rs#L582) | ✅ |
| [Cyrano](src/import.rs#L454) | ✅ |
| [Dawn](src/import.rs#L548) | ✅ |
| [Eri](src/import.rs#L767) | ✅ |
| [Gladion's Final Battle](src/import.rs#L748) | ✅ |
| [Gwynn](src/import.rs#L468) | ✅ |
| [Hilda](src/import.rs#L525) | ✅ |
| [Janine's Secret Art](src/import.rs#L798) | ✅ |
| [Judge](src/import.rs#L403) | ✅ |
| [Kieran](src/import.rs#L752) | ✅ |
| [Lana's Aid](src/import.rs#L702) | ✅ |
| [Lillie's Determination](src/import.rs#L404) | ✅ |
| [Morty's Conviction](src/import.rs#L762) | ✅ |
| [N's Plan](src/import.rs#L720) | ✅ |
| [Rosa's Encouragement](src/import.rs#L722) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L716) | ✅ |
| [Surfer](src/import.rs#L740) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L606) | ✅ |
| [Wally's Compassion](src/import.rs#L797) | ✅ |
| [Xerosic's Machinations](src/import.rs#L766) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L440) | ✅ |
| [Bug Catching Set](src/import.rs#L662) | ✅ |
| [Crushing Hammer](src/import.rs#L439) | ✅ |
| [Dusk Ball](src/import.rs#L828) | ✅ |
| [Energy Recycler](src/import.rs#L910) | ✅ |
| [Energy Retrieval](src/import.rs#L813) | ✅ |
| [Energy Search](src/import.rs#L799) | ✅ |
| [Energy Switch](src/import.rs#L510) | ✅ |
| [Enhanced Hammer](src/import.rs#L384) | ✅ |
| [Glass Trumpet](src/import.rs#L385) | ✅ |
| [Hand Trimmer](src/import.rs#L827) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L698) | ✅ |
| [N's PP Up](src/import.rs#L620) | ✅ |
| [Night Stretcher](src/import.rs#L411) | ✅ |
| [Prime Catcher](src/import.rs#L829) | ✅ |
| [Rare Candy](src/import.rs#L605) | ✅ |
| [Sacred Ash](src/import.rs#L482) | ✅ |
| [Secret Box](src/import.rs#L859) | ✅ |
| [Special Red Card](src/import.rs#L578) | ✅ |
| [Strange Timepiece](src/import.rs#L830) | ✅ |
| [Switch](src/import.rs#L697) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L896) | ✅ |
| [Tera Orb](src/import.rs#L511) | ✅ |
| [Tool Scrapper](src/import.rs#L379) | ✅ |
| [Transformation Tome](src/import.rs#L855) | ✅ |
| [Ultra Ball](src/import.rs#L496) | ✅ |
| [Unfair Stamp](src/import.rs#L690) | ✅ |
| [Wondrous Patch](src/import.rs#L634) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L831) | ✅ |
| [Binding Mochi](src/import.rs#L834) | ✅ |
| [Brave Bangle](src/import.rs#L833) | ✅ |
| [Handheld Fan](src/import.rs#L838) | ✅ |
| [Hero's Cape](src/import.rs#L832) | ✅ |
| [Lillie's Pearl](src/import.rs#L835) | ✅ |
| [Lucky Helmet](src/import.rs#L837) | ✅ |
| [Powerglass](src/import.rs#L839) | ✅ |
| [Punk Helmet](src/import.rs#L836) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L842) | ✅ |
| [Area Zero Underdepths](src/import.rs#L400) | ✅ |
| [Battle Cage](src/import.rs#L401) | ✅ |
| [Festival Grounds](src/import.rs#L851) | ✅ |
| [Forest of Vitality](src/import.rs#L850) | ✅ |
| [Gravity Mountain](src/import.rs#L840) | ✅ |
| [Jamming Tower](src/import.rs#L848) | ✅ |
| [Lumiose City](src/import.rs#L847) | ✅ |
| [N's Castle](src/import.rs#L841) | ✅ |
| [Nighttime Mine](src/import.rs#L399) | ✅ |
| [Risky Ruins](src/import.rs#L849) | ✅ |
| [Team Rocket's Factory](src/import.rs#L843) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L402) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1261) | ✅ |
| [Enriching Energy](src/import.rs#L1244) | ✅ |
| [Growing Grass Energy](src/import.rs#L1243) | ✅ |
| [Mist Energy](src/import.rs#L1258) | ✅ |
| [Prism Energy](src/import.rs#L1264) | ✅ |
| [Spiky Energy](src/import.rs#L1255) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1247) | ✅ |

### Pokémon (84/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1426) | [✅](src/import.rs#L1426) | [✅](src/import.rs#L1332) |
| [Alakazam](src/import.rs#L1522) | [✅](src/import.rs#L1522) | [✅](src/import.rs#L1322) |
| [Annihilape](src/import.rs#L1455) | [✅](src/import.rs#L1455) | [✅](src/import.rs#L1288) |
| [Applin](src/import.rs#L1416) | [✅](src/import.rs#L1416) | — |
| [Bayleef](src/import.rs#L1458) | [✅](src/import.rs#L1458) | — |
| [Beldum](src/import.rs#L1438) | [✅](src/import.rs#L1438) | — |
| [Blaziken ex](src/import.rs#L1505) | [✅](src/import.rs#L1505) | [✅](src/import.rs#L1338) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1486) | [✅](src/import.rs#L1486) | [✅](src/import.rs#L1296) |
| [Brute Bonnet](src/import.rs#L1397) | [✅](src/import.rs#L1397) | — |
| [Budew](src/import.rs#L1463) | [✅](src/import.rs#L1463) | — |
| [Buneary](src/import.rs#L1457) | [✅](src/import.rs#L1457) | — |
| [Carvanha](src/import.rs#L1375) | [✅](src/import.rs#L1375) | — |
| [Celebi](src/import.rs#L1456) | [✅](src/import.rs#L1456) | — |
| Chi-Yu | ❌ | — |
| [Chien-Pao](src/import.rs#L1506) | [✅](src/import.rs#L1506) | [✅](src/import.rs#L1341) |
| [Chikorita](src/import.rs#L1459) | [✅](src/import.rs#L1459) | — |
| Cofagrigus | ❌ | — |
| [Combusken](src/import.rs#L1473) | [✅](src/import.rs#L1473) | — |
| [Crustle](src/import.rs#L1534) | [✅](src/import.rs#L1534) | [✅](src/import.rs#L1277) |
| [Dedenne](src/import.rs#L1413) | [✅](src/import.rs#L1413) | — |
| [Dipplin](src/import.rs#L1532) | [✅](src/import.rs#L1532) | ❌ |
| [Dragapult ex](src/import.rs#L1420) | [✅](src/import.rs#L1420) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1305) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1427) | [✅](src/import.rs#L1427) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1329) |
| [Dudunsparce ex](src/import.rs#L1388) | [✅](src/import.rs#L1388) | — |
| [Dunsparce](src/import.rs#L1439) | [✅](src/import.rs#L1439) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1333) |
| [Dusknoir](src/import.rs#L1503) | [✅](src/import.rs#L1503) | [✅](src/import.rs#L1334) |
| [Duskull](src/import.rs#L1442) | [✅](src/import.rs#L1442) | — |
| [Dwebble](src/import.rs#L1432) | [✅](src/import.rs#L1432) | — |
| [Elgyem](src/import.rs#L1462) | [✅](src/import.rs#L1462) | — |
| [Enamorus](src/import.rs#L1482) | [✅](src/import.rs#L1482) | — |
| [Fan Rotom](src/import.rs#L1510) | [✅](src/import.rs#L1510) | [✅](src/import.rs#L1351) |
| [Fezandipiti ex](src/import.rs#L1542) | [✅](src/import.rs#L1542) | [✅](src/import.rs#L1323) |
| [Flutter Mane](src/import.rs#L1501) | [✅](src/import.rs#L1501) | [✅](src/import.rs#L1299) |
| [Genesect](src/import.rs#L1514) | [✅](src/import.rs#L1514) | ❌ |
| [Genesect ex](src/import.rs#L1504) | [✅](src/import.rs#L1504) | [✅](src/import.rs#L1335) |
| [Goldeen](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1429) | [✅](src/import.rs#L1429) | [✅](src/import.rs#L1289) |
| [Hydrapple ex](src/import.rs#L1487) | [✅](src/import.rs#L1487) | [✅](src/import.rs#L1290) |
| [Iron Crown ex](src/import.rs#L1447) | [✅](src/import.rs#L1447) | [✅](src/import.rs#L1285) |
| [Iron Leaves ex](src/import.rs#L1509) | [✅](src/import.rs#L1509) | [✅](src/import.rs#L1342) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1318) |
| [Koraidon ex](src/import.rs#L1448) | [✅](src/import.rs#L1448) | — |
| Kyurem | ❌ | ❌ |
| [Latias ex](src/import.rs#L1528) | [✅](src/import.rs#L1528) | [✅](src/import.rs#L1276) |
| [Lillie's Clefairy ex](src/import.rs#L1549) | [✅](src/import.rs#L1549) | [✅](src/import.rs#L1280) |
| Mega Absol ex | ❌ | — |
| [Mega Excadrill ex](src/import.rs#L1490) | [✅](src/import.rs#L1490) | — |
| [Mega Kangaskhan ex](src/import.rs#L1538) | [✅](src/import.rs#L1538) | [✅](src/import.rs#L1273) |
| Mega Lopunny ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L1430) | [✅](src/import.rs#L1430) | — |
| [Mega Skarmory ex](src/import.rs#L1491) | [✅](src/import.rs#L1491) | — |
| Mega Slowbro ex | ❌ | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| [Meowth ex](src/import.rs#L1541) | [✅](src/import.rs#L1541) | [✅](src/import.rs#L1317) |
| [Metagross](src/import.rs#L1404) | [✅](src/import.rs#L1404) | — |
| [Metang](src/import.rs#L1529) | [✅](src/import.rs#L1529) | [✅](src/import.rs#L1308) |
| [Moltres](src/import.rs#L1440) | [✅](src/import.rs#L1440) | — |
| [Munkidori](src/import.rs#L1535) | [✅](src/import.rs#L1535) | [✅](src/import.rs#L1311) |
| [N's Darmanitan](src/import.rs#L1385) | [✅](src/import.rs#L1385) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1382) | [✅](src/import.rs#L1382) | — |
| [N's Zekrom](src/import.rs#L1394) | [✅](src/import.rs#L1394) | — |
| N's Zoroark ex | ❌ | ❌ |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1531) | [✅](src/import.rs#L1531) | [✅](src/import.rs#L1319) |
| [Paldean Tauros](src/import.rs#L1378) | [✅](src/import.rs#L1378) | — |
| [Passimian](src/import.rs#L1391) | [✅](src/import.rs#L1391) | — |
| [Patrat](src/import.rs#L1530) | [✅](src/import.rs#L1530) | [✅](src/import.rs#L1278) |
| Pecharunt | ❌ | ❌ |
| [Pecharunt ex](src/import.rs#L1511) | [✅](src/import.rs#L1511) | [✅](src/import.rs#L1358) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1279) |
| [Rabsca](src/import.rs#L1444) | [✅](src/import.rs#L1444) | [✅](src/import.rs#L1284) |
| [Raging Bolt ex](src/import.rs#L1409) | [✅](src/import.rs#L1409) | — |
| [Rellor](src/import.rs#L1376) | [✅](src/import.rs#L1376) | — |
| [Seaking](src/import.rs#L1518) | [✅](src/import.rs#L1518) | ❌ |
| [Shaymin](src/import.rs#L1525) | [✅](src/import.rs#L1525) | [✅](src/import.rs#L1283) |
| [Slowking](src/import.rs#L1433) | [✅](src/import.rs#L1433) | — |
| [Slowpoke](src/import.rs#L1441) | [✅](src/import.rs#L1441) | ❌ |
| Smoochum | ❌ | — |
| [Stunfisk](src/import.rs#L1483) | [✅](src/import.rs#L1483) | — |
| [Tapu Bulu](src/import.rs#L1377) | [✅](src/import.rs#L1377) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1302) |
| [Teal Mask Ogerpon ex](src/import.rs#L1543) | [✅](src/import.rs#L1543) | [✅](src/import.rs#L1326) |
| Thwackey | [✅](src/import.rs#L234) | ❌ |
| [Torchic](src/import.rs#L1443) | [✅](src/import.rs#L1443) | — |
| [Toxel](src/import.rs#L1428) | [✅](src/import.rs#L1428) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1345) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1418) | [✅](src/import.rs#L1418) | — |
| [Yveltal](src/import.rs#L1417) | [✅](src/import.rs#L1417) | — |
| [Zeraora](src/import.rs#L1400) | [✅](src/import.rs#L1400) | — |

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





























