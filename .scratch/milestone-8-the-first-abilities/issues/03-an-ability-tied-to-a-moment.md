# An Ability tied to a moment

Type: task
Status: resolved

`Meowth ex`'s Ability, "Last-Ditch Catch": *"Once during your turn,
when you play this Pokémon from your hand onto your Bench, you may
use this Ability. Search your deck for a Supporter card, reveal it,
and put it into your hand. Then, shuffle your deck. You can't use
more than 1 Ability that has 'Last-Ditch' in its name each turn."*
56 slots.

- [x] The trigger is tied to the play itself, not offered as a
      standing `Action::UseAbility`
- [x] `Limit::AbilityUsed` is shared with the standing-Ability shape —
      a played trigger and a chosen one differ only in what opens the
      choice, not in how the once-per-turn bookkeeping works
- [x] `Meowth ex` plays

Recorded in [ADR 0071](../../../docs/adr/0071-an-ability-tied-to-a-moment-hooks-its-trigger-site.md).

## Resolution

`AbilityEffect::WhenBenchedFromHandMaySearchSupporter`, excluded from
`Action::UseAbility`'s own offering. A new `trigger_last_ditch_catch`
hooks `Action::PlayBasic` right where `apply_risky_ruins` already
reads a Stadium's own continuous effect, opening
`Phase::DecidingToUseLastDitchCatch` only when a Supporter exists to
find and the limit is unspent. Declining does not spend the limit.

`Meowth ex`'s own attack, `Tuck Tail`, needed
`AttackEffect::ReturnSelfAndAttachedToHand` too — the first attack
effect to remove the attacker from play entirely, moving its whole
card stack and attachments to hand together (the same "together" rule
22 already keeps for a knockout) and opening `Phase::Promoting`.

Admits all 3 Meowth ex prints. Coverage: `admitted` 585 -> 588.
