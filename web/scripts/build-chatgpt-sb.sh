#!/bin/sh
# packages/chatgpt is its own, separately-evolving package — not always
# merged to whatever commit web/ itself is built from. Build its
# Storybook into public/chatgpt when it's there; skip quietly, not
# fail the whole site's build, when it isn't.
set -e

DIR="../packages/chatgpt"

if [ ! -d "$DIR" ]; then
  echo "packages/chatgpt not present — skipping the /chatgpt build."
  exit 0
fi

cd "$DIR"
# Vercel's build runs with NODE_ENV=production, under which plain
# `pnpm install` silently skips devDependencies — Storybook, Vite, and
# everything this build actually needs live there, not in
# `dependencies`. --prod=false overrides that regardless of NODE_ENV.
pnpm install --prod=false
# The local binary directly, not `pnpm run build-storybook` — a nested
# run here would pick up web/'s own `storybook` off PATH (a different
# major version) instead of this package's own, since the outer
# script's bin dir is already on it. This binary is unambiguous.
./node_modules/.bin/storybook build --output-dir ../../web/public/chatgpt --quiet
