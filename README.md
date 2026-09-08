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
| [AZ's Tranquility](src/import.rs#L696) | ✅ |
| [Black Belt's Training](src/import.rs#L704) | ✅ |
| [Boss's Orders](src/import.rs#L375) | ✅ |
| Briar | ❌ |
| [Brock's Scouting](src/import.rs#L734) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L636) | ✅ |
| [Crispin](src/import.rs#L542) | ✅ |
| [Cyrano](src/import.rs#L428) | ✅ |
| [Dawn](src/import.rs#L508) | ✅ |
| [Eri](src/import.rs#L727) | ✅ |
| [Gladion's Final Battle](src/import.rs#L708) | ✅ |
| [Gwynn](src/import.rs#L442) | ✅ |
| [Hilda](src/import.rs#L485) | ✅ |
| [Janine's Secret Art](src/import.rs#L758) | ✅ |
| [Judge](src/import.rs#L377) | ✅ |
| [Kieran](src/import.rs#L712) | ✅ |
| [Lana's Aid](src/import.rs#L662) | ✅ |
| [Lillie's Determination](src/import.rs#L378) | ✅ |
| [Morty's Conviction](src/import.rs#L722) | ✅ |
| [N's Plan](src/import.rs#L680) | ✅ |
| [Rosa's Encouragement](src/import.rs#L682) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L676) | ✅ |
| [Surfer](src/import.rs#L700) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L566) | ✅ |
| [Wally's Compassion](src/import.rs#L757) | ✅ |
| [Xerosic's Machinations](src/import.rs#L726) | ✅ |

### Items (25/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L414) | ✅ |
| [Bug Catching Set](src/import.rs#L622) | ✅ |
| [Crushing Hammer](src/import.rs#L413) | ✅ |
| [Dusk Ball](src/import.rs#L788) | ✅ |
| [Energy Recycler](src/import.rs#L870) | ✅ |
| [Energy Retrieval](src/import.rs#L773) | ✅ |
| [Energy Search](src/import.rs#L759) | ✅ |
| [Energy Switch](src/import.rs#L484) | ✅ |
| Enhanced Hammer | ❌ |
| Glass Trumpet | ❌ |
| [Hand Trimmer](src/import.rs#L787) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L658) | ✅ |
| [N's PP Up](src/import.rs#L580) | ✅ |
| [Night Stretcher](src/import.rs#L385) | ✅ |
| [Prime Catcher](src/import.rs#L789) | ✅ |
| [Rare Candy](src/import.rs#L565) | ✅ |
| [Sacred Ash](src/import.rs#L456) | ✅ |
| [Secret Box](src/import.rs#L819) | ✅ |
| [Special Red Card](src/import.rs#L538) | ✅ |
| [Strange Timepiece](src/import.rs#L790) | ✅ |
| [Switch](src/import.rs#L657) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L856) | ✅ |
| Tera Orb | ❌ |
| [Tool Scrapper](src/import.rs#L376) | ✅ |
| [Transformation Tome](src/import.rs#L815) | ✅ |
| [Ultra Ball](src/import.rs#L470) | ✅ |
| [Unfair Stamp](src/import.rs#L650) | ✅ |
| [Wondrous Patch](src/import.rs#L594) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L791) | ✅ |
| [Binding Mochi](src/import.rs#L794) | ✅ |
| [Brave Bangle](src/import.rs#L793) | ✅ |
| [Handheld Fan](src/import.rs#L798) | ✅ |
| [Hero's Cape](src/import.rs#L792) | ✅ |
| [Lillie's Pearl](src/import.rs#L795) | ✅ |
| [Lucky Helmet](src/import.rs#L797) | ✅ |
| [Powerglass](src/import.rs#L799) | ✅ |
| [Punk Helmet](src/import.rs#L796) | ✅ |

### Stadiums (9/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L802) | ✅ |
| Area Zero Underdepths | ❌ |
| Battle Cage | ❌ |
| [Festival Grounds](src/import.rs#L811) | ✅ |
| [Forest of Vitality](src/import.rs#L810) | ✅ |
| [Gravity Mountain](src/import.rs#L800) | ✅ |
| [Jamming Tower](src/import.rs#L808) | ✅ |
| [Lumiose City](src/import.rs#L807) | ✅ |
| [N's Castle](src/import.rs#L801) | ✅ |
| Nighttime Mine | ❌ |
| [Risky Ruins](src/import.rs#L809) | ✅ |
| [Team Rocket's Factory](src/import.rs#L803) | ✅ |
| Team Rocket's Watchtower | ❌ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1021) | ✅ |
| [Enriching Energy](src/import.rs#L1004) | ✅ |
| [Growing Grass Energy](src/import.rs#L1003) | ✅ |
| [Mist Energy](src/import.rs#L1018) | ✅ |
| [Prism Energy](src/import.rs#L1024) | ✅ |
| [Spiky Energy](src/import.rs#L1015) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1007) | ✅ |

### Pokémon (69/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| Abra | [✅](src/import.rs#L1151) | [✅](src/import.rs#L1067) |
| Alakazam | [✅](src/import.rs#L1209) | [✅](src/import.rs#L1057) |
| Annihilape | ❌ | ❌ |
| Applin | [✅](src/import.rs#L1141) | — |
| Bayleef | [✅](src/import.rs#L1171) | — |
| Beldum | [✅](src/import.rs#L1163) | — |
| Blaziken ex | [✅](src/import.rs#L1192) | [✅](src/import.rs#L1073) |
| Bloodmoon Ursaluna ex | ❌ | ❌ |
| Brute Bonnet | ❌ | — |
| Budew | [✅](src/import.rs#L1176) | — |
| Buneary | [✅](src/import.rs#L1170) | — |
| Carvanha | [✅](src/import.rs#L1110) | — |
| Celebi | [✅](src/import.rs#L1169) | — |
| Chi-Yu | ❌ | — |
| Chien-Pao | [✅](src/import.rs#L1193) | [✅](src/import.rs#L1076) |
| Chikorita | [✅](src/import.rs#L1172) | — |
| Cofagrigus | ❌ | — |
| Combusken | [✅](src/import.rs#L1186) | — |
| Crustle | [✅](src/import.rs#L1221) | [✅](src/import.rs#L1037) |
| Dedenne | [✅](src/import.rs#L1138) | — |
| Dipplin | [✅](src/import.rs#L1219) | ❌ |
| Dragapult ex | [✅](src/import.rs#L1145) | — |
| Drakloak | [✅](src/import.rs#L234) | [✅](src/import.rs#L1043) |
| Dreepy | [✅](src/import.rs#L234) | — |
| Drilbur | [✅](src/import.rs#L1152) | ❌ |
| Dudunsparce | [✅](src/import.rs#L234) | [✅](src/import.rs#L1064) |
| Dudunsparce ex | [✅](src/import.rs#L1123) | — |
| Dunsparce | [✅](src/import.rs#L1164) | — |
| Dusclops | [✅](src/import.rs#L234) | [✅](src/import.rs#L1068) |
| Dusknoir | [✅](src/import.rs#L1190) | [✅](src/import.rs#L1069) |
| Duskull | [✅](src/import.rs#L1167) | — |
| Dwebble | [✅](src/import.rs#L1157) | — |
| Elgyem | [✅](src/import.rs#L1175) | — |
| Enamorus | ❌ | — |
| Fan Rotom | [✅](src/import.rs#L1197) | [✅](src/import.rs#L1086) |
| Fezandipiti ex | [✅](src/import.rs#L1229) | [✅](src/import.rs#L1058) |
| Flutter Mane | ❌ | ❌ |
| Genesect | [✅](src/import.rs#L1201) | ❌ |
| Genesect ex | [✅](src/import.rs#L1191) | [✅](src/import.rs#L1070) |
| Goldeen | [✅](src/import.rs#L234) | ❌ |
| Grookey | [✅](src/import.rs#L234) | — |
| Hoothoot | [✅](src/import.rs#L1154) | ❌ |
| Hydrapple ex | ❌ | ❌ |
| Iron Crown ex | ❌ | ❌ |
| Iron Leaves ex | [✅](src/import.rs#L1196) | [✅](src/import.rs#L1077) |
| Kadabra | [✅](src/import.rs#L234) | [✅](src/import.rs#L1056) |
| Koraidon ex | ❌ | — |
| Kyurem | ❌ | ❌ |
| Latias ex | [✅](src/import.rs#L1215) | [✅](src/import.rs#L1036) |
| Lillie's Clefairy ex | ❌ | ❌ |
| Mega Absol ex | ❌ | — |
| Mega Excadrill ex | ❌ | — |
| Mega Kangaskhan ex | [✅](src/import.rs#L1225) | [✅](src/import.rs#L1033) |
| Mega Lopunny ex | ❌ | — |
| Mega Sharpedo ex | [✅](src/import.rs#L1155) | — |
| Mega Skarmory ex | ❌ | — |
| Mega Slowbro ex | ❌ | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| Meowth ex | [✅](src/import.rs#L1228) | [✅](src/import.rs#L1055) |
| Metagross | ❌ | — |
| Metang | [✅](src/import.rs#L1216) | [✅](src/import.rs#L1046) |
| Moltres | [✅](src/import.rs#L1165) | — |
| Munkidori | [✅](src/import.rs#L1222) | [✅](src/import.rs#L1049) |
| N's Darmanitan | [✅](src/import.rs#L1120) | — |
| N's Darumaka | [✅](src/import.rs#L234) | — |
| N's Reshiram | [✅](src/import.rs#L1117) | — |
| N's Zekrom | [✅](src/import.rs#L1129) | — |
| N's Zoroark ex | ❌ | ❌ |
| N's Zorua | [✅](src/import.rs#L234) | — |
| Noctowl | [✅](src/import.rs#L1218) | ❌ |
| Paldean Tauros | [✅](src/import.rs#L1113) | — |
| Passimian | [✅](src/import.rs#L1126) | — |
| Patrat | [✅](src/import.rs#L1217) | [✅](src/import.rs#L1038) |
| Pecharunt | ❌ | ❌ |
| Pecharunt ex | [✅](src/import.rs#L1198) | [✅](src/import.rs#L1093) |
| Psyduck | [✅](src/import.rs#L234) | [✅](src/import.rs#L1039) |
| Rabsca | ❌ | ❌ |
| Raging Bolt ex | ❌ | — |
| Rellor | [✅](src/import.rs#L1111) | — |
| Seaking | [✅](src/import.rs#L1205) | ❌ |
| Shaymin | [✅](src/import.rs#L1212) | ❌ |
| Slowking | [✅](src/import.rs#L1158) | — |
| Slowpoke | [✅](src/import.rs#L1166) | ❌ |
| Smoochum | ❌ | — |
| Stunfisk | ❌ | — |
| Tapu Bulu | [✅](src/import.rs#L1112) | — |
| Tatsugiri | [✅](src/import.rs#L234) | [✅](src/import.rs#L1040) |
| Teal Mask Ogerpon ex | [✅](src/import.rs#L1230) | [✅](src/import.rs#L1061) |
| Thwackey | [✅](src/import.rs#L234) | ❌ |
| Torchic | [✅](src/import.rs#L1168) | — |
| Toxel | [✅](src/import.rs#L1153) | — |
| Toxtricity | [✅](src/import.rs#L234) | [✅](src/import.rs#L1080) |
| Wellspring Mask Ogerpon ex | [✅](src/import.rs#L1143) | — |
| Yveltal | [✅](src/import.rs#L1142) | — |
| Zeraora | [✅](src/import.rs#L1135) | — |

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





























