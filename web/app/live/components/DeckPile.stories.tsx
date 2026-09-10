import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { DeckPile } from "./DeckPile";
import { card, noArt, swatchArt } from "./fixtures";

const discard = [
  card({ name: "Iono" }),
  card({ name: "Nest Ball" }),
  card({ name: "Boss's Orders" }),
];

const meta = {
  title: "live/DeckPile",
  component: DeckPile,
  args: { deck: 42, discard, art: noArt, mine: true, onView: fn() },
} satisfies Meta<typeof DeckPile>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Mine: Story = {};
export const WithArt: Story = { args: { art: swatchArt } };
export const Opponent: Story = { args: { mine: false } };
export const EmptyDiscard: Story = { args: { discard: [], onView: undefined } };
