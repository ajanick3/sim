import Box from "@mui/material/Box";

/** `WirePokemon.conditions`, laid over the card's middle the way the
 *  live board already does — Poisoned, Asleep, Paralyzed, and the
 *  rest, joined by a comma when more than one applies at once. */
export function SpecialConditions({ conditions }: { conditions: string[] }) {
  if (conditions.length === 0) return null;
  return (
    <Box
      sx={{
        position: "absolute",
        insetInline: 0,
        top: "50%",
        bgcolor: "rgba(0,0,0,0.6)",
        color: "warning.main",
        fontSize: 8,
        textAlign: "center",
      }}
    >
      {conditions.join(", ")}
    </Box>
  );
}
