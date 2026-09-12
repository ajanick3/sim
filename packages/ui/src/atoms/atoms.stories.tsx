import type { Meta, StoryObj } from "@storybook/react";
import Stack from "@mui/material/Stack";
import { Card } from "../primitives/Card";
import { CardOverlay } from "./CardOverlay";
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

/** Every overlay atom, on a real Card via `CardOverlay`, so they're
 *  seen the way a player actually sees them — laid over art with
 *  plain flexbox, not floating alone or positioned by coordinates. */
export const OnACard: Story = {
  render: () => (
    <Stack direction="row" spacing={4} alignItems="flex-start">
      <Card size="bench" name="Pikachu ex" src={pikachuEx}>
        <CardOverlay
          top={<HealthBar hp={200} tiny />}
          middle={<DamageCounter damage={60} />}
          bottomStart={<ToolBadge name="Rescue Board" />}
          bottomEnd={<EnergyChip kind="Lightning" size={10} />}
        />
      </Card>
      <ActiveIndicator>
        <Card size="active" name="Pikachu ex" src={pikachuEx}>
          <CardOverlay top={<HealthBar hp={200} />} middle={<DamageCounter damage={120} large />} />
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
