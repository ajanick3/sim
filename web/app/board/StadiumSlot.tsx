"use client";

import type { WireCard } from "../view";
import { PlayingCard } from "./PlayingCard";
import type { Art } from "./shared";

/** The Stadium in play. Draws the card when one is out, a dashed
 *  placeholder when the slot is empty. `ghost` keeps the space but hides
 *  it, so the centre lane stays balanced. `placeHere` lights the empty
 *  slot up the same way an empty Bench/Active spot does, for a selected
 *  Stadium in hand. */
export function StadiumSlot({
  card,
  art,
  ghost = false,
  placeHere,
}: {
  card?: WireCard | null;
  art?: Art;
  ghost?: boolean;
  placeHere?: () => void;
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
    <button
      type="button"
      data-keep-selection
      data-drop-id={placeHere ? "slot:stadium" : undefined}
      disabled={!placeHere}
      onClick={placeHere}
      className={`flex h-[64px] w-[46px] flex-col items-center justify-center rounded-card border border-dashed text-center text-[8px] disabled:cursor-default ${
        placeHere ? "border-accent bg-accent/10 text-accent animate-pulse" : "border-white/15 text-dim"
      } ${ghost ? "invisible" : ""}`}
    >
      {placeHere ? "place here" : "stadium"}
    </button>
  );
}
