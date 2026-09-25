import type { Meta, StoryObj } from "@storybook/nextjs";
import { HealthBadge } from "./HealthBadge";

import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Atoms/HealthBadge",
  component: HealthBadge,
  args: { hp: 70 },
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 24 }}>
        <div style={{ width: 110 }}>
          <Story />
        </div>
      </div>
    ),
  ],
} satisfies Meta<typeof HealthBadge>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Yours: Story = {};
export const Opponent: Story = { args: { opponent: true } };
