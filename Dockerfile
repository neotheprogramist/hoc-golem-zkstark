FROM clux/muslrust:stable
RUN cargo install \
  --git https://github.com/lambdaclass/lambdaworks \
  --tag v0.5.0 \
  --features=cli \
  cairo-platinum-prover
ENTRYPOINT [ "platinum-prover" ]
