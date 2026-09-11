"use client";

/** Turn number, both prize counts, END TURN, a log button, and a » to
 *  collapse the lot — floating over the board as a column of small,
 *  unobtrusive FABs pinned to the right edge and centred in the
 *  viewport, rather than a boxed sidebar that eats into the board's
 *  own width. */
export function SideRail({
  myPrizes,
  oppPrizes,
  turn,
  yourTurn,
  canEndTurn,
  onEndTurn,
  onLog,
  onHide,
}: {
  myPrizes: number;
  oppPrizes: number;
  turn: number;
  yourTurn: boolean;
  canEndTurn: boolean;
  onEndTurn: () => void;
  onLog: () => void;
  onHide: () => void;
}) {
  const fab =
    "grid place-items-center rounded-full border border-edge bg-panel/80 shadow-[0_2px_10px_rgba(0,0,0,0.4)] backdrop-blur transition-colors";
  return (
    <div className="fixed right-2 top-1/2 z-30 flex -translate-y-1/2 flex-col items-center gap-2.5">
      <button
        onClick={onHide}
        aria-label="Hide controls"
        className={`${fab} size-7 text-[11px] text-dim hover:border-accent hover:text-text`}
      >
        ›
      </button>
      <span className="rounded-full bg-panel/70 px-2 py-0.5 text-[9px] text-dim backdrop-blur">
        turn {turn}
      </span>
      <div className={`${fab} size-9 border-none bg-prize/90 text-base font-bold text-black`}>
        {oppPrizes}
      </div>
      <button
        onClick={onEndTurn}
        disabled={!canEndTurn}
        className={`${fab} size-14 border-warn/70 bg-warn/25 text-[10px] font-bold leading-tight text-warn disabled:opacity-40`}
      >
        END
        <br />
        TURN
      </button>
      <div className={`${fab} size-9 border-none bg-accent/90 text-base font-bold text-black`}>
        {myPrizes}
      </div>
      <span className="rounded-full bg-panel/70 px-2 py-0.5 text-[9px] text-dim backdrop-blur">
        {yourTurn ? "your move" : "waiting"}
      </span>
      <button
        onClick={onLog}
        aria-label="Log"
        className={`${fab} size-9 text-base hover:border-accent`}
      >
        📜
      </button>
    </div>
  );
}
