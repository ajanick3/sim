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
| Items | 47 | 85 |
| Tools | 20 | 35 |
| Stadiums | 17 | 31 |
| Special Energy | 10 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L1141) | ✅ |
| [Black Belt's Training](src/import.rs#L1149) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L762) | ✅ |
| [Brock's Scouting](src/import.rs#L1179) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L1081) | ✅ |
| [Crispin](src/import.rs#L987) | ✅ |
| [Cyrano](src/import.rs#L836) | ✅ |
| [Dawn](src/import.rs#L953) | ✅ |
| [Eri](src/import.rs#L1172) | ✅ |
| [Gladion's Final Battle](src/import.rs#L1153) | ✅ |
| [Gwynn](src/import.rs#L850) | ✅ |
| [Hilda](src/import.rs#L907) | ✅ |
| [Janine's Secret Art](src/import.rs#L1203) | ✅ |
| [Judge](src/import.rs#L785) | ✅ |
| [Kieran](src/import.rs#L1157) | ✅ |
| [Lana's Aid](src/import.rs#L1107) | ✅ |
| [Lillie's Determination](src/import.rs#L786) | ✅ |
| [Morty's Conviction](src/import.rs#L1167) | ✅ |
| [N's Plan](src/import.rs#L1125) | ✅ |
| [Rosa's Encouragement](src/import.rs#L1127) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L1121) | ✅ |
| [Surfer](src/import.rs#L1145) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L1011) | ✅ |
| [Wally's Compassion](src/import.rs#L1202) | ✅ |
| [Xerosic's Machinations](src/import.rs#L1171) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L822) | ✅ |
| [Bug Catching Set](src/import.rs#L1067) | ✅ |
| [Crushing Hammer](src/import.rs#L821) | ✅ |
| [Dusk Ball](src/import.rs#L1233) | ✅ |
| [Energy Recycler](src/import.rs#L1315) | ✅ |
| [Energy Retrieval](src/import.rs#L1218) | ✅ |
| [Energy Search](src/import.rs#L1204) | ✅ |
| [Energy Switch](src/import.rs#L892) | ✅ |
| [Enhanced Hammer](src/import.rs#L766) | ✅ |
| [Glass Trumpet](src/import.rs#L767) | ✅ |
| [Hand Trimmer](src/import.rs#L1232) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L1103) | ✅ |
| [N's PP Up](src/import.rs#L1025) | ✅ |
| [Night Stretcher](src/import.rs#L793) | ✅ |
| [Prime Catcher](src/import.rs#L1234) | ✅ |
| [Rare Candy](src/import.rs#L1010) | ✅ |
| [Sacred Ash](src/import.rs#L864) | ✅ |
| [Secret Box](src/import.rs#L1264) | ✅ |
| [Special Red Card](src/import.rs#L983) | ✅ |
| [Strange Timepiece](src/import.rs#L1235) | ✅ |
| [Switch](src/import.rs#L1102) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1301) | ✅ |
| [Tera Orb](src/import.rs#L893) | ✅ |
| [Tool Scrapper](src/import.rs#L761) | ✅ |
| [Transformation Tome](src/import.rs#L1260) | ✅ |
| [Ultra Ball](src/import.rs#L878) | ✅ |
| [Unfair Stamp](src/import.rs#L1095) | ✅ |
| [Wondrous Patch](src/import.rs#L1039) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1236) | ✅ |
| [Binding Mochi](src/import.rs#L1239) | ✅ |
| [Brave Bangle](src/import.rs#L1238) | ✅ |
| [Handheld Fan](src/import.rs#L1243) | ✅ |
| [Hero's Cape](src/import.rs#L1237) | ✅ |
| [Lillie's Pearl](src/import.rs#L1240) | ✅ |
| [Lucky Helmet](src/import.rs#L1242) | ✅ |
| [Powerglass](src/import.rs#L1244) | ✅ |
| [Punk Helmet](src/import.rs#L1241) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1247) | ✅ |
| [Area Zero Underdepths](src/import.rs#L782) | ✅ |
| [Battle Cage](src/import.rs#L783) | ✅ |
| [Festival Grounds](src/import.rs#L1256) | ✅ |
| [Forest of Vitality](src/import.rs#L1255) | ✅ |
| [Gravity Mountain](src/import.rs#L1245) | ✅ |
| [Jamming Tower](src/import.rs#L1253) | ✅ |
| [Lumiose City](src/import.rs#L1252) | ✅ |
| [N's Castle](src/import.rs#L1246) | ✅ |
| [Nighttime Mine](src/import.rs#L781) | ✅ |
| [Risky Ruins](src/import.rs#L1254) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1248) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L784) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1694) | ✅ |
| [Enriching Energy](src/import.rs#L1677) | ✅ |
| [Growing Grass Energy](src/import.rs#L1676) | ✅ |
| [Mist Energy](src/import.rs#L1691) | ✅ |
| [Prism Energy](src/import.rs#L1697) | ✅ |
| [Spiky Energy](src/import.rs#L1688) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1680) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1892) | [✅](src/import.rs#L1892) | [✅](src/import.rs#L1774) |
| [Alakazam](src/import.rs#L2047) | [✅](src/import.rs#L2047) | [✅](src/import.rs#L1764) |
| [Annihilape](src/import.rs#L1921) | [✅](src/import.rs#L1921) | [✅](src/import.rs#L1730) |
| [Applin](src/import.rs#L1882) | [✅](src/import.rs#L1882) | — |
| [Bayleef](src/import.rs#L1924) | [✅](src/import.rs#L1924) | — |
| [Beldum](src/import.rs#L1904) | [✅](src/import.rs#L1904) | — |
| [Blaziken ex](src/import.rs#L1988) | [✅](src/import.rs#L1988) | [✅](src/import.rs#L1804) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1952) | [✅](src/import.rs#L1952) | [✅](src/import.rs#L1738) |
| [Brute Bonnet](src/import.rs#L1863) | [✅](src/import.rs#L1863) | — |
| [Budew](src/import.rs#L1929) | [✅](src/import.rs#L1929) | — |
| [Buneary](src/import.rs#L1923) | [✅](src/import.rs#L1923) | — |
| [Carvanha](src/import.rs#L1841) | [✅](src/import.rs#L1841) | — |
| [Celebi](src/import.rs#L1922) | [✅](src/import.rs#L1922) | — |
| [Chi-Yu](src/import.rs#L2020) | [✅](src/import.rs#L2020) | — |
| [Chien-Pao](src/import.rs#L1989) | [✅](src/import.rs#L1989) | [✅](src/import.rs#L1807) |
| [Chikorita](src/import.rs#L1925) | [✅](src/import.rs#L1925) | — |
| [Cofagrigus](src/import.rs#L1970) | [✅](src/import.rs#L1970) | — |
| [Combusken](src/import.rs#L1939) | [✅](src/import.rs#L1939) | — |
| [Crustle](src/import.rs#L2059) | [✅](src/import.rs#L2059) | [✅](src/import.rs#L1719) |
| [Dedenne](src/import.rs#L1879) | [✅](src/import.rs#L1879) | — |
| [Dipplin](src/import.rs#L2004) | [✅](src/import.rs#L2004) | [✅](src/import.rs#L1792) |
| [Dragapult ex](src/import.rs#L1886) | [✅](src/import.rs#L1886) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1747) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1893) | [✅](src/import.rs#L1893) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1771) |
| [Dudunsparce ex](src/import.rs#L1854) | [✅](src/import.rs#L1854) | — |
| [Dunsparce](src/import.rs#L1905) | [✅](src/import.rs#L1905) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1775) |
| [Dusknoir](src/import.rs#L1986) | [✅](src/import.rs#L1986) | [✅](src/import.rs#L1776) |
| [Duskull](src/import.rs#L1908) | [✅](src/import.rs#L1908) | — |
| [Dwebble](src/import.rs#L1898) | [✅](src/import.rs#L1898) | — |
| [Elgyem](src/import.rs#L1928) | [✅](src/import.rs#L1928) | — |
| [Enamorus](src/import.rs#L1948) | [✅](src/import.rs#L1948) | — |
| [Fan Rotom](src/import.rs#L1993) | [✅](src/import.rs#L1993) | [✅](src/import.rs#L1817) |
| [Fezandipiti ex](src/import.rs#L2067) | [✅](src/import.rs#L2067) | [✅](src/import.rs#L1765) |
| [Flutter Mane](src/import.rs#L1984) | [✅](src/import.rs#L1984) | [✅](src/import.rs#L1741) |
| [Genesect](src/import.rs#L2039) | [✅](src/import.rs#L2039) | [✅](src/import.rs#L1780) |
| [Genesect ex](src/import.rs#L1987) | [✅](src/import.rs#L1987) | [✅](src/import.rs#L1777) |
| [Goldeen](src/import.rs#L2002) | [✅](src/import.rs#L2002) | [✅](src/import.rs#L1790) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1895) | [✅](src/import.rs#L1895) | [✅](src/import.rs#L1731) |
| [Hydrapple ex](src/import.rs#L1953) | [✅](src/import.rs#L1953) | [✅](src/import.rs#L1732) |
| [Iron Crown ex](src/import.rs#L1913) | [✅](src/import.rs#L1913) | [✅](src/import.rs#L1727) |
| [Iron Leaves ex](src/import.rs#L1992) | [✅](src/import.rs#L1992) | [✅](src/import.rs#L1808) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1760) |
| [Koraidon ex](src/import.rs#L1914) | [✅](src/import.rs#L1914) | — |
| [Kyurem](src/import.rs#L2036) | [✅](src/import.rs#L2036) | [✅](src/import.rs#L1799) |
| [Latias ex](src/import.rs#L2053) | [✅](src/import.rs#L2053) | [✅](src/import.rs#L1718) |
| [Lillie's Clefairy ex](src/import.rs#L2077) | [✅](src/import.rs#L2077) | [✅](src/import.rs#L1722) |
| [Mega Absol ex](src/import.rs#L1960) | [✅](src/import.rs#L1960) | — |
| [Mega Excadrill ex](src/import.rs#L1956) | [✅](src/import.rs#L1956) | — |
| [Mega Kangaskhan ex](src/import.rs#L2063) | [✅](src/import.rs#L2063) | [✅](src/import.rs#L1715) |
| [Mega Lopunny ex](src/import.rs#L1861) | [✅](src/import.rs#L1861) | — |
| [Mega Sharpedo ex](src/import.rs#L1896) | [✅](src/import.rs#L1896) | — |
| [Mega Skarmory ex](src/import.rs#L1974) | [✅](src/import.rs#L1974) | — |
| [Mega Slowbro ex](src/import.rs#L2014) | [✅](src/import.rs#L2014) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1786) |
| [Meowth ex](src/import.rs#L2066) | [✅](src/import.rs#L2066) | [✅](src/import.rs#L1759) |
| [Metagross](src/import.rs#L1870) | [✅](src/import.rs#L1870) | — |
| [Metang](src/import.rs#L2054) | [✅](src/import.rs#L2054) | [✅](src/import.rs#L1750) |
| [Moltres](src/import.rs#L1906) | [✅](src/import.rs#L1906) | — |
| [Munkidori](src/import.rs#L2060) | [✅](src/import.rs#L2060) | [✅](src/import.rs#L1753) |
| [N's Darmanitan](src/import.rs#L1851) | [✅](src/import.rs#L1851) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1848) | [✅](src/import.rs#L1848) | — |
| [N's Zekrom](src/import.rs#L1860) | [✅](src/import.rs#L1860) | — |
| [N's Zoroark ex](src/import.rs#L2033) | [✅](src/import.rs#L2033) | [✅](src/import.rs#L1783) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L2056) | [✅](src/import.rs#L2056) | [✅](src/import.rs#L1761) |
| [Paldean Tauros](src/import.rs#L1844) | [✅](src/import.rs#L1844) | — |
| [Passimian](src/import.rs#L1857) | [✅](src/import.rs#L1857) | — |
| [Patrat](src/import.rs#L2055) | [✅](src/import.rs#L2055) | [✅](src/import.rs#L1720) |
| [Pecharunt](src/import.rs#L1997) | [✅](src/import.rs#L1997) | [✅](src/import.rs#L1787) |
| [Pecharunt ex](src/import.rs#L1994) | [✅](src/import.rs#L1994) | [✅](src/import.rs#L1824) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1721) |
| [Rabsca](src/import.rs#L1910) | [✅](src/import.rs#L1910) | [✅](src/import.rs#L1726) |
| [Raging Bolt ex](src/import.rs#L1875) | [✅](src/import.rs#L1875) | — |
| [Rellor](src/import.rs#L1842) | [✅](src/import.rs#L1842) | — |
| [Seaking](src/import.rs#L2003) | [✅](src/import.rs#L2003) | [✅](src/import.rs#L1791) |
| [Shaymin](src/import.rs#L2050) | [✅](src/import.rs#L2050) | [✅](src/import.rs#L1725) |
| [Slowking](src/import.rs#L1899) | [✅](src/import.rs#L1899) | — |
| [Slowpoke](src/import.rs#L1907) | [✅](src/import.rs#L1907) | ❌ |
| [Smoochum](src/import.rs#L1967) | [✅](src/import.rs#L1967) | — |
| [Stunfisk](src/import.rs#L1949) | [✅](src/import.rs#L1949) | — |
| [Tapu Bulu](src/import.rs#L1843) | [✅](src/import.rs#L1843) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1744) |
| [Teal Mask Ogerpon ex](src/import.rs#L2068) | [✅](src/import.rs#L2068) | [✅](src/import.rs#L1768) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1793) |
| [Torchic](src/import.rs#L1909) | [✅](src/import.rs#L1909) | — |
| [Toxel](src/import.rs#L1894) | [✅](src/import.rs#L1894) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1811) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1884) | [✅](src/import.rs#L1884) | — |
| [Yveltal](src/import.rs#L1883) | [✅](src/import.rs#L1883) | — |
| [Zeraora](src/import.rs#L1866) | [✅](src/import.rs#L1866) | — |

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

