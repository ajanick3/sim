# Evolution

Type: task
Status: resolved

Add evolution. Rules 19 to 22 are written and unimplemented, and both
committed decks are built on Stage 1 and Stage 2 lines.

This one is Nick's. It needs no effect system, the rules are already stated,
and the engine has every part it depends on.

- [x] A Pokémon evolves from the card it names, and only from that card
- [x] It must have been in play since the beginning of the turn, and cannot
      evolve the turn it was played
- [x] It cannot evolve twice in one turn, and neither player evolves on the
      first turn of the game
- [x] It keeps its damage and its attachments, and loses every Special
      Condition
- [x] Evolution works on the Active and on a Benched Pokémon

## Answer

Resolved 2026-09-06 on branch `feat/evolution`, taken by the agent at the
operator's explicit direction after this ticket had stood as
`ready-for-human`.

`PokemonInPlay` now holds the whole stack of cards a Pokémon has been —
`cards: Vec<CardId>` rather than one `CardId` — because a knockout discards
every stage together (rule 38), and the current name, HP, and attacks read
from the top of the stack. A second flag, `evolved_this_turn`, is the
Pokémon's own once-per-turn limit; `played_on_turn` was already there,
recording when the Pokémon first came into play, and evolution reads it
unchanged.

`Action::Evolve { card, target }` matches a Pokémon in hand against its
`evolve_from` name against every Pokémon in play, gated on rules 18-20:
not the first turn of the game, the target in play since before this turn,
and not already evolved this turn. Evolving pushes the card onto the stack,
clears Special Conditions (rule 22), and leaves damage and attachments alone.

`data/cards.json` gains ~72 admitted cards: `import.rs` no longer refuses
every Stage 1 and 2 outright, only ones missing the `evolveFrom` name it
would need to be evolved into. Coverage moves from 274/3051 (9.0%) to
346/3051 (11.3%).

**One real bug found and fixed along the way.** `CardDef::is_basic_pokemon`
checked only "is this a Pokémon card," which was accidentally correct while
every admitted Pokémon was a Basic. Importing Stage 1 and 2 cards broke that
accident: without the fix, a Stage 1 or 2 card in hand could be placed
directly as Active or on the Bench, bypassing evolution entirely. It now
checks `evolve_from.is_none()`, at its one definition, so every caller
(`PlaceActive`, `PlaceOnBench`, `PlayBasic`, the mulligan check) was fixed at
once.

Written test first, 7 tests in `tests/evolution.rs`, plus updates to two
`tests/card_types.rs` assertions whose expected counts moved once evolution
changed what a real evolution card gets refused for.
