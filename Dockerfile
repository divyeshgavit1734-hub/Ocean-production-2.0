FROM rust:1-bookworm AS builder

WORKDIR /app

COPY backend/Cargo.toml ./Cargo.toml
COPY backend/src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/ocean-production-backend .

ENV PORT=3000

EXPOSE 3000

CMD ["./ocean-production-backend"]