"use client";

import type { WireCard } from "../view";
import { PlayingCard } from "./PlayingCard";
import { CARD_SIZE } from "./sizes";
import type { Art } from "./shared";
import { useBoardVariant } from "./variant";

/** The Stadium in play, the whole card, always upright — a little
 *  larger than a deck/discard pile so its text stays legible. A dashed
 *  placeholder fills the slot when empty, and lights up with
 *  `placeHere` the same way an empty Bench/Active spot does, for a
 *  selected Stadium in hand. `ghost` keeps the space but hides it, so
 *  the centre lane stays balanced. */
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
  const roundedCourt = useBoardVariant() === "rounded-court";
  if (card && art) {
    return (
      <PlayingCard
        surface="tray"
        size="stadium"
        crop="full"
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
      className={`${CARD_SIZE.stadium} flex flex-none flex-col items-center justify-center text-center disabled:cursor-default ${
        roundedCourt
          ? "gap-1 rounded-2xl border border-dashed text-[9px]"
          : "rounded-card border border-dashed text-[8px]"
      } ${
        placeHere
          ? "border-accent bg-accent/10 text-accent animate-pulse"
          : "border-white/15 text-dim"
      } ${ghost ? "invisible" : ""}`}
    >
      {placeHere ? (
        "place here"
      ) : roundedCourt ? (
        <>
          <span className="text-[15px] leading-none">+</span>
          Stadium
        </>
      ) : (
        "stadium"
      )}
    </button>
  );
}
