import type { CSSProperties, ReactNode } from "react";
import crushingHammer from "./assets/crushing-hammer.png";
import type { BoardCard } from "./types";

export const CARD_ASPECT_RATIO = "245 / 337";

type CardSurfaceProps = {
  card: BoardCard;
  selected?: boolean;
  onSelect?: (card: BoardCard) => void;
  className?: string;
  children?: ReactNode;
};

export function CardSurface({ card, selected = false, onSelect, className = "", children }: CardSurfaceProps) {
  const Component = onSelect ? "button" : "div";
  const style: CSSProperties = {
    aspectRatio: CARD_ASPECT_RATIO,
    backgroundImage: `url(${card.imageUrl ?? crushingHammer})`,
  };

  return (
    <Component
      type={onSelect ? "button" : undefined}
      className={`chatgpt-card${selected ? " chatgpt-card--selected" : ""}${className ? ` ${className}` : ""}`}
      style={style}
      role={onSelect ? undefined : "img"}
      aria-label={card.name}
      aria-pressed={onSelect ? selected : undefined}
      data-card-id={card.id}
      data-card-ratio={CARD_ASPECT_RATIO}
      onClick={onSelect ? () => onSelect(card) : undefined}
    >
      {card.damage !== undefined && card.damage > 0 && <span className="chatgpt-damage">{card.damage}</span>}
      {children}
    </Component>
  );
}

export function EmptyCardSlot({ label }: { label: string }) {
  return (
    <div
      className="chatgpt-card-slot"
      style={{ aspectRatio: CARD_ASPECT_RATIO }}
      aria-label={label}
      data-card-ratio={CARD_ASPECT_RATIO}
    />
  );
}
