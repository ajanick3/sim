#!/usr/bin/env python3
"""Fetch the 2026 World Championships decklists into decks/2026-worlds/.

The source is the operator's own site, which serves each list as plain text
at one URL per player. A default `urllib` request gets a 403 from the site's
bot protection; a browser User-Agent does not, so this sends one.

    python3 tools/fetch_worlds_decks.py

Player order is placement, 1st through 64th; the output file is named
`<placement>-<player-slug>.txt`, zero-padded to two digits.
"""

import sys
import time
import urllib.error
import urllib.request

BASE = "https://pkmn.nickaja.rocks/tournaments/2026-world-championships/player"
UA = (
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 "
    "(KHTML, like Gecko) Chrome/120.0 Safari/537.36"
)
OUT_DIR = "decks/2026-worlds"

# Placement order, 1st through 64th.
PLACEMENTS = [
    "andrew-hedrick", "diego-cassiraga", "brent-tonisson", "henry-chao",
    "rune-heiremans", "satoshi-matsui", "mateusz-laszkiewicz", "nathan-spry",
    "ojvind-svinhufvud", "kian-amini", "calvin-connor", "henry-thompson",
    "cali-white", "hui-yuan-huang", "grant-walworth", "alberto-cortes",
    "keito-arai", "alex-schemanske", "angus-johnson", "natalie-millar",
    "cerys-jones", "aaron-arturo-aguirre-osuna", "marco-cifuentes",
    "yerco-valencia", "chun-chit-cheung", "michele-schiraldi",
    "nina-eisenblatter", "alessio-cefola", "jackson-ford", "gan-pee-wei-jun",
    "aarni-karjala", "makani-tran", "aleksi-jantti", "federico-nattkemper",
    "minho-song", "stephane-ivanoff", "boming-wang", "rahul-reddy",
    "emiliano-zapata", "katy-montgomerie", "guilherme-stroschon",
    "daniel-merlo", "tomi-markkula", "kazuki-yuasa", "james-kowalski",
    "rajveer-singh", "hugo-de-torres", "joey-gaffney", "david-zheng",
    "shih-wei-chen", "christian-labella", "nathan-ginsburg", "benjamin-pham",
    "yoshiyuki-yamaguchi", "joji-koyama", "kensuke-miyagi", "piper-lepine",
    "yu-zheng-cha", "jose-lopez", "liam-halliburton", "jack-wong",
    "james-cox", "marco-garcia", "noah-sakadjian",
]


def fetch(slug: str) -> str:
    request = urllib.request.Request(f"{BASE}/{slug}/list", headers={"User-Agent": UA})
    with urllib.request.urlopen(request, timeout=30) as response:
        return response.read().decode("utf-8")


def main():
    import os

    os.makedirs(OUT_DIR, exist_ok=True)
    failures = []
    for placement, slug in enumerate(PLACEMENTS, start=1):
        path = f"{OUT_DIR}/{placement:02d}-{slug}.txt"
        try:
            text = fetch(slug)
        except urllib.error.URLError as error:
            print(f"  FAILED {slug}: {error}", file=sys.stderr)
            failures.append(slug)
            continue
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text if text.endswith("\n") else text + "\n")
        time.sleep(0.15)

    print(f"wrote {len(PLACEMENTS) - len(failures)} of {len(PLACEMENTS)} decks", file=sys.stderr)
    if failures:
        print(f"failed: {failures}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
