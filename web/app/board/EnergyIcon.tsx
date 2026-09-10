"use client";

import { useId, type CSSProperties, type SVGProps } from "react";

export const basicEnergyKinds = [
  "grass",
  "fire",
  "water",
  "lightning",
  "psychic",
  "fighting",
  "darkness",
  "metal",
] as const;

export const specialEnergyKinds = [
  "mist",
  "neo-upper",
  "legacy",
  "boomerang",
  "enriching",
  "spiky",
  "team-rockets",
  "ignition",
  "prism",
  "growing-grass",
  "rocky-fighting",
  "bubbly-water",
  "magnetic-metal",
] as const;

export type BasicEnergyKind = (typeof basicEnergyKinds)[number];
export type SpecialEnergyKind = (typeof specialEnergyKinds)[number];
export type EnergyKind = BasicEnergyKind | SpecialEnergyKind;
export type RegulationMark = "H" | "I" | "J";

type EnergyDefinition = {
  label: string;
  color: string;
  ink: string;
  mark?: RegulationMark;
};

export const energyDefinitions = {
  grass: { label: "Grass Energy", color: "#64B95A", ink: "#FFFFFF" },
  fire: { label: "Fire Energy", color: "#E95A47", ink: "#FFFFFF" },
  water: { label: "Water Energy", color: "#4AA9D8", ink: "#FFFFFF" },
  lightning: { label: "Lightning Energy", color: "#F6D84A", ink: "#2B2B2B" },
  psychic: { label: "Psychic Energy", color: "#B46AB3", ink: "#FFFFFF" },
  fighting: { label: "Fighting Energy", color: "#C78355", ink: "#FFFFFF" },
  darkness: { label: "Darkness Energy", color: "#53605B", ink: "#FFFFFF" },
  metal: { label: "Metal Energy", color: "#A9B4B8", ink: "#1F2937" },
  mist: { label: "Mist Energy", color: "#91A9C6", ink: "#FFFFFF", mark: "H" },
  "neo-upper": { label: "Neo Upper Energy", color: "#8E68B8", ink: "#FFFFFF", mark: "H" },
  legacy: { label: "Legacy Energy", color: "#D29A42", ink: "#FFFFFF", mark: "H" },
  boomerang: { label: "Boomerang Energy", color: "#C36B49", ink: "#FFFFFF", mark: "H" },
  enriching: { label: "Enriching Energy", color: "#6E9E55", ink: "#FFFFFF", mark: "H" },
  spiky: { label: "Spiky Energy", color: "#A56A9E", ink: "#FFFFFF", mark: "I" },
  "team-rockets": { label: "Team Rocket's Energy", color: "#3E4148", ink: "#FFFFFF", mark: "I" },
  ignition: { label: "Ignition Energy", color: "#DF6545", ink: "#FFFFFF", mark: "I" },
  prism: { label: "Prism Energy", color: "#527C92", ink: "#FFFFFF", mark: "I" },
  "growing-grass": { label: "Growing Grass Energy", color: "#5FAF62", ink: "#FFFFFF", mark: "J" },
  "rocky-fighting": { label: "Rocky Fighting Energy", color: "#B77950", ink: "#FFFFFF", mark: "J" },
  "bubbly-water": { label: "Bubbly Water Energy", color: "#459BC5", ink: "#FFFFFF", mark: "J" },
  "magnetic-metal": { label: "Magnetic Metal Energy", color: "#879AA2", ink: "#FFFFFF", mark: "J" },
} as const satisfies Record<EnergyKind, EnergyDefinition>;

export type EnergyIconProps = Omit<SVGProps<SVGSVGElement>, "color"> & {
  kind: EnergyKind;
  size?: number | string;
  label?: string;
  decorative?: boolean;
  color?: string;
  ink?: string;
  raised?: boolean;
};

const lineProps = {
  fill: "none",
  stroke: "currentColor",
  strokeWidth: 3.25,
  strokeLinecap: "round",
  strokeLinejoin: "round",
} as const;

