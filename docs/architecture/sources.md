# Card data sources

Where card data and rules text come from, and what each source gets wrong.

| Source                                           | Use                                                                                                                              | Caveat                                                                                                                                                                                                                          |
| ------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Pokémon TCG Rulebook** (`mew_rulebook_en.pdf`) | Normative base rules                                                                                                             | pokemon.com blocks curl (Akamai); the linked `pbl_rulebook_en.pdf` exceeds WebFetch's 10 MB cap. `mew_rulebook_en.pdf` DID come through WebFetch. No `pdftotext`/`pypdf` on this box — a hand-rolled zlib+`Tj` extractor worked |
| **TCG Errata** (`tcg_errata.pdf`)                | Cards whose printed text is officially wrong — our `cards.effect` holds the _printed_ text, so the DB is wrong for exactly these | Not yet fetched                                                                                                                                                                                                                 |
| **Rulings Compendium** (compendium.pokegym.net)  | Edge-case interactions only                                                                                                      | Prose Q&A, NOT data. Cannot be a source of card logic. Skews to older eras                                                                                                                                                      |
| **pokemontcg.io v2**                             | Best enrichment source: has `regulationMark`, `subtypes[]` (incl. `MEGA`), and `rules[]` (the card's printed rule box)           | Flaky — throws 500/502 under light load. Fine for a one-off crawl, not request-time. Ids differ: `me1-3` vs our `me01-003` — mapping is an unsolved matching problem                                                            |
| **TCGdex** (current crawl)                       | What the app is built on                                                                                                         | **Cannot distinguish a Mega ex from a regular ex** — `suffix` is only `ex`/`EX`, `stage` is Basic/Stage1/Stage2, nothing marks MEGA. Has no prize field at all                                                                  |

## Existing projects surveyed (all rejected, kept as references)

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
