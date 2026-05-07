FROM rust:1-bookworm AS builder

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

RUN useradd --create-home --shell /usr/sbin/nologin appuser
WORKDIR /app
COPY --from=builder /app/target/release/incident_commander_rs /usr/local/bin/incident_commander_rs
USER appuser

EXPOSE 8088
ENTRYPOINT ["incident_commander_rs"]
CMD ["--provider", "mock", "--listen", "0.0.0.0:8088"]
