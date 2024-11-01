FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/blogposts /usr/local/bin/blogposts
EXPOSE 3000
CMD ["blogposts"]