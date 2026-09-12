import type { ReactNode } from "react";

/** The flex arrangement every Card's overlay content shares: a top
 *  row (right-aligned — the HP pill), a flexible middle (Damage
 *  Counter, Special Conditions), and a bottom row (a Tool at the
 *  start, its Energy at the end). Replaces what used to be several
 *  independently absolutely-positioned atoms with one shared flex
 *  layout — nothing here is placed by its own coordinates; each slot
 *  is placed by where it sits in this flex column, and each row
 *  arranges its own children the same way. */
export function CardOverlay({
  top,
  middle,
  bottomStart,
  bottomEnd,
}: {
  top?: ReactNode;
  middle?: ReactNode;
  bottomStart?: ReactNode;
  bottomEnd?: ReactNode;
}) {
  return (
    <div style={{ display: "flex", flexDirection: "column", width: "100%", height: "100%" }}>
      <div style={{ display: "flex", justifyContent: "flex-end", padding: 2 }}>{top}</div>
      <div
        style={{
          display: "flex",
          flex: 1,
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          gap: 2,
        }}
      >
        {middle}
      </div>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-end", padding: 2 }}>
        <div>{bottomStart}</div>
        <div style={{ display: "flex", gap: 2 }}>{bottomEnd}</div>
      </div>
    </div>
  );
}
