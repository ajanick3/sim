# A forced self-knockout raises damage to effective HP, no separate primitive

**Status:** Accepted — 2026-09-07

`Dusclops`'s and `Dusknoir`'s `Cursed Blast` — "you may put N damage
counters on 1 of your opponent's Pokémon. If you use this Ability,
this Pokémon is Knocked Out" — is the first card whose own text
Knocks out a Pokémon outright, independent of how much damage it
carries. `knock_out_the_dead` is the only place `knock_out` runs,
gated on `remaining_hp(pokemon) <= 0`, and it also awards the Prize —
building a second, HP-independent knockout path would duplicate that
prize-awarding logic and risk it drifting out of sync. Instead,
`Action::DamageOpponentForCursedBlast` raises the carrier's own
`damage` to at least `effective_hp(pokemon)` and calls `settle`: the
ordinary sweep finds it already "dead" by the only rule it knows, and
knocks it out — Prize included — the same way any other knockout
would. The opponent's chosen target takes its damage counters through
the same normal path, so if that damage also happens to exceed their
own HP, both Pokémon leave play in the one `settle` call, exactly as
a real double-knockout would.

The Ability itself opens `Phase::DecidingCursedBlastTarget`, spending
`Limit::AbilityUsed` only once a target is actually chosen — opening
the choice and declining does not count as using it, the same
distinction `Teal Dance` (ADR 0074) and `Last-Ditch Catch` (ADR 0071)
already draw.
