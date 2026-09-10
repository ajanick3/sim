import type { Meta, StoryObj } from "@storybook/nextjs";
import { StadiumSlot } from "./StadiumSlot";

const meta = {
  title: "live/StadiumSlot",
  component: StadiumSlot,
  args: { ghost: false },
} satisfies Meta<typeof StadiumSlot>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Empty: Story = {};
export const Ghost: Story = {
  args: { ghost: true },
  parameters: { docs: { description: { story: "Holds the space but hidden." } } },
};
