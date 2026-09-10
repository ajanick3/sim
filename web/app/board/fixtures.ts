// Sample data for the /live component stories. None of it hits the
// network: `noArt` draws every card as its fallback face, `swatchArt`
// hands back a tiny inline gradient so the <img> path renders too.

import type { WireActionMeta, WireCard, WirePokemon, WireSide, WireView } from "../view";
import type { Art } from "./shared";

export const noArt: Art = () => null;

/** A 5:7 gradient data URI, so stories can exercise the image path. */
export const swatchArt: Art = (printId) => {
  const hue = ([...printId].reduce((a, c) => a + c.charCodeAt(0), 0) * 37) % 360;
  const svg =
    `<svg xmlns='http://www.w3.org/2000/svg' width='250' height='350'>` +
    `<defs><linearGradient id='g' x1='0' y1='0' x2='1' y2='1'>` +
    `<stop offset='0' stop-color='hsl(${hue} 60% 55%)'/>` +
    `<stop offset='1' stop-color='hsl(${(hue + 40) % 360} 55% 35%)'/>` +
    `</linearGradient></defs><rect width='250' height='350' fill='url(#g)'/>` +
    `<rect x='14' y='14' width='222' height='40' rx='6' fill='rgba(255,255,255,0.85)'/></svg>`;
  return `data:image/svg+xml;utf8,${encodeURIComponent(svg)}`;
};

let nextId = 1;
const id = () => nextId++;

export function card(overrides: Partial<WireCard> = {}): WireCard {
  const i = id();
  return {
    id: i,
    name: `Card ${i}`,
    def: 0,
    print_id: `test-${i}`,
    energy_type: null,
    category: "item",
    ...overrides,
  };
}

export function pokemon(overrides: Partial<WirePokemon> = {}): WirePokemon {
  const i = id();
  const hp = overrides.hp ?? 120;
  const damage = overrides.damage ?? 0;
  return {
    id: i,
    name: `Mon ${i}`,
    print_id: `mon-${i}`,
    hp,
    damage,
    remaining_hp: hp - damage,
    conditions: [],
    attached: [],
    ...overrides,
  };
}

export const fireEnergy = (): WireCard =>
  card({ name: "Fire Energy", energy_type: "Fire", category: "energy" });
export const psychicEnergy = (): WireCard =>
  card({ name: "Psychic Energy", energy_type: "Psychic", category: "energy" });

export const sampleHand: WireCard[] = [
  card({ name: "Munkidori", category: "pokemon" }),
  card({ name: "Dreepy", category: "pokemon" }),
  card({ name: "Iono", category: "supporter" }),
  card({ name: "Ultra Ball", category: "item" }),
  card({ name: "Buddy-Buddy Poffin", category: "item" }),
  card({ name: "Rescue Board", category: "tool" }),
  card({ name: "Area Zero Underdepths", category: "stadium" }),
  psychicEnergy(),
  psychicEnergy(),
];

export const attacker = (): WirePokemon =>
  pokemon({
    name: "Dragapult ex",
    hp: 320,
    damage: 90,
    conditions: [],
    attached: [fireEnergy(), fireEnergy()],
  });

export const defender = (): WirePokemon =>
  pokemon({ name: "Budew", hp: 70, damage: 30, conditions: ["Asleep"] });

export function side(overrides: Partial<WireSide> = {}): WireSide {
  return {
    player: 0,
    hand_count: 5,
    library_count: 42,
    prize_count: 4,
    discard: [card({ name: "Sparkling Crystal" }), card({ name: "Night Stretcher" })],
    active: attacker(),
    bench: [pokemon({ name: "Dreepy" }), pokemon({ name: "Drakloak" })],
    ...overrides,
  };
}

export const sampleView: WireView = {
  you: 0,
  current: 0,
  turn_number: 4,
  phase: "Main",
  your_hand: sampleHand,
  stadium: card({ name: "Area Zero Underdepths", category: "stadium" }),
  library_in_search: null,
  sides: [
    side({ player: 0 }),
    side({
      player: 1,
      prize_count: 5,
      active: defender(),
      bench: [pokemon({ name: "Pecharunt ex" })],
    }),
  ],
};

/** A search prompt: three cards to take plus a stop line. */
export const takeMeta: WireActionMeta[] = [
  {
    kind: "TakeCard",
    card: null,
    target: null,
    card_face: { print_id: "p1", name: "Ralts", energy_type: null, category: "pokemon" },
  },
  {
    kind: "TakeCard",
    card: null,
    target: null,
    card_face: { print_id: "p2", name: "Kirlia", energy_type: null, category: "pokemon" },
  },
  {
    kind: "TakeCard",
    card: null,
    target: null,
    card_face: { print_id: "p3", name: "Gardevoir ex", energy_type: null, category: "pokemon" },
  },
  { kind: "FinishDeciding", card: null, target: null },
];
export const takeActions = ["Take Ralts", "Take Kirlia", "Take Gardevoir ex", "Stop taking"];
