import type { Meta, StoryObj } from "@storybook/nextjs";
import { Board } from "./Board";
import { CRUSHING_HAMMER_ART, DREEPY_ART } from "../../_fixtures/cardArt";

const meta = { component: Board, parameters: { layout: "fullscreen" } } satisfies Meta<
  typeof Board
>;
export default meta;
type Story = StoryObj<typeof meta>;
const pokemon = (id: number) => ({ id, name: "Dreepy", imageUrl: DREEPY_ART, hp: 70 });
export const Default: Story = {
  args: {
    active: pokemon(1),
    opponentActive: pokemon(2),
    bench: [pokemon(3), pokemon(4)],
    opponentBench: [pokemon(5), pokemon(6)],
    hand: Array.from({ length: 10 }, (_, id) => ({
      id,
      name: "Crushing Hammer",
      imageUrl: CRUSHING_HAMMER_ART,
    })),
    deckCount: 24,
    opponentDeckCount: 31,
    discardCount: 6,
    opponentDiscardCount: 3,
    discardImageUrl: CRUSHING_HAMMER_ART,
    opponentDiscardImageUrl: CRUSHING_HAMMER_ART,
    prizesRemaining: 4,
    opponentPrizesRemaining: 5,
  },
};
