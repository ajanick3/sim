# A Stage 2 carries the Basic beneath it, resolved at import

**Status:** Accepted — 2026-09-06

`Rare Candy` evolves a Basic Pokémon in play straight into a Stage 2 from
hand, skipping the Stage 1 between them. `evolve_from` names one link —
a Stage 2's immediate parent — so answering "does this Stage 2 belong on
that Basic" means walking a second link the engine had never needed to
read: the Stage 1's own parent. Two things make that walk harder than it
looks. The middle card, the Stage 1, need not be admitted at all — a Stage 1
with an ability is refused (`Refusal::HasAnAbility`) and never enters the
`CardDb`, yet its name still answers the question. And the walk cannot run
at play time, because the engine only ever holds admitted `CardDef`s: it has
no route back to the artifact's raw JSON once a game is in progress.

The alternative was to walk the chain live, giving `GameState` a reference to
the raw pool so `legal_actions` could ask it. That was rejected: the engine
is supposed to be a pure function of its cards, its seed, and its actions
(ADR 0001), never of a side channel back to import-time data. Instead, the
walk happens once, in `import::load`, using every raw Pokémon record in the
artifact — admitted or refused — to build a name-to-parent table. Each
admitted Stage 2 resolves its Basic through that table and carries the
result as `Pokemon::evolves_from_basic: Option<&'static str>`, the same way
`evolve_from` already carries the Stage 1's name. `legal_actions` reads it
exactly as it reads `evolve_from` for an ordinary evolution: no side
channel, no lookup outside the `CardDef` in hand.

## Consequences

`Rare Candy`'s phase, `Phase::EvolvingWithRareCandy`, never offers a
declining action: `legal_actions` guarantees at least one matching
(Stage 2, Basic) pair exists before the card can be played at all — the same
pattern `TrainerEffect::MoveAttachedEnergy` established. A Stage 2 whose
chain does not resolve (the Stage 1 print is missing from the pool
entirely) carries `evolves_from_basic: None` and is simply never offered a
pairing; it is not refused at import, since ordinary evolution through its
own `evolve_from` still works.
