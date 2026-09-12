import Box from "@mui/material/Box";
import { Card } from "../primitives/Card";
import { CardOverlay } from "../atoms/CardOverlay";
import { HealthBar } from "../atoms/HealthBar";
import { DamageCounter } from "../atoms/DamageCounter";
import { EnergyChip } from "../atoms/EnergyChip";
import { ToolBadge } from "../atoms/ToolBadge";
import { attachedParts, type PlayCard } from "./types";

const BENCH_LIMIT = 5;
const NEAR_WIDTH = 320;
const FAR_WIDTH = 240;

/** The Bench — at least five slots; more than five (a raised limit)
 *  squishes the row narrower instead of wrapping it. `far` scales the
 *  whole row down, the way the opponent's side of the board always
 *  reads farther away. `maxWidth` overrides the near/far default —
 *  the caller's own court may be narrower than either. */
export function BenchRegion({
  mons,
  far = false,
  maxWidth,
  onSelect,
}: {
  mons: (PlayCard | null)[];
  far?: boolean;
  maxWidth?: number;
  onSelect?: (id: number) => void;
}) {
  const cols = Math.max(BENCH_LIMIT, mons.length);
  const slots = [...mons, ...Array(Math.max(0, cols - mons.length)).fill(null)];
  return (
    <Box
      sx={{
        display: "grid",
        gridTemplateColumns: `repeat(${cols}, 1fr)`,
        gap: 0.75,
        width: "100%",
        maxWidth: maxWidth ?? (far ? FAR_WIDTH : NEAR_WIDTH),
        mx: "auto",
      }}
    >
      {slots.map((mon, i) => {
        const { energies, tool } = mon ? attachedParts(mon) : { energies: [], tool: null };
        return (
          <Card
            key={mon?.id ?? `empty-${i}`}
            size="bench"
            fluid
            src={mon?.src}
            name={mon?.name ?? "empty"}
            onClick={mon && onSelect ? () => onSelect(mon.id) : undefined}
          >
            {mon && (
              <CardOverlay
                top={<HealthBar hp={mon.hp} tiny />}
                middle={mon.damage > 0 && <DamageCounter damage={mon.damage} />}
                bottomStart={tool && <ToolBadge name={tool.name} />}
                bottomEnd={energies.map((e) => (
                  <EnergyChip key={e.id} kind={e.energyType!} size={10} />
                ))}
              />
            )}
          </Card>
        );
      })}
    </Box>
  );
}
