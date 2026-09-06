# A Trainer's name is matched unless a print overrides it

**Status:** Accepted — 2026-09-06

`known_trainer` reads a Trainer's behaviour from its printed name, and every
card built so far shares that assumption: a name match is safe only where
every print of a name carries the same rules text. That safety had been
checked once, by hand, and the check itself was wrong in a way that would
have misled the next person to run it — it compared raw text, so `Ultra
Ball` read as two different cards over a single card printed with a double
space in one place and a single space in another. `tools/check_trainer_name_safety.py`
fixes the check: it collapses whitespace before comparing, and running it
against the committed artifact finds every Trainer name in the pool safe to
match by name today.

Two shapes were live for the day that check stops passing. The first kept
`known_trainer` as the only table and gave it a second key, the print id,
tried before the name — one function, two lookup paths tangled together.
The second split the two into `known_trainer_by_print` and `known_trainer`,
checked in that order, each a plain match on one kind of string. The second
won: a card whose name is unsafe to match gets one new arm in one function,
and every card whose name is safe — everything today — is untouched.

## Consequences

`known_trainer_by_print` ships empty of any real card, and stays that way
until the day the check finds a genuine conflict. Two synthetic print ids,
`test-print-a` and `test-print-b`, exist in it so `tests/import.rs` can
prove the override actually takes priority over the name — the same reason
[`crate::cards`] ships a small synthetic pool of its own for the engine to
play with, rather than leaving the mechanism untested until a real card
needs it.
