# A damage bonus can last only this turn

**Status:** Accepted — 2026-09-07

`Black Belt's Training` and `Gladion's Final Battle` both add damage
"during this turn" against a restricted opponent's Active — a Pokémon ex
for one, the opposite, a Pokémon with no Rule Box, for the other — and
neither lasts past the turn that played them. Milestone 6 already named
the mechanism this is not: a Tool or Stadium's continuous effect, which
holds for as long as the card itself stays in play, checked wherever the
rule it modifies runs, and still out of scope. A this-turn bonus is
smaller than that in every direction that matters — it never outlives a
turn boundary, and nothing needs to stay in play to keep it alive — so it
earned its own field rather than borrowing the larger mechanism early.

`GameState::turn_bonus: Option<(u32, TurnBonusTarget)>` is that field,
read in `damage_dealt` at step 32 — "effects on the attacking player's
Pokémon," the slot the function's own comment already reserved before any
card needed it — and cleared in `begin_turn` beside `spent`, the same
place every other once-a-turn fact already resets. One bonus at a time is
enough for the whole pool today: nothing plays two such cards before an
attack lands.

## Consequences

`TurnBonusTarget` names only what the pool actually restricts to —
`OpponentActiveEx` and `OpponentActiveWithoutRuleBox` — both always the
opponent's Active, since nothing bonuses an attack against a Benched
Pokémon. `Requirement::HandSizeIs` is unrelated in mechanism but landed in
the same ticket: `Gladion's Final Battle` needed it to gate "only when it
is the last card in your hand," checked while the card itself is still
counted.
