FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release
FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y libpq-dev \
    iputils-ping \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/blogposts /usr/local/bin/blogposts
EXPOSE 3000
CMD ["blogposts"]