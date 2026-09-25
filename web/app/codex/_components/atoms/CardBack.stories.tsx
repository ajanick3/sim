import type { Meta, StoryObj } from "@storybook/nextjs";
import { CardBack } from "./CardBack";
import { CardFrame } from "./CardFrame";
import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Atoms/CardBack",
  component: CardBack,

  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 24 }}>
        <div style={{ width: 110 }}>
          <CardFrame label="Card preview">
            <Story />
          </CardFrame>
        </div>
      </div>
    ),
  ],
} satisfies Meta<typeof CardBack>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Default: Story = {};
