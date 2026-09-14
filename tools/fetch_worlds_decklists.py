#!/usr/bin/env python3
"""Fetch the 2026 World Championships' Day 2 decklists into decks/2026-worlds/.

The source is limitlesstcg's tournament page:

    https://limitlesstcg.com/tournaments/515/decklists

It embeds every decklist it has directly in one page — no per-player fetch
needed — and it has one only for a player who reached a Day 2 standing.
Someone eliminated in Swiss has no decklist here at all, so 143 is this
source's ceiling, not a number this script chose.

    python3 tools/fetch_worlds_decklists.py

Output matches the format already in decks/2026-worlds/: three sections
(Pokémon/Trainer/Energy), each line `<count> <name> <set> <number>`, card
names with diacritics stripped, numbers zero-padded to 3 digits. Filenames
are `<placement>-<player-slug>.txt`, zero-padded to 3 digits — the field
runs past 99. A file that already exists is left alone, so a re-run only
fills in what is still missing.
"""

import html
import re
import sys
import unicodedata
import urllib.request

URL = "https://limitlesstcg.com/tournaments/515/decklists"
UA = (
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 "
    "(KHTML, like Gecko) Chrome/120.0 Safari/537.36"
)
OUT_DIR = "decks/2026-worlds"

BLOCK_RE = re.compile(r'<div class="tournament-decklist">')
ORDINAL_RE = re.compile(r'data-toggle data-target="decklist-\d+">(\d+)\w+ ([^<]+)<')
COLUMN_RE = re.compile(
    r'<div class="decklist-column-heading">([^(<]+)\((\d+)\)</div>(.*?)</div>\s*</div>',
    re.S,
)
CARD_RE = re.compile(
    r'<div class="decklist-card" data-set="([^"]*)" data-number="([^"]*)"[^>]*>'
    r'.*?<span class="card-count">(\d+)</span>\s*'
    r'<span class="card-name">([^<]+)</span>',
    re.S,
)


def fetch(url: str) -> str:
    request = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read().decode("utf-8")


# Letters NFKD does not decompose into a base letter plus a combining mark —
# each is its own code point, not a diacritic on a plainer one. A name from
# an international player's field is where these show up.
NON_DECOMPOSING = str.maketrans(
    {"Ł": "L", "ł": "l", "Đ": "D", "đ": "d", "Ø": "O", "ø": "o"}
)


def strip_diacritics(name: str) -> str:
    name = name.translate(NON_DECOMPOSING)
    normalized = unicodedata.normalize("NFKD", name)
    return "".join(c for c in normalized if not unicodedata.combining(c))


def slugify(name: str) -> str:
    ascii_name = strip_diacritics(name)
    ascii_name = re.sub(r"[^a-zA-Z0-9]+", "-", ascii_name).strip("-")
    return ascii_name.lower()


def parse_deck(block: str) -> str:
    out = []
    for heading, count, body in COLUMN_RE.findall(block):
        out.append(f"{heading.strip()}: {count}")
        for set_code, number, card_count, card_name in CARD_RE.findall(body):
            name = strip_diacritics(html.unescape(card_name).strip())
            out.append(f"{card_count} {name} {set_code} {int(number):03d}")
        out.append("")
    return "\n".join(out).rstrip("\n") + "\n"


def main():
    import glob
    import os

    text = fetch(URL)
    blocks = BLOCK_RE.split(text)[1:]

    os.makedirs(OUT_DIR, exist_ok=True)
    written, skipped, failed = 0, 0, []

    for block in blocks:
        m = ORDINAL_RE.search(block)
        if not m:
            print("  could not read a standing/name header in a block", file=sys.stderr)
            continue
        placement, name = int(m.group(1)), html.unescape(m.group(2)).strip()

        # The placement, not the slug, is the unique key: a committed file's
        # slug may already spell the player's name shorter than this page
        # does (a nickname kept from an earlier, hand-checked import), and
        # that spelling should win over re-deriving one from the live page.
        if glob.glob(f"{OUT_DIR}/{placement:03d}-*.txt"):
            skipped += 1
            continue

        path = f"{OUT_DIR}/{placement:03d}-{slugify(name)}.txt"

        deck_text = parse_deck(block)
        if not deck_text.strip():
            failed.append(name)
            print(f"  no cards parsed for {placement} {name}", file=sys.stderr)
            continue

        with open(path, "w", encoding="utf-8") as handle:
            handle.write(deck_text)
        written += 1

    print(f"wrote {written}, skipped {skipped} already present", file=sys.stderr)
    if failed:
        print(f"failed: {failed}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
