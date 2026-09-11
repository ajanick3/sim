# Senior review of the engine

A whole-codebase review, written to teach. It reads the engine as if a
mid-level developer wrote it and a senior engineer now walks through it with
them. It runs the two axes the `code-review` skill defines:

- **Standards** — does the code follow this repo's documented rules and stay
  clear of common code smells?
- **Spec** — does the code still honour what the architecture documents and the
  ADRs claim?

The two axes ran as separate reviewers so neither coloured the other. Their
findings stay under separate headings below and are not merged or re-ranked
across axes. Each axis ends with its own "fix first" list. There is no single
winner across the two.

Deviation from the skill: the skill caps each axis at 400 words, which suits a
small diff. This review covers about 14,600 lines of Rust and the user asked
for teaching, so each axis was allowed roughly 1,500 words with quoted code.

## What is already strong

State this first so the rest is trustworthy.

- **The arena-index pattern in `src/ids.rs` is textbook.** `CardId(u32)`,
  `PokemonId(u32)`, `CardDefId(u32)` with `.index()` accessors, and a module
  comment that says why. This is the right answer to "do not pass raw
  integers around".
- **`legal_actions` is a real single interface.** The engine calls it in
  exactly one place — the gate at `engine.rs:15`. `apply` is a thin executor
  behind that gate. ADR 0003 holds.
- **The engine is pure.** No `println!`, no `fs`, no `Instant`, no socket in
  `engine.rs`, `state.rs`, `action.rs`, `view.rs`, or `card.rs`. ADR 0002 /
  0007 holds in the code.
- **The log and replay rules hold.** `state.history.push(action)` sits after
  every early `return Err` and after the mid-arm `return Ok(())`, so a refused
  or partial action records nothing. `undo` is `replay` of the log minus its
  last entry. ADR 0013 and ADR 0014 hold.
- **Static effects are read, not dispatched.** `resolve_trainer` marks every
  static or Tool effect `unreachable!` at play time; the readers derive from
  what is attached now. ADR 0042 holds.
- **`rng.rs`, `decklist.rs`, `view.rs` masking design** — each is sound and
  each explains its shape in a comment. No finding.
- **`card.rs` at 1,796 lines is not a size problem.** It is flat `enum` data
  under the documented one-variant-per-card discipline (ADR 0004). Do not
  refactor it for length.

## The one lesson that matters most

Both axes land on the same structural fact from opposite sides.

`legal_actions` (`action.rs:476-1854`, about 1,380 lines) and `apply`
(`engine.rs:14-2662`, about 2,650 lines) are two giant `match` blocks on the
same `Phase` and `Action` types. They do **not** duplicate the legality logic
— the Spec axis confirmed `apply` trusts the gate and re-derives nothing of
substance. What they duplicate is **shape**: the same four-line "scan a zone,
test a filter, push a `TakeX`, push a `FinishX`" appears about 40 times in
`legal_actions`, and `player_to_act` has about 87 near-identical arms that all
return `Some(player)`.

The teaching point: when a `match` has dozens of arms that all have the same
body shape, the thing that varies belongs in **data or a helper**, not in a
new arm each time. A `push_zone_choices(actions, zone, filter, make_action,
finish_action)` helper turns 400 lines into 40. Hoisting a single `actor:
PlayerId` beside `phase`, or giving `Phase` an `actor()` method, collapses
`player_to_act`.

This is a planned refactor, not a first fix — it touches about 40 arms and the
matching `apply` dispatch. Naming it as "large, valuable, do it deliberately"
is itself the senior judgement.

---

## Standards

Calibration: most findings below are Fowler smell-baseline heuristics, i.e.
judgement calls. Only three cite the `AGENTS.md` register directly; they are
ranked first within their sections. Anything `rustfmt` or `clippy` already
catches is skipped — bare `cargo clippy` on the engine is clean but for two
trivial `collapsible_match` warnings.

### action.rs

