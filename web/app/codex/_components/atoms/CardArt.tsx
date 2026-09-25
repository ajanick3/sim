import { useState } from "react";
import styles from "./atoms.module.css";

/** The frame supplies the accessible name. Art inside it is decorative. */
export function CardArt({ src }: { src: string | null }) {
  const [failedSrc, setFailedSrc] = useState<string | null>(null);
  if (!src || failedSrc === src) {
    return <span className={styles.artFallback}>Image unavailable</span>;
  }
  return (
    // eslint-disable-next-line @next/next/no-img-element
    <img
      className={styles.art}
      src={src}
      alt=""
      draggable={false}
      onError={() => setFailedSrc(src)}
      width={245}
      height={337}
    />
  );
}
