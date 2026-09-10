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

### Standard coverage

Every card in the artifact, by name; the tables below track the field.

| Kind | Built | Total |
| --- | --- | --- |
| Supporters | 53 | 78 |
| Items | 43 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1049) | ✅ |
| [Black Belt's Training](src/import.rs#L1057) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L670) | ✅ |
| [Brock's Scouting](src/import.rs#L1087) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L989) | ✅ |
| [Crispin](src/import.rs#L895) | ✅ |
| [Cyrano](src/import.rs#L744) | ✅ |
| [Dawn](src/import.rs#L861) | ✅ |
| [Eri](src/import.rs#L1080) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1061) | ✅ |
| [Gwynn](src/import.rs#L758) | ✅ |
| [Hilda](src/import.rs#L815) | ✅ |
| [Janine's Secret Art](src/import.rs#L1111) | ✅ |
| [Judge](src/import.rs#L693) | ✅ |
| [Kieran](src/import.rs#L1065) | ✅ |
| [Lana's Aid](src/import.rs#L1015) | ✅ |
| [Lillie's Determination](src/import.rs#L694) | ✅ |
| [Morty's Conviction](src/import.rs#L1075) | ✅ |
| [N's Plan](src/import.rs#L1033) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1035) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1029) | ✅ |
| [Surfer](src/import.rs#L1053) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L919) | ✅ |
| [Wally's Compassion](src/import.rs#L1110) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1079) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L730) | ✅ |
| [Bug Catching Set](src/import.rs#L975) | ✅ |
| [Crushing Hammer](src/import.rs#L729) | ✅ |
| [Dusk Ball](src/import.rs#L1141) | ✅ |
| [Energy Recycler](src/import.rs#L1223) | ✅ |
| [Energy Retrieval](src/import.rs#L1126) | ✅ |
| [Energy Search](src/import.rs#L1112) | ✅ |
| [Energy Switch](src/import.rs#L800) | ✅ |
| [Enhanced Hammer](src/import.rs#L674) | ✅ |
| [Glass Trumpet](src/import.rs#L675) | ✅ |
| [Hand Trimmer](src/import.rs#L1140) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1011) | ✅ |
| [N's PP Up](src/import.rs#L933) | ✅ |
| [Night Stretcher](src/import.rs#L701) | ✅ |
| [Prime Catcher](src/import.rs#L1142) | ✅ |
| [Rare Candy](src/import.rs#L918) | ✅ |
| [Sacred Ash](src/import.rs#L772) | ✅ |
| [Secret Box](src/import.rs#L1172) | ✅ |
| [Special Red Card](src/import.rs#L891) | ✅ |
| [Strange Timepiece](src/import.rs#L1143) | ✅ |
| [Switch](src/import.rs#L1010) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1209) | ✅ |
| [Tera Orb](src/import.rs#L801) | ✅ |
| [Tool Scrapper](src/import.rs#L669) | ✅ |
| [Transformation Tome](src/import.rs#L1168) | ✅ |
| [Ultra Ball](src/import.rs#L786) | ✅ |
| [Unfair Stamp](src/import.rs#L1003) | ✅ |
| [Wondrous Patch](src/import.rs#L947) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1144) | ✅ |
| [Binding Mochi](src/import.rs#L1147) | ✅ |
| [Brave Bangle](src/import.rs#L1146) | ✅ |
| [Handheld Fan](src/import.rs#L1151) | ✅ |
| [Hero's Cape](src/import.rs#L1145) | ✅ |
| [Lillie's Pearl](src/import.rs#L1148) | ✅ |
| [Lucky Helmet](src/import.rs#L1150) | ✅ |
| [Powerglass](src/import.rs#L1152) | ✅ |
| [Punk Helmet](src/import.rs#L1149) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1155) | ✅ |
| [Area Zero Underdepths](src/import.rs#L690) | ✅ |
| [Battle Cage](src/import.rs#L691) | ✅ |
| [Festival Grounds](src/import.rs#L1164) | ✅ |
| [Forest of Vitality](src/import.rs#L1163) | ✅ |
| [Gravity Mountain](src/import.rs#L1153) | ✅ |
| [Jamming Tower](src/import.rs#L1161) | ✅ |
| [Lumiose City](src/import.rs#L1160) | ✅ |
| [N's Castle](src/import.rs#L1154) | ✅ |
| [Nighttime Mine](src/import.rs#L689) | ✅ |
| [Risky Ruins](src/import.rs#L1162) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1156) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L692) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1602) | ✅ |
| [Enriching Energy](src/import.rs#L1585) | ✅ |
| [Growing Grass Energy](src/import.rs#L1584) | ✅ |
| [Mist Energy](src/import.rs#L1599) | ✅ |
| [Prism Energy](src/import.rs#L1605) | ✅ |
| [Spiky Energy](src/import.rs#L1596) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1588) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1800) | [✅](src/import.rs#L1800) | [✅](src/import.rs#L1682) |
| [Alakazam](src/import.rs#L1955) | [✅](src/import.rs#L1955) | [✅](src/import.rs#L1672) |
| [Annihilape](src/import.rs#L1829) | [✅](src/import.rs#L1829) | [✅](src/import.rs#L1638) |
| [Applin](src/import.rs#L1790) | [✅](src/import.rs#L1790) | — |
| [Bayleef](src/import.rs#L1832) | [✅](src/import.rs#L1832) | — |
| [Beldum](src/import.rs#L1812) | [✅](src/import.rs#L1812) | — |
| [Blaziken ex](src/import.rs#L1896) | [✅](src/import.rs#L1896) | [✅](src/import.rs#L1712) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1860) | [✅](src/import.rs#L1860) | [✅](src/import.rs#L1646) |
| [Brute Bonnet](src/import.rs#L1771) | [✅](src/import.rs#L1771) | — |
| [Budew](src/import.rs#L1837) | [✅](src/import.rs#L1837) | — |
| [Buneary](src/import.rs#L1831) | [✅](src/import.rs#L1831) | — |
| [Carvanha](src/import.rs#L1749) | [✅](src/import.rs#L1749) | — |
| [Celebi](src/import.rs#L1830) | [✅](src/import.rs#L1830) | — |
| [Chi-Yu](src/import.rs#L1928) | [✅](src/import.rs#L1928) | — |
| [Chien-Pao](src/import.rs#L1897) | [✅](src/import.rs#L1897) | [✅](src/import.rs#L1715) |
| [Chikorita](src/import.rs#L1833) | [✅](src/import.rs#L1833) | — |
| [Cofagrigus](src/import.rs#L1878) | [✅](src/import.rs#L1878) | — |
| [Combusken](src/import.rs#L1847) | [✅](src/import.rs#L1847) | — |
| [Crustle](src/import.rs#L1967) | [✅](src/import.rs#L1967) | [✅](src/import.rs#L1627) |
| [Dedenne](src/import.rs#L1787) | [✅](src/import.rs#L1787) | — |
| [Dipplin](src/import.rs#L1912) | [✅](src/import.rs#L1912) | [✅](src/import.rs#L1700) |
| [Dragapult ex](src/import.rs#L1794) | [✅](src/import.rs#L1794) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1655) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1801) | [✅](src/import.rs#L1801) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1679) |
| [Dudunsparce ex](src/import.rs#L1762) | [✅](src/import.rs#L1762) | — |
| [Dunsparce](src/import.rs#L1813) | [✅](src/import.rs#L1813) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1683) |
| [Dusknoir](src/import.rs#L1894) | [✅](src/import.rs#L1894) | [✅](src/import.rs#L1684) |
| [Duskull](src/import.rs#L1816) | [✅](src/import.rs#L1816) | — |
| [Dwebble](src/import.rs#L1806) | [✅](src/import.rs#L1806) | — |
| [Elgyem](src/import.rs#L1836) | [✅](src/import.rs#L1836) | — |
| [Enamorus](src/import.rs#L1856) | [✅](src/import.rs#L1856) | — |
| [Fan Rotom](src/import.rs#L1901) | [✅](src/import.rs#L1901) | [✅](src/import.rs#L1725) |
| [Fezandipiti ex](src/import.rs#L1975) | [✅](src/import.rs#L1975) | [✅](src/import.rs#L1673) |
| [Flutter Mane](src/import.rs#L1892) | [✅](src/import.rs#L1892) | [✅](src/import.rs#L1649) |
| [Genesect](src/import.rs#L1947) | [✅](src/import.rs#L1947) | [✅](src/import.rs#L1688) |
| [Genesect ex](src/import.rs#L1895) | [✅](src/import.rs#L1895) | [✅](src/import.rs#L1685) |
| [Goldeen](src/import.rs#L1910) | [✅](src/import.rs#L1910) | [✅](src/import.rs#L1698) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1803) | [✅](src/import.rs#L1803) | [✅](src/import.rs#L1639) |
| [Hydrapple ex](src/import.rs#L1861) | [✅](src/import.rs#L1861) | [✅](src/import.rs#L1640) |
| [Iron Crown ex](src/import.rs#L1821) | [✅](src/import.rs#L1821) | [✅](src/import.rs#L1635) |
| [Iron Leaves ex](src/import.rs#L1900) | [✅](src/import.rs#L1900) | [✅](src/import.rs#L1716) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1668) |
| [Koraidon ex](src/import.rs#L1822) | [✅](src/import.rs#L1822) | — |
| [Kyurem](src/import.rs#L1944) | [✅](src/import.rs#L1944) | [✅](src/import.rs#L1707) |
| [Latias ex](src/import.rs#L1961) | [✅](src/import.rs#L1961) | [✅](src/import.rs#L1626) |
| [Lillie's Clefairy ex](src/import.rs#L1985) | [✅](src/import.rs#L1985) | [✅](src/import.rs#L1630) |
| [Mega Absol ex](src/import.rs#L1868) | [✅](src/import.rs#L1868) | — |
| [Mega Excadrill ex](src/import.rs#L1864) | [✅](src/import.rs#L1864) | — |
| [Mega Kangaskhan ex](src/import.rs#L1971) | [✅](src/import.rs#L1971) | [✅](src/import.rs#L1623) |
| [Mega Lopunny ex](src/import.rs#L1769) | [✅](src/import.rs#L1769) | — |
| [Mega Sharpedo ex](src/import.rs#L1804) | [✅](src/import.rs#L1804) | — |
| [Mega Skarmory ex](src/import.rs#L1882) | [✅](src/import.rs#L1882) | — |
| [Mega Slowbro ex](src/import.rs#L1922) | [✅](src/import.rs#L1922) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1694) |
| [Meowth ex](src/import.rs#L1974) | [✅](src/import.rs#L1974) | [✅](src/import.rs#L1667) |
| [Metagross](src/import.rs#L1778) | [✅](src/import.rs#L1778) | — |
| [Metang](src/import.rs#L1962) | [✅](src/import.rs#L1962) | [✅](src/import.rs#L1658) |
| [Moltres](src/import.rs#L1814) | [✅](src/import.rs#L1814) | — |
| [Munkidori](src/import.rs#L1968) | [✅](src/import.rs#L1968) | [✅](src/import.rs#L1661) |
| [N's Darmanitan](src/import.rs#L1759) | [✅](src/import.rs#L1759) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1756) | [✅](src/import.rs#L1756) | — |
| [N's Zekrom](src/import.rs#L1768) | [✅](src/import.rs#L1768) | — |
| [N's Zoroark ex](src/import.rs#L1941) | [✅](src/import.rs#L1941) | [✅](src/import.rs#L1691) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1964) | [✅](src/import.rs#L1964) | [✅](src/import.rs#L1669) |
| [Paldean Tauros](src/import.rs#L1752) | [✅](src/import.rs#L1752) | — |
| [Passimian](src/import.rs#L1765) | [✅](src/import.rs#L1765) | — |
| [Patrat](src/import.rs#L1963) | [✅](src/import.rs#L1963) | [✅](src/import.rs#L1628) |
| [Pecharunt](src/import.rs#L1905) | [✅](src/import.rs#L1905) | [✅](src/import.rs#L1695) |
| [Pecharunt ex](src/import.rs#L1902) | [✅](src/import.rs#L1902) | [✅](src/import.rs#L1732) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1629) |
| [Rabsca](src/import.rs#L1818) | [✅](src/import.rs#L1818) | [✅](src/import.rs#L1634) |
| [Raging Bolt ex](src/import.rs#L1783) | [✅](src/import.rs#L1783) | — |
| [Rellor](src/import.rs#L1750) | [✅](src/import.rs#L1750) | — |
| [Seaking](src/import.rs#L1911) | [✅](src/import.rs#L1911) | [✅](src/import.rs#L1699) |
| [Shaymin](src/import.rs#L1958) | [✅](src/import.rs#L1958) | [✅](src/import.rs#L1633) |
| [Slowking](src/import.rs#L1807) | [✅](src/import.rs#L1807) | — |
| [Slowpoke](src/import.rs#L1815) | [✅](src/import.rs#L1815) | ❌ |
| [Smoochum](src/import.rs#L1875) | [✅](src/import.rs#L1875) | — |
| [Stunfisk](src/import.rs#L1857) | [✅](src/import.rs#L1857) | — |
| [Tapu Bulu](src/import.rs#L1751) | [✅](src/import.rs#L1751) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1652) |
| [Teal Mask Ogerpon ex](src/import.rs#L1976) | [✅](src/import.rs#L1976) | [✅](src/import.rs#L1676) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1701) |
| [Torchic](src/import.rs#L1817) | [✅](src/import.rs#L1817) | — |
| [Toxel](src/import.rs#L1802) | [✅](src/import.rs#L1802) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1719) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1792) | [✅](src/import.rs#L1792) | — |
| [Yveltal](src/import.rs#L1791) | [✅](src/import.rs#L1791) | — |
| [Zeraora](src/import.rs#L1774) | [✅](src/import.rs#L1774) | — |

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

