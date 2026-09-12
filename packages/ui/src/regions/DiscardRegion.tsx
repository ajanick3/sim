import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import { Card } from "../primitives/Card";
import type { PocketCard } from "./types";

/** The discard pile — its top card's art if it holds any, a count
 *  underneath either way. Tapping it (when it holds anything) is how
 *  a player opens the `DiscardViewDialog`. */
export function DiscardRegion({ cards, onOpen }: { cards: PocketCard[]; onOpen?: () => void }) {
  const top = cards[cards.length - 1];
  return (
    <Stack spacing={0.5} alignItems="center">
      <Card size="pile" src={top?.src} name={top?.name ?? "discard"} onClick={cards.length ? onOpen : undefined} />
      <Typography variant="caption" color="text.secondary">
        {cards.length === 0 ? "discard 0" : cards.length}
      </Typography>
    </Stack>
  );
}
