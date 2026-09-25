import type { CardState } from "../atoms/CardFrame";
import { CardFrame } from "../atoms/CardFrame";
import { CardArt } from "../atoms/CardArt";
import { DamageCounter } from "../atoms/DamageCounter";
import { HealthBadge } from "../atoms/HealthBadge";
import styles from "./cards.module.css";

export type PokemonCardProps = {
  name: string;
  imageUrl: string | null;
  hp: number;
  damage?: number;
  opponent?: boolean;
  state?: CardState;
  onSelect?: () => void;
};

export function PokemonCard({
  name,
  imageUrl,
  hp,
  damage = 0,
  opponent = false,
  state = "resting",
  onSelect,
}: PokemonCardProps) {
  const content = (
    <>
      <CardArt src={imageUrl} />
      <span className={styles.hp}>
        <HealthBadge hp={hp} opponent={opponent} />
      </span>
      {damage > 0 && (
        <span className={styles.damage}>
          <DamageCounter damage={damage} />
        </span>
      )}
    </>
  );
  if (onSelect)
    return (
      <CardFrame label={`${name}, ${hp} HP remaining`} interactive state={state} onClick={onSelect}>
        {content}
      </CardFrame>
    );
  return (
    <CardFrame label={`${name}, ${hp} HP remaining`} state={state}>
      {content}
    </CardFrame>
  );
}
