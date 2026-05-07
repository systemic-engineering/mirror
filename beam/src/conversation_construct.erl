-module(conversation_construct).

%% Behaviour for language constructs.
%%
%% Each syntactic extension (extends, translate, record, ...) implements
%% this behaviour. The kernel compiles bootstrap.conv → BEAM, then
%% dispatches unknown constructs to the registered module.
%%
%% Callbacks:
%%   resolve/2 — merge/validate with namespace context
%%   compile/1 — emit Erlang Abstract Format terms

-callback resolve(Data :: term(), Context :: map()) ->
    {ok, Resolved :: term()} | {error, Reason :: binary()}.

-callback compile(Resolved :: term()) ->
    {ok, EAF :: list()} | {error, Reason :: binary()}.
