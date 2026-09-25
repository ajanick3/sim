import type { Meta, StoryObj } from "@storybook/nextjs";
import { PrizeMarker } from "./PrizeMarker";

import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Atoms/PrizeMarker",
  component: PrizeMarker,

  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 24 }}>
        <div style={{ width: 110 }}>
          <Story />
        </div>
      </div>
    ),
  ],
} satisfies Meta<typeof PrizeMarker>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Remaining: Story = {};
export const Taken: Story = { args: { taken: true } };
