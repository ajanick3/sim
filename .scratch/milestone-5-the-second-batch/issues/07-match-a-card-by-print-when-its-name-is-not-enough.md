# Match a card by print when its name is not enough

Type: task
Status: ready-for-agent

`known_trainer_effect` matches by printed name. It is safe today because
every print of each name agrees, and that safety was checked — but the check
itself is wrong in a way that will mislead the next person to run it: it
compares raw text, so `Ultra Ball` reports as two different cards when the
only difference is a double space.

- [ ] The check that a name is safe to match on ignores whitespace
- [ ] A card can be matched by print id where its name is not enough
- [ ] A recorded decision on which cards need it, if any do today
