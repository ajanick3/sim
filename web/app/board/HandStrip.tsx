"use client";

import type { PointerEvent as ReactPointerEvent, RefObject } from "react";
import { useState } from "react";
import type { Selection } from "../session";
import type { WireActionMeta, WireCard } from "../view";
import { PlayingCard } from "./PlayingCard";
import { splitHandRows, type Art } from "./shared";

/** The hand: cards sorted by category into two fixed-height rows, each
 *  the whole card. The block's own height never changes — a hand that
 *  overflows it scrolls vertically rather than growing a third row.
 *  Tapping a selected card confirms it with a toss animation toward the
 *  move's destination; a press-and-drag starts a drag the board
 *  resolves. */
export function HandStrip({
  hand,
  meta,
  selection,
  onHand,
  onConfirm,
  confirmIndex,
  art,
  onCardPointerDown,
  suppressClickRef,
  draggingCard,
}: {
  hand: WireCard[];
  meta: WireActionMeta[];
  selection: Selection;
  onHand: (card: number) => void;
  onConfirm: (index: number) => void;
  /** The one move a selected card resolves to, if it is unambiguous. */
  confirmIndex?: number;
  art: Art;
  /** Begin a possible drag from this card. */
  onCardPointerDown: (card: number, e: ReactPointerEvent) => void;
  /** Set true by a finished drag; the click it spawns is then skipped. */
  suppressClickRef: RefObject<boolean>;
  /** The card currently being dragged, dimmed in its slot. */
  draggingCard: number | null;
}) {
  const rows = splitHandRows(hand);
  const [tossing, setTossing] = useState<number | null>(null);
  const reduce =
    typeof window !== "undefined" &&
    typeof window.matchMedia === "function" &&
    window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  const onCardClick = (c: WireCard, selected: boolean, el: HTMLElement) => {
    if (selected && confirmIndex !== undefined) {
      if (reduce) {
        onConfirm(confirmIndex);
        return;
      }
      // Fly the card to where the move actually sends it — the target
      // Pokémon, the empty slot it fills, or the discard pile.
      const m = meta[confirmIndex];
      const sel =
        m?.target != null
          ? `[data-drop-id="mon:${m.target}"]`
          : m?.kind === "PlaceActive"
            ? '[data-drop-id="slot:active"]'
            : m?.kind === "PlayBasic" || m?.kind === "PlaceOnBench"
              ? '[data-drop-id="slot:bench"]'
              : '[data-toss-target="discard"]';
      const dest = document.querySelector(sel)?.getBoundingClientRect();
      if (dest) {
        const from = el.getBoundingClientRect();
        el.style.setProperty(
          "--toss-x",
          `${dest.left + dest.width / 2 - (from.left + from.width / 2)}px`,
        );
        el.style.setProperty(
          "--toss-y",
          `${dest.top + dest.height / 2 - (from.top + from.height / 2)}px`,
        );
      }
      setTossing(c.id);
      return;
    }
    onHand(c.id);
  };

  return (
    <div className="mt-1 rounded-lg border-2 border-accent/60 p-1">
      <div className="mb-1 text-[10px] uppercase tracking-widest text-dim">
        Hand ({hand.length})
      </div>
      {hand.length === 0 ? (
        <span className="px-2 py-8 text-[11px] text-dim">empty</span>
      ) : (
        <div className="flex h-[376px] flex-col gap-2 overflow-y-auto">
          {rows.map((row, ri) => (
            <div key={ri} className="flex flex-wrap justify-center gap-2">
              {row.map((c) => {
                const playable = meta.some((x) => x.card === c.id);
                const selected = selection?.kind === "hand" && selection.card === c.id;
                const src = art(c.print_id);
                return (
                  <PlayingCard
                    key={c.id}
                    size="picker"
                    crop="full"
                    src={src}
                    name={c.name}
                    energyType={c.energy_type}
                    selected={selected}
                    dimmed={draggingCard === c.id}
                    raised={selected && tossing !== c.id}
                    interactive
                    data-keep-selection
                    disabled={!playable}
                    onPointerDown={playable ? (e) => onCardPointerDown(c.id, e) : undefined}
                    onClick={(e) => {
                      if (suppressClickRef.current) {
                        suppressClickRef.current = false;
                        return;
                      }
                      onCardClick(c, selected, e.currentTarget);
                    }}
                    onAnimationEnd={(e) => {
                      if (
                        e.animationName === "card-toss" &&
                        tossing === c.id &&
                        confirmIndex !== undefined
                      )
                        onConfirm(confirmIndex);
                    }}
                    className={`touch-none transition-transform duration-150 will-change-transform hover:z-20 hover:-translate-y-2 hover:scale-[1.05] disabled:opacity-50 ${
                      tossing === c.id
                        ? "card-toss z-40"
                        : selected
                          ? "card-tap z-30 -translate-y-3 scale-[1.06]"
                          : ""
                    }`}
                  />
                );
              })}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
