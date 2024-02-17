#!/usr/bin/env zsh

source .venv/bin/activate && \
cairo-run \
  --proof_mode \
  --print_output \
  --layout small \
  --air_public_input resources/main_public_input.json \
  --air_private_input resources/main_private_input.json \
  --trace_file resources/main_trace.bin \
  --memory_file resources/main_memory.bin \
  --program resources/main.json \
  --program_input src/main_input.json && \
cd .. && \
deactivate
