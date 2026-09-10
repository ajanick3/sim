import type { Meta, StoryObj } from "@storybook/nextjs";
import { CardFace } from "./CardFace";
import { swatchArt } from "./fixtures";

// CardFace fills its positioned parent, so every story frames it in a
// card-shaped box.
const meta = {
  title: "live/CardFace",
  component: CardFace,
  decorators: [
    (Story) => (
      <span className="relative block h-[168px] w-[120px] overflow-hidden rounded-[7px] border border-black/10 bg-white">
        <Story />
      </span>
    ),
  ],
  args: { src: null, name: "Ultra Ball", energyType: null },
} satisfies Meta<typeof CardFace>;

export default meta;
type Story = StoryObj<typeof meta>;

export const RealArt: Story = { args: { src: swatchArt("ultra-ball") } };
export const NameFallback: Story = { args: { src: null, name: "Professor's Research" } };
export const BasicEnergy: Story = {
  args: { src: null, name: "Psychic Energy", energyType: "Psychic" },
};
export const FireEnergy: Story = {
  args: { src: null, name: "Fire Energy", energyType: "Fire" },
};
