# A Trainer that plays as a Pokémon is imported as one, and self-discards by name

**Status:** Accepted — 2026-09-22

The seven "Antique … Fossil" Items (Armor, Cover, Jaw, Plume, Root, Sail,
Skull) print `category: "Trainer"`, `trainerType: "Item"`, but their own
effect text reads: "Play this card as if it were a 60-HP Basic {C}
Pokémon. This card can't be affected by any Special Conditions and can't
retreat. At any time during your turn, you may discard this card from
play." Each also carries one unique passive Ability. `read_card` in
`src/import.rs` dispatches once on `card["category"]` and never revisits
the choice: `"Trainer"` always builds a `CardDef::Trainer`, `"Pokemon"`
always builds a `CardDef::Pokemon`. Nothing about these seven fits either
branch cleanly — they need the whole Pokémon apparatus (a Bench slot, HP,
Retreat, Prize on knockout) with none of the Trainer machinery
(`resolve_trainer`, `Trainer::slots()`) ever touching them again once
played.

## Decision

`read_card` special-cases these seven names before the `category` match:
each builds a `CardDef::Pokemon` directly from the same JSON record's own
`hp`/`abilities` fields (present despite `category: "Trainer"`), Basic,
Colorless, no attacks, `retreat_cost: u8::MAX` — the same "make it
unplayable with a fixed cost" trick this effort has already used
elsewhere rather than adding a `cannot_retreat: bool` a Pokémon carries
in fewer than a dozen cases total. The card is drawn from and stays in
hand exactly like a Basic Pokémon (`Action::PlayBasic` admits it — the
action already only checks `is_basic_pokemon()`, not the card's own
category), so no `Action` or `legal_actions` change is needed to *play*
one.

Two more facts about these seven do not fit anywhere a `bool` field on
`Pokemon` could cheaply go, since `Pokemon` has one real construction
site in `src/import.rs` but 147 literal constructions across
`src/cards.rs` and `tests/*.rs` that a new required field would touch:

- **"Can't be affected by any Special Conditions."** Read directly in
  `GameState::inflict`, gated on the exact printed name (a fixed list of
  the seven), the same way a handful of other single-card carve-outs in
  this codebase (`Cobalt Command`'s own name check, `BonusDamageVsActive-
  ExForCarrierNamed`) already read a name rather than a new struct flag.
- **"At any time during your turn, you may discard this card from
  play."** A new `Action::DiscardOwnPokemonFromPlay { pokemon }`, offered
  in `legal_actions` during `Phase::Main` for any of the player's own
  in-play Pokémon whose name is one of the seven. `apply` moves its own
  card and everything attached to the owner's discard (rule 22, the same
  "moves together" rule a knockout already keeps), clears the Active
  slot if it was Active, and removes it from the Bench if not — no
  Prize, no `knocked_out` flag, the same shape `discard_benched_pokemon`
  (`Area Zero Underdepths`) already established for a non-Knockout exit,
  widened to cover the Active spot too.

Each Fossil's own unique Ability is a new, ordinary passive
`AbilityEffect` variant, read at whichever single site its own text
names (`damage_dealt_with`, `GameState::inflict`, or a new site where
none existed) — no different in shape from any other passive Ability
this effort has already built.

## Consequences

- `read_card`'s name-based special case is the one place a future card
  sharing this "Trainer that plays as a Pokémon" shape would need to be
  added; a second family of such cards big enough to earn its own
  dispatch key would be the trigger to generalize this into something
  less name-list-shaped.
- `Action::DiscardOwnPokemonFromPlay` is gated purely by name today. If
  a future non-Fossil card ever grants the same voluntary self-discard,
  the gate widens to whatever condition that card actually prints,
  rather than staying Fossil-specific by construction.
- A Fossil's `retreat_cost: u8::MAX` means `effective_retreat_cost`
  never comes back low enough to pay — cheaper than a new field, at the
  cost of being a convention future readers need this record to explain
  rather than a type that enforces it.
