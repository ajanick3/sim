import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { CoinFlip } from "./CoinFlip";

const meta = {
  title: "board/CoinFlip",
  component: CoinFlip,
  parameters: { layout: "fullscreen" },
  args: { results: ["heads"], onDone: fn(), persist: true },
} satisfies Meta<typeof CoinFlip>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Heads: Story = {};
export const Tails: Story = { args: { results: ["tails"] } };
export const FourCoins: Story = { args: { results: ["heads", "tails", "heads", "heads"] } };
export const AllTails: Story = { args: { results: ["tails", "tails", "tails"] } };