**1. `legal_actions` repeats one zone-scan shape about 40 times (Duplicated
Code — judgement call, highest-value lesson).**

```rust
// action.rs:738
Phase::SearchingDeckForBasics { player: whose, .. } => {
    for card in &state.player(whose).deck {
        if state.matches_filter(*card, CardFilter::PokemonOfStage(Stage::Basic)) {
            actions.push(Action::TakeBasicPokemonForCallForFamily { card: *card });
        }
    }
    actions.push(Action::FinishCallForFamily);
    return actions;
}
// action.rs:747 — the next arm, same shape
Phase::SearchingDeckForBasicsOfType { player: whose, kind, .. } => {
    for card in &state.player(whose).deck {
        if state.matches_filter(*card, CardFilter::BasicPokemonOfType(kind)) {
            actions.push(Action::TakeBasicPokemonOfTypeForEnergyAttach { card: *card });
        }
    }
    actions.push(Action::FinishSearchingBasicsOfType);
    return actions;
}
```

Recurs at 756, 764, 822, 831, 979, and more, plus a four-times copy of an
attached-Energy scan at 1263-1298. Extract a helper parameterised by the
action constructor and the finish action.

**2. `player_to_act` — about 87 near-identical arms (Shotgun Surgery —
judgement call).** Almost every arm is `{ player, .. } => Some(player)`.
Adding one phase forces edits in `Phase`, `player_to_act`, `legal_actions`,
`describe`, and `apply`. The "whose choice is this" fact should live once,
beside `phase`.

**3. `describe` is Divergent Change (single responsibility — judgement
call).** `describe` (`action.rs:1894-2232`, 340 lines) is presentation text
living in the rules module. It changes for UI-wording reasons; the rest of the
file changes for rules reasons. Move it to its own module.

**4. Two phases re-derive stored data and guard with `unreachable!`
(judgement call).**

```rust
// action.rs:958
let AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(kind) =
    state.pokemon_def(pokemon).ability.expect("named only when carried").effect
else {
    unreachable!("this phase only ever opens for this effect");
};
```

Same at 1001. Both dig `kind` back out of the ability definition and add a
panic for the impossible mismatch. The repo already solves this:
`Phase::ChoosingOwnEnergyToDiscardForBonusDamage` stores `kind: Type` inline.
Store `kind` on these two phases when they open and both panics disappear.

**5. `unreachable!("filtered out above")` decoupled from its filter
(`action.rs:1450`, judgement call, minor).** The invariant lives 16 lines away
in a `.filter()` call. An edit to either half breaks the other silently.

### state.rs

**6. `begin_turn` — three copies of an "arm then clear" state machine
(Duplicated Code — judgement call, high bug density).** `state.rs:1574`, 1582,
1593. Three fields, three hand-copies of a delayed-arm lifecycle the comments
admit is easy to get wrong. Name the concept: a `DelayedRestriction<T>` with
`tick(is_relevant_turn: bool)`.

**7. Eight-plus copies of "scan both sides for an undisabled passive ability"
(Repeated Switches — judgement call).** `state.rs:1216`, 1202, 1116, 1172,
1283, 1302, 1507, and `effective_weakness` at 1141. A
`fn any_passive(&self, side, want: fn(AbilityEffect) -> bool) -> bool` covers
the boolean cases; `effective_weakness` needs a `find_map` sibling.

