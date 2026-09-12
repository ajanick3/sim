import Box from "@mui/material/Box";

/** A Tool attached to a Pokémon — one card carries at most one, so
 *  this is a single small badge, not a list. */
export function ToolBadge({ name }: { name: string }) {
  return (
    <Box
      title={name}
      sx={{
        position: "absolute",
        bottom: 2,
        left: 2,
        display: "grid",
        placeItems: "center",
        width: 16,
        height: 16,
        borderRadius: "50%",
        bgcolor: "rgba(0,0,0,0.6)",
        fontSize: 10,
        lineHeight: 1,
      }}
    >
      🔧
    </Box>
  );
}
