import type { ComponentPropsWithoutRef } from "react";
import styles from "./atoms.module.css";

type IconButtonProps = Omit<ComponentPropsWithoutRef<"button">, "aria-label"> & { label: string };

export function IconButton({ label, className = "", children, ...props }: IconButtonProps) {
  return (
    <button
      {...props}
      type="button"
      className={`${styles.iconButton} ${className}`}
      aria-label={label}
      title={label}
    >
      <span aria-hidden="true">{children}</span>
    </button>
  );
}
