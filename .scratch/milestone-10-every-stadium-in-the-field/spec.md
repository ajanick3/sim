# Spec: every Stadium in the field

## Problem

Milestone 9 closed every Tool in the field. Stadiums are last in the
standing order (Supporters, Items, Tools, Stadiums). `state.stadium:
Option<(PlayerId, CardId)>` already exists — rule 58 already keeps one
in play, discards the old one when a new one replaces it — but nothing
reads a Stadium's effect anywhere. Every Stadium built this milestone
is the first read of that field for anything beyond "which card is it."

13 names in the committed decks:

| Card                       | Effect                                                       |
| --------------------------- | ------------------------------------------------------------- |
| Forest of Vitality          | A Grass Pokémon may evolve the turn it is played (not turn 1) |
| Risky Ruins                 | A non-Darkness Basic benched this turn takes 2 damage counters |
| Battle Cage                 | Attack/Ability damage from the opponent cannot hit a Benched Pokémon |
| Nighttime Mine               | Each Tera Pokémon's attacks cost 1 more Colorless |
| Team Rocket's Factory       | Once a turn, a player who played a "Team Rocket" Supporter may draw 2 |
| Team Rocket's Watchtower    | Colorless Pokémon in play have no Abilities |
| Jamming Tower                | Every attached Tool has no effect |
| Lumiose City                  | Once a turn, search a Basic to the Bench, shuffle, end the turn |
| Festival Grounds             | A Pokémon carrying any Energy cannot take or recover from Special Conditions |
| Academy at Night             | Once a turn, put a card from hand on top of the deck |
| Area Zero Underdepths        | A Tera-controlling player's Bench holds up to 8; leaving play (or losing every Tera Pokémon) discards down to 5 |
| Gravity Mountain              | Every Stage 2 in play has -30 HP |
| N's Castle                    | N's Pokémon in play have no Retreat Cost |

## Refused outright

- **Nighttime Mine** and **Area Zero Underdepths** both gate on "a Tera
  Pokémon," the concept [ADR 0033](../../docs/adr/0033-briar-is-refused-for-a-concept-the-artifact-lacks.md)
  already refused `Briar` for. Applies unchanged.
- **Team Rocket's Watchtower** disables Abilities — a concept nothing
  built has at all (Milestone 8, the Abilities effort, has not started).
  Every Pokémon in this engine already has no Abilities, structurally,
  the same way [ADR 0034](../../docs/adr/0034-enhanced-hammer-is-refused-for-a-card-kind-out-of-scope.md)
  refused `Enhanced Hammer` for a card kind (Special Energy) that
  cannot exist here. Admitting this card would do nothing, always —
  refuse it the same way, rather than build a check against a concept
  that can never be true.

## What every other one turns on

**A Stadium's effect must be read by both players, unconditionally,
from wherever the relevant stat or action is already read** — this is
the fork the milestone actually turns on, distinct from a Tool (owned
by one Pokémon) or a Supporter's `turn_bonus` (owned by one player,
"this turn"). `state.stadium` names a card; every read site needs a
"what does the Stadium in play say" check alongside whatever it
already checks.

Three shapes, by how deep that check has to reach:

- **A derived stat, the same shape ADR 0042 already set for a Tool.**
  `Gravity Mountain` (-30 HP for a Stage 2) extends `effective_hp`;
  `N's Castle` (no Retreat Cost for N's Pokémon) extends
  `effective_retreat_cost`. Both already take an in-play `PokemonId`
  and derive from what's attached — reading `state.stadium` too, inside
  the same function, is the same kind of change ADR 0042 made once
  already, not a new one.
- **A once-per-turn choice, the shape `Limit` already tracks** (rule
  13's once-a-turn Supporter/Stadium gate is exactly this). `Team
  Rocket's Factory`, `Lumiose City`, and `Academy at Night` are each "once
  during each player's turn, may [choice]" — a new `Limit` variant per
  card (or a shared one, parameterized), offered in `legal_actions`
  only while the matching Stadium is in play and unspent.
- **A rule read at a specific existing decision point**, one each:
  `Forest of Vitality` extends the evolution eligibility check
  (`action.rs`, the same two sites `cannot_evolve_this_turn` already
  touches); `Risky Ruins` hooks wherever a Basic goes onto the Bench;
  `Battle Cage` hooks wherever attack damage is applied to a Benched
  Pokémon (nothing does yet — attacks only ever hit the Active in this
  engine, so this may be vacuous the same way Team Rocket's Watchtower
  is, pending a check of whether any built or plannable attack ever
  hits a Bench); `Jamming Tower` hooks every Tool-reading site Milestone
  9 built (`effective_hp`, `effective_retreat_cost`, the two
  `damage_dealt`/`attack` Tool loops); `Festival Grounds` hooks
  `inflict`/the Special-Condition-recovery path.

Order: the derived-stat pair first (cheapest, most precedented), then
the once-per-turn trio (one new shared mechanism, reused three times),
then the rest one at a time, checking each against the pool before
assuming it needs new machinery — the discipline every milestone here
has used.
