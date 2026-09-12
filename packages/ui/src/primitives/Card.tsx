import Paper from "@mui/material/Paper";
import type { ReactNode } from "react";
import { CardImage } from "./CardImage";

/** Every size a `Card` renders at, named rather than spelled out in
 *  pixels at each call site. `active`/`benchFar` are the top-of-print
 *  sliver a Pokémon in play has always shown; the rest are the whole
 *  card, roughly 5:7 portrait. */
export type CardSize = "active" | "activeFar" | "bench" | "hand" | "pile" | "picker" | "stadium";

const SIZE_PX: Record<CardSize, { width: number; height: number; crop: "top" | "full" }> = {
  active: { width: 168, height: 99, crop: "top" },
  activeFar: { width: 126, height: 74, crop: "top" },
  bench: { width: 64, height: 90, crop: "full" },
  hand: { width: 110, height: 154, crop: "full" },
  pile: { width: 46, height: 64, crop: "full" },
  picker: { width: 132, height: 184, crop: "full" },
  stadium: { width: 72, height: 100, crop: "full" },
};

export type CardProps = {
  size: CardSize;
  /** TCGdex art, or null to show the plain-name fallback. */
  src?: string | null;
  name: string;
  /** Degrees of static rotation — the fanned-hand look. 0 for anything
   *  in play; a small alternating value per card for a held hand. */
  tilt?: number;
  /** Fill the parent's width instead of `size`'s fixed pixels, at
   *  `size`'s own aspect ratio — how the Bench squishes to fit more
   *  than five columns instead of wrapping. */
  fluid?: boolean;
  /** Lift and ring a card that's the current selection. */
  selected?: boolean;
  onClick?: () => void;
  /** HP pill, damage counter, Energy chips, Tool badge — anything laid
   *  over the art, positioned absolutely by the caller. */
  children?: ReactNode;
};

/** The one card frame this library draws — Active, Bench, Hand, a
 *  deck/discard pile, a search picker, the Stadium. `size` picks the
 *  box and whether the art shows cropped from the top or in full;
 *  `tilt` is the only "physics" a card carries for now — a fixed
 *  rotation, not a spring or a drag simulation. */
export function Card({
  size,
  src = null,
  name,
  tilt = 0,
  fluid = false,
  selected = false,
  onClick,
  children,
}: CardProps) {
  const { width, height, crop } = SIZE_PX[size];
  return (
    <Paper
      component={onClick ? "button" : "div"}
      onClick={onClick}
      elevation={selected ? 8 : 2}
      sx={{
        position: "relative",
        width: fluid ? "100%" : width,
        height: fluid ? "auto" : height,
        aspectRatio: fluid ? `${width} / ${height}` : undefined,
        flex: "none",
        overflow: "hidden",
        borderRadius: 1.5,
        transform: tilt ? `rotate(${tilt}deg)` : undefined,
        transformOrigin: "center bottom",
        outline: selected ? "2px solid" : "none",
        outlineColor: "primary.main",
        cursor: onClick ? "pointer" : "default",
        border: "none",
        p: 0,
        bgcolor: "background.paper",
        textAlign: "left",
        font: "inherit",
        color: "inherit",
      }}
    >
      {src ? <CardImage src={src} alt={name} crop={crop} /> : <CardFallback name={name} />}
      {children}
    </Paper>
  );
}

// A separate, tiny function (not exported) so `Card` itself stays
// readable — the no-art fallback is a rare path, not the main shape.
function CardFallback({ name }: { name: string }) {
  return (
    <div
      style={{
        position: "absolute",
        inset: 0,
        display: "flex",
        alignItems: "center",
        padding: 4,
        fontSize: 10,
        fontWeight: 600,
        lineHeight: 1.2,
      }}
    >
      {name}
    </div>
  );
}
