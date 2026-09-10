# The wasm boundary reports what each legal action touches

**Status:** Accepted — 2026-09-10

`Game.legal_actions()` returns a `String[]` — the text `describe` prints
for each legal `Action`, index-aligned with what `apply` takes. The
browser UI can show that list as buttons and nothing more. It cannot
anchor a move to the board: click a Pokémon and see its moves, click a
hand card and see its targets. The `Action` enum carries the ids
(`CardId`, `PokemonId`) that would let it, but they stop at the boundary.

## Decision

A new method, `Game.action_meta()`, returns a JSON array index-aligned
with `legal_actions()`. Each entry is `{ kind, card, target }`:

- `kind` — the `Action` variant's name, taken from its `Debug` form the
  same way `phase_tag` already takes the phase's. Exhaustive for free; a
  new variant needs no change here.
- `card` — the `CardId` the action names, as a number, or `null`. The
  hand card played, the attachment moved, the card taken.
- `target` — the `PokemonId` the action names, as a number, or `null`.
  The Pokémon evolved, attached to, promoted, healed, switched in.

`legal_actions()` is unchanged and stays the label source. `action_meta`
adds the structure beside it, not instead of it.

The extraction is a match in `sim-wasm`, not the engine, and it is **not
exhaustive**: the common in-play moves — play, evolve, attach, retreat,
attack, promote, take, move Energy, heal, discard — get their ids; every
other variant, mostly phase-resolution prompts, reports `null`/`null`
and stays label-only. A card added later degrades to a plain button
until someone wires its variant, which is safe.

`WirePokemon` gains an `id` (the `PokemonId` as a number) so the UI can
match a `target` to the Pokémon it drew.

## Consequences

- The board-anchored UI reads `action_meta` to group moves under the
  board element they touch; the flat labelled list remains the fallback
  panel.
- `kind` lets the web `groupActions` helper switch from matching label
  prefixes to reading the variant name, later.
- No engine type gains a `serde` derive; the match lives at the boundary,
  as ADR 0096 requires.
- `actor` is left out. The only action whose actor is not its target is
  `Attack`, and its actor is always the current Active, which the UI
  already knows.
