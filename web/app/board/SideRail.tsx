"use client";

/** The right-hand rail: turn number, both prize counts, the END TURN
 *  button, a log button, and a » to collapse the whole rail. */
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
  return (
    <div className="flex w-[72px] flex-none flex-col items-center gap-2 pt-1">
      <button
        onClick={onHide}
        aria-label="Hide controls"
        className="w-full rounded-md border-edge py-0.5 text-[11px] text-dim transition-colors hover:border-accent hover:text-text"
      >
        ›
      </button>
      <div className="text-[10px] text-dim">turn {turn}</div>
      <div className="grid h-9 w-9 place-items-center rounded bg-prize text-lg font-bold">
        {oppPrizes}
      </div>
      <button
        onClick={onEndTurn}
        disabled={!canEndTurn}
        className="w-full rounded-md border-warn bg-warn/20 px-1 py-2 text-[11px] font-bold leading-tight text-warn disabled:opacity-40"
      >
        END
        <br />
        TURN
      </button>
      <div className="grid h-9 w-9 place-items-center rounded bg-accent text-lg font-bold text-black">
        {myPrizes}
      </div>
      <div className="text-[10px] text-dim">{yourTurn ? "your move" : "waiting"}</div>
      <button
        onClick={onLog}
        aria-label="Log"
        className="mt-1 grid size-9 place-items-center rounded-full border-edge text-base"
      >
        📜
      </button>
    </div>
  );
}
