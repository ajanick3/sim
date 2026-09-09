# ACE SPEC is a lookup by print ID, not a field on Trainer or Energy

**Status:** Accepted — 2026-09-09

Rule 3 of deck construction limits a deck to one ACE SPEC card total,
across every kind that carries the classification: Item, Tool,
Stadium, or Special Energy — never a Supporter. `Genesect`'s `ACE
Nullifier` also needs to know, at two points in `legal_actions`,
whether a card in hand is one. The two live shapes were a stored
field threaded through `Trainer` and `Energy` construction, or a
lookup function read on demand.

A stored field would touch every existing construction site of both
structs — 109 `CardDef::Trainer` and 74 `CardDef::Energy` literals
across the codebase, almost all of them test fixtures, to add one
field only two real call sites ever read. `Marker` earned its place
on `Pokemon` in [0081](0081-a-pokemon-carries-explicit-markers-not-an-inferred-suffix.md)
because it replaced a name-based inference that was already wrong for
one species; ACE SPEC carries no such problem to solve. `is_ace_spec`
in `src/import.rs` reads a `const ACE_SPEC_PRINT_IDS` table instead,
the same shape `TERA_PRINT_IDS`, `ANCIENT_PRINT_IDS`, and
`FUTURE_PRINT_IDS` already take, called at `decklist::check` for Rule
3 and by `Genesect`'s Ability at both offering sites for an ACE SPEC
play (`PlayTrainer`'s timing, `PlayTool`, and the Energy-attach
offering).

The table is scoped to the five ACE SPEC names the sample decks use
(`Prime Catcher`, `Unfair Stamp`, `Enriching Energy`, `Hero's Cape`,
`Secret Box`), cross-checked against pkmncards.com's own `is:ace-spec`
listing rather than guessed — a guess this same table nearly carried
`Prism Energy` and `Sacred Ash` on, both later found to be ordinary,
non-ACE-SPEC cards. It is open, not closed: ACE SPEC is still being
printed, and a name outside this five has simply never been checked
here.
