# syntax=docker/dockerfile:1

FROM rust:1.71-slim-buster
WORKDIR /app
COPY ./backend .
RUN cargo build --release
EXPOSE 3000
CMD ["./target/release/key-value-store"]


# Multi stage not working
# FROM rust:1.71-slim-buster AS builder
# WORKDIR /app
# COPY ./backend .
# RUN cargo build --release

# FROM debian:buster-slim
# COPY --from=builder /app/target/release/key-value-store /
# EXPOSE 3000
# CMD ["./key-value-store"]