import { CardFrame } from "../atoms/CardFrame";
import { CardBack } from "../atoms/CardBack";
import { CardArt } from "../atoms/CardArt";
import { CountBadge } from "../atoms/CountBadge";
import styles from "./cards.module.css";

type CardPileProps =
  | { kind: "deck"; count: number }
  | { kind: "discard"; count: number; topCardName?: string; topCardImageUrl?: string | null };

export function CardPile(props: CardPileProps) {
  const label =
    props.kind === "deck" ? `Deck, ${props.count} cards` : `Discard pile, ${props.count} cards`;
  return (
    <div className={styles.pile} data-kind={props.kind}>
      <CardFrame label={label}>
        {props.kind === "deck" ? (
          <CardBack />
        ) : props.topCardImageUrl ? (
          <CardArt src={props.topCardImageUrl} />
        ) : (
          <span className={styles.empty}>Empty</span>
        )}
      </CardFrame>
      <span className={styles.pileCount}>
        <CountBadge
          count={props.count}
          label={props.kind === "deck" ? "Deck cards" : "Discarded cards"}
        />
      </span>
      <span className={styles.pileLabel}>{props.kind === "deck" ? "Deck" : "Discard"}</span>
    </div>
  );
}
