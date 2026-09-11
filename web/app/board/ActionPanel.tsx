"use client";

import { COPY_COLORS, groupActions, groupActionsAt } from "../session";
import { SEAT_NAME } from "./shared";

function SectionHeading({
  children,
  className = "",
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <h2 className={`m-0 text-[13px] uppercase tracking-[0.5px] text-dim ${className}`}>
      {children}
    </h2>
  );
}

/** The full legal-action list, grouped by kind, each move a squarish
 *  chip rather than a wide list row. At phone width this is still the
 *  collapsed "All actions" escape hatch for a phase with no on-board
 *  affordance yet; from tablet width up it sits permanently in the
 *  centre lane, across from the Stadium. `only` narrows it to the
 *  current selection's moves either way. */
export function ActionPanel({
  actions,
  only,
  onClearSelection,
  seat,
  busy,
  onAct,
}: {
  actions: string[];
  /** When set, show only these action indices — the current selection's moves. */
  only?: number[];
  onClearSelection: () => void;
  seat: number | undefined;
  busy: boolean;
  onAct: (index: number) => void;
}) {
  const groups = only ? groupActionsAt(actions, only) : groupActions(actions);
  return (
    <section>
      <div className="mb-1.5 flex items-center gap-2">
        <SectionHeading>
          {seat !== undefined ? `${SEAT_NAME[seat]} to act` : "Waiting"}
        </SectionHeading>
        {only && (
          <button className="text-[11px]" onClick={onClearSelection}>
            Clear selection
          </button>
        )}
      </div>
      <div className="grid gap-[10px]">
        {only && groups.length === 0 && (
          <p className="text-[12px] text-dim">No move here. Pick something else.</p>
        )}
        {groups.map((g) => (
          <div key={g.group}>
            <SectionHeading className="mb-1">{g.group}</SectionHeading>
            <div className="grid grid-cols-[repeat(auto-fill,minmax(60px,1fr))] gap-1.5">
              {g.items.map((item) => (
                <button
                  key={item.index}
                  disabled={busy}
                  onClick={() => onAct(item.index)}
                  className="flex min-h-[52px] flex-col items-center justify-center gap-1 rounded-md px-1.5 py-1.5 text-center text-[11px] leading-tight"
                >
                  {item.copy !== undefined && (
                    <span
                      aria-hidden
                      className="size-2 rounded-full"
                      style={{ background: COPY_COLORS[item.copy % COPY_COLORS.length] }}
                    />
                  )}
                  {item.label}
                </button>
              ))}
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
