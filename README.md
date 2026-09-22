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

`decks/2026-worlds/` holds the field of the 2026 World Championships, one file
per player, named `<placement>-<player-slug>.txt`, zero-padded to three
digits. `tools/fetch_worlds_decklists.py` fetches all 143 from limitlesstcg's
Decklists tab, the tournament's Day 2 standings — the site holds no decklist
for an entrant who did not reach one. All 143 are kept and check clean.

### Standard coverage

Every card in the artifact, by name. The Trainer and Special Energy tables below cover the whole pool; the Pokémon table stays scoped to the field (the decks under `decks/`).

| Kind | Built | Total |
| --- | --- | --- |
| Supporters | 73 | 78 |
| Items | 62 | 85 |
| Tools | 27 | 35 |
| Stadiums | 25 | 31 |
| Special Energy | 15 | 17 |

### Supporters (73/78 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1366) | ✅ |
| [Acerola's Mischief](src/import.rs#L1568) | ✅ |
| [Amarys](src/import.rs#L820) | ✅ |
| Anthea & Concordia | ❌ — not yet triaged |
| [Bianca's Devotion](src/import.rs#L1351) | ✅ |
| [Billy & O'Nare](src/import.rs#L807) | ✅ |
| [Black Belt's Training](src/import.rs#L1374) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L929) | ✅ |
| [Brock's Scouting](src/import.rs#L1436) | ✅ |
| [Canari](src/import.rs#L766) | ✅ |
| [Caretaker](src/import.rs#L1667) | ✅ |
| [Carmine](src/import.rs#L723) | ✅ |
| [Cassiopeia](src/import.rs#L784) | ✅ |
| [Cheren](src/import.rs#L234) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1305) | ✅ |
| [Clemont's Quick Wit](src/import.rs#L921) | ✅ |
| [Colress's Tenacity](src/import.rs#L1154) | ✅ |
| [Cook](src/import.rs#L380) | ✅ |
| [Crispin](src/import.rs#L1211) | ✅ |
| [Cyrano](src/import.rs#L1059) | ✅ |
| [Dawn](src/import.rs#L1177) | ✅ |
| [Drasna](src/import.rs#L816) | ✅ |
| [Drayton](src/import.rs#L866) | ✅ |
| [Emcee's Hype](src/import.rs#L799) | ✅ |
| [Emma](src/import.rs#L815) | ✅ |
| [Eri](src/import.rs#L1429) | ✅ |
| [Ethan's Adventure](src/import.rs#L460) | ✅ |
| [Explorer's Guidance](src/import.rs#L852) | ✅ |
| [Fennel](src/import.rs#L917) | ✅ |
| [Firebreather](src/import.rs#L752) | ✅ |
| [Friends in Paldea](src/import.rs#L234) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1382) | ✅ |
| Grimsley's Move | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Gwynn](src/import.rs#L1073) | ✅ |
| [Harlequin](src/import.rs#L901) | ✅ |
| [Hassel](src/import.rs#L838) | ✅ |
| [Hilda](src/import.rs#L1130) | ✅ |
| [Iris's Fighting Spirit](src/import.rs#L913) | ✅ |
| [Jacinthe](src/import.rs#L701) | ✅ |
| [Janine's Secret Art](src/import.rs#L1460) | ✅ |
| [Jasmine's Gaze](src/import.rs#L390) | ✅ |
| [Jett](src/import.rs#L798) | ✅ |
| [Judge](src/import.rs#L952) | ✅ |
| [Kieran](src/import.rs#L1414) | ✅ |
| [Kofu](src/import.rs#L1659) | ✅ |
| [Lacey](src/import.rs#L893) | ✅ |
| [Lana's Aid](src/import.rs#L1331) | ✅ |
| [Larry's Skill](src/import.rs#L1386) | ✅ |
| [Lillie's Determination](src/import.rs#L953) | ✅ |
| [Lisia's Appeal](src/import.rs#L1562) | ✅ |
| Lt. Surge's Bargain | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Lucian](src/import.rs#L889) | ✅ |
| [Misty's Vitality](src/import.rs#L708) | ✅ |
| [Morty's Conviction](src/import.rs#L1424) | ✅ |
| [N's Plan](src/import.rs#L1349) | ✅ |
| [Naveen](src/import.rs#L912) | ✅ |
| [Perrin](src/import.rs#L1663) | ✅ |
| [Philippe](src/import.rs#L738) | ✅ |
| [Picnicker](src/import.rs#L722) | ✅ |
| [Pokémon Center Lady](src/import.rs#L1350) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1352) | ✅ |
| [Roxie's Performance](src/import.rs#L400) | ✅ |
| Ruffian | ❌ — needs a target-then-discard-both phase, scoped to one Pokémon chosen up front |
| [Rust Syndicate Grunt](src/import.rs#L1345) | ✅ |
| [Salvatore](src/import.rs#L1561) | ✅ |
| [Surfer](src/import.rs#L1370) | ✅ |
| [Tarragon](src/import.rs#L724) | ✅ |
| [Team Rocket's Archer](src/import.rs#L780) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1522) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1530) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1235) | ✅ |
| [Team Rocket's Proton](src/import.rs#L474) | ✅ |
| Tyme | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Urbain](src/import.rs#L379) | ✅ |
| [Waitress](src/import.rs#L824) | ✅ |
| [Wally's Compassion](src/import.rs#L1459) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1428) | ✅ |

### Items (62/85 built)

| Card | Status |
| --- | --- |
| Accompanying Flute | ❌ — not yet triaged |
| Antique Armor Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Cover Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Jaw Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Plume Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Root Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Sail Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Antique Skull Fossil | ❌ — plays as a 60-HP Basic Pokémon — a whole mechanic, no seam for it yet |
| Arven's Sandwich | ❌ — not yet triaged |
| [Awakening Drum](src/import.rs#L406) | ✅ |
| Blowtorch | ❌ — not yet triaged |
| [Boxed Order](src/import.rs#L645) | ✅ |
| [Brilliant Blender](src/import.rs#L589) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L1045) | ✅ |
| [Bug Catching Set](src/import.rs#L1291) | ✅ |
| Call Bell | ❌ — needs the "playable on the first turn" allowance |
| Chill Teaser Toy | ❌ — needs the "playable on the first turn" allowance |
| [Crushing Hammer](src/import.rs#L988) | ✅ |
| [Dangerous Laser](src/import.rs#L577) | ✅ |
| [Dark Bell](src/import.rs#L584) | ✅ |
| Deduction Kit | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Dragon Elixir](src/import.rs#L688) | ✅ |
| [Dusk Ball](src/import.rs#L1490) | ✅ |
| [Energy Coin](src/import.rs#L443) | ✅ |
| [Energy Recycler](src/import.rs#L1645) | ✅ |
| [Energy Retrieval](src/import.rs#L1475) | ✅ |
| [Energy Search](src/import.rs#L1461) | ✅ |
| [Energy Search Pro](src/import.rs#L1003) | ✅ |
| Energy Swatter | ❌ — not yet triaged |
| [Energy Switch](src/import.rs#L1115) | ✅ |
| [Enhanced Hammer](src/import.rs#L933) | ✅ |
| [Fighting Gong](src/import.rs#L1496) | ✅ |
| [Glass Trumpet](src/import.rs#L934) | ✅ |
| Great Haul Net | ❌ — not yet triaged |
| [Hand Trimmer](src/import.rs#L1489) | ✅ |
| [Hole-Digging Shovel](src/import.rs#L381) | ✅ |
| [Hop's Bag](src/import.rs#L488) | ✅ |
| [Hyper Aroma](src/import.rs#L617) | ✅ |
| [Iron Defender](src/import.rs#L394) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1327) | ✅ |
| Love Ball | ❌ — not yet triaged |
| [Lumiose Galette](src/import.rs#L687) | ✅ |
| [Master Ball](src/import.rs#L603) | ✅ |
| [Max Rod](src/import.rs#L659) | ✅ |
| Meddling Memo | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Mega Signal](src/import.rs#L1017) | ✅ |
| Megaton Blower | ❌ — not yet triaged |
| [Miracle Headset](src/import.rs#L673) | ✅ |
| [N's PP Up](src/import.rs#L1249) | ✅ |
| [Night Stretcher](src/import.rs#L960) | ✅ |
| Ogre's Mask | ❌ — not yet triaged |
| [Poké Ball](src/import.rs#L429) | ✅ |
| [Poké Pad](src/import.rs#L974) | ✅ |
| [Poké Vital A](src/import.rs#L697) | ✅ |
| [Pokégear 3.0](src/import.rs#L1277) | ✅ |
| [Pokémon Catcher](src/import.rs#L1492) | ✅ |
| [Potion](src/import.rs#L692) | ✅ |
| [Precious Trolley](src/import.rs#L989) | ✅ |
| [Premium Power Pro](src/import.rs#L1378) | ✅ |
| [Prime Catcher](src/import.rs#L1491) | ✅ |
| [Rare Candy](src/import.rs#L1234) | ✅ |
| [Reboot Pod](src/import.rs#L410) | ✅ |
| Redeemable Ticket | ❌ — not yet triaged |
| [Repel](src/import.rs#L588) | ✅ |
| Roto-Stick | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Sacred Ash](src/import.rs#L1087) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1563) | ✅ |
| Scramble Switch | ❌ — not yet triaged |
| [Secret Box](src/import.rs#L1594) | ✅ |
| [Special Red Card](src/import.rs#L1207) | ✅ |
| [Strange Timepiece](src/import.rs#L1510) | ✅ |
| [Super Potion](src/import.rs#L696) | ✅ |
| [Switch](src/import.rs#L1326) | ✅ |
| [TM Machine](src/import.rs#L1031) | ✅ |
| Team Rocket's Bother-Bot | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Team Rocket's Great Ball](src/import.rs#L1534) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1631) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1564) | ✅ |
| [Tera Orb](src/import.rs#L1116) | ✅ |
| [Tool Scrapper](src/import.rs#L928) | ✅ |
| [Transformation Tome](src/import.rs#L1590) | ✅ |
| [Treasure Tracker](src/import.rs#L631) | ✅ |
| [Ultra Ball](src/import.rs#L1101) | ✅ |
| [Unfair Stamp](src/import.rs#L1319) | ✅ |
| [Wondrous Patch](src/import.rs#L1263) | ✅ |

### Tools (27/35 built)

| Card | Status |
| --- | --- |
| [Adversity Policy](src/import.rs#L576) | ✅ |
| [Air Balloon](src/import.rs#L1511) | ✅ |
| Amulet of Hope | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Babiri Berry](src/import.rs#L538) | ✅ |
| Backtrack Badge | ❌ — not yet triaged |
| [Binding Mochi](src/import.rs#L1514) | ✅ |
| [Brave Bangle](src/import.rs#L1513) | ✅ |
| [Colbur Berry](src/import.rs#L542) | ✅ |
| Core Memory | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Counter Gain](src/import.rs#L420) | ✅ |
| [Cynthia's Power Weight](src/import.rs#L502) | ✅ |
| Deluxe Bomb | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Gravity Gemstone](src/import.rs#L412) | ✅ |
| [Haban Berry](src/import.rs#L558) | ✅ |
| [Handheld Fan](src/import.rs#L1518) | ✅ |
| Heavy Baton | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Hero's Cape](src/import.rs#L1512) | ✅ |
| [Hop's Choice Band](src/import.rs#L421) | ✅ |
| [Light Ball](src/import.rs#L383) | ✅ |
| [Lillie's Pearl](src/import.rs#L1515) | ✅ |
| [Lucky Helmet](src/import.rs#L1517) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Occa Berry](src/import.rs#L546) | ✅ |
| [Passho Berry](src/import.rs#L550) | ✅ |
| [Payapa Berry](src/import.rs#L554) | ✅ |
| [Powerglass](src/import.rs#L1519) | ✅ |
| [Punk Helmet](src/import.rs#L1516) | ✅ |
| [Rescue Board](src/import.rs#L389) | ✅ |
| [Sacred Charm](src/import.rs#L387) | ✅ |
| [Sparkling Crystal](src/import.rs#L416) | ✅ |
| Survival Brace | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Team Rocket's Hypnotizer](src/import.rs#L569) | ✅ |
| Technical Machine: Fluorite | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Thick Scale](src/import.rs#L562) | ✅ |
| Tremendous Bomb | ❌ — not yet triaged |

### Stadiums (25/31 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1572) | ✅ |
| Ange Floette | ❌ — not yet triaged |
| [Area Zero Underdepths](src/import.rs#L949) | ✅ |
| [Battle Cage](src/import.rs#L950) | ✅ |
| Celebratory Fanfare | ❌ — not yet triaged |
| [Community Center](src/import.rs#L1579) | ✅ |
| Dizzying Valley | ❌ — not yet triaged |
| [Festival Grounds](src/import.rs#L1586) | ✅ |
| [Forest of Vitality](src/import.rs#L1585) | ✅ |
| Fossil Quarry | ❌ — not yet triaged |
| [Full Metal Lab](src/import.rs#L534) | ✅ |
| Grand Tree | ❌ — not yet triaged |
| [Granite Cave](src/import.rs#L506) | ✅ |
| [Gravity Mountain](src/import.rs#L1520) | ✅ |
| [Jamming Tower](src/import.rs#L1583) | ✅ |
| [Levincia](src/import.rs#L518) | ✅ |
| [Lively Stadium](src/import.rs#L388) | ✅ |
| [Lumiose City](src/import.rs#L1577) | ✅ |
| [Mystery Garden](src/import.rs#L526) | ✅ |
| [N's Castle](src/import.rs#L1521) | ✅ |
| Neutralization Zone | ❌ — not yet triaged |
| [Nighttime Mine](src/import.rs#L948) | ✅ |
| [Paradise Resort](src/import.rs#L514) | ✅ |
| [Perilous Jungle](src/import.rs#L411) | ✅ |
| [Postwick](src/import.rs#L510) | ✅ |
| [Prism Tower](src/import.rs#L1578) | ✅ |
| [Risky Ruins](src/import.rs#L1584) | ✅ |
| [Spikemuth Gym](src/import.rs#L522) | ✅ |
| [Surfing Beach](src/import.rs#L530) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1573) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L951) | ✅ |

### Special Energy (15/17 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L2036) | ✅ |
| [Bubbly Water Energy](src/import.rs#L2048) | ✅ |
| [Enriching Energy](src/import.rs#L2019) | ✅ |
| [Growing Grass Energy](src/import.rs#L2018) | ✅ |
| [Ignition Energy](src/import.rs#L2060) | ✅ |
| Legacy Energy | ❌ — its Prize-count clause needs the deferred Prize-count mechanic |
| [Magnetic Metal Energy](src/import.rs#L2045) | ✅ |
| [Mist Energy](src/import.rs#L2033) | ✅ |
| [Neo Upper Energy](src/import.rs#L2066) | ✅ |
| [Nitro Fire Energy](src/import.rs#L2051) | ✅ |
| [Prism Energy](src/import.rs#L2039) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L2042) | ✅ |
| [Shadowy Darkness Energy](src/import.rs#L2057) | ✅ |
| [Spiky Energy](src/import.rs#L2030) | ✅ |
| Team Rocket's Energy | ❌ — attaches only to a Team Rocket's Pokémon and discards itself instantly off any other — nothing validates an attach against the carrier's identity yet |
| [Telepathic Psychic Energy](src/import.rs#L2022) | ✅ |
| [Voltaic Lightning Energy](src/import.rs#L2054) | ✅ |

### Pokémon (field) (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2253) | [✅](src/import.rs#L2253) | [✅](src/import.rs#L2135) |
| [Alakazam](src/import.rs#L2408) | [✅](src/import.rs#L2408) | [✅](src/import.rs#L2125) |
| [Annihilape](src/import.rs#L2282) | [✅](src/import.rs#L2282) | [✅](src/import.rs#L2091) |
| [Applin](src/import.rs#L2243) | [✅](src/import.rs#L2243) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2285) | [✅](src/import.rs#L2285) | — |
| [Beldum](src/import.rs#L2265) | [✅](src/import.rs#L2265) | — |
| [Blaziken ex](src/import.rs#L2349) | [✅](src/import.rs#L2349) | [✅](src/import.rs#L2165) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2313) | [✅](src/import.rs#L2313) | [✅](src/import.rs#L2099) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2224) | [✅](src/import.rs#L2224) | — |
| [Budew](src/import.rs#L2290) | [✅](src/import.rs#L2290) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2284) | [✅](src/import.rs#L2284) | — |
| [Carvanha](src/import.rs#L2202) | [✅](src/import.rs#L2202) | — |
| [Celebi](src/import.rs#L2283) | [✅](src/import.rs#L2283) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2381) | [✅](src/import.rs#L2381) | — |
| [Chien-Pao](src/import.rs#L2350) | [✅](src/import.rs#L2350) | [✅](src/import.rs#L2168) |
| [Chikorita](src/import.rs#L2286) | [✅](src/import.rs#L2286) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2331) | [✅](src/import.rs#L2331) | — |
| [Combusken](src/import.rs#L2300) | [✅](src/import.rs#L2300) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2420) | [✅](src/import.rs#L2420) | [✅](src/import.rs#L2080) |
| [Dedenne](src/import.rs#L2240) | [✅](src/import.rs#L2240) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2365) | [✅](src/import.rs#L2365) | [✅](src/import.rs#L2153) |
| [Dragapult ex](src/import.rs#L2247) | [✅](src/import.rs#L2247) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2108) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2254) | [✅](src/import.rs#L2254) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2132) |
| [Dudunsparce ex](src/import.rs#L2215) | [✅](src/import.rs#L2215) | — |
| [Dunsparce](src/import.rs#L2266) | [✅](src/import.rs#L2266) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2136) |
| [Dusknoir](src/import.rs#L2347) | [✅](src/import.rs#L2347) | [✅](src/import.rs#L2137) |
| [Duskull](src/import.rs#L2269) | [✅](src/import.rs#L2269) | — |
| [Dwebble](src/import.rs#L2259) | [✅](src/import.rs#L2259) | — |
| [Elgyem](src/import.rs#L2289) | [✅](src/import.rs#L2289) | — |
| [Enamorus](src/import.rs#L2309) | [✅](src/import.rs#L2309) | — |
| [Fan Rotom](src/import.rs#L2354) | [✅](src/import.rs#L2354) | [✅](src/import.rs#L2178) |
| [Fezandipiti ex](src/import.rs#L2428) | [✅](src/import.rs#L2428) | [✅](src/import.rs#L2126) |
| [Flutter Mane](src/import.rs#L2345) | [✅](src/import.rs#L2345) | [✅](src/import.rs#L2102) |
| [Genesect](src/import.rs#L2400) | [✅](src/import.rs#L2400) | [✅](src/import.rs#L2141) |
| [Genesect ex](src/import.rs#L2348) | [✅](src/import.rs#L2348) | [✅](src/import.rs#L2138) |
| [Goldeen](src/import.rs#L2363) | [✅](src/import.rs#L2363) | [✅](src/import.rs#L2151) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2256) | [✅](src/import.rs#L2256) | [✅](src/import.rs#L2092) |
| [Hydrapple ex](src/import.rs#L2314) | [✅](src/import.rs#L2314) | [✅](src/import.rs#L2093) |
| [Iron Crown ex](src/import.rs#L2274) | [✅](src/import.rs#L2274) | [✅](src/import.rs#L2088) |
| [Iron Leaves ex](src/import.rs#L2353) | [✅](src/import.rs#L2353) | [✅](src/import.rs#L2169) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2121) |
| [Koraidon ex](src/import.rs#L2275) | [✅](src/import.rs#L2275) | — |
| [Kyurem](src/import.rs#L2397) | [✅](src/import.rs#L2397) | [✅](src/import.rs#L2160) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2414) | [✅](src/import.rs#L2414) | [✅](src/import.rs#L2079) |
| [Lillie's Clefairy ex](src/import.rs#L2438) | [✅](src/import.rs#L2438) | [✅](src/import.rs#L2083) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2321) | [✅](src/import.rs#L2321) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2317) | [✅](src/import.rs#L2317) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2424) | [✅](src/import.rs#L2424) | [✅](src/import.rs#L2076) |
| [Mega Lopunny ex](src/import.rs#L2222) | [✅](src/import.rs#L2222) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2257) | [✅](src/import.rs#L2257) | — |
| [Mega Skarmory ex](src/import.rs#L2335) | [✅](src/import.rs#L2335) | — |
| [Mega Slowbro ex](src/import.rs#L2375) | [✅](src/import.rs#L2375) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2147) |
| [Meowth ex](src/import.rs#L2427) | [✅](src/import.rs#L2427) | [✅](src/import.rs#L2120) |
| [Metagross](src/import.rs#L2231) | [✅](src/import.rs#L2231) | — |
| [Metang](src/import.rs#L2415) | [✅](src/import.rs#L2415) | [✅](src/import.rs#L2111) |
| [Moltres](src/import.rs#L2267) | [✅](src/import.rs#L2267) | — |
| [Munkidori](src/import.rs#L2421) | [✅](src/import.rs#L2421) | [✅](src/import.rs#L2114) |
| [N's Darmanitan](src/import.rs#L2212) | [✅](src/import.rs#L2212) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2209) | [✅](src/import.rs#L2209) | — |
| [N's Zekrom](src/import.rs#L2221) | [✅](src/import.rs#L2221) | — |
| [N's Zoroark ex](src/import.rs#L2394) | [✅](src/import.rs#L2394) | [✅](src/import.rs#L2144) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2417) | [✅](src/import.rs#L2417) | [✅](src/import.rs#L2122) |
| [Paldean Tauros](src/import.rs#L2205) | [✅](src/import.rs#L2205) | — |
| [Passimian](src/import.rs#L2218) | [✅](src/import.rs#L2218) | — |
| [Patrat](src/import.rs#L2416) | [✅](src/import.rs#L2416) | [✅](src/import.rs#L2081) |
| [Pecharunt](src/import.rs#L2358) | [✅](src/import.rs#L2358) | [✅](src/import.rs#L2148) |
| [Pecharunt ex](src/import.rs#L2355) | [✅](src/import.rs#L2355) | [✅](src/import.rs#L2185) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2082) |
| [Rabsca](src/import.rs#L2271) | [✅](src/import.rs#L2271) | [✅](src/import.rs#L2087) |
| [Raging Bolt ex](src/import.rs#L2236) | [✅](src/import.rs#L2236) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L2203) | [✅](src/import.rs#L2203) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2364) | [✅](src/import.rs#L2364) | [✅](src/import.rs#L2152) |
| [Shaymin](src/import.rs#L2411) | [✅](src/import.rs#L2411) | [✅](src/import.rs#L2086) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2260) | [✅](src/import.rs#L2260) | — |
| [Slowpoke](src/import.rs#L2268) | [✅](src/import.rs#L2268) | ❌ |
| [Smoochum](src/import.rs#L2328) | [✅](src/import.rs#L2328) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2310) | [✅](src/import.rs#L2310) | — |
| [Tapu Bulu](src/import.rs#L2204) | [✅](src/import.rs#L2204) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2105) |
| [Teal Mask Ogerpon ex](src/import.rs#L2429) | [✅](src/import.rs#L2429) | [✅](src/import.rs#L2129) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2154) |
| [Torchic](src/import.rs#L2270) | [✅](src/import.rs#L2270) | — |
| [Toxel](src/import.rs#L2255) | [✅](src/import.rs#L2255) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2172) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2245) | [✅](src/import.rs#L2245) | — |
| [Yveltal](src/import.rs#L2244) | [✅](src/import.rs#L2244) | — |
| [Zeraora](src/import.rs#L2227) | [✅](src/import.rs#L2227) | — |
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

