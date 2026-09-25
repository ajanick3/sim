import type { Meta, StoryObj } from "@storybook/nextjs";
import { CardSlot } from "./CardSlot";

import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Atoms/CardSlot",
  component: CardSlot,
  args: { label: "Empty bench slot" },
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 24 }}>
        <div style={{ width: 110 }}>
          <Story />
        </div>
      </div>
    ),
  ],
} satisfies Meta<typeof CardSlot>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Empty: Story = {};
export const Target: Story = { args: { onPlace: () => {} } };
export const Disabled: Story = { args: { onPlace: () => {}, disabled: true } };
