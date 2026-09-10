import type { Meta, StoryObj } from "@storybook/nextjs";
import { card, noArt, swatchArt } from "./fixtures";
import { StadiumSlot } from "./StadiumSlot";

const meta = {
  title: "board/StadiumSlot",
  component: StadiumSlot,
  args: { ghost: false },
} satisfies Meta<typeof StadiumSlot>;

export default meta;
type Story = StoryObj<typeof meta>;

const stadium = card({ name: "Area Zero Underdepths", category: "stadium" });

export const Empty: Story = {};
export const Ghost: Story = {
  args: { ghost: true },
  parameters: { docs: { description: { story: "Holds the space but hidden." } } },
};
export const InPlay: Story = { args: { card: stadium, art: swatchArt } };
export const InPlayNoArt: Story = { args: { card: stadium, art: noArt } };
