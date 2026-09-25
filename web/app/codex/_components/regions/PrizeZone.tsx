import { PrizeMarker } from "../atoms/PrizeMarker";
import styles from "./regions.module.css";

export function PrizeZone({ remaining }: { remaining: number }) {
  const safeRemaining = Math.max(0, Math.min(6, remaining));
  return (
    <div className={styles.prizes} aria-label={`${safeRemaining} prizes remaining`}>
      {Array.from({ length: 6 }, (_, index) => (
        <PrizeMarker key={index} taken={index >= safeRemaining} />
      ))}
    </div>
  );
}
