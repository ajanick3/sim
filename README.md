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
cargo run --bin coverage             # 1004 of 3051 Standard cards (32.9%)
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

`decks/2026-worlds/` holds the field of the 2026 World Championships, and
`decks/2026-baltimore/` the field of the Baltimore Regional, each one file
per player named `<placement>-<player-slug>.txt`, zero-padded to three
digits. `tools/fetch_worlds_decklists.py <tournament-id> <decks-slug>` fetches
a tournament's own Decklists tab from limitlesstcg — every entrant it
publishes, commonly a Day 2 standing but sometimes a whole field:

```sh
python3 tools/fetch_worlds_decklists.py 515 2026-worlds     # 143 decks
python3 tools/fetch_worlds_decklists.py 577 2026-baltimore  # 559 decks
```

All 702 are kept and check clean.

### Standard coverage

Every card in the artifact, by name. The Trainer and Special Energy tables below cover the whole pool; the Pokémon table stays scoped to the field (the decks under `decks/`).

| Kind | Built | Total |
| --- | --- | --- |
| Supporters | 73 | 78 |
| Items | 68 | 85 |
| Tools | 27 | 35 |
| Stadiums | 25 | 31 |
| Special Energy | 15 | 17 |

### Supporters (73/78 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1405) | ✅ |
| [Acerola's Mischief](src/import.rs#L1607) | ✅ |
| [Amarys](src/import.rs#L859) | ✅ |
| Anthea & Concordia | ❌ — not yet triaged |
| [Bianca's Devotion](src/import.rs#L1390) | ✅ |
| [Billy & O'Nare](src/import.rs#L846) | ✅ |
| [Black Belt's Training](src/import.rs#L1413) | ✅ |
| [Boss's Orders](src/import.rs#L417) | ✅ |
| [Briar](src/import.rs#L968) | ✅ |
| [Brock's Scouting](src/import.rs#L1475) | ✅ |
| [Canari](src/import.rs#L805) | ✅ |
| [Caretaker](src/import.rs#L1706) | ✅ |
| [Carmine](src/import.rs#L762) | ✅ |
| [Cassiopeia](src/import.rs#L823) | ✅ |
| [Cheren](src/import.rs#L234) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1344) | ✅ |
| [Clemont's Quick Wit](src/import.rs#L960) | ✅ |
| [Colress's Tenacity](src/import.rs#L1193) | ✅ |
| [Cook](src/import.rs#L419) | ✅ |
| [Crispin](src/import.rs#L1250) | ✅ |
| [Cyrano](src/import.rs#L1098) | ✅ |
| [Dawn](src/import.rs#L1216) | ✅ |
| [Drasna](src/import.rs#L855) | ✅ |
| [Drayton](src/import.rs#L905) | ✅ |
| [Emcee's Hype](src/import.rs#L838) | ✅ |
| [Emma](src/import.rs#L854) | ✅ |
| [Eri](src/import.rs#L1468) | ✅ |
| [Ethan's Adventure](src/import.rs#L499) | ✅ |
| [Explorer's Guidance](src/import.rs#L891) | ✅ |
| [Fennel](src/import.rs#L956) | ✅ |
| [Firebreather](src/import.rs#L791) | ✅ |
| [Friends in Paldea](src/import.rs#L234) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1421) | ✅ |
| Grimsley's Move | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Gwynn](src/import.rs#L1112) | ✅ |
| [Harlequin](src/import.rs#L940) | ✅ |
| [Hassel](src/import.rs#L877) | ✅ |
| [Hilda](src/import.rs#L1169) | ✅ |
| [Iris's Fighting Spirit](src/import.rs#L952) | ✅ |
| [Jacinthe](src/import.rs#L740) | ✅ |
| [Janine's Secret Art](src/import.rs#L1499) | ✅ |
| [Jasmine's Gaze](src/import.rs#L429) | ✅ |
| [Jett](src/import.rs#L837) | ✅ |
| [Judge](src/import.rs#L991) | ✅ |
| [Kieran](src/import.rs#L1453) | ✅ |
| [Kofu](src/import.rs#L1698) | ✅ |
| [Lacey](src/import.rs#L932) | ✅ |
| [Lana's Aid](src/import.rs#L1370) | ✅ |
| [Larry's Skill](src/import.rs#L1425) | ✅ |
| [Lillie's Determination](src/import.rs#L992) | ✅ |
| [Lisia's Appeal](src/import.rs#L1601) | ✅ |
| Lt. Surge's Bargain | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Lucian](src/import.rs#L928) | ✅ |
| [Misty's Vitality](src/import.rs#L747) | ✅ |
| [Morty's Conviction](src/import.rs#L1463) | ✅ |
| [N's Plan](src/import.rs#L1388) | ✅ |
| [Naveen](src/import.rs#L951) | ✅ |
| [Perrin](src/import.rs#L1702) | ✅ |
| [Philippe](src/import.rs#L777) | ✅ |
| [Picnicker](src/import.rs#L761) | ✅ |
| [Pokémon Center Lady](src/import.rs#L1389) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1391) | ✅ |
| [Roxie's Performance](src/import.rs#L439) | ✅ |
| Ruffian | ❌ — needs a target-then-discard-both phase, scoped to one Pokémon chosen up front |
| [Rust Syndicate Grunt](src/import.rs#L1384) | ✅ |
| [Salvatore](src/import.rs#L1600) | ✅ |
| [Surfer](src/import.rs#L1409) | ✅ |
| [Tarragon](src/import.rs#L763) | ✅ |
| [Team Rocket's Archer](src/import.rs#L819) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1561) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1569) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1274) | ✅ |
| [Team Rocket's Proton](src/import.rs#L513) | ✅ |
| Tyme | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Urbain](src/import.rs#L418) | ✅ |
| [Waitress](src/import.rs#L863) | ✅ |
| [Wally's Compassion](src/import.rs#L1498) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1467) | ✅ |

### Items (68/85 built)

| Card | Status |
| --- | --- |
| Accompanying Flute | ❌ — not yet triaged |
| [Antique Armor Fossil](src/import.rs#L234) | ✅ |
| [Antique Cover Fossil](src/import.rs#L234) | ✅ |
| [Antique Jaw Fossil](src/import.rs#L234) | ✅ |
| [Antique Plume Fossil](src/import.rs#L234) | ✅ |
| [Antique Root Fossil](src/import.rs#L234) | ✅ |
| Antique Sail Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| [Antique Skull Fossil](src/import.rs#L234) | ✅ |
| Arven's Sandwich | ❌ — not yet triaged |
| [Awakening Drum](src/import.rs#L445) | ✅ |
| Blowtorch | ❌ — not yet triaged |
| [Boxed Order](src/import.rs#L684) | ✅ |
| [Brilliant Blender](src/import.rs#L628) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L1084) | ✅ |
| [Bug Catching Set](src/import.rs#L1330) | ✅ |
| Call Bell | ❌ — needs the "playable on the first turn" allowance |
| Chill Teaser Toy | ❌ — needs the "playable on the first turn" allowance |
| [Crushing Hammer](src/import.rs#L1027) | ✅ |
| [Dangerous Laser](src/import.rs#L616) | ✅ |
| [Dark Bell](src/import.rs#L623) | ✅ |
| Deduction Kit | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Dragon Elixir](src/import.rs#L727) | ✅ |
| [Dusk Ball](src/import.rs#L1529) | ✅ |
| [Energy Coin](src/import.rs#L482) | ✅ |
| [Energy Recycler](src/import.rs#L1684) | ✅ |
| [Energy Retrieval](src/import.rs#L1514) | ✅ |
| [Energy Search](src/import.rs#L1500) | ✅ |
| [Energy Search Pro](src/import.rs#L1042) | ✅ |
| Energy Swatter | ❌ — not yet triaged |
| [Energy Switch](src/import.rs#L1154) | ✅ |
| [Enhanced Hammer](src/import.rs#L972) | ✅ |
| [Fighting Gong](src/import.rs#L1535) | ✅ |
| [Glass Trumpet](src/import.rs#L973) | ✅ |
| Great Haul Net | ❌ — not yet triaged |
| [Hand Trimmer](src/import.rs#L1528) | ✅ |
| [Hole-Digging Shovel](src/import.rs#L420) | ✅ |
| [Hop's Bag](src/import.rs#L527) | ✅ |
| [Hyper Aroma](src/import.rs#L656) | ✅ |
| [Iron Defender](src/import.rs#L433) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1366) | ✅ |
| Love Ball | ❌ — not yet triaged |
| [Lumiose Galette](src/import.rs#L726) | ✅ |
| [Master Ball](src/import.rs#L642) | ✅ |
| [Max Rod](src/import.rs#L698) | ✅ |
| Meddling Memo | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Mega Signal](src/import.rs#L1056) | ✅ |
| Megaton Blower | ❌ — not yet triaged |
| [Miracle Headset](src/import.rs#L712) | ✅ |
| [N's PP Up](src/import.rs#L1288) | ✅ |
| [Night Stretcher](src/import.rs#L999) | ✅ |
| Ogre's Mask | ❌ — not yet triaged |
| [Poké Ball](src/import.rs#L468) | ✅ |
| [Poké Pad](src/import.rs#L1013) | ✅ |
| [Poké Vital A](src/import.rs#L736) | ✅ |
| [Pokégear 3.0](src/import.rs#L1316) | ✅ |
| [Pokémon Catcher](src/import.rs#L1531) | ✅ |
| [Potion](src/import.rs#L731) | ✅ |
| [Precious Trolley](src/import.rs#L1028) | ✅ |
| [Premium Power Pro](src/import.rs#L1417) | ✅ |
| [Prime Catcher](src/import.rs#L1530) | ✅ |
| [Rare Candy](src/import.rs#L1273) | ✅ |
| [Reboot Pod](src/import.rs#L449) | ✅ |
| Redeemable Ticket | ❌ — not yet triaged |
| [Repel](src/import.rs#L627) | ✅ |
| Roto-Stick | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Sacred Ash](src/import.rs#L1126) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1602) | ✅ |
| Scramble Switch | ❌ — not yet triaged |
| [Secret Box](src/import.rs#L1633) | ✅ |
| [Special Red Card](src/import.rs#L1246) | ✅ |
| [Strange Timepiece](src/import.rs#L1549) | ✅ |
| [Super Potion](src/import.rs#L735) | ✅ |
| [Switch](src/import.rs#L1365) | ✅ |
| [TM Machine](src/import.rs#L1070) | ✅ |
| Team Rocket's Bother-Bot | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Team Rocket's Great Ball](src/import.rs#L1573) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1670) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1603) | ✅ |
| [Tera Orb](src/import.rs#L1155) | ✅ |
| [Tool Scrapper](src/import.rs#L967) | ✅ |
| [Transformation Tome](src/import.rs#L1629) | ✅ |
| [Treasure Tracker](src/import.rs#L670) | ✅ |
| [Ultra Ball](src/import.rs#L1140) | ✅ |
| [Unfair Stamp](src/import.rs#L1358) | ✅ |
| [Wondrous Patch](src/import.rs#L1302) | ✅ |

### Tools (27/35 built)

| Card | Status |
| --- | --- |
| [Adversity Policy](src/import.rs#L615) | ✅ |
| [Air Balloon](src/import.rs#L1550) | ✅ |
| Amulet of Hope | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Babiri Berry](src/import.rs#L577) | ✅ |
| Backtrack Badge | ❌ — not yet triaged |
| [Binding Mochi](src/import.rs#L1553) | ✅ |
| [Brave Bangle](src/import.rs#L1552) | ✅ |
| [Colbur Berry](src/import.rs#L581) | ✅ |
| Core Memory | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Counter Gain](src/import.rs#L459) | ✅ |
| [Cynthia's Power Weight](src/import.rs#L541) | ✅ |
| Deluxe Bomb | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Gravity Gemstone](src/import.rs#L451) | ✅ |
| [Haban Berry](src/import.rs#L597) | ✅ |
| [Handheld Fan](src/import.rs#L1557) | ✅ |
| Heavy Baton | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Hero's Cape](src/import.rs#L1551) | ✅ |
| [Hop's Choice Band](src/import.rs#L460) | ✅ |
| [Light Ball](src/import.rs#L422) | ✅ |
| [Lillie's Pearl](src/import.rs#L1554) | ✅ |
| [Lucky Helmet](src/import.rs#L1556) | ✅ |
| [Maximum Belt](src/import.rs#L421) | ✅ |
| [Occa Berry](src/import.rs#L585) | ✅ |
| [Passho Berry](src/import.rs#L589) | ✅ |
| [Payapa Berry](src/import.rs#L593) | ✅ |
| [Powerglass](src/import.rs#L1558) | ✅ |
| [Punk Helmet](src/import.rs#L1555) | ✅ |
| [Rescue Board](src/import.rs#L428) | ✅ |
| [Sacred Charm](src/import.rs#L426) | ✅ |
| [Sparkling Crystal](src/import.rs#L455) | ✅ |
| Survival Brace | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Team Rocket's Hypnotizer](src/import.rs#L608) | ✅ |
| Technical Machine: Fluorite | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Thick Scale](src/import.rs#L601) | ✅ |
| Tremendous Bomb | ❌ — not yet triaged |

### Stadiums (25/31 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1611) | ✅ |
| Ange Floette | ❌ — not yet triaged |
| [Area Zero Underdepths](src/import.rs#L988) | ✅ |
| [Battle Cage](src/import.rs#L989) | ✅ |
| Celebratory Fanfare | ❌ — not yet triaged |
| [Community Center](src/import.rs#L1618) | ✅ |
| Dizzying Valley | ❌ — not yet triaged |
| [Festival Grounds](src/import.rs#L1625) | ✅ |
| [Forest of Vitality](src/import.rs#L1624) | ✅ |
| Fossil Quarry | ❌ — not yet triaged |
| [Full Metal Lab](src/import.rs#L573) | ✅ |
| Grand Tree | ❌ — not yet triaged |
| [Granite Cave](src/import.rs#L545) | ✅ |
| [Gravity Mountain](src/import.rs#L1559) | ✅ |
| [Jamming Tower](src/import.rs#L1622) | ✅ |
| [Levincia](src/import.rs#L557) | ✅ |
| [Lively Stadium](src/import.rs#L427) | ✅ |
| [Lumiose City](src/import.rs#L1616) | ✅ |
| [Mystery Garden](src/import.rs#L565) | ✅ |
| [N's Castle](src/import.rs#L1560) | ✅ |
| Neutralization Zone | ❌ — not yet triaged |
| [Nighttime Mine](src/import.rs#L987) | ✅ |
| [Paradise Resort](src/import.rs#L553) | ✅ |
| [Perilous Jungle](src/import.rs#L450) | ✅ |
| [Postwick](src/import.rs#L549) | ✅ |
| [Prism Tower](src/import.rs#L1617) | ✅ |
| [Risky Ruins](src/import.rs#L1623) | ✅ |
| [Spikemuth Gym](src/import.rs#L561) | ✅ |
| [Surfing Beach](src/import.rs#L569) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1612) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L990) | ✅ |

### Special Energy (15/17 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L2075) | ✅ |
| [Bubbly Water Energy](src/import.rs#L2087) | ✅ |
| [Enriching Energy](src/import.rs#L2058) | ✅ |
| [Growing Grass Energy](src/import.rs#L2057) | ✅ |
| [Ignition Energy](src/import.rs#L2099) | ✅ |
| Legacy Energy | ❌ — its Prize-count clause needs the deferred Prize-count mechanic |
| [Magnetic Metal Energy](src/import.rs#L2084) | ✅ |
| [Mist Energy](src/import.rs#L2072) | ✅ |
| [Neo Upper Energy](src/import.rs#L2105) | ✅ |
| [Nitro Fire Energy](src/import.rs#L2090) | ✅ |
| [Prism Energy](src/import.rs#L2078) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L2081) | ✅ |
| [Shadowy Darkness Energy](src/import.rs#L2096) | ✅ |
| [Spiky Energy](src/import.rs#L2069) | ✅ |
| Team Rocket's Energy | ❌ — attaches only to a Team Rocket's Pokémon and discards itself instantly off any other — nothing validates an attach against the carrier's identity yet |
| [Telepathic Psychic Energy](src/import.rs#L2061) | ✅ |
| [Voltaic Lightning Energy](src/import.rs#L2093) | ✅ |

### Pokémon (field) (114/204 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2312) | [✅](src/import.rs#L2312) | [✅](src/import.rs#L2174) |
| [Alakazam](src/import.rs#L2467) | [✅](src/import.rs#L2467) | [✅](src/import.rs#L2164) |
| [Annihilape](src/import.rs#L2341) | [✅](src/import.rs#L2341) | [✅](src/import.rs#L2130) |
| [Applin](src/import.rs#L2302) | [✅](src/import.rs#L2302) | — |
| Archaludon | ❌ | ❌ |
| Archaludon ex | ❌ | ❌ |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2344) | [✅](src/import.rs#L2344) | — |
| [Beldum](src/import.rs#L2324) | [✅](src/import.rs#L2324) | — |
| Blaziken | ❌ | — |
| [Blaziken ex](src/import.rs#L2408) | [✅](src/import.rs#L2408) | [✅](src/import.rs#L2204) |
| Bloodmoon Ursaluna | ❌ | ❌ |
| [Bloodmoon Ursaluna ex](src/import.rs#L2372) | [✅](src/import.rs#L2372) | [✅](src/import.rs#L2138) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2283) | [✅](src/import.rs#L2283) | — |
| [Budew](src/import.rs#L2349) | [✅](src/import.rs#L2349) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2343) | [✅](src/import.rs#L2343) | — |
| [Carvanha](src/import.rs#L2261) | [✅](src/import.rs#L2261) | — |
| [Celebi](src/import.rs#L2342) | [✅](src/import.rs#L2342) | — |
| Ceruledge ex | ❌ | — |
| Chandelure | ❌ | ❌ |
| [Charcadet](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Chi-Yu](src/import.rs#L2440) | [✅](src/import.rs#L2440) | — |
| [Chien-Pao](src/import.rs#L2409) | [✅](src/import.rs#L2409) | [✅](src/import.rs#L2207) |
| [Chikorita](src/import.rs#L2345) | [✅](src/import.rs#L2345) | — |
| Cinderace | ❌ | ❌ |
| [Clefairy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Cofagrigus](src/import.rs#L2390) | [✅](src/import.rs#L2390) | — |
| [Combusken](src/import.rs#L2359) | [✅](src/import.rs#L2359) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2479) | [✅](src/import.rs#L2479) | [✅](src/import.rs#L2119) |
| Cynthia's Gabite | [✅](src/import.rs#L234) | ❌ |
| Cynthia's Garchomp ex | ❌ | — |
| Cynthia's Gible | ❌ | — |
| [Cynthia's Roselia](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Cynthia's Roserade | [✅](src/import.rs#L234) | ❌ |
| Cynthia's Spiritomb | ❌ | — |
| [Dedenne](src/import.rs#L2299) | [✅](src/import.rs#L2299) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2424) | [✅](src/import.rs#L2424) | [✅](src/import.rs#L2192) |
| [Dragapult ex](src/import.rs#L2306) | [✅](src/import.rs#L2306) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2147) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2313) | [✅](src/import.rs#L2313) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2171) |
| [Dudunsparce ex](src/import.rs#L2274) | [✅](src/import.rs#L2274) | — |
| [Dunsparce](src/import.rs#L2325) | [✅](src/import.rs#L2325) | — |
| Duraludon | ❌ | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2175) |
| [Dusknoir](src/import.rs#L2406) | [✅](src/import.rs#L2406) | [✅](src/import.rs#L2176) |
| [Duskull](src/import.rs#L2328) | [✅](src/import.rs#L2328) | — |
| [Dwebble](src/import.rs#L2318) | [✅](src/import.rs#L2318) | — |
| Eelektrik | [✅](src/import.rs#L234) | ❌ |
| [Electrike](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Elgyem](src/import.rs#L2348) | [✅](src/import.rs#L2348) | — |
| Empoleon ex | ❌ | ❌ |
| [Enamorus](src/import.rs#L2368) | [✅](src/import.rs#L2368) | — |
| Ethan's Cyndaquil | ❌ | — |
| Ethan's Quilava | [✅](src/import.rs#L234) | ❌ |
| Ethan's Typhlosion | ❌ | — |
| [Fan Rotom](src/import.rs#L2413) | [✅](src/import.rs#L2413) | [✅](src/import.rs#L2217) |
| [Fezandipiti ex](src/import.rs#L2487) | [✅](src/import.rs#L2487) | [✅](src/import.rs#L2165) |
| [Flutter Mane](src/import.rs#L2404) | [✅](src/import.rs#L2404) | [✅](src/import.rs#L2141) |
| Frillish | ❌ | — |
| [Froakie](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Frogadier](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Froslass | [✅](src/import.rs#L234) | ❌ |
| Galvantula | ❌ | ❌ |
| [Genesect](src/import.rs#L2459) | [✅](src/import.rs#L2459) | [✅](src/import.rs#L2180) |
| [Genesect ex](src/import.rs#L2407) | [✅](src/import.rs#L2407) | [✅](src/import.rs#L2177) |
| [Goldeen](src/import.rs#L2422) | [✅](src/import.rs#L2422) | [✅](src/import.rs#L2190) |
| Greninja ex | ❌ | — |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| Haxorus | ❌ | — |
| Ho-Oh | ❌ | — |
| [Hoothoot](src/import.rs#L2315) | [✅](src/import.rs#L2315) | [✅](src/import.rs#L2131) |
| Hop's Cramorant | ❌ | — |
| Hop's Dubwool | [✅](src/import.rs#L234) | ❌ |
| Hop's Phantump | ❌ | — |
| Hop's Snorlax | ❌ | ❌ |
| Hop's Trevenant | ❌ | — |
| [Hop's Wooloo](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hop's Zacian ex | ❌ | — |
| [Hydrapple ex](src/import.rs#L2373) | [✅](src/import.rs#L2373) | [✅](src/import.rs#L2132) |
| [Iron Crown ex](src/import.rs#L2333) | [✅](src/import.rs#L2333) | [✅](src/import.rs#L2127) |
| [Iron Leaves ex](src/import.rs#L2412) | [✅](src/import.rs#L2412) | [✅](src/import.rs#L2208) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Jellicent ex | ❌ | ❌ |
| Joltik | ❌ | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2160) |
| [Koraidon ex](src/import.rs#L2334) | [✅](src/import.rs#L2334) | — |
| [Kyurem](src/import.rs#L2456) | [✅](src/import.rs#L2456) | [✅](src/import.rs#L2199) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2473) | [✅](src/import.rs#L2473) | [✅](src/import.rs#L2118) |
| [Lillie's Clefairy ex](src/import.rs#L2497) | [✅](src/import.rs#L2497) | [✅](src/import.rs#L2122) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Manectric | ❌ | — |
| Maractus | ❌ | ❌ |
| Marnie's Grimmsnarl ex | ❌ | ❌ |
| Marnie's Impidimp | ❌ | — |
| [Marnie's Morgrem](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Marnie's Morpeko | ❌ | — |
| [Mega Absol ex](src/import.rs#L2380) | [✅](src/import.rs#L2380) | — |
| Mega Audino ex | ❌ | — |
| Mega Chandelure ex | ❌ | ❌ |
| Mega Diancie ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2376) | [✅](src/import.rs#L2376) | — |
| Mega Froslass ex | ❌ | — |
| Mega Greninja ex | ❌ | ❌ |
| Mega Hawlucha ex | ❌ | ❌ |
| [Mega Kangaskhan ex](src/import.rs#L2483) | [✅](src/import.rs#L2483) | [✅](src/import.rs#L2115) |
| [Mega Lopunny ex](src/import.rs#L2281) | [✅](src/import.rs#L2281) | — |
| Mega Lucario ex | ❌ | — |
| Mega Manectric ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2316) | [✅](src/import.rs#L2316) | — |
| [Mega Skarmory ex](src/import.rs#L2394) | [✅](src/import.rs#L2394) | — |
| [Mega Slowbro ex](src/import.rs#L2434) | [✅](src/import.rs#L2434) | — |
| Mega Starmie ex | ❌ | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2186) |
| [Meowth ex](src/import.rs#L2486) | [✅](src/import.rs#L2486) | [✅](src/import.rs#L2159) |
| [Metagross](src/import.rs#L2290) | [✅](src/import.rs#L2290) | — |
| [Metang](src/import.rs#L2474) | [✅](src/import.rs#L2474) | [✅](src/import.rs#L2150) |
| [Moltres](src/import.rs#L2326) | [✅](src/import.rs#L2326) | — |
| Mr. Mime | ❌ | — |
| [Munkidori](src/import.rs#L2480) | [✅](src/import.rs#L2480) | [✅](src/import.rs#L2153) |
| [N's Darmanitan](src/import.rs#L2271) | [✅](src/import.rs#L2271) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| N's Purrloin | ❌ | — |
| [N's Reshiram](src/import.rs#L2268) | [✅](src/import.rs#L2268) | — |
| [N's Zekrom](src/import.rs#L2280) | [✅](src/import.rs#L2280) | — |
| [N's Zoroark ex](src/import.rs#L2453) | [✅](src/import.rs#L2453) | [✅](src/import.rs#L2183) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2476) | [✅](src/import.rs#L2476) | [✅](src/import.rs#L2161) |
| [Paldean Tauros](src/import.rs#L2264) | [✅](src/import.rs#L2264) | — |
| [Passimian](src/import.rs#L2277) | [✅](src/import.rs#L2277) | — |
| [Patrat](src/import.rs#L2475) | [✅](src/import.rs#L2475) | [✅](src/import.rs#L2120) |
| Pawmot | ❌ | — |
| [Pecharunt](src/import.rs#L2417) | [✅](src/import.rs#L2417) | [✅](src/import.rs#L2187) |
| [Pecharunt ex](src/import.rs#L2414) | [✅](src/import.rs#L2414) | [✅](src/import.rs#L2224) |
| [Pikachu ex](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Pikipek | ❌ | — |
| [Piplup](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2121) |
| Purrloin | ❌ | — |
| [Rabsca](src/import.rs#L2330) | [✅](src/import.rs#L2330) | [✅](src/import.rs#L2126) |
| [Raging Bolt ex](src/import.rs#L2295) | [✅](src/import.rs#L2295) | — |
| Regigigas | ❌ | — |
| Relicanth | [✅](src/import.rs#L234) | ❌ |
| [Rellor](src/import.rs#L2262) | [✅](src/import.rs#L2262) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2423) | [✅](src/import.rs#L2423) | [✅](src/import.rs#L2191) |
| [Shaymin](src/import.rs#L2470) | [✅](src/import.rs#L2470) | [✅](src/import.rs#L2125) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2319) | [✅](src/import.rs#L2319) | — |
| [Slowpoke](src/import.rs#L2327) | [✅](src/import.rs#L2327) | ❌ |
| [Smoochum](src/import.rs#L2387) | [✅](src/import.rs#L2387) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spectrier | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Staryu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Stunfisk](src/import.rs#L2369) | [✅](src/import.rs#L2369) | — |
| [Tapu Bulu](src/import.rs#L2263) | [✅](src/import.rs#L2263) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2144) |
| [Teal Mask Ogerpon ex](src/import.rs#L2488) | [✅](src/import.rs#L2488) | [✅](src/import.rs#L2168) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Honchkrow | ❌ | — |
| Team Rocket's Kangaskhan ex | ❌ | — |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Murkrow | ❌ | — |
| Team Rocket's Porygon | ❌ | — |
| Team Rocket's Porygon2 | ❌ | — |
| Team Rocket's Sneasel | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| Team Rocket's Wobbuffet | ❌ | — |
| Terapagos ex | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2193) |
| Togekiss | [✅](src/import.rs#L234) | ❌ |
| [Togepi](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Torchic](src/import.rs#L2329) | [✅](src/import.rs#L2329) | — |
| Toucannon | ❌ | ❌ |
| [Toxel](src/import.rs#L2314) | [✅](src/import.rs#L2314) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2211) |
| Trumbeak | ❌ | — |
| Tynamo | ❌ | — |
| Victini | [✅](src/import.rs#L234) | ❌ |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2304) | [✅](src/import.rs#L2304) | — |
| [Yveltal](src/import.rs#L2303) | [✅](src/import.rs#L2303) | — |
| [Zeraora](src/import.rs#L2286) | [✅](src/import.rs#L2286) | — |
| Zoroark | ❌ | — |

