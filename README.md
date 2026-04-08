# cyphera-postgres

Cyphera format-preserving encryption extension for PostgreSQL 17. Built with [pgrx](https://github.com/pgcentralfoundation/pgrx) — Rust runs natively inside Postgres.

```sql
SELECT
    '123-45-6789' AS original,
    cyphera_protect('ssn', '123-45-6789') AS protected,
    cyphera_unprotect(cyphera_protect('ssn', '123-45-6789')) AS accessed;

  original   |   protected    |  accessed
-------------+----------------+-------------
 123-45-6789 | T01i6J-xF-07pX | 123-45-6789
```

## Quick Start

```bash
docker compose up -d
psql -h localhost -U postgres -d cyphera_demo
# password: cyphera
```

## Usage

```sql
-- Protect (tagged, dashes preserved)
SELECT cyphera_protect('ssn', '123-45-6789');
-- → T01i6J-xF-07pX

-- Access using embedded tag (no policy name needed)
SELECT cyphera_unprotect(cyphera_protect('ssn', '123-45-6789'));
-- → 123-45-6789

-- Bulk
SELECT name, ssn, cyphera_protect('ssn', ssn) AS protected
FROM (VALUES
    ('Alice', '123-45-6789'),
    ('Bob',   '987-65-4321'),
    ('Carol', '555-12-3456')
) AS t(name, ssn);

 name  |     ssn     |   protected
-------+-------------+----------------
 Alice | 123-45-6789 | T01i6J-xF-07pX
 Bob   | 987-65-4321 | T01Q1I-cH-Sdcb
 Carol | 555-12-3456 | T01b54-Un-4zHt
```

## Policy Configuration

Place `cyphera.yaml` at `/etc/cyphera/cyphera.yaml` or set `CYPHERA_POLICY_FILE`.

```yaml
policies:
  ssn:
    engine: ff1
    key_ref: demo-key
    tag: T01

keys:
  demo-key:
    material: "2B7E151628AED2A6ABF7158809CF4F3C"
```

## How It Works

This extension uses the [cyphera](https://crates.io/crates/cyphera) Rust crate compiled as a native PostgreSQL extension via pgrx. FF1/FF3 format-preserving encryption runs inside the Postgres process — no network calls, no external services.

## Status

Alpha. Uses `cyphera` crate from crates.io.

## License

Apache 2.0 — Copyright 2026 Horizon Digital Engineering LLC
