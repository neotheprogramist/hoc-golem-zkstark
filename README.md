python -m venv .venv

pip install --upgrade pip

source .venv/bin/activate

pip install --upgrade pip

pip install -U pip dapp-runner

curl https://raw.githubusercontent.com/golemfactory/dapp-store/81e3f50aba90a84d335a26cb9cc2ea778193be11/apps/todo-app.yaml > webapp.yaml

curl https://raw.githubusercontent.com/golemfactory/dapp-runner/main/configs/default.yaml > config.yaml

curl -sSf https://join.golem.network/as-requestor | bash -

yagna service run

yagna payment fund

podman build -t golem-example .

podman push localhost/golem-example:latest docker.io/neoprogram/golem-example

npx gvmkit-build neoprogram/golem-example

npx gvmkit-build neoprogram/hoc-golem-zkstark --push --nologin

npx ts-node src/index.ts
