FROM rust:1.81 AS builder
WORKDIR /usr/src/project-maverick
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/project-maverick /usr/local/bin/project-maverick
CMD ["project-maverick"]
