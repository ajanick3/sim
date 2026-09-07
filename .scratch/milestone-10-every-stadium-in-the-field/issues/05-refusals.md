# Nighttime Mine, Area Zero Underdepths, Team Rocket's Watchtower & Battle Cage

Type: task
Status: resolved

Four Stadiums refused outright, none needing new code:

- **Nighttime Mine** and **Area Zero Underdepths** both gate on "a Tera
  Pokémon," the concept [ADR 0033](../../../docs/adr/0033-briar-is-refused-for-a-concept-the-artifact-lacks.md)
  already refused `Briar` for. Applies unchanged.
- **Team Rocket's Watchtower** ("Colorless Pokémon... have no
  Abilities") and **Battle Cage** ("prevent... damage counters... on
  Benched Pokémon") each disable a mechanism this engine does not have
  at all — no Ability exists anywhere, and no attack this engine runs
  ever hits a Bench. Recorded in
  [ADR 0051](../../../docs/adr/0051-battle-cage-and-team-rockets-watchtower-are-refused.md).

- [x] A recorded decision for each

## Resolution

No `known_trainer` entries added for any of the four. Coverage does not
move.
