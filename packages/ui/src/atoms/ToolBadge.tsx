/** A Tool attached to a Pokémon — one card carries at most one, so
 *  this is a single small badge, not a list. Placed by the caller
 *  (`CardOverlay`'s bottom-start slot), not by its own coordinates. */
export function ToolBadge({ name }: { name: string }) {
  return (
    <span
      title={name}
      style={{
        display: "grid",
        placeItems: "center",
        width: 16,
        height: 16,
        borderRadius: "50%",
        background: "rgba(0,0,0,0.6)",
        fontSize: 10,
        lineHeight: 1,
        flex: "none",
      }}
    >
      🔧
    </span>
  );
}
