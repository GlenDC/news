# 1: Build the exe
FROM rust:1.58-alpine as builder
WORKDIR /usr/src

RUN USER=root cargo new --bin plabayo-news-web
WORKDIR /usr/src/plabayo-news-web
COPY plabayo-news-web/ ./
RUN cargo build --release

# 2: Copy the exe and extra files ("static") to an empty Docker image
FROM scratch
COPY --from=builder /usr/src/plabayo-news-web/target/release/plabayo-news-web .
USER 1000
CMD ["./plabayo-news-web"]
