import type { Meta, StoryObj } from "@storybook/react";
import Stack from "@mui/material/Stack";
import { Card } from "../primitives/Card";
import { HealthBar } from "./HealthBar";
import { DamageCounter } from "./DamageCounter";
import { EnergyChip } from "./EnergyChip";
import { ToolBadge } from "./ToolBadge";
import { ActiveIndicator } from "./ActiveIndicator";

const meta = { title: "atoms/overview" } satisfies Meta;
export default meta;
type Story = StoryObj<typeof meta>;

/** Every overlay atom, on a real Card, so they're seen the way a
 *  player actually sees them — laid over art, not floating alone. */
export const OnACard: Story = {
  render: () => (
    <Stack direction="row" spacing={4} alignItems="flex-start">
      <Card size="bench" name="Munkidori">
        <HealthBar hp={110} tiny />
        <DamageCounter damage={30} top="40%" left="50%" />
        <div style={{ position: "absolute", bottom: 2, right: 2, display: "flex", gap: 2 }}>
          <EnergyChip kind="Psychic" size={10} />
        </div>
        <ToolBadge name="Rescue Board" />
      </Card>
      <ActiveIndicator>
        <Card size="active" name="Dragapult ex">
          <HealthBar hp={330} />
          <DamageCounter damage={90} top="35%" left="60%" large />
        </Card>
      </ActiveIndicator>
    </Stack>
  ),
};
