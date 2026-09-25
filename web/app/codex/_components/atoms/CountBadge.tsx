import styles from "./atoms.module.css";

export function CountBadge({ count, label }: { count: number; label: string }) {
  return (
    <span className={styles.count} aria-label={`${label}: ${count}`}>
      {count}
    </span>
  );
}
