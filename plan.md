# Pokémon TCG Simulator — design record

Status: **Milestone 1 done.** Written 2026-09-03, updated 2026-09-05.
Resume by reading this file top to bottom; everything needed is here.

## 1. What this is

A Pokémon TCG rules engine written in Rust, in this repo
(`/home/nick/nick/sim`). The `pkmn` repo is a sibling.

**The primary goal is learning Rust.** The simulator is the vehicle. Process
matters more than outcome. Every scope decision below follows from that and
should be re-derived, not overridden, if the goal changes.

## 2. Settled decisions

| Topic        | Decision                                                                                                |
| ------------ | ------------------------------------------------------------------------------------------------------- |
| Language     | Rust                                                                                                    |
| Repo         | `sim`, separate from `pkmn`. Remote: `github.com/ajanick3/sim`                                           |
| Reuse        | Write our own engine. No code taken from any existing project                                           |
| Scope        | **Deep engine, near-zero cards.** NOT the Standard pool, NOT 3000 cards                                 |
| Milestone 1  | Two synthetic Basic Pokémon that attack until someone wins                                              |
| Interface    | Human-playable text game first; bots afterwards                                                         |
| Working mode | Claude writes with narrated reasoning → Nick takes over once the turn loop compiles and a game finishes |
| State model  | Arena / index-based (`Vec` + typed indices e.g. `PokemonId(u32)`)                                       |
| Card data    | Engine is **pure**: no Turso, no async, no I/O                                                          |
| Legality     | Regulation mark ONLY. Standard = H, I, J (as of 2026-09-03)                                             |

### Why scope is inverted from the obvious plan

The first instinct was "implement Standard" (~3023 cards). That is backwards for
a learning project: the ~3000 card impls are repetitive — you learn Rust in the
first thirty and grind the rest. The **engine core** is what teaches Rust (enums,
exhaustive `match`, ownership over a mutable game graph, error types, traits).
So: deep engine, tiny synthetic card set.

### Why arena/index state

Recorded in [ADR 0001](docs/adr/0001-arena-and-index-state.md).

### Why the engine must not touch Turso

Recorded in [ADR 0002](docs/adr/0002-pure-engine-with-a-json-seam.md). Note that
Milestone 1 needs **no card data at all** — the cards are literals in
`src/cards.rs`. Build the bridge later.

## 3. Still open

- **Card data bridge.** The export script in `pkmn` and the JSON format are not
  written. Nothing needs them until the engine outgrows literal cards.
- The naming question is settled: **Decklist** and **Library** join the domain
  glossary, and `Deck` keeps its tournament meaning.

## 4. Deferred, with findings already banked

Do not re-research these.

- **Prize values.** `Pokémon ex rule` = **2 prizes**; `Mega Evolution ex Rule` =
  **3 prizes**. No V/VMAX/VSTAR/GX in the current pool — rotated out.
- **Prize count is computed at KO time, not a card property.** Cards adjust it:
  `Legacy Energy` (−1, once per game), `Lillie's Pearl` (−1), `Briar` (+1,
  conditional), `Anthea & Concordia` (+3), `Redeemable Ticket` (rewrites the
  prize pile). Model it as a mutable field on a knockout effect that starts at 1
  and is adjusted — NOT as a static column.
- **Subtype tags eventually needed:** `ex` (~497), `MEGA` (~103), `Tera` (~94),
  `Pokémon Tool` (~52), `Ancient` (~43), `Future` (~40), `ACE SPEC` (~33),
  `Special` (~22).
- **Masked per-player observation views** — needed before bots, so an AI can't
  silently cheat by seeing hidden zones. Retrofitting is painful.
- **Card identity:** key implementations by **name + behaviour version** with a
  print-id → implementation lookup, not by print id. Same class of problem as
  the existing "Canonical card" glossary entry.
- **Randomness:** seeded PRNG stored in state, plus an injectable scripted
  sequence for tests. Gives reproducible replays. Built in `src/rng.rs`.

## 5. Data sources

