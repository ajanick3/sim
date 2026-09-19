"use client";

import { Suspense } from "react";
import GameShell from "../game-shell";
import { BoardVariantProvider } from "../board/variant";

// PR #342's mockup — both Actives as full portrait cards, the same
// treatment Bench and Hand already carry — routed live here so it
// sits next to /341 and /play for comparison. Not the shipped board;
// delete this route (and the `card-forward` branches it feeds) once
// a direction is picked.
export default function Mockup342Page() {
  return (
    <BoardVariantProvider value="card-forward">
      <Suspense fallback={null}>
        <GameShell />
      </Suspense>
    </BoardVariantProvider>
  );
}
