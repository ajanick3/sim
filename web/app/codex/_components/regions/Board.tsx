"use client";

import { useState } from "react";
import { PlayerBoard } from "./PlayerBoard";
import type { BenchPokemon } from "./BenchRow";
import { HandGrid, type HandGridCard } from "./HandGrid";
import styles from "./board.module.css";

export type BoardProps = {
  active: BenchPokemon | null;
  opponentActive: BenchPokemon | null;
  bench: BenchPokemon[];
  opponentBench: BenchPokemon[];
  hand: HandGridCard[];
  deckCount: number;
  opponentDeckCount: number;
  discardCount: number;
  opponentDiscardCount: number;
  discardImageUrl?: string | null;
  opponentDiscardImageUrl?: string | null;
  prizesRemaining: number;
  opponentPrizesRemaining: number;
  selectedHandId?: number | null;
  selectedPokemonId?: number | null;
  onHandSelect?: (id: number) => void;
  onPokemonSelect?: (id: number) => void;
};

export function Board(props: BoardProps) {
  const [internalSelectedId, setInternalSelectedId] = useState<number | null>(null);
  const selectedId = props.selectedHandId === undefined ? internalSelectedId : props.selectedHandId;
  const selectHand = (id: number) => {
    if (props.onHandSelect) props.onHandSelect(id);
    else setInternalSelectedId((current) => (current === id ? null : id));
  };
  return (
    <main className={styles.board} aria-label="Pokémon battlefield">
      <PlayerBoard
        opponent
        active={props.opponentActive}
        bench={props.opponentBench}
        deckCount={props.opponentDeckCount}
        discardCount={props.opponentDiscardCount}
        discardImageUrl={props.opponentDiscardImageUrl}
        prizesRemaining={props.opponentPrizesRemaining}
      />

      <div className={styles.centerLine} aria-hidden="true">
        <span />
      </div>

      <PlayerBoard
        active={props.active}
        bench={props.bench}
        deckCount={props.deckCount}
        discardCount={props.discardCount}
        discardImageUrl={props.discardImageUrl}
        prizesRemaining={props.prizesRemaining}
        selectedPokemonId={props.selectedPokemonId}
        onPokemonSelect={props.onPokemonSelect}
      />

      <div className={styles.hand}>
        <HandGrid cards={props.hand} selectedId={selectedId} onSelect={selectHand} />
      </div>
    </main>
  );
}
