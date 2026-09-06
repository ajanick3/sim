# Rare candy and the evolution chain

Type: task
Status: resolved

`Rare Candy` puts a Stage 2 from hand straight onto a Basic in play,
skipping the Stage 1. 40 slots.

`evolve_from` holds one link: a Stage 2 names the Stage 1 it comes from, never
the Basic beneath it. Answering "does this Stage 2 evolve from that Basic"
means walking two links, and the middle card need not be in the deck at all —
only in the pool.

- [x] The chain from a Basic to a Stage 2 can be asked about
- [x] A recorded decision on where the chain is walked, at load or at play
- [x] `Rare Candy` plays, and refuses a Stage 2 that does not belong

## Resolution

`Pokemon` carries a new field, `evolves_from_basic: Option<&'static str>`,
resolved once at import from a name-to-parent table built over every raw
Pokémon record in the artifact — admitted or refused, since the Stage 1
between a Basic and a Stage 2 need not itself be admitted for its name to
answer the question.
[ADR 0019](../../../docs/adr/0019-a-stage-2-carries-its-basic-by-name.md)
records why the walk happens at load rather than at play: the engine holds
no route back to the raw artifact once a game is in progress, and reads
`evolves_from_basic` exactly the way it already read `evolve_from`.

`Rare Candy` plays as `TrainerEffect::EvolveSkippingOneStage`, a new phase
(`Phase::EvolvingWithRareCandy`) that pairs a Stage 2 in hand with a Basic in
play the same way `MoveAttachedEnergy` pairs two Pokémon: `legal_actions`
guarantees a matching pair exists before the card may be played, so the
phase itself never declines.

Verified against a real chain: `Ampharos` resolves to `Mareep` through
`Flaaffy`, read from the raw name table rather than from the `CardDb` —
the table is built the same way whether or not the middle print happens to
be admitted, which this pool does not have a case to exercise directly, but
`me04-028` (a refused print of `Flaaffy`, for attack text) shows the table
draws from every print, not only the ones the engine can run.

Coverage went 392 → 394 (both prints), and the field went 1348 → 1388
playable slots of 3660 — 37.9%.
