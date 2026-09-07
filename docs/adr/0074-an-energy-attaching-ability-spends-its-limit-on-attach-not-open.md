# An Energy-attaching Ability spends its limit on attach, not on opening the choice

**Status:** Accepted — 2026-09-07

`Teal Mask Ogerpon ex`'s `Teal Dance` — "Once during your turn, you
may attach a Basic Grass Energy card from your hand to this Pokémon.
If you attached Energy to a Pokémon in this way, draw a card" — is the
first Ability where attaching Energy is the effect itself, not a
side cost paid to reach one. `Action::UseAbility` had, until now,
always resolved an Ability's whole effect in the one action; this one
needs a further choice (which Energy, if any), so it opens
`Phase::DecidingToUseTealDance` instead. `Limit::AbilityUsed` is
spent only in `Action::AttachEnergyForTealDance`, not in
`Action::UseAbility` itself — opening the choice and then declining
must not count as using the Ability, the same distinction ticket 03's
`Last-Ditch Catch` already drew between offering a search and actually
taking from it. `Action::UseAbility`'s own dispatch moves its
`Limit::AbilityUsed` spend into each immediately-resolving arm
individually, rather than unconditionally before the match, to make
room for an arm that defers it.

The attach itself does not touch `Limit::EnergyAttached`, the normal
once-per-turn manual-attach gate — an Ability's own Energy attachment
is independent of it, per the card's own ruling, and nothing here
reads or spends that limit.

`Teal Mask Ogerpon ex`'s own attack, `Myriad Leaf Shower` ("This
attack does 30 more damage for each Energy attached to both Active
Pokémon"), needed `Count::EnergyOnBothActivesCount` — the first count
read from both sides of the board at once, rather than one side alone
the way every existing `Count` variant already is. `count_for_attack`
gains a `defender` parameter to read it.
