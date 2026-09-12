import Fab from "@mui/material/Fab";
import Tooltip from "@mui/material/Tooltip";

/** Every move a player makes over the board, reduced to one glyph
 *  each — the reimagined language this library speaks instead of a
 *  labeled button list. Attach is the Energy-attach action, not a
 *  Tool; a Tool has no action of its own, it just rides along with
 *  PlayTrainer. */
export type ActionKind = "attach" | "retreat" | "attack" | "ability" | "evolve";

const ACTION_ICON: Record<ActionKind, string> = {
  attach: "⚡",
  retreat: "🔄",
  attack: "⚔️",
  ability: "✨",
  evolve: "🧬",
};

/** One action as a small, round, icon-only FAB — no label sitting on
 *  the button itself. `label` still exists, for the tooltip and for
 *  anyone using a screen reader, but the button's own face is only
 *  ever the glyph. */
export function ActionFab({
  kind,
  label,
  disabled = false,
  onClick,
}: {
  kind: ActionKind;
  label: string;
  disabled?: boolean;
  onClick: () => void;
}) {
  return (
    <Tooltip title={label}>
      <span>
        <Fab
          size="small"
          color={kind === "attack" ? "error" : kind === "ability" ? "secondary" : "primary"}
          aria-label={label}
          disabled={disabled}
          onClick={onClick}
        >
          <span role="img" aria-hidden="true" style={{ fontSize: 18 }}>
            {ACTION_ICON[kind]}
          </span>
        </Fab>
      </span>
    </Tooltip>
  );
}
