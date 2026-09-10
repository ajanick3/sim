import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import type { WireActionMeta } from "../view";
import { card, noArt, psychicEnergy, swatchArt } from "./fixtures";
import { HandStrip } from "./HandStrip";

const smallHand = [
  card({ name: "Munkidori", category: "pokemon" }),
  card({ name: "Iono", category: "supporter" }),
  card({ name: "Ultra Ball", category: "item" }),
  card({ name: "Rescue Board", category: "tool" }),
];

const bigHand = [
  ...smallHand,
  card({ name: "Buddy-Buddy Poffin", category: "item" }),
  card({ name: "Nest Ball", category: "item" }),
  card({ name: "Area Zero Underdepths", category: "stadium" }),
  psychicEnergy(),
  psychicEnergy(),
  psychicEnergy(),
];

// Every card playable.
const playable = (hand: typeof smallHand): WireActionMeta[] =>
  hand.map((c) => ({ kind: "PlayTrainer", card: c.id, target: null }));

const meta = {
  title: "live/HandStrip",
  component: HandStrip,
  args: {
    hand: smallHand,
    meta: playable(smallHand),
    selection: null,
    onHand: fn(),
    onConfirm: fn(),
    art: swatchArt,
    onCardPointerDown: fn(),
    suppressClickRef: { current: false },
    draggingCard: null,
  },
} satisfies Meta<typeof HandStrip>;

export default meta;
type Story = StoryObj<typeof meta>;

export const OneRow: Story = {};
export const TwoRows: Story = { args: { hand: bigHand, meta: playable(bigHand) } };
export const NoArt: Story = { args: { art: noArt } };
export const SomeUnplayable: Story = { args: { meta: playable(smallHand).slice(0, 2) } };
export const Selected: Story = {
  args: { selection: { kind: "hand", card: smallHand[0].id }, confirmIndex: 0 },
};
export const Empty: Story = { args: { hand: [], meta: [] } };
