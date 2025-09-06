FROM rust:1.80 AS builder
WORKDIR /usr/src/nexus
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/nexus/target/release/nexus /usr/local/bin/nexus
COPY configs /configs
EXPOSE 3000
CMD ["nexus"]