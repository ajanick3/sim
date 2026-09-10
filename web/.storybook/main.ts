import type { StorybookConfig } from "@storybook/nextjs";

// Stories live next to the components they exercise, under app/board.
// No `staticDirs` — no story loads from `public/`, and the static
// Storybook is built into `public/storybook`, which cannot copy its
// own parent.
const config: StorybookConfig = {
  stories: ["../app/**/*.stories.@(ts|tsx)"],
  addons: [],
  framework: { name: "@storybook/nextjs", options: {} },
};

export default config;
