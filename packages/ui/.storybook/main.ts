import type { StorybookConfig } from "@storybook/react-vite";

const config: StorybookConfig = {
  stories: ["../src/**/*.stories.@(ts|tsx)"],
  // addon-essentials bundles Controls (along with Actions, Backgrounds,
  // Docs, Viewport) — nothing further to add for the panel itself.
  addons: ["@storybook/addon-essentials"],
  framework: {
    name: "@storybook/react-vite",
    options: {},
  },
  typescript: {
    // The plain-JS docgen essentials falls back to can't read a union
    // like CardSize or ActionKind off a TS type — every Control would
    // render as a free-text box instead of the dropdown/switch a
    // named union or boolean actually wants.
    reactDocgen: "react-docgen-typescript",
  },
};

export default config;
