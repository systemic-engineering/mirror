-module(extends).
-behaviour(conversation_construct).

%% Extends — grammar extension via parent type merging.
%%
%% When a grammar declares `extends @smash, @controller`, this module
%% resolves the parent grammars and merges their types into the child.
%%
%% Currently operates at the module metadata level: verifies parents
%% exist as loaded BEAM modules and returns the relationship graph.
%% Full type merging (reading parent type registries from the Rust
%% kernel) will follow once the bootstrap is proven.

-export([resolve/2, compile/1]).
-export([resolve_extends/1]).

%% ── conversation_construct callbacks ─────────────────────────────────────────

%% Resolve extends: verify all parent domains are loaded.
%% Data = #{domain => Domain, extends => [Parent1, Parent2, ...]}
%% Context = #{loaded => [Domain1, Domain2, ...]} (booted domains)
resolve(#{domain := Domain, extends := Parents} = Data, #{loaded := Loaded}) ->
    case check_parents(Parents, Loaded, []) of
        {ok, _} -> {ok, Data#{resolved => true}};
        {error, Missing} ->
            Msg = iolist_to_binary(
                io_lib:format("~s extends unloaded domains: ~p", [Domain, Missing])
            ),
            {error, Msg}
    end;
resolve(Data, _Context) ->
    {ok, Data}.

%% Compile resolved extends data — currently a no-op since the Rust
%% kernel already emits the extends/0 function in the actor module.
%% When type merging moves to BEAM, this will emit merged EAF.
compile(#{resolved := true} = _Data) ->
    {ok, []};
compile(_Data) ->
    {ok, []}.

%% ── Public API ───────────────────────────────────────────────────────────────

%% Resolve extends for a compiled module by reading its extends/0.
%% Returns {ok, Parents} or {error, Reason}.
resolve_extends(ModuleBinary) when is_binary(ModuleBinary) ->
    Module = binary_to_atom(ModuleBinary, utf8),
    try
        Parents = Module:extends(),
        {ok, Parents}
    catch
        _:Reason ->
            {error, iolist_to_binary(io_lib:format("~p", [Reason]))}
    end.

%% ── Internal ─────────────────────────────────────────────────────────────────

check_parents([], _Loaded, []) ->
    {ok, all_loaded};
check_parents([], _Loaded, Missing) ->
    {error, lists:reverse(Missing)};
check_parents([Parent | Rest], Loaded, Missing) ->
    ConvModule = <<"conv_", Parent/binary>>,
    case lists:member(ConvModule, Loaded) orelse is_module_loaded(ConvModule) of
        true -> check_parents(Rest, Loaded, Missing);
        false -> check_parents(Rest, Loaded, [Parent | Missing])
    end.

is_module_loaded(ModuleBinary) ->
    Module = binary_to_atom(ModuleBinary, utf8),
    case code:is_loaded(Module) of
        {file, _} -> true;
        false -> false
    end.
