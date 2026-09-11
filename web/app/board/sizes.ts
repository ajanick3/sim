// The card sizes the board uses, named. Every card frame picks one of
// these instead of spelling out `w-[…] h-[…]`.

export type CardSize = "active" | "activeFar" | "fluid" | "pile" | "picker" | "stadium";

/** Tailwind width/height classes for each named size. `active` and
 *  `activeFar` are the top-of-print slice, at your own scale and the
 *  opponent's smaller "farther away" one. `fluid` has no size of its
 *  own — it fills whatever grid column it sits in, so the Bench and
 *  Hand can both scale their card size to the screen instead of
 *  carrying a fixed pixel width. Every other entry is the whole card
 *  (roughly 5:7 portrait) at a fixed size: `pile` (a deck or discard
 *  stack), `picker` (a search grid) and `stadium` (the Stadium slot, a
 *  little larger to carry its text). */
export const CARD_SIZE: Record<CardSize, string> = {
  active: "w-[168px] h-[235px]",
  activeFar: "w-[126px] h-[176px]",
  fluid: "w-full aspect-[5/7]",
  pile: "w-[46px] h-[64px]",
  picker: "w-[132px] h-[184px]",
  stadium: "w-[72px] h-[100px]",
};
