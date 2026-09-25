import styles from "./atoms.module.css";

export function DamageCounter({ damage }: { damage: number }) {
  if (damage <= 0) return null;
  return (
    <span className={styles.damage} aria-label={`${damage} damage`}>
      {damage}
    </span>
  );
}
