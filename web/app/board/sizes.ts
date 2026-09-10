// The card sizes the board uses, named. Every card frame picks one of
// these instead of spelling out `w-[…] h-[…]`.

export type CardSize = "active" | "bench" | "benchSmall" | "hand" | "pile" | "picker";

/** Tailwind width/height classes for each named size. `active`, `bench`
 *  and `hand` are the same top-of-the-print slice at different zooms;
 *  `pile` and `picker` are the whole card (roughly 5:7 portrait). */
export const CARD_SIZE: Record<CardSize, string> = {
  active: "w-[200px] h-[118px]",
  bench: "w-[120px] h-[71px]",
  benchSmall: "w-[92px] h-[54px]",
  hand: "w-[116px] h-[82px]",
  pile: "w-[46px] h-[64px]",
  picker: "w-[132px] h-[184px]",
};
