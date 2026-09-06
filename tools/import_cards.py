#!/usr/bin/env python3
"""Import the Standard-legal cards from TCGdex into one JSON artifact.

Standard is regulation marks H, I, and J. The crawl asks TCGdex for the cards
of each mark, then reads each card and keeps the fields a rules engine needs.
Prices, images, variants, and rarity are dropped: they change often and the
engine never reads them.

    python3 tools/import_cards.py [output]      # default data/cards.json

The script uses the standard library only, so the crate stays free of a
dependency it would carry only for a crawl.
"""

import json
import sys
import time
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor

API = "https://api.tcgdex.net/v2/en"
MARKS = ["H", "I", "J"]
WORKERS = 8

# What a rules engine reads. Everything else TCGdex returns is dropped.
KEEP = [
    "id", "localId", "name", "category", "regulationMark",
    "stage", "suffix", "evolveFrom", "hp", "types", "retreat",
    "weaknesses", "resistances", "attacks", "abilities",
    "trainerType", "energyType", "effect", "description",
]


def fetch(url, attempts=4):
    """Read one URL as JSON, backing off when the API pushes back."""
    for attempt in range(attempts):
        try:
            with urllib.request.urlopen(url, timeout=30) as response:
                return json.load(response)
        except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as error:
            if attempt == attempts - 1:
                raise RuntimeError(f"{url}: {error}") from error
            time.sleep(2**attempt)
    return None


def card_ids():
    """Every card id in Standard, with the mark that made it legal."""
    ids = {}
    for mark in MARKS:
        brief = fetch(f"{API}/cards?regulationMark={mark}")
        for card in brief:
            ids[card["id"]] = mark
        print(f"  mark {mark}: {len(brief)} cards", file=sys.stderr)
    return ids


def trim(card):
    """Keep the fields the engine reads, and the set the card came from."""
    kept = {key: card[key] for key in KEEP if key in card}
    kept["set"] = card.get("set", {}).get("id")
    # TCGdex holds one card whose mark is lower case. Legality reads the mark,
    # so the case is normalised here rather than at every reader.
    if "regulationMark" in kept:
        kept["regulationMark"] = kept["regulationMark"].upper()
    return kept


def main():
    output = sys.argv[1] if len(sys.argv) > 1 else "data/cards.json"

    print("Reading the card lists", file=sys.stderr)
    ids = card_ids()
    print(f"{len(ids)} cards to read", file=sys.stderr)

    started = time.time()
    with ThreadPoolExecutor(max_workers=WORKERS) as pool:
        cards = list(pool.map(lambda i: fetch(f"{API}/cards/{i}"), ids))
    print(f"read in {time.time() - started:.0f}s", file=sys.stderr)

    trimmed = sorted((trim(card) for card in cards), key=lambda c: c["id"])
    missing = [c["id"] for c in trimmed if "regulationMark" not in c]
    if missing:
        print(f"warning: {len(missing)} cards carry no mark", file=sys.stderr)

    artifact = {
        "schema": 1,
        "source": "https://api.tcgdex.net/v2/en",
        "imported": time.strftime("%Y-%m-%d"),
        "marks": MARKS,
        "count": len(trimmed),
        "cards": trimmed,
    }
    with open(output, "w", encoding="utf-8") as handle:
        json.dump(artifact, handle, ensure_ascii=False, indent=1, sort_keys=True)
        handle.write("\n")
    print(f"wrote {output}: {len(trimmed)} cards", file=sys.stderr)


if __name__ == "__main__":
    main()
