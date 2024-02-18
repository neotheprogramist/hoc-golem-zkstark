How tu build project:

- ./entrypoint.sh

Check Locally:

- cargo run --release --bin app
- cargo run --release --bin get 1414999590036870372770778692906046516351208194453833185145989807125183271772
- cargo run --release --bin post 1414999590036870372770778692906046516351208194453833185145989807125183271772 1 < resources/main.proof
- cargo run --release --bin get 1414999590036870372770778692906046516351208194453833185145989807125183271772

Deploy to Golem:

- export YAGNA_AUTOCONF_APPKEY=hoc_golem_zkstark && yagna service run
- podman build -t neoprogram/hoc-golem-zkstark:latest .
- podman push localhost/hoc-golem-zkstark:latest docker.io/neoprogram/hoc-golem-zkstark:latest
- npx gvmkit-build neoprogram/hoc-golem-zkstark:latest --push --nologin
- node src/verify.mjs
