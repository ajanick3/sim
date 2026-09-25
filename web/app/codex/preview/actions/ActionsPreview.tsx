import { ActionDialog } from "../../_components/dialogs/ActionDialog";
import { BoardPreview } from "../board/BoardPreview";

export function ActionsPreview() {
  return (
    <>
      <BoardPreview />
      <ActionDialog
        title="Choose an action"
        description="Dreepy is selected. Choose one legal move."
        actions={[
          { id: 1, label: "Petty Grudge", detail: "Attack for 10 damage" },
          { id: 2, label: "Bite", detail: "Attack for 40 damage" },
          { id: 3, label: "Retreat", detail: "Choose a Pokémon from your bench" },
        ]}
      />
    </>
  );
}
