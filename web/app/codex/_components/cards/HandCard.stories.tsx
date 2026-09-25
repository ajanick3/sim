import type { Meta, StoryObj } from "@storybook/nextjs";
import { HandCard } from "./HandCard";
import { CRUSHING_HAMMER_ART } from "../../_fixtures/cardArt";
import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Cards/HandCard",
  component: HandCard,
  args: { name: "Crushing Hammer", imageUrl: CRUSHING_HAMMER_ART, onSelect: () => {} },
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ width: 76, padding: 20 }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof HandCard>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Playable: Story = {};
export const Selected: Story = { args: { state: "selected" } };
export const Unavailable: Story = { args: { state: "unavailable", disabled: true } };
