# ---- build stage ----
FROM rust:1.90-bookworm AS builder
WORKDIR /app

# 의존성만 먼저 빌드해서 레이어 캐시 확보
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src

# 실제 소스 복사 후 빌드
COPY src ./src
# main.rs의 mtime이 더미보다 오래됐을 수 있어 강제로 재빌드 유도
RUN touch src/main.rs && cargo build --release

# ---- runtime stage ----
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/url_shortener /usr/local/bin/url_shortener
ENV PORT=3000
EXPOSE 3000
USER nobody
CMD ["url_shortener"]
