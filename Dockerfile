FROM debian:unstable-slim

WORKDIR /app

COPY ./target/release/tennis_bot /app

CMD ["/app/tennis_bot"]

