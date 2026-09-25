import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { ActionDialog } from "./ActionDialog";

const meta = {
  component: ActionDialog,
  args: {
    title: "Choose an action",
    description: "Dreepy is selected.",
    onChoose: fn(),
    actions: [
      { id: 1, label: "Bite", detail: "Attack for 40 damage" },
      { id: 2, label: "Retreat", detail: "Choose a Benched Pokémon" },
    ],
  },
} satisfies Meta<typeof ActionDialog>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Default: Story = {};
