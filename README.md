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
| Supporters | 54 | 78 |
| Items | 45 | 85 |
| Tools | 20 | 35 |
| Stadiums | 16 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1135) | ✅ |
| [Black Belt's Training](src/import.rs#L1143) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L756) | ✅ |
| [Brock's Scouting](src/import.rs#L1173) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1075) | ✅ |
| [Crispin](src/import.rs#L981) | ✅ |
| [Cyrano](src/import.rs#L830) | ✅ |
| [Dawn](src/import.rs#L947) | ✅ |
| [Eri](src/import.rs#L1166) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1147) | ✅ |
| [Gwynn](src/import.rs#L844) | ✅ |
| [Hilda](src/import.rs#L901) | ✅ |
| [Janine's Secret Art](src/import.rs#L1197) | ✅ |
| [Judge](src/import.rs#L779) | ✅ |
| [Kieran](src/import.rs#L1151) | ✅ |
| [Lana's Aid](src/import.rs#L1101) | ✅ |
| [Lillie's Determination](src/import.rs#L780) | ✅ |
| [Morty's Conviction](src/import.rs#L1161) | ✅ |
| [N's Plan](src/import.rs#L1119) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1121) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1115) | ✅ |
| [Surfer](src/import.rs#L1139) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1005) | ✅ |
| [Wally's Compassion](src/import.rs#L1196) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1165) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L816) | ✅ |
| [Bug Catching Set](src/import.rs#L1061) | ✅ |
| [Crushing Hammer](src/import.rs#L815) | ✅ |
| [Dusk Ball](src/import.rs#L1227) | ✅ |
| [Energy Recycler](src/import.rs#L1309) | ✅ |
| [Energy Retrieval](src/import.rs#L1212) | ✅ |
| [Energy Search](src/import.rs#L1198) | ✅ |
| [Energy Switch](src/import.rs#L886) | ✅ |
| [Enhanced Hammer](src/import.rs#L760) | ✅ |
| [Glass Trumpet](src/import.rs#L761) | ✅ |
| [Hand Trimmer](src/import.rs#L1226) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1097) | ✅ |
| [N's PP Up](src/import.rs#L1019) | ✅ |
| [Night Stretcher](src/import.rs#L787) | ✅ |
| [Prime Catcher](src/import.rs#L1228) | ✅ |
| [Rare Candy](src/import.rs#L1004) | ✅ |
| [Sacred Ash](src/import.rs#L858) | ✅ |
| [Secret Box](src/import.rs#L1258) | ✅ |
| [Special Red Card](src/import.rs#L977) | ✅ |
| [Strange Timepiece](src/import.rs#L1229) | ✅ |
| [Switch](src/import.rs#L1096) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1295) | ✅ |
| [Tera Orb](src/import.rs#L887) | ✅ |
| [Tool Scrapper](src/import.rs#L755) | ✅ |
| [Transformation Tome](src/import.rs#L1254) | ✅ |
| [Ultra Ball](src/import.rs#L872) | ✅ |
| [Unfair Stamp](src/import.rs#L1089) | ✅ |
| [Wondrous Patch](src/import.rs#L1033) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1230) | ✅ |
| [Binding Mochi](src/import.rs#L1233) | ✅ |
| [Brave Bangle](src/import.rs#L1232) | ✅ |
| [Handheld Fan](src/import.rs#L1237) | ✅ |
| [Hero's Cape](src/import.rs#L1231) | ✅ |
| [Lillie's Pearl](src/import.rs#L1234) | ✅ |
| [Lucky Helmet](src/import.rs#L1236) | ✅ |
| [Powerglass](src/import.rs#L1238) | ✅ |
| [Punk Helmet](src/import.rs#L1235) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1241) | ✅ |
| [Area Zero Underdepths](src/import.rs#L776) | ✅ |
| [Battle Cage](src/import.rs#L777) | ✅ |
| [Festival Grounds](src/import.rs#L1250) | ✅ |
| [Forest of Vitality](src/import.rs#L1249) | ✅ |
| [Gravity Mountain](src/import.rs#L1239) | ✅ |
| [Jamming Tower](src/import.rs#L1247) | ✅ |
| [Lumiose City](src/import.rs#L1246) | ✅ |
| [N's Castle](src/import.rs#L1240) | ✅ |
| [Nighttime Mine](src/import.rs#L775) | ✅ |
| [Risky Ruins](src/import.rs#L1248) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1242) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L778) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1688) | ✅ |
| [Enriching Energy](src/import.rs#L1671) | ✅ |
| [Growing Grass Energy](src/import.rs#L1670) | ✅ |
| [Mist Energy](src/import.rs#L1685) | ✅ |
| [Prism Energy](src/import.rs#L1691) | ✅ |
| [Spiky Energy](src/import.rs#L1682) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1674) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1886) | [✅](src/import.rs#L1886) | [✅](src/import.rs#L1768) |
| [Alakazam](src/import.rs#L2041) | [✅](src/import.rs#L2041) | [✅](src/import.rs#L1758) |
| [Annihilape](src/import.rs#L1915) | [✅](src/import.rs#L1915) | [✅](src/import.rs#L1724) |
| [Applin](src/import.rs#L1876) | [✅](src/import.rs#L1876) | — |
| [Bayleef](src/import.rs#L1918) | [✅](src/import.rs#L1918) | — |
| [Beldum](src/import.rs#L1898) | [✅](src/import.rs#L1898) | — |
| [Blaziken ex](src/import.rs#L1982) | [✅](src/import.rs#L1982) | [✅](src/import.rs#L1798) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1946) | [✅](src/import.rs#L1946) | [✅](src/import.rs#L1732) |
| [Brute Bonnet](src/import.rs#L1857) | [✅](src/import.rs#L1857) | — |
| [Budew](src/import.rs#L1923) | [✅](src/import.rs#L1923) | — |
| [Buneary](src/import.rs#L1917) | [✅](src/import.rs#L1917) | — |
| [Carvanha](src/import.rs#L1835) | [✅](src/import.rs#L1835) | — |
| [Celebi](src/import.rs#L1916) | [✅](src/import.rs#L1916) | — |
| [Chi-Yu](src/import.rs#L2014) | [✅](src/import.rs#L2014) | — |
| [Chien-Pao](src/import.rs#L1983) | [✅](src/import.rs#L1983) | [✅](src/import.rs#L1801) |
| [Chikorita](src/import.rs#L1919) | [✅](src/import.rs#L1919) | — |
| [Cofagrigus](src/import.rs#L1964) | [✅](src/import.rs#L1964) | — |
| [Combusken](src/import.rs#L1933) | [✅](src/import.rs#L1933) | — |
| [Crustle](src/import.rs#L2053) | [✅](src/import.rs#L2053) | [✅](src/import.rs#L1713) |
| [Dedenne](src/import.rs#L1873) | [✅](src/import.rs#L1873) | — |
| [Dipplin](src/import.rs#L1998) | [✅](src/import.rs#L1998) | [✅](src/import.rs#L1786) |
| [Dragapult ex](src/import.rs#L1880) | [✅](src/import.rs#L1880) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1741) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1887) | [✅](src/import.rs#L1887) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1765) |
| [Dudunsparce ex](src/import.rs#L1848) | [✅](src/import.rs#L1848) | — |
| [Dunsparce](src/import.rs#L1899) | [✅](src/import.rs#L1899) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1769) |
| [Dusknoir](src/import.rs#L1980) | [✅](src/import.rs#L1980) | [✅](src/import.rs#L1770) |
| [Duskull](src/import.rs#L1902) | [✅](src/import.rs#L1902) | — |
| [Dwebble](src/import.rs#L1892) | [✅](src/import.rs#L1892) | — |
| [Elgyem](src/import.rs#L1922) | [✅](src/import.rs#L1922) | — |
| [Enamorus](src/import.rs#L1942) | [✅](src/import.rs#L1942) | — |
| [Fan Rotom](src/import.rs#L1987) | [✅](src/import.rs#L1987) | [✅](src/import.rs#L1811) |
| [Fezandipiti ex](src/import.rs#L2061) | [✅](src/import.rs#L2061) | [✅](src/import.rs#L1759) |
| [Flutter Mane](src/import.rs#L1978) | [✅](src/import.rs#L1978) | [✅](src/import.rs#L1735) |
| [Genesect](src/import.rs#L2033) | [✅](src/import.rs#L2033) | [✅](src/import.rs#L1774) |
| [Genesect ex](src/import.rs#L1981) | [✅](src/import.rs#L1981) | [✅](src/import.rs#L1771) |
| [Goldeen](src/import.rs#L1996) | [✅](src/import.rs#L1996) | [✅](src/import.rs#L1784) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1889) | [✅](src/import.rs#L1889) | [✅](src/import.rs#L1725) |
| [Hydrapple ex](src/import.rs#L1947) | [✅](src/import.rs#L1947) | [✅](src/import.rs#L1726) |
| [Iron Crown ex](src/import.rs#L1907) | [✅](src/import.rs#L1907) | [✅](src/import.rs#L1721) |
| [Iron Leaves ex](src/import.rs#L1986) | [✅](src/import.rs#L1986) | [✅](src/import.rs#L1802) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1754) |
| [Koraidon ex](src/import.rs#L1908) | [✅](src/import.rs#L1908) | — |
| [Kyurem](src/import.rs#L2030) | [✅](src/import.rs#L2030) | [✅](src/import.rs#L1793) |
| [Latias ex](src/import.rs#L2047) | [✅](src/import.rs#L2047) | [✅](src/import.rs#L1712) |
| [Lillie's Clefairy ex](src/import.rs#L2071) | [✅](src/import.rs#L2071) | [✅](src/import.rs#L1716) |
| [Mega Absol ex](src/import.rs#L1954) | [✅](src/import.rs#L1954) | — |
| [Mega Excadrill ex](src/import.rs#L1950) | [✅](src/import.rs#L1950) | — |
| [Mega Kangaskhan ex](src/import.rs#L2057) | [✅](src/import.rs#L2057) | [✅](src/import.rs#L1709) |
| [Mega Lopunny ex](src/import.rs#L1855) | [✅](src/import.rs#L1855) | — |
| [Mega Sharpedo ex](src/import.rs#L1890) | [✅](src/import.rs#L1890) | — |
| [Mega Skarmory ex](src/import.rs#L1968) | [✅](src/import.rs#L1968) | — |
| [Mega Slowbro ex](src/import.rs#L2008) | [✅](src/import.rs#L2008) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1780) |
| [Meowth ex](src/import.rs#L2060) | [✅](src/import.rs#L2060) | [✅](src/import.rs#L1753) |
| [Metagross](src/import.rs#L1864) | [✅](src/import.rs#L1864) | — |
| [Metang](src/import.rs#L2048) | [✅](src/import.rs#L2048) | [✅](src/import.rs#L1744) |
| [Moltres](src/import.rs#L1900) | [✅](src/import.rs#L1900) | — |
| [Munkidori](src/import.rs#L2054) | [✅](src/import.rs#L2054) | [✅](src/import.rs#L1747) |
| [N's Darmanitan](src/import.rs#L1845) | [✅](src/import.rs#L1845) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1842) | [✅](src/import.rs#L1842) | — |
| [N's Zekrom](src/import.rs#L1854) | [✅](src/import.rs#L1854) | — |
| [N's Zoroark ex](src/import.rs#L2027) | [✅](src/import.rs#L2027) | [✅](src/import.rs#L1777) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2050) | [✅](src/import.rs#L2050) | [✅](src/import.rs#L1755) |
| [Paldean Tauros](src/import.rs#L1838) | [✅](src/import.rs#L1838) | — |
| [Passimian](src/import.rs#L1851) | [✅](src/import.rs#L1851) | — |
| [Patrat](src/import.rs#L2049) | [✅](src/import.rs#L2049) | [✅](src/import.rs#L1714) |
| [Pecharunt](src/import.rs#L1991) | [✅](src/import.rs#L1991) | [✅](src/import.rs#L1781) |
| [Pecharunt ex](src/import.rs#L1988) | [✅](src/import.rs#L1988) | [✅](src/import.rs#L1818) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1715) |
| [Rabsca](src/import.rs#L1904) | [✅](src/import.rs#L1904) | [✅](src/import.rs#L1720) |
| [Raging Bolt ex](src/import.rs#L1869) | [✅](src/import.rs#L1869) | — |
| [Rellor](src/import.rs#L1836) | [✅](src/import.rs#L1836) | — |
| [Seaking](src/import.rs#L1997) | [✅](src/import.rs#L1997) | [✅](src/import.rs#L1785) |
| [Shaymin](src/import.rs#L2044) | [✅](src/import.rs#L2044) | [✅](src/import.rs#L1719) |
| [Slowking](src/import.rs#L1893) | [✅](src/import.rs#L1893) | — |
| [Slowpoke](src/import.rs#L1901) | [✅](src/import.rs#L1901) | ❌ |
| [Smoochum](src/import.rs#L1961) | [✅](src/import.rs#L1961) | — |
| [Stunfisk](src/import.rs#L1943) | [✅](src/import.rs#L1943) | — |
| [Tapu Bulu](src/import.rs#L1837) | [✅](src/import.rs#L1837) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1738) |
| [Teal Mask Ogerpon ex](src/import.rs#L2062) | [✅](src/import.rs#L2062) | [✅](src/import.rs#L1762) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1787) |
| [Torchic](src/import.rs#L1903) | [✅](src/import.rs#L1903) | — |
| [Toxel](src/import.rs#L1888) | [✅](src/import.rs#L1888) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1805) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1878) | [✅](src/import.rs#L1878) | — |
| [Yveltal](src/import.rs#L1877) | [✅](src/import.rs#L1877) | — |
| [Zeraora](src/import.rs#L1860) | [✅](src/import.rs#L1860) | — |

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

