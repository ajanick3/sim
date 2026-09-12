import type { Meta, StoryObj } from "@storybook/react";
import Stack from "@mui/material/Stack";
import { Card, type CardSize } from "./Card";
import pikachuEx from "../assets/pikachu-ex.png";

const meta = {
  title: "primitives/Card",
  component: Card,
  args: {
    name: "Pikachu ex",
    src: pikachuEx,
  },
} satisfies Meta<typeof Card>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Active: Story = { args: { size: "active" } };
export const ActiveFar: Story = { args: { size: "activeFar" } };
export const Bench: Story = { args: { size: "bench" } };
export const Hand: Story = { args: { size: "hand" } };
export const Pile: Story = { args: { size: "pile" } };
export const Picker: Story = { args: { size: "picker" } };
export const Stadium: Story = { args: { size: "stadium" } };
export const Selected: Story = { args: { size: "hand", selected: true } };
export const Tilted: Story = { args: { size: "hand", tilt: 18 } };
export const NoArt: Story = { args: { size: "hand", src: null, name: "Ultra Ball" } };

/** Every size side by side, at a glance. */
export const AllSizes: Story = {
  args: { size: "active" },
  render: () => (
    <Stack direction="row" spacing={2} alignItems="flex-end">
      {(["pile", "stadium", "activeFar", "active", "bench", "hand", "picker"] as CardSize[]).map(
        (size) => (
          <Stack key={size} spacing={0.5} alignItems="center">
            <Card size={size} name="Pikachu ex" src={pikachuEx} />
            <code style={{ fontSize: 11 }}>{size}</code>
          </Stack>
        ),
      )}
    </Stack>
  ),
};

/** A held hand — every card leaning back by the same angle, the read
 *  of looking across a table from a seated chair, not a flat fan spin.
 *  The "physics" this library carries for now: a fixed 3D lean, no
 *  drag simulation. */
export const HeldHand: Story = {
  args: { size: "hand" },
  render: () => (
    <Stack direction="row" spacing={-2}>
      {[0, 1, 2, 3, 4].map((i) => (
        <Card key={i} size="hand" name="Pikachu ex" src={pikachuEx} tilt={10} />
      ))}
    </Stack>
  ),
};
