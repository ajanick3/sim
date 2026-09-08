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
| [AZ's Tranquility](src/import.rs#L735) | ✅ |
| [Black Belt's Training](src/import.rs#L743) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L380) | ✅ |
| [Brock's Scouting](src/import.rs#L773) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L675) | ✅ |
| [Crispin](src/import.rs#L581) | ✅ |
| [Cyrano](src/import.rs#L453) | ✅ |
| [Dawn](src/import.rs#L547) | ✅ |
| [Eri](src/import.rs#L766) | ✅ |
| [Gladion's Final Battle](src/import.rs#L747) | ✅ |
| [Gwynn](src/import.rs#L467) | ✅ |
| [Hilda](src/import.rs#L524) | ✅ |
| [Janine's Secret Art](src/import.rs#L797) | ✅ |
| [Judge](src/import.rs#L402) | ✅ |
| [Kieran](src/import.rs#L751) | ✅ |
| [Lana's Aid](src/import.rs#L701) | ✅ |
| [Lillie's Determination](src/import.rs#L403) | ✅ |
| [Morty's Conviction](src/import.rs#L761) | ✅ |
| [N's Plan](src/import.rs#L719) | ✅ |
| [Rosa's Encouragement](src/import.rs#L721) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L715) | ✅ |
| [Surfer](src/import.rs#L739) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L605) | ✅ |
| [Wally's Compassion](src/import.rs#L796) | ✅ |
| [Xerosic's Machinations](src/import.rs#L765) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L439) | ✅ |
| [Bug Catching Set](src/import.rs#L661) | ✅ |
| [Crushing Hammer](src/import.rs#L438) | ✅ |
| [Dusk Ball](src/import.rs#L827) | ✅ |
| [Energy Recycler](src/import.rs#L909) | ✅ |
| [Energy Retrieval](src/import.rs#L812) | ✅ |
| [Energy Search](src/import.rs#L798) | ✅ |
| [Energy Switch](src/import.rs#L509) | ✅ |
| [Enhanced Hammer](src/import.rs#L384) | ✅ |
| [Glass Trumpet](src/import.rs#L385) | ✅ |
| [Hand Trimmer](src/import.rs#L826) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L697) | ✅ |
| [N's PP Up](src/import.rs#L619) | ✅ |
| [Night Stretcher](src/import.rs#L410) | ✅ |
| [Prime Catcher](src/import.rs#L828) | ✅ |
| [Rare Candy](src/import.rs#L604) | ✅ |
| [Sacred Ash](src/import.rs#L481) | ✅ |
| [Secret Box](src/import.rs#L858) | ✅ |
| [Special Red Card](src/import.rs#L577) | ✅ |
| [Strange Timepiece](src/import.rs#L829) | ✅ |
| [Switch](src/import.rs#L696) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L895) | ✅ |
| [Tera Orb](src/import.rs#L510) | ✅ |
| [Tool Scrapper](src/import.rs#L379) | ✅ |
| [Transformation Tome](src/import.rs#L854) | ✅ |
| [Ultra Ball](src/import.rs#L495) | ✅ |
| [Unfair Stamp](src/import.rs#L689) | ✅ |
| [Wondrous Patch](src/import.rs#L633) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L830) | ✅ |
| [Binding Mochi](src/import.rs#L833) | ✅ |
| [Brave Bangle](src/import.rs#L832) | ✅ |
| [Handheld Fan](src/import.rs#L837) | ✅ |
| [Hero's Cape](src/import.rs#L831) | ✅ |
| [Lillie's Pearl](src/import.rs#L834) | ✅ |
| [Lucky Helmet](src/import.rs#L836) | ✅ |
| [Powerglass](src/import.rs#L838) | ✅ |
| [Punk Helmet](src/import.rs#L835) | ✅ |

### Stadiums (12/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L841) | ✅ |
| [Area Zero Underdepths](src/import.rs#L400) | ✅ |
| [Battle Cage](src/import.rs#L401) | ✅ |
| [Festival Grounds](src/import.rs#L850) | ✅ |
| [Forest of Vitality](src/import.rs#L849) | ✅ |
| [Gravity Mountain](src/import.rs#L839) | ✅ |
| [Jamming Tower](src/import.rs#L847) | ✅ |
| [Lumiose City](src/import.rs#L846) | ✅ |
| [N's Castle](src/import.rs#L840) | ✅ |
| [Nighttime Mine](src/import.rs#L399) | ✅ |
| [Risky Ruins](src/import.rs#L848) | ✅ |
| [Team Rocket's Factory](src/import.rs#L842) | ✅ |
| Team Rocket's Watchtower | ❌ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1202) | ✅ |
| [Enriching Energy](src/import.rs#L1185) | ✅ |
| [Growing Grass Energy](src/import.rs#L1184) | ✅ |
| [Mist Energy](src/import.rs#L1199) | ✅ |
| [Prism Energy](src/import.rs#L1205) | ✅ |
| [Spiky Energy](src/import.rs#L1196) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1188) | ✅ |

### Pokémon (69/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| Abra | [✅](src/import.rs#L1332) | [✅](src/import.rs#L1248) |
| Alakazam | [✅](src/import.rs#L1390) | [✅](src/import.rs#L1238) |
| Annihilape | ❌ | ❌ |
| Applin | [✅](src/import.rs#L1322) | — |
| Bayleef | [✅](src/import.rs#L1352) | — |
| Beldum | [✅](src/import.rs#L1344) | — |
| Blaziken ex | [✅](src/import.rs#L1373) | [✅](src/import.rs#L1254) |
| Bloodmoon Ursaluna ex | ❌ | ❌ |
| Brute Bonnet | ❌ | — |
| Budew | [✅](src/import.rs#L1357) | — |
| Buneary | [✅](src/import.rs#L1351) | — |
| Carvanha | [✅](src/import.rs#L1291) | — |
| Celebi | [✅](src/import.rs#L1350) | — |
| Chi-Yu | ❌ | — |
| Chien-Pao | [✅](src/import.rs#L1374) | [✅](src/import.rs#L1257) |
| Chikorita | [✅](src/import.rs#L1353) | — |
| Cofagrigus | ❌ | — |
| Combusken | [✅](src/import.rs#L1367) | — |
| Crustle | [✅](src/import.rs#L1402) | [✅](src/import.rs#L1218) |
| Dedenne | [✅](src/import.rs#L1319) | — |
| Dipplin | [✅](src/import.rs#L1400) | ❌ |
| Dragapult ex | [✅](src/import.rs#L1326) | — |
| Drakloak | [✅](src/import.rs#L234) | [✅](src/import.rs#L1224) |
| Dreepy | [✅](src/import.rs#L234) | — |
| Drilbur | [✅](src/import.rs#L1333) | ❌ |
| Dudunsparce | [✅](src/import.rs#L234) | [✅](src/import.rs#L1245) |
| Dudunsparce ex | [✅](src/import.rs#L1304) | — |
| Dunsparce | [✅](src/import.rs#L1345) | — |
| Dusclops | [✅](src/import.rs#L234) | [✅](src/import.rs#L1249) |
| Dusknoir | [✅](src/import.rs#L1371) | [✅](src/import.rs#L1250) |
| Duskull | [✅](src/import.rs#L1348) | — |
| Dwebble | [✅](src/import.rs#L1338) | — |
| Elgyem | [✅](src/import.rs#L1356) | — |
| Enamorus | ❌ | — |
| Fan Rotom | [✅](src/import.rs#L1378) | [✅](src/import.rs#L1267) |
| Fezandipiti ex | [✅](src/import.rs#L1410) | [✅](src/import.rs#L1239) |
| Flutter Mane | ❌ | ❌ |
| Genesect | [✅](src/import.rs#L1382) | ❌ |
| Genesect ex | [✅](src/import.rs#L1372) | [✅](src/import.rs#L1251) |
| Goldeen | [✅](src/import.rs#L234) | ❌ |
| Grookey | [✅](src/import.rs#L234) | — |
| Hoothoot | [✅](src/import.rs#L1335) | ❌ |
| Hydrapple ex | ❌ | ❌ |
| Iron Crown ex | ❌ | ❌ |
| Iron Leaves ex | [✅](src/import.rs#L1377) | [✅](src/import.rs#L1258) |
| Kadabra | [✅](src/import.rs#L234) | [✅](src/import.rs#L1237) |
| Koraidon ex | ❌ | — |
| Kyurem | ❌ | ❌ |
| Latias ex | [✅](src/import.rs#L1396) | [✅](src/import.rs#L1217) |
| Lillie's Clefairy ex | ❌ | ❌ |
| Mega Absol ex | ❌ | — |
| Mega Excadrill ex | ❌ | — |
| Mega Kangaskhan ex | [✅](src/import.rs#L1406) | [✅](src/import.rs#L1214) |
| Mega Lopunny ex | ❌ | — |
| Mega Sharpedo ex | [✅](src/import.rs#L1336) | — |
| Mega Skarmory ex | ❌ | — |
| Mega Slowbro ex | ❌ | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| Meowth ex | [✅](src/import.rs#L1409) | [✅](src/import.rs#L1236) |
| Metagross | ❌ | — |
| Metang | [✅](src/import.rs#L1397) | [✅](src/import.rs#L1227) |
| Moltres | [✅](src/import.rs#L1346) | — |
| Munkidori | [✅](src/import.rs#L1403) | [✅](src/import.rs#L1230) |
| N's Darmanitan | [✅](src/import.rs#L1301) | — |
| N's Darumaka | [✅](src/import.rs#L234) | — |
| N's Reshiram | [✅](src/import.rs#L1298) | — |
| N's Zekrom | [✅](src/import.rs#L1310) | — |
| N's Zoroark ex | ❌ | ❌ |
| N's Zorua | [✅](src/import.rs#L234) | — |
| Noctowl | [✅](src/import.rs#L1399) | ❌ |
| Paldean Tauros | [✅](src/import.rs#L1294) | — |
| Passimian | [✅](src/import.rs#L1307) | — |
| Patrat | [✅](src/import.rs#L1398) | [✅](src/import.rs#L1219) |
| Pecharunt | ❌ | ❌ |
| Pecharunt ex | [✅](src/import.rs#L1379) | [✅](src/import.rs#L1274) |
| Psyduck | [✅](src/import.rs#L234) | [✅](src/import.rs#L1220) |
| Rabsca | ❌ | ❌ |
| Raging Bolt ex | ❌ | — |
| Rellor | [✅](src/import.rs#L1292) | — |
| Seaking | [✅](src/import.rs#L1386) | ❌ |
| Shaymin | [✅](src/import.rs#L1393) | ❌ |
| Slowking | [✅](src/import.rs#L1339) | — |
| Slowpoke | [✅](src/import.rs#L1347) | ❌ |
| Smoochum | ❌ | — |
| Stunfisk | ❌ | — |
| Tapu Bulu | [✅](src/import.rs#L1293) | — |
| Tatsugiri | [✅](src/import.rs#L234) | [✅](src/import.rs#L1221) |
| Teal Mask Ogerpon ex | [✅](src/import.rs#L1411) | [✅](src/import.rs#L1242) |
| Thwackey | [✅](src/import.rs#L234) | ❌ |
| Torchic | [✅](src/import.rs#L1349) | — |
| Toxel | [✅](src/import.rs#L1334) | — |
| Toxtricity | [✅](src/import.rs#L234) | [✅](src/import.rs#L1261) |
| Wellspring Mask Ogerpon ex | [✅](src/import.rs#L1324) | — |
| Yveltal | [✅](src/import.rs#L1323) | — |
| Zeraora | [✅](src/import.rs#L1316) | — |

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





























