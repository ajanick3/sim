import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { DeckSearchDialog } from "./DeckSearchDialog";
import { CRUSHING_HAMMER_ART, DREEPY_ART } from "../../_fixtures/cardArt";

const meta = {
  component: DeckSearchDialog,
  args: {
    onConfirm: fn(),
    onDone: fn(),
    cards: Array.from({ length: 12 }, (_, id) => ({
      id,
      name: id % 3 ? "Crushing Hammer" : "Dreepy",
      imageUrl: id % 3 ? CRUSHING_HAMMER_ART : DREEPY_ART,
      eligible: id % 4 !== 2,
    })),
  },
} satisfies Meta<typeof DeckSearchDialog>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Default: Story = {};
