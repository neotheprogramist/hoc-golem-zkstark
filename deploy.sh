#!/usr/bin/env bash

podman build -t hoc-golem-zkstark . && \
podman push hoc-golem-zkstark:latest docker.io/neoprogram/hoc-golem-zkstark:latest && \
npx gvmkit-build hoc-golem-zkstark:latest --push --nologin
