import type { Preview } from "@storybook/react";

const preview: Preview = {
  parameters: {
    backgrounds: { default: "dark", values: [{ name: "dark", value: "#121212" }] },
  },
};

export default preview;
