-- cyphera extension SQL definitions

CREATE OR REPLACE FUNCTION cyphera_ff1_encrypt(value text, key_hex text, alphabet text)
RETURNS text
AS 'MODULE_PATHNAME', 'cyphera_ff1_encrypt'
LANGUAGE C IMMUTABLE STRICT;

CREATE OR REPLACE FUNCTION cyphera_ff1_decrypt(value text, key_hex text, alphabet text)
RETURNS text
AS 'MODULE_PATHNAME', 'cyphera_ff1_decrypt'
LANGUAGE C IMMUTABLE STRICT;

CREATE OR REPLACE FUNCTION cyphera_protect(policy_name text, value text)
RETURNS text
AS 'MODULE_PATHNAME', 'cyphera_protect'
LANGUAGE C IMMUTABLE STRICT;

CREATE OR REPLACE FUNCTION cyphera_unprotect(policy_name text, value text)
RETURNS text
AS 'MODULE_PATHNAME', 'cyphera_unprotect'
LANGUAGE C IMMUTABLE STRICT;
