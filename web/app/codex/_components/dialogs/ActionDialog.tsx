"use client";

import { DialogFrame } from "./DialogFrame";
import styles from "./dialogs.module.css";

export type GameAction = {
  id: number;
  label: string;
  detail?: string;
  tone?: "default" | "warning";
};

export function ActionDialog({
  title,
  description,
  actions,
  onChoose,
  showCancel = true,
}: {
  title: string;
  description?: string;
  actions: GameAction[];
  onChoose?: (id: number) => void;
  showCancel?: boolean;
}) {
  return (
    <DialogFrame
      title={title}
      description={description}
      compact
      footer={showCancel ? <button className={styles.secondaryButton}>Cancel</button> : undefined}
    >
      <div className={styles.actionGrid}>
        {actions.map((action) => (
          <button
            key={action.id}
            className={styles.actionButton}
            data-tone={action.tone}
            onClick={() => onChoose?.(action.id)}
          >
            <strong>{action.label}</strong>
            {action.detail && <span>{action.detail}</span>}
          </button>
        ))}
      </div>
    </DialogFrame>
  );
}
