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
  onCancel,
}: {
  title: string;
  description?: string;
  actions: GameAction[];
  onChoose?: (id: number) => void;
  showCancel?: boolean;
  onCancel?: () => void;
}) {
  return (
    <DialogFrame
      title={title}
      description={description}
      compact
      onDismiss={showCancel ? onCancel : undefined}
      footer={
        showCancel ? (
          <button className={styles.secondaryButton} onClick={onCancel}>
            Cancel
          </button>
        ) : undefined
      }
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
