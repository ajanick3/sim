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
Checkup, knockouts, Prizes, the three win conditions, evolution, and eight
Trainers. What it does not do yet: Abilities and Stadiums.

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
`cargo run --bin progress_table` regenerates this section from
`data/cards.json` and the committed decks.

### Supporters (25/26 built)

| Card | Status |
| --- | --- |
| AZ's Tranquility | ✅ |
| Black Belt's Training | ✅ |
| Boss's Orders | ✅ |
| Briar | ❌ |
| Brock's Scouting | ✅ |
| Ciphermaniac's Codebreaking | ✅ |
| Crispin | ✅ |
| Cyrano | ✅ |
| Dawn | ✅ |
| Eri | ✅ |
| Gladion's Final Battle | ✅ |
| Gwynn | ✅ |
| Hilda | ✅ |
| Janine's Secret Art | ✅ |
| Judge | ✅ |
| Kieran | ✅ |
| Lana's Aid | ✅ |
| Lillie's Determination | ✅ |
| Morty's Conviction | ✅ |
| N's Plan | ✅ |
| Rosa's Encouragement | ✅ |
| Rust Syndicate Grunt | ✅ |
| Surfer | ✅ |
| Team Rocket's Petrel | ✅ |
| Wally's Compassion | ✅ |
| Xerosic's Machinations | ✅ |

### Items (24/28 built)

| Card | Status |
| --- | --- |
| Buddy-Buddy Poffin | ✅ |
| Bug Catching Set | ✅ |
| Crushing Hammer | ✅ |
| Dusk Ball | ✅ |
| Energy Recycler | ✅ |
| Energy Retrieval | ✅ |
| Energy Search | ✅ |
| Energy Switch | ✅ |
| Enhanced Hammer | ❌ |
| Glass Trumpet | ❌ |
| Hand Trimmer | ✅ |
| Jumbo Ice Cream | ✅ |
| N's PP Up | ✅ |
| Night Stretcher | ✅ |
| Prime Catcher | ✅ |
| Rare Candy | ✅ |
| Sacred Ash | ✅ |
| Secret Box | ✅ |
| Special Red Card | ✅ |
| Strange Timepiece | ✅ |
| Switch | ✅ |
| Team Rocket's Transceiver | ✅ |
| Tera Orb | ❌ |
| Tool Scrapper | ❌ |
| Transformation Tome | ✅ |
| Ultra Ball | ✅ |
| Unfair Stamp | ✅ |
| Wondrous Patch | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| Air Balloon | ✅ |
| Binding Mochi | ✅ |
| Brave Bangle | ✅ |
| Handheld Fan | ✅ |
| Hero's Cape | ✅ |
| Lillie's Pearl | ✅ |
| Lucky Helmet | ✅ |
| Powerglass | ✅ |
| Punk Helmet | ✅ |

### Stadiums (9/13 built)

| Card | Status |
| --- | --- |
| Academy at Night | ✅ |
| Area Zero Underdepths | ❌ |
| Battle Cage | ❌ |
| Festival Grounds | ✅ |
| Forest of Vitality | ✅ |
| Gravity Mountain | ✅ |
| Jamming Tower | ✅ |
| Lumiose City | ✅ |
| N's Castle | ✅ |
| Nighttime Mine | ❌ |
| Risky Ruins | ✅ |
| Team Rocket's Factory | ✅ |
| Team Rocket's Watchtower | ❌ |

### Pokémon (18/95 built)

| Card | Status |
| --- | --- |
| Abra | ❌ |
| Alakazam | ❌ |
| Annihilape | ❌ |
| Applin | ✅ |
| Bayleef | ✅ |
| Beldum | ✅ |
| Blaziken ex | ❌ |
| Bloodmoon Ursaluna ex | ❌ |
| Brute Bonnet | ❌ |
| Budew | ❌ |
| Buneary | ✅ |
| Carvanha | ✅ |
| Celebi | ❌ |
| Chi-Yu | ❌ |
| Chien-Pao | ❌ |
| Chikorita | ✅ |
| Cofagrigus | ❌ |
| Combusken | ✅ |
| Crustle | ❌ |
| Dedenne | ❌ |
| Dipplin | ❌ |
| Dragapult ex | ❌ |
| Drakloak | ❌ |
| Dreepy | ✅ |
| Drilbur | ✅ |
| Dudunsparce | ❌ |
| Dudunsparce ex | ❌ |
| Dunsparce | ❌ |
| Dusclops | ❌ |
| Dusknoir | ❌ |
| Duskull | ❌ |
| Dwebble | ❌ |
| Elgyem | ❌ |
| Enamorus | ❌ |
| Fan Rotom | ❌ |
| Fezandipiti ex | ❌ |
| Flutter Mane | ❌ |
| Genesect | ❌ |
| Genesect ex | ❌ |
| Goldeen | ✅ |
| Grookey | ✅ |
| Hoothoot | ❌ |
| Hydrapple ex | ❌ |
| Iron Crown ex | ❌ |
| Iron Leaves ex | ❌ |
| Kadabra | ❌ |
| Koraidon ex | ❌ |
| Kyurem | ❌ |
| Latias ex | ❌ |
| Lillie's Clefairy ex | ❌ |
| Mega Absol ex | ❌ |
| Mega Excadrill ex | ❌ |
| Mega Kangaskhan ex | ❌ |
| Mega Lopunny ex | ❌ |
| Mega Sharpedo ex | ❌ |
| Mega Skarmory ex | ❌ |
| Mega Slowbro ex | ❌ |
| Meganium | ❌ |
| Meowth ex | ❌ |
| Metagross | ❌ |
| Metang | ✅ |
| Moltres | ❌ |
| Munkidori | ❌ |
| N's Darmanitan | ❌ |
| N's Darumaka | ✅ |
| N's Reshiram | ❌ |
| N's Zekrom | ❌ |
| N's Zoroark ex | ❌ |
| N's Zorua | ✅ |
| Noctowl | ❌ |
| Paldean Tauros | ❌ |
| Passimian | ❌ |
| Patrat | ❌ |
| Pecharunt | ❌ |
| Pecharunt ex | ❌ |
| Psyduck | ❌ |
| Rabsca | ❌ |
| Raging Bolt ex | ❌ |
| Rellor | ✅ |
| Seaking | ❌ |
| Shaymin | ❌ |
| Slowking | ❌ |
| Slowpoke | ✅ |
| Smoochum | ❌ |
| Stunfisk | ❌ |
| Tapu Bulu | ✅ |
| Tatsugiri | ❌ |
| Teal Mask Ogerpon ex | ❌ |
| Thwackey | ❌ |
| Torchic | ✅ |
| Toxel | ❌ |
| Toxtricity | ❌ |
| Wellspring Mask Ogerpon ex | ❌ |
| Yveltal | ❌ |
| Zeraora | ❌ |

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
























