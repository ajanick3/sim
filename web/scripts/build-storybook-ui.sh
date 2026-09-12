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

cd "$DIR"
# Vercel's build runs with NODE_ENV=production, under which plain
# `npm install` silently skips devDependencies — Storybook, Vite, and
# everything this build actually needs live there, not in
# `dependencies`. --include=dev overrides that regardless of NODE_ENV.
npm install --no-audit --no-fund --include=dev
# The local binary directly, not `npm run build-storybook` — a nested
# `npm run` here picks up web/'s own `storybook` off PATH (a different
# major version) instead of this package's own, since npm prepends
# the *outer* script's bin dir too. This binary is unambiguous.
./node_modules/.bin/storybook build --output-dir ../../web/public/storybook-ui --quiet
