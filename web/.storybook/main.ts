import type { StorybookConfig } from "@storybook/nextjs";

// Stories live next to the components they exercise, under app/live/components.
const config: StorybookConfig = {
  stories: ["../app/**/*.stories.@(ts|tsx)"],
  addons: [],
  framework: { name: "@storybook/nextjs", options: {} },
  staticDirs: ["../public"],
};

export default config;
