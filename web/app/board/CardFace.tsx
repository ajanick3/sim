"use client";

import { CardArt } from "./CardArt";
import { EnergyIcon, energyDefinitions, energyKind } from "./EnergyIcon";

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
  const kind = energyKind(energyType);
  if (kind) {
    // TCGdex has no art for Basic Energy — draw the card as its type
    // colour behind the type icon.
    const colour = energyDefinitions[kind].color;
    return (
      <span
        className="absolute inset-0 flex items-center justify-center"
        style={{ background: `linear-gradient(155deg, ${colour}, ${colour}bb 55%, ${colour}77)` }}
      >
        <EnergyIcon kind={kind} size="58%" decorative />
      </span>
    );
  }
  return (
    <span className="absolute inset-0 bg-card p-1 text-left text-[10px] font-semibold leading-tight text-card-ink">
      {name}
    </span>
  );
}
