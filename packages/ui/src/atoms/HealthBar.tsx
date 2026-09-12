import Box from "@mui/material/Box";

/** The HP pill a card prints top-right, overlaid on a `Card`. Not a
 *  bar despite the name a health indicator usually gets — this game
 *  shows remaining HP as a number, the way the physical card does,
 *  never a depleting bar. Named for what a reader expects to find,
 *  documented here so the name doesn't have to guess twice. */
export function HealthBar({ hp, tiny = false }: { hp: number; tiny?: boolean }) {
  return (
    <Box
      sx={{
        position: "absolute",
        top: 2,
        right: 2,
        bgcolor: "rgba(0,0,0,0.75)",
        color: "common.white",
        fontWeight: 700,
        fontSize: tiny ? 8 : 10,
        borderRadius: 0.5,
        px: 0.5,
        lineHeight: 1.4,
      }}
    >
      {hp}
    </Box>
  );
}
