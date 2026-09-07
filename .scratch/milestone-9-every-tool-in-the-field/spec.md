# Spec: every Tool in the field

## Problem

Milestone 8 closed with every Item in the field built or refused.
Tools are next, per the standing order (Supporters, Items, Tools,
Stadiums). Nothing built attaches a Tool at all: `PlayTrainer` sends
every `TrainerKind::Tool` straight to discard, same as an Item.

9 names in the committed decks. `cargo run --bin blockers` names them
once this milestone's own attach mechanism exists to build against;
until then they all fall under "Trainer, not yet built."

## The attach mechanism, and what it touches

A Tool attaches to a Pokémon and stays there — it is not consumed the
way an Item is. This is genuinely new: `Action::AttachEnergy { card,
target }` is the closest precedent, not `PlayTrainer`. A `PlayTool
{ card, target }` action follows that shape.

Two rules nothing enforces yet:

- **One Tool per Pokémon.** `has_a_target` for a Tool is "some Pokémon
  in play carrying no Tool already"; the attach handler rejects a
  second one the same way `AttachEnergy` has no such limit to enforce
  (Energy stacks; Tools do not).
- **A Tool needs a target at play time**, unlike an Item. Both
  `legal_actions`' timing arm (`TrainerKind::Item | TrainerKind::Tool
  => true`) and `engine.rs`'s `PlayTrainer` (same combined arm, straight
  to discard) need splitting.

### Where a Tool attaches: `attached`, and the audit that buys it

The natural place is `PokemonInPlay.attached` — Energy already lives
there, and knockout discard (rule 38), `Tool Scrapper`, and view masking
all work through existing code paths for free. The cost: every existing
reader of `.attached` was written when it could only ever hold Energy,
and needs checking before a Tool can share the field.

`grep -n "\.attached" src/*.rs` and classify each site. Confirmed so
far:
- `Action::HealMegaEx` (`engine.rs`, `std::mem::take` then
  `hand.extend`, no filter) — **unsafe**. Wally's Compassion's text is
  "all *Energy* attached to it into your hand"; today harmless since
  `attached` only ever holds Energy, but a Tool sitting there would
  silently return to hand too, and no existing test has a Tool attached
  to notice. Filter to `is_energy()` before this milestone's first Tool
  lands, as its own ticket.
- `DiscardingOpponentEnergy` and `ActiveHasAtLeastEnergy` already
  filter `is_energy()` — safe.
- Attack Energy-cost payment — check before building.
- `Transformation Tome`'s identity swap carries `attached` wholesale,
  Tool and Energy alike — correct either way, no change needed.
- Knock-out discard (`engine.rs`, `knock_out`) already discards the
  whole `attached` list — correct for a Tool too (rule 38 discards
  everything attached, not only Energy).

## The static-effect mechanism

This is the fork the milestone actually turns on: most Tools modify a
number the engine already computes once and stores nowhere derived —
HP, prizes, retreat cost, damage. The engine has never had a printed
value and an in-play value differ before.

**Decision, not yet made: derive at read time, don't mutate on
attach.** Precedent already in the codebase for this shape:
`damage_dealt` reads `state.turn_bonus` at "step 32" rather than
mutating a stored damage value when the bonus was granted;
`pokemon_def` derives identity from `top_card()` rather than storing a
cached name. Mutate-on-attach/unmutate-on-discard is the alternative,
and it is fragile against the engine's purity and replay guarantees
(ADR 0002, ADR 0009) the same way a cached derived value always is.

The consequence, once decided: `hp` and `prizes` become two different
reads, where today each is one.

- A zone card's HP (`CardFilter::BasicPokemonWithHpAtMost`, Buddy-Buddy
  Poffin) reads the **printed** value — a card in a deck or hand has no
  Tool attached to modify it.
- An in-play Pokémon's HP (`knock_out_the_dead`'s `damage >= hp`) reads
  the **effective** value once a Tool can raise it.
- Same split for prizes: `CardFilter::PokemonEx`,
  `TurnBonusTarget::OpponentActiveEx`, `HealingMegaEx`'s `prizes == 3`
  are printed reads; `take_prizes` is the effective one.
  [ADR 0010](../../docs/adr/0010-a-prize-value-is-read-from-the-card-name.md)
  already decided prizes are read at knockout time "so a card that
  adjusts the count has somewhere to act," naming `Lillie's Pearl`
  explicitly — this is the precedent for deriving, not re-deriving it.

## Solution shape (to firm up ticket by ticket)

1. The attach mechanism: `PlayTool { card, target }`, one-Tool-per-
   Pokémon, split `TrainerKind::Item | TrainerKind::Tool` arms. The
   `HealMegaEx` filter fix rides in the same ticket, before any Tool can
   expose it.
2. A first Tool with no static effect (if one exists in the 9) to prove
   the attach mechanism alone.
3. The static-effect split (printed vs. effective HP/prizes), against
   whichever Tool needs it first.
4. The rest, ticket by ticket, the same discipline every milestone here
   has used: check the pool for an existing shape before adding one.

`Secret Box`'s Tool slot (Milestone 8) is dead in real games today —
no Tool is admitted, so it can never find one. The first Tool this
milestone admits turns that slot on; a regression test then, not now.
