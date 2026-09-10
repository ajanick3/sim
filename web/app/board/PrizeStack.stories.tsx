import type { Meta, StoryObj } from "@storybook/nextjs";
import { PrizeStack } from "./PrizeStack";

const meta = {
  title: "live/PrizeStack",
  component: PrizeStack,
  args: { count: 4 },
  argTypes: { count: { control: { type: "range", min: 0, max: 6 } } },
} satisfies Meta<typeof PrizeStack>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Full: Story = { args: { count: 6 } };
export const Midway: Story = { args: { count: 3 } };
export const OneLeft: Story = { args: { count: 1 } };
export const Taken: Story = { args: { count: 0 } };
