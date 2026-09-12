import Box from "@mui/material/Box";

/** The Prize stack — six pips face down, one taken (and gone) each
 *  time a Knockout pays out. Never shows the cards; only the count
 *  remaining is public. */
export function PrizesRegion({ remaining, color = "error" }: { remaining: number; color?: "error" | "primary" }) {
  return (
    <Box sx={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: 0.5, width: 32 }}>
      {Array.from({ length: 6 }, (_, i) => (
        <Box
          key={i}
          sx={{
            width: 14,
            height: 20,
            borderRadius: 0.5,
            border: "1px solid",
            borderColor: `${color}.main`,
            bgcolor: i < remaining ? `${color}.dark` : "transparent",
          }}
        />
      ))}
    </Box>
  );
}
