# cargo build --release / compile locally and copy to dockerfile
FROM debian:unstable-slim

WORKDIR /app

COPY ./target/release/tennis_bot /app

CMD ["/app/tennis_bot"]

