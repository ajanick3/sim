import styles from "./atoms.module.css";

export function HealthBadge({ hp, opponent = false }: { hp: number; opponent?: boolean }) {
  return (
    <span className={styles.health} data-opponent={opponent} aria-label={`${hp} HP remaining`}>
      {hp}
    </span>
  );
}
