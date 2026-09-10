"use client";

import type { WirePokemon } from "../view";
import { EnergyIcon, energyKind } from "./EnergyIcon";
import { CARD_SIZE } from "./sizes";
import { damageSpot, type Art } from "./shared";

/** One Pokémon in play — Active or Bench — as a card showing the top
 *  slice of its print, with HP, damage counter, attached Energy and any
 *  Special Conditions laid over it. `mon = null` draws an empty slot,
 *  which a held card can be tapped onto when `placeHere` is set. */
export function LiveMon({
  mon,
  active = false,
  small = false,
  art,
  onSelect,
  selectable = false,
  selected = false,
  dropTarget = false,
  hovered = false,
  placeHere,
}: {
  mon: WirePokemon | null;
  active?: boolean;
  small?: boolean;
  art: Art;
  onSelect?: () => void;
  selectable?: boolean;
  selected?: boolean;
  dropTarget?: boolean;
  /** A drag is over this card right now. */
  hovered?: boolean;
  /** Empty slot: a selected hand card can be placed here. */
  placeHere?: () => void;
}) {
  const size = active ? CARD_SIZE.active : small ? CARD_SIZE.benchSmall : CARD_SIZE.bench;
  if (!mon) {
    return (
      <button
        type="button"
        data-keep-selection
        data-drop-id={placeHere ? (active ? "slot:active" : "slot:bench") : undefined}
        disabled={!placeHere}
        onClick={placeHere}
        className={`${size} flex flex-none items-center justify-center rounded-card border border-dashed text-[9px] disabled:cursor-default ${
          placeHere
            ? "border-accent bg-accent/10 text-accent animate-pulse"
            : "border-white/15 text-dim"
        }`}
      >
        {placeHere ? "place here" : active ? "no Active" : ""}
      </button>
    );
  }
  const src = art(mon.print_id);
  const interactive = selectable && !!onSelect;
  const ring = selected
    ? "z-20 ring-2 ring-accent border-accent"
    : dropTarget
      ? hovered
        ? "z-30 border-white ring-4 ring-white shadow-[0_0_0_3px_#fff,0_0_28px_10px_rgba(255,255,255,0.95)]"
        : "z-20 border-white ring-2 ring-white shadow-[0_0_0_2px_#fff,0_0_18px_5px_rgba(255,255,255,0.7)]"
      : active
        ? "border-accent"
        : "border-edge";
  const energies = mon.attached.filter((c) => c.energy_type);
  return (
    <button
      type="button"
      data-keep-selection
      data-drop-id={`mon:${mon.id}`}
      disabled={!interactive}
      onClick={onSelect}
      className={`deal-in ${size} ${
        selected ? "card-tap" : ""
      } relative flex flex-none flex-col overflow-hidden rounded-card border bg-panel transition-colors disabled:cursor-default disabled:opacity-100 ${ring} ${
        interactive ? "hover:border-accent" : ""
      }`}
    >
      {/* Every card on the board — Active and Bench alike — shows the top
          slice of the print, cropped from the top edge. No scrim: the
          only thing laid over a card is the dimming of an illegal one. */}
      {src ? (
        // eslint-disable-next-line @next/next/no-img-element
        <img
          src={src}
          alt={mon.name}
          loading="lazy"
          className="absolute inset-0 size-full object-cover object-top"
        />
      ) : (
        <span className="relative z-10 p-1 text-[9px] font-semibold leading-tight">{mon.name}</span>
      )}

      {/* HP pill — top-right, where a card prints it. */}
      <span className="absolute right-0.5 top-0.5 z-10 rounded bg-black/75 px-1 text-[9px] font-bold">
        {mon.hp}
      </span>
      {/* Damage counter: a coin dropped on the illustration, its spot
          fixed per Pokémon so it does not jump between renders. */}
      {mon.damage > 0 && (
        <span
          className={`absolute z-10 grid -translate-x-1/2 -translate-y-1/2 place-items-center rounded-full border-2 border-black/50 bg-damage font-black text-black shadow-[0_2px_5px_rgba(0,0,0,0.6)] ${
            active ? "size-11 text-base" : "size-6 text-[10px]"
          }`}
          style={damageSpot(mon.id)}
        >
          {mon.damage}
        </span>
      )}

      {/* Energy — bottom-right, always full strength however the card
          reads, on a dark pill so the icons stay bright over any art. */}
      {energies.length > 0 && (
        <span className="absolute bottom-0.5 right-0.5 z-20 flex gap-0.5 rounded-full bg-black/45 px-1 py-0.5 opacity-100">
          {energies.map((c) => {
            const kind = energyKind(c.energy_type);
            return kind ? (
              <EnergyIcon key={c.id} kind={kind} size={14} decorative />
            ) : (
              <span
                key={c.id}
                title={`${c.energy_type} Energy`}
                className="size-2.5 rounded-full border border-black/50 bg-dim"
              />
            );
          })}
        </span>
      )}

      {mon.conditions.length > 0 && (
        <span className="absolute inset-x-0 top-1/2 z-10 bg-black/60 text-center text-[8px] text-warn">
          {mon.conditions.join(", ")}
        </span>
      )}
    </button>
  );
}
