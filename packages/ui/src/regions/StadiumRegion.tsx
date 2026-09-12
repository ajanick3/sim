import Box from "@mui/material/Box";
import { Card } from "../primitives/Card";
import type { PocketCard } from "./types";

/** The Stadium — one card shared by both players, or an empty "+"
 *  slot. Unlike everything else on the board it belongs to neither
 *  side, so it never scales with the near/far read the two players'
 *  own regions carry. */
export function StadiumRegion({ card }: { card: PocketCard | null }) {
  if (!card) {
    return (
      <Box
        sx={{
          width: 72,
          height: 100,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          gap: 0.5,
          borderRadius: 1.5,
          border: "1px dashed",
          borderColor: "text.disabled",
          color: "text.disabled",
          fontSize: 11,
        }}
      >
        <span style={{ fontSize: 18, lineHeight: 1 }}>+</span>
        Stadium
      </Box>
    );
  }
  return <Card size="stadium" src={card.src} name={card.name} />;
}
