"use client";

import { DeckSearchDialog, type SearchCard } from "../../_components/dialogs/DeckSearchDialog";
import { CRUSHING_HAMMER_ART, DREEPY_ART } from "../../_fixtures/cardArt";
import { BoardPreview } from "../board/BoardPreview";

const cards: SearchCard[] = Array.from({ length: 18 }, (_, index) => ({
  id: index + 1,
  name: index % 3 === 0 ? "Dreepy" : "Crushing Hammer",
  imageUrl: index % 3 === 0 ? DREEPY_ART : CRUSHING_HAMMER_ART,
  eligible: index % 4 !== 2,
}));

export function DeckSearchPreview() {
  return (
    <>
      <BoardPreview />
      <DeckSearchDialog cards={cards} onDone={() => {}} />
    </>
  );
}
