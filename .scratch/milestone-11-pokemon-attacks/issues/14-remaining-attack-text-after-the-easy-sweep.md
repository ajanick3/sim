# Remaining Attack text after the easy sweep

Type: task
Status: open

Ticket 13 closed the spec's own 13-ticket order. This session then
kept sweeping `cargo run --bin blockers`'s `Attack text` bucket for
names it could admit from an already-built or trivially-mirrored
shape, well past the spec's own survey: Beldum, Dunsparce (Trading
Places), Moltres, Torchic, Celebi, Buneary, Bayleef, Chikorita,
Dunsparce (Dig) and Elgyem (Hide), Budew, Wellspring Mask Ogerpon ex,
Paldean Tauros, Combusken, Elgyem (Slight Shift), and Zeraora — 14
more PRs, coverage 546 -> 581.

What is left, per `blockers`, is the harder remainder: every name
still there pairs its own attack with a second one this session found
genuinely new complexity in, not a quick mirror:

- **Metagross** (9 slots) — `Meteor Mash`'s persistent same-attack
  bonus across the turn boundary ("this Pokémon's Meteor Mash attack
  does 60 more damage" next turn) is a shape nothing built reads:
  every existing "next turn" restriction is a flag, not a
  per-attack-name bonus that must still find the right attack later.
- **Slowking** (9 slots) — `Seek Inspiration` is refused (ADR 0066);
  its other prints (`Wash the Slate Clean`, already built) leave only
  prints still paired with `Seek Inspiration` itself.
- **Raging Bolt ex** (9 slots) — `Bellowing Thunder`'s "discard any
  amount of Basic Energy... this attack does 70 damage for each"
  is a player-chosen variable count, not a fixed one; `Burst Roar`
  discards the whole hand and draws 6.
- **Mega Excadrill ex** (8 slots) — `Maximum Drilling`'s "if this
  Pokémon has at least 2 extra Energy attached" is a threshold read
  against the attack's own cost, not a flat board fact; `Undermine`'s
  top-of-deck discard was deferred in ticket 11 for exactly this
  pairing.
- **Mega Skarmory ex**, **Mega Lopunny ex**, **Hoothoot**, **Koraidon
  ex**, **Chi-Yu**, and the remaining 1-slot names — each combines two
  shapes, at least one bespoke, for one or two prints. Read fresh
  against the pool before assuming refusal.

Recommendation: revisit this list once Abilities gives the pool a
reason to touch these Pokémon's other attacks anyway, or when a slot
count here rises enough to justify the shape on its own. Not
scheduled as its own ticket order — a candidate list, not a plan.
