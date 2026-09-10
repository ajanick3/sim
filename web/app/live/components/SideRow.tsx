"use client";

import type { Selection } from "../../session";
import type { WireActionMeta, WireCard, WireSide } from "../../view";
import { DeckPile } from "./DeckPile";
import { LiveMon } from "./LiveMon";
import { PrizeStack } from "./PrizeStack";
import { monHooks, type Art } from "./shared";

/** One player's row: prize stack, the five Bench slots, and the deck /
 *  discard pile. `mine` tints it and lets empty slots be placed onto. */
export function SideRow({
  side,
  label,
  mine = false,
  art,
  meta,
  selection,
  dropTargets,
  onPokemon,
  onPlaceBench,
  onViewDiscard,
}: {
  side: WireSide;
  label: string;
  mine?: boolean;
  art: Art;
  meta: WireActionMeta[];
  selection: Selection;
  dropTargets: Map<number, number>;
  onPokemon: (id: number) => void;
  onPlaceBench?: () => void;
  onViewDiscard?: (cards: WireCard[], label: string) => void;
}) {
  // Five slots: the Pokémon on the Bench, then empty pads to fill.
  const slots = [...side.bench, ...Array(Math.max(0, 5 - side.bench.length)).fill(null)];
  return (
    <div className={`flex items-start gap-2 ${mine ? "" : "flex-row-reverse"}`}>
      <PrizeStack count={side.prize_count} />
      <div
        className={`min-w-0 flex-1 rounded-lg border-2 p-1.5 ${
          mine ? "border-accent/60" : "border-warn/50"
        }`}
      >
        <div className="mb-1 flex items-center justify-between text-[10px] uppercase tracking-widest text-dim">
          <span>{label}</span>
          <span>hand {side.hand_count}</span>
        </div>
        <div className="flex flex-wrap gap-1.5">
          {slots.map((m, i) =>
            m ? (
              <LiveMon
                key={i}
                mon={m}
                art={art}
                small
                {...monHooks(m, meta, selection, dropTargets, onPokemon)}
              />
            ) : (
              <LiveMon
                key={i}
                mon={null}
                art={art}
                small
                placeHere={mine ? onPlaceBench : undefined}
              />
            ),
          )}
        </div>
      </div>
      <DeckPile
        deck={side.library_count}
        discard={side.discard}
        art={art}
        mine={mine}
        onView={
          onViewDiscard && side.discard.length > 0
            ? () => onViewDiscard(side.discard, `${label} — discard`)
            : undefined
        }
      />
    </div>
  );
}
