FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
FROM debian:buster-slim
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/docker /usr/local/bin/docker
EXPOSE 8080
CMD ["docker"]
