# A play-triggered Ability hooks its own trigger site directly

**Status:** Accepted — 2026-09-07

`Meowth ex`'s `Last-Ditch Catch` — "Once during your turn, when you
play this Pokémon from your hand onto your Bench, you may use this
Ability" — fires at a moment, not on demand. Ticket 01's
`Action::UseAbility` models an Ability the player reaches for
whenever its own gates hold; this Ability has no such standing
window; it exists only in the instant `Action::PlayBasic` benches the
card. `AbilityEffect::WhenBenchedFromHandMaySearchSupporter` is
excluded from `Action::UseAbility`'s own offering outright (`eligible`
returns `false` for it), and a new `trigger_last_ditch_catch` runs
right after `Action::PlayBasic` benches the Pokémon — the same site
`apply_risky_ruins` already reads a Stadium's continuous effect from,
generalized to a card's own Ability instead of a Stadium's. `Limit::AbilityUsed`
is reused unchanged: the trigger checks it before opening a phase, and
`Action::TakeSupporterForLastDitchCatch` spends it, exactly the way
`Action::UseAbility` already does — a played trigger and a chosen one
share the same once-per-turn bookkeeping, they only differ in what
opens the choice. Declining does not spend the limit: the card's own
"may" is a choice not to act at all, not a spent turn.

`Meowth ex`'s own attack, `Tuck Tail` ("Put this Pokémon and all
attached cards into your hand"), needed `AttackEffect::ReturnSelfAndAttachedToHand`
alongside the Ability to admit the same card. It moves the attacker's
whole card stack and every attachment together, the same "together"
rule 22 already keeps for a knockout (`knock_out`'s own `cards`/`attached`
move, mirrored here to hand instead of discard), and opens `Phase::Promoting`
since removing the Active always needs a replacement — skipped
outright with no Bench to promote from, since there would be nothing
to leave the Active Spot for.
