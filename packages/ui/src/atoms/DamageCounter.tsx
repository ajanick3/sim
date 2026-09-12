import Box from "@mui/material/Box";

/** A damage counter dropped on the illustration — a coin, per the
 *  physical game. `top`/`left` are percentages of the card, fixed per
 *  Pokémon by the caller so the spot doesn't jump between renders. */
export function DamageCounter({
  damage,
  top,
  left,
  large = false,
}: {
  damage: number;
  top: string;
  left: string;
  large?: boolean;
}) {
  const size = large ? 44 : 24;
  return (
    <Box
      sx={{
        position: "absolute",
        top,
        left,
        transform: "translate(-50%, -50%)",
        width: size,
        height: size,
        display: "grid",
        placeItems: "center",
        borderRadius: "50%",
        border: "2px solid rgba(0,0,0,0.5)",
        bgcolor: "warning.main",
        color: "common.black",
        fontWeight: 900,
        fontSize: large ? 16 : 10,
        boxShadow: "0 2px 5px rgba(0,0,0,0.6)",
      }}
    >
      {damage}
    </Box>
  );
}