function EnergyGlyph({ kind }: { kind: EnergyKind }) {
  switch (kind) {
    case "grass":
      return (
        <path d="M27 43C14 34 15 18 41 13c2 24-5 33-18 31m1-3c4-10 8-15 15-23" {...lineProps} />
      );
    case "fire":
      return (
        <path
          d="M33 11c2 10-6 12-6 21 0 4 2 7 5 9-1-7 4-10 7-15 6 5 9 11 7 18-2 8-9 13-17 12-10-1-16-8-15-17 1-9 8-17 19-28Z"
          fill="currentColor"
        />
      );
    case "water":
      return (
        <path
          d="M30 10S15 28 15 40a15 15 0 0 0 30 0c0-12-15-30-15-30Zm8 33c-1 5-5 8-10 8"
          {...lineProps}
        />
      );
    case "lightning":
      return <path d="m35 8-18 28h11l-3 20 20-30H33l2-18Z" fill="currentColor" />;
    case "psychic":
      return (
        <>
          <path d="M12 31s7-13 18-13 18 13 18 13-7 13-18 13S12 31 12 31Z" {...lineProps} />
          <circle cx="30" cy="31" r="6" fill="currentColor" />
          <path d="M18 49c7 4 17 4 24 0" {...lineProps} />
        </>
      );
    case "fighting":
      return (
        <path
          d="M16 29v-7a4 4 0 0 1 8 0v-5a4 4 0 0 1 8 0v3a4 4 0 0 1 8 0v4a4 4 0 0 1 8 1v12c0 12-7 19-18 19-8 0-13-4-17-11l-4-8a4 4 0 0 1 7-4l4 6V29h-4Z"
          fill="currentColor"
        />
      );
    case "darkness":
      return (
        <>
          <path d="M40 13a19 19 0 1 0 7 31 16 16 0 1 1-7-31Z" fill="currentColor" />
          <path d="m42 18 2 4 4 2-4 2-2 4-2-4-4-2 4-2 2-4Z" fill="currentColor" />
        </>
      );
    case "metal":
      return (
        <>
          <path d="m19 12 22 0 11 18-11 18H19L8 30l11-18Z" {...lineProps} />
          <circle cx="30" cy="30" r="9" {...lineProps} />
        </>
      );
    case "mist":
      return (
        <>
          <path d="M14 37h31a8 8 0 0 0 0-16 12 12 0 0 0-22-2 9 9 0 0 0-9 18Z" fill="currentColor" />
          <path d="M15 44h27M21 50h24" {...lineProps} />
        </>
      );
    case "neo-upper":
      return (
        <>
          <path d="m15 40 15-22 15 22M20 47l10-14 10 14" {...lineProps} />
          <path d="m30 8 3 6 7 1-5 5 1 7-6-3-6 3 1-7-5-5 7-1 3-6Z" fill="currentColor" />
        </>
      );
    case "legacy":
      return (
        <>
          <path
            d="M19 13h22M19 47h22M21 13c0 11 8 11 8 17s-8 6-8 17m18-34c0 11-8 11-8 17s8 6 8 17"
            {...lineProps}
          />
          <circle cx="30" cy="30" r="3" fill="currentColor" />
        </>
      );
    case "boomerang":
      return (
        <path
          d="M12 18c17 0 27 8 36 25l-9 5c-7-10-12-14-20-16l8 12-8 5L8 23l4-5Z"
          fill="currentColor"
        />
      );
    case "enriching":
      return (
        <>
          <path
            d="M30 49V29m0 5c-10 0-15-6-15-14 9-1 15 4 15 14Zm0-6c1-9 7-14 16-13 0 8-5 14-16 13Z"
            {...lineProps}
          />
          <path d="m45 34 2 4 4 2-4 2-2 4-2-4-4-2 4-2 2-4Z" fill="currentColor" />
        </>
      );
    case "spiky":
      return (
        <>
          <path
            d="m30 8 5 11 11-5-5 11 11 5-11 5 5 11-11-5-5 11-5-11-11 5 5-11-11-5 11-5-5-11 11 5 5-11Z"
            fill="currentColor"
          />
          <circle cx="30" cy="30" r="7" fill="var(--energy-fill)" />
        </>
      );
    case "team-rockets":
      return (
        <path
          d="M17 11h18c10 0 15 5 15 13 0 6-3 10-9 12l8 13H36l-7-12v12H17V11Zm12 10v8h5c3 0 5-1 5-4s-2-4-5-4h-5Z"
          fill="currentColor"
        />
      );
    case "ignition":
      return (
        <>
          <path
            d="M31 7c2 10-7 13-7 22 0 4 2 7 5 9-1-7 4-10 8-15 7 6 9 14 5 22-3 7-10 10-17 8-9-2-13-10-10-18 3-9 11-16 16-28Z"
            fill="currentColor"
          />
          <path d="m45 10 2 5 5 2-5 2-2 5-2-5-5-2 5-2 2-5Z" fill="currentColor" />
        </>
      );
    case "prism":
      return (
        <>
          <path d="m30 9 20 36H10L30 9Z" {...lineProps} />
          <path d="m30 9-4 36m4-23 20 23M26 45 13 40" {...lineProps} />
        </>
      );
    case "growing-grass":
      return (
        <>
          <path d="M27 46C15 38 16 22 40 17c1 21-5 29-17 29m1-3c4-9 8-14 14-20" {...lineProps} />
          <path d="M46 10v12M40 16h12" {...lineProps} />
        </>
      );
    case "rocky-fighting":
      return (
        <>
          <path d="m8 47 14-25 7 10 7-18 16 33H8Z" fill="currentColor" />
          <path d="m22 22 5 7 2-4 4 6" {...lineProps} />
        </>
      );
    case "bubbly-water":
      return (
        <>
          <circle cx="26" cy="35" r="10" {...lineProps} />
          <circle cx="40" cy="20" r="7" {...lineProps} />
          <circle cx="16" cy="17" r="4" fill="currentColor" />
          <circle cx="45" cy="43" r="4" fill="currentColor" />
        </>
      );
    case "magnetic-metal":
      return (
        <>
          <path d="M14 15v18a16 16 0 0 0 32 0V15H35v18a5 5 0 0 1-10 0V15H14Z" fill="currentColor" />
          <path d="M14 23h11M35 23h11" stroke="var(--energy-fill)" strokeWidth="3" />
        </>
      );
  }
}

