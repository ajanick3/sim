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
| [AZ's Tranquility](src/import.rs#L717) | ✅ |
| [Black Belt's Training](src/import.rs#L725) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L380) | ✅ |
| [Brock's Scouting](src/import.rs#L755) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L657) | ✅ |
| [Crispin](src/import.rs#L563) | ✅ |
| [Cyrano](src/import.rs#L435) | ✅ |
| [Dawn](src/import.rs#L529) | ✅ |
| [Eri](src/import.rs#L748) | ✅ |
| [Gladion's Final Battle](src/import.rs#L729) | ✅ |
| [Gwynn](src/import.rs#L449) | ✅ |
| [Hilda](src/import.rs#L506) | ✅ |
| [Janine's Secret Art](src/import.rs#L779) | ✅ |
| [Judge](src/import.rs#L384) | ✅ |
| [Kieran](src/import.rs#L733) | ✅ |
| [Lana's Aid](src/import.rs#L683) | ✅ |
| [Lillie's Determination](src/import.rs#L385) | ✅ |
| [Morty's Conviction](src/import.rs#L743) | ✅ |
| [N's Plan](src/import.rs#L701) | ✅ |
| [Rosa's Encouragement](src/import.rs#L703) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L697) | ✅ |
| [Surfer](src/import.rs#L721) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L587) | ✅ |
| [Wally's Compassion](src/import.rs#L778) | ✅ |
| [Xerosic's Machinations](src/import.rs#L747) | ✅ |

### Items (26/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L421) | ✅ |
| [Bug Catching Set](src/import.rs#L643) | ✅ |
| [Crushing Hammer](src/import.rs#L420) | ✅ |
| [Dusk Ball](src/import.rs#L809) | ✅ |
| [Energy Recycler](src/import.rs#L891) | ✅ |
| [Energy Retrieval](src/import.rs#L794) | ✅ |
| [Energy Search](src/import.rs#L780) | ✅ |
| [Energy Switch](src/import.rs#L491) | ✅ |
| Enhanced Hammer | ❌ |
| Glass Trumpet | ❌ |
| [Hand Trimmer](src/import.rs#L808) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L679) | ✅ |
| [N's PP Up](src/import.rs#L601) | ✅ |
| [Night Stretcher](src/import.rs#L392) | ✅ |
| [Prime Catcher](src/import.rs#L810) | ✅ |
| [Rare Candy](src/import.rs#L586) | ✅ |
| [Sacred Ash](src/import.rs#L463) | ✅ |
| [Secret Box](src/import.rs#L840) | ✅ |
| [Special Red Card](src/import.rs#L559) | ✅ |
| [Strange Timepiece](src/import.rs#L811) | ✅ |
| [Switch](src/import.rs#L678) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L877) | ✅ |
| [Tera Orb](src/import.rs#L492) | ✅ |
| [Tool Scrapper](src/import.rs#L379) | ✅ |
| [Transformation Tome](src/import.rs#L836) | ✅ |
| [Ultra Ball](src/import.rs#L477) | ✅ |
| [Unfair Stamp](src/import.rs#L671) | ✅ |
| [Wondrous Patch](src/import.rs#L615) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L812) | ✅ |
| [Binding Mochi](src/import.rs#L815) | ✅ |
| [Brave Bangle](src/import.rs#L814) | ✅ |
| [Handheld Fan](src/import.rs#L819) | ✅ |
| [Hero's Cape](src/import.rs#L813) | ✅ |
| [Lillie's Pearl](src/import.rs#L816) | ✅ |
| [Lucky Helmet](src/import.rs#L818) | ✅ |
| [Powerglass](src/import.rs#L820) | ✅ |
| [Punk Helmet](src/import.rs#L817) | ✅ |

### Stadiums (9/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L823) | ✅ |
| Area Zero Underdepths | ❌ |
| Battle Cage | ❌ |
| [Festival Grounds](src/import.rs#L832) | ✅ |
| [Forest of Vitality](src/import.rs#L831) | ✅ |
| [Gravity Mountain](src/import.rs#L821) | ✅ |
| [Jamming Tower](src/import.rs#L829) | ✅ |
| [Lumiose City](src/import.rs#L828) | ✅ |
| [N's Castle](src/import.rs#L822) | ✅ |
| Nighttime Mine | ❌ |
| [Risky Ruins](src/import.rs#L830) | ✅ |
| [Team Rocket's Factory](src/import.rs#L824) | ✅ |
| Team Rocket's Watchtower | ❌ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1184) | ✅ |
| [Enriching Energy](src/import.rs#L1167) | ✅ |
| [Growing Grass Energy](src/import.rs#L1166) | ✅ |
| [Mist Energy](src/import.rs#L1181) | ✅ |
| [Prism Energy](src/import.rs#L1187) | ✅ |
| [Spiky Energy](src/import.rs#L1178) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1170) | ✅ |

### Pokémon (69/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| Abra | [✅](src/import.rs#L1314) | [✅](src/import.rs#L1230) |
| Alakazam | [✅](src/import.rs#L1372) | [✅](src/import.rs#L1220) |
| Annihilape | ❌ | ❌ |
| Applin | [✅](src/import.rs#L1304) | — |
| Bayleef | [✅](src/import.rs#L1334) | — |
| Beldum | [✅](src/import.rs#L1326) | — |
| Blaziken ex | [✅](src/import.rs#L1355) | [✅](src/import.rs#L1236) |
| Bloodmoon Ursaluna ex | ❌ | ❌ |
| Brute Bonnet | ❌ | — |
| Budew | [✅](src/import.rs#L1339) | — |
| Buneary | [✅](src/import.rs#L1333) | — |
| Carvanha | [✅](src/import.rs#L1273) | — |
| Celebi | [✅](src/import.rs#L1332) | — |
| Chi-Yu | ❌ | — |
| Chien-Pao | [✅](src/import.rs#L1356) | [✅](src/import.rs#L1239) |
| Chikorita | [✅](src/import.rs#L1335) | — |
| Cofagrigus | ❌ | — |
| Combusken | [✅](src/import.rs#L1349) | — |
| Crustle | [✅](src/import.rs#L1384) | [✅](src/import.rs#L1200) |
| Dedenne | [✅](src/import.rs#L1301) | — |
| Dipplin | [✅](src/import.rs#L1382) | ❌ |
| Dragapult ex | [✅](src/import.rs#L1308) | — |
| Drakloak | [✅](src/import.rs#L234) | [✅](src/import.rs#L1206) |
| Dreepy | [✅](src/import.rs#L234) | — |
| Drilbur | [✅](src/import.rs#L1315) | ❌ |
| Dudunsparce | [✅](src/import.rs#L234) | [✅](src/import.rs#L1227) |
| Dudunsparce ex | [✅](src/import.rs#L1286) | — |
| Dunsparce | [✅](src/import.rs#L1327) | — |
| Dusclops | [✅](src/import.rs#L234) | [✅](src/import.rs#L1231) |
| Dusknoir | [✅](src/import.rs#L1353) | [✅](src/import.rs#L1232) |
| Duskull | [✅](src/import.rs#L1330) | — |
| Dwebble | [✅](src/import.rs#L1320) | — |
| Elgyem | [✅](src/import.rs#L1338) | — |
| Enamorus | ❌ | — |
| Fan Rotom | [✅](src/import.rs#L1360) | [✅](src/import.rs#L1249) |
| Fezandipiti ex | [✅](src/import.rs#L1392) | [✅](src/import.rs#L1221) |
| Flutter Mane | ❌ | ❌ |
| Genesect | [✅](src/import.rs#L1364) | ❌ |
| Genesect ex | [✅](src/import.rs#L1354) | [✅](src/import.rs#L1233) |
| Goldeen | [✅](src/import.rs#L234) | ❌ |
| Grookey | [✅](src/import.rs#L234) | — |
| Hoothoot | [✅](src/import.rs#L1317) | ❌ |
| Hydrapple ex | ❌ | ❌ |
| Iron Crown ex | ❌ | ❌ |
| Iron Leaves ex | [✅](src/import.rs#L1359) | [✅](src/import.rs#L1240) |
| Kadabra | [✅](src/import.rs#L234) | [✅](src/import.rs#L1219) |
| Koraidon ex | ❌ | — |
| Kyurem | ❌ | ❌ |
| Latias ex | [✅](src/import.rs#L1378) | [✅](src/import.rs#L1199) |
| Lillie's Clefairy ex | ❌ | ❌ |
| Mega Absol ex | ❌ | — |
| Mega Excadrill ex | ❌ | — |
| Mega Kangaskhan ex | [✅](src/import.rs#L1388) | [✅](src/import.rs#L1196) |
| Mega Lopunny ex | ❌ | — |
| Mega Sharpedo ex | [✅](src/import.rs#L1318) | — |
| Mega Skarmory ex | ❌ | — |
| Mega Slowbro ex | ❌ | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| Meowth ex | [✅](src/import.rs#L1391) | [✅](src/import.rs#L1218) |
| Metagross | ❌ | — |
| Metang | [✅](src/import.rs#L1379) | [✅](src/import.rs#L1209) |
| Moltres | [✅](src/import.rs#L1328) | — |
| Munkidori | [✅](src/import.rs#L1385) | [✅](src/import.rs#L1212) |
| N's Darmanitan | [✅](src/import.rs#L1283) | — |
| N's Darumaka | [✅](src/import.rs#L234) | — |
| N's Reshiram | [✅](src/import.rs#L1280) | — |
| N's Zekrom | [✅](src/import.rs#L1292) | — |
| N's Zoroark ex | ❌ | ❌ |
| N's Zorua | [✅](src/import.rs#L234) | — |
| Noctowl | [✅](src/import.rs#L1381) | ❌ |
| Paldean Tauros | [✅](src/import.rs#L1276) | — |
| Passimian | [✅](src/import.rs#L1289) | — |
| Patrat | [✅](src/import.rs#L1380) | [✅](src/import.rs#L1201) |
| Pecharunt | ❌ | ❌ |
| Pecharunt ex | [✅](src/import.rs#L1361) | [✅](src/import.rs#L1256) |
| Psyduck | [✅](src/import.rs#L234) | [✅](src/import.rs#L1202) |
| Rabsca | ❌ | ❌ |
| Raging Bolt ex | ❌ | — |
| Rellor | [✅](src/import.rs#L1274) | — |
| Seaking | [✅](src/import.rs#L1368) | ❌ |
| Shaymin | [✅](src/import.rs#L1375) | ❌ |
| Slowking | [✅](src/import.rs#L1321) | — |
| Slowpoke | [✅](src/import.rs#L1329) | ❌ |
| Smoochum | ❌ | — |
| Stunfisk | ❌ | — |
| Tapu Bulu | [✅](src/import.rs#L1275) | — |
| Tatsugiri | [✅](src/import.rs#L234) | [✅](src/import.rs#L1203) |
| Teal Mask Ogerpon ex | [✅](src/import.rs#L1393) | [✅](src/import.rs#L1224) |
| Thwackey | [✅](src/import.rs#L234) | ❌ |
| Torchic | [✅](src/import.rs#L1331) | — |
| Toxel | [✅](src/import.rs#L1316) | — |
| Toxtricity | [✅](src/import.rs#L234) | [✅](src/import.rs#L1243) |
| Wellspring Mask Ogerpon ex | [✅](src/import.rs#L1306) | — |
| Yveltal | [✅](src/import.rs#L1305) | — |
| Zeraora | [✅](src/import.rs#L1298) | — |

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





























