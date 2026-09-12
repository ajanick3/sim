import Box from "@mui/material/Box";
import type { ReactNode } from "react";

/** The ring around whichever Pokémon is the Active — a border, not a
 *  badge, so it reads as "this card's spot" rather than a stamp on
 *  the card itself. Wrap the `Card` in this rather than adding a prop
 *  to `Card` for it: the Active-ness is a fact about the spot, not the
 *  card. */
export function ActiveIndicator({ children }: { children: ReactNode }) {
  return (
    <Box
      sx={{
        display: "inline-flex",
        borderRadius: 2,
        boxShadow: (t) => `0 0 0 2px ${t.palette.primary.main}`,
      }}
    >
      {children}
    </Box>
  );
}
