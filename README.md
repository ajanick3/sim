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

### Supporters (25/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L713) | ✅ |
| [Black Belt's Training](src/import.rs#L721) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| Briar | ❌ |
| [Brock's Scouting](src/import.rs#L751) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L653) | ✅ |
| [Crispin](src/import.rs#L559) | ✅ |
| [Cyrano](src/import.rs#L431) | ✅ |
| [Dawn](src/import.rs#L525) | ✅ |
| [Eri](src/import.rs#L744) | ✅ |
| [Gladion's Final Battle](src/import.rs#L725) | ✅ |
| [Gwynn](src/import.rs#L445) | ✅ |
| [Hilda](src/import.rs#L502) | ✅ |
| [Janine's Secret Art](src/import.rs#L775) | ✅ |
| [Judge](src/import.rs#L380) | ✅ |
| [Kieran](src/import.rs#L729) | ✅ |
| [Lana's Aid](src/import.rs#L679) | ✅ |
| [Lillie's Determination](src/import.rs#L381) | ✅ |
| [Morty's Conviction](src/import.rs#L739) | ✅ |
| [N's Plan](src/import.rs#L697) | ✅ |
| [Rosa's Encouragement](src/import.rs#L699) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L693) | ✅ |
| [Surfer](src/import.rs#L717) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L583) | ✅ |
| [Wally's Compassion](src/import.rs#L774) | ✅ |
| [Xerosic's Machinations](src/import.rs#L743) | ✅ |

### Items (26/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L417) | ✅ |
| [Bug Catching Set](src/import.rs#L639) | ✅ |
| [Crushing Hammer](src/import.rs#L416) | ✅ |
| [Dusk Ball](src/import.rs#L805) | ✅ |
| [Energy Recycler](src/import.rs#L887) | ✅ |
| [Energy Retrieval](src/import.rs#L790) | ✅ |
| [Energy Search](src/import.rs#L776) | ✅ |
| [Energy Switch](src/import.rs#L487) | ✅ |
| Enhanced Hammer | ❌ |
| Glass Trumpet | ❌ |
| [Hand Trimmer](src/import.rs#L804) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L675) | ✅ |
| [N's PP Up](src/import.rs#L597) | ✅ |
| [Night Stretcher](src/import.rs#L388) | ✅ |
| [Prime Catcher](src/import.rs#L806) | ✅ |
| [Rare Candy](src/import.rs#L582) | ✅ |
| [Sacred Ash](src/import.rs#L459) | ✅ |
| [Secret Box](src/import.rs#L836) | ✅ |
| [Special Red Card](src/import.rs#L555) | ✅ |
| [Strange Timepiece](src/import.rs#L807) | ✅ |
| [Switch](src/import.rs#L674) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L873) | ✅ |
| [Tera Orb](src/import.rs#L488) | ✅ |
| [Tool Scrapper](src/import.rs#L379) | ✅ |
| [Transformation Tome](src/import.rs#L832) | ✅ |
| [Ultra Ball](src/import.rs#L473) | ✅ |
| [Unfair Stamp](src/import.rs#L667) | ✅ |
| [Wondrous Patch](src/import.rs#L611) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L808) | ✅ |
| [Binding Mochi](src/import.rs#L811) | ✅ |
| [Brave Bangle](src/import.rs#L810) | ✅ |
| [Handheld Fan](src/import.rs#L815) | ✅ |
| [Hero's Cape](src/import.rs#L809) | ✅ |
| [Lillie's Pearl](src/import.rs#L812) | ✅ |
| [Lucky Helmet](src/import.rs#L814) | ✅ |
| [Powerglass](src/import.rs#L816) | ✅ |
| [Punk Helmet](src/import.rs#L813) | ✅ |

### Stadiums (9/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L819) | ✅ |
| Area Zero Underdepths | ❌ |
| Battle Cage | ❌ |
| [Festival Grounds](src/import.rs#L828) | ✅ |
| [Forest of Vitality](src/import.rs#L827) | ✅ |
| [Gravity Mountain](src/import.rs#L817) | ✅ |
| [Jamming Tower](src/import.rs#L825) | ✅ |
| [Lumiose City](src/import.rs#L824) | ✅ |
| [N's Castle](src/import.rs#L818) | ✅ |
| Nighttime Mine | ❌ |
| [Risky Ruins](src/import.rs#L826) | ✅ |
| [Team Rocket's Factory](src/import.rs#L820) | ✅ |
| Team Rocket's Watchtower | ❌ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1180) | ✅ |
| [Enriching Energy](src/import.rs#L1163) | ✅ |
| [Growing Grass Energy](src/import.rs#L1162) | ✅ |
| [Mist Energy](src/import.rs#L1177) | ✅ |
| [Prism Energy](src/import.rs#L1183) | ✅ |
| [Spiky Energy](src/import.rs#L1174) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1166) | ✅ |

### Pokémon (69/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| Abra | [✅](src/import.rs#L1310) | [✅](src/import.rs#L1226) |
| Alakazam | [✅](src/import.rs#L1368) | [✅](src/import.rs#L1216) |
| Annihilape | ❌ | ❌ |
| Applin | [✅](src/import.rs#L1300) | — |
| Bayleef | [✅](src/import.rs#L1330) | — |
| Beldum | [✅](src/import.rs#L1322) | — |
| Blaziken ex | [✅](src/import.rs#L1351) | [✅](src/import.rs#L1232) |
| Bloodmoon Ursaluna ex | ❌ | ❌ |
| Brute Bonnet | ❌ | — |
| Budew | [✅](src/import.rs#L1335) | — |
| Buneary | [✅](src/import.rs#L1329) | — |
| Carvanha | [✅](src/import.rs#L1269) | — |
| Celebi | [✅](src/import.rs#L1328) | — |
| Chi-Yu | ❌ | — |
| Chien-Pao | [✅](src/import.rs#L1352) | [✅](src/import.rs#L1235) |
| Chikorita | [✅](src/import.rs#L1331) | — |
| Cofagrigus | ❌ | — |
| Combusken | [✅](src/import.rs#L1345) | — |
| Crustle | [✅](src/import.rs#L1380) | [✅](src/import.rs#L1196) |
| Dedenne | [✅](src/import.rs#L1297) | — |
| Dipplin | [✅](src/import.rs#L1378) | ❌ |
| Dragapult ex | [✅](src/import.rs#L1304) | — |
| Drakloak | [✅](src/import.rs#L234) | [✅](src/import.rs#L1202) |
| Dreepy | [✅](src/import.rs#L234) | — |
| Drilbur | [✅](src/import.rs#L1311) | ❌ |
| Dudunsparce | [✅](src/import.rs#L234) | [✅](src/import.rs#L1223) |
| Dudunsparce ex | [✅](src/import.rs#L1282) | — |
| Dunsparce | [✅](src/import.rs#L1323) | — |
| Dusclops | [✅](src/import.rs#L234) | [✅](src/import.rs#L1227) |
| Dusknoir | [✅](src/import.rs#L1349) | [✅](src/import.rs#L1228) |
| Duskull | [✅](src/import.rs#L1326) | — |
| Dwebble | [✅](src/import.rs#L1316) | — |
| Elgyem | [✅](src/import.rs#L1334) | — |
| Enamorus | ❌ | — |
| Fan Rotom | [✅](src/import.rs#L1356) | [✅](src/import.rs#L1245) |
| Fezandipiti ex | [✅](src/import.rs#L1388) | [✅](src/import.rs#L1217) |
| Flutter Mane | ❌ | ❌ |
| Genesect | [✅](src/import.rs#L1360) | ❌ |
| Genesect ex | [✅](src/import.rs#L1350) | [✅](src/import.rs#L1229) |
| Goldeen | [✅](src/import.rs#L234) | ❌ |
| Grookey | [✅](src/import.rs#L234) | — |
| Hoothoot | [✅](src/import.rs#L1313) | ❌ |
| Hydrapple ex | ❌ | ❌ |
| Iron Crown ex | ❌ | ❌ |
| Iron Leaves ex | [✅](src/import.rs#L1355) | [✅](src/import.rs#L1236) |
| Kadabra | [✅](src/import.rs#L234) | [✅](src/import.rs#L1215) |
| Koraidon ex | ❌ | — |
| Kyurem | ❌ | ❌ |
| Latias ex | [✅](src/import.rs#L1374) | [✅](src/import.rs#L1195) |
| Lillie's Clefairy ex | ❌ | ❌ |
| Mega Absol ex | ❌ | — |
| Mega Excadrill ex | ❌ | — |
| Mega Kangaskhan ex | [✅](src/import.rs#L1384) | [✅](src/import.rs#L1192) |
| Mega Lopunny ex | ❌ | — |
| Mega Sharpedo ex | [✅](src/import.rs#L1314) | — |
| Mega Skarmory ex | ❌ | — |
| Mega Slowbro ex | ❌ | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| Meowth ex | [✅](src/import.rs#L1387) | [✅](src/import.rs#L1214) |
| Metagross | ❌ | — |
| Metang | [✅](src/import.rs#L1375) | [✅](src/import.rs#L1205) |
| Moltres | [✅](src/import.rs#L1324) | — |
| Munkidori | [✅](src/import.rs#L1381) | [✅](src/import.rs#L1208) |
| N's Darmanitan | [✅](src/import.rs#L1279) | — |
| N's Darumaka | [✅](src/import.rs#L234) | — |
| N's Reshiram | [✅](src/import.rs#L1276) | — |
| N's Zekrom | [✅](src/import.rs#L1288) | — |
| N's Zoroark ex | ❌ | ❌ |
| N's Zorua | [✅](src/import.rs#L234) | — |
| Noctowl | [✅](src/import.rs#L1377) | ❌ |
| Paldean Tauros | [✅](src/import.rs#L1272) | — |
| Passimian | [✅](src/import.rs#L1285) | — |
| Patrat | [✅](src/import.rs#L1376) | [✅](src/import.rs#L1197) |
| Pecharunt | ❌ | ❌ |
| Pecharunt ex | [✅](src/import.rs#L1357) | [✅](src/import.rs#L1252) |
| Psyduck | [✅](src/import.rs#L234) | [✅](src/import.rs#L1198) |
| Rabsca | ❌ | ❌ |
| Raging Bolt ex | ❌ | — |
| Rellor | [✅](src/import.rs#L1270) | — |
| Seaking | [✅](src/import.rs#L1364) | ❌ |
| Shaymin | [✅](src/import.rs#L1371) | ❌ |
| Slowking | [✅](src/import.rs#L1317) | — |
| Slowpoke | [✅](src/import.rs#L1325) | ❌ |
| Smoochum | ❌ | — |
| Stunfisk | ❌ | — |
| Tapu Bulu | [✅](src/import.rs#L1271) | — |
| Tatsugiri | [✅](src/import.rs#L234) | [✅](src/import.rs#L1199) |
| Teal Mask Ogerpon ex | [✅](src/import.rs#L1389) | [✅](src/import.rs#L1220) |
| Thwackey | [✅](src/import.rs#L234) | ❌ |
| Torchic | [✅](src/import.rs#L1327) | — |
| Toxel | [✅](src/import.rs#L1312) | — |
| Toxtricity | [✅](src/import.rs#L234) | [✅](src/import.rs#L1239) |
| Wellspring Mask Ogerpon ex | [✅](src/import.rs#L1302) | — |
| Yveltal | [✅](src/import.rs#L1301) | — |
| Zeraora | [✅](src/import.rs#L1294) | — |

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





























