import type { Meta, StoryObj } from "@storybook/react";
import {
  MediumDesktopPokemonBoard,
  MobilePokemonBoard,
  PokemonBoard,
  TabletPokemonBoard,
} from "./PokemonBoard";
import type { PokemonBoardState } from "./types";

const state: PokemonBoardState = {
  turn: 4,
  isPlayerTurn: true,
  stadiumName: "Beach Court",
  phaseLabel: "Main phase",
  player: {
    name: "Nick",
    active: { id: 1, name: "Player active", damage: 30 },
    bench: [
      { id: 2, name: "Player bench 1" },
      { id: 3, name: "Player bench 2" },
      { id: 4, name: "Player bench 3" },
      null,
    ],
    hand: Array.from({ length: 7 }, (_, index) => ({
      id: 10 + index,
      name: `Hand card ${index + 1}`,
    })),
    deckCount: 31,
    discardCount: 4,
    prizesRemaining: 4,
  },
  opponent: {
    name: "Rival",
    active: { id: 20, name: "Opponent active", damage: 80 },
    bench: [{ id: 21, name: "Opponent bench 1" }],
    deckCount: 28,
    discardCount: 7,
    prizesRemaining: 3,
  },
  log: ["Nick drew a card", "Rival ended their turn"],
};

const meta: Meta<typeof PokemonBoard> = {
  title: "ChatGPT board/PokemonBoard",
  component: PokemonBoard,
  args: { state },
};

export default meta;

type Story = StoryObj<typeof PokemonBoard>;

/** Picks a layout from the viewport — resize the preview to see it switch. */
export const Responsive: Story = {};

export const Mobile: StoryObj<typeof MobilePokemonBoard> = {
  render: (args) => <MobilePokemonBoard {...args} />,
  args: { state },
};

export const Tablet: StoryObj<typeof TabletPokemonBoard> = {
  render: (args) => <TabletPokemonBoard {...args} />,
  args: { state, selectedCardId: 10 },
};

export const MediumDesktop: StoryObj<typeof MediumDesktopPokemonBoard> = {
  render: (args) => <MediumDesktopPokemonBoard {...args} />,
  args: { state, selectedCardId: 10 },
};
