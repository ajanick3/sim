"use client";

import { useState } from "react";
import { HandCard } from "../cards/HandCard";
import { DialogFrame } from "./DialogFrame";
import styles from "./dialogs.module.css";

export type SearchCard = {
  id: number;
  name: string;
  imageUrl: string | null;
  eligible: boolean;
};

export function DeckSearchDialog({
  cards,
  maxSelections = 1,
  onConfirm,
  onDone,
  doneLabel = "Done",
}: {
  cards: SearchCard[];
  maxSelections?: number;
  onConfirm?: (ids: number[]) => void;
  onDone?: () => void;
  /** The engine's own label for `onDone` (e.g. "Stop taking cards"), so
   *  the header button reads the same as the action it sends. */
  doneLabel?: string;
}) {
  const [selected, setSelected] = useState<number[]>([]);
  const sortedCards = cards
    .map((card, index) => ({ card, index }))
    .sort((a, b) => Number(b.card.eligible) - Number(a.card.eligible) || a.index - b.index)
    .map(({ card }) => card);
  const toggle = (id: number) => {
    setSelected((current) =>
      current.includes(id)
        ? current.filter((selectedId) => selectedId !== id)
        : maxSelections === 1
          ? [id]
          : [...current, id].slice(-maxSelections),
    );
  };
  return (
    <DialogFrame
      title="Search your deck"
      description={`Choose ${maxSelections === 1 ? "a card" : `up to ${maxSelections} cards`}. Dimmed cards cannot be taken.`}
      headerAction={
        onDone && (
          <button className={styles.headerButton} onClick={onDone}>
            {doneLabel}
          </button>
        )
      }
      footer={
        <>
          <span className={styles.selectionCount}>{selected.length} selected</span>
          <button
            className={styles.primaryButton}
            disabled={selected.length === 0}
            onClick={() => onConfirm?.(selected)}
          >
            Add to hand
          </button>
        </>
      }
    >
      <div className={styles.searchGrid} aria-label="Cards in deck">
        {sortedCards.map((card) => (
          <HandCard
            key={card.id}
            name={card.name}
            imageUrl={card.imageUrl}
            state={
              selected.includes(card.id) ? "selected" : card.eligible ? "resting" : "unavailable"
            }
            disabled={!card.eligible}
            onSelect={() => toggle(card.id)}
          />
        ))}
      </div>
    </DialogFrame>
  );
}
