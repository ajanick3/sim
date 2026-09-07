# A conditional-damage attack's `+`-suffixed printed damage still parses

**Status:** Accepted — 2026-09-08

## Context

`Applin`'s `Tumbling Attack` prints damage as `"10+"` — a string, the
same shape `"40×"` already is for a per-count attack (ADR 0055), but
with the other suffix TCGdex uses for "this attack's damage can be
more than what's printed." `read_attack`'s damage parsing only accepted
`×`; the first attempt to admit `Applin` refused it on
`DamageIsNotANumber` even though `known_attack` had already matched
`AttackEffect::CoinFlipBonusDamage` for it — the mechanism was right,
the parsing that should have let it through was not.

## Decision

The `String` arm of `read_attack`'s damage match accepts either suffix,
trimming both (`s.trim_end_matches(['×', '+'])`) the same way — still
gated on `effect.is_some()`, so an unmatched `+`-suffixed attack still
refuses on `DamageIsNotANumber`, unchanged. The digits are read only to
confirm the print agrees with what `known_attack` supplies, for both
suffixes alike; `CoinFlipBonusDamage`'s own bonus amount, not this
parsed base, is what actually varies the damage.

## Consequences

Every printed-damage suffix TCGdex uses for a conditional or variable
amount (`×`, `+`, and any future one the sample turns up) is read the
same way: parsed as the base, trusted only once an effect already
matched. A card refusing on `DamageIsNotANumber` after `known_attack`
matched it is the signal to check for a suffix this parsing does not
handle yet, the way this one was found.
