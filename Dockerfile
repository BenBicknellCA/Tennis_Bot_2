FROM rust:1.68.0 AS build
WORKDIR /app
ENV CARGO_TARGET_DIR=/target
COPY . .

RUN cargo install --path . --verbose --locked
RUN cargo build --release





FROM debian:stable-slim
ENV CARGO_TARGET_DIR=/target
RUN apt-get update && apt-get -y install ca-certificates libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/local/cargo/bin/tennis_bot /bin
COPY --from=build /app/.env /bin
ENV source /bin/.env
CMD ["tennis_bot"]
