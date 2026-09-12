# @sim/ui

A visual component library for the board — built on MUI, documented
in its own Storybook, and completely independent of `web/`'s Tailwind
app and of the Rust engine. It draws pictures of a board a caller
feeds it plain data for; it holds no game rules.

This is a parallel exploration, not a replacement: `web/` keeps
running on Tailwind exactly as it does today. Wiring this library into
the live board is a separate, later decision.

## Run it

```sh
npm install
npm run storybook   # http://localhost:6007
```

## Shape

- `src/primitives/` — `Card`, `CardImage`. The one card frame the
  library draws, at a named size, plus the `<img>` it wraps.
- `src/atoms/` — small overlays a `Card` carries: `HealthBar`,
  `DamageCounter`, `EnergyChip`, `ToolBadge`, `ActiveIndicator`.
- `src/regions/` — the composite board areas: `ActiveRegion`,
  `BenchRegion`, `HandRegion`, `PrizesRegion`, `DeckRegion`,
  `DiscardRegion` (+ `DiscardViewDialog`), `StadiumRegion`,
  `SearchRegion`.

See [`CONTEXT.md`](./CONTEXT.md) for the vocabulary — Region, Card,
Atom, Far — and why "Zone" and "Slot" were deliberately not reused
from the engine.
