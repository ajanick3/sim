// The card sizes the board uses, named. Every card frame picks one of
// these instead of spelling out `w-[…] h-[…]`.

export type CardSize = "active" | "bench" | "benchSmall" | "hand" | "pile" | "picker";

/** Tailwind width/height classes for each named size. `active` and
 *  `hand` are cropped to the top of the print (landscape-ish); `bench`,
 *  `pile` and `picker` keep the whole card (roughly 5:7 portrait). */
export const CARD_SIZE: Record<CardSize, string> = {
  active: "w-[200px] h-[118px]",
  bench: "w-[96px] min-h-[134px]",
  benchSmall: "w-[64px] min-h-[90px]",
  hand: "w-[116px] h-[82px]",
  pile: "w-[46px] h-[64px]",
  picker: "w-[132px] h-[184px]",
};
