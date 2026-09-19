"use client";

import { Suspense } from "react";
import GameShell from "../game-shell";
import { BoardVariantProvider } from "../board/variant";

// PR #341's mockup — rounded court, a glow ring on your own Active, a
// pill END TURN, curved dividers — routed live here so it sits next
// to /342 and /play for comparison. Not the shipped board; delete
// this route (and the `rounded-court` branches it feeds) once a
// direction is picked.
export default function Mockup341Page() {
  return (
    <BoardVariantProvider value="rounded-court">
      <Suspense fallback={null}>
        <GameShell />
      </Suspense>
    </BoardVariantProvider>
  );
}
