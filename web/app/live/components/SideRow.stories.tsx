import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { noArt, pokemon, side, swatchArt } from "./fixtures";
import { SideRow } from "./SideRow";

const meta = {
  title: "live/SideRow",
  component: SideRow,
  args: {
    side: side(),
    label: "🥇 You",
    mine: true,
    art: swatchArt,
    meta: [],
    selection: null,
    dropTargets: new Map<number, number>(),
    onPokemon: fn(),
    onViewDiscard: fn(),
  },
} satisfies Meta<typeof SideRow>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Mine: Story = {};
export const Opponent: Story = { args: { label: "🥈 Opponent", mine: false } };
export const NoArt: Story = { args: { art: noArt } };
export const FullBench: Story = {
  args: {
    side: side({
      bench: [
        pokemon({ name: "Dreepy" }),
        pokemon({ name: "Drakloak" }),
        pokemon({ name: "Dreepy" }),
        pokemon({ name: "Budew" }),
        pokemon({ name: "Fezandipiti ex" }),
      ],
    }),
  },
};
export const EmptyBench: Story = { args: { side: side({ bench: [] }), onPlaceBench: fn() } };
