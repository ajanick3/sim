#!/usr/bin/env python3
"""Check that matching a Trainer's effect by its printed name is safe.

`known_trainer` in `src/import.rs` reads one effect for every card of a
name. That is safe only where every print of the name carries the same
rules text — a comparison that must ignore whitespace, since TCGdex prints
the same card with a single space in one place and a double space in
another. A raw-text comparison reports those as two different cards; this
one does not.

    python3 tools/check_trainer_name_safety.py [data/cards.json]

Exits 1 and lists every name with more than one distinct effect, once
whitespace is collapsed. Exits 0, silently, when every name is safe.
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

    by_name = {}
    for card in cards:
        if card.get("category") != "Trainer":
            continue
        by_name.setdefault(card["name"], set()).add(normalize(card.get("effect")))

    unsafe = {name: texts for name, texts in by_name.items() if len(texts) > 1}
    if not unsafe:
        print(f"every Trainer name in {path} is safe to match by name")
        return 0

    print("these names carry more than one effect, once whitespace is ignored:")
    for name, texts in sorted(unsafe.items()):
        print(f"\n{name}:")
        for text in sorted(texts):
            print(f"  {text!r}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
