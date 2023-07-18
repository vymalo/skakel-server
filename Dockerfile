FROM --platform=linux/amd64 lukemathwalker/cargo-chef:latest-rust-1.69.0 AS chef
WORKDIR /app

FROM --platform=linux/amd64 chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=linux/amd64 chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN mkdir -p /app
COPY --from=builder /app/target/release/skakel-backend /app
WORKDIR /app
ENV JAEGER_ENABLED=true
ENV PORT=8000

EXPOSE 8000
ENTRYPOINT ["./skakel-backend"]