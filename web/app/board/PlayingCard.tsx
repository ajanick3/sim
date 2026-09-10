"use client";

import type { ComponentPropsWithoutRef, ReactNode } from "react";
import { CardFace } from "./CardFace";
import { CARD_SIZE, type CardSize } from "./sizes";

/** The physical card frame the board reuses: white stock, the one card
 *  radius, the warm table shadow, and a fill that is the print's art
 *  (cropped from the top, or whole) or — when there is no art — the
 *  drawn Energy face or the card name. Movement and phase animation come
 *  from the caller via `className`; overlays come as `children`. */
type PlayingCardProps = {
  size: CardSize;
  /** "top" shows the name bar and head of the illustration; "full" the whole card. */
  crop?: "top" | "full";
  src: string | null;
  name: string;
  energyType?: string | null;
  selected?: boolean;
  dropTarget?: boolean;
  dimmed?: boolean;
  /** Sit at the raised shadow — a card that has lifted off the table. */
  raised?: boolean;
  /** Add the hover lift of the shadow. */
  interactive?: boolean;
  className?: string;
  children?: ReactNode;
} & Omit<ComponentPropsWithoutRef<"button">, "className" | "children">;

export function PlayingCard({
  size,
  crop = "top",
  src,
  name,
  energyType = null,
  selected = false,
  dropTarget = false,
  dimmed = false,
  raised = false,
  interactive = false,
  className = "",
  children,
  ...rest
}: PlayingCardProps) {
  const ring = selected ? "ring-2 ring-accent" : dropTarget ? "ring-2 ring-white" : "";
  const shadow = dropTarget
    ? "shadow-[0_0_0_2px_#fff,0_0_18px_5px_rgba(255,255,255,0.7)]"
    : raised
      ? "shadow-card-raised"
      : interactive
        ? "shadow-card transition-shadow hover:shadow-card-raised"
        : "shadow-card";
  return (
    <button
      type="button"
      {...rest}
      className={`relative ${CARD_SIZE[size]} flex-none overflow-hidden rounded-card border border-black/10 bg-card ${shadow} ${ring} ${
        dimmed ? "opacity-30" : ""
      } ${className}`}
    >
      {src ? (
        // eslint-disable-next-line @next/next/no-img-element
        <img
          src={src}
          alt={name}
          loading="lazy"
          className={`absolute inset-0 size-full object-cover ${crop === "top" ? "object-top" : ""}`}
        />
      ) : (
        <CardFace src={null} name={name} energyType={energyType} />
      )}
      {children}
    </button>
  );
}
