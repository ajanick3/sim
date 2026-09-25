import type { CardState } from "../atoms/CardFrame";
import { HandCard } from "../cards/HandCard";
import styles from "./regions.module.css";

export type HandGridCard = {
  id: number;
  name: string;
  imageUrl: string | null;
  playable?: boolean;
};
export function HandGrid({
  cards,
  selectedId,
  onSelect,
}: {
  cards: HandGridCard[];
  selectedId?: number | null;
  onSelect: (id: number) => void;
}) {
  return (
    <section className={styles.hand} aria-label={`Hand, ${cards.length} cards`}>
      <div className={styles.handGrid}>
        {cards.map((card) => {
          const state: CardState =
            selectedId === card.id
              ? "selected"
              : card.playable === false
                ? "unavailable"
                : "resting";
          return (
            <HandCard
              key={card.id}
              name={card.name}
              imageUrl={card.imageUrl}
              state={state}
              disabled={card.playable === false}
              onSelect={() => onSelect(card.id)}
            />
          );
        })}
      </div>
    </section>
  );
}
