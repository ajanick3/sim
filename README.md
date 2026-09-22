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
of Trainer, Pokémon Abilities, Pokémon attacks with their own effects, and a
growing set of Special Energy.

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
cargo run --bin coverage             # 996 of 3051 Standard cards (32.6%)
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
per player, named `<placement>-<player-slug>.txt`, zero-padded to three
digits. `tools/fetch_worlds_decklists.py` fetches all 143 from limitlesstcg's
Decklists tab, the tournament's Day 2 standings — the site holds no decklist
for an entrant who did not reach one. All 143 are kept and check clean.

### Standard coverage

Every card in the artifact, by name. The Trainer and Special Energy tables below cover the whole pool; the Pokémon table stays scoped to the field (the decks under `decks/`).

| Kind | Built | Total |
| --- | --- | --- |
| Supporters | 70 | 78 |
| Items | 61 | 85 |
| Tools | 26 | 35 |
| Stadiums | 21 | 31 |
| Special Energy | 15 | 17 |

### Supporters (70/78 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1325) | ✅ |
| [Acerola's Mischief](src/import.rs#L1527) | ✅ |
| [Amarys](src/import.rs#L779) | ✅ |
| Anthea & Concordia | ❌ — requires six named N's Pokémon in play at once and grants 3 extra Prizes on a KO — a compound in-play check plus the deferred prize-count support |
| [Bianca's Devotion](src/import.rs#L1310) | ✅ |
| [Billy & O'Nare](src/import.rs#L766) | ✅ |
| [Black Belt's Training](src/import.rs#L1333) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L888) | ✅ |
| [Brock's Scouting](src/import.rs#L1395) | ✅ |
| [Canari](src/import.rs#L725) | ✅ |
| Caretaker | ❌ — shuffles itself back into the deck instead of discarding, but only if Community Center is in play — no effect yet reads a specific Stadium's presence at resolve time |
| [Carmine](src/import.rs#L682) | ✅ |
| [Cassiopeia](src/import.rs#L743) | ✅ |
| [Cheren](src/import.rs#L234) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1264) | ✅ |
| [Clemont's Quick Wit](src/import.rs#L880) | ✅ |
| [Colress's Tenacity](src/import.rs#L1113) | ✅ |
| [Cook](src/import.rs#L380) | ✅ |
| [Crispin](src/import.rs#L1170) | ✅ |
| [Cyrano](src/import.rs#L1018) | ✅ |
| [Dawn](src/import.rs#L1136) | ✅ |
| [Drasna](src/import.rs#L775) | ✅ |
| [Drayton](src/import.rs#L825) | ✅ |
| [Emcee's Hype](src/import.rs#L758) | ✅ |
| [Emma](src/import.rs#L774) | ✅ |
| [Eri](src/import.rs#L1388) | ✅ |
| [Ethan's Adventure](src/import.rs#L435) | ✅ |
| [Explorer's Guidance](src/import.rs#L811) | ✅ |
| [Fennel](src/import.rs#L876) | ✅ |
| [Firebreather](src/import.rs#L711) | ✅ |
| [Friends in Paldea](src/import.rs#L234) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1341) | ✅ |
| Grimsley's Move | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Gwynn](src/import.rs#L1032) | ✅ |
| [Harlequin](src/import.rs#L860) | ✅ |
| [Hassel](src/import.rs#L797) | ✅ |
| [Hilda](src/import.rs#L1089) | ✅ |
| [Iris's Fighting Spirit](src/import.rs#L872) | ✅ |
| [Jacinthe](src/import.rs#L660) | ✅ |
| [Janine's Secret Art](src/import.rs#L1419) | ✅ |
| [Jasmine's Gaze](src/import.rs#L390) | ✅ |
| [Jett](src/import.rs#L757) | ✅ |
| [Judge](src/import.rs#L911) | ✅ |
| [Kieran](src/import.rs#L1373) | ✅ |
| Kofu | ❌ — bottom-decks exactly 2 chosen cards, refusing itself if the hand holds fewer than 2, then draws 4 — a choose-then-gate shape none of the bottom-deck reskins share |
| [Lacey](src/import.rs#L852) | ✅ |
| [Lana's Aid](src/import.rs#L1290) | ✅ |
| [Larry's Skill](src/import.rs#L1345) | ✅ |
| [Lillie's Determination](src/import.rs#L912) | ✅ |
| [Lisia's Appeal](src/import.rs#L1521) | ✅ |
| Lt. Surge's Bargain | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Lucian](src/import.rs#L848) | ✅ |
| [Misty's Vitality](src/import.rs#L667) | ✅ |
| [Morty's Conviction](src/import.rs#L1383) | ✅ |
| [N's Plan](src/import.rs#L1308) | ✅ |
| [Naveen](src/import.rs#L871) | ✅ |
| Perrin | ❌ — reveals up to 2 Pokémon from hand into the deck, then tutors that many Pokémon back — a two-step "count what you gave up, then search that count" mechanic |
| [Philippe](src/import.rs#L697) | ✅ |
| [Picnicker](src/import.rs#L681) | ✅ |
| [Pokémon Center Lady](src/import.rs#L1309) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1311) | ✅ |
| [Roxie's Performance](src/import.rs#L400) | ✅ |
| Ruffian | ❌ — needs a target-then-discard-both phase, scoped to one Pokémon chosen up front |
| [Rust Syndicate Grunt](src/import.rs#L1304) | ✅ |
| [Salvatore](src/import.rs#L1520) | ✅ |
| [Surfer](src/import.rs#L1329) | ✅ |
| [Tarragon](src/import.rs#L683) | ✅ |
| [Team Rocket's Archer](src/import.rs#L739) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1481) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1489) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1194) | ✅ |
| [Team Rocket's Proton](src/import.rs#L449) | ✅ |
| Tyme | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Urbain](src/import.rs#L379) | ✅ |
| [Waitress](src/import.rs#L783) | ✅ |
| [Wally's Compassion](src/import.rs#L1418) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1387) | ✅ |

### Items (61/85 built)

| Card | Status |
| --- | --- |
| Accompanying Flute | ❌ — reveals the top 5 of the opponent's deck and fills the opponent's own Bench with any Basics found — the first search reading the opponent's deck to fill the opponent's board |
| Antique Armor Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Cover Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Jaw Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Plume Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Root Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Sail Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Skull Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Arven's Sandwich | ❌ — heals 30, or 100 if the healed Pokémon's own name is an Arven's Pokémon — the bonus reads the target's name, not the carrier's, unlike every named-bonus shape built so far |
| [Awakening Drum](src/import.rs#L406) | ✅ |
| Blowtorch | ❌ — costs discarding a Basic Fire Energy from hand to play at all — the first Item whose own legality spends a hand card as its price, not just a `Requirement` check |
| [Boxed Order](src/import.rs#L604) | ✅ |
| [Brilliant Blender](src/import.rs#L548) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L1004) | ✅ |
| [Bug Catching Set](src/import.rs#L1250) | ✅ |
| Call Bell | ❌ — needs the "playable on the first turn" allowance |
| Chill Teaser Toy | ❌ — needs the "playable on the first turn" allowance |
| [Crushing Hammer](src/import.rs#L947) | ✅ |
| [Dangerous Laser](src/import.rs#L536) | ✅ |
| [Dark Bell](src/import.rs#L543) | ✅ |
| Deduction Kit | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Dragon Elixir](src/import.rs#L647) | ✅ |
| [Dusk Ball](src/import.rs#L1449) | ✅ |
| [Energy Coin](src/import.rs#L435) | ✅ |
| [Energy Recycler](src/import.rs#L1604) | ✅ |
| [Energy Retrieval](src/import.rs#L1434) | ✅ |
| [Energy Search](src/import.rs#L1420) | ✅ |
| [Energy Search Pro](src/import.rs#L962) | ✅ |
| Energy Swatter | ❌ — opponent reveals their hand and you choose an Energy card there — the interactive-opponent-choice bucket already named in Deferred |
| [Energy Switch](src/import.rs#L1074) | ✅ |
| [Enhanced Hammer](src/import.rs#L892) | ✅ |
| [Fighting Gong](src/import.rs#L1455) | ✅ |
| [Glass Trumpet](src/import.rs#L893) | ✅ |
| Great Haul Net | ❌ — shuffles chosen cards from the discard pile back into the deck (in either of two categories) — today's shuffles only return unchosen peek cards, never chosen discard-pile ones |
| [Hand Trimmer](src/import.rs#L1448) | ✅ |
| [Hole-Digging Shovel](src/import.rs#L381) | ✅ |
| [Hop's Bag](src/import.rs#L463) | ✅ |
| [Hyper Aroma](src/import.rs#L576) | ✅ |
| [Iron Defender](src/import.rs#L394) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1286) | ✅ |
| Love Ball | ❌ — searches the deck for a Pokémon sharing a name with one the opponent has in play — a tutor filter keyed to the opponent's board state, not a fixed name or prefix |
| [Lumiose Galette](src/import.rs#L646) | ✅ |
| [Master Ball](src/import.rs#L562) | ✅ |
| [Max Rod](src/import.rs#L618) | ✅ |
| Meddling Memo | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Mega Signal](src/import.rs#L976) | ✅ |
| Megaton Blower | ❌ — discards every Tool and Special Energy from every opponent Pokémon at once, plus a Stadium — the existing discard effects work one target at a time |
| [Miracle Headset](src/import.rs#L632) | ✅ |
| [N's PP Up](src/import.rs#L1208) | ✅ |
| [Night Stretcher](src/import.rs#L919) | ✅ |
| Ogre's Mask | ❌ — swaps a Pokémon ex in the discard pile with one in play, carrying over attachments, damage, and conditions — a whole discard-to-active swap mechanic |
| [Poké Ball](src/import.rs#L421) | ✅ |
| [Poké Pad](src/import.rs#L933) | ✅ |
| [Poké Vital A](src/import.rs#L656) | ✅ |
| [Pokégear 3.0](src/import.rs#L1236) | ✅ |
| [Pokémon Catcher](src/import.rs#L1451) | ✅ |
| [Potion](src/import.rs#L651) | ✅ |
| [Precious Trolley](src/import.rs#L948) | ✅ |
| [Premium Power Pro](src/import.rs#L1337) | ✅ |
| [Prime Catcher](src/import.rs#L1450) | ✅ |
| [Rare Candy](src/import.rs#L1193) | ✅ |
| [Reboot Pod](src/import.rs#L410) | ✅ |
| Redeemable Ticket | ❌ — reshuffles the Prize cards and redraws that many — the first Item to touch the Prize pile itself, and it wants the deferred prize-count support |
| [Repel](src/import.rs#L547) | ✅ |
| Roto-Stick | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Sacred Ash](src/import.rs#L1046) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1522) | ✅ |
| Scramble Switch | ❌ — switches Active and Bench, then may move all Energy from the Pokémon just benched onto the new Active — switching exists, moving Energy along with one does not |
| [Secret Box](src/import.rs#L1553) | ✅ |
| [Special Red Card](src/import.rs#L1166) | ✅ |
| [Strange Timepiece](src/import.rs#L1469) | ✅ |
| [Super Potion](src/import.rs#L655) | ✅ |
| [Switch](src/import.rs#L1285) | ✅ |
| [TM Machine](src/import.rs#L990) | ✅ |
| Team Rocket's Bother-Bot | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Team Rocket's Great Ball](src/import.rs#L1493) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1590) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1523) | ✅ |
| [Tera Orb](src/import.rs#L1075) | ✅ |
| [Tool Scrapper](src/import.rs#L887) | ✅ |
| [Transformation Tome](src/import.rs#L1549) | ✅ |
| [Treasure Tracker](src/import.rs#L590) | ✅ |
| [Ultra Ball](src/import.rs#L1060) | ✅ |
| [Unfair Stamp](src/import.rs#L1278) | ✅ |
| [Wondrous Patch](src/import.rs#L1222) | ✅ |

### Tools (26/35 built)

| Card | Status |
| --- | --- |
| [Adversity Policy](src/import.rs#L535) | ✅ |
| [Air Balloon](src/import.rs#L1470) | ✅ |
| Amulet of Hope | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Babiri Berry](src/import.rs#L497) | ✅ |
| Backtrack Badge | ❌ — re-flips an attack's coins after seeing the results — no Tool can intercept and redo flips that already landed |
| [Binding Mochi](src/import.rs#L1473) | ✅ |
| [Brave Bangle](src/import.rs#L1472) | ✅ |
| [Colbur Berry](src/import.rs#L501) | ✅ |
| Core Memory | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Counter Gain](src/import.rs#L420) | ✅ |
| [Cynthia's Power Weight](src/import.rs#L477) | ✅ |
| Deluxe Bomb | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Gravity Gemstone](src/import.rs#L412) | ✅ |
| [Haban Berry](src/import.rs#L517) | ✅ |
| [Handheld Fan](src/import.rs#L1477) | ✅ |
| Heavy Baton | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Hero's Cape](src/import.rs#L1471) | ✅ |
| Hop's Choice Band | ❌ — combines a named-carrier attack-cost discount with a named-carrier bonus-damage clause in one Tool — both shapes exist separately; the combined variant is unbuilt |
| [Light Ball](src/import.rs#L383) | ✅ |
| [Lillie's Pearl](src/import.rs#L1474) | ✅ |
| [Lucky Helmet](src/import.rs#L1476) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Occa Berry](src/import.rs#L505) | ✅ |
| [Passho Berry](src/import.rs#L509) | ✅ |
| [Payapa Berry](src/import.rs#L513) | ✅ |
| [Powerglass](src/import.rs#L1478) | ✅ |
| [Punk Helmet](src/import.rs#L1475) | ✅ |
| [Rescue Board](src/import.rs#L389) | ✅ |
| [Sacred Charm](src/import.rs#L387) | ✅ |
| [Sparkling Crystal](src/import.rs#L416) | ✅ |
| Survival Brace | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Team Rocket's Hypnotizer](src/import.rs#L528) | ✅ |
| Technical Machine: Fluorite | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Thick Scale](src/import.rs#L521) | ✅ |
| Tremendous Bomb | ❌ — retaliates only against a Mega Evolution Pokémon ex specifically, past a 240-damage threshold, then discards itself — the reactive-damage Tools don't yet gate on the attacker's own category or self-discard after firing |

### Stadiums (21/31 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1531) | ✅ |
| Ange Floette | ❌ — can be put into play only by discarding a specific other Stadium (Prism Tower) that same turn — no rule lets a Stadium's play condition consume another Stadium already in play |
| [Area Zero Underdepths](src/import.rs#L908) | ✅ |
| [Battle Cage](src/import.rs#L909) | ✅ |
| Celebratory Fanfare | ❌ — heals every Pokémon on both sides at once and ends the turn only if healing happened — pairs the "ends the turn if it did anything" clause with a heal instead of a search for the first time |
| [Community Center](src/import.rs#L1538) | ✅ |
| Dizzying Valley | ❌ — stops Confusion from clearing on evolve or devolve — evolving already always clears conditions, and nothing hooks that step to skip it |
| [Festival Grounds](src/import.rs#L1545) | ✅ |
| [Forest of Vitality](src/import.rs#L1544) | ✅ |
| Fossil Quarry | ❌ — searches for up to 2 "Antique …" Items and benches them as Pokémon — part of the already-deferred Trainer-as-Pokémon Antique Fossil mechanic |
| [Full Metal Lab](src/import.rs#L493) | ✅ |
| Grand Tree | ❌ — searches for a Stage 1, evolves it onto a Basic, then chains into a Stage 2 of that same Pokémon — a two-step forced-evolution search nothing performs yet |
| [Granite Cave](src/import.rs#L481) | ✅ |
| [Gravity Mountain](src/import.rs#L1479) | ✅ |
| [Jamming Tower](src/import.rs#L1542) | ✅ |
| Levincia | ❌ — once per turn, returns up to 2 Basic Lightning Energy from discard to hand — a per-turn Stadium search keyed to a type instead of a name; feasible, unbuilt |
| [Lively Stadium](src/import.rs#L388) | ✅ |
| [Lumiose City](src/import.rs#L1536) | ✅ |
| Mystery Garden | ❌ — discards an Energy to draw up to your in-play Pokémon count — the draw target reads the board instead of a fixed number the way `DrawUpToHandSize` does |
| [N's Castle](src/import.rs#L1480) | ✅ |
| Neutralization Zone | ❌ — blocks damage from Pokémon ex/V onto non-Rule-Box Pokémon and can't return from the discard pile — needs a Rule-Box read and a discard-pile lockout, neither tracked today |
| [Nighttime Mine](src/import.rs#L907) | ✅ |
| [Paradise Resort](src/import.rs#L489) | ✅ |
| [Perilous Jungle](src/import.rs#L411) | ✅ |
| [Postwick](src/import.rs#L485) | ✅ |
| [Prism Tower](src/import.rs#L1537) | ✅ |
| [Risky Ruins](src/import.rs#L1543) | ✅ |
| Spikemuth Gym | ❌ — once per turn, searches for a Marnie's Pokémon to hand — a name-prefix Stadium search; feasible, unbuilt |
| Surfing Beach | ❌ — once per turn, switches Active with a Benched Water Pokémon — no Stadium effect yet grants a free switch action |
| [Team Rocket's Factory](src/import.rs#L1532) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L910) | ✅ |

### Special Energy (15/17 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1983) | ✅ |
| [Bubbly Water Energy](src/import.rs#L1995) | ✅ |
| [Enriching Energy](src/import.rs#L1966) | ✅ |
| [Growing Grass Energy](src/import.rs#L1965) | ✅ |
| [Ignition Energy](src/import.rs#L2007) | ✅ |
| Legacy Energy | ❌ — its Prize-count clause needs the deferred Prize-count mechanic |
| [Magnetic Metal Energy](src/import.rs#L1992) | ✅ |
| [Mist Energy](src/import.rs#L1980) | ✅ |
| [Neo Upper Energy](src/import.rs#L2013) | ✅ |
| [Nitro Fire Energy](src/import.rs#L1998) | ✅ |
| [Prism Energy](src/import.rs#L1986) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L1989) | ✅ |
| [Shadowy Darkness Energy](src/import.rs#L2004) | ✅ |
| [Spiky Energy](src/import.rs#L1977) | ✅ |
| Team Rocket's Energy | ❌ — attaches only to a Team Rocket's Pokémon and discards itself instantly off any other — nothing validates an attach against the carrier's identity yet |
| [Telepathic Psychic Energy](src/import.rs#L1969) | ✅ |
| [Voltaic Lightning Energy](src/import.rs#L2001) | ✅ |

### Pokémon (field) (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2200) | [✅](src/import.rs#L2200) | [✅](src/import.rs#L2082) |
| [Alakazam](src/import.rs#L2355) | [✅](src/import.rs#L2355) | [✅](src/import.rs#L2072) |
| [Annihilape](src/import.rs#L2229) | [✅](src/import.rs#L2229) | [✅](src/import.rs#L2038) |
| [Applin](src/import.rs#L2190) | [✅](src/import.rs#L2190) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2232) | [✅](src/import.rs#L2232) | — |
| [Beldum](src/import.rs#L2212) | [✅](src/import.rs#L2212) | — |
| [Blaziken ex](src/import.rs#L2296) | [✅](src/import.rs#L2296) | [✅](src/import.rs#L2112) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2260) | [✅](src/import.rs#L2260) | [✅](src/import.rs#L2046) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2171) | [✅](src/import.rs#L2171) | — |
| [Budew](src/import.rs#L2237) | [✅](src/import.rs#L2237) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2231) | [✅](src/import.rs#L2231) | — |
| [Carvanha](src/import.rs#L2149) | [✅](src/import.rs#L2149) | — |
| [Celebi](src/import.rs#L2230) | [✅](src/import.rs#L2230) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2328) | [✅](src/import.rs#L2328) | — |
| [Chien-Pao](src/import.rs#L2297) | [✅](src/import.rs#L2297) | [✅](src/import.rs#L2115) |
| [Chikorita](src/import.rs#L2233) | [✅](src/import.rs#L2233) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2278) | [✅](src/import.rs#L2278) | — |
| [Combusken](src/import.rs#L2247) | [✅](src/import.rs#L2247) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2367) | [✅](src/import.rs#L2367) | [✅](src/import.rs#L2027) |
| [Dedenne](src/import.rs#L2187) | [✅](src/import.rs#L2187) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2312) | [✅](src/import.rs#L2312) | [✅](src/import.rs#L2100) |
| [Dragapult ex](src/import.rs#L2194) | [✅](src/import.rs#L2194) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2055) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2201) | [✅](src/import.rs#L2201) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2079) |
| [Dudunsparce ex](src/import.rs#L2162) | [✅](src/import.rs#L2162) | — |
| [Dunsparce](src/import.rs#L2213) | [✅](src/import.rs#L2213) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2083) |
| [Dusknoir](src/import.rs#L2294) | [✅](src/import.rs#L2294) | [✅](src/import.rs#L2084) |
| [Duskull](src/import.rs#L2216) | [✅](src/import.rs#L2216) | — |
| [Dwebble](src/import.rs#L2206) | [✅](src/import.rs#L2206) | — |
| [Elgyem](src/import.rs#L2236) | [✅](src/import.rs#L2236) | — |
| [Enamorus](src/import.rs#L2256) | [✅](src/import.rs#L2256) | — |
| [Fan Rotom](src/import.rs#L2301) | [✅](src/import.rs#L2301) | [✅](src/import.rs#L2125) |
| [Fezandipiti ex](src/import.rs#L2375) | [✅](src/import.rs#L2375) | [✅](src/import.rs#L2073) |
| [Flutter Mane](src/import.rs#L2292) | [✅](src/import.rs#L2292) | [✅](src/import.rs#L2049) |
| [Genesect](src/import.rs#L2347) | [✅](src/import.rs#L2347) | [✅](src/import.rs#L2088) |
| [Genesect ex](src/import.rs#L2295) | [✅](src/import.rs#L2295) | [✅](src/import.rs#L2085) |
| [Goldeen](src/import.rs#L2310) | [✅](src/import.rs#L2310) | [✅](src/import.rs#L2098) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2203) | [✅](src/import.rs#L2203) | [✅](src/import.rs#L2039) |
| [Hydrapple ex](src/import.rs#L2261) | [✅](src/import.rs#L2261) | [✅](src/import.rs#L2040) |
| [Iron Crown ex](src/import.rs#L2221) | [✅](src/import.rs#L2221) | [✅](src/import.rs#L2035) |
| [Iron Leaves ex](src/import.rs#L2300) | [✅](src/import.rs#L2300) | [✅](src/import.rs#L2116) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2068) |
| [Koraidon ex](src/import.rs#L2222) | [✅](src/import.rs#L2222) | — |
| [Kyurem](src/import.rs#L2344) | [✅](src/import.rs#L2344) | [✅](src/import.rs#L2107) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2361) | [✅](src/import.rs#L2361) | [✅](src/import.rs#L2026) |
| [Lillie's Clefairy ex](src/import.rs#L2385) | [✅](src/import.rs#L2385) | [✅](src/import.rs#L2030) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2268) | [✅](src/import.rs#L2268) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2264) | [✅](src/import.rs#L2264) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2371) | [✅](src/import.rs#L2371) | [✅](src/import.rs#L2023) |
| [Mega Lopunny ex](src/import.rs#L2169) | [✅](src/import.rs#L2169) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2204) | [✅](src/import.rs#L2204) | — |
| [Mega Skarmory ex](src/import.rs#L2282) | [✅](src/import.rs#L2282) | — |
| [Mega Slowbro ex](src/import.rs#L2322) | [✅](src/import.rs#L2322) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2094) |
| [Meowth ex](src/import.rs#L2374) | [✅](src/import.rs#L2374) | [✅](src/import.rs#L2067) |
| [Metagross](src/import.rs#L2178) | [✅](src/import.rs#L2178) | — |
| [Metang](src/import.rs#L2362) | [✅](src/import.rs#L2362) | [✅](src/import.rs#L2058) |
| [Moltres](src/import.rs#L2214) | [✅](src/import.rs#L2214) | — |
| [Munkidori](src/import.rs#L2368) | [✅](src/import.rs#L2368) | [✅](src/import.rs#L2061) |
| [N's Darmanitan](src/import.rs#L2159) | [✅](src/import.rs#L2159) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2156) | [✅](src/import.rs#L2156) | — |
| [N's Zekrom](src/import.rs#L2168) | [✅](src/import.rs#L2168) | — |
| [N's Zoroark ex](src/import.rs#L2341) | [✅](src/import.rs#L2341) | [✅](src/import.rs#L2091) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2364) | [✅](src/import.rs#L2364) | [✅](src/import.rs#L2069) |
| [Paldean Tauros](src/import.rs#L2152) | [✅](src/import.rs#L2152) | — |
| [Passimian](src/import.rs#L2165) | [✅](src/import.rs#L2165) | — |
| [Patrat](src/import.rs#L2363) | [✅](src/import.rs#L2363) | [✅](src/import.rs#L2028) |
| [Pecharunt](src/import.rs#L2305) | [✅](src/import.rs#L2305) | [✅](src/import.rs#L2095) |
| [Pecharunt ex](src/import.rs#L2302) | [✅](src/import.rs#L2302) | [✅](src/import.rs#L2132) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2029) |
| [Rabsca](src/import.rs#L2218) | [✅](src/import.rs#L2218) | [✅](src/import.rs#L2034) |
| [Raging Bolt ex](src/import.rs#L2183) | [✅](src/import.rs#L2183) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L2150) | [✅](src/import.rs#L2150) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2311) | [✅](src/import.rs#L2311) | [✅](src/import.rs#L2099) |
| [Shaymin](src/import.rs#L2358) | [✅](src/import.rs#L2358) | [✅](src/import.rs#L2033) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2207) | [✅](src/import.rs#L2207) | — |
| [Slowpoke](src/import.rs#L2215) | [✅](src/import.rs#L2215) | ❌ |
| [Smoochum](src/import.rs#L2275) | [✅](src/import.rs#L2275) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2257) | [✅](src/import.rs#L2257) | — |
| [Tapu Bulu](src/import.rs#L2151) | [✅](src/import.rs#L2151) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2052) |
| [Teal Mask Ogerpon ex](src/import.rs#L2376) | [✅](src/import.rs#L2376) | [✅](src/import.rs#L2076) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2101) |
| [Torchic](src/import.rs#L2217) | [✅](src/import.rs#L2217) | — |
| [Toxel](src/import.rs#L2202) | [✅](src/import.rs#L2202) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2119) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2192) | [✅](src/import.rs#L2192) | — |
| [Yveltal](src/import.rs#L2191) | [✅](src/import.rs#L2191) | — |
| [Zeraora](src/import.rs#L2174) | [✅](src/import.rs#L2174) | — |
| Zoroark | ❌ | — |


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

