import styles from "./atoms.module.css";

/** Adapted from packages/ui/src/atoms/PokeBall.tsx. */
export function PrizeMarker({ taken = false }: { taken?: boolean }) {
  return (
    <svg
      className={styles.prize}
      data-taken={taken}
      viewBox="0 0 24 24"
      role="img"
      aria-label={taken ? "Prize taken" : "Prize remaining"}
    >
      <circle cx="12" cy="12" r="10.5" fill="#edf7ff" stroke="#102433" strokeWidth="1.5" />
      <path d="M1.5 12a10.5 10.5 0 0 1 21 0z" fill="#f04b4d" stroke="#102433" strokeWidth="1.5" />
      <path d="M1.5 12h21" stroke="#102433" strokeWidth="2" />
      <circle cx="12" cy="12" r="3.5" fill="#edf7ff" stroke="#102433" strokeWidth="1.5" />
      {taken && <path d="m7 12 3 3 7-7" fill="none" stroke="#102433" strokeWidth="2" />}
    </svg>
  );
}
