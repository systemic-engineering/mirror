-module(conversation_nif).
-export([parse_conv/1, compile_grammar/1, compile_grammar_traced/1]).
-on_load(init/0).

init() ->
    PrivDir = code:priv_dir(conversation_beam),
    NifPath = filename:join(PrivDir, "conversation_nif"),
    erlang:load_nif(NifPath, 0).

parse_conv(_Source) ->
    erlang:nif_error(nif_not_loaded).

compile_grammar(_Source) ->
    erlang:nif_error(nif_not_loaded).

compile_grammar_traced(_Source) ->
    erlang:nif_error(nif_not_loaded).
