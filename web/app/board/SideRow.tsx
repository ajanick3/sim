"use client";

import type { Selection } from "../session";
import type { WireActionMeta, WireCard, WireSide } from "../view";
import { DeckPile } from "./DeckPile";
import { LiveMon } from "./LiveMon";
import { PrizeStack } from "./PrizeStack";
import { monHooks, type Art } from "./shared";

// A near-side Bench column: five whole cards side by side, this wide
// each, with this much air between them. The opponent's Bench caps at
// a fraction of this width instead — the same "farther away" read the
// Active row already gives their card, done here at the row level
// since a Bench card's own size is fluid, not a fixed token.
const BENCH_COLUMN_PX = 64;
const BENCH_GAP_PX = 6;
const FAR_SCALE = 0.75;

const CHIP = "rounded-full bg-panel/70 px-2 py-0.5 backdrop-blur";

/** One player's row: prize stack, the Bench (five columns wide at
 *  least; a raised Bench limit squishes the row narrower rather than
 *  wrapping it), and the deck / discard pile. The seat label and hand
 *  count used to sit inside the bordered panel as a header line,
 *  costing it a whole row of height the Bench could use instead — they
 *  now sit below the panel as two small chips. `mine` tints the panel,
 *  scales the Bench to its full near-side width, and lets empty slots
 *  be placed onto — the opponent's Bench renders the same grid capped
 *  to `FAR_SCALE` of that width. */
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
  hoverDropId = null,
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
  hoverDropId?: string | null;
}) {
  // At least five columns; a raised Bench limit adds more instead of
  // wrapping a second row.
  const cols = Math.max(5, side.bench.length);
  const slots = [...side.bench, ...Array(Math.max(0, cols - side.bench.length)).fill(null)];
  const maxWidth = (5 * BENCH_COLUMN_PX + 4 * BENCH_GAP_PX) * (mine ? 1 : FAR_SCALE);
  return (
    <div className="flex flex-col gap-1">
      <div className={`flex items-start gap-2 ${mine ? "" : "flex-row-reverse"}`}>
        <PrizeStack count={side.prize_count} />
        <div
          className={`min-w-0 flex-1 rounded-lg border-2 p-1.5 ${
            mine ? "border-accent/60" : "border-warn/50"
          }`}
        >
          <div
            className={`grid gap-1.5 ${mine ? "" : "ml-auto"}`}
            style={{
              gridTemplateColumns: `repeat(${cols}, minmax(0, 1fr))`,
              maxWidth: `${maxWidth}px`,
            }}
          >
            {slots.map((m, i) =>
              m ? (
                <LiveMon
                  key={i}
                  mon={m}
                  art={art}
                  {...monHooks(m, meta, selection, dropTargets, onPokemon, false, hoverDropId)}
                />
              ) : (
                <LiveMon key={i} mon={null} art={art} placeHere={mine ? onPlaceBench : undefined} />
              ),
            )}
          </div>
        </div>
        <DeckPile
          deck={side.deck_count}
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
      <div className="flex items-center gap-1.5 text-[9px] uppercase tracking-widest text-dim">
        <span className={CHIP}>{label}</span>
        <span className={CHIP}>hand {side.hand_count}</span>
      </div>
    </div>
  );
}
