# Rare candy and the evolution chain

Type: task
Status: ready-for-agent

`Rare Candy` puts a Stage 2 from hand straight onto a Basic in play,
skipping the Stage 1. 40 slots.

`evolve_from` holds one link: a Stage 2 names the Stage 1 it comes from, never
the Basic beneath it. Answering "does this Stage 2 evolve from that Basic"
means walking two links, and the middle card need not be in the deck at all —
only in the pool.

- [ ] The chain from a Basic to a Stage 2 can be asked about
- [ ] A recorded decision on where the chain is walked, at load or at play
- [ ] `Rare Candy` plays, and refuses a Stage 2 that does not belong
