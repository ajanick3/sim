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
| Supporters | 52 | 78 |
| Items | 30 | 85 |
| Tools | 9 | 35 |
| Stadiums | 13 | 31 |
| Special Energy | 7 | 17 |

### Supporters (26/26 built)

| Card | Status |
| --- | --- |
| [AZ's Tranquility](src/import.rs#L923) | ✅ |
| [Black Belt's Training](src/import.rs#L931) | ✅ |
| [Boss's Orders](src/import.rs#L378) | ✅ |
| [Briar](src/import.rs#L544) | ✅ |
| [Brock's Scouting](src/import.rs#L961) | ✅ |
| [Ciphermaniac's Codebreaking](src/import.rs#L863) | ✅ |
| [Crispin](src/import.rs#L769) | ✅ |
| [Cyrano](src/import.rs#L618) | ✅ |
| [Dawn](src/import.rs#L735) | ✅ |
| [Eri](src/import.rs#L954) | ✅ |
| [Gladion's Final Battle](src/import.rs#L935) | ✅ |
| [Gwynn](src/import.rs#L632) | ✅ |
| [Hilda](src/import.rs#L689) | ✅ |
| [Janine's Secret Art](src/import.rs#L985) | ✅ |
| [Judge](src/import.rs#L567) | ✅ |
| [Kieran](src/import.rs#L939) | ✅ |
| [Lana's Aid](src/import.rs#L889) | ✅ |
| [Lillie's Determination](src/import.rs#L568) | ✅ |
| [Morty's Conviction](src/import.rs#L949) | ✅ |
| [N's Plan](src/import.rs#L907) | ✅ |
| [Rosa's Encouragement](src/import.rs#L909) | ✅ |
| [Rust Syndicate Grunt](src/import.rs#L903) | ✅ |
| [Surfer](src/import.rs#L927) | ✅ |
| [Team Rocket's Petrel](src/import.rs#L793) | ✅ |
| [Wally's Compassion](src/import.rs#L984) | ✅ |
| [Xerosic's Machinations](src/import.rs#L953) | ✅ |

### Items (28/28 built)

| Card | Status |
| --- | --- |
| [Buddy-Buddy Poffin](src/import.rs#L604) | ✅ |
| [Bug Catching Set](src/import.rs#L849) | ✅ |
| [Crushing Hammer](src/import.rs#L603) | ✅ |
| [Dusk Ball](src/import.rs#L1015) | ✅ |
| [Energy Recycler](src/import.rs#L1097) | ✅ |
| [Energy Retrieval](src/import.rs#L1000) | ✅ |
| [Energy Search](src/import.rs#L986) | ✅ |
| [Energy Switch](src/import.rs#L674) | ✅ |
| [Enhanced Hammer](src/import.rs#L548) | ✅ |
| [Glass Trumpet](src/import.rs#L549) | ✅ |
| [Hand Trimmer](src/import.rs#L1014) | ✅ |
| [Jumbo Ice Cream](src/import.rs#L885) | ✅ |
| [N's PP Up](src/import.rs#L807) | ✅ |
| [Night Stretcher](src/import.rs#L575) | ✅ |
| [Prime Catcher](src/import.rs#L1016) | ✅ |
| [Rare Candy](src/import.rs#L792) | ✅ |
| [Sacred Ash](src/import.rs#L646) | ✅ |
| [Secret Box](src/import.rs#L1046) | ✅ |
| [Special Red Card](src/import.rs#L765) | ✅ |
| [Strange Timepiece](src/import.rs#L1017) | ✅ |
| [Switch](src/import.rs#L884) | ✅ |
| [Team Rocket's Transceiver](src/import.rs#L1083) | ✅ |
| [Tera Orb](src/import.rs#L675) | ✅ |
| [Tool Scrapper](src/import.rs#L543) | ✅ |
| [Transformation Tome](src/import.rs#L1042) | ✅ |
| [Ultra Ball](src/import.rs#L660) | ✅ |
| [Unfair Stamp](src/import.rs#L877) | ✅ |
| [Wondrous Patch](src/import.rs#L821) | ✅ |

### Tools (9/9 built)

| Card | Status |
| --- | --- |
| [Air Balloon](src/import.rs#L1018) | ✅ |
| [Binding Mochi](src/import.rs#L1021) | ✅ |
| [Brave Bangle](src/import.rs#L1020) | ✅ |
| [Handheld Fan](src/import.rs#L1025) | ✅ |
| [Hero's Cape](src/import.rs#L1019) | ✅ |
| [Lillie's Pearl](src/import.rs#L1022) | ✅ |
| [Lucky Helmet](src/import.rs#L1024) | ✅ |
| [Powerglass](src/import.rs#L1026) | ✅ |
| [Punk Helmet](src/import.rs#L1023) | ✅ |

### Stadiums (13/13 built)

| Card | Status |
| --- | --- |
| [Academy at Night](src/import.rs#L1029) | ✅ |
| [Area Zero Underdepths](src/import.rs#L564) | ✅ |
| [Battle Cage](src/import.rs#L565) | ✅ |
| [Festival Grounds](src/import.rs#L1038) | ✅ |
| [Forest of Vitality](src/import.rs#L1037) | ✅ |
| [Gravity Mountain](src/import.rs#L1027) | ✅ |
| [Jamming Tower](src/import.rs#L1035) | ✅ |
| [Lumiose City](src/import.rs#L1034) | ✅ |
| [N's Castle](src/import.rs#L1028) | ✅ |
| [Nighttime Mine](src/import.rs#L563) | ✅ |
| [Risky Ruins](src/import.rs#L1036) | ✅ |
| [Team Rocket's Factory](src/import.rs#L1030) | ✅ |
| [Team Rocket's Watchtower](src/import.rs#L566) | ✅ |

### Special Energy (7/7 built)

| Card | Status |
| --- | --- |
| [Boomerang Energy](src/import.rs#L1476) | ✅ |
| [Enriching Energy](src/import.rs#L1459) | ✅ |
| [Growing Grass Energy](src/import.rs#L1458) | ✅ |
| [Mist Energy](src/import.rs#L1473) | ✅ |
| [Prism Energy](src/import.rs#L1479) | ✅ |
| [Spiky Energy](src/import.rs#L1470) | ✅ |
| [Telepathic Psychic Energy](src/import.rs#L1462) | ✅ |

### Pokémon (95/95 built)

| Card | Attacks | Ability |
| --- | --- | --- |
| [Abra](src/import.rs#L1665) | [✅](src/import.rs#L1665) | [✅](src/import.rs#L1547) |
| [Alakazam](src/import.rs#L1820) | [✅](src/import.rs#L1820) | [✅](src/import.rs#L1537) |
| [Annihilape](src/import.rs#L1694) | [✅](src/import.rs#L1694) | [✅](src/import.rs#L1503) |
| [Applin](src/import.rs#L1655) | [✅](src/import.rs#L1655) | — |
| [Bayleef](src/import.rs#L1697) | [✅](src/import.rs#L1697) | — |
| [Beldum](src/import.rs#L1677) | [✅](src/import.rs#L1677) | — |
| [Blaziken ex](src/import.rs#L1761) | [✅](src/import.rs#L1761) | [✅](src/import.rs#L1577) |
| [Bloodmoon Ursaluna ex](src/import.rs#L1725) | [✅](src/import.rs#L1725) | [✅](src/import.rs#L1511) |
| [Brute Bonnet](src/import.rs#L1636) | [✅](src/import.rs#L1636) | — |
| [Budew](src/import.rs#L1702) | [✅](src/import.rs#L1702) | — |
| [Buneary](src/import.rs#L1696) | [✅](src/import.rs#L1696) | — |
| [Carvanha](src/import.rs#L1614) | [✅](src/import.rs#L1614) | — |
| [Celebi](src/import.rs#L1695) | [✅](src/import.rs#L1695) | — |
| [Chi-Yu](src/import.rs#L1793) | [✅](src/import.rs#L1793) | — |
| [Chien-Pao](src/import.rs#L1762) | [✅](src/import.rs#L1762) | [✅](src/import.rs#L1580) |
| [Chikorita](src/import.rs#L1698) | [✅](src/import.rs#L1698) | — |
| [Cofagrigus](src/import.rs#L1743) | [✅](src/import.rs#L1743) | — |
| [Combusken](src/import.rs#L1712) | [✅](src/import.rs#L1712) | — |
| [Crustle](src/import.rs#L1832) | [✅](src/import.rs#L1832) | [✅](src/import.rs#L1492) |
| [Dedenne](src/import.rs#L1652) | [✅](src/import.rs#L1652) | — |
| [Dipplin](src/import.rs#L1777) | [✅](src/import.rs#L1777) | [✅](src/import.rs#L1565) |
| [Dragapult ex](src/import.rs#L1659) | [✅](src/import.rs#L1659) | — |
| [Drakloak](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1520) |
| [Dreepy](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Drilbur](src/import.rs#L1666) | [✅](src/import.rs#L1666) | ❌ |
| [Dudunsparce](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1544) |
| [Dudunsparce ex](src/import.rs#L1627) | [✅](src/import.rs#L1627) | — |
| [Dunsparce](src/import.rs#L1678) | [✅](src/import.rs#L1678) | — |
| [Dusclops](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1548) |
| [Dusknoir](src/import.rs#L1759) | [✅](src/import.rs#L1759) | [✅](src/import.rs#L1549) |
| [Duskull](src/import.rs#L1681) | [✅](src/import.rs#L1681) | — |
| [Dwebble](src/import.rs#L1671) | [✅](src/import.rs#L1671) | — |
| [Elgyem](src/import.rs#L1701) | [✅](src/import.rs#L1701) | — |
| [Enamorus](src/import.rs#L1721) | [✅](src/import.rs#L1721) | — |
| [Fan Rotom](src/import.rs#L1766) | [✅](src/import.rs#L1766) | [✅](src/import.rs#L1590) |
| [Fezandipiti ex](src/import.rs#L1840) | [✅](src/import.rs#L1840) | [✅](src/import.rs#L1538) |
| [Flutter Mane](src/import.rs#L1757) | [✅](src/import.rs#L1757) | [✅](src/import.rs#L1514) |
| [Genesect](src/import.rs#L1812) | [✅](src/import.rs#L1812) | [✅](src/import.rs#L1553) |
| [Genesect ex](src/import.rs#L1760) | [✅](src/import.rs#L1760) | [✅](src/import.rs#L1550) |
| [Goldeen](src/import.rs#L1775) | [✅](src/import.rs#L1775) | [✅](src/import.rs#L1563) |
| [Grookey](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Hoothoot](src/import.rs#L1668) | [✅](src/import.rs#L1668) | [✅](src/import.rs#L1504) |
| [Hydrapple ex](src/import.rs#L1726) | [✅](src/import.rs#L1726) | [✅](src/import.rs#L1505) |
| [Iron Crown ex](src/import.rs#L1686) | [✅](src/import.rs#L1686) | [✅](src/import.rs#L1500) |
| [Iron Leaves ex](src/import.rs#L1765) | [✅](src/import.rs#L1765) | [✅](src/import.rs#L1581) |
| [Kadabra](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1533) |
| [Koraidon ex](src/import.rs#L1687) | [✅](src/import.rs#L1687) | — |
| [Kyurem](src/import.rs#L1809) | [✅](src/import.rs#L1809) | [✅](src/import.rs#L1572) |
| [Latias ex](src/import.rs#L1826) | [✅](src/import.rs#L1826) | [✅](src/import.rs#L1491) |
| [Lillie's Clefairy ex](src/import.rs#L1850) | [✅](src/import.rs#L1850) | [✅](src/import.rs#L1495) |
| [Mega Absol ex](src/import.rs#L1733) | [✅](src/import.rs#L1733) | — |
| [Mega Excadrill ex](src/import.rs#L1729) | [✅](src/import.rs#L1729) | — |
| [Mega Kangaskhan ex](src/import.rs#L1836) | [✅](src/import.rs#L1836) | [✅](src/import.rs#L1488) |
| [Mega Lopunny ex](src/import.rs#L1634) | [✅](src/import.rs#L1634) | — |
| [Mega Sharpedo ex](src/import.rs#L1669) | [✅](src/import.rs#L1669) | — |
| [Mega Skarmory ex](src/import.rs#L1747) | [✅](src/import.rs#L1747) | — |
| [Mega Slowbro ex](src/import.rs#L1787) | [✅](src/import.rs#L1787) | — |
| [Meganium](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1559) |
| [Meowth ex](src/import.rs#L1839) | [✅](src/import.rs#L1839) | [✅](src/import.rs#L1532) |
| [Metagross](src/import.rs#L1643) | [✅](src/import.rs#L1643) | — |
| [Metang](src/import.rs#L1827) | [✅](src/import.rs#L1827) | [✅](src/import.rs#L1523) |
| [Moltres](src/import.rs#L1679) | [✅](src/import.rs#L1679) | — |
| [Munkidori](src/import.rs#L1833) | [✅](src/import.rs#L1833) | [✅](src/import.rs#L1526) |
| [N's Darmanitan](src/import.rs#L1624) | [✅](src/import.rs#L1624) | — |
| [N's Darumaka](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [N's Reshiram](src/import.rs#L1621) | [✅](src/import.rs#L1621) | — |
| [N's Zekrom](src/import.rs#L1633) | [✅](src/import.rs#L1633) | — |
| [N's Zoroark ex](src/import.rs#L1806) | [✅](src/import.rs#L1806) | [✅](src/import.rs#L1556) |
| [N's Zorua](src/import.rs#L234) | [✅](src/import.rs#L234) | — |
| [Noctowl](src/import.rs#L1829) | [✅](src/import.rs#L1829) | [✅](src/import.rs#L1534) |
| [Paldean Tauros](src/import.rs#L1617) | [✅](src/import.rs#L1617) | — |
| [Passimian](src/import.rs#L1630) | [✅](src/import.rs#L1630) | — |
| [Patrat](src/import.rs#L1828) | [✅](src/import.rs#L1828) | [✅](src/import.rs#L1493) |
| [Pecharunt](src/import.rs#L1770) | [✅](src/import.rs#L1770) | [✅](src/import.rs#L1560) |
| [Pecharunt ex](src/import.rs#L1767) | [✅](src/import.rs#L1767) | [✅](src/import.rs#L1597) |
| [Psyduck](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1494) |
| [Rabsca](src/import.rs#L1683) | [✅](src/import.rs#L1683) | [✅](src/import.rs#L1499) |
| [Raging Bolt ex](src/import.rs#L1648) | [✅](src/import.rs#L1648) | — |
| [Rellor](src/import.rs#L1615) | [✅](src/import.rs#L1615) | — |
| [Seaking](src/import.rs#L1776) | [✅](src/import.rs#L1776) | [✅](src/import.rs#L1564) |
| [Shaymin](src/import.rs#L1823) | [✅](src/import.rs#L1823) | [✅](src/import.rs#L1498) |
| [Slowking](src/import.rs#L1672) | [✅](src/import.rs#L1672) | — |
| [Slowpoke](src/import.rs#L1680) | [✅](src/import.rs#L1680) | ❌ |
| [Smoochum](src/import.rs#L1740) | [✅](src/import.rs#L1740) | — |
| [Stunfisk](src/import.rs#L1722) | [✅](src/import.rs#L1722) | — |
| [Tapu Bulu](src/import.rs#L1616) | [✅](src/import.rs#L1616) | — |
| [Tatsugiri](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1517) |
| [Teal Mask Ogerpon ex](src/import.rs#L1841) | [✅](src/import.rs#L1841) | [✅](src/import.rs#L1541) |
| [Thwackey](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1566) |
| [Torchic](src/import.rs#L1682) | [✅](src/import.rs#L1682) | — |
| [Toxel](src/import.rs#L1667) | [✅](src/import.rs#L1667) | — |
| [Toxtricity](src/import.rs#L234) | [✅](src/import.rs#L234) | [✅](src/import.rs#L1584) |
| [Wellspring Mask Ogerpon ex](src/import.rs#L1657) | [✅](src/import.rs#L1657) | — |
| [Yveltal](src/import.rs#L1656) | [✅](src/import.rs#L1656) | — |
| [Zeraora](src/import.rs#L1639) | [✅](src/import.rs#L1639) | — |

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

