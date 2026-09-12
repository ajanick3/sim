/** The HP pill a card prints top-right — a plain element now, not
 *  self-positioned; the caller (`CardOverlay`) places it top-right by
 *  putting it in a `justify-content: flex-end` flex row, not by
 *  giving it its own coordinates. Not a bar despite the name a health
 *  indicator usually gets — this game shows remaining HP as a number,
 *  the way the physical card does, never a depleting bar. */
export function HealthBar({ hp, tiny = false }: { hp: number; tiny?: boolean }) {
  return (
    <span
      style={{
        background: "rgba(0,0,0,0.75)",
        color: "#fff",
        fontWeight: 700,
        fontSize: tiny ? 8 : 10,
        borderRadius: 3,
        padding: "0 4px",
        lineHeight: 1.4,
      }}
    >
      {hp}
    </span>
  );
}
