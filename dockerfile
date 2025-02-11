FROM rust:1-alpine3.21 AS base
RUN apk update \
    && apk add --no-cache --purge openssl-dev openssl-libs-static musl-dev libc-dev gcc make

FROM base AS chef
RUN cargo install cargo-chef

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release 
RUN make zombie

FROM alpine:3.21.2 AS runtime
USER 1000
WORKDIR /app
COPY --from=builder /app/target/release/earlyd /app/earlyd
COPY bin/oops.sh /app/oops.sh
COPY --from=builder /app/bin/zombie-maker /app/zombie-maker
ENTRYPOINT ["/app/earlyd"]
