# `Durable Body` reads the same flag `Lillie's Pearl` already reads

**Status:** Accepted — 2026-09-09

`Annihilape`'s `Durable Body` — "If this Pokémon would be Knocked Out
by damage from an attack, flip a coin. If heads, this Pokémon is not
Knocked Out, and its remaining HP becomes 10" — is the first Ability
in this engine able to prevent a Knockout outright, rather than only
redirect or reduce the damage that leads to one. It also needs to
tell an attack-caused Knockout apart from a checkup's (Poison,
Burned), since its own text names only "damage from an attack."

`knock_out_the_dead` already carries that exact distinction:
`Lillie's Pearl` (fewer Prizes on an attack-caused Knockout) reads
`state.attacking_defender`, set only when the most recent damage came
from an attack and consumed the moment `knock_out_the_dead` runs.
`Durable Body` reads the same flag, at the very top of the per-Pokémon
loop — before a Prize is ever taken — and on heads sets `damage` to
`effective_hp - 10` and `continue`s past the Knockout and Prize logic
entirely, rather than running through `knock_out` and `take_prizes`
first and trying to undo their effects after.

`Ghostly Blow`, the attack on the same print, closes alongside it:
`PlacesDamageCountersOnChosenOpponentBenched` is
`DamageChosenOpponentBenchedEx` widened from only a Benched ex to any
Benched Pokémon — the same widening `ChoosingAnyBenchedDamageTarget`
takes from `ChoosingBenchedExDamageTarget`.
