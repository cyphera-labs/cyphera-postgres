FROM rust:latest AS builder

# Add PostgreSQL apt repo for PG17
RUN apt-get update && apt-get install -y curl gnupg lsb-release && \
    curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc | gpg --dearmor -o /usr/share/keyrings/pgdg.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/pgdg.gpg] http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list && \
    apt-get update && apt-get install -y \
    postgresql-server-dev-17 \
    postgresql-17 \
    pkg-config \
    libssl-dev \
    libreadline-dev \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-pgrx
RUN cargo install --locked cargo-pgrx@0.13.1
RUN cargo pgrx init --pg17 /usr/lib/postgresql/17/bin/pg_config

WORKDIR /app
COPY Cargo.toml .
COPY cyphera_postgres.control .
COPY src/ src/

# Build the extension
RUN cargo pgrx package --pg-config /usr/lib/postgresql/17/bin/pg_config

# Production image
FROM postgres:17-bookworm

# Copy the built extension
COPY --from=builder /app/target/release/cyphera_postgres-pg17/usr /usr

# Copy policy config
COPY config/cyphera.json /etc/cyphera/cyphera.json

ENV CYPHERA_POLICY_FILE=/etc/cyphera/cyphera.json

# Auto-create extension on startup
COPY init.sql /docker-entrypoint-initdb.d/
