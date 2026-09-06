# Spec: read and check a decklist

## Problem

There is no way to give the simulator a real decklist. `GameState::new` takes
indices into a card database, and the only builder hardcodes the three
synthetic cards. An operator with a tournament list has nowhere to put it.

## Solution

Read the text format a player already has — the export from the official
client, `4 Pikachu ex SVI 63` — match each line to a card in the artifact, and
check the list against deck construction.

## User stories

- An operator pastes a decklist and is told whether it is legal.
- An operator is told which cards the engine can play, so a real deck becomes a
  coverage target.
- A card the checker cannot match is named, never silently skipped.

## Implementation decisions

Match on the set abbreviation and the number, which is what a list carries.
TCGdex holds the official abbreviation per set, so the importer writes a set
table into the artifact.

Basic Energy is not in the artifact, because it carries no regulation mark. The
checker knows the nine basic Energy by name.

## Testing decisions

Tests parse real lines against the real artifact, so a format change fails
loudly.

## Out of scope

Playing the deck. Deck construction rule 3, one ACE SPEC per deck, which the
data cannot express — the checker says so rather than guessing.
