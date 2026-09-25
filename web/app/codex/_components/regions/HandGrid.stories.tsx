import type { Meta, StoryObj } from "@storybook/nextjs";
import { HandGrid } from "./HandGrid";
import { DREEPY_ART, CRUSHING_HAMMER_ART } from "../../_fixtures/cardArt";
import theme from "../../_styles/theme.module.css";

const cards = Array.from({ length: 10 }, (_, index) => ({
  id: index + 1,
  name: index % 3 === 0 ? "Dreepy" : "Crushing Hammer",
  imageUrl: index % 3 === 0 ? DREEPY_ART : CRUSHING_HAMMER_ART,
  playable: index !== 8,
}));
const meta = {
  title: "Codex/Regions/HandGrid",
  component: HandGrid,
  args: { cards, onSelect: () => {} },
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ width: 390, padding: 10 }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof HandGrid>;
export default meta;
type Story = StoryObj<typeof meta>;
export const TwoRows: Story = {};
export const Selected: Story = { args: { selectedId: 3 } };
export const Empty: Story = { args: { cards: [] } };
