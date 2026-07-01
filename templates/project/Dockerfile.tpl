# Stage 1: Build Frontend with Bun
FROM oven/bun:1 AS frontend-builder
WORKDIR /app/frontend
COPY frontend/ .
RUN bun install
RUN bun run build

# Stage 2: Build Backend with Rust
FROM rust:1.75-slim AS backend-builder
WORKDIR /build
COPY . .
RUN cargo build --release

# Stage 3: Final Runtime Image
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=backend-builder /build/target/release/{{ PROJECT_ID }} .
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist
COPY --from=frontend-builder /app/frontend/public ./frontend/public
COPY .env .
EXPOSE {{ BACKEND_PORT }}
EXPOSE {{ FRONTEND_PORT }}
CMD ["./{{ PROJECT_ID }}"]
