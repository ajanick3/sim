"use client";

import { CardSlot } from "../atoms/CardSlot";
import { PokemonCard } from "../cards/PokemonCard";
import { CardPile } from "../cards/CardPile";
import { BenchRow, type BenchPokemon } from "./BenchRow";
import { PrizeZone } from "./PrizeZone";
import styles from "./board.module.css";

export type PlayerBoardProps = {
  /** True for the seat across the table: read-only, no selection. */
  opponent?: boolean;
  active: BenchPokemon | null;
  bench: BenchPokemon[];
  deckCount: number;
  discardCount: number;
  discardImageUrl?: string | null;
  prizesRemaining: number;
  selectedPokemonId?: number | null;
  onPokemonSelect?: (id: number) => void;
};

/** One side of the board: piles, prizes, bench, and the Active slot. The
 * opponent's side is the same layout, mirrored and read-only. */
export function PlayerBoard({
  opponent = false,
  active,
  bench,
  deckCount,
  discardCount,
  discardImageUrl,
  prizesRemaining,
  selectedPokemonId,
  onPokemonSelect,
}: PlayerBoardProps) {
  const deckPile = <CardPile kind="deck" count={deckCount} />;
  const discardPile = (
    <CardPile kind="discard" count={discardCount} topCardImageUrl={discardImageUrl} />
  );
  return (
    <>
      <div className={`${styles.piles} ${opponent ? styles.opponentPiles : styles.playerPiles}`}>
        {opponent ? (
          <>
            {discardPile}
            {deckPile}
          </>
        ) : (
          <>
            {deckPile}
            {discardPile}
          </>
        )}
      </div>
      <div className={`${styles.prizes} ${opponent ? styles.opponentPrizes : styles.playerPrizes}`}>
        <PrizeZone remaining={prizesRemaining} />
      </div>
      <div className={`${styles.bench} ${opponent ? styles.opponentBench : styles.playerBench}`}>
        <BenchRow
          cards={bench}
          opponent={opponent}
          selectedId={opponent ? undefined : selectedPokemonId}
          onSelect={opponent ? undefined : onPokemonSelect}
        />
      </div>
      <div className={`${styles.active} ${opponent ? styles.opponentActive : styles.playerActive}`}>
        {active ? (
          <PokemonCard
            {...active}
            opponent={opponent}
            state={!opponent && selectedPokemonId === active.id ? "selected" : "resting"}
            onSelect={!opponent && onPokemonSelect ? () => onPokemonSelect(active.id) : undefined}
          />
        ) : (
          <CardSlot label="Active" />
        )}
      </div>
    </>
  );
}
