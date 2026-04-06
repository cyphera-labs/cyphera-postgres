#include <postgres.h>
#include <fmgr.h>
#include <utils/builtins.h>
#include <string.h>
#include <stdlib.h>

#ifdef PG_MODULE_MAGIC
PG_MODULE_MAGIC;
#endif

/*
 * Dummy cipher: reversible character shift within an alphabet.
 * Placeholder until real FF1 FPE is wired in via cyphera-rust.
 *
 * Format preservation: non-alphabet characters (dashes, spaces) stay in place.
 */

static const char *DIGITS = "0123456789";
static const char *ALPHA_LOWER = "abcdefghijklmnopqrstuvwxyz";
static const char *ALPHANUMERIC = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

static const char *
resolve_alphabet(const char *name)
{
    if (strcmp(name, "digits") == 0) return DIGITS;
    if (strcmp(name, "alpha_lower") == 0) return ALPHA_LOWER;
    if (strcmp(name, "alphanumeric") == 0) return ALPHANUMERIC;
    return DIGITS;
}

static int
derive_shift(const char *key_hex)
{
    unsigned int h = 0;
    for (int i = 0; key_hex[i]; i++)
        h = 31 * h + (unsigned char)key_hex[i];
    return (h % 256) + 1;
}

static char *
dummy_encrypt(const char *input, const char *alphabet_name, const char *key_hex)
{
    const char *alpha = resolve_alphabet(alphabet_name);
    int alpha_len = strlen(alpha);
    int shift = derive_shift(key_hex);
    int len = strlen(input);
    char *result = palloc(len + 1);

    for (int i = 0; i < len; i++)
    {
        const char *p = strchr(alpha, input[i]);
        if (p)
        {
            int idx = (int)(p - alpha);
            result[i] = alpha[(idx + shift) % alpha_len];
        }
        else
        {
            result[i] = input[i];
        }
    }
    result[len] = '\0';
    return result;
}

static char *
dummy_decrypt(const char *input, const char *alphabet_name, const char *key_hex)
{
    const char *alpha = resolve_alphabet(alphabet_name);
    int alpha_len = strlen(alpha);
    int shift = derive_shift(key_hex);
    int len = strlen(input);
    char *result = palloc(len + 1);

    for (int i = 0; i < len; i++)
    {
        const char *p = strchr(alpha, input[i]);
        if (p)
        {
            int idx = (int)(p - alpha);
            result[i] = alpha[((idx - (shift % alpha_len)) + alpha_len) % alpha_len];
        }
        else
        {
            result[i] = input[i];
        }
    }
    result[len] = '\0';
    return result;
}

/* ── cyphera_ff1_encrypt(value, key_hex, alphabet) ── */

PG_FUNCTION_INFO_V1(cyphera_ff1_encrypt);

Datum
cyphera_ff1_encrypt(PG_FUNCTION_ARGS)
{
    text *value_t = PG_GETARG_TEXT_PP(0);
    text *key_t = PG_GETARG_TEXT_PP(1);
    text *alpha_t = PG_GETARG_TEXT_PP(2);

    char *value = text_to_cstring(value_t);
    char *key_hex = text_to_cstring(key_t);
    char *alpha_name = text_to_cstring(alpha_t);

    char *encrypted = dummy_encrypt(value, alpha_name, key_hex);
    PG_RETURN_TEXT_P(cstring_to_text(encrypted));
}

/* ── cyphera_ff1_decrypt(value, key_hex, alphabet) ── */

PG_FUNCTION_INFO_V1(cyphera_ff1_decrypt);

Datum
cyphera_ff1_decrypt(PG_FUNCTION_ARGS)
{
    text *value_t = PG_GETARG_TEXT_PP(0);
    text *key_t = PG_GETARG_TEXT_PP(1);
    text *alpha_t = PG_GETARG_TEXT_PP(2);

    char *value = text_to_cstring(value_t);
    char *key_hex = text_to_cstring(key_t);
    char *alpha_name = text_to_cstring(alpha_t);

    char *decrypted = dummy_decrypt(value, alpha_name, key_hex);
    PG_RETURN_TEXT_P(cstring_to_text(decrypted));
}

/* ── cyphera_protect(policy_name, value) ── */
/* For now, hardcoded demo policies. Will use real policy file via cyphera-rust. */

PG_FUNCTION_INFO_V1(cyphera_protect);

Datum
cyphera_protect(PG_FUNCTION_ARGS)
{
    text *policy_t = PG_GETARG_TEXT_PP(0);
    text *value_t = PG_GETARG_TEXT_PP(1);

    char *policy = text_to_cstring(policy_t);
    char *value = text_to_cstring(value_t);

    /* Hardcoded demo key */
    const char *key_hex = "2B7E151628AED2A6ABF7158809CF4F3C";
    const char *alpha_name = "digits";

    if (strcmp(policy, "name") == 0)
        alpha_name = "alpha_lower";

    char *encrypted = dummy_encrypt(value, alpha_name, key_hex);
    PG_RETURN_TEXT_P(cstring_to_text(encrypted));
}

/* ── cyphera_unprotect(policy_name, value) ── */

PG_FUNCTION_INFO_V1(cyphera_unprotect);

Datum
cyphera_unprotect(PG_FUNCTION_ARGS)
{
    text *policy_t = PG_GETARG_TEXT_PP(0);
    text *value_t = PG_GETARG_TEXT_PP(1);

    char *policy = text_to_cstring(policy_t);
    char *value = text_to_cstring(value_t);

    const char *key_hex = "2B7E151628AED2A6ABF7158809CF4F3C";
    const char *alpha_name = "digits";

    if (strcmp(policy, "name") == 0)
        alpha_name = "alpha_lower";

    char *decrypted = dummy_decrypt(value, alpha_name, key_hex);
    PG_RETURN_TEXT_P(cstring_to_text(decrypted));
}
