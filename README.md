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
| [AZ's Tranquility](src/import.rs#L734) | ✅ |
| [Black Belt's Training](src/import.rs#L742) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L380) | ✅ |
| [Brock's Scouting](src/import.rs#L772) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L674) | ✅ |
| [Crispin](src/import.rs#L580) | ✅ |
| [Cyrano](src/import.rs#L452) | ✅ |
| [Dawn](src/import.rs#L546) | ✅ |
| [Eri](src/import.rs#L765) | ✅ |
| [Gladion's Final Battle](src/import.rs#L746) | ✅ |
| [Gwynn](src/import.rs#L466) | ✅ |
| [Hilda](src/import.rs#L523) | ✅ |
| [Janine's Secret Art](src/import.rs#L796) | ✅ |
| [Judge](src/import.rs#L401) | ✅ |
| [Kieran](src/import.rs#L750) | ✅ |
| [Lana's Aid](src/import.rs#L700) | ✅ |
| [Lillie's Determination](src/import.rs#L402) | ✅ |
| [Morty's Conviction](src/import.rs#L760) | ✅ |
| [N's Plan](src/import.rs#L718) | ✅ |
| [Rosa's Encouragement](src/import.rs#L720) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L714) | ✅ |
| [Surfer](src/import.rs#L738) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L604) | ✅ |
| [Wally's Compassion](src/import.rs#L795) | ✅ |
| [Xerosic's Machinations](src/import.rs#L764) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L438) | ✅ |
| [Bug Catching Set](src/import.rs#L660) | ✅ |
| [Crushing Hammer](src/import.rs#L437) | ✅ |
| [Dusk Ball](src/import.rs#L826) | ✅ |
| [Energy Recycler](src/import.rs#L908) | ✅ |
| [Energy Retrieval](src/import.rs#L811) | ✅ |
| [Energy Search](src/import.rs#L797) | ✅ |
| [Energy Switch](src/import.rs#L508) | ✅ |
| [Enhanced Hammer](src/import.rs#L384) | ✅ |
| [Glass Trumpet](src/import.rs#L385) | ✅ |
| [Hand Trimmer](src/import.rs#L825) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L696) | ✅ |
| [N's PP Up](src/import.rs#L618) | ✅ |
| [Night Stretcher](src/import.rs#L409) | ✅ |
| [Prime Catcher](src/import.rs#L827) | ✅ |
| [Rare Candy](src/import.rs#L603) | ✅ |
| [Sacred Ash](src/import.rs#L480) | ✅ |
| [Secret Box](src/import.rs#L857) | ✅ |
| [Special Red Card](src/import.rs#L576) | ✅ |
| [Strange Timepiece](src/import.rs#L828) | ✅ |
| [Switch](src/import.rs#L695) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L894) | ✅ |
| [Tera Orb](src/import.rs#L509) | ✅ |
| [Tool Scrapper](src/import.rs#L379) | ✅ |
| [Transformation Tome](src/import.rs#L853) | ✅ |
| [Ultra Ball](src/import.rs#L494) | ✅ |
| [Unfair Stamp](src/import.rs#L688) | ✅ |
| [Wondrous Patch](src/import.rs#L632) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L829) | ✅ |
| [Binding Mochi](src/import.rs#L832) | ✅ |
| [Brave Bangle](src/import.rs#L831) | ✅ |
| [Handheld Fan](src/import.rs#L836) | ✅ |
| [Hero's Cape](src/import.rs#L830) | ✅ |
| [Lillie's Pearl](src/import.rs#L833) | ✅ |
| [Lucky Helmet](src/import.rs#L835) | ✅ |
| [Powerglass](src/import.rs#L837) | ✅ |
| [Punk Helmet](src/import.rs#L834) | ✅ |

### Stadiums (11/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L840) | ✅ |
| [Area Zero Underdepths](src/import.rs#L400) | ✅ |
| Battle Cage | ❌ |
| [Festival Grounds](src/import.rs#L849) | ✅ |
| [Forest of Vitality](src/import.rs#L848) | ✅ |
| [Gravity Mountain](src/import.rs#L838) | ✅ |
| [Jamming Tower](src/import.rs#L846) | ✅ |
| [Lumiose City](src/import.rs#L845) | ✅ |
| [N's Castle](src/import.rs#L839) | ✅ |
| [Nighttime Mine](src/import.rs#L399) | ✅ |
| [Risky Ruins](src/import.rs#L847) | ✅ |
| [Team Rocket's Factory](src/import.rs#L841) | ✅ |
| Team Rocket's Watchtower | ❌ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1201) | ✅ |
| [Enriching Energy](src/import.rs#L1184) | ✅ |
| [Growing Grass Energy](src/import.rs#L1183) | ✅ |
| [Mist Energy](src/import.rs#L1198) | ✅ |
| [Prism Energy](src/import.rs#L1204) | ✅ |
| [Spiky Energy](src/import.rs#L1195) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1187) | ✅ |

### Pokémon (69/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| Abra | [✅](src/import.rs#L1331) | [✅](src/import.rs#L1247) |
| Alakazam | [✅](src/import.rs#L1389) | [✅](src/import.rs#L1237) |
| Annihilape | ❌ | ❌ |
| Applin | [✅](src/import.rs#L1321) | — |
| Bayleef | [✅](src/import.rs#L1351) | — |
| Beldum | [✅](src/import.rs#L1343) | — |
| Blaziken ex | [✅](src/import.rs#L1372) | [✅](src/import.rs#L1253) |
| Bloodmoon Ursaluna ex | ❌ | ❌ |
| Brute Bonnet | ❌ | — |
| Budew | [✅](src/import.rs#L1356) | — |
| Buneary | [✅](src/import.rs#L1350) | — |
| Carvanha | [✅](src/import.rs#L1290) | — |
| Celebi | [✅](src/import.rs#L1349) | — |
| Chi-Yu | ❌ | — |
| Chien-Pao | [✅](src/import.rs#L1373) | [✅](src/import.rs#L1256) |
| Chikorita | [✅](src/import.rs#L1352) | — |
| Cofagrigus | ❌ | — |
| Combusken | [✅](src/import.rs#L1366) | — |
| Crustle | [✅](src/import.rs#L1401) | [✅](src/import.rs#L1217) |
| Dedenne | [✅](src/import.rs#L1318) | — |
| Dipplin | [✅](src/import.rs#L1399) | ❌ |
| Dragapult ex | [✅](src/import.rs#L1325) | — |
| Drakloak | [✅](src/import.rs#L234) | [✅](src/import.rs#L1223) |
| Dreepy | [✅](src/import.rs#L234) | — |
| Drilbur | [✅](src/import.rs#L1332) | ❌ |
| Dudunsparce | [✅](src/import.rs#L234) | [✅](src/import.rs#L1244) |
| Dudunsparce ex | [✅](src/import.rs#L1303) | — |
| Dunsparce | [✅](src/import.rs#L1344) | — |
| Dusclops | [✅](src/import.rs#L234) | [✅](src/import.rs#L1248) |
| Dusknoir | [✅](src/import.rs#L1370) | [✅](src/import.rs#L1249) |
| Duskull | [✅](src/import.rs#L1347) | — |
| Dwebble | [✅](src/import.rs#L1337) | — |
| Elgyem | [✅](src/import.rs#L1355) | — |
| Enamorus | ❌ | — |
| Fan Rotom | [✅](src/import.rs#L1377) | [✅](src/import.rs#L1266) |
| Fezandipiti ex | [✅](src/import.rs#L1409) | [✅](src/import.rs#L1238) |
| Flutter Mane | ❌ | ❌ |
| Genesect | [✅](src/import.rs#L1381) | ❌ |
| Genesect ex | [✅](src/import.rs#L1371) | [✅](src/import.rs#L1250) |
| Goldeen | [✅](src/import.rs#L234) | ❌ |
| Grookey | [✅](src/import.rs#L234) | — |
| Hoothoot | [✅](src/import.rs#L1334) | ❌ |
| Hydrapple ex | ❌ | ❌ |
| Iron Crown ex | ❌ | ❌ |
| Iron Leaves ex | [✅](src/import.rs#L1376) | [✅](src/import.rs#L1257) |
| Kadabra | [✅](src/import.rs#L234) | [✅](src/import.rs#L1236) |
| Koraidon ex | ❌ | — |
| Kyurem | ❌ | ❌ |
| Latias ex | [✅](src/import.rs#L1395) | [✅](src/import.rs#L1216) |
| Lillie's Clefairy ex | ❌ | ❌ |
| Mega Absol ex | ❌ | — |
| Mega Excadrill ex | ❌ | — |
| Mega Kangaskhan ex | [✅](src/import.rs#L1405) | [✅](src/import.rs#L1213) |
| Mega Lopunny ex | ❌ | — |
| Mega Sharpedo ex | [✅](src/import.rs#L1335) | — |
| Mega Skarmory ex | ❌ | — |
| Mega Slowbro ex | ❌ | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| Meowth ex | [✅](src/import.rs#L1408) | [✅](src/import.rs#L1235) |
| Metagross | ❌ | — |
| Metang | [✅](src/import.rs#L1396) | [✅](src/import.rs#L1226) |
| Moltres | [✅](src/import.rs#L1345) | — |
| Munkidori | [✅](src/import.rs#L1402) | [✅](src/import.rs#L1229) |
| N's Darmanitan | [✅](src/import.rs#L1300) | — |
| N's Darumaka | [✅](src/import.rs#L234) | — |
| N's Reshiram | [✅](src/import.rs#L1297) | — |
| N's Zekrom | [✅](src/import.rs#L1309) | — |
| N's Zoroark ex | ❌ | ❌ |
| N's Zorua | [✅](src/import.rs#L234) | — |
| Noctowl | [✅](src/import.rs#L1398) | ❌ |
| Paldean Tauros | [✅](src/import.rs#L1293) | — |
| Passimian | [✅](src/import.rs#L1306) | — |
| Patrat | [✅](src/import.rs#L1397) | [✅](src/import.rs#L1218) |
| Pecharunt | ❌ | ❌ |
| Pecharunt ex | [✅](src/import.rs#L1378) | [✅](src/import.rs#L1273) |
| Psyduck | [✅](src/import.rs#L234) | [✅](src/import.rs#L1219) |
| Rabsca | ❌ | ❌ |
| Raging Bolt ex | ❌ | — |
| Rellor | [✅](src/import.rs#L1291) | — |
| Seaking | [✅](src/import.rs#L1385) | ❌ |
| Shaymin | [✅](src/import.rs#L1392) | ❌ |
| Slowking | [✅](src/import.rs#L1338) | — |
| Slowpoke | [✅](src/import.rs#L1346) | ❌ |
| Smoochum | ❌ | — |
| Stunfisk | ❌ | — |
| Tapu Bulu | [✅](src/import.rs#L1292) | — |
| Tatsugiri | [✅](src/import.rs#L234) | [✅](src/import.rs#L1220) |
| Teal Mask Ogerpon ex | [✅](src/import.rs#L1410) | [✅](src/import.rs#L1241) |
| Thwackey | [✅](src/import.rs#L234) | ❌ |
| Torchic | [✅](src/import.rs#L1348) | — |
| Toxel | [✅](src/import.rs#L1333) | — |
| Toxtricity | [✅](src/import.rs#L234) | [✅](src/import.rs#L1260) |
| Wellspring Mask Ogerpon ex | [✅](src/import.rs#L1323) | — |
| Yveltal | [✅](src/import.rs#L1322) | — |
| Zeraora | [✅](src/import.rs#L1315) | — |

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





























