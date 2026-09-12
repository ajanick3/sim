import Box from "@mui/material/Box";
import { Card } from "../primitives/Card";
import type { PocketCard } from "./types";

// Every held card leans back by the same amount — a hand of cards
// held toward the viewer, not fanned open across a table, so a single
// shared angle is right where the old per-card alternating spin
// wasn't. See `Card`'s own `tilt` doc for what the angle now means.
const HAND_TILT_DEG = 10;

/** The Hand — a tilted, overlapping row rather than the live app's
 *  five-column grid. `count` alone (no cards) draws the opponent's
 *  Hand: a face-down pile with a number, never the cards themselves. */
export function HandRegion({
  cards,
  count,
  onSelect,
}: {
  cards?: PocketCard[];
  count?: number;
  onSelect?: (id: number) => void;
}) {
  if (cards === undefined) {
    return (
      <Box sx={{ fontSize: 12, color: "text.secondary" }}>
        Hand · {count ?? 0}
      </Box>
    );
  }
  return (
    <Box sx={{ display: "flex", pl: 3 }}>
      {cards.map((c, i) => (
        <Box key={c.id} sx={{ ml: i === 0 ? 0 : -3 }}>
          <Card
            size="hand"
            src={c.src}
            name={c.name}
            tilt={HAND_TILT_DEG}
            onClick={onSelect ? () => onSelect(c.id) : undefined}
          />
        </Box>
      ))}
    </Box>
  );
}
