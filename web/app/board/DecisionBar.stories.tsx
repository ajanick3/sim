import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { noArt, swatchArt, takeActions, takeMeta } from "./fixtures";
import { asDecision } from "./shared";
import { DecisionBar } from "./DecisionBar";

const meta = {
  title: "live/DecisionBar",
  component: DecisionBar,
  args: {
    actions: takeActions,
    meta: takeMeta,
    decision: asDecision(takeActions)!,
    busy: false,
    onAct: fn(),
    art: swatchArt,
  },
} satisfies Meta<typeof DecisionBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const TakeFromDeck: Story = {};
export const NoArt: Story = { args: { art: noArt } };
export const Discard: Story = {
  args: {
    actions: ["Discard Rare Candy", "Discard Ultra Ball", "Stop discarding"],
    meta: [
      {
        kind: "DiscardFromHand",
        card: null,
        target: null,
        card_face: { print_id: "i1", name: "Rare Candy", energy_type: null, category: "item" },
      },
      {
        kind: "DiscardFromHand",
        card: null,
        target: null,
        card_face: { print_id: "i2", name: "Ultra Ball", energy_type: null, category: "item" },
      },
      { kind: "FinishDiscardingFromHand", card: null, target: null },
    ],
    decision: asDecision(["Discard Rare Candy", "Discard Ultra Ball", "Stop discarding"])!,
  },
};
