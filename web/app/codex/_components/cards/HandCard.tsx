import type { CardState } from "../atoms/CardFrame";
import { CardFrame } from "../atoms/CardFrame";
import { CardArt } from "../atoms/CardArt";

export function HandCard({
  name,
  imageUrl,
  state = "resting",
  disabled = false,
  onSelect,
}: {
  name: string;
  imageUrl: string | null;
  state?: CardState;
  disabled?: boolean;
  onSelect: () => void;
}) {
  return (
    <CardFrame label={name} interactive state={state} disabled={disabled} onClick={onSelect}>
      <CardArt src={imageUrl} />
    </CardFrame>
  );
}
