"use client";

import { CardArt } from "./CardArt";
import { ENERGY_COLOR } from "./energy";

/** The face of a card in a tile: real art when there is any, a drawn
 *  Energy card for Basic Energy (TCGdex has no art for those), else the
 *  name. Fills its positioned parent. */
export function CardFace({
  src,
  name,
  energyType,
}: {
  src: string | null;
  name: string;
  energyType: string | null;
}) {
  if (src) return <CardArt src={src} alt={name} />;
  if (energyType) {
    // TCGdex has no art for Basic Energy, so draw the card: the type
    // colour edge to edge with the big centre disc a real one carries.
    const colour = ENERGY_COLOR[energyType] ?? "var(--color-dim)";
    return (
      <span
        className="absolute inset-0 flex items-center justify-center"
        style={{ background: `linear-gradient(155deg, ${colour}, ${colour}bb 55%, ${colour}77)` }}
      >
        <span
          className="grid size-[46%] place-items-center rounded-full border-[3px] border-white/80"
          style={{ background: `radial-gradient(circle at 38% 32%, #ffffffd0, ${colour} 72%)` }}
        >
          <span className="text-[10px] font-black uppercase text-black/55">
            {energyType.slice(0, 2)}
          </span>
        </span>
      </span>
    );
  }
  return (
    <span className="absolute inset-0 bg-card p-1 text-left text-[10px] font-semibold leading-tight text-card-ink">
      {name}
    </span>
  );
}
