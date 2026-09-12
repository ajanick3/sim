/** A damage counter dropped on the illustration — a coin, per the
 *  physical game. Used to sit at a per-Pokémon scattered spot fixed
 *  by absolute `top`/`left` coordinates; a flex layout can't place an
 *  item at an arbitrary point, so it now just centres in whatever
 *  space `CardOverlay`'s middle row gives it — a real, accepted
 *  simplification, not an oversight. */
export function DamageCounter({ damage, large = false }: { damage: number; large?: boolean }) {
  const size = large ? 44 : 24;
  return (
    <span
      style={{
        width: size,
        height: size,
        display: "grid",
        placeItems: "center",
        borderRadius: "50%",
        border: "2px solid rgba(0,0,0,0.5)",
        background: "#f97316",
        color: "#000",
        fontWeight: 900,
        fontSize: large ? 16 : 10,
        boxShadow: "0 2px 5px rgba(0,0,0,0.6)",
        flex: "none",
      }}
    >
      {damage}
    </span>
  );
}
