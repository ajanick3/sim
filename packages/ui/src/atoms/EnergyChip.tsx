import Box from "@mui/material/Box";

export type EnergyKind =
  | "Colorless"
  | "Fire"
  | "Water"
  | "Grass"
  | "Lightning"
  | "Psychic"
  | "Fighting"
  | "Darkness"
  | "Metal"
  | "Dragon"
  | "Fairy";

const ENERGY_COLOR: Record<EnergyKind, string> = {
  Colorless: "#c9c9c9",
  Fire: "#f0a24b",
  Water: "#6ea8fe",
  Grass: "#7bd88f",
  Lightning: "#f0d84b",
  Psychic: "#c98bf0",
  Fighting: "#c2703d",
  Darkness: "#5a5a6e",
  Metal: "#9aa2b1",
  Dragon: "#b08a4b",
  Fairy: "#f0a3cf",
};

/** One attached Energy, drawn as its type's colour rather than an
 *  icon set the library doesn't have yet. Small enough to sit several
 *  in a row on a `Card`'s corner. */
export function EnergyChip({ kind, size = 14 }: { kind: EnergyKind; size?: number }) {
  return (
    <Box
      title={`${kind} Energy`}
      sx={{
        width: size,
        height: size,
        borderRadius: "50%",
        bgcolor: ENERGY_COLOR[kind],
        border: "1px solid rgba(0,0,0,0.5)",
      }}
    />
  );
}
