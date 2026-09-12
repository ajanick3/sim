# packages/ui glossary

This package's own domain: a visual component library for the board,
independent of the game engine and of `web/`'s Tailwind app. It knows
nothing about the rules — it draws pictures of a board a caller feeds
it plain data for.

**Region**:
A composite area of the board — Active, Bench, Hand, Prizes, Deck,
Discard, Stadium, Search — built from one or more `Card`s plus
whatever atoms overlay them. Deliberately not "Zone": the engine's own
`Zone` type (`Zone::Deck`/`Hand`/`Discard`) is narrower — it excludes
Active, Bench, and Stadium — so reusing "Zone" here would misname most
Regions. Also not "Slot": the engine already uses `Slot` for a
search-effect's own per-card offer.

**Card**:
The one card frame this library draws, at a named `CardSize`, showing
either the top-of-print sliver (Active) or the whole illustration
(everything else). `fluid` lets a `Card` fill its parent's width at
its size's own aspect ratio instead of a fixed pixel box — how a
squished Bench row keeps working past five columns.

**Atom**:
A small overlay a `Card` carries but doesn't draw itself: `HealthBar`
(the HP pill — a number, never a depleting bar, per the physical
card), `DamageCounter`, `EnergyChip`, `ToolBadge`, `ActiveIndicator`
(the ring marking a spot as the Active).

**Far**:
A Region prop, not a Card prop: the opponent's side of the board reads
at a smaller scale than the player's own. Lives on `ActiveRegion` and
`BenchRegion`, since a Bench card's own size is fluid, not fixed.
