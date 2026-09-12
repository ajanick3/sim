/** One Prize, drawn as a Poké Ball rather than a colored pip — the
 *  actual token real Prize cards are represented by. `taken` dims and
 *  desaturates it: a Prize already paid out reads as spent, not just
 *  a different color. An original drawing, not a traced image, so it
 *  scales crisp at any size and needs no asset file. */
export function PokeBall({ taken = false, size = 20 }: { taken?: boolean; size?: number }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      role="img"
      aria-label={taken ? "Prize taken" : "Prize remaining"}
      style={{
        opacity: taken ? 0.35 : 1,
        filter: taken ? "grayscale(1)" : undefined,
        flex: "none",
      }}
    >
      <circle cx="12" cy="12" r="10.5" fill="#fff" stroke="#1a1a1a" strokeWidth="1.2" />
      <path d="M1.5 12a10.5 10.5 0 0 1 21 0z" fill="#e3350d" stroke="#1a1a1a" strokeWidth="1.2" />
      <rect x="1.5" y="11.2" width="21" height="1.6" fill="#1a1a1a" />
      <circle cx="12" cy="12" r="3.2" fill="#fff" stroke="#1a1a1a" strokeWidth="1.2" />
      <circle cx="12" cy="12" r="1.4" fill="#fff" stroke="#1a1a1a" strokeWidth="0.8" />
    </svg>
  );
}
