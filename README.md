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
| [AZ's Tranquility](src/import.rs#L718) | ✅ |
| [Black Belt's Training](src/import.rs#L726) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L380) | ✅ |
| [Brock's Scouting](src/import.rs#L756) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L658) | ✅ |
| [Crispin](src/import.rs#L564) | ✅ |
| [Cyrano](src/import.rs#L436) | ✅ |
| [Dawn](src/import.rs#L530) | ✅ |
| [Eri](src/import.rs#L749) | ✅ |
| [Gladion's Final Battle](src/import.rs#L730) | ✅ |
| [Gwynn](src/import.rs#L450) | ✅ |
| [Hilda](src/import.rs#L507) | ✅ |
| [Janine's Secret Art](src/import.rs#L780) | ✅ |
| [Judge](src/import.rs#L385) | ✅ |
| [Kieran](src/import.rs#L734) | ✅ |
| [Lana's Aid](src/import.rs#L684) | ✅ |
| [Lillie's Determination](src/import.rs#L386) | ✅ |
| [Morty's Conviction](src/import.rs#L744) | ✅ |
| [N's Plan](src/import.rs#L702) | ✅ |
| [Rosa's Encouragement](src/import.rs#L704) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L698) | ✅ |
| [Surfer](src/import.rs#L722) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L588) | ✅ |
| [Wally's Compassion](src/import.rs#L779) | ✅ |
| [Xerosic's Machinations](src/import.rs#L748) | ✅ |

### Items (27/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L422) | ✅ |
| [Bug Catching Set](src/import.rs#L644) | ✅ |
| [Crushing Hammer](src/import.rs#L421) | ✅ |
| [Dusk Ball](src/import.rs#L810) | ✅ |
| [Energy Recycler](src/import.rs#L892) | ✅ |
| [Energy Retrieval](src/import.rs#L795) | ✅ |
| [Energy Search](src/import.rs#L781) | ✅ |
| [Energy Switch](src/import.rs#L492) | ✅ |
| [Enhanced Hammer](src/import.rs#L384) | ✅ |
| Glass Trumpet | ❌ |
| [Hand Trimmer](src/import.rs#L809) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L680) | ✅ |
| [N's PP Up](src/import.rs#L602) | ✅ |
| [Night Stretcher](src/import.rs#L393) | ✅ |
| [Prime Catcher](src/import.rs#L811) | ✅ |
| [Rare Candy](src/import.rs#L587) | ✅ |
| [Sacred Ash](src/import.rs#L464) | ✅ |
| [Secret Box](src/import.rs#L841) | ✅ |
| [Special Red Card](src/import.rs#L560) | ✅ |
| [Strange Timepiece](src/import.rs#L812) | ✅ |
| [Switch](src/import.rs#L679) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L878) | ✅ |
| [Tera Orb](src/import.rs#L493) | ✅ |
| [Tool Scrapper](src/import.rs#L379) | ✅ |
| [Transformation Tome](src/import.rs#L837) | ✅ |
| [Ultra Ball](src/import.rs#L478) | ✅ |
| [Unfair Stamp](src/import.rs#L672) | ✅ |
| [Wondrous Patch](src/import.rs#L616) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L813) | ✅ |
| [Binding Mochi](src/import.rs#L816) | ✅ |
| [Brave Bangle](src/import.rs#L815) | ✅ |
| [Handheld Fan](src/import.rs#L820) | ✅ |
| [Hero's Cape](src/import.rs#L814) | ✅ |
| [Lillie's Pearl](src/import.rs#L817) | ✅ |
| [Lucky Helmet](src/import.rs#L819) | ✅ |
| [Powerglass](src/import.rs#L821) | ✅ |
| [Punk Helmet](src/import.rs#L818) | ✅ |

### Stadiums (9/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L824) | ✅ |
| Area Zero Underdepths | ❌ |
| Battle Cage | ❌ |
| [Festival Grounds](src/import.rs#L833) | ✅ |
| [Forest of Vitality](src/import.rs#L832) | ✅ |
| [Gravity Mountain](src/import.rs#L822) | ✅ |
| [Jamming Tower](src/import.rs#L830) | ✅ |
| [Lumiose City](src/import.rs#L829) | ✅ |
| [N's Castle](src/import.rs#L823) | ✅ |
| Nighttime Mine | ❌ |
| [Risky Ruins](src/import.rs#L831) | ✅ |
| [Team Rocket's Factory](src/import.rs#L825) | ✅ |
| Team Rocket's Watchtower | ❌ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1185) | ✅ |
| [Enriching Energy](src/import.rs#L1168) | ✅ |
| [Growing Grass Energy](src/import.rs#L1167) | ✅ |
| [Mist Energy](src/import.rs#L1182) | ✅ |
| [Prism Energy](src/import.rs#L1188) | ✅ |
| [Spiky Energy](src/import.rs#L1179) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1171) | ✅ |

### Pokémon (69/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| Abra | [✅](src/import.rs#L1315) | [✅](src/import.rs#L1231) |
| Alakazam | [✅](src/import.rs#L1373) | [✅](src/import.rs#L1221) |
| Annihilape | ❌ | ❌ |
| Applin | [✅](src/import.rs#L1305) | — |
| Bayleef | [✅](src/import.rs#L1335) | — |
| Beldum | [✅](src/import.rs#L1327) | — |
| Blaziken ex | [✅](src/import.rs#L1356) | [✅](src/import.rs#L1237) |
| Bloodmoon Ursaluna ex | ❌ | ❌ |
| Brute Bonnet | ❌ | — |
| Budew | [✅](src/import.rs#L1340) | — |
| Buneary | [✅](src/import.rs#L1334) | — |
| Carvanha | [✅](src/import.rs#L1274) | — |
| Celebi | [✅](src/import.rs#L1333) | — |
| Chi-Yu | ❌ | — |
| Chien-Pao | [✅](src/import.rs#L1357) | [✅](src/import.rs#L1240) |
| Chikorita | [✅](src/import.rs#L1336) | — |
| Cofagrigus | ❌ | — |
| Combusken | [✅](src/import.rs#L1350) | — |
| Crustle | [✅](src/import.rs#L1385) | [✅](src/import.rs#L1201) |
| Dedenne | [✅](src/import.rs#L1302) | — |
| Dipplin | [✅](src/import.rs#L1383) | ❌ |
| Dragapult ex | [✅](src/import.rs#L1309) | — |
| Drakloak | [✅](src/import.rs#L234) | [✅](src/import.rs#L1207) |
| Dreepy | [✅](src/import.rs#L234) | — |
| Drilbur | [✅](src/import.rs#L1316) | ❌ |
| Dudunsparce | [✅](src/import.rs#L234) | [✅](src/import.rs#L1228) |
| Dudunsparce ex | [✅](src/import.rs#L1287) | — |
| Dunsparce | [✅](src/import.rs#L1328) | — |
| Dusclops | [✅](src/import.rs#L234) | [✅](src/import.rs#L1232) |
| Dusknoir | [✅](src/import.rs#L1354) | [✅](src/import.rs#L1233) |
| Duskull | [✅](src/import.rs#L1331) | — |
| Dwebble | [✅](src/import.rs#L1321) | — |
| Elgyem | [✅](src/import.rs#L1339) | — |
| Enamorus | ❌ | — |
| Fan Rotom | [✅](src/import.rs#L1361) | [✅](src/import.rs#L1250) |
| Fezandipiti ex | [✅](src/import.rs#L1393) | [✅](src/import.rs#L1222) |
| Flutter Mane | ❌ | ❌ |
| Genesect | [✅](src/import.rs#L1365) | ❌ |
| Genesect ex | [✅](src/import.rs#L1355) | [✅](src/import.rs#L1234) |
| Goldeen | [✅](src/import.rs#L234) | ❌ |
| Grookey | [✅](src/import.rs#L234) | — |
| Hoothoot | [✅](src/import.rs#L1318) | ❌ |
| Hydrapple ex | ❌ | ❌ |
| Iron Crown ex | ❌ | ❌ |
| Iron Leaves ex | [✅](src/import.rs#L1360) | [✅](src/import.rs#L1241) |
| Kadabra | [✅](src/import.rs#L234) | [✅](src/import.rs#L1220) |
| Koraidon ex | ❌ | — |
| Kyurem | ❌ | ❌ |
| Latias ex | [✅](src/import.rs#L1379) | [✅](src/import.rs#L1200) |
| Lillie's Clefairy ex | ❌ | ❌ |
| Mega Absol ex | ❌ | — |
| Mega Excadrill ex | ❌ | — |
| Mega Kangaskhan ex | [✅](src/import.rs#L1389) | [✅](src/import.rs#L1197) |
| Mega Lopunny ex | ❌ | — |
| Mega Sharpedo ex | [✅](src/import.rs#L1319) | — |
| Mega Skarmory ex | ❌ | — |
| Mega Slowbro ex | ❌ | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| Meowth ex | [✅](src/import.rs#L1392) | [✅](src/import.rs#L1219) |
| Metagross | ❌ | — |
| Metang | [✅](src/import.rs#L1380) | [✅](src/import.rs#L1210) |
| Moltres | [✅](src/import.rs#L1329) | — |
| Munkidori | [✅](src/import.rs#L1386) | [✅](src/import.rs#L1213) |
| N's Darmanitan | [✅](src/import.rs#L1284) | — |
| N's Darumaka | [✅](src/import.rs#L234) | — |
| N's Reshiram | [✅](src/import.rs#L1281) | — |
| N's Zekrom | [✅](src/import.rs#L1293) | — |
| N's Zoroark ex | ❌ | ❌ |
| N's Zorua | [✅](src/import.rs#L234) | — |
| Noctowl | [✅](src/import.rs#L1382) | ❌ |
| Paldean Tauros | [✅](src/import.rs#L1277) | — |
| Passimian | [✅](src/import.rs#L1290) | — |
| Patrat | [✅](src/import.rs#L1381) | [✅](src/import.rs#L1202) |
| Pecharunt | ❌ | ❌ |
| Pecharunt ex | [✅](src/import.rs#L1362) | [✅](src/import.rs#L1257) |
| Psyduck | [✅](src/import.rs#L234) | [✅](src/import.rs#L1203) |
| Rabsca | ❌ | ❌ |
| Raging Bolt ex | ❌ | — |
| Rellor | [✅](src/import.rs#L1275) | — |
| Seaking | [✅](src/import.rs#L1369) | ❌ |
| Shaymin | [✅](src/import.rs#L1376) | ❌ |
| Slowking | [✅](src/import.rs#L1322) | — |
| Slowpoke | [✅](src/import.rs#L1330) | ❌ |
| Smoochum | ❌ | — |
| Stunfisk | ❌ | — |
| Tapu Bulu | [✅](src/import.rs#L1276) | — |
| Tatsugiri | [✅](src/import.rs#L234) | [✅](src/import.rs#L1204) |
| Teal Mask Ogerpon ex | [✅](src/import.rs#L1394) | [✅](src/import.rs#L1225) |
| Thwackey | [✅](src/import.rs#L234) | ❌ |
| Torchic | [✅](src/import.rs#L1332) | — |
| Toxel | [✅](src/import.rs#L1317) | — |
| Toxtricity | [✅](src/import.rs#L234) | [✅](src/import.rs#L1244) |
| Wellspring Mask Ogerpon ex | [✅](src/import.rs#L1307) | — |
| Yveltal | [✅](src/import.rs#L1306) | — |
| Zeraora | [✅](src/import.rs#L1299) | — |

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





























