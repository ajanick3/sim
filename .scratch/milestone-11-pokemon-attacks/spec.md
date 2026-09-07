# Spec: Pokémon attacks, including their effects

## Problem

Every Trainer-kind milestone is closed. `cargo run --bin blockers`
shows two refusal buckets left of any size: `HasAnAbility` (703 slots)
and `AttackHasText` (370 slots). Per the user's explicit split, these
become two milestones, not one — this one is attacks; Abilities comes
after.

`read_attack` in `import.rs` refuses any attack whose `effect` field is
non-empty, unconditionally. Nothing has ever read an attack's own
effect text — every admitted Pokémon so far happens to print only
plain attacks (damage, at most one `inflicts` condition). This
milestone is the first read of that text at all.

## What the sample actually says

Read the top 40 blockers by slot count (`cargo run --bin blockers`,
`Attack text` breakdown) — 50 distinct names, 370 slots. Recurring
shapes, not fifty one-off cards:

- **Recoil.** "This Pokémon also does N damage to itself" — Carvanha,
  Rellor, Tapu Bulu, Paldean Tauros's second attack. The simplest new
  primitive; damage lands on the attacker, not the defender.
- **A count multiplies the damage.** "N damage for each X" where X is
  a board fact already readable — a Pokémon's own damage counters
  (N's Reshiram), a named subset of the player's own Pokémon with
  damage (Paldean Tauros), the opponent's Basic Energy in discard (N's
  Darmanitan), the opponent's Pokémon ex in play (Dudunsparce ex), the
  player's own Basic Pokémon in play (Passimian). One shape, several
  counted facts.
- **Damage ignores effects on the defender.** "This attack's damage
  isn't affected by any effects on your opponent's Active Pokémon" —
  N's Zekrom, Mega Lopunny ex, Dudunsparce ex. A flag on the damage
  order: skip Weakness/Resistance and any Tool- or Stadium-derived
  bonus/reduction on the defender's side, nothing on the attacker's.
- **A Special Condition, with or without a coin flip.** Direct
  (`Brute Bonnet`'s Poison) or flipped (`Zeraora`, `Dedenne`,
  `Applin`'s bonus damage, `Elgyem`'s damage prevention) — the coin
  flip shape `CoinFlipDiscardOpponentEnergy` already set for a Trainer
  effect, read here for an attack instead.
- **A restriction lasting through the opponent's next turn.** Can't
  retreat (`Yveltal`, `Wellspring Mask Ogerpon ex`), can't play a kind
  of card, less damage taken — several distinct restrictions sharing
  one lifetime: granted now, read during the opponent's very next
  turn, gone after. `turn_bonus`'s "this turn" lifetime does not fit;
  this is "their next turn," survives the boundary between the two.
- **A restriction on the attacker's own next turn.** Can't attack at
  all (N's Zekrom's second attack), can't use this specific attack
  again (`Koraidon ex`). The mirror of the above, own side, own next
  turn.
- **Damage to a Benched Pokémon, alongside or instead of the Active.**
  `Dragapult ex`, `N's Darmanitan`, `Mega Skarmory ex`,
  `Wellspring Mask Ogerpon ex`'s second attack. Nothing built has ever
  damaged a Bench — every Stadium ticket that touched this
  (`Battle Cage`) was refused for exactly this absence. This is the
  ticket that finally builds it.
- **A cost paid in the attacker's own attached Energy, for more
  damage or a bench hit.** `Metagross`, `Raging Bolt ex`,
  `Mega Sharpedo ex`, `Mega Excadrill ex`, `N's Darmanitan`,
  `Mega Skarmory ex`. Discarding or shuffling back the attacker's own
  Energy, sometimes "any amount," sometimes a fixed count.
- **A switch.** `Abra` switches itself out — `SwitchOwnActive`'s shape,
  read from an attack instead of a Trainer. `Metagross` switches the
  opponent's Active — `SwitchOpponentActive`'s shape, same reuse.
- **A search.** `Drilbur`/`Toxel` search up to 2 Basic Pokémon to the
  Bench — the exact shape `Buddy-Buddy Poffin` already built.
  `Duskull` searches its own discard for copies of itself onto the
  Bench — a narrower version of the same search, filtered to its own
  name.
- **Reads or moves the opponent's hand or deck.** `Hoothoot` reveals
  the opponent's hand (`Eri`'s reveal, without the discard half).
  `Mega Excadrill ex` discards the top 2 of the opponent's deck — new,
  no Trainer built has ever read from the *top* of a zone destructively
  rather than the player's own choice.
- **Draws.** `Mega Sharpedo ex` draws 2 outright — trivial.

Not scoped here, deferred or refused on inspection: `Slowking`'s
"discard the top card, and if it's a Pokémon, use one of its attacks
as this one" (copies an attack — a card whose own text depends on
another card's, nothing built or planned does this); `Dedenne`'s
Energy-count-matched multi-attach (a search whose *limit* is itself a
board-read count, not a printed number); `Dwebble`'s search-to-evolve.
Each gets its own ticket to decide build-or-refuse once the more
common shapes are running, the same discipline `Briar` and
`Enhanced Hammer` already set — not assumed refused up front.

## The mechanism

Nothing dispatches an attack's own effect yet. `Attack` gains an
`effect: Option<AttackEffect>` field, a new enum parallel to
`TrainerEffect` but read from `attack()`, not `resolve_trainer`. A
print-or-name lookup table, `known_attack`, mirrors `known_trainer`
exactly — checked in `read_attack` before the unconditional refusal;
`known_trainer_by_print`'s already-proven print-override shape covers
the day a name collision needs it (none does yet).

Reused outright, no new primitive: `SwitchOwnActive`,
`SwitchOpponentActive`, the `Decide`-with-`Destination::Bench` search
shape, the coin-flip-then-effect shape. Each of these already exists
as a `TrainerEffect`; an `AttackEffect` arm that dispatches into the
same phase machinery, rather than duplicating it, is the first
question ticket 1 answers concretely, once the mechanism exists to ask
it against.

## Ticket order

1. The mechanism itself, plus recoil — the smallest real case, proving
   `known_attack`, the new field, and `attack()`'s new dispatch step
   end to end.
2. Damage multiplied by a counted board fact — several cards, one
   shape, a handful of counted-fact variants.
3. Damage that ignores the defender's own effects — a flag read at the
   same damage-order steps Weakness/Resistance already are.
4. A Special Condition, direct or coin-flipped.
5. A restriction through the opponent's next turn.
6. A restriction on the attacker's own next turn.
7. Damage to a Benched Pokémon — the mechanism nothing has built yet.
8. A cost paid in the attacker's own Energy.
9. A switch, reusing `SwitchOwnActive`/`SwitchOpponentActive` outright.
10. A search, reusing the `Decide`-to-Bench shape.
11. Hand/deck reads: reveal, and a destructive top-of-deck discard.
12. Draws.
13. The three deferred-on-inspection cards: a recorded build-or-refuse
    decision each.

Same discipline as every Trainer-kind milestone: check the pool for an
existing shape before adding one, and a card that genuinely cannot run
gets refused with a recorded reason, not silently dropped.
