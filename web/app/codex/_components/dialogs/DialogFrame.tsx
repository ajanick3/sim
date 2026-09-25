import type { ReactNode } from "react";
import styles from "./dialogs.module.css";

export function DialogFrame({
  title,
  description,
  children,
  footer,
  compact = false,
}: {
  title: string;
  description?: string;
  children: ReactNode;
  footer?: ReactNode;
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
        </header>
        <div className={styles.body}>{children}</div>
        {footer && <footer className={styles.footer}>{footer}</footer>}
      </section>
    </div>
  );
}
