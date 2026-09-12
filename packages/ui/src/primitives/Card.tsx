import Paper from "@mui/material/Paper";
import type { ReactNode } from "react";
import { CardImage } from "./CardImage";

/** Every size a `Card` renders at, named rather than spelled out in
 *  pixels at each call site — every one the whole card, roughly 5:7
 *  portrait. Active used to show only the top-of-print sliver; it
 *  reads full now, the same treatment Bench and Hand already carry,
 *  just larger — the dominant card the reference layout gives it. */
export type CardSize = "active" | "activeFar" | "bench" | "hand" | "pile" | "picker" | "stadium";

const SIZE_PX: Record<CardSize, { width: number; height: number; crop: "top" | "full" }> = {
  active: { width: 150, height: 210, crop: "full" },
  activeFar: { width: 112, height: 157, crop: "full" },
  bench: { width: 64, height: 90, crop: "full" },
  hand: { width: 110, height: 154, crop: "full" },
  pile: { width: 46, height: 64, crop: "full" },
  picker: { width: 132, height: 184, crop: "full" },
  stadium: { width: 72, height: 100, crop: "full" },
};

// A real card's corner curvature is proportional to its size, not a
// fixed radius — matched here as one percentage every Card size (and
// every fluid, squished-to-fit width) scales against consistently.
const CARD_RADIUS_PCT = 5;

export type CardProps = {
  size: CardSize;
  /** TCGdex art, or null to show the plain-name fallback. */
  src?: string | null;
  name: string;
  /** Degrees the card leans back, away from the viewer, top edge
   *  receding — the read of a card lying on a table seen from a
   *  seated chair, not a flat spin. 0 for a card viewed straight on. */
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
 *  `tilt` is the only "physics" a card carries for now — a fixed 3D
 *  lean, not a spring or a drag simulation. */
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
        // A percentage, not a fixed spacing unit: a fixed radius reads
        // chunky on a 46px pile and barely-there on a 150px Active —
        // scaling with the box's own rendered size (CSS computes a
        // percentage border-radius from each dimension) keeps every
        // size, fixed or fluid, looking like the same card.
        borderRadius: `${CARD_RADIUS_PCT}%`,
        // A perspective lean, not a flat spin: the top edge recedes as
        // if the card were lying on a table and the viewer were seated
        // in front of it, not looking straight down at it.
        transform: tilt ? `perspective(600px) rotateX(${tilt}deg)` : undefined,
        transformStyle: "preserve-3d",
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
