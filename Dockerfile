FROM clux/muslrust:stable AS chef
RUN cargo install cargo-chef
WORKDIR /app

RUN rustup target add aarch64-unknown-linux-musl && rustup target add x86_64-unknown-linux-musl

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target aarch64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-musl --bin pantheon-app

FROM scratch AS runtime
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/pantheon-app /usr/local/bin/
CMD ["/usr/local/bin/pantheon-app"]


