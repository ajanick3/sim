import Box from "@mui/material/Box";
import type { EnergyType } from "../regions/types";

// Re-exported so a caller need not reach into `regions/types` just to
// name an Energy type — `EnergyType` there is the source of truth
// (the engine's actual ten, no "Fairy": this game has none).
export type { EnergyType };

const ENERGY_COLOR: Record<EnergyType, string> = {
  Grass: "#7bd88f",
  Fire: "#f0a24b",
  Water: "#6ea8fe",
  Lightning: "#f0d84b",
  Psychic: "#c98bf0",
  Fighting: "#c2703d",
  Darkness: "#5a5a6e",
  Metal: "#9aa2b1",
  Dragon: "#b08a4b",
  Colorless: "#c9c9c9",
};

/** One attached Energy, drawn as its type's colour rather than an
 *  icon set the library doesn't have yet. Small enough to sit several
 *  in a row on a `Card`'s corner. */
export function EnergyChip({ kind, size = 14 }: { kind: EnergyType; size?: number }) {
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
