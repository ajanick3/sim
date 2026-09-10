"use client";

import { useState } from "react";

/** A card's illustration, filling its positioned parent, with a scrim
 *  at the foot so overlaid chips stay readable. Renders nothing if the
 *  image 404s — the caller's fallback shows through. */
export function CardArt({ src, alt }: { src: string; alt: string }) {
  const [broken, setBroken] = useState(false);
  if (broken) return null;
  return (
    <>
      {/* eslint-disable-next-line @next/next/no-img-element */}
      <img
        src={src}
        alt={alt}
        loading="lazy"
        onError={() => setBroken(true)}
        className="absolute inset-0 size-full rounded-card object-cover"
      />
      <div className="absolute inset-x-0 bottom-0 h-2/3 rounded-b-card bg-gradient-to-t from-black/85 to-transparent" />
    </>
  );
}
