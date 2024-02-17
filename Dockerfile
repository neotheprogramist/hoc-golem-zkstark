# Using the `rust-musl-builder` as base image, instead of 
# the official Rust toolchain
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/get /usr/local/bin/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/post /usr/local/bin/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/prove /usr/local/bin/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/verify /usr/local/bin/
RUN ls -al /usr/local/bin/
ENTRYPOINT [ "app" ]