| Source                                           | Use                                                                                                                              | Caveat                                                                                                                                                                                                                          |
| ------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Pokémon TCG Rulebook** (`mew_rulebook_en.pdf`) | Normative base rules                                                                                                             | pokemon.com blocks curl (Akamai); the linked `pbl_rulebook_en.pdf` exceeds WebFetch's 10 MB cap. `mew_rulebook_en.pdf` DID come through WebFetch. No `pdftotext`/`pypdf` on this box — a hand-rolled zlib+`Tj` extractor worked |
| **TCG Errata** (`tcg_errata.pdf`)                | Cards whose printed text is officially wrong — our `cards.effect` holds the _printed_ text, so the DB is wrong for exactly these | Not yet fetched                                                                                                                                                                                                                 |
| **Rulings Compendium** (compendium.pokegym.net)  | Edge-case interactions only                                                                                                      | Prose Q&A, NOT data. Cannot be a source of card logic. Skews to older eras                                                                                                                                                      |
| **pokemontcg.io v2**                             | Best enrichment source: has `regulationMark`, `subtypes[]` (incl. `MEGA`), and `rules[]` (the card's printed rule box)           | Flaky — throws 500/502 under light load. Fine for a one-off crawl, not request-time. Ids differ: `me1-3` vs our `me01-003` — mapping is an unsolved matching problem                                                            |
| **TCGdex** (current crawl)                       | What the app is built on                                                                                                         | **Cannot distinguish a Mega ex from a regular ex** — `suffix` is only `ex`/`EX`, `stage` is Basic/Stage1/Stage2, nothing marks MEGA. Has no prize field at all                                                                  |

### Existing projects surveyed (all rejected, kept as references)

- **[ryuu-play](https://github.com/keeshii/ryuu-play)** — TS, MIT, ~880 cards,
  actively pushed. **Best design reference.** Has `packages/simple-bot`, a
  store-based state machine, 27 prompt types, and
  `KnockOutEffect { prize_count: 1 }` adjusted by card tags, plus
  `state.rules.noPrizeForFossil` showing era-variant rules parameterised.
  Rejected because: zero H/I/J cards; `CardTag` has only `EX, GX, LV_X, SP,
ACE_SPEC, FOSSIL`; it's an app (server + Angular + cordova), not a library;
  and its **generator-based** prompt model is the design most in tension with
  fast headless self-play.
- **[tcgone-engine-contrib](https://github.com/axpendix/tcgone-engine-contrib)** —
  Groovy, Apache-2.0. Card-effect DSL that survived a decade — good vocabulary
  reference for effect primitives. Stops at gen8; **zero Scarlet & Violet**.
- **[deckgym-core](https://github.com/bcollazo/deckgym-core)** — Rust, well-built
  for bot simulation. **Disqualified twice:** it's TCG _Pocket_ (a different
  game), and it's **AGPL-3.0** (viral over network use — would oblige releasing
  pokedex's source).
- **[PTCG-Bench](https://github.com/zjunet/PTCG-Bench)** — Python, MIT, ~100
  cards, LLM-agent benchmark. Small, research-grade.

**Conclusion of the survey: no executable Pokémon TCG card logic exists for the
current era, in any language, under any license.** That's the actual state of
the ecosystem, confirmed across five independent projects.

## 6. Base rules (extracted from the official Rulebook)

Verified against the primary PDF. Nick reviewed and corrected these.

### Deck construction

1. Exactly 60 cards.
2. Max 4 copies of any card with the same name — except Basic Energy (unlimited).
3. Max 1 ACE SPEC card **total** in the deck (not 1 per name).
4. _(Not in rulebook)_ Legality by regulation mark + the Banned Card List.

### Setup

5. Coin flip; **winner chooses** who goes first.
6. Each player shuffles and draws 7.
7. Mulligan: no Basic in hand → reveal hand, shuffle back, draw 7. Repeat.
8. Per mulligan the opponent took, you **may** draw 1 extra card.
9. Place 1 Basic face down as Active; up to 5 more face down on the Bench.
10. Top 6 cards aside face down as Prizes.
11. Both players flip Pokémon face up; game begins.

### Turn structure — 3 parts, in order

12. **Draw a card.** Deck empty and cannot draw → **you lose**.
13. **Do any of these, in any order:**
    - Put Basic Pokémon from hand onto Bench — any number (Bench cap 5)
    - Evolve — any number
    - Attach an Energy from hand — **once per turn**
    - Play Trainers — Items any number, Tools any number, **1 Supporter**, **1 Stadium**
    - Retreat — **once per turn**
    - Use Abilities — any number, from Active _and_ Bench
14. **Attack, then the turn ends.** Cannot return to step 13.

### First-turn restrictions (player going first)

15. They **do** draw a card.
16. They **cannot play a Supporter** on their first turn.
17. They **skip the attack step** on their first turn.
18. **Neither player** can evolve on their first turn (unless a card says so).

### Evolution

19. Played on top of the Pokémon it evolves from, which must have been in play
    **since the beginning of your turn**.
20. Cannot evolve a Pokémon the turn it was played; cannot evolve the same
    Pokémon twice in one turn.
21. Works on Active **or** Benched Pokémon.
22. Keeps attached cards and damage counters; **clears all Special Conditions**
    and other effects.

### Retreat

23. Once per turn. Discard Energy equal to the Retreat Cost (free if none).
24. Requires at least one Benched Pokémon.
25. **Asleep and Paralyzed prevent retreating.** Confused does not.
26. Damage counters and attachments travel with the Pokémon.
27. Moving to the Bench **removes all Special Conditions**.
28. You **may still attack** after retreating, with the new Active.

### Attacking — damage order (the rulebook's explicit sequence)

29. Check the **attacking** Pokémon has the required Energy. (Normally the
    Active — but see Alakazam ex below.)
30. Do what the attack requires (coin flips). Confused's flip happens **before**.
31. Start from base damage, applying the attack's own text.
32. Apply effects on **your** Pokémon (before Weakness/Resistance).
    **Stop if damage is 0.**
33. Apply **Weakness** (increase), then **Resistance** (decrease).
34. Apply effects on the **defending** Pokémon (after Weakness/Resistance).
35. 1 damage counter per 10 final damage. **0 or less → no counters.**
36. **Weakness/Resistance never apply to Benched Pokémon.**
37. Effects that say "put damage counters" bypass all of the above.

### Knockout & prizes

38. Damage >= HP → Knocked Out; the Pokémon and all attached cards go to its
    owner's discard.
39. The **opponent of the KO'd player takes 1 Prize** (or as many as specified).
40. The player whose Active was KO'd chooses a new Active from their Bench.

### Win conditions

41. Take all your Prize cards.
42. Opponent has no Pokémon in play when they must choose a new Active.
43. Opponent cannot draw at the start of their turn.
44. If 41 and 42 would trigger at once, **you still win**.

### Pokémon Checkup (between turns)

45. Happens after a turn ends, before the next begins.
46. Special Conditions first, then other between-turn effects.
47. You choose the order of your own effects within it.
48. Anything at 0 HP after checkup is KO'd — new Active chosen, Prize taken —
    **then** the next turn starts.

### Special Conditions

49. Only the **Active** Pokémon can have them.
50. **Asleep** — cannot attack or retreat. Checkup: flip; heads recovers.
51. **Paralyzed** — cannot attack or retreat. Recovers at the checkup **after
    its owner's next turn**.
52. **Confused** — flip before attacking; tails = attack doesn't happen and
    **3 damage counters on your own Pokémon**.
53. **Burned** — checkup: **2 damage counters**, then flip; heads removes it.
54. **Poisoned** — checkup: **1 damage counter**.
55. **Asleep / Confused / Paralyzed are mutually exclusive** — the most recent
    replaces the others (they all rotate the card).
56. **Burned and Poisoned are independent** — can coexist with each other and
    with a rotation condition.
57. A second Burn or Poison **replaces** the existing one rather than stacking.

### Stadiums & Tools

58. A Stadium stays in play; only one at a time; a new one discards the old.
59. **Cannot** play a Stadium with the same name as one already in play.
60. One Stadium played per turn.
61. **1 Pokémon Tool per Pokémon by default**, but card abilities raise the cap:
    - _Self-scoped:_ `Garbodor VMAX` "Rubbish Collecting", `Genesect GX`
      "Double Drive" — this Pokémon may have up to 2.
    - _Team-scoped:_ `Rotom ex` (me02-029) "Multi Adapter" — each of your
      Pokémon with "Rotom" in its name may have up to 2.
    - Every one carries: _"If this Ability goes away, discard Pokémon Tools
      until only 1 remains."_ So the limit is a **continuous invariant
      re-checked when an ability is lost**, not an attach-time check.

## 7. Edge cases that break naive designs

Design the engine so these are expressible from day one.

- **`Alakazam ex` (`sv03.5-065`, reg G) — "Dimensional Hand"**:
  _"This attack can be used even if this Pokémon is on the Bench."_
  Breaks the assumption that the attacker is the Active Pokémon (rule 29).
  Exactly 5 prints in the SV pool — rare enough to design around by accident
  and then be wrong. Open questions for the Compendium: does a Benched attacker
  apply Weakness/Resistance (rule 36 says W/R never apply to Bench)? Can it
  attack while Asleep/Paralyzed, given those are Active-only?
- **Tool limits are ability-derived and dynamic** — see rule 61.
- **Prize count is computed at KO time** — see §4.
- **Mega Evolution is NOT a new stage.** In XY, `M Alakazam EX` had
  `stage = MEGA`. In the ME era, `Mega Venusaur ex` is `Stage2` evolving from
  `Ivysaur`; `Mega Zeraora ex` and `Mega Darkrai ex` are **Basic** with 270-280
  HP. There is no `MEGA` stage in reg J. Good news for the engine: "Mega" is
  naming, not mechanics — the rulebook's "all the normal rules for Evolution
  apply" is literal.

## 8. Known data problems in this repo

- `sets.legal_standard` is `1` for **zero rows**; `cards.legal_standard` is
  populated but stale. **Do not trust either** — use regulation mark.
  (See the `legality-by-regulation-mark` memory.)
- `cards` has **no prize-value column** and no way to identify Mega ex.
- The ability-coverage worry was **investigated and dismissed**: ability rate is
  a consistent ~22% across marks H (247/1086), I (254/1101), J (74/370).
  `abilities_json` being null usually means the card genuinely has no ability.
  There is nothing to fix here.

## 9. Current era reference (2026-09-03)

Standard = regulation marks **H, I, J** — 3023 legal cards.
Series: **Mega Evolution**, 8 sets, 2025-09-25 → 2026-07-17:
`mee` (Mega Evolution Energy), `mep` (promos), `me01` MEG, `me02` PFL,
`me02.5` ASC, `me03` POR, `me04` CRI, `me05` PBL (Pitch Black).

## 10. Where the code stands

Milestone 1 is built and its tests pass: setup with mulligans, the turn loop,
the damage order, knockouts, Prizes, and the three win conditions, driven either
by `cargo run` as a text game or by a bot through the same action list.

Two synthetic Basics and one Energy live in `src/cards.rs`. The interface is
[ADR 0003](docs/adr/0003-legal-actions-is-the-engine-interface.md).

Deliberate Milestone 1 shortcuts, each waiting for its own milestone:

- Setup places the first Basic as Active and the rest on the Bench instead of
  asking, and skips the coin flip for who goes first.
- An attack cost counts Energy; it does not match types.
- No Special Conditions, so Pokémon Checkup has nothing to do.
- No evolution, Trainers, Abilities, or Stadiums.

## 11. Next actions on resume

1. Nick writes the next card unaided — a third Basic, added to `src/cards.rs`.
2. Give Setup its own phase, so placement and the opening coin flip become
   ordinary legal actions.
3. Match Energy types in an attack cost.
4. Special Conditions and the Pokémon Checkup between turns.
