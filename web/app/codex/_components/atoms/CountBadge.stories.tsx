import type { Meta, StoryObj } from "@storybook/nextjs";
import { CountBadge } from "./CountBadge";

import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Atoms/CountBadge",
  component: CountBadge,
  args: { count: 24, label: "Deck cards" },
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 24 }}>
        <div style={{ width: 110 }}>
          <Story />
        </div>
      </div>
    ),
  ],
} satisfies Meta<typeof CountBadge>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Default: Story = {};
export const Empty: Story = { args: { count: 0 } };
