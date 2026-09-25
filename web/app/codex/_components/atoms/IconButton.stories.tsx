import type { Meta, StoryObj } from "@storybook/nextjs";
import { IconButton } from "./IconButton";

import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Atoms/IconButton",
  component: IconButton,
  args: { label: "Close", children: "×" },
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 24 }}>
        <div style={{ width: 110 }}>
          <Story />
        </div>
      </div>
    ),
  ],
} satisfies Meta<typeof IconButton>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Default: Story = {};
export const Disabled: Story = { args: { disabled: true } };
