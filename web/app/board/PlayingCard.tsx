"use client";

import type { ComponentPropsWithoutRef, ReactNode } from "react";
import { CardFace } from "./CardFace";
import { CARD_SIZE, type CardSize } from "./sizes";

/** The one card frame the board draws — Active, Bench, hand, picker,
 *  pile, drag ghost. `size` zooms it; `surface` is the paper card
 *  (white stock, warm shadow) or the tray card a Pokémon in play sits
 *  on (panel ground, a border). The fill is the print's art (cropped
 *  from the top, or whole) or, with no art, the drawn Energy face or
 *  the name. Movement and phase animation come from `className`;
 *  overlays — HP, damage, Energy — come as `children`. */
type PlayingCardProps = {
  size: CardSize;
  surface?: "paper" | "tray";
  /** "top" shows the name bar and head of the illustration; "full" the whole card. */
  crop?: "top" | "full";
  src: string | null;
  name: string;
  energyType?: string | null;
  selected?: boolean;
  dropTarget?: boolean;
  /** A drag is over this card right now — the drop ring gets louder. */
  hovered?: boolean;
  dimmed?: boolean;
  /** Sit at the raised shadow — a card that has lifted off the table. */
  raised?: boolean;
  /** Add the hover lift of the shadow. */
  interactive?: boolean;
  /** The tray card's resting border colour, e.g. `border-accent` for the Active. */
  restingBorder?: string;
  className?: string;
  children?: ReactNode;
} & Omit<ComponentPropsWithoutRef<"button">, "className" | "children">;

export function PlayingCard({
  size,
  surface = "paper",
  crop = "top",
  src,
  name,
  energyType = null,
  selected = false,
  dropTarget = false,
  hovered = false,
  dimmed = false,
  raised = false,
  interactive = false,
  restingBorder = "border-edge",
  className = "",
  children,
  ...rest
}: PlayingCardProps) {
  const tray = surface === "tray";

  const ring = selected
    ? "z-20 ring-2 ring-accent"
    : dropTarget
      ? hovered
        ? "z-30 ring-4 ring-white shadow-[0_0_0_3px_#fff,0_0_28px_10px_rgba(255,255,255,0.95)]"
        : "z-20 ring-2 ring-white shadow-[0_0_0_2px_#fff,0_0_18px_5px_rgba(255,255,255,0.7)]"
      : "";

  const base = tray
    ? `border bg-panel transition-colors ${selected ? "border-accent" : dropTarget ? "border-white" : restingBorder}`
    : `border border-black/10 bg-card ${
        dropTarget
          ? ""
          : raised
            ? "shadow-card-raised"
            : interactive
              ? "shadow-card transition-shadow hover:shadow-card-raised"
              : "shadow-card"
      }`;

  return (
    <button
      type="button"
      {...rest}
      className={`relative ${CARD_SIZE[size]} flex-none overflow-hidden rounded-card ${base} ${ring} ${
        dimmed ? "opacity-30" : ""
      } ${interactive && tray ? "hover:border-accent" : ""} ${className}`}
    >
      {src ? (
        // eslint-disable-next-line @next/next/no-img-element
        <img
          src={src}
          alt={name}
          loading="lazy"
          className={`absolute inset-0 size-full object-cover ${crop === "top" ? "object-top" : ""}`}
        />
      ) : tray ? (
        <span className="relative z-10 p-1 text-[9px] font-semibold leading-tight">{name}</span>
      ) : (
        <CardFace src={null} name={name} energyType={energyType} />
      )}
      {children}
    </button>
  );
}
