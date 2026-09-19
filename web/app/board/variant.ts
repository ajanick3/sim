"use client";

import { createContext, useContext } from "react";

/** Which board treatment to draw. `classic` is the shipped board;
 *  `rounded-court` and `card-forward` are the two style directions
 *  from PRs #341 and #342, routed live at `/341` and `/342` so they
 *  can sit next to each other and against `/play` for comparison
 *  before either lands for real. A component reads this instead of
 *  forking — the classic path stays the only one `/play` ever hits. */
export type BoardVariant = "classic" | "rounded-court" | "card-forward";

const BoardVariantContext = createContext<BoardVariant>("classic");

export const BoardVariantProvider = BoardVariantContext.Provider;

export function useBoardVariant(): BoardVariant {
  return useContext(BoardVariantContext);
}
