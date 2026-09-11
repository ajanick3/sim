// The card sizes the board uses, named. Every card frame picks one of
// these instead of spelling out `w-[…] h-[…]`.

export type CardSize = "active" | "activeFar" | "benchFluid" | "pile" | "picker" | "stadium";

/** Tailwind width/height classes for each named size. `active` and
 *  `activeFar` are the top-of-the-print slice, at your own scale and the
 *  opponent's smaller "farther away" one. `benchFluid` has no size of
 *  its own — it fills whatever grid column it sits in — so every other
 *  entry here is the whole card (roughly 5:7 portrait): `pile` (a deck
 *  or discard stack), `picker` (a search grid or the Hand) and
 *  `stadium` (the Stadium slot, a little larger to carry its text). */
export const CARD_SIZE: Record<CardSize, string> = {
  active: "w-[200px] h-[118px]",
  activeFar: "w-[150px] h-[89px]",
  benchFluid: "w-full aspect-[5/7]",
  pile: "w-[46px] h-[64px]",
  picker: "w-[132px] h-[184px]",
  stadium: "w-[72px] h-[100px]",
};
