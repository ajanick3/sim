import styles from "./atoms.module.css";

type CardSlotProps = { label: string; onPlace?: () => void; disabled?: boolean };

export function CardSlot({ label, onPlace, disabled = false }: CardSlotProps) {
  if (!onPlace) {
    return <div className={styles.slot} role="img" aria-label={label} data-card-frame />;
  }
  return (
    <button
      type="button"
      className={styles.slot}
      data-target="true"
      data-card-frame
      disabled={disabled}
      aria-label={label}
      onClick={onPlace}
    >
      <span aria-hidden="true" className={styles.plus}>
        +
      </span>
      <span className={styles.slotLabel}>Place here</span>
    </button>
  );
}
