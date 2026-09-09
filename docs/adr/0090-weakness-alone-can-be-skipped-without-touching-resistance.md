# Weakness alone can be skipped, without touching Resistance

**Status:** Accepted — 2026-09-09

`Chi-Yu`'s `Whirling Envy` reads "This attack's damage isn't affected
by Weakness" — Resistance is left untouched, unlike
`AttackEffect::IgnoresDefendersEffects`, whose own text ("isn't
affected by any effects on your opponent's Active Pokémon") already
skips Weakness, Resistance, and everything else on the defender's
side together via one `ignore_defenders_effects: bool` passed into
`damage_dealt_with`. Reusing that flag would have skipped Resistance
too, which this card's own printed text never asks for.

`damage_dealt_with` takes a second, narrower flag, `ignore_weakness:
bool`, read only at the Weakness step and left out of the Resistance
step entirely. `BonusDamageIfOwnDamageCountersAtLeastIgnoringWeakness`
sets it; every other variant leaves it `false`, so this is additive —
no existing call site's behavior changes. The bonus-damage half of the
same card (`+90 damage at 2 or more of the attacker's own damage
counters`) is an ordinary threshold read in the `base` match, the same
shape `BonusDamageIfOwnDamaged` already takes.

Chi-Yu's other two prints round out the same milestone: `Ground
Melter` (`BonusDamageIfStadiumInPlayThenDiscardsIt`) discards
whichever Stadium is in play, either side's, after bonus damage lands;
`Scorching Earth`
(`DiscardsOpponentsStadiumThenOpponentCannotPlayStadiumsNextTurn`)
narrows the discard to the opponent's own Stadium and chains a
next-turn restriction onto it — the same `opponent_next_turn_restriction`
lifetime `OpponentCannotPlayItemsNextTurn` already carries, read at
the Stadium-offering site in `legal_actions` instead of the Item one.
