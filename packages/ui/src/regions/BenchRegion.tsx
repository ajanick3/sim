import Box from "@mui/material/Box";
import { Card } from "../primitives/Card";
import { HealthBar } from "../atoms/HealthBar";
import { DamageCounter } from "../atoms/DamageCounter";
import type { PlayCard } from "./types";

const BENCH_LIMIT = 5;

/** The Bench — at least five slots; more than five (a raised limit)
 *  squishes the row narrower instead of wrapping it. `far` scales the
 *  whole row down, the way the opponent's side of the board always
 *  reads farther away. */
export function BenchRegion({
  mons,
  far = false,
  onSelect,
}: {
  mons: (PlayCard | null)[];
  far?: boolean;
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
        width: far ? 240 : 320,
        mx: "auto",
      }}
    >
      {slots.map((mon, i) => (
        <Card
          key={mon?.id ?? `empty-${i}`}
          size="bench"
          fluid
          src={mon?.src}
          name={mon?.name ?? "empty"}
          onClick={mon && onSelect ? () => onSelect(mon.id) : undefined}
        >
          {mon && (
            <>
              <HealthBar hp={mon.hp} tiny />
              {mon.damage > 0 && <DamageCounter damage={mon.damage} top="40%" left="50%" />}
            </>
          )}
        </Card>
      ))}
    </Box>
  );
}
