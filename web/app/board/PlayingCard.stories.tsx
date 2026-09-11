import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { swatchArt } from "./fixtures";
import { PlayingCard } from "./PlayingCard";

const meta = {
  title: "board/PlayingCard",
  component: PlayingCard,
  args: {
    size: "picker",
    src: swatchArt("munkidori"),
    name: "Munkidori",
    energyType: null,
    onClick: fn(),
  },
} satisfies Meta<typeof PlayingCard>;

export default meta;
type Story = StoryObj<typeof meta>;

export const TopCrop: Story = {};
export const FullCrop: Story = { args: { size: "pile", crop: "full", src: swatchArt("pile") } };
export const NoArtEnergy: Story = {
  args: { src: null, name: "Psychic Energy", energyType: "Psychic" },
};
export const NoArtName: Story = { args: { src: null, name: "Professor's Research" } };
export const Selected: Story = { args: { selected: true } };
export const Raised: Story = { args: { raised: true } };
export const DropTarget: Story = { args: { dropTarget: true } };
export const Dimmed: Story = { args: { dimmed: true } };
export const WithOverlay: Story = {
  args: {
    size: "active",
    src: swatchArt("dragapult"),
    name: "Dragapult ex",
    children: (
      <span className="absolute left-0.5 top-0.5 z-10 rounded bg-black/75 px-1 text-[9px] font-bold">
        320
      </span>
    ),
  },
};
