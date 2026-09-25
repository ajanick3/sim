import { CardSlot } from "../atoms/CardSlot";
import { PokemonCard } from "../cards/PokemonCard";
import styles from "./regions.module.css";

export type BenchPokemon = {
  id: number;
  name: string;
  imageUrl: string | null;
  hp: number;
  damage?: number;
};

export function BenchRow({
  cards,
  opponent = false,
  selectedId,
  onSelect,
}: {
  cards: BenchPokemon[];
  opponent?: boolean;
  selectedId?: number | null;
  onSelect?: (id: number) => void;
}) {
  return (
    <section className={styles.bench} aria-label={`${opponent ? "Opponent" : "Your"} bench`}>
      {Array.from({ length: 5 }, (_, index) => {
        const card = cards[index];
        return card ? (
          <PokemonCard
            key={card.id}
            {...card}
            opponent={opponent}
            state={selectedId === card.id ? "selected" : "resting"}
            onSelect={onSelect ? () => onSelect(card.id) : undefined}
          />
        ) : (
          <CardSlot key={`slot-${index}`} label="Bench" />
        );
      })}
    </section>
  );
}
