# An optional fixed-count Energy cost lets the engine pick the cards

**Status:** Accepted — 2026-09-07

`Wellspring Mask Ogerpon ex`'s `Torrential Pump` reads "You may
shuffle 3 Energy attached to this Pokémon into your deck. If you do,
this attack also does 120 damage to 1 of your opponent's Benched
Pokémon" — the first optional cost in this milestone with a genuine
yes/no gate before anything else happens (ticket 08's discard was
unconditional; ticket 09's search could always be declined only after
starting). Two questions were live: whether accepting needs the player
to choose *which* Energy cards shuffle, and whether the option should
even appear when the attacker cannot pay it.

Every Energy this pool admits is fungible for an effect like this one
— nothing distinguishes one attached Energy card from another once a
fixed count is shuffled away, unlike `Phase::DiscardingForRetreat`
where a mixed-type cost can matter. `Action::AcceptShuffleEnergyForBenchDamage`
takes the first `count` Energy cards `attack` finds attached, with no
per-card choice, rather than opening a `Phase::DiscardingForRetreat`-shaped
multi-step pick for a distinction that carries no game consequence
here. `MayShuffleFixedEnergyThenDamageChosenBenched`'s own resolution
opens `Phase::DecidingToShuffleEnergyForBenchDamage` only when the
attacker already carries at least `count` Energy — an attacker short
of the cost never sees a "may" that could not be paid, matching the
project's running rule that an effect with no valid application opens
no phase at all.
