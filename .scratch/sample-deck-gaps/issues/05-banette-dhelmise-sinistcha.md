# Banette, Dhelmise, Sinistcha

Type: task
Status: needs-triage

None of the three build at all — not attacks, not Abilities. The PBL
prints named by the sample deck (Banette PBL 34, Dhelmise PBL 39,
Sinistcha PBL 6) were not in the local `data/cards.json` snapshot
checked for this ticket; several other prints of each name exist with
different text (evolution stage and attacks vary by set). Pull the
exact PBL print before starting.

## Acceptance criteria

- [ ] Confirm each PBL print's exact attack/Ability text from
      `data/cards.json`.
- [ ] All three build a Pokémon, attacks and any Ability included.
- [ ] A test per new attack/Ability effect.
- [ ] README progress table updated.
