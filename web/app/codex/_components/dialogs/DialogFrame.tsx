import type { ReactNode } from "react";
import styles from "./dialogs.module.css";

export function DialogFrame({
  title,
  description,
  children,
  footer,
  headerAction,
  compact = false,
  onDismiss,
}: {
  title: string;
  description?: string;
  children: ReactNode;
  footer?: ReactNode;
  /** Rendered top right, across from the title — for an action that closes
   *  or leaves the dialog, so it stays reachable without a second, floating
   *  dialog appearing on top of this one. */
  headerAction?: ReactNode;
  compact?: boolean;
  /** Closes the dialog on a click outside it. Omit for a dialog the player
   *  must resolve with one of its own buttons (a mandatory choice, a card
   *  search) rather than dismiss. */
  onDismiss?: () => void;
}) {
  return (
    <div className={styles.backdrop} onClick={onDismiss ? () => onDismiss() : undefined}>
      <section
        className={styles.dialog}
        data-compact={compact}
        role="dialog"
        aria-modal="true"
        aria-labelledby="codex-dialog-title"
        aria-describedby={description ? "codex-dialog-description" : undefined}
        onClick={(event) => event.stopPropagation()}
      >
        <header className={styles.header}>
          <div>
            <h1 id="codex-dialog-title">{title}</h1>
            {description && <p id="codex-dialog-description">{description}</p>}
          </div>
          {headerAction}
        </header>
        <div className={styles.body}>{children}</div>
        {footer && <footer className={styles.footer}>{footer}</footer>}
      </section>
    </div>
  );
}
