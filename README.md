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

### Pokémon (81/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1417) | [✅](src/import.rs#L1417) | [✅](src/import.rs#L1323) |
| [Alakazam](src/import.rs#L1506) | [✅](src/import.rs#L1506) | [✅](src/import.rs#L1313) |
| [Annihilape](src/import.rs#L1446) | [✅](src/import.rs#L1446) | [✅](src/import.rs#L1288) |
| [Applin](src/import.rs#L1407) | [✅](src/import.rs#L1407) | — |
| [Bayleef](src/import.rs#L1449) | [✅](src/import.rs#L1449) | — |
| [Beldum](src/import.rs#L1429) | [✅](src/import.rs#L1429) | — |
| [Blaziken ex](src/import.rs#L1489) | [✅](src/import.rs#L1489) | [✅](src/import.rs#L1329) |
| Bloodmoon Ursaluna ex | ❌ | ❌ |
| [Brute Bonnet](src/import.rs#L1388) | [✅](src/import.rs#L1388) | — |
| [Budew](src/import.rs#L1454) | [✅](src/import.rs#L1454) | — |
| [Buneary](src/import.rs#L1448) | [✅](src/import.rs#L1448) | — |
| [Carvanha](src/import.rs#L1366) | [✅](src/import.rs#L1366) | — |
| [Celebi](src/import.rs#L1447) | [✅](src/import.rs#L1447) | — |
| Chi-Yu | ❌ | — |
| [Chien-Pao](src/import.rs#L1490) | [✅](src/import.rs#L1490) | [✅](src/import.rs#L1332) |
| [Chikorita](src/import.rs#L1450) | [✅](src/import.rs#L1450) | — |
| Cofagrigus | ❌ | — |
| [Combusken](src/import.rs#L1464) | [✅](src/import.rs#L1464) | — |
| [Crustle](src/import.rs#L1518) | [✅](src/import.rs#L1518) | [✅](src/import.rs#L1277) |
| [Dedenne](src/import.rs#L1404) | [✅](src/import.rs#L1404) | — |
| [Dipplin](src/import.rs#L1516) | [✅](src/import.rs#L1516) | ❌ |
| [Dragapult ex](src/import.rs#L1411) | [✅](src/import.rs#L1411) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1296) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1418) | [✅](src/import.rs#L1418) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1320) |
| [Dudunsparce ex](src/import.rs#L1379) | [✅](src/import.rs#L1379) | — |
| [Dunsparce](src/import.rs#L1430) | [✅](src/import.rs#L1430) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1324) |
| [Dusknoir](src/import.rs#L1487) | [✅](src/import.rs#L1487) | [✅](src/import.rs#L1325) |
| [Duskull](src/import.rs#L1433) | [✅](src/import.rs#L1433) | — |
| [Dwebble](src/import.rs#L1423) | [✅](src/import.rs#L1423) | — |
| [Elgyem](src/import.rs#L1453) | [✅](src/import.rs#L1453) | — |
| [Enamorus](src/import.rs#L1473) | [✅](src/import.rs#L1473) | — |
| [Fan Rotom](src/import.rs#L1494) | [✅](src/import.rs#L1494) | [✅](src/import.rs#L1342) |
| [Fezandipiti ex](src/import.rs#L1526) | [✅](src/import.rs#L1526) | [✅](src/import.rs#L1314) |
| [Flutter Mane](src/import.rs#L1485) | [✅](src/import.rs#L1485) | [✅](src/import.rs#L1290) |
| [Genesect](src/import.rs#L1498) | [✅](src/import.rs#L1498) | ❌ |
| [Genesect ex](src/import.rs#L1488) | [✅](src/import.rs#L1488) | [✅](src/import.rs#L1326) |
| [Goldeen](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1420) | [✅](src/import.rs#L1420) | [✅](src/import.rs#L1289) |
| Hydrapple ex | ❌ | ❌ |
| [Iron Crown ex](src/import.rs#L1438) | [✅](src/import.rs#L1438) | [✅](src/import.rs#L1285) |
| [Iron Leaves ex](src/import.rs#L1493) | [✅](src/import.rs#L1493) | [✅](src/import.rs#L1333) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1309) |
| [Koraidon ex](src/import.rs#L1439) | [✅](src/import.rs#L1439) | — |
| Kyurem | ❌ | ❌ |
| [Latias ex](src/import.rs#L1512) | [✅](src/import.rs#L1512) | [✅](src/import.rs#L1276) |
| [Lillie's Clefairy ex](src/import.rs#L1533) | [✅](src/import.rs#L1533) | [✅](src/import.rs#L1280) |
| Mega Absol ex | ❌ | — |
| [Mega Excadrill ex](src/import.rs#L1477) | [✅](src/import.rs#L1477) | — |
| [Mega Kangaskhan ex](src/import.rs#L1522) | [✅](src/import.rs#L1522) | [✅](src/import.rs#L1273) |
| Mega Lopunny ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L1421) | [✅](src/import.rs#L1421) | — |
| Mega Skarmory ex | ❌ | — |
| Mega Slowbro ex | ❌ | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| [Meowth ex](src/import.rs#L1525) | [✅](src/import.rs#L1525) | [✅](src/import.rs#L1308) |
| [Metagross](src/import.rs#L1395) | [✅](src/import.rs#L1395) | — |
| [Metang](src/import.rs#L1513) | [✅](src/import.rs#L1513) | [✅](src/import.rs#L1299) |
| [Moltres](src/import.rs#L1431) | [✅](src/import.rs#L1431) | — |
| [Munkidori](src/import.rs#L1519) | [✅](src/import.rs#L1519) | [✅](src/import.rs#L1302) |
| [N's Darmanitan](src/import.rs#L1376) | [✅](src/import.rs#L1376) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1373) | [✅](src/import.rs#L1373) | — |
| [N's Zekrom](src/import.rs#L1385) | [✅](src/import.rs#L1385) | — |
| N's Zoroark ex | ❌ | ❌ |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1515) | [✅](src/import.rs#L1515) | [✅](src/import.rs#L1310) |
| [Paldean Tauros](src/import.rs#L1369) | [✅](src/import.rs#L1369) | — |
| [Passimian](src/import.rs#L1382) | [✅](src/import.rs#L1382) | — |
| [Patrat](src/import.rs#L1514) | [✅](src/import.rs#L1514) | [✅](src/import.rs#L1278) |
| Pecharunt | ❌ | ❌ |
| [Pecharunt ex](src/import.rs#L1495) | [✅](src/import.rs#L1495) | [✅](src/import.rs#L1349) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1279) |
| [Rabsca](src/import.rs#L1435) | [✅](src/import.rs#L1435) | [✅](src/import.rs#L1284) |
| [Raging Bolt ex](src/import.rs#L1400) | [✅](src/import.rs#L1400) | — |
| [Rellor](src/import.rs#L1367) | [✅](src/import.rs#L1367) | — |
| [Seaking](src/import.rs#L1502) | [✅](src/import.rs#L1502) | ❌ |
| [Shaymin](src/import.rs#L1509) | [✅](src/import.rs#L1509) | [✅](src/import.rs#L1283) |
| [Slowking](src/import.rs#L1424) | [✅](src/import.rs#L1424) | — |
| [Slowpoke](src/import.rs#L1432) | [✅](src/import.rs#L1432) | ❌ |
| Smoochum | ❌ | — |
| [Stunfisk](src/import.rs#L1474) | [✅](src/import.rs#L1474) | — |
| [Tapu Bulu](src/import.rs#L1368) | [✅](src/import.rs#L1368) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1293) |
| [Teal Mask Ogerpon ex](src/import.rs#L1527) | [✅](src/import.rs#L1527) | [✅](src/import.rs#L1317) |
| Thwackey | [✅](src/import.rs#L234) | ❌ |
| [Torchic](src/import.rs#L1434) | [✅](src/import.rs#L1434) | — |
| [Toxel](src/import.rs#L1419) | [✅](src/import.rs#L1419) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1336) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1409) | [✅](src/import.rs#L1409) | — |
| [Yveltal](src/import.rs#L1408) | [✅](src/import.rs#L1408) | — |
| [Zeraora](src/import.rs#L1391) | [✅](src/import.rs#L1391) | — |

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





























