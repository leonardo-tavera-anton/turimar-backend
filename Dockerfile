# Stage 1: Compilacion
FROM rust:1.75-slim as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

# Stage 2: Imagen final liviana de ejecucion
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/turimar-backend /app/turimar-backend

EXPOSE 3000
CMD ["/app/turimar-backend"]