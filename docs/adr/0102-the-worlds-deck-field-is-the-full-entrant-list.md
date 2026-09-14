# The Worlds deck field is limitlesstcg's Day 2 standings, all 143

**Status:** Accepted — 2026-09-14

`decks/2026-worlds/` held the top 64 finishers of the 2026 World
Championships, fetched from the operator's own mirror site as plain text.
Three of the 64 were dropped: their decklists, in the language each player
submitted them in, named cards by a set code this artifact's English pool
does not hold. limitlesstcg's tournament page
(`limitlesstcg.com/tournaments/515`) lists 797 entrants in total, and its
Decklists tab was read to see how much further the field could grow.
Fetching it settled two open questions at once. First, the site publishes a
decklist for only the 143 players who reached a Day 2 standing — an entrant
eliminated in Swiss has no decklist page here at all, so 143 is a ceiling
this source cannot pass, not a number chosen freely. Second, and unexpected:
the site's decklists show each card by its normalized English print
regardless of what language the player entered it in, so the three
placements dropped from the original 64 resolve cleanly through this source
— the language gap that blocked them closes for free. All 143 are kept, and
`tools/fetch_worlds_decklists.py` parses them from the one Decklists page
directly, in the same three-section, one-line-per-card format the existing
files already used. Growing the field past 99 also meant widening the
`<placement>-<player-slug>` filename's zero-padding from two digits to
three, so the existing 64 files were renamed to match.
