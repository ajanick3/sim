"use client";

/** The dashed placeholder for the Stadium in play. `ghost` keeps the
 *  space but hides it, so the centre lane stays balanced. */
export function StadiumSlot({ ghost = false }: { ghost?: boolean }) {
  return (
    <div
      className={`flex h-[90px] w-[56px] flex-col items-center justify-center rounded border border-dashed border-white/15 text-center text-[8px] text-dim ${
        ghost ? "invisible" : ""
      }`}
    >
      stadium
    </div>
  );
}
