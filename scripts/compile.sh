#!/usr/bin/env zsh

source .venv/bin/activate && \
cairo-compile \
  --proof_mode \
  --output resources/main.json \
  src/main.cairo && \
deactivate
