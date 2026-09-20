# @sim/chatgpt

This package supplies the responsive Pokémon board layouts approved in the
ChatGPT design review. It uses the repository's React and MUI stack.

## Components

- `PokemonBoard` selects the layout from the viewport width.
- `MobilePokemonBoard` uses a five-column, two-row hand.
- `TabletPokemonBoard` uses a seven-column hand and compact actions.
- `MediumDesktopPokemonBoard` adds a persistent action and log rail.

All rendered cards use the exact `245 / 337` source-image ratio. Perspective
applies to the court once. It does not change a card's intrinsic dimensions.

## Checks

```sh
pnpm install
pnpm run typecheck
pnpm run test
```
