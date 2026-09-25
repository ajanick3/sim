"use client";

import { useState } from "react";
import { PokemonCard } from "../../_components/cards/PokemonCard";
import { CardPile } from "../../_components/cards/CardPile";
import { HandGrid, type HandGridCard } from "../../_components/regions/HandGrid";
import { PrizeZone } from "../../_components/regions/PrizeZone";
import { DREEPY_ART, CRUSHING_HAMMER_ART } from "../../_fixtures/cardArt";
import theme from "../../_styles/theme.module.css";
import styles from "./preview.module.css";

const hand: HandGridCard[] = Array.from({ length: 10 }, (_, index) => ({
  id: index + 1,
  name: index % 3 === 0 ? "Dreepy" : "Crushing Hammer",
  imageUrl: index % 3 === 0 ? DREEPY_ART : CRUSHING_HAMMER_ART,
  playable: index !== 8,
}));

export function ComponentsPreview() {
  const [selectedId, setSelectedId] = useState<number | null>(3);
  return (
    <main className={`${theme.theme} ${styles.page}`}>
      <header>
        <span>SIM / COMPONENT REVIEW / 02</span>
        <h1>Cards become game pieces.</h1>
        <p>Dreepy occupies Pokémon zones. Crushing Hammer stays in the hand and discard.</p>
      </header>
      <section className={styles.panel}>
        <h2>Pokémon cards</h2>
        <div className={styles.pokemonRow}>
          <figure>
            <div>
              <PokemonCard name="Dreepy" imageUrl={DREEPY_ART} hp={70} onSelect={() => {}} />
            </div>
            <figcaption>Active</figcaption>
          </figure>
          <figure>
            <div>
              <PokemonCard
                name="Dreepy"
                imageUrl={DREEPY_ART}
                hp={70}
                damage={30}
                opponent
                onSelect={() => {}}
              />
            </div>
            <figcaption>Opponent + damage</figcaption>
          </figure>
          <figure>
            <div>
              <PokemonCard
                name="Dreepy"
                imageUrl={DREEPY_ART}
                hp={70}
                state="target"
                onSelect={() => {}}
              />
            </div>
            <figcaption>Valid target</figcaption>
          </figure>
        </div>
      </section>
      <div className={styles.columns}>
        <section className={styles.panel}>
          <h2>Deck + discard</h2>
          <div className={styles.piles}>
            <div>
              <CardPile kind="deck" count={24} />
            </div>
            <div>
              <CardPile
                kind="discard"
                count={6}
                topCardName="Crushing Hammer"
                topCardImageUrl={CRUSHING_HAMMER_ART}
              />
            </div>
          </div>
        </section>
        <section className={styles.panel}>
          <h2>Six prize markers</h2>
          <div className={styles.prizeExamples}>
            <div>
              <PrizeZone remaining={6} />
              <span>6 remaining</span>
            </div>
            <div>
              <PrizeZone remaining={3} />
              <span>3 remaining</span>
            </div>
          </div>
        </section>
      </div>
      <section className={`${styles.panel} ${styles.handPanel}`}>
        <div className={styles.handTitle}>
          <div>
            <h2>Two-row hand</h2>
            <p>Five columns. Ten cards. Tap any playable card.</p>
          </div>
          <output>{selectedId ? `Card ${selectedId} selected` : "No selection"}</output>
        </div>
        <HandGrid
          cards={hand}
          selectedId={selectedId}
          onSelect={(id) => setSelectedId((current) => (current === id ? null : id))}
        />
      </section>
    </main>
  );
}
