// The shapes every Region composes from — deliberately independent of
// the live game's wire format (`web/app/view.ts`). This library draws
// pictures of a board; it does not know the engine.

export type PlayCard = {
  id: number;
  name: string;
  src?: string | null;
  hp: number;
  damage: number;
  energies?: { id: number; kind: string }[];
  tool?: string | null;
};

export type PocketCard = {
  id: number;
  name: string;
  src?: string | null;
};
