#!/usr/bin/env bash

cargo run --release --bin prove \
    resources/main_trace.bin \
    resources/main_memory.bin \
    resources/main.proof
