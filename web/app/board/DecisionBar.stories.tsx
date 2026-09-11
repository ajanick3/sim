import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { card, noArt, swatchArt, takeActions, takeMeta } from "./fixtures";
import { asDecision } from "./shared";
import type { WireCard } from "../view";
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

// A whole-deck search: six cards in the deck, two of which this step
// may take. The rest are drawn dimmed.
const deck: WireCard[] = [
  card({ name: "Gardevoir ex", category: "pokemon", print_id: "p-gardevoir" }),
  card({ name: "Kirlia", category: "pokemon", print_id: "p-kirlia" }),
  card({ name: "Ralts", category: "pokemon", print_id: "p-ralts" }),
  card({ name: "Iono", category: "supporter", print_id: "t-iono" }),
  card({ name: "Ultra Ball", category: "item", print_id: "t-ultraball" }),
  card({ name: "Psychic Energy", category: "energy", print_id: "e-psychic" }),
];
const takeActionsLib = ["Take Ralts", "Take Kirlia", "Stop searching"];

export const WholeDeck: Story = {
  args: {
    actions: takeActionsLib,
    meta: [
      { kind: "TakeCard", card: deck[2].id, target: null },
      { kind: "TakeCard", card: deck[1].id, target: null },
      { kind: "FinishDeciding", card: null, target: null },
    ],
    decision: asDecision(takeActionsLib)!,
    deck,
  },
};
