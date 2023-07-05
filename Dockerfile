<<<<<<< HEAD
FROM debian:unstable-slim
||||||| 9b07b57
=======
# cargo build --release / compile locally and copy to dockerfile
FROM debian:unstable-slim
>>>>>>> 3d2fead502d5c53302773f738a2b786011c2566a

WORKDIR /app

COPY ./target/release/tennis_bot /app

CMD ["/app/tennis_bot"]

