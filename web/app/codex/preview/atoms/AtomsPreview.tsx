"use client";

import { useState, type ReactNode } from "react";
import { CardFrame } from "../../_components/atoms/CardFrame";
import { CardArt } from "../../_components/atoms/CardArt";
import { DREEPY_ART, CRUSHING_HAMMER_ART } from "../../_fixtures/cardArt";
import { CardBack } from "../../_components/atoms/CardBack";
import { CardSlot } from "../../_components/atoms/CardSlot";
import { PrizeMarker } from "../../_components/atoms/PrizeMarker";
import { CountBadge } from "../../_components/atoms/CountBadge";
import { HealthBadge } from "../../_components/atoms/HealthBadge";
import { DamageCounter } from "../../_components/atoms/DamageCounter";
import { IconButton } from "../../_components/atoms/IconButton";
import theme from "../../_styles/theme.module.css";
import styles from "./preview.module.css";

function Example({ label, children }: { label: string; children: ReactNode }) {
  return (
    <figure className={styles.example}>
      <div>{children}</div>
      <figcaption>{label}</figcaption>
    </figure>
  );
}

export function AtomsPreview() {
  const [selected, setSelected] = useState(false);
  const [message, setMessage] = useState(
    "Select the first card or place it in the highlighted slot.",
  );
  return (
    <main className={`${theme.theme} ${styles.page}`}>
      <header className={styles.header}>
        <div>
          <span className={styles.eyebrow}>SIM / COMPONENT REVIEW / 01</span>
          <h1>Small pieces. Same board.</h1>
          <p>Real React components • Pokémon + Trainer art • Fixed 245:337 ratio</p>
        </div>
        <span className={styles.stage}>Atoms</span>
      </header>

      <section className={styles.section} aria-labelledby="cards-heading">
        <div className={styles.sectionTitle}>
          <span>01</span>
          <div>
            <h2 id="cards-heading">Card frame + artwork</h2>
            <p>One frame for every card. Selection and valid targets have distinct cues.</p>
          </div>
        </div>
        <div className={styles.cards}>
          <Example label="Resting / tap to select">
            <CardFrame
              label="Select Dreepy"
              interactive
              state={selected ? "selected" : "resting"}
              onClick={() => {
                setSelected(!selected);
                setMessage(selected ? "Selection cleared." : "Dreepy selected.");
              }}
            >
              <CardArt src={DREEPY_ART} />
            </CardFrame>
          </Example>
          <Example label="Selected">
            <CardFrame label="Selected Dreepy" state="selected">
              <CardArt src={DREEPY_ART} />
            </CardFrame>
          </Example>
          <Example label="Valid target">
            <CardFrame
              label="Place card on target"
              interactive
              state="target"
              onClick={() => setMessage("Target selected.")}
            >
              <CardArt src={DREEPY_ART} />
            </CardFrame>
          </Example>
          <Example label="Trainer / unavailable">
            <CardFrame label="Unavailable Crushing Hammer" interactive state="unavailable">
              <CardArt src={CRUSHING_HAMMER_ART} />
            </CardFrame>
          </Example>
          <Example label="Card back">
            <CardFrame label="Face-down card">
              <CardBack />
            </CardFrame>
          </Example>
          <Example label="Missing artwork">
            <CardFrame label="Card with unavailable artwork">
              <CardArt src={null} />
            </CardFrame>
          </Example>
        </div>
      </section>

      <div className={styles.split}>
        <section className={styles.section} aria-labelledby="slots-heading">
          <div className={styles.sectionTitle}>
            <span>02</span>
            <div>
              <h2 id="slots-heading">Empty slots</h2>
              <p>Card-sized spaces, with an explicit placement cue.</p>
            </div>
          </div>
          <div className={styles.slots}>
            <Example label="Empty">
              <CardSlot label="Empty bench slot" />
            </Example>
            <Example label="Place here">
              <CardSlot
                label="Place card on bench"
                onPlace={() => {
                  setSelected(false);
                  setMessage("Dreepy placed on the bench (preview).");
                }}
              />
            </Example>
            <Example label="Disabled">
              <CardSlot label="Placement unavailable" onPlace={() => {}} disabled />
            </Example>
          </div>
        </section>
        <section className={styles.section} aria-labelledby="details-heading">
          <div className={styles.sectionTitle}>
            <span>03</span>
            <div>
              <h2 id="details-heading">Markers + badges</h2>
              <p>Small indicators, without duplicate prize counts.</p>
            </div>
          </div>
          <div className={styles.badges}>
            <Example label="Prize remaining">
              <PrizeMarker />
            </Example>
            <Example label="Prize taken">
              <PrizeMarker taken />
            </Example>
            <Example label="Deck count">
              <CountBadge count={24} label="Deck cards" />
            </Example>
            <Example label="Your HP">
              <HealthBadge hp={70} />
            </Example>
            <Example label="Opponent HP">
              <HealthBadge hp={70} opponent />
            </Example>
            <Example label="Damage">
              <DamageCounter damage={30} />
            </Example>
          </div>
        </section>
      </div>

      <section className={styles.section} aria-labelledby="controls-heading">
        <div className={styles.sectionTitle}>
          <span>04</span>
          <div>
            <h2 id="controls-heading">Controls + keyboard focus</h2>
            <p>44px icon buttons. Yellow focus outline; cyan card selection.</p>
          </div>
        </div>
        <div className={styles.controls}>
          <IconButton
            label="Open menu"
            onClick={() => setMessage("Menu button activated (preview).")}
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </IconButton>
          <IconButton
            label="Close preview"
            onClick={() => setMessage("Close button activated (preview).")}
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="m6 6 12 12M6 18 18 6" />
            </svg>
          </IconButton>
          <IconButton label="Unavailable menu" disabled>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </IconButton>
          <p role="status" className={styles.status}>
            {message}
          </p>
        </div>
      </section>
      <footer className={styles.footer}>
        Next: Pokémon cards, hand cards, piles, and the six-marker prize zone.
      </footer>
    </main>
  );
}
