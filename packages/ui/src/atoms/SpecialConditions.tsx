/** `WirePokemon.conditions`, laid over the card's middle — Poisoned,
 *  Asleep, Paralyzed, and the rest, joined by a comma when more than
 *  one applies at once. Sits in `CardOverlay`'s flexible middle slot
 *  now, not pinned to `top: 50%` by its own coordinates. */
export function SpecialConditions({ conditions }: { conditions: string[] }) {
  if (conditions.length === 0) return null;
  return (
    <span
      style={{
        width: "100%",
        background: "rgba(0,0,0,0.6)",
        color: "#f0a24b",
        fontSize: 8,
        textAlign: "center",
      }}
    >
      {conditions.join(", ")}
    </span>
  );
}
