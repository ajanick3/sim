import type { ReactNode } from "react";
import styles from "./dialogs.module.css";

export function DialogFrame({
  title,
  description,
  children,
  footer,
  headerAction,
  compact = false,
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
}) {
  return (
    <div className={styles.backdrop}>
      <section
        className={styles.dialog}
        data-compact={compact}
        role="dialog"
        aria-modal="true"
        aria-labelledby="codex-dialog-title"
        aria-describedby={description ? "codex-dialog-description" : undefined}
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
