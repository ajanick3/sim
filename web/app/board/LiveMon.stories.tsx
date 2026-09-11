import type { Meta, StoryObj } from "@storybook/nextjs";
import { fn } from "storybook/test";
import { attacker, defender, noArt, pokemon, swatchArt } from "./fixtures";
import { LiveMon } from "./LiveMon";

const meta = {
  title: "live/LiveMon",
  component: LiveMon,
  args: { mon: attacker(), art: swatchArt, onSelect: fn() },
} satisfies Meta<typeof LiveMon>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Active: Story = { args: { active: true } };
export const ActiveNoArt: Story = { args: { active: true, art: noArt } };
export const Bench: Story = { args: { mon: pokemon({ name: "Dreepy", hp: 70 }) } };
export const BenchSmall: Story = {
  args: { mon: pokemon({ name: "Dreepy", hp: 70 }) },
};
export const Damaged: Story = { args: { active: true, mon: defender() } };
export const WithEnergyAndCondition: Story = {
  args: {
    active: true,
    mon: pokemon({
      name: "Charizard ex",
      hp: 330,
      damage: 60,
      conditions: ["Burned"],
      attached: [
        {
          id: 1,
          name: "Fire Energy",
          def: 0,
          print_id: "e1",
          energy_type: "Fire",
          category: "energy",
        },
        {
          id: 2,
          name: "Fire Energy",
          def: 0,
          print_id: "e2",
          energy_type: "Fire",
          category: "energy",
        },
      ],
    }),
  },
};
export const Selectable: Story = { args: { active: true, selectable: true } };
export const Selected: Story = { args: { active: true, selectable: true, selected: true } };
export const DropTarget: Story = { args: { selectable: true, dropTarget: true } };
export const DropTargetHovered: Story = {
  args: { selectable: true, dropTarget: true, hovered: true },
};
export const EmptyActive: Story = { args: { mon: null, active: true } };
export const EmptyPlaceHere: Story = { args: { mon: null, placeHere: fn() } };