**8. `effective_hp`'s exhaustive `EnergyEffect` match versus `strategy.rs`'s
catch-all — an undocumented convention conflict (`AGENTS.md`: "Raise a
conflict... never settle it silently").** `state.rs:1035` spells out every
variant so a new one fails to compile; `strategy.rs:178` uses `_ =>` on
purpose. Both carry a comment defending themselves. The repo has not decided
whether an effect-enum match is a compile-time tripwire or a tolerant
classifier. Record the rule in a short note or ADR.

### card.rs

**9. Doc comments restate the same "narrowed shape" fact about 40 times
(`AGENTS.md`: "Give one fact one home").** `card.rs:132`, 137, 150, 155, 160,
168, 1031, 1044, 1059, and more. One module-level note would replace about 40
near-duplicate paragraphs.

**10. `Trainer::slots` carries an inline `const` for one card
(`card.rs:586`, Middle Man — minor).** Fine today. A second Stadium with a
fixed search shape means a second special case here rather than data on the
variant. Watch for a third.

### strategy.rs

**11. The 12-entry `tiers` table plus 12 one-line predicates (borderline
Speculative Generality — judgement call).** Six of the twelve are one-line
`matches!` wrappers taking two unused params to fit the function-pointer
signature. Inline the `matches!` tiers; keep helpers only where they need
`view` and `db`. Low stakes — the module doc says it will be revisited.

### crates/sim-wasm/src/lib.rs

**12. `phase_tag` parses `Debug` output as the wire contract
(`lib.rs:269`, fragile coupling — judgement call).**

```rust
fn phase_tag(view: &PlayerView) -> String {
    let debug = format!("{:?}", view.phase);
    debug.split(|c: char| c.is_whitespace() || c == '(').next().unwrap_or("").to_string()
}
```

Renaming a `Phase` variant silently changes the JSON the browser receives.
Add `Phase::tag(&self) -> &'static str` in `state.rs`, matched explicitly,
owned by the engine. This also matches ADR 0096's stated intent.

**13. `default-members = ["."]` hides `sim-wasm` from bare `cargo fmt` /
`clippy`, and the `Cargo.toml` comment is stale (`AGENTS.md`: "Delete a line
that changes nothing").** The comment says the wasm crate will be "added as a
member when it arrives" — it has arrived. Bare `cargo fmt` / `clippy` at the
root touch only `default-members`, so `crates/sim-wasm` has drifted:
`cargo fmt --check -p sim-wasm` reports an un-applied diff. Fix the config gap
(a `--workspace` step in CI, or documented `-p sim-wasm` runs), then the
formatting.

### Primitive Obsession — names as `&'static str` (judgement call, do not
over-correct)

Species and ability identity is matched as raw strings: `action.rs:1418`,
1880 (`pokemon_def(target).name == from`), `state.rs:86`
(`AbilityUsed(PlayerId, PokemonId, &'static str)`), several `card.rs` filters.
This is deliberate and documented — ADR 0019, ADR 0072, and `import::leak`
exist to support it. Do **not** replace evolution matching with `CardDefId`;
that contradicts the ADRs. The defensible move is a newtype:
`SpeciesName(&'static str)` and `AbilityName(&'static str)`. It keeps the
ADRs' semantics and makes a name comparison impossible to confuse with a
`print_id` comparison. The `.contains("Team Rocket")` match at `engine.rs:110`
is the same underlying issue on the other axis's file.

### Standards — the three to fix first

Ranked by risk-adjusted payoff, deliberately not the same as the top lessons.

1. **`state.rs` passive-ability scan helper (finding 7).** Eight-plus call
   sites, purely mechanical, zero semantic risk. Makes the "abilities
   disabled" gate impossible to forget on a new site.
2. **`Phase::tag()` in `state.rs`, consumed by `sim-wasm` (finding 12).**
   Small, removes a real correctness hazard, aligns the seam with ADR 0096.
3. **`Cargo.toml` — fix the stale comment and close the fmt/clippy gap
   (finding 13).** One comment rewrite plus one `--workspace` CI step. The
   only pure-register item with a one-line fix.

The `legal_actions` extraction (finding 1) is the most valuable lesson but a
planned refactor, not a first fix.

---

## Spec

Scope: `engine.rs` (`apply`, `attack`, `attack_with`, `resolve_attack_effect`,
`resolve_trainer`, `retreat`, `settle`), `action.rs::legal_actions`,
`state.rs`, `view.rs`, `strategy.rs`, `bin/play.rs`. Checked against
`docs/architecture/{rules,effects,sources}.md` and ADRs 0002, 0003, 0006,
0009, 0013, 0014, 0042, 0095.

Headline: the load-bearing ADRs hold more cleanly than a reviewer expects. The
defects are at the edges — a panic on the ADR-0095 seam, one doc-order versus
code-order conflict in the damage sequence, and the masked view leaking card
identities through its `phase` field.

### Structural — `legal_actions` versus `apply`: no meaningful re-derivation

ADR 0003: "`apply` refuses anything the list does not hold." The gate at
`engine.rs:15` is the only `legal_actions` call site in the engine. Everything
downstream trusts it:

- **`attack()` (`engine.rs:3221`) re-derives nothing.** Rule 29 affordability,
  the tera surcharge, per-prize discounts, colourless overrides, `held()` for
  Asleep/Paralyzed, rule 17's first-turn skip — all of it lives only in
  `legal_actions` (`action.rs:1633-1660`). `attack()` clones the attack and
  runs it, with no affordability check and no `Result`.
- **`resolve_attack_effect` (659 lines) is infallible** — no `return Err`, two
  `unreachable!` (`engine.rs:4098`, 4101) for effects the dispatcher already
  handled.
- **The turn arms** (`PlayBasic`, `Evolve`, `AttachEnergy`, `PlayTool`,
  `PlayTrainer` — `engine.rs:62-224`) re-check nothing; `engine.rs:146`
  comments "`legal_actions` has already checked it".

The roughly 40 `_ => return Err(IllegalAction)` arms and `setup_player(state)?`
are **not** a parallel gate. They are irrefutable-match fallbacks — the phase
is destructured for its payload and the `_` branch is dead given line 15. They
fire only if `legal_actions` itself has a phase bug. The `.expect` /
`unreachable!` panics in `apply` are the same: type-narrowing unwraps of an
`Option` whose `Some`-ness `legal_actions` guarantees. They assert on
`legal_actions`' correctness; they are not a second gate.

The one deliberate, recorded duplication is retreat cost (ADR 0042): both
`action.rs:1598` and `engine.rs:3189` call `state.effective_retreat_cost`, so
the two reads cannot disagree. This is the model to teach — shared derivation,
not copied logic.

The panic that **is** load-bearing on the gate is in the driver:

```rust
// bin/play.rs:93
apply(&mut state, action).expect("a Strategy chose from legal_actions itself");
```

See "fix first" #1.

### Invariants only partially upheld

**ADR 0006 — the masked view leaks card identities through `phase`.** `view.rs`
copies `phase: state.phase` verbatim. `Phase` is `Copy` and several variants
carry raw `CardId`s into hidden zones: `Phase::Deciding { card, previous, .. }`
(`state.rs:189`), `ChoosingOneOf { card }` (`state.rs:235`),
`LookingAtBottomOfDeck`, the deck-search phases. For the acting player
that is correct — they search their own deck. But `PlayerView::of(state,
opponent)` copies the same `phase`, so an opponent view built mid-search would
carry the searcher's deck card identities. Latent, not live: `play.rs:88`
and `selfplay.rs:44` build the view only for `player_to_act`. Nothing in
`view.rs` sanitizes `phase`, so ADR 0006's "hidden" claim is only partly
upheld.

**ADR 0013 / 0014 / 0002 / 0009 / 0042 — upheld.** Details in "What is already
strong" above.

### Drift — a superseded ADR still cited as the live guarantee

ADR 0002 is `Status: Superseded by 0007 — 2026-09-06`. Its purity clause moved
to 0007, which says "This record carries the whole decision." But two Accepted
ADRs still point at 0002 by number for that guarantee:

```
0042:37  the engine's purity and replay guarantees (ADR 0002, ADR 0009) make a
0096:17  keeps the seam of [ADR 0002](0002-pure-engine-with-a-json-seam.md)
```

A reader following either cross-reference lands on a "Superseded" banner. Per
the register ("Raise a conflict... never settle it silently"), the fix is a
one-line repoint of both to 0007 — raised here, not edited silently.

### Behaviour with no spec or ADR backing

1. **The action list and phase payload sit outside the mask, and no ADR says
   so.** ADR 0086 records one specific carve-out (Claw of Darkness reads the
   opponent's hand). The general rule — that `legal_actions` output and `Phase`
   fields expose card identities the `PlayerView` zone lists hide — has no
   record. ADR 0006 and ADR 0095 both turn on "what a decision-maker may
   read", so this is a real decision with no home.
2. **Rule 32's "stop at 0" is applied to `base`, before the step-32 additions
   it is commented as guarding.** See "fix first" #2.
3. **`settle`'s loop ordering (`engine.rs:4564-4640`)** — game-over check,
   `knock_out_the_dead`, Area Zero bench-shrink, rule-40 promote,
   end-turn/checkup/turn-start, with `is_over()` re-checked three times. Rules
   44/48 and ADRs 0053/0082 cover fragments; the specific precedence (bench
   shrink opens before a pending promote) is a real decision with no record.

### Spec — the three to fix first

1. **`bin/play.rs:93` — `apply(...).expect(...)` at the ADR-0095 seam.** ADR
   0095 says the `Strategy` interface is "built to be swapped out over time — a
   hand-written heuristic today, and eventually something trained". Today's
   `HeuristicStrategy` cannot return an off-list action, so the `.expect` is
   invisible. A trained policy that returns an action not in `legal` crashes
   the driver instead of being refused — `apply` would return `Err`, but the
   caller unwraps. Handle the `Err` (re-prompt or fall back); do not `expect`
   it.
2. **Rule 32 ordering — `damage_dealt_with` stops at 0 before step-32
   additions.** `rules.md:70-71` places the "stop if damage is 0" after the
   your-Pokémon effects. `engine.rs:4413-4416` runs the check against `base`,
   then adds `turn_bonus`, attacker Tools
   (`BonusDamageWithoutRuleBoxVsEx`), Cobalt Command, Lose Cool at 4417-4502.
   A 0-base attack carrying `BonusDamageWithoutRuleBoxVsEx` yields 0 and never
   reaches the Tool loop. Defensible as a reading, but it contradicts the
   documented order and the decision is unrecorded. Raise per the register; do
   not adjudicate against the real rulebook here.
3. **`PlayerView` exposes `state.phase` verbatim, leaking deck `CardId`s.**
   Latent because callers only view for `player_to_act`. Either sanitize
   `phase` in `PlayerView::of`, or write the ADR that says the action list and
   phase payload are unmasked by design.

---

## Summary

- **Standards:** 13 findings (3 cite the register: 8, 9, 13; the rest are
  judgement calls). Worst within axis: finding 1 — `legal_actions` repeats one
  zone-scan shape about 40 times.
- **Spec:** 8 findings across violations, drift, and unrecorded decisions.
  Worst within axis: `bin/play.rs:93` panics on an off-list action at exactly
  the ADR-0095 strategy seam.

No single winner across the two axes — the separation exists to stop one
masking the other.

## Suggested order of work

1. `state.rs` passive-ability scan helper (Standards 7).
2. `Phase::tag()` for the wasm seam (Standards 12).
3. `Cargo.toml` comment plus `--workspace` fmt/clippy in CI (Standards 13).
4. Handle the `Err` in `bin/play.rs:93` (Spec fix-first 1).
5. Raise the Rule 32 ordering conflict as an ADR question (Spec fix-first 2).
6. Sanitize `phase` in `PlayerView::of`, or record that the list is unmasked
   (Spec fix-first 3, plus unrecorded-decision 1).
7. Repoint ADR 0042 and ADR 0096 from 0002 to 0007 (Spec drift).
8. Record the exhaustive-match convention (Standards 8).
9. Plan the `legal_actions` / `player_to_act` extraction as its own effort
   (Standards 1 and 2) — large, valuable, done deliberately.
