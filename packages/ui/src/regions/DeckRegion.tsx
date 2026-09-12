import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import { Card } from "../primitives/Card";

/** The deck — a face-down pile with a count. Never shows a card face;
 *  the count is the only public fact about it. */
export function DeckRegion({ count }: { count: number }) {
  return (
    <Stack spacing={0.5} alignItems="center">
      <Card size="pile" src={null} name="Deck" />
      <Typography variant="caption" color="text.secondary">
        {count}
      </Typography>
    </Stack>
  );
}
