import type { Meta, StoryObj } from "@storybook/nextjs";
import { DamageCounter } from "./DamageCounter";

import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Atoms/DamageCounter",
  component: DamageCounter,
  args: { damage: 30 },
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 24 }}>
        <div style={{ width: 110 }}>
          <Story />
        </div>
      </div>
    ),
  ],
} satisfies Meta<typeof DamageCounter>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Damaged: Story = {};
export const Undamaged: Story = { args: { damage: 0 } };
