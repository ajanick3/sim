"use client";

import { useState } from "react";
import { CardSlot } from "../atoms/CardSlot";
import { PokemonCard } from "../cards/PokemonCard";
import { CardPile } from "../cards/CardPile";
import { BenchRow, type BenchPokemon } from "./BenchRow";
import { HandGrid, type HandGridCard } from "./HandGrid";
import { PrizeZone } from "./PrizeZone";
import styles from "./battlefield.module.css";

export type BattlefieldProps = {
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

export function Battlefield(props: BattlefieldProps) {
  const [internalSelectedId, setInternalSelectedId] = useState<number | null>(null);
  const selectedId = props.selectedHandId === undefined ? internalSelectedId : props.selectedHandId;
  const selectHand = (id: number) => {
    if (props.onHandSelect) props.onHandSelect(id);
    else setInternalSelectedId((current) => (current === id ? null : id));
  };
  return (
    <main className={styles.board} aria-label="Pokémon battlefield">
      <div className={`${styles.piles} ${styles.opponentPiles}`}>
        <CardPile
          kind="discard"
          count={props.opponentDiscardCount}
          topCardImageUrl={props.opponentDiscardImageUrl}
        />
        <CardPile kind="deck" count={props.opponentDeckCount} />
      </div>
      <div className={`${styles.prizes} ${styles.opponentPrizes}`}>
        <PrizeZone remaining={props.opponentPrizesRemaining} />
      </div>
      <div className={`${styles.bench} ${styles.opponentBench}`}>
        <BenchRow cards={props.opponentBench} opponent />
      </div>
      <div className={`${styles.active} ${styles.opponentActive}`}>
        {props.opponentActive ? (
          <PokemonCard {...props.opponentActive} opponent />
        ) : (
          <CardSlot label="Active" />
        )}
      </div>

      <div className={styles.centerLine} aria-hidden="true">
        <span />
      </div>

      <div className={`${styles.active} ${styles.playerActive}`}>
        {props.active ? (
          <PokemonCard
            {...props.active}
            state={props.selectedPokemonId === props.active.id ? "selected" : "resting"}
            onSelect={
              props.onPokemonSelect ? () => props.onPokemonSelect?.(props.active!.id) : undefined
            }
          />
        ) : (
          <CardSlot label="Active" />
        )}
      </div>
      <div className={`${styles.prizes} ${styles.playerPrizes}`}>
        <PrizeZone remaining={props.prizesRemaining} />
      </div>
      <div className={`${styles.bench} ${styles.playerBench}`}>
        <BenchRow
          cards={props.bench}
          selectedId={props.selectedPokemonId}
          onSelect={props.onPokemonSelect}
        />
      </div>
      <div className={`${styles.piles} ${styles.playerPiles}`}>
        <CardPile kind="deck" count={props.deckCount} />
        <CardPile
          kind="discard"
          count={props.discardCount}
          topCardImageUrl={props.discardImageUrl}
        />
      </div>

      <div className={styles.hand}>
        <HandGrid cards={props.hand} selectedId={selectedId} onSelect={selectHand} />
      </div>
    </main>
  );
}
