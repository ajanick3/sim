import Stack from "@mui/material/Stack";
import { ActionFab, type ActionKind } from "../atoms/ActionFab";

export type BoardAction = {
  id: number;
  kind: ActionKind;
  label: string;
  disabled?: boolean;
};

/** Every move on offer, as a row of `ActionFab`s — the reimagined
 *  replacement for a grouped, labeled action list. A player reads the
 *  board by its icons, not by a paragraph of button text. */
export function ActionBar({ actions, onAct }: { actions: BoardAction[]; onAct: (id: number) => void }) {
  return (
    <Stack direction="row" spacing={1}>
      {actions.map((a) => (
        <ActionFab key={a.id} kind={a.kind} label={a.label} disabled={a.disabled} onClick={() => onAct(a.id)} />
      ))}
    </Stack>
  );
}
