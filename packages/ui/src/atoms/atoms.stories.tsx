import type { Meta, StoryObj } from "@storybook/react";
import Stack from "@mui/material/Stack";
import { Card } from "../primitives/Card";
import { HealthBar } from "./HealthBar";
import { DamageCounter } from "./DamageCounter";
import { EnergyChip } from "./EnergyChip";
import { ToolBadge } from "./ToolBadge";
import { ActiveIndicator } from "./ActiveIndicator";
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
