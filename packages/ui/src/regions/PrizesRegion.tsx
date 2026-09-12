import Box from "@mui/material/Box";
import { PokeBall } from "../atoms/PokeBall";

/** The Prize stack — six Poké Balls, one dimmed each time a Knockout
 *  pays a Prize out. Never shows the cards themselves; only the count
 *  remaining is public. */
export function PrizesRegion({ remaining }: { remaining: number }) {
  return (
    <Box sx={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: 0.5, width: 32 }}>
      {Array.from({ length: 6 }, (_, i) => (
        <PokeBall key={i} size={14} taken={i >= remaining} />
      ))}
    </Box>
  );
}
