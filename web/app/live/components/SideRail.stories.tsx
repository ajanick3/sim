import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { SideRail } from "./SideRail";

const meta = {
  title: "live/SideRail",
  component: SideRail,
  args: {
    myPrizes: 4,
    oppPrizes: 5,
    turn: 4,
    yourTurn: true,
    canEndTurn: true,
    onEndTurn: fn(),
    onLog: fn(),
    onHide: fn(),
  },
} satisfies Meta<typeof SideRail>;

export default meta;
type Story = StoryObj<typeof meta>;

export const YourTurn: Story = {};
export const Waiting: Story = { args: { yourTurn: false, canEndTurn: false } };
export const MatchPoint: Story = { args: { myPrizes: 1, oppPrizes: 1 } };
