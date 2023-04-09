FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

#--- Planner
FROM chef AS planner
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

#--- Build
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin cdn-dns

#--- Runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y  \
    && apt install -y sudo vim systemctl\
    && apt-get autoremove -y \
    && apt-get clean -y 

COPY --from=builder /app/target/release/cdn-dns cdn-dns
COPY config config
COPY connections.json connections.json
ENV APP_ENV production
ENTRYPOINT ["./cdn-dns"]
