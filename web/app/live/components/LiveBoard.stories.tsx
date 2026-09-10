import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import type { WireView } from "../../view";
import { defender, pokemon, sampleView, side, swatchArt } from "./fixtures";
import { LiveBoard } from "./LiveBoard";

const meta = {
  title: "live/LiveBoard",
  component: LiveBoard,
  parameters: { layout: "fullscreen" },
  args: {
    view: sampleView,
    actions: ["End turn"],
    meta: [{ kind: "EndTurn", card: null, target: null }],
    selection: null,
    onSelect: fn(),
    art: swatchArt,
    seat: 0,
    busy: false,
    onAct: fn(),
    log: ["🥇 One drew 7.", "🥇 One benched Dreepy.", "🥈 Two took a Prize."],
  },
} satisfies Meta<typeof LiveBoard>;

export default meta;
type Story = StoryObj<typeof meta>;

export const MidGame: Story = {};

export const CoinFlip: Story = {
  args: {
    actions: ["One takes the first turn", "Two takes the first turn"],
    meta: [
      { kind: "ChooseWhoGoesFirst", card: null, target: null },
      { kind: "ChooseWhoGoesFirst", card: null, target: null },
    ],
  },
};

export const AbilityPrompt: Story = {
  args: {
    actions: ["Use Psychic Draw", "Decline Psychic Draw"],
    meta: [
      { kind: "AcceptPsychicDraw", card: null, target: null },
      { kind: "DeclinePsychicDraw", card: null, target: null },
    ],
  },
};

export const PromoteAfterKnockout: Story = {
  args: {
    view: {
      ...sampleView,
      sides: [
        side({
          player: 0,
          active: null,
          bench: [pokemon({ name: "Dreepy" }), pokemon({ name: "Drakloak" })],
        }),
        sampleView.sides[1],
      ],
    } as WireView,
    actions: ["Promote Dreepy", "Promote Drakloak"],
    meta: [
      { kind: "Promote", card: null, target: 0 },
      { kind: "Promote", card: null, target: 0 },
    ],
  },
};

export const RailHidden: Story = {
  args: {},
  play: async ({ canvas, userEvent }) => {
    await userEvent.click(await canvas.findByLabelText("Hide controls"));
  },
};

export const OpponentActiveAsleep: Story = {
  args: {
    view: {
      ...sampleView,
      sides: [sampleView.sides[0], side({ player: 1, active: defender() })],
    } as WireView,
  },
};
