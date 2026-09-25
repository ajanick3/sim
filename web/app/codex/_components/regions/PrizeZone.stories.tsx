import type { Meta, StoryObj } from "@storybook/nextjs";
import { PrizeZone } from "./PrizeZone";
import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Regions/PrizeZone",
  component: PrizeZone,
  args: { remaining: 6 },
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 20 }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof PrizeZone>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Full: Story = {};
export const HalfTaken: Story = { args: { remaining: 3 } };
export const Empty: Story = { args: { remaining: 0 } };
