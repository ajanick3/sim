"use client";

import { CardArt } from "../../table";
import type { WireCard } from "../../view";
import { CARD_SIZE } from "./sizes";
import type { Art } from "./shared";

/** The deck stack and, below it, the discard pile as a button — tapping
 *  it opens a viewer for that pile. */
export function DeckPile({
  deck,
  discard,
  art,
  mine = false,
  onView,
}: {
  deck: number;
  discard: WireCard[];
  art: Art;
  mine?: boolean;
  onView?: () => void;
}) {
  const top = discard.at(-1);
  return (
    <div className="flex flex-col items-center gap-1">
      <div
        className={`relative ${CARD_SIZE.pile} rounded-card border border-white/20 bg-white/5 shadow-[2px_2px_0_rgba(255,255,255,0.06)]`}
      >
        <span className="absolute inset-x-0 bottom-0.5 text-center text-[10px] text-dim">
          {deck}
        </span>
      </div>
      <button
        type="button"
        data-keep-selection
        data-toss-target={mine ? "discard" : undefined}
        onClick={onView}
        disabled={!onView}
        aria-label="View discard pile"
        className={`relative ${CARD_SIZE.pile} overflow-hidden rounded-card border border-white/15 bg-panel p-0 disabled:cursor-default enabled:hover:border-accent`}
      >
        {top && art(top.print_id) ? (
          <CardArt src={art(top.print_id)!} alt={top.name} />
        ) : (
          <span className="absolute inset-0 flex items-center justify-center px-0.5 text-center text-[8px] leading-tight text-dim">
            {top?.name ?? "discard"}
          </span>
        )}
        <span className="absolute inset-x-0 bottom-0 bg-black/60 text-center text-[9px]">
          {discard.length}
        </span>
      </button>
    </div>
  );
}
