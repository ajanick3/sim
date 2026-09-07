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
| [AZ's Tranquility](src/import.rs#L682) | ✅ |
| [Black Belt's Training](src/import.rs#L690) | ✅ |
| [Boss's Orders](src/import.rs#L362) | ✅ |
| Briar | ❌ |
| [Brock's Scouting](src/import.rs#L720) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L622) | ✅ |
| [Crispin](src/import.rs#L528) | ✅ |
| [Cyrano](src/import.rs#L414) | ✅ |
| [Dawn](src/import.rs#L494) | ✅ |
| [Eri](src/import.rs#L713) | ✅ |
| [Gladion's Final Battle](src/import.rs#L694) | ✅ |
| [Gwynn](src/import.rs#L428) | ✅ |
| [Hilda](src/import.rs#L471) | ✅ |
| [Janine's Secret Art](src/import.rs#L744) | ✅ |
| [Judge](src/import.rs#L363) | ✅ |
| [Kieran](src/import.rs#L698) | ✅ |
| [Lana's Aid](src/import.rs#L648) | ✅ |
| [Lillie's Determination](src/import.rs#L364) | ✅ |
| [Morty's Conviction](src/import.rs#L708) | ✅ |
| [N's Plan](src/import.rs#L666) | ✅ |
| [Rosa's Encouragement](src/import.rs#L668) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L662) | ✅ |
| [Surfer](src/import.rs#L686) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L552) | ✅ |
| [Wally's Compassion](src/import.rs#L743) | ✅ |
| [Xerosic's Machinations](src/import.rs#L712) | ✅ |

### Items (24/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L400) | ✅ |
| [Bug Catching Set](src/import.rs#L608) | ✅ |
| [Crushing Hammer](src/import.rs#L399) | ✅ |
| [Dusk Ball](src/import.rs#L774) | ✅ |
| [Energy Recycler](src/import.rs#L856) | ✅ |
| [Energy Retrieval](src/import.rs#L759) | ✅ |
| [Energy Search](src/import.rs#L745) | ✅ |
| [Energy Switch](src/import.rs#L470) | ✅ |
| Enhanced Hammer | ❌ |
| Glass Trumpet | ❌ |
| [Hand Trimmer](src/import.rs#L773) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L644) | ✅ |
| [N's PP Up](src/import.rs#L566) | ✅ |
| [Night Stretcher](src/import.rs#L371) | ✅ |
| [Prime Catcher](src/import.rs#L775) | ✅ |
| [Rare Candy](src/import.rs#L551) | ✅ |
| [Sacred Ash](src/import.rs#L442) | ✅ |
| [Secret Box](src/import.rs#L805) | ✅ |
| [Special Red Card](src/import.rs#L524) | ✅ |
| [Strange Timepiece](src/import.rs#L776) | ✅ |
| [Switch](src/import.rs#L643) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L842) | ✅ |
| Tera Orb | ❌ |
| Tool Scrapper | ❌ |
| [Transformation Tome](src/import.rs#L801) | ✅ |
| [Ultra Ball](src/import.rs#L456) | ✅ |
| [Unfair Stamp](src/import.rs#L636) | ✅ |
| [Wondrous Patch](src/import.rs#L580) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L777) | ✅ |
| [Binding Mochi](src/import.rs#L780) | ✅ |
| [Brave Bangle](src/import.rs#L779) | ✅ |
| [Handheld Fan](src/import.rs#L784) | ✅ |
| [Hero's Cape](src/import.rs#L778) | ✅ |
| [Lillie's Pearl](src/import.rs#L781) | ✅ |
| [Lucky Helmet](src/import.rs#L783) | ✅ |
| [Powerglass](src/import.rs#L785) | ✅ |
| [Punk Helmet](src/import.rs#L782) | ✅ |

### Stadiums (9/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L788) | ✅ |
| Area Zero Underdepths | ❌ |
| Battle Cage | ❌ |
| [Festival Grounds](src/import.rs#L797) | ✅ |
| [Forest of Vitality](src/import.rs#L796) | ✅ |
| [Gravity Mountain](src/import.rs#L786) | ✅ |
| [Jamming Tower](src/import.rs#L794) | ✅ |
| [Lumiose City](src/import.rs#L793) | ✅ |
| [N's Castle](src/import.rs#L787) | ✅ |
| Nighttime Mine | ❌ |
| [Risky Ruins](src/import.rs#L795) | ✅ |
| [Team Rocket's Factory](src/import.rs#L789) | ✅ |
| Team Rocket's Watchtower | ❌ |

### Pokémon (37/95 built)

| Card | Status |
| --- | --- |
| [Abra](src/import.rs#L992) | ✅ |
| Alakazam | ❌ |
| Annihilape | ❌ |
| [Applin](src/import.rs#L982) | ✅ |
| [Bayleef](src/import.rs#L234) | ✅ |
| [Beldum](src/import.rs#L1004) | ✅ |
| Blaziken ex | ❌ |
| Bloodmoon Ursaluna ex | ❌ |
| Brute Bonnet | ❌ |
| Budew | ❌ |
| [Buneary](src/import.rs#L1011) | ✅ |
| [Carvanha](src/import.rs#L951) | ✅ |
| [Celebi](src/import.rs#L1010) | ✅ |
| Chi-Yu | ❌ |
| Chien-Pao | ❌ |
| [Chikorita](src/import.rs#L234) | ✅ |
| Cofagrigus | ❌ |
| [Combusken](src/import.rs#L234) | ✅ |
| Crustle | ❌ |
| [Dedenne](src/import.rs#L979) | ✅ |
| Dipplin | ❌ |
| [Dragapult ex](src/import.rs#L986) | ✅ |
| Drakloak | ❌ |
| [Dreepy](src/import.rs#L234) | ✅ |
| [Drilbur](src/import.rs#L993) | ✅ |
| Dudunsparce | ❌ |
| [Dudunsparce ex](src/import.rs#L964) | ✅ |
| [Dunsparce](src/import.rs#L1005) | ✅ |
| Dusclops | ❌ |
| Dusknoir | ❌ |
| [Duskull](src/import.rs#L1008) | ✅ |
| [Dwebble](src/import.rs#L998) | ✅ |
| Elgyem | ❌ |
| Enamorus | ❌ |
| Fan Rotom | ❌ |
| Fezandipiti ex | ❌ |
| Flutter Mane | ❌ |
| Genesect | ❌ |
| Genesect ex | ❌ |
| [Goldeen](src/import.rs#L234) | ✅ |
| [Grookey](src/import.rs#L234) | ✅ |
| [Hoothoot](src/import.rs#L995) | ✅ |
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
| [Mega Sharpedo ex](src/import.rs#L996) | ✅ |
| Mega Skarmory ex | ❌ |
| Mega Slowbro ex | ❌ |
| Meganium | ❌ |
| Meowth ex | ❌ |
| Metagross | ❌ |
| [Metang](src/import.rs#L234) | ✅ |
| [Moltres](src/import.rs#L1006) | ✅ |
| Munkidori | ❌ |
| [N's Darmanitan](src/import.rs#L961) | ✅ |
| [N's Darumaka](src/import.rs#L234) | ✅ |
| [N's Reshiram](src/import.rs#L958) | ✅ |
| [N's Zekrom](src/import.rs#L970) | ✅ |
| N's Zoroark ex | ❌ |
| [N's Zorua](src/import.rs#L234) | ✅ |
| Noctowl | ❌ |
| [Paldean Tauros](src/import.rs#L954) | ✅ |
| [Passimian](src/import.rs#L967) | ✅ |
| Patrat | ❌ |
| Pecharunt | ❌ |
| Pecharunt ex | ❌ |
| Psyduck | ❌ |
| Rabsca | ❌ |
| Raging Bolt ex | ❌ |
| [Rellor](src/import.rs#L952) | ✅ |
| Seaking | ❌ |
| Shaymin | ❌ |
| [Slowking](src/import.rs#L999) | ✅ |
| [Slowpoke](src/import.rs#L1007) | ✅ |
| Smoochum | ❌ |
| Stunfisk | ❌ |
| [Tapu Bulu](src/import.rs#L953) | ✅ |
| Tatsugiri | ❌ |
| Teal Mask Ogerpon ex | ❌ |
| Thwackey | ❌ |
| [Torchic](src/import.rs#L1009) | ✅ |
| [Toxel](src/import.rs#L994) | ✅ |
| Toxtricity | ❌ |
| Wellspring Mask Ogerpon ex | ❌ |
| [Yveltal](src/import.rs#L983) | ✅ |
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





























