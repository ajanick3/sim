import type { MouseEventHandler, PointerEventHandler, ReactNode } from "react";
import styles from "./atoms.module.css";

export type CardState = "resting" | "selected" | "target" | "unavailable";

type CardFrameProps = {
  label: string;
  children: ReactNode;
  state?: CardState;
  className?: string;
} & (
  | {
      interactive: true;
      disabled?: boolean;
      onClick?: MouseEventHandler<HTMLButtonElement>;
      onPointerDown?: PointerEventHandler<HTMLButtonElement>;
    }
  | {
      interactive?: false;
      disabled?: never;
      onClick?: never;
      onPointerDown?: never;
    }
);

/** The parent sets the width. This frame keeps the full card ratio. */
export function CardFrame(props: CardFrameProps) {
  const { label, children, state = "resting", className = "" } = props;
  const frameClass = `${styles.frame} ${className}`;
  const content = (
    <>
      <span className={styles.face}>{children}</span>
      {state === "selected" && (
        <span className={styles.check} aria-hidden="true">
          ✓
        </span>
      )}
      {state === "target" && (
        <span className={styles.targetLabel} aria-hidden="true">
          Place here
        </span>
      )}
    </>
  );

  if (props.interactive) {
    return (
      <button
        type="button"
        className={frameClass}
        data-state={state}
        data-card-frame
        aria-label={label}
        aria-pressed={state === "selected"}
        disabled={props.disabled || state === "unavailable"}
        onClick={props.onClick}
        onPointerDown={props.onPointerDown}
      >
        {content}
      </button>
    );
  }

  return (
    <div className={frameClass} data-state={state} data-card-frame role="group" aria-label={label}>
      {content}
    </div>
  );
}
