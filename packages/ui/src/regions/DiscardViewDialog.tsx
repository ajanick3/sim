import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import IconButton from "@mui/material/IconButton";
import Box from "@mui/material/Box";
import { Card } from "../primitives/Card";
import type { PocketCard } from "./types";

/** Every card in one side's discard, laid out for browsing — a real
 *  MUI Dialog rather than the hand-rolled overlay the Tailwind board
 *  used, so focus trapping and Escape-to-close come for free. */
export function DiscardViewDialog({
  open,
  label,
  cards,
  onClose,
}: {
  open: boolean;
  label: string;
  cards: PocketCard[];
  onClose: () => void;
}) {
  return (
    <Dialog open={open} onClose={onClose} maxWidth="sm" fullWidth>
      <DialogTitle sx={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
        {label} ({cards.length})
        <IconButton onClick={onClose} aria-label="Close" size="small">
          ✕
        </IconButton>
      </DialogTitle>
      <DialogContent>
        <Box sx={{ display: "grid", gridTemplateColumns: "repeat(5, 1fr)", gap: 1 }}>
          {cards.length === 0 && <Box sx={{ color: "text.secondary" }}>empty</Box>}
          {cards.map((c) => (
            <Card key={c.id} size="picker" fluid src={c.src} name={c.name} />
          ))}
        </Box>
      </DialogContent>
    </Dialog>
  );
}
