// The card sizes the board uses, named. Every card frame picks one of
// these instead of spelling out `w-[…] h-[…]`.

export type CardSize =
  | "active"
  | "activeFar"
  | "activeCard"
  | "activeFarCard"
  | "fluid"
  | "pile"
  | "picker"
  | "stadium";

/** Tailwind width/height classes for each named size. `active` and
 *  `activeFar` are the top-of-print slice, at your own scale and the
 *  opponent's smaller "farther away" one. `activeCard` and
 *  `activeFarCard` are the same pair for the `card-forward` board
 *  variant (PR #342, routed at `/342`), which draws the whole card
 *  there instead of a sliver — bigger, and shaped like the portrait
 *  every other size already is. `fluid` has no size of its own — it
 *  fills whatever grid column it sits in, so the Bench and Hand can
 *  both scale their card size to the screen instead of carrying a
 *  fixed pixel width. Every other entry is the whole card (roughly
 *  5:7 portrait) at a fixed size: `pile` (a deck or discard stack),
 *  `picker` (a search grid) and `stadium` (the Stadium slot, a little
 *  larger to carry its text). */
export const CARD_SIZE: Record<CardSize, string> = {
  active: "w-[200px] h-[118px]",
  activeFar: "w-[150px] h-[89px]",
  activeCard: "w-[168px] h-[235px]",
  activeFarCard: "w-[126px] h-[176px]",
  fluid: "w-full aspect-[5/7]",
  pile: "w-[46px] h-[64px]",
  picker: "w-[132px] h-[184px]",
  stadium: "w-[72px] h-[100px]",
};
