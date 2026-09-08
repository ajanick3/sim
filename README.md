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
of Trainer, and the first Pokémon Abilities.

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

## Card progress

Which cards in `decks/` the engine plays today, by kind, in the order the
Trainers effort takes them: Supporters, Items, Tools, Stadiums, Pokémon.
A Pokémon carries two separate columns, Attacks and Ability, since
Milestone 11 and Milestone 8 make separate progress on the same
species — a card plays only once both read (`—` marks a Pokémon
printed with no Ability at all). `cargo run --bin progress_table`
regenerates this section from `data/cards.json` and the committed
### Supporters (25/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L683) | ✅ |
| [Black Belt's Training](src/import.rs#L691) | ✅ |
| [Boss's Orders](src/import.rs#L362) | ✅ |
| Briar | ❌ |
| [Brock's Scouting](src/import.rs#L721) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L623) | ✅ |
| [Crispin](src/import.rs#L529) | ✅ |
| [Cyrano](src/import.rs#L415) | ✅ |
| [Dawn](src/import.rs#L495) | ✅ |
| [Eri](src/import.rs#L714) | ✅ |
| [Gladion's Final Battle](src/import.rs#L695) | ✅ |
| [Gwynn](src/import.rs#L429) | ✅ |
| [Hilda](src/import.rs#L472) | ✅ |
| [Janine's Secret Art](src/import.rs#L745) | ✅ |
| [Judge](src/import.rs#L364) | ✅ |
| [Kieran](src/import.rs#L699) | ✅ |
| [Lana's Aid](src/import.rs#L649) | ✅ |
| [Lillie's Determination](src/import.rs#L365) | ✅ |
| [Morty's Conviction](src/import.rs#L709) | ✅ |
| [N's Plan](src/import.rs#L667) | ✅ |
| [Rosa's Encouragement](src/import.rs#L669) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L663) | ✅ |
| [Surfer](src/import.rs#L687) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L553) | ✅ |
| [Wally's Compassion](src/import.rs#L744) | ✅ |
| [Xerosic's Machinations](src/import.rs#L713) | ✅ |

### Items (25/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L401) | ✅ |
| [Bug Catching Set](src/import.rs#L609) | ✅ |
| [Crushing Hammer](src/import.rs#L400) | ✅ |
| [Dusk Ball](src/import.rs#L775) | ✅ |
| [Energy Recycler](src/import.rs#L857) | ✅ |
| [Energy Retrieval](src/import.rs#L760) | ✅ |
| [Energy Search](src/import.rs#L746) | ✅ |
| [Energy Switch](src/import.rs#L471) | ✅ |
| Enhanced Hammer | ❌ |
| Glass Trumpet | ❌ |
| [Hand Trimmer](src/import.rs#L774) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L645) | ✅ |
| [N's PP Up](src/import.rs#L567) | ✅ |
| [Night Stretcher](src/import.rs#L372) | ✅ |
| [Prime Catcher](src/import.rs#L776) | ✅ |
| [Rare Candy](src/import.rs#L552) | ✅ |
| [Sacred Ash](src/import.rs#L443) | ✅ |
| [Secret Box](src/import.rs#L806) | ✅ |
| [Special Red Card](src/import.rs#L525) | ✅ |
| [Strange Timepiece](src/import.rs#L777) | ✅ |
| [Switch](src/import.rs#L644) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L843) | ✅ |
| Tera Orb | ❌ |
| [Tool Scrapper](src/import.rs#L363) | ✅ |
| [Transformation Tome](src/import.rs#L802) | ✅ |
| [Ultra Ball](src/import.rs#L457) | ✅ |
| [Unfair Stamp](src/import.rs#L637) | ✅ |
| [Wondrous Patch](src/import.rs#L581) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L778) | ✅ |
| [Binding Mochi](src/import.rs#L781) | ✅ |
| [Brave Bangle](src/import.rs#L780) | ✅ |
| [Handheld Fan](src/import.rs#L785) | ✅ |
| [Hero's Cape](src/import.rs#L779) | ✅ |
| [Lillie's Pearl](src/import.rs#L782) | ✅ |
| [Lucky Helmet](src/import.rs#L784) | ✅ |
| [Powerglass](src/import.rs#L786) | ✅ |
| [Punk Helmet](src/import.rs#L783) | ✅ |

### Stadiums (9/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L789) | ✅ |
| Area Zero Underdepths | ❌ |
| Battle Cage | ❌ |
| [Festival Grounds](src/import.rs#L798) | ✅ |
| [Forest of Vitality](src/import.rs#L797) | ✅ |
| [Gravity Mountain](src/import.rs#L787) | ✅ |
| [Jamming Tower](src/import.rs#L795) | ✅ |
| [Lumiose City](src/import.rs#L794) | ✅ |
| [N's Castle](src/import.rs#L788) | ✅ |
| Nighttime Mine | ❌ |
| [Risky Ruins](src/import.rs#L796) | ✅ |
| [Team Rocket's Factory](src/import.rs#L790) | ✅ |
| Team Rocket's Watchtower | ❌ |

### Pokémon (58/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| Abra | [✅](src/import.rs#L1084) | [✅](src/import.rs#L1000) |
| Alakazam | [✅](src/import.rs#L1137) | [✅](src/import.rs#L990) |
| Annihilape | ❌ | ❌ |
| Applin | [✅](src/import.rs#L1074) | — |
| Bayleef | [✅](src/import.rs#L1104) | — |
| Beldum | [✅](src/import.rs#L1096) | — |
| Blaziken ex | [✅](src/import.rs#L1125) | [✅](src/import.rs#L1006) |
| Bloodmoon Ursaluna ex | ❌ | ❌ |
| Brute Bonnet | ❌ | — |
| Budew | [✅](src/import.rs#L1109) | — |
| Buneary | [✅](src/import.rs#L1103) | — |
| Carvanha | [✅](src/import.rs#L1043) | — |
| Celebi | [✅](src/import.rs#L1102) | — |
| Chi-Yu | ❌ | — |
| Chien-Pao | [✅](src/import.rs#L1126) | [✅](src/import.rs#L1009) |
| Chikorita | [✅](src/import.rs#L1105) | — |
| Cofagrigus | ❌ | — |
| Combusken | [✅](src/import.rs#L1119) | — |
| Crustle | ❌ | ❌ |
| Dedenne | [✅](src/import.rs#L1071) | — |
| Dipplin | ❌ | ❌ |
| Dragapult ex | [✅](src/import.rs#L1078) | — |
| Drakloak | [✅](src/import.rs#L234) | ❌ |
| Dreepy | [✅](src/import.rs#L234) | — |
| Drilbur | [✅](src/import.rs#L1085) | ❌ |
| Dudunsparce | [✅](src/import.rs#L234) | [✅](src/import.rs#L997) |
| Dudunsparce ex | [✅](src/import.rs#L1056) | — |
| Dunsparce | [✅](src/import.rs#L1097) | — |
| Dusclops | [✅](src/import.rs#L234) | [✅](src/import.rs#L1001) |
| Dusknoir | [✅](src/import.rs#L1123) | [✅](src/import.rs#L1002) |
| Duskull | [✅](src/import.rs#L1100) | — |
| Dwebble | [✅](src/import.rs#L1090) | — |
| Elgyem | [✅](src/import.rs#L1108) | — |
| Enamorus | ❌ | — |
| Fan Rotom | [✅](src/import.rs#L1130) | [✅](src/import.rs#L1019) |
| Fezandipiti ex | [✅](src/import.rs#L1144) | [✅](src/import.rs#L991) |
| Flutter Mane | ❌ | ❌ |
| Genesect | [✅](src/import.rs#L1134) | ❌ |
| Genesect ex | [✅](src/import.rs#L1124) | [✅](src/import.rs#L1003) |
| Goldeen | [✅](src/import.rs#L234) | ❌ |
| Grookey | [✅](src/import.rs#L234) | — |
| Hoothoot | [✅](src/import.rs#L1087) | ❌ |
| Hydrapple ex | ❌ | ❌ |
| Iron Crown ex | ❌ | ❌ |
| Iron Leaves ex | [✅](src/import.rs#L1129) | [✅](src/import.rs#L1010) |
| Kadabra | [✅](src/import.rs#L234) | [✅](src/import.rs#L989) |
| Koraidon ex | ❌ | — |
| Kyurem | ❌ | ❌ |
| Latias ex | ❌ | ❌ |
| Lillie's Clefairy ex | ❌ | ❌ |
| Mega Absol ex | ❌ | — |
| Mega Excadrill ex | ❌ | — |
| Mega Kangaskhan ex | [✅](src/import.rs#L1140) | [✅](src/import.rs#L985) |
| Mega Lopunny ex | ❌ | — |
| Mega Sharpedo ex | [✅](src/import.rs#L1088) | — |
| Mega Skarmory ex | ❌ | — |
| Mega Slowbro ex | ❌ | — |
| Meganium | [✅](src/import.rs#L234) | ❌ |
| Meowth ex | [✅](src/import.rs#L1143) | [✅](src/import.rs#L988) |
| Metagross | ❌ | — |
| Metang | [✅](src/import.rs#L234) | ❌ |
| Moltres | [✅](src/import.rs#L1098) | — |
| Munkidori | ❌ | ❌ |
| N's Darmanitan | [✅](src/import.rs#L1053) | — |
| N's Darumaka | [✅](src/import.rs#L234) | — |
| N's Reshiram | [✅](src/import.rs#L1050) | — |
| N's Zekrom | [✅](src/import.rs#L1062) | — |
| N's Zoroark ex | ❌ | ❌ |
| N's Zorua | [✅](src/import.rs#L234) | — |
| Noctowl | [✅](src/import.rs#L234) | ❌ |
| Paldean Tauros | [✅](src/import.rs#L1046) | — |
| Passimian | [✅](src/import.rs#L1059) | — |
| Patrat | [✅](src/import.rs#L234) | ❌ |
| Pecharunt | ❌ | ❌ |
| Pecharunt ex | [✅](src/import.rs#L1131) | [✅](src/import.rs#L1026) |
| Psyduck | [✅](src/import.rs#L234) | ❌ |
| Rabsca | ❌ | ❌ |
| Raging Bolt ex | ❌ | — |
| Rellor | [✅](src/import.rs#L1044) | — |
| Seaking | ❌ | ❌ |
| Shaymin | [✅](src/import.rs#L234) | ❌ |
| Slowking | [✅](src/import.rs#L1091) | — |
| Slowpoke | [✅](src/import.rs#L1099) | ❌ |
| Smoochum | ❌ | — |
| Stunfisk | ❌ | — |
| Tapu Bulu | [✅](src/import.rs#L1045) | — |
| Tatsugiri | [✅](src/import.rs#L234) | ❌ |
| Teal Mask Ogerpon ex | [✅](src/import.rs#L1145) | [✅](src/import.rs#L994) |
| Thwackey | [✅](src/import.rs#L234) | ❌ |
| Torchic | [✅](src/import.rs#L1101) | — |
| Toxel | [✅](src/import.rs#L1086) | — |
| Toxtricity | [✅](src/import.rs#L234) | [✅](src/import.rs#L1013) |
| Wellspring Mask Ogerpon ex | [✅](src/import.rs#L1076) | — |
| Yveltal | [✅](src/import.rs#L1075) | — |
| Zeraora | [✅](src/import.rs#L1068) | — |

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





























