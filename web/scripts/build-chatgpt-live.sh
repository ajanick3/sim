#!/bin/sh
# packages/chatgpt's own live-play app (src/live/) — the real engine,
# drawn with its actual components in complete isolation (its own
# React 18, its own MUI, no shared runtime with this Next.js app).
# Built into public/chatgpt-live by a plain `vite build`, same as its
# Storybook is built into public/chatgpt by build-chatgpt-sb.sh. Skips
# quietly, not fail the whole site's build, when the package isn't there.
set -e

DIR="../packages/chatgpt"

if [ ! -d "$DIR" ]; then
  echo "packages/chatgpt not present — skipping the /chatgpt-live build."
  exit 0
fi

cd "$DIR"
pnpm install --prod=false
./node_modules/.bin/vite build --outDir ../../web/public/chatgpt-live --logLevel warn
