"use client";

import type { WireCard } from "../view";
import { PlayingCard } from "./PlayingCard";
import type { Art } from "./shared";

/** The Stadium in play. Draws the card when one is out, a dashed
 *  placeholder when the slot is empty. `ghost` keeps the space but hides
 *  it, so the centre lane stays balanced. */
export function StadiumSlot({
  card,
  art,
  ghost = false,
}: {
  card?: WireCard | null;
  art?: Art;
  ghost?: boolean;
}) {
  if (card && art) {
    return (
      <PlayingCard
        surface="tray"
        size="pile"
        crop="top"
        src={art(card.print_id)}
        name={card.name}
        title={`Stadium: ${card.name}`}
      />
    );
  }
  return (
    <div
      className={`flex h-[64px] w-[46px] flex-col items-center justify-center rounded-card border border-dashed border-white/15 text-center text-[8px] text-dim ${
        ghost ? "invisible" : ""
      }`}
    >
      stadium
    </div>
  );
}