export function EnergyIcon({
  kind,
  size = 48,
  label,
  decorative = false,
  color,
  ink,
  raised = true,
  style,
  ...props
}: EnergyIconProps) {
  const id = useId().replaceAll(":", "");
  const definition = energyDefinitions[kind];
  const accessibleLabel = label ?? definition.label;
  const mergedStyle = {
    "--energy-fill": color ?? definition.color,
    display: "block",
    flex: "0 0 auto",
    ...style,
  } as CSSProperties;

  return (
    <svg
      viewBox="0 0 60 60"
      width={size}
      height={size}
      role={decorative ? undefined : "img"}
      aria-hidden={decorative || undefined}
      aria-label={decorative ? undefined : accessibleLabel}
      focusable="false"
      style={mergedStyle}
      {...props}
    >
      {raised && (
        <defs>
          <filter id={`${id}-shadow`} x="-30%" y="-20%" width="160%" height="170%">
            <feDropShadow
              dx="0"
              dy="2"
              stdDeviation="2.25"
              floodColor="#16202A"
              floodOpacity="0.24"
            />
          </filter>
        </defs>
      )}
      <circle
        cx="30"
        cy="30"
        r="27"
        fill="var(--energy-fill)"
        stroke="rgba(255,255,255,0.42)"
        strokeWidth="1.5"
        filter={raised ? `url(#${id}-shadow)` : undefined}
      />
      <circle cx="30" cy="29" r="23.5" fill="none" stroke="rgba(0,0,0,0.08)" strokeWidth="1" />
      <g color={ink ?? definition.ink}>
        <EnergyGlyph kind={kind} />
      </g>
    </svg>
  );
}

/** The wire view names an Energy's type as "Fire", "Psychic", … — the
 *  eight Basic types. Map one to an icon kind, or null when it is not a
 *  type we draw. */
const WIRE_TO_KIND: Record<string, BasicEnergyKind> = {
  Grass: "grass",
  Fire: "fire",
  Water: "water",
  Lightning: "lightning",
  Psychic: "psychic",
  Fighting: "fighting",
  Darkness: "darkness",
  Metal: "metal",
};

export function energyKind(wireType: string | null | undefined): BasicEnergyKind | null {
  return wireType ? (WIRE_TO_KIND[wireType] ?? null) : null;
}
