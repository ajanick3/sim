import type { Meta, StoryObj } from "@storybook/react";
import { ActionBar } from "./ActionBar";

const meta = {
  title: "regions/ActionBar",
  component: ActionBar,
  args: { onAct: () => {} },
} satisfies Meta<typeof ActionBar>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Every action kind, at once — the whole reimagined vocabulary: five
 *  icons, no label sitting on a button. */
export const AllKinds: Story = {
  args: {
    actions: [
      { id: 1, kind: "attach", label: "Attach Psychic Energy" },
      { id: 2, kind: "retreat", label: "Retreat" },
      { id: 3, kind: "attack", label: "Psybeam" },
      { id: 4, kind: "ability", label: "Use Last-Ditch Catch" },
      { id: 5, kind: "evolve", label: "Evolve into Alakazam" },
    ],
  },
};

export const OneDisabled: Story = {
  args: {
    actions: [
      { id: 1, kind: "attack", label: "Psybeam" },
      { id: 2, kind: "retreat", label: "Retreat", disabled: true },
    ],
  },
};
