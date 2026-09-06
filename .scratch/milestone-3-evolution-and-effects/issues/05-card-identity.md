# Card identity

Type: task
Status: resolved

412 Pokemon names are printed with differing behaviour, and a card
definition keeps only a name. Nothing can say which printed card an
implementation came from.

`CardRef` already carries the print id for every card read. This ticket puts
identity on the definition itself, so a game can name the card it is playing.

- [x] A card definition carries its print id
- [x] Two cards of the same name and different text stay separate
- [x] The text interface names the card a player is looking at

## Answer

Resolved 2026-09-06 on branch `feat/card-identity`.

`Pokemon` and `Energy` both carry `print_id: &'static str` now, and
`CardDef::print_id()` reads either. `import.rs` sets it from the artifact's
own `id` field (`me01-008`); a literal in `src/cards.rs` carries a synthetic
one (`synthetic-cinderpup`); the basic Energy the engine supplies gets a
stable one too (`basic-fire-energy`).

`CardDb` never merged same-named cards — `add` always pushes a new entry, so
two printings with the same name were already separate `CardDefId`s. What was
missing was a way to tell them apart afterward; `print_id` is that. Written
test first, `tests/card_identity.rs`.

The text interface's board display now names each Pokémon with its print id
alongside its name: `Cinderpup (synthetic-cinderpup) 70/70 HP`.

**A real hang found and fixed along the way**, unrelated to identity itself.
`tests/import.rs::the_engine_plays_a_game_with_imported_cards` built a deck
from `import.admitted[0]` on the assumption it would be a Basic. Ticket 01
made that assumption false: the first admitted card is now Ivysaur, a Stage 1
evolving from Bulbasaur, and a 60-card deck with no Basic anywhere never
reaches a legal opening board — `GameState::new`'s mulligan loop has no
iteration cap, since a legal deck was never expected to lack one, and it spun
forever. The test now picks the first admitted card that
`is_basic_pokemon()`. The engine itself was not at fault: deck legality is
the decklist checker's job, not something `GameState::new` re-validates.
