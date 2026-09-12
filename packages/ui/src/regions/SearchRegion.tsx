import Box from "@mui/material/Box";
import { Card } from "../primitives/Card";
import type { PocketCard } from "./types";

/** A deck search — every card in the pile that matches, laid out to
 *  pick from. Cards the current search can't take are dimmed rather
 *  than hidden, so the player still sees the whole deck. */
export function SearchRegion({
  cards,
  takeable,
  onTake,
}: {
  cards: PocketCard[];
  /** ids the current search actually lets the player take. */
  takeable: Set<number>;
  onTake: (id: number) => void;
}) {
  return (
    <Box sx={{ display: "grid", gridTemplateColumns: "repeat(5, 1fr)", gap: 1 }}>
      {cards.map((c) => {
        const can = takeable.has(c.id);
        return (
          <Box key={c.id} sx={{ opacity: can ? 1 : 0.35 }}>
            <Card size="picker" fluid src={c.src} name={c.name} onClick={can ? () => onTake(c.id) : undefined} />
          </Box>
        );
      })}
    </Box>
  );
}
