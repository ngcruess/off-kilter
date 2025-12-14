# Build stage
FROM rust:1.75 as builder

WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY Cargo.toml ./

# Build dependencies first for better caching
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source and build
COPY backend/src ./src
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/kilter-board-backend /app/kilter-board-backend

EXPOSE 3000

CMD ["./kilter-board-backend"]