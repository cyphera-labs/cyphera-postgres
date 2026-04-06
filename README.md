# cyphera-postgres

Format-preserving encryption UDFs for PostgreSQL.

> **Note**: Currently uses a dummy (reversible shift) cipher as a placeholder.
> Real FF1 FPE will be wired in via `cyphera-rust` native extension.

## Quick Start

```bash
docker compose up -d
psql -h localhost -U postgres -d cyphera_demo < demo.sql
# password: cyphera
```

## Functions

### Policy-based (primary interface)

```sql
SELECT cyphera_protect('ssn', '123-45-6789');
-- → '456-78-9012' (format preserved)

SELECT cyphera_unprotect('ssn', '456-78-9012');
-- → '123-45-6789'
```

### Direct engine (testing / demos)

```sql
SELECT cyphera_ff1_encrypt('123456789', '<key_hex>', 'digits');
SELECT cyphera_ff1_decrypt('<ciphertext>', '<key_hex>', 'digits');
```

## Alphabets

| Name | Characters |
|------|-----------|
| `digits` | `0-9` |
| `alpha_lower` | `a-z` |
| `alphanumeric` | `0-9a-zA-Z` |
