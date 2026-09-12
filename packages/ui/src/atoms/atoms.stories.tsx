import type { Meta, StoryObj } from "@storybook/react";
import Stack from "@mui/material/Stack";
import { Card } from "../primitives/Card";
import { HealthBar } from "./HealthBar";
import { DamageCounter } from "./DamageCounter";
import { EnergyChip } from "./EnergyChip";
import { ToolBadge } from "./ToolBadge";
import { ActiveIndicator } from "./ActiveIndicator";
import { PokeBall } from "./PokeBall";
import pikachuEx from "../assets/pikachu-ex.png";

const meta = { title: "atoms/overview" } satisfies Meta;
export default meta;
type Story = StoryObj<typeof meta>;

/** Every overlay atom, on a real Card, so they're seen the way a
 *  player actually sees them — laid over art, not floating alone. */
export const OnACard: Story = {
  render: () => (
    <Stack direction="row" spacing={4} alignItems="flex-start">
      <Card size="bench" name="Pikachu ex" src={pikachuEx}>
        <HealthBar hp={200} tiny />
        <DamageCounter damage={60} top="40%" left="50%" />
        <div style={{ position: "absolute", bottom: 2, right: 2, display: "flex", gap: 2 }}>
          <EnergyChip kind="Lightning" size={10} />
        </div>
        <ToolBadge name="Rescue Board" />
      </Card>
      <ActiveIndicator>
        <Card size="active" name="Pikachu ex" src={pikachuEx}>
          <HealthBar hp={200} />
          <DamageCounter damage={120} top="35%" left="60%" large />
        </Card>
      </ActiveIndicator>
    </Stack>
  ),
};

/** Six Prizes, one taken with each — the dimmed, desaturated one is
 *  already paid out. */
export const PrizeStack: Story = {
  render: () => (
    <div style={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: 6, width: 44 }}>
      {[0, 1, 2, 3, 4, 5].map((i) => (
        <PokeBall key={i} taken={i >= 4} />
      ))}
    </div>
  ),
};
