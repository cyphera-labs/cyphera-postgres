CREATE EXTENSION IF NOT EXISTS cyphera;

-- Verify the extension loaded
SELECT cyphera_ff1_encrypt('123456789', '2B7E151628AED2A6ABF7158809CF4F3C', 'digits') AS test;
