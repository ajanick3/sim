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
cargo run --bin coverage             # 978 of 3051 Standard cards (32.1%)
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
| Supporters | 68 | 78 |
| Items | 61 | 85 |
| Tools | 23 | 35 |
| Stadiums | 19 | 31 |
| Special Energy | 15 | 17 |

### Supporters (68/78 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1284) | ✅ |
| [Acerola's Mischief](src/import.rs#L1486) | ✅ |
| Amarys | ❌ — not yet triaged |
| Anthea & Concordia | ❌ — not yet triaged |
| [Bianca's Devotion](src/import.rs#L1269) | ✅ |
| [Billy & O'Nare](src/import.rs#L743) | ✅ |
| [Black Belt's Training](src/import.rs#L1292) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L847) | ✅ |
| [Brock's Scouting](src/import.rs#L1354) | ✅ |
| [Canari](src/import.rs#L702) | ✅ |
| Caretaker | ❌ — not yet triaged |
| [Carmine](src/import.rs#L659) | ✅ |
| [Cassiopeia](src/import.rs#L720) | ✅ |
| [Cheren](src/import.rs#L234) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1223) | ✅ |
| [Clemont's Quick Wit](src/import.rs#L839) | ✅ |
| [Colress's Tenacity](src/import.rs#L1072) | ✅ |
| [Cook](src/import.rs#L380) | ✅ |
| [Crispin](src/import.rs#L1129) | ✅ |
| [Cyrano](src/import.rs#L977) | ✅ |
| [Dawn](src/import.rs#L1095) | ✅ |
| [Drasna](src/import.rs#L752) | ✅ |
| [Drayton](src/import.rs#L784) | ✅ |
| [Emcee's Hype](src/import.rs#L735) | ✅ |
| [Emma](src/import.rs#L751) | ✅ |
| [Eri](src/import.rs#L1347) | ✅ |
| [Ethan's Adventure](src/import.rs#L435) | ✅ |
| [Explorer's Guidance](src/import.rs#L770) | ✅ |
| [Fennel](src/import.rs#L835) | ✅ |
| [Firebreather](src/import.rs#L688) | ✅ |
| [Friends in Paldea](src/import.rs#L234) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1300) | ✅ |
| Grimsley's Move | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Gwynn](src/import.rs#L991) | ✅ |
| [Harlequin](src/import.rs#L819) | ✅ |
| [Hassel](src/import.rs#L756) | ✅ |
| [Hilda](src/import.rs#L1048) | ✅ |
| [Iris's Fighting Spirit](src/import.rs#L831) | ✅ |
| [Jacinthe](src/import.rs#L637) | ✅ |
| [Janine's Secret Art](src/import.rs#L1378) | ✅ |
| [Jasmine's Gaze](src/import.rs#L390) | ✅ |
| [Jett](src/import.rs#L734) | ✅ |
| [Judge](src/import.rs#L870) | ✅ |
| [Kieran](src/import.rs#L1332) | ✅ |
| Kofu | ❌ — not yet triaged |
| [Lacey](src/import.rs#L811) | ✅ |
| [Lana's Aid](src/import.rs#L1249) | ✅ |
| [Larry's Skill](src/import.rs#L1304) | ✅ |
| [Lillie's Determination](src/import.rs#L871) | ✅ |
| [Lisia's Appeal](src/import.rs#L1480) | ✅ |
| Lt. Surge's Bargain | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Lucian](src/import.rs#L807) | ✅ |
| [Misty's Vitality](src/import.rs#L644) | ✅ |
| [Morty's Conviction](src/import.rs#L1342) | ✅ |
| [N's Plan](src/import.rs#L1267) | ✅ |
| [Naveen](src/import.rs#L830) | ✅ |
| Perrin | ❌ — not yet triaged |
| [Philippe](src/import.rs#L674) | ✅ |
| [Picnicker](src/import.rs#L658) | ✅ |
| [Pokémon Center Lady](src/import.rs#L1268) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1270) | ✅ |
| [Roxie's Performance](src/import.rs#L400) | ✅ |
| Ruffian | ❌ — needs a target-then-discard-both phase, scoped to one Pokémon chosen up front |
| [Rust Syndicate Grunt](src/import.rs#L1263) | ✅ |
| [Salvatore](src/import.rs#L1479) | ✅ |
| [Surfer](src/import.rs#L1288) | ✅ |
| [Tarragon](src/import.rs#L660) | ✅ |
| [Team Rocket's Archer](src/import.rs#L716) | ✅ |
| [Team Rocket's Ariana](src/import.rs#L1440) | ✅ |
| [Team Rocket's Giovanni](src/import.rs#L1448) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1153) | ✅ |
| [Team Rocket's Proton](src/import.rs#L449) | ✅ |
| Tyme | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Urbain](src/import.rs#L379) | ✅ |
| Waitress | ❌ — not yet triaged |
| [Wally's Compassion](src/import.rs#L1377) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1346) | ✅ |

### Items (61/85 built)

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
| [Boxed Order](src/import.rs#L581) | ✅ |
| [Brilliant Blender](src/import.rs#L525) | ✅ |
| [Buddy-Buddy Poffin](src/import.rs#L963) | ✅ |
| [Bug Catching Set](src/import.rs#L1209) | ✅ |
| Call Bell | ❌ — needs the "playable on the first turn" allowance |
| Chill Teaser Toy | ❌ — needs the "playable on the first turn" allowance |
| [Crushing Hammer](src/import.rs#L906) | ✅ |
| [Dangerous Laser](src/import.rs#L513) | ✅ |
| [Dark Bell](src/import.rs#L520) | ✅ |
| Deduction Kit | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Dragon Elixir](src/import.rs#L624) | ✅ |
| [Dusk Ball](src/import.rs#L1408) | ✅ |
| Energy Coin | ❌ — not yet triaged |
| [Energy Recycler](src/import.rs#L1563) | ✅ |
| [Energy Retrieval](src/import.rs#L1393) | ✅ |
| [Energy Search](src/import.rs#L1379) | ✅ |
| [Energy Search Pro](src/import.rs#L921) | ✅ |
| Energy Swatter | ❌ — not yet triaged |
| [Energy Switch](src/import.rs#L1033) | ✅ |
| [Enhanced Hammer](src/import.rs#L851) | ✅ |
| [Fighting Gong](src/import.rs#L1414) | ✅ |
| [Glass Trumpet](src/import.rs#L852) | ✅ |
| Great Haul Net | ❌ — not yet triaged |
| [Hand Trimmer](src/import.rs#L1407) | ✅ |
| [Hole-Digging Shovel](src/import.rs#L381) | ✅ |
| [Hop's Bag](src/import.rs#L463) | ✅ |
| [Hyper Aroma](src/import.rs#L553) | ✅ |
| [Iron Defender](src/import.rs#L394) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1245) | ✅ |
| Love Ball | ❌ — not yet triaged |
| [Lumiose Galette](src/import.rs#L623) | ✅ |
| [Master Ball](src/import.rs#L539) | ✅ |
| [Max Rod](src/import.rs#L595) | ✅ |
| Meddling Memo | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Mega Signal](src/import.rs#L935) | ✅ |
| Megaton Blower | ❌ — not yet triaged |
| [Miracle Headset](src/import.rs#L609) | ✅ |
| [N's PP Up](src/import.rs#L1167) | ✅ |
| [Night Stretcher](src/import.rs#L878) | ✅ |
| Ogre's Mask | ❌ — not yet triaged |
| [Poké Ball](src/import.rs#L421) | ✅ |
| [Poké Pad](src/import.rs#L892) | ✅ |
| [Poké Vital A](src/import.rs#L633) | ✅ |
| [Pokégear 3.0](src/import.rs#L1195) | ✅ |
| [Pokémon Catcher](src/import.rs#L1410) | ✅ |
| [Potion](src/import.rs#L628) | ✅ |
| [Precious Trolley](src/import.rs#L907) | ✅ |
| [Premium Power Pro](src/import.rs#L1296) | ✅ |
| [Prime Catcher](src/import.rs#L1409) | ✅ |
| [Rare Candy](src/import.rs#L1152) | ✅ |
| [Reboot Pod](src/import.rs#L410) | ✅ |
| Redeemable Ticket | ❌ — not yet triaged |
| [Repel](src/import.rs#L524) | ✅ |
| Roto-Stick | ❌ — needs to peek and then discard or reorder — today's peek always shuffles back |
| [Sacred Ash](src/import.rs#L1005) | ✅ |
| [Scoop Up Cyclone](src/import.rs#L1481) | ✅ |
| Scramble Switch | ❌ — not yet triaged |
| [Secret Box](src/import.rs#L1512) | ✅ |
| [Special Red Card](src/import.rs#L1125) | ✅ |
| [Strange Timepiece](src/import.rs#L1428) | ✅ |
| [Super Potion](src/import.rs#L632) | ✅ |
| [Switch](src/import.rs#L1244) | ✅ |
| [TM Machine](src/import.rs#L949) | ✅ |
| Team Rocket's Bother-Bot | ❌ — needs an interactive opponent choice the engine has no shape for yet |
| [Team Rocket's Great Ball](src/import.rs#L1452) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1549) | ✅ |
| [Team Rocket's Venture Bomb](src/import.rs#L1482) | ✅ |
| [Tera Orb](src/import.rs#L1034) | ✅ |
| [Tool Scrapper](src/import.rs#L846) | ✅ |
| [Transformation Tome](src/import.rs#L1508) | ✅ |
| [Treasure Tracker](src/import.rs#L567) | ✅ |
| [Ultra Ball](src/import.rs#L1019) | ✅ |
| [Unfair Stamp](src/import.rs#L1237) | ✅ |
| [Wondrous Patch](src/import.rs#L1181) | ✅ |

### Tools (23/35 built)

| Card | Status |
| --- | --- |
| Adversity Policy | ❌ — not yet triaged |
| [Air Balloon](src/import.rs#L1429) | ✅ |
| Amulet of Hope | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Babiri Berry](src/import.rs#L489) | ✅ |
| Backtrack Badge | ❌ — not yet triaged |
| [Binding Mochi](src/import.rs#L1432) | ✅ |
| [Brave Bangle](src/import.rs#L1431) | ✅ |
| [Colbur Berry](src/import.rs#L493) | ✅ |
| Core Memory | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| [Counter Gain](src/import.rs#L420) | ✅ |
| [Cynthia's Power Weight](src/import.rs#L477) | ✅ |
| Deluxe Bomb | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Gravity Gemstone](src/import.rs#L412) | ✅ |
| [Haban Berry](src/import.rs#L509) | ✅ |
| [Handheld Fan](src/import.rs#L1436) | ✅ |
| Heavy Baton | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| [Hero's Cape](src/import.rs#L1430) | ✅ |
| Hop's Choice Band | ❌ — not yet triaged |
| [Light Ball](src/import.rs#L383) | ✅ |
| [Lillie's Pearl](src/import.rs#L1433) | ✅ |
| [Lucky Helmet](src/import.rs#L1435) | ✅ |
| [Maximum Belt](src/import.rs#L382) | ✅ |
| [Occa Berry](src/import.rs#L497) | ✅ |
| [Passho Berry](src/import.rs#L501) | ✅ |
| [Payapa Berry](src/import.rs#L505) | ✅ |
| [Powerglass](src/import.rs#L1437) | ✅ |
| [Punk Helmet](src/import.rs#L1434) | ✅ |
| [Rescue Board](src/import.rs#L389) | ✅ |
| [Sacred Charm](src/import.rs#L387) | ✅ |
| [Sparkling Crystal](src/import.rs#L416) | ✅ |
| Survival Brace | ❌ — needs a pause for a choice at the moment of knockout, before cards move to discard |
| Team Rocket's Hypnotizer | ❌ — not yet triaged |
| Technical Machine: Fluorite | ❌ — the Tool grants an attack; today an attack only ever comes from a Pokémon's own printed list |
| Thick Scale | ❌ — not yet triaged |
| Tremendous Bomb | ❌ — not yet triaged |

### Stadiums (19/31 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1490) | ✅ |
| Ange Floette | ❌ — not yet triaged |
| [Area Zero Underdepths](src/import.rs#L867) | ✅ |
| [Battle Cage](src/import.rs#L868) | ✅ |
| Celebratory Fanfare | ❌ — not yet triaged |
| [Community Center](src/import.rs#L1497) | ✅ |
| Dizzying Valley | ❌ — not yet triaged |
| [Festival Grounds](src/import.rs#L1504) | ✅ |
| [Forest of Vitality](src/import.rs#L1503) | ✅ |
| Fossil Quarry | ❌ — not yet triaged |
| [Full Metal Lab](src/import.rs#L485) | ✅ |
| Grand Tree | ❌ — not yet triaged |
| [Granite Cave](src/import.rs#L481) | ✅ |
| [Gravity Mountain](src/import.rs#L1438) | ✅ |
| [Jamming Tower](src/import.rs#L1501) | ✅ |
| Levincia | ❌ — not yet triaged |
| [Lively Stadium](src/import.rs#L388) | ✅ |
| [Lumiose City](src/import.rs#L1495) | ✅ |
| Mystery Garden | ❌ — not yet triaged |
| [N's Castle](src/import.rs#L1439) | ✅ |
| Neutralization Zone | ❌ — not yet triaged |
| [Nighttime Mine](src/import.rs#L866) | ✅ |
| Paradise Resort | ❌ — not yet triaged |
| [Perilous Jungle](src/import.rs#L411) | ✅ |
| Postwick | ❌ — not yet triaged |
| [Prism Tower](src/import.rs#L1496) | ✅ |
| [Risky Ruins](src/import.rs#L1502) | ✅ |
| Spikemuth Gym | ❌ — not yet triaged |
| Surfing Beach | ❌ — not yet triaged |
| [Team Rocket's Factory](src/import.rs#L1491) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L869) | ✅ |

### Special Energy (15/17 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1942) | ✅ |
| [Bubbly Water Energy](src/import.rs#L1954) | ✅ |
| [Enriching Energy](src/import.rs#L1925) | ✅ |
| [Growing Grass Energy](src/import.rs#L1924) | ✅ |
| [Ignition Energy](src/import.rs#L1966) | ✅ |
| Legacy Energy | ❌ — its Prize-count clause needs the deferred Prize-count mechanic |
| [Magnetic Metal Energy](src/import.rs#L1951) | ✅ |
| [Mist Energy](src/import.rs#L1939) | ✅ |
| [Neo Upper Energy](src/import.rs#L1972) | ✅ |
| [Nitro Fire Energy](src/import.rs#L1957) | ✅ |
| [Prism Energy](src/import.rs#L1945) | ✅ |
| [Rocky Fighting Energy](src/import.rs#L1948) | ✅ |
| [Shadowy Darkness Energy](src/import.rs#L1963) | ✅ |
| [Spiky Energy](src/import.rs#L1936) | ✅ |
| Team Rocket's Energy | ❌ — attaches only to a Team Rocket's Pokémon and discards itself instantly off any other — nothing validates an attach against the carrier's identity yet |
| [Telepathic Psychic Energy](src/import.rs#L1928) | ✅ |
| [Voltaic Lightning Energy](src/import.rs#L1960) | ✅ |

### Pokémon (field) (102/131 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L2159) | [✅](src/import.rs#L2159) | [✅](src/import.rs#L2041) |
| [Alakazam](src/import.rs#L2314) | [✅](src/import.rs#L2314) | [✅](src/import.rs#L2031) |
| [Annihilape](src/import.rs#L2188) | [✅](src/import.rs#L2188) | [✅](src/import.rs#L1997) |
| [Applin](src/import.rs#L2149) | [✅](src/import.rs#L2149) | — |
| Banette | ❌ | ❌ |
| [Bayleef](src/import.rs#L2191) | [✅](src/import.rs#L2191) | — |
| [Beldum](src/import.rs#L2171) | [✅](src/import.rs#L2171) | — |
| [Blaziken ex](src/import.rs#L2255) | [✅](src/import.rs#L2255) | [✅](src/import.rs#L2071) |
| [Bloodmoon Ursaluna ex](src/import.rs#L2219) | [✅](src/import.rs#L2219) | [✅](src/import.rs#L2005) |
| Bouffalant | ❌ | ❌ |
| Bronzong | [✅](src/import.rs#L234) | ❌ |
| [Bronzor](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Brute Bonnet](src/import.rs#L2130) | [✅](src/import.rs#L2130) | — |
| [Budew](src/import.rs#L2196) | [✅](src/import.rs#L2196) | — |
| Bulbasaur | ❌ | — |
| [Buneary](src/import.rs#L2190) | [✅](src/import.rs#L2190) | — |
| [Carvanha](src/import.rs#L2108) | [✅](src/import.rs#L2108) | — |
| [Celebi](src/import.rs#L2189) | [✅](src/import.rs#L2189) | — |
| Chandelure | ❌ | ❌ |
| [Chi-Yu](src/import.rs#L2287) | [✅](src/import.rs#L2287) | — |
| [Chien-Pao](src/import.rs#L2256) | [✅](src/import.rs#L2256) | [✅](src/import.rs#L2074) |
| [Chikorita](src/import.rs#L2192) | [✅](src/import.rs#L2192) | — |
| Cinderace | ❌ | ❌ |
| [Cofagrigus](src/import.rs#L2237) | [✅](src/import.rs#L2237) | — |
| [Combusken](src/import.rs#L2206) | [✅](src/import.rs#L2206) | — |
| Comfey | ❌ | — |
| Cornerstone Mask Ogerpon ex | ❌ | ❌ |
| [Crustle](src/import.rs#L2326) | [✅](src/import.rs#L2326) | [✅](src/import.rs#L1986) |
| [Dedenne](src/import.rs#L2146) | [✅](src/import.rs#L2146) | — |
| Dhelmise | ❌ | — |
| [Dipplin](src/import.rs#L2271) | [✅](src/import.rs#L2271) | [✅](src/import.rs#L2059) |
| [Dragapult ex](src/import.rs#L2153) | [✅](src/import.rs#L2153) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2014) |
| Drapion | ❌ | — |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L2160) | [✅](src/import.rs#L2160) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2038) |
| [Dudunsparce ex](src/import.rs#L2121) | [✅](src/import.rs#L2121) | — |
| [Dunsparce](src/import.rs#L2172) | [✅](src/import.rs#L2172) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2042) |
| [Dusknoir](src/import.rs#L2253) | [✅](src/import.rs#L2253) | [✅](src/import.rs#L2043) |
| [Duskull](src/import.rs#L2175) | [✅](src/import.rs#L2175) | — |
| [Dwebble](src/import.rs#L2165) | [✅](src/import.rs#L2165) | — |
| [Elgyem](src/import.rs#L2195) | [✅](src/import.rs#L2195) | — |
| [Enamorus](src/import.rs#L2215) | [✅](src/import.rs#L2215) | — |
| [Fan Rotom](src/import.rs#L2260) | [✅](src/import.rs#L2260) | [✅](src/import.rs#L2084) |
| [Fezandipiti ex](src/import.rs#L2334) | [✅](src/import.rs#L2334) | [✅](src/import.rs#L2032) |
| [Flutter Mane](src/import.rs#L2251) | [✅](src/import.rs#L2251) | [✅](src/import.rs#L2008) |
| [Genesect](src/import.rs#L2306) | [✅](src/import.rs#L2306) | [✅](src/import.rs#L2047) |
| [Genesect ex](src/import.rs#L2254) | [✅](src/import.rs#L2254) | [✅](src/import.rs#L2044) |
| [Goldeen](src/import.rs#L2269) | [✅](src/import.rs#L2269) | [✅](src/import.rs#L2057) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Hariyama | ❌ | ❌ |
| [Hoothoot](src/import.rs#L2162) | [✅](src/import.rs#L2162) | [✅](src/import.rs#L1998) |
| [Hydrapple ex](src/import.rs#L2220) | [✅](src/import.rs#L2220) | [✅](src/import.rs#L1999) |
| [Iron Crown ex](src/import.rs#L2180) | [✅](src/import.rs#L2180) | [✅](src/import.rs#L1994) |
| [Iron Leaves ex](src/import.rs#L2259) | [✅](src/import.rs#L2259) | [✅](src/import.rs#L2075) |
| [Ivysaur](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2027) |
| [Koraidon ex](src/import.rs#L2181) | [✅](src/import.rs#L2181) | — |
| [Kyurem](src/import.rs#L2303) | [✅](src/import.rs#L2303) | [✅](src/import.rs#L2066) |
| Lampent | ❌ | — |
| [Latias ex](src/import.rs#L2320) | [✅](src/import.rs#L2320) | [✅](src/import.rs#L1985) |
| [Lillie's Clefairy ex](src/import.rs#L2344) | [✅](src/import.rs#L2344) | [✅](src/import.rs#L1989) |
| [Litwick](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Lunatone | [✅](src/import.rs#L234) | ❌ |
| [Makuhita](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Mega Absol ex](src/import.rs#L2227) | [✅](src/import.rs#L2227) | — |
| Mega Chandelure ex | ❌ | ❌ |
| [Mega Excadrill ex](src/import.rs#L2223) | [✅](src/import.rs#L2223) | — |
| Mega Froslass ex | ❌ | — |
| [Mega Kangaskhan ex](src/import.rs#L2330) | [✅](src/import.rs#L2330) | [✅](src/import.rs#L1982) |
| [Mega Lopunny ex](src/import.rs#L2128) | [✅](src/import.rs#L2128) | — |
| Mega Lucario ex | ❌ | — |
| Mega Meganium ex | ❌ | — |
| [Mega Sharpedo ex](src/import.rs#L2163) | [✅](src/import.rs#L2163) | — |
| [Mega Skarmory ex](src/import.rs#L2241) | [✅](src/import.rs#L2241) | — |
| [Mega Slowbro ex](src/import.rs#L2281) | [✅](src/import.rs#L2281) | — |
| Mega Venusaur ex | ❌ | ❌ |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2053) |
| [Meowth ex](src/import.rs#L2333) | [✅](src/import.rs#L2333) | [✅](src/import.rs#L2026) |
| [Metagross](src/import.rs#L2137) | [✅](src/import.rs#L2137) | — |
| [Metang](src/import.rs#L2321) | [✅](src/import.rs#L2321) | [✅](src/import.rs#L2017) |
| [Moltres](src/import.rs#L2173) | [✅](src/import.rs#L2173) | — |
| [Munkidori](src/import.rs#L2327) | [✅](src/import.rs#L2327) | [✅](src/import.rs#L2020) |
| [N's Darmanitan](src/import.rs#L2118) | [✅](src/import.rs#L2118) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L2115) | [✅](src/import.rs#L2115) | — |
| [N's Zekrom](src/import.rs#L2127) | [✅](src/import.rs#L2127) | — |
| [N's Zoroark ex](src/import.rs#L2300) | [✅](src/import.rs#L2300) | [✅](src/import.rs#L2050) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2323) | [✅](src/import.rs#L2323) | [✅](src/import.rs#L2028) |
| [Paldean Tauros](src/import.rs#L2111) | [✅](src/import.rs#L2111) | — |
| [Passimian](src/import.rs#L2124) | [✅](src/import.rs#L2124) | — |
| [Patrat](src/import.rs#L2322) | [✅](src/import.rs#L2322) | [✅](src/import.rs#L1987) |
| [Pecharunt](src/import.rs#L2264) | [✅](src/import.rs#L2264) | [✅](src/import.rs#L2054) |
| [Pecharunt ex](src/import.rs#L2261) | [✅](src/import.rs#L2261) | [✅](src/import.rs#L2091) |
| Poltchageist | [✅](src/import.rs#L234) | ❌ |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1988) |
| [Rabsca](src/import.rs#L2177) | [✅](src/import.rs#L2177) | [✅](src/import.rs#L1993) |
| [Raging Bolt ex](src/import.rs#L2142) | [✅](src/import.rs#L2142) | — |
| Regigigas | ❌ | — |
| [Rellor](src/import.rs#L2109) | [✅](src/import.rs#L2109) | — |
| [Riolu](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Seaking](src/import.rs#L2270) | [✅](src/import.rs#L2270) | [✅](src/import.rs#L2058) |
| [Shaymin](src/import.rs#L2317) | [✅](src/import.rs#L2317) | [✅](src/import.rs#L1992) |
| [Shuppet](src/import.rs#L234) | [✅](src/import.rs#L234) | ❌ |
| Sinistcha | ❌ | ❌ |
| [Slowking](src/import.rs#L2166) | [✅](src/import.rs#L2166) | — |
| [Slowpoke](src/import.rs#L2174) | [✅](src/import.rs#L2174) | ❌ |
| [Smoochum](src/import.rs#L2234) | [✅](src/import.rs#L2234) | — |
| [Snorunt](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| Solrock | ❌ | — |
| Spiritomb | ❌ | ❌ |
| [Stunfisk](src/import.rs#L2216) | [✅](src/import.rs#L2216) | — |
| [Tapu Bulu](src/import.rs#L2110) | [✅](src/import.rs#L2110) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2011) |
| [Teal Mask Ogerpon ex](src/import.rs#L2335) | [✅](src/import.rs#L2335) | [✅](src/import.rs#L2035) |
| Team Rocket's Articuno | ❌ | ❌ |
| Team Rocket's Mewtwo ex | ❌ | ❌ |
| Team Rocket's Mimikyu | ❌ | — |
| Team Rocket's Spidops | ❌ | ❌ |
| Team Rocket's Tarountula | ❌ | — |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2060) |
| [Torchic](src/import.rs#L2176) | [✅](src/import.rs#L2176) | — |
| [Toxel](src/import.rs#L2161) | [✅](src/import.rs#L2161) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L2078) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L2151) | [✅](src/import.rs#L2151) | — |
| [Yveltal](src/import.rs#L2150) | [✅](src/import.rs#L2150) | — |
| [Zeraora](src/import.rs#L2133) | [✅](src/import.rs#L2133) | — |
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

