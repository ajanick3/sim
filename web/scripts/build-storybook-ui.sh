#!/bin/sh
# packages/ui is its own, separately-evolving package — not always
# merged to whatever commit web/ itself is built from. Build its
# Storybook into public/storybook-ui when it's there; skip quietly,
# not fail the whole site's build, when it isn't.
set -e

DIR="../packages/ui"

if [ ! -d "$DIR" ]; then
  echo "packages/ui not present — skipping the /storybook-ui build."
  exit 0
fi

(cd "$DIR" && npm install --no-audit --no-fund && npm run build-storybook -- --output-dir ../../web/public/storybook-ui --quiet)
