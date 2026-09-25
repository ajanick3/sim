import { Battlefield } from "../../_components/regions/Battlefield";
import type { BenchPokemon } from "../../_components/regions/BenchRow";
import type { HandGridCard } from "../../_components/regions/HandGrid";
import { CRUSHING_HAMMER_ART, DREEPY_ART } from "../../_fixtures/cardArt";
import theme from "../../_styles/theme.module.css";
import styles from "./preview.module.css";

const pokemon = (id: number, damage = 0): BenchPokemon => ({
  id,
  name: "Dreepy",
  imageUrl: DREEPY_ART,
  hp: 70 - damage,
  damage,
});
const hand: HandGridCard[] = Array.from({ length: 10 }, (_, index) => ({
  id: index + 1,
  name: index % 3 === 0 ? "Dreepy" : "Crushing Hammer",
  imageUrl: index % 3 === 0 ? DREEPY_ART : CRUSHING_HAMMER_ART,
}));

export function BoardPreview() {
  return (
    <div className={`${theme.theme} ${styles.shell}`}>
      <Battlefield
        active={pokemon(1, 10)}
        opponentActive={pokemon(2, 30)}
        bench={[pokemon(3), pokemon(4, 20), pokemon(5)]}
        opponentBench={[pokemon(6), pokemon(7), pokemon(8, 10), pokemon(9)]}
        hand={hand}
        deckCount={24}
        opponentDeckCount={31}
        discardCount={6}
        opponentDiscardCount={3}
        discardImageUrl={CRUSHING_HAMMER_ART}
        opponentDiscardImageUrl={CRUSHING_HAMMER_ART}
        prizesRemaining={4}
        opponentPrizesRemaining={5}
      />
    </div>
  );
}
