import styles from "./atoms.module.css";

export function CardBack() {
  return (
    <span className={styles.back} aria-hidden="true">
      <svg viewBox="0 0 48 48" fill="none">
        <circle cx="24" cy="24" r="19" stroke="currentColor" strokeWidth="5" />
        <path d="M5 24h38" stroke="currentColor" strokeWidth="5" />
        <circle cx="24" cy="24" r="7" fill="#123950" stroke="currentColor" strokeWidth="4" />
      </svg>
    </span>
  );
}
