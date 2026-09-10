import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { asPrompt } from "./shared";
import { PromptBar } from "./PromptBar";

const psychicDraw = asPrompt(["Use Psychic Draw", "Decline Psychic Draw"])!;
const bonusDraw = asPrompt(["Take a bonus card", "Take no more bonus cards"])!;

const meta = {
  title: "live/PromptBar",
  component: PromptBar,
  args: { busy: false, onAct: fn(), prompt: psychicDraw },
} satisfies Meta<typeof PromptBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const AbilityYesNo: Story = {};
export const BonusDraw: Story = { args: { prompt: bonusDraw } };
export const MultipleAccepts: Story = {
  args: {
    prompt: {
      verb: "Use Powerglass?",
      accepts: [
        { label: "Fire Energy", index: 0 },
        { label: "Water Energy", index: 1 },
      ],
      decline: 2,
    },
  },
};
export const Busy: Story = { args: { busy: true } };
