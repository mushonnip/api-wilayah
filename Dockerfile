FROM rust:1.85-slim AS builder

WORKDIR /app

# Pre-cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY . .
# touch main.rs so cargo rebuilds the binary
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/api-wilayah /usr/local/bin/api-wilayah

ENV PORT=8989
EXPOSE 8989

CMD ["api-wilayah"]
