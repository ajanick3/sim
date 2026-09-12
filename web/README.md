# web

The browser front end: a Next.js app that plays one game against the wasm
build of the engine.

- `app/game-shell.tsx` — loads the engine, replays the recipe the URL
  carries, applies moves, keeps the address bar in step for Back / Forward.
- `app/board/` — the board. One component per file, a card frame plus the
  pieces it composes (`LiveMon`, `HandStrip`, `SideRow`, `DeckPile`,
  `DecisionBar`, `PromptBar`, `SideRail`, …).
- `app/globals.css` — the palette and card tokens (`--color-*`,
  `--radius-card`, `--shadow-card`); `app/board/sizes.ts` — the named card
  size scale. A board component picks a token or a named size; it does not
  spell out a colour, a radius, or `w-[…] h-[…]`.

## Scripts

| script | does |
| --- | --- |
| `pnpm run dev` | dev server |
| `pnpm run check` | lint, format check, typecheck, unit tests |
| `pnpm run build` | production build |
| `pnpm run storybook` | Storybook dev server on :6006 |
| `pnpm run build-storybook` | static Storybook build |
| `pnpm run wasm` | rebuild `public/pkg` from `crates/sim-wasm` |

## The stories rule

A board component is not finished without a `.stories.tsx` next to it that
covers its states — every visual branch a prop can take. Story data comes
from `app/board/fixtures.ts`; nothing in a story hits the network.
