# Shuppet and Poltchageist: the missing Ability

Type: task
Status: needs-triage

Both build an attack already (`src/import.rs`, per the README Pokémon
table). Both are missing their Ability, per the same table's Ability
column (❌ for each). The PBL prints named by the sample deck (Shuppet
PBL 33, Poltchageist PBL 5) were not in the local `data/cards.json`
snapshot checked for this ticket — pull the exact print's Ability text
from the artifact before starting, rather than reusing another print's
wording; other Shuppet/Poltchageist prints in the data carry different
Abilities (e.g. "Hide 'n' Sneak", "Storehouse Hideaway").

## Acceptance criteria

- [ ] Confirm the PBL print's exact Ability text from `data/cards.json`.
- [ ] The Ability builds in `known_ability` / `resolve_ability` as
      appropriate.
- [ ] A test exercises the Ability's effect.
- [ ] README progress table updated (Ability column moves to ✅ for
      both).
