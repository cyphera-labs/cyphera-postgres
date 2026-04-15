# cyphera-postgres

[![CI](https://github.com/cyphera-labs/cyphera-postgres/actions/workflows/ci.yml/badge.svg)](https://github.com/cyphera-labs/cyphera-postgres/actions/workflows/ci.yml)
[![Security](https://github.com/cyphera-labs/cyphera-postgres/actions/workflows/codeql.yml/badge.svg)](https://github.com/cyphera-labs/cyphera-postgres/actions/workflows/codeql.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)

Format-preserving encryption for [PostgreSQL](https://www.postgresql.org/) — native Rust extension powered by Cyphera.

Built on [`cyphera`](https://crates.io/crates/cyphera) from crates.io via [pgrx](https://github.com/pgcentralfoundation/pgrx).

## Quick Start (Demo)

```bash
docker compose up -d
psql -h localhost -U postgres -d cyphera_demo
# password: cyphera
```

```sql
SELECT cyphera_protect('ssn', '123-45-6789') AS protected;
-- → T01i6J-xF-07pX

SELECT cyphera_access(cyphera_protect('ssn', '123-45-6789')) AS accessed;
-- → 123-45-6789
```

## Build

### Via Docker (recommended)

```bash
docker build -t cyphera-postgres .
```

### From source

Requires Rust, cargo-pgrx, and PostgreSQL 17 dev headers:

```bash
cargo install cargo-pgrx --version 0.13.1 --locked
cargo pgrx init --pg17 /usr/lib/postgresql/17/bin/pg_config
cargo build --features pg17
```

## Install / Deploy

### Docker

Use the provided `docker-compose.yml` — it builds the extension and creates a Postgres instance with Cyphera loaded.

### Self-hosted Postgres

1. Build the extension: `cargo pgrx package --pg-config /path/to/pg_config`
2. Copy the built files to your Postgres extension directory
3. Place `cyphera.json` at `/etc/cyphera/cyphera.json`
4. In psql: `CREATE EXTENSION cyphera_postgres;`

## Usage

```sql
-- Protect (tagged, dashes preserved)
SELECT cyphera_protect('ssn', '123-45-6789');
-- → T01i6J-xF-07pX

-- Access using embedded tag (no policy name needed)
SELECT cyphera_access(cyphera_protect('ssn', '123-45-6789'));
-- → 123-45-6789

-- Bulk protect
SELECT name, ssn, cyphera_protect('ssn', ssn) AS protected
FROM customers;

-- In-place protection on INSERT
INSERT INTO customers_protected (name, ssn)
SELECT name, cyphera_protect('ssn', ssn) FROM customers;
```

## Operations

### Policy Configuration

- Policy file: `/etc/cyphera/cyphera.json` (override with `CYPHERA_POLICY_FILE` env var)
- Policy loaded once at first function call — restart Postgres to reload

### Monitoring

- Errors return as SQL errors — visible in psql and application logs
- Extension loaded: `SELECT * FROM pg_extension WHERE extname = 'cyphera_postgres';`

### Upgrading

1. Build new extension with updated `cyphera` crate version in `Cargo.toml`
2. Replace extension files in Postgres extension directory
3. `ALTER EXTENSION cyphera_postgres UPDATE;`
4. Restart Postgres

### Troubleshooting

- **Extension not found** — `CREATE EXTENSION cyphera_postgres;` not run, or files not in extension dir
- **"Unknown policy"** — check `CYPHERA_POLICY_FILE` path and cyphera.json contents
- **Build fails** — ensure pgrx version matches (0.13.x), Postgres dev headers installed

## Policy File

```json
{
  "policies": {
    "ssn": { "engine": "ff1", "key_ref": "demo-key", "tag": "T01" },
    "credit_card": { "engine": "ff1", "key_ref": "demo-key", "tag": "T02" }
  },
  "keys": {
    "demo-key": { "material": "2B7E151628AED2A6ABF7158809CF4F3C" }
  }
}
```

## Future

- PGXN package for easy `pgxn install`
- Cloud provider extension registries (AWS RDS, GCP Cloud SQL, Supabase)
- PostgreSQL 14/15/16 support (currently 17 only)
- Performance benchmarking vs application-layer encryption

## License

Apache 2.0 — Copyright 2026 Horizon Digital Engineering LLC
