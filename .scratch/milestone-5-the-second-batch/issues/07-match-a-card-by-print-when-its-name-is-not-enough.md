# Match a card by print when its name is not enough

Type: task
Status: resolved

`known_trainer_effect` matches by printed name. It is safe today because
every print of each name agrees, and that safety was checked — but the check
itself is wrong in a way that will mislead the next person to run it: it
compares raw text, so `Ultra Ball` reports as two different cards when the
only difference is a double space.

- [x] The check that a name is safe to match on ignores whitespace
- [x] A card can be matched by print id where its name is not enough
- [x] A recorded decision on which cards need it, if any do today

## Resolution

`tools/check_trainer_name_safety.py` collapses whitespace before comparing
a name's printed effect texts. Run against the committed artifact, it
confirms `Ultra Ball`'s two texts collapse to one, and finds every Trainer
name in the pool safe to match by name — nothing needs the override today.

`known_trainer_by_print` is checked before `known_trainer`, keyed on a
print's own id rather than its name. It ships empty of any real card;
[ADR 0020](../../../docs/adr/0020-a-trainer-name-is-matched-unless-a-print-overrides-it.md)
records why it is a separate table checked first rather than a second key
on the same one. Two synthetic print ids in it let `tests/import.rs` prove
the override takes priority, the way `crate::cards` ships a synthetic pool
for the engine's own tests.

This closes Milestone 5. All nine cards the milestone targeted play:
Buddy-Buddy Poffin, Ultra Ball, Crispin, Energy Switch, Dawn, Hilda, Rare
Candy, Cyrano, and Special Red Card.
