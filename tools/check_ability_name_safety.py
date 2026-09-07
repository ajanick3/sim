#!/usr/bin/env python3
"""Check that matching an Ability's effect by species and name is safe.

`known_ability` in `src/import.rs` reads one effect for every
`(pokemon_name, ability_name)` pair. That is safe only where every print
of that species carrying that Ability name prints the same rules text —
a comparison that must ignore whitespace, the same way
`check_trainer_name_safety.py` does for a Trainer's name alone.

Unlike a Trainer, an Ability is keyed by species *and* name together, so
two different species sharing an Ability name (`Kadabra`'s and
`Alakazam`'s "Psychic Draw") are never a collision here — only the same
species printing the same Ability name two different ways would be.

    python3 tools/check_ability_name_safety.py [data/cards.json]

Exits 1 and lists every (species, Ability name) pair with more than one
distinct effect, once whitespace is collapsed. Exits 0, silently, when
every pair is safe.
"""

import json
import re
import sys


def normalize(text):
    """Collapse runs of whitespace so a double space is not a difference."""
    return re.sub(r"\s+", " ", text or "").strip()


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else "data/cards.json"
    with open(path, encoding="utf-8") as f:
        cards = json.load(f)["cards"]

    by_pair = {}
    for card in cards:
        if card.get("category") != "Pokemon":
            continue
        for ability in card.get("abilities") or []:
            key = (card["name"], ability.get("name"))
            by_pair.setdefault(key, set()).add(normalize(ability.get("effect")))

    unsafe = {pair: texts for pair, texts in by_pair.items() if len(texts) > 1}
    if not unsafe:
        print(f"every (species, Ability name) pair in {path} is safe to match by name")
        return

    for (species, name), texts in sorted(unsafe.items()):
        print(f"{species} — {name}:")
        for text in sorted(texts):
            print(f"  {text!r}")
    sys.exit(1)


if __name__ == "__main__":
    main()
