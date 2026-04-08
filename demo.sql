-- Cyphera PostgreSQL Extension Demo
-- Run: docker compose up -d && psql -h localhost -U postgres -d cyphera_demo

-- Protect SSNs (tagged, passthroughs preserved)
SELECT cyphera_protect('ssn', '123-45-6789') AS protected_ssn;

-- Access using embedded tag (no policy name needed)
SELECT cyphera_unprotect(cyphera_protect('ssn', '123-45-6789')) AS accessed_ssn;

-- Access with explicit policy
SELECT cyphera_access('ssn', cyphera_protect('ssn', '123-45-6789')) AS accessed_ssn;

-- Round-trip proof
SELECT
    '123-45-6789' AS original,
    cyphera_protect('ssn', '123-45-6789') AS protected,
    cyphera_unprotect(cyphera_protect('ssn', '123-45-6789')) AS accessed;

-- Bulk example
SELECT
    name,
    ssn AS original_ssn,
    cyphera_protect('ssn', ssn) AS protected_ssn
FROM (VALUES
    ('Alice', '123-45-6789'),
    ('Bob',   '987-65-4321'),
    ('Carol', '555-12-3456')
) AS t(name, ssn);
