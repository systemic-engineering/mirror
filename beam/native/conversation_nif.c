/*
 * conversation_nif.c — Bridges Rust conversation parser to Erlang NIF API.
 *
 * Calls extern "C" functions from the conversation Rust static library.
 * Returns Gleam-compatible {ok, Binary} | {error, Binary} tagged tuples.
 */

#include <erl_nif.h>
#include <string.h>

#define CONV_BUF_SIZE 4096

/* Rust FFI symbols from libconversation.a */
extern int conv_parse(
    const unsigned char *src_ptr, size_t src_len,
    unsigned char *out_ptr, size_t out_cap,
    size_t *out_len
);

/* --- NIF functions --- */

static ERL_NIF_TERM nif_parse_conv(ErlNifEnv *env, int argc,
                                    const ERL_NIF_TERM argv[]) {
    ErlNifBinary source_bin;

    if (!enif_inspect_binary(env, argv[0], &source_bin))
        return enif_make_badarg(env);

    unsigned char buf[CONV_BUF_SIZE];
    size_t out_len = 0;

    int rc = conv_parse(source_bin.data, source_bin.size,
                        buf, CONV_BUF_SIZE, &out_len);

    /* Build the output binary */
    ERL_NIF_TERM out_term;
    unsigned char *out_buf = enif_make_new_binary(env, out_len, &out_term);
    memcpy(out_buf, buf, out_len);

    if (rc == 0) {
        return enif_make_tuple2(env,
                   enif_make_atom(env, "ok"),
                   out_term);
    } else {
        return enif_make_tuple2(env,
                   enif_make_atom(env, "error"),
                   out_term);
    }
}

static ErlNifFunc nif_funcs[] = {
    {"parse_conv", 1, nif_parse_conv, 0},
};

ERL_NIF_INIT(conversation_nif, nif_funcs, NULL, NULL, NULL, NULL)
