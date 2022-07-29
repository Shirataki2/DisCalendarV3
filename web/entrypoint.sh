#!/bin/bash
if [ "$1" = '--dev' ]; then
bash
else
pnpm build
pnpm start
fi
