import type { Preview } from "@storybook/nextjs";
import "../app/globals.css";

// The board's palette comes from globals.css (`@theme` tokens on :root and a
// dark `body` background). Every story renders on that same ground.
const preview: Preview = {
  parameters: {
    layout: "centered",
    controls: { expanded: true },
  },
  decorators: [
    (Story) => (
      <div style={{ minWidth: 320, padding: 16 }}>
        <Story />
      </div>
    ),
  ],
};

export default preview;
