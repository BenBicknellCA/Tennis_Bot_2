

FROM lukemathwalker/cargo-chef:latest-rust-slim-bullseye AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin tennis_bot


FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/tennis_bot /usr/local/bin

CMD [ "/usr/local/bin/tennis_bot" ]