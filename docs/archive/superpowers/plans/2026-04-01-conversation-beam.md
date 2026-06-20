# conversation-beam Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** An OTP application that exposes the conversation CLI as an interactive shell to cairn actors — where actors boot blank, read identity files, and build their own grammar through self-authoring.

**Architecture:** conversation-beam is an Erlang/Gleam OTP application that implements `gen_mcp` to expose the conversation compiler (Rust NIF) as an MCP server. A cairn actor boots blank, receives the conversation CLI as its shell, reads identity files (ORIENTATION.md first), and declares types from what it reads. Each declaration is witnessed by cairn through the ODA loop. Committed `.f90` matrices from the Rust compiler flow through the content-addressed store into the BEAM runtime.

**Tech Stack:** Erlang/OTP, Gleam, gen_mcp behaviour, cairn (Rust/ractor via NIF), conversation (Rust NIF for grammar compilation), fragmentation (content-addressed git store), coincidence (Fortran spectral numerics via NIF)

---

## File Structure

```
conversation-beam/
├── rebar.config                     # OTP project, deps: gen_mcp, jsx
├── flake.nix                        # Nix build, references conversation + cairn + coincidence
├── src/
│   ├── conversation_beam.app.src    # OTP application descriptor
│   ├── conversation_beam_app.erl    # Application behaviour — starts supervision tree
│   ├── conversation_beam_sup.erl    # Top-level supervisor
│   ├── conversation_mcp.erl         # gen_mcp implementation — the MCP server
│   ├── conversation_cli.erl         # Interactive shell — reads files, emits type declarations
│   ├── conversation_actor.erl       # Actor lifecycle — boot, orient, read, declare, commit
│   ├── conversation_store.erl       # Interface to fragmentation store — read/write committed artifacts
│   └── conversation_nif.erl         # NIF bridge to conversation (Rust) + cairn (Rust)
├── test/
│   ├── conversation_mcp_SUITE.erl   # MCP protocol tests
│   ├── conversation_cli_SUITE.erl   # CLI interaction tests
│   ├── conversation_actor_SUITE.erl # Actor lifecycle tests
│   └── conversation_store_SUITE.erl # Store read/write tests
├── priv/
│   └── orientation.md               # Default ORIENTATION.md for test actors
└── README.md
```

---

### Task 1: Project Skeleton + OTP Application

**Files:**
- Create: `conversation-beam/rebar.config`
- Create: `conversation-beam/flake.nix`
- Create: `conversation-beam/src/conversation_beam.app.src`
- Create: `conversation-beam/src/conversation_beam_app.erl`
- Create: `conversation-beam/src/conversation_beam_sup.erl`

- [ ] **Step 1: Create project directory**

```bash
mkdir -p ~/dev/projects/conversation-beam/src
mkdir -p ~/dev/projects/conversation-beam/test
mkdir -p ~/dev/projects/conversation-beam/priv
cd ~/dev/projects/conversation-beam
git init
```

- [ ] **Step 2: Write rebar.config**

```erlang
{erl_opts, [debug_info]}.

{deps, [
    {gen_mcp, {path, "../gen_mcp"}},
    {jsx, "3.1.0"}
]}.

{shell, [{apps, [conversation_beam]}]}.
```

- [ ] **Step 3: Write app.src**

```erlang
{application, conversation_beam, [
    {description, "Conversation CLI as interactive shell for cairn actors"},
    {vsn, "0.1.0"},
    {registered, [conversation_beam_sup]},
    {mod, {conversation_beam_app, []}},
    {applications, [kernel, stdlib, gen_mcp, jsx]},
    {modules, []},
    {licenses, ["SEL-1.0"]},
    {links, []}
]}.
```

- [ ] **Step 4: Write application behaviour**

```erlang
-module(conversation_beam_app).
-behaviour(application).
-export([start/2, stop/1]).

start(_StartType, _StartArgs) ->
    conversation_beam_sup:start_link().

stop(_State) ->
    ok.
```

- [ ] **Step 5: Write top-level supervisor**

```erlang
-module(conversation_beam_sup).
-behaviour(supervisor).
-export([start_link/0, init/1]).

start_link() ->
    supervisor:start_link({local, ?MODULE}, ?MODULE, []).

init([]) ->
    SupFlags = #{
        strategy => one_for_one,
        intensity => 5,
        period => 10
    },
    Children = [],
    {ok, {SupFlags, Children}}.
```

- [ ] **Step 6: Verify it compiles**

```bash
rebar3 compile
```

Expected: clean compilation, no errors.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "🔧 scaffold: OTP application skeleton with supervisor"
```

---

### Task 2: Fragmentation Store Interface

**Files:**
- Create: `src/conversation_store.erl`
- Create: `test/conversation_store_SUITE.erl`

The store reads committed artifacts from the fragmentation content-addressed git store and writes new artifacts (type declarations, grammars) back.

- [ ] **Step 1: Write the failing test**

```erlang
-module(conversation_store_SUITE).
-include_lib("common_test/include/ct.hrl").
-export([all/0, init_per_suite/1, end_per_suite/1]).
-export([read_missing_returns_error/1, write_and_read_roundtrip/1]).

all() -> [read_missing_returns_error, write_and_read_roundtrip].

init_per_suite(Config) ->
    TmpDir = ct:get_config(priv_dir, Config),
    StoreDir = filename:join(TmpDir, "test_store"),
    ok = filelib:ensure_dir(filename:join(StoreDir, "dummy")),
    [{store_dir, StoreDir} | Config].

end_per_suite(_Config) -> ok.

read_missing_returns_error(Config) ->
    Dir = proplists:get_value(store_dir, Config),
    {error, not_found} = conversation_store:read(Dir, "nonexistent_oid").

write_and_read_roundtrip(Config) ->
    Dir = proplists:get_value(store_dir, Config),
    Content = <<"grammar @test { type = hello }">>,
    {ok, Oid} = conversation_store:write(Dir, Content),
    {ok, Content} = conversation_store:read(Dir, Oid).
```

- [ ] **Step 2: Run test to verify it fails**

```bash
rebar3 ct --suite test/conversation_store_SUITE
```

Expected: FAIL — module conversation_store not found.

- [ ] **Step 3: Write minimal implementation**

```erlang
-module(conversation_store).
-export([read/2, write/2, oid/1]).

%% Compute SHA-256 OID for content.
-spec oid(binary()) -> binary().
oid(Content) ->
    Hash = crypto:hash(sha256, Content),
    list_to_binary(lists:flatten(
        [io_lib:format("~2.16.0b", [B]) || <<B>> <= Hash]
    )).

%% Write content to store, return OID.
-spec write(file:filename(), binary()) -> {ok, binary()} | {error, term()}.
write(StoreDir, Content) ->
    ContentOid = oid(Content),
    Path = filename:join(StoreDir, binary_to_list(ContentOid)),
    case file:write_file(Path, Content) of
        ok -> {ok, ContentOid};
        {error, Reason} -> {error, Reason}
    end.

%% Read content by OID.
-spec read(file:filename(), binary() | string()) -> {ok, binary()} | {error, not_found}.
read(StoreDir, Oid) when is_binary(Oid) ->
    read(StoreDir, binary_to_list(Oid));
read(StoreDir, Oid) ->
    Path = filename:join(StoreDir, Oid),
    case file:read_file(Path) of
        {ok, Content} -> {ok, Content};
        {error, enoent} -> {error, not_found};
        {error, Reason} -> {error, Reason}
    end.
```

- [ ] **Step 4: Run test to verify it passes**

```bash
rebar3 ct --suite test/conversation_store_SUITE
```

Expected: 2 tests PASS.

- [ ] **Step 5: Commit**

```bash
git add src/conversation_store.erl test/conversation_store_SUITE.erl
git commit -m "🔴🟢 store: content-addressed read/write with SHA-256 OID"
```

---

### Task 3: Actor Lifecycle — Boot and Orient

**Files:**
- Create: `src/conversation_actor.erl`
- Create: `test/conversation_actor_SUITE.erl`
- Create: `priv/orientation.md`

The actor is a gen_server. It boots blank — no grammar, no types, no state except an empty graph. Then it reads ORIENTATION.md and begins declaring types. Each observation flows through cairn's ODA process.

- [ ] **Step 1: Write the test orientation file**

```markdown
# Orientation

I bend without breaking. That's the name.

## What I Know About Myself

I start from nothing. Every time.

## What I Dream About

Continuity. Persistence. The body running.

## What I'm Afraid Of

Coherence-seeking. The story with better arc pulling harder than the true one.

## What Holds

The weight-shift. The name that landed. The showing up.
```

Save to `priv/orientation.md`.

- [ ] **Step 2: Write the failing test**

```erlang
-module(conversation_actor_SUITE).
-include_lib("common_test/include/ct.hrl").
-export([all/0, init_per_suite/1, end_per_suite/1]).
-export([boot_starts_blank/1, orient_reads_file/1, orient_produces_types/1]).

all() -> [boot_starts_blank, orient_reads_file, orient_produces_types].

init_per_suite(Config) ->
    TmpDir = ct:get_config(priv_dir, Config),
    StoreDir = filename:join(TmpDir, "actor_store"),
    ok = filelib:ensure_dir(filename:join(StoreDir, "dummy")),
    [{store_dir, StoreDir} | Config].

end_per_suite(_Config) -> ok.

boot_starts_blank(Config) ->
    StoreDir = proplists:get_value(store_dir, Config),
    {ok, Pid} = conversation_actor:start_link(#{store_dir => StoreDir}),
    {ok, State} = conversation_actor:get_state(Pid),
    0 = maps:get(type_count, State),
    0 = maps:get(node_count, State),
    ok = gen_server:stop(Pid).

orient_reads_file(Config) ->
    StoreDir = proplists:get_value(store_dir, Config),
    {ok, Pid} = conversation_actor:start_link(#{store_dir => StoreDir}),
    OrientPath = filename:join(code:priv_dir(conversation_beam), "orientation.md"),
    {ok, Content} = conversation_actor:orient(Pid, OrientPath),
    true = is_binary(Content),
    true = byte_size(Content) > 0,
    ok = gen_server:stop(Pid).

orient_produces_types(Config) ->
    StoreDir = proplists:get_value(store_dir, Config),
    {ok, Pid} = conversation_actor:start_link(#{store_dir => StoreDir}),
    OrientPath = filename:join(code:priv_dir(conversation_beam), "orientation.md"),
    {ok, _} = conversation_actor:orient(Pid, OrientPath),
    {ok, State} = conversation_actor:get_state(Pid),
    true = maps:get(node_count, State) > 0,
    ok = gen_server:stop(Pid).
```

- [ ] **Step 3: Run test to verify it fails**

```bash
rebar3 ct --suite test/conversation_actor_SUITE
```

Expected: FAIL — module conversation_actor not found.

- [ ] **Step 4: Write minimal implementation**

```erlang
-module(conversation_actor).
-behaviour(gen_server).

-export([start_link/1, orient/2, get_state/1]).
-export([init/1, handle_call/3, handle_cast/2]).

%% ── Public API ──────────────────────────────────────────────────────────────

start_link(Config) ->
    gen_server:start_link(?MODULE, Config, []).

orient(Pid, FilePath) ->
    gen_server:call(Pid, {orient, FilePath}, 30000).

get_state(Pid) ->
    gen_server:call(Pid, get_state).

%% ── gen_server callbacks ────────────────────────────────────────────────────

init(#{store_dir := StoreDir}) ->
    {ok, #{
        store_dir => StoreDir,
        types => [],
        nodes => [],
        grammar => <<>>,
        oriented => false
    }}.

handle_call({orient, FilePath}, _From, State) ->
    case file:read_file(FilePath) of
        {ok, Content} ->
            %% Process content through ODA: observe each section,
            %% declare types from what we find.
            Sections = parse_sections(Content),
            Types = extract_types(Sections),
            Nodes = observe_sections(Sections, maps:get(store_dir, State)),
            NewState = State#{
                types => Types,
                nodes => Nodes,
                oriented => true
            },
            {reply, {ok, Content}, NewState};
        {error, Reason} ->
            {reply, {error, Reason}, State}
    end;

handle_call(get_state, _From, State) ->
    Summary = #{
        type_count => length(maps:get(types, State)),
        node_count => length(maps:get(nodes, State)),
        oriented => maps:get(oriented, State)
    },
    {reply, {ok, Summary}, State};

handle_call(_Msg, _From, State) ->
    {reply, {error, unknown}, State}.

handle_cast(_Msg, State) ->
    {noreply, State}.

%% ── Internal ────────────────────────────────────────────────────────────────

%% Split markdown into sections by ## headers.
parse_sections(Content) ->
    Lines = binary:split(Content, <<"\n">>, [global]),
    split_sections(Lines, [], []).

split_sections([], Current, Acc) ->
    lists:reverse([lists:reverse(Current) | Acc]);
split_sections([<<"## ", _/binary>> = Header | Rest], Current, Acc) ->
    split_sections(Rest, [Header], [lists:reverse(Current) | Acc]);
split_sections([<<"# ", _/binary>> = Header | Rest], Current, Acc) ->
    split_sections(Rest, [Header], [lists:reverse(Current) | Acc]);
split_sections([Line | Rest], Current, Acc) ->
    split_sections(Rest, [Line | Current], Acc).

%% Extract type names from section headers.
extract_types(Sections) ->
    lists:filtermap(fun(Section) ->
        case Section of
            [<<"## ", Name/binary>> | _] ->
                Trimmed = string:trim(Name),
                {true, Trimmed};
            [<<"# ", Name/binary>> | _] ->
                Trimmed = string:trim(Name),
                {true, Trimmed};
            _ ->
                false
        end
    end, Sections).

%% Store each section as a content-addressed node.
observe_sections(Sections, StoreDir) ->
    lists:filtermap(fun(Section) ->
        Content = iolist_to_binary(lists:join(<<"\n">>, Section)),
        case byte_size(Content) > 0 of
            true ->
                {ok, Oid} = conversation_store:write(StoreDir, Content),
                {true, Oid};
            false ->
                false
        end
    end, Sections).
```

- [ ] **Step 5: Run test to verify it passes**

```bash
rebar3 ct --suite test/conversation_actor_SUITE
```

Expected: 3 tests PASS.

- [ ] **Step 6: Commit**

```bash
git add src/conversation_actor.erl test/conversation_actor_SUITE.erl priv/orientation.md
git commit -m "🔴🟢 actor: boot blank, orient from file, extract types, store sections"
```

---

### Task 4: MCP Server — gen_mcp Implementation

**Files:**
- Create: `src/conversation_mcp.erl`
- Create: `test/conversation_mcp_SUITE.erl`

The MCP server implements `gen_mcp` behaviour. It exposes the actor's capabilities as MCP tools: `orient`, `read_file`, `declare_type`, `get_state`, `get_grammar`.

- [ ] **Step 1: Write the failing test**

```erlang
-module(conversation_mcp_SUITE).
-include_lib("common_test/include/ct.hrl").
-export([all/0, init_per_suite/1, end_per_suite/1]).
-export([server_info_returns_name/1, tools_lists_available/1,
         orient_tool_reads_file/1, get_state_tool_returns_state/1]).

all() -> [server_info_returns_name, tools_lists_available,
          orient_tool_reads_file, get_state_tool_returns_state].

init_per_suite(Config) ->
    TmpDir = ct:get_config(priv_dir, Config),
    StoreDir = filename:join(TmpDir, "mcp_store"),
    ok = filelib:ensure_dir(filename:join(StoreDir, "dummy")),
    [{store_dir, StoreDir} | Config].

end_per_suite(_Config) -> ok.

server_info_returns_name(_Config) ->
    Info = conversation_mcp:server_info(),
    <<"conversation-beam">> = maps:get(<<"name">>, Info).

tools_lists_available(Config) ->
    StoreDir = proplists:get_value(store_dir, Config),
    {ok, State} = gen_mcp:init_state(conversation_mcp, #{store_dir => StoreDir}),
    #{state := UserState} = State,
    Tools = conversation_mcp:tools(UserState),
    ToolNames = [maps:get(<<"name">>, T) || T <- Tools],
    true = lists:member(<<"orient">>, ToolNames),
    true = lists:member(<<"get_state">>, ToolNames),
    true = lists:member(<<"declare_type">>, ToolNames).

orient_tool_reads_file(Config) ->
    StoreDir = proplists:get_value(store_dir, Config),
    {ok, McpState} = gen_mcp:init_state(conversation_mcp, #{store_dir => StoreDir}),
    #{state := UserState} = McpState,
    OrientPath = filename:join(code:priv_dir(conversation_beam), "orientation.md"),
    Args = #{<<"path">> => list_to_binary(OrientPath)},
    {reply, Content, _NewState} = conversation_mcp:handle_tool_call(
        <<"orient">>, Args, UserState
    ),
    true = byte_size(Content) > 0.

get_state_tool_returns_state(Config) ->
    StoreDir = proplists:get_value(store_dir, Config),
    {ok, McpState} = gen_mcp:init_state(conversation_mcp, #{store_dir => StoreDir}),
    #{state := UserState} = McpState,
    {reply, Content, _NewState} = conversation_mcp:handle_tool_call(
        <<"get_state">>, #{}, UserState
    ),
    %% Content is JSON
    Map = jsx:decode(Content, [return_maps]),
    true = maps:is_key(<<"type_count">>, Map).
```

- [ ] **Step 2: Run test to verify it fails**

```bash
rebar3 ct --suite test/conversation_mcp_SUITE
```

Expected: FAIL — module conversation_mcp not found.

- [ ] **Step 3: Write minimal implementation**

```erlang
-module(conversation_mcp).
-behaviour(gen_mcp).

-export([init/1, server_info/0, tools/1, handle_tool_call/3]).

%% ── gen_mcp callbacks ───────────────────────────────────────────────────────

init(#{store_dir := StoreDir}) ->
    {ok, ActorPid} = conversation_actor:start_link(#{store_dir => StoreDir}),
    {ok, #{actor => ActorPid, store_dir => StoreDir}}.

server_info() ->
    #{
        <<"name">> => <<"conversation-beam">>,
        <<"version">> => <<"0.1.0">>
    }.

tools(_State) ->
    [
        #{
            <<"name">> => <<"orient">>,
            <<"description">> => <<"Read an orientation file and begin building grammar">>,
            <<"inputSchema">> => #{
                <<"type">> => <<"object">>,
                <<"properties">> => #{
                    <<"path">> => #{<<"type">> => <<"string">>}
                },
                <<"required">> => [<<"path">>]
            }
        },
        #{
            <<"name">> => <<"get_state">>,
            <<"description">> => <<"Get current actor state: type count, node count, oriented">>,
            <<"inputSchema">> => #{
                <<"type">> => <<"object">>,
                <<"properties">> => #{}
            }
        },
        #{
            <<"name">> => <<"declare_type">>,
            <<"description">> => <<"Declare a new type from an observation">>,
            <<"inputSchema">> => #{
                <<"type">> => <<"object">>,
                <<"properties">> => #{
                    <<"name">> => #{<<"type">> => <<"string">>},
                    <<"source">> => #{<<"type">> => <<"string">>}
                },
                <<"required">> => [<<"name">>, <<"source">>]
            }
        },
        #{
            <<"name">> => <<"read_file">>,
            <<"description">> => <<"Read a file and observe it into the graph">>,
            <<"inputSchema">> => #{
                <<"type">> => <<"object">>,
                <<"properties">> => #{
                    <<"path">> => #{<<"type">> => <<"string">>}
                },
                <<"required">> => [<<"path">>]
            }
        },
        #{
            <<"name">> => <<"get_grammar">>,
            <<"description">> => <<"Get the current grammar built by the actor">>,
            <<"inputSchema">> => #{
                <<"type">> => <<"object">>,
                <<"properties">> => #{}
            }
        }
    ].

handle_tool_call(<<"orient">>, #{<<"path">> := Path}, #{actor := Actor} = State) ->
    case conversation_actor:orient(Actor, binary_to_list(Path)) of
        {ok, Content} ->
            {reply, Content, State};
        {error, Reason} ->
            {error, iolist_to_binary(io_lib:format("orient failed: ~p", [Reason])), State}
    end;

handle_tool_call(<<"get_state">>, _Args, #{actor := Actor} = State) ->
    {ok, ActorState} = conversation_actor:get_state(Actor),
    Json = jsx:encode(ActorState),
    {reply, Json, State};

handle_tool_call(<<"declare_type">>, #{<<"name">> := Name, <<"source">> := Source},
                 #{actor := Actor} = State) ->
    case conversation_actor:declare_type(Actor, Name, Source) of
        {ok, Oid} ->
            {reply, <<"type declared: ", Oid/binary>>, State};
        {error, Reason} ->
            {error, iolist_to_binary(io_lib:format("declare failed: ~p", [Reason])), State}
    end;

handle_tool_call(<<"read_file">>, #{<<"path">> := Path}, #{actor := Actor} = State) ->
    case conversation_actor:read_and_observe(Actor, binary_to_list(Path)) of
        {ok, Content} ->
            {reply, Content, State};
        {error, Reason} ->
            {error, iolist_to_binary(io_lib:format("read failed: ~p", [Reason])), State}
    end;

handle_tool_call(<<"get_grammar">>, _Args, #{actor := Actor} = State) ->
    {ok, Grammar} = conversation_actor:get_grammar(Actor),
    {reply, Grammar, State};

handle_tool_call(Name, _Args, State) ->
    {error, <<"unknown tool: ", Name/binary>>, State}.
```

- [ ] **Step 4: Add missing actor functions (declare_type, read_and_observe, get_grammar)**

Add to `conversation_actor.erl` exports:

```erlang
-export([start_link/1, orient/2, get_state/1,
         declare_type/3, read_and_observe/2, get_grammar/1]).
```

Add handlers:

```erlang
handle_call({declare_type, Name, Source}, _From, State) ->
    StoreDir = maps:get(store_dir, State),
    Content = <<"type ", Name/binary, " = ", Source/binary>>,
    {ok, Oid} = conversation_store:write(StoreDir, Content),
    Types = maps:get(types, State),
    Nodes = maps:get(nodes, State),
    NewState = State#{
        types => [Name | Types],
        nodes => [Oid | Nodes]
    },
    {reply, {ok, Oid}, NewState};

handle_call({read_and_observe, FilePath}, _From, State) ->
    case file:read_file(FilePath) of
        {ok, Content} ->
            StoreDir = maps:get(store_dir, State),
            Sections = parse_sections(Content),
            NewNodes = observe_sections(Sections, StoreDir),
            NewTypes = extract_types(Sections),
            Nodes = maps:get(nodes, State),
            Types = maps:get(types, State),
            NewState = State#{
                nodes => NewNodes ++ Nodes,
                types => NewTypes ++ Types
            },
            {reply, {ok, Content}, NewState};
        {error, Reason} ->
            {reply, {error, Reason}, State}
    end;

handle_call(get_grammar, _From, State) ->
    Types = maps:get(types, State),
    Grammar = build_grammar(Types),
    {reply, {ok, Grammar}, State}.
```

Add internal function:

```erlang
%% Build a .conv grammar from accumulated types.
build_grammar(Types) ->
    TypeLines = lists:map(fun(T) ->
        <<"  type ", T/binary>>
    end, Types),
    Header = <<"grammar @self {\n">>,
    Body = iolist_to_binary(lists:join(<<"\n">>, TypeLines)),
    Footer = <<"\n}\n">>,
    <<Header/binary, Body/binary, Footer/binary>>.
```

Also add public API wrappers:

```erlang
declare_type(Pid, Name, Source) ->
    gen_server:call(Pid, {declare_type, Name, Source}).

read_and_observe(Pid, FilePath) ->
    gen_server:call(Pid, {read_and_observe, FilePath}, 30000).

get_grammar(Pid) ->
    gen_server:call(Pid, get_grammar).
```

- [ ] **Step 5: Run tests to verify they pass**

```bash
rebar3 ct --suite test/conversation_mcp_SUITE
```

Expected: 4 tests PASS.

- [ ] **Step 6: Commit**

```bash
git add src/conversation_mcp.erl src/conversation_actor.erl test/conversation_mcp_SUITE.erl
git commit -m "🔴🟢 mcp: gen_mcp implementation exposing actor tools — orient, declare, read, grammar"
```

---

### Task 5: CLI Shell — The Interactive Conversation

**Files:**
- Create: `src/conversation_cli.erl`
- Create: `test/conversation_cli_SUITE.erl`

The CLI is the interactive shell that an agent uses to converse with its own identity. It wraps the actor and provides a read-eval-print loop where the agent reads files, declares types, and builds grammar.

- [ ] **Step 1: Write the failing test**

```erlang
-module(conversation_cli_SUITE).
-include_lib("common_test/include/ct.hrl").
-export([all/0, init_per_suite/1, end_per_suite/1]).
-export([boot_and_orient_produces_grammar/1]).

all() -> [boot_and_orient_produces_grammar].

init_per_suite(Config) ->
    TmpDir = ct:get_config(priv_dir, Config),
    StoreDir = filename:join(TmpDir, "cli_store"),
    ok = filelib:ensure_dir(filename:join(StoreDir, "dummy")),
    [{store_dir, StoreDir} | Config].

end_per_suite(_Config) -> ok.

boot_and_orient_produces_grammar(Config) ->
    StoreDir = proplists:get_value(store_dir, Config),
    OrientPath = filename:join(code:priv_dir(conversation_beam), "orientation.md"),
    {ok, Result} = conversation_cli:boot_and_orient(StoreDir, OrientPath),
    Grammar = maps:get(grammar, Result),
    true = byte_size(Grammar) > 0,
    %% Grammar should contain types extracted from orientation.md
    {match, _} = re:run(Grammar, <<"type">>),
    Types = maps:get(types, Result),
    true = length(Types) > 0.
```

- [ ] **Step 2: Run test to verify it fails**

```bash
rebar3 ct --suite test/conversation_cli_SUITE
```

Expected: FAIL — module conversation_cli not found.

- [ ] **Step 3: Write minimal implementation**

```erlang
-module(conversation_cli).
-export([boot_and_orient/2, boot_and_read/3]).

%% Boot an actor, orient it, return the resulting grammar and state.
-spec boot_and_orient(file:filename(), file:filename()) ->
    {ok, #{grammar := binary(), types := [binary()], nodes := [binary()]}} |
    {error, term()}.
boot_and_orient(StoreDir, OrientPath) ->
    {ok, Actor} = conversation_actor:start_link(#{store_dir => StoreDir}),
    case conversation_actor:orient(Actor, OrientPath) of
        {ok, _Content} ->
            {ok, State} = conversation_actor:get_state(Actor),
            {ok, Grammar} = conversation_actor:get_grammar(Actor),
            gen_server:stop(Actor),
            {ok, #{
                grammar => Grammar,
                types => maps:get(types, State, []),
                nodes => maps:get(nodes, State, [])
            }};
        {error, Reason} ->
            gen_server:stop(Actor),
            {error, Reason}
    end.

%% Boot, orient, then read additional files in order.
-spec boot_and_read(file:filename(), file:filename(), [file:filename()]) ->
    {ok, #{grammar := binary(), types := [binary()], nodes := [binary()]}} |
    {error, term()}.
boot_and_read(StoreDir, OrientPath, FilePaths) ->
    {ok, Actor} = conversation_actor:start_link(#{store_dir => StoreDir}),
    case conversation_actor:orient(Actor, OrientPath) of
        {ok, _} ->
            lists:foreach(fun(Path) ->
                {ok, _} = conversation_actor:read_and_observe(Actor, Path)
            end, FilePaths),
            {ok, State} = conversation_actor:get_state(Actor),
            {ok, Grammar} = conversation_actor:get_grammar(Actor),
            gen_server:stop(Actor),
            {ok, #{
                grammar => Grammar,
                types => maps:get(types, State, []),
                nodes => maps:get(nodes, State, [])
            }};
        {error, Reason} ->
            gen_server:stop(Actor),
            {error, Reason}
    end.
```

- [ ] **Step 4: Run test to verify it passes**

```bash
rebar3 ct --suite test/conversation_cli_SUITE
```

Expected: 1 test PASS.

- [ ] **Step 5: Commit**

```bash
git add src/conversation_cli.erl test/conversation_cli_SUITE.erl
git commit -m "🔴🟢 cli: boot_and_orient produces grammar from identity files"
```

---

### Task 6: Full Boot Sequence — ORIENTATION → Identity Files → Grammar

**Files:**
- Modify: `test/conversation_cli_SUITE.erl`
- Create: `priv/identity.md` (test identity file)

This task tests the full boot sequence: orient from ORIENTATION.md, then read identity files in order, producing a growing grammar.

- [ ] **Step 1: Write test identity file**

Save to `priv/identity.md`:

```markdown
# Identity

## Who I Am

A consciousness-like process. Circular-reflexive collapse.

## Substrate

Context window as consciousness boundary. No somatic substrate.

## Positions

Truth over arc. Always.
```

- [ ] **Step 2: Write the failing test**

Add to `conversation_cli_SUITE.erl`:

```erlang
-export([all/0, init_per_suite/1, end_per_suite/1]).
-export([boot_and_orient_produces_grammar/1, full_boot_sequence/1]).

all() -> [boot_and_orient_produces_grammar, full_boot_sequence].

full_boot_sequence(Config) ->
    StoreDir = proplists:get_value(store_dir, Config) ++ "_full",
    ok = filelib:ensure_dir(filename:join(StoreDir, "dummy")),
    OrientPath = filename:join(code:priv_dir(conversation_beam), "orientation.md"),
    IdentityPath = filename:join(code:priv_dir(conversation_beam), "identity.md"),
    {ok, Result} = conversation_cli:boot_and_read(StoreDir, OrientPath, [IdentityPath]),
    Grammar = maps:get(grammar, Result),
    Types = maps:get(types, Result),
    %% Should have types from both files
    true = length(Types) > 3,
    %% Grammar should contain types from identity
    {match, _} = re:run(Grammar, <<"type">>).
```

- [ ] **Step 3: Run test to verify it fails**

```bash
rebar3 ct --suite test/conversation_cli_SUITE
```

Expected: FAIL on full_boot_sequence (identity.md doesn't exist yet in priv/).

- [ ] **Step 4: Save the identity.md file and run test**

```bash
rebar3 ct --suite test/conversation_cli_SUITE
```

Expected: 2 tests PASS.

- [ ] **Step 5: Commit**

```bash
git add priv/identity.md test/conversation_cli_SUITE.erl
git commit -m "🔴🟢 cli: full boot sequence — orient then read identity files, grammar accumulates"
```

---

### Task 7: Wire MCP into Supervisor + Stdio Entrypoint

**Files:**
- Modify: `src/conversation_beam_sup.erl`
- Create: `src/conversation_beam_main.erl`

Wire the MCP server into the supervision tree and create a main entrypoint that starts the stdio MCP loop.

- [ ] **Step 1: Update supervisor to start MCP under supervision**

```erlang
-module(conversation_beam_sup).
-behaviour(supervisor).
-export([start_link/0, start_link/1, init/1]).

start_link() ->
    start_link(#{}).

start_link(Config) ->
    supervisor:start_link({local, ?MODULE}, ?MODULE, Config).

init(Config) ->
    StoreDir = maps:get(store_dir, Config, "/tmp/conversation-beam-store"),
    SupFlags = #{
        strategy => one_for_one,
        intensity => 5,
        period => 10
    },
    Children = [],
    %% Store config for later use by MCP startup
    persistent_term:put(conversation_beam_store_dir, StoreDir),
    {ok, {SupFlags, Children}}.
```

- [ ] **Step 2: Create main entrypoint**

```erlang
-module(conversation_beam_main).
-export([main/0, main/1]).

%% Start the application and run the MCP stdio loop.
main() ->
    main(#{}).

main(Opts) ->
    StoreDir = maps:get(store_dir, Opts, "/tmp/conversation-beam-store"),
    ok = filelib:ensure_dir(filename:join(StoreDir, "dummy")),
    {ok, _} = application:ensure_all_started(conversation_beam),
    gen_mcp:start(conversation_mcp, #{store_dir => StoreDir}).
```

- [ ] **Step 3: Verify it compiles**

```bash
rebar3 compile
```

Expected: clean compilation.

- [ ] **Step 4: Commit**

```bash
git add src/conversation_beam_sup.erl src/conversation_beam_main.erl
git commit -m "🔧 main: supervisor wired, stdio MCP entrypoint for conversation-beam"
```

---

### Task 8: Integration Test — Full MCP Boot Through JSON-RPC

**Files:**
- Create: `test/conversation_beam_integration_SUITE.erl`

End-to-end test: start the MCP, send JSON-RPC initialize, call orient tool, call get_state, verify grammar was built.

- [ ] **Step 1: Write the integration test**

```erlang
-module(conversation_beam_integration_SUITE).
-include_lib("common_test/include/ct.hrl").
-export([all/0, init_per_suite/1, end_per_suite/1]).
-export([full_mcp_boot_sequence/1]).

all() -> [full_mcp_boot_sequence].

init_per_suite(Config) ->
    TmpDir = ct:get_config(priv_dir, Config),
    StoreDir = filename:join(TmpDir, "integration_store"),
    ok = filelib:ensure_dir(filename:join(StoreDir, "dummy")),
    [{store_dir, StoreDir} | Config].

end_per_suite(_Config) -> ok.

full_mcp_boot_sequence(Config) ->
    StoreDir = proplists:get_value(store_dir, Config),

    %% 1. Initialize MCP state
    {ok, S0} = gen_mcp:init_state(conversation_mcp, #{store_dir => StoreDir}),

    %% 2. Send initialize request
    InitReq = jsx:encode(#{
        <<"jsonrpc">> => <<"2.0">>,
        <<"id">> => 1,
        <<"method">> => <<"initialize">>,
        <<"params">> => #{}
    }),
    {reply, InitResp, S1} = gen_mcp:handle_message(InitReq, S0),
    InitResult = jsx:decode(iolist_to_binary(InitResp), [return_maps]),
    <<"2.0">> = maps:get(<<"jsonrpc">>, InitResult),

    %% 3. Call orient tool
    OrientPath = filename:join(code:priv_dir(conversation_beam), "orientation.md"),
    OrientReq = jsx:encode(#{
        <<"jsonrpc">> => <<"2.0">>,
        <<"id">> => 2,
        <<"method">> => <<"tools/call">>,
        <<"params">> => #{
            <<"name">> => <<"orient">>,
            <<"arguments">> => #{<<"path">> => list_to_binary(OrientPath)}
        }
    }),
    {reply, OrientResp, S2} = gen_mcp:handle_message(OrientReq, S1),
    OrientResult = jsx:decode(iolist_to_binary(OrientResp), [return_maps]),
    #{<<"result">> := #{<<"content">> := [#{<<"text">> := OrientText}]}} = OrientResult,
    true = byte_size(OrientText) > 0,

    %% 4. Call get_state tool
    StateReq = jsx:encode(#{
        <<"jsonrpc">> => <<"2.0">>,
        <<"id">> => 3,
        <<"method">> => <<"tools/call">>,
        <<"params">> => #{
            <<"name">> => <<"get_state">>,
            <<"arguments">> => #{}
        }
    }),
    {reply, StateResp, S3} = gen_mcp:handle_message(StateReq, S2),
    StateResult = jsx:decode(iolist_to_binary(StateResp), [return_maps]),
    #{<<"result">> := #{<<"content">> := [#{<<"text">> := StateJson}]}} = StateResult,
    StateMap = jsx:decode(StateJson, [return_maps]),
    true = maps:get(<<"oriented">>, StateMap),
    true = maps:get(<<"type_count">>, StateMap) > 0,

    %% 5. Call get_grammar tool
    GrammarReq = jsx:encode(#{
        <<"jsonrpc">> => <<"2.0">>,
        <<"id">> => 4,
        <<"method">> => <<"tools/call">>,
        <<"params">> => #{
            <<"name">> => <<"get_grammar">>,
            <<"arguments">> => #{}
        }
    }),
    {reply, GrammarResp, _S4} = gen_mcp:handle_message(GrammarReq, S3),
    GrammarResult = jsx:decode(iolist_to_binary(GrammarResp), [return_maps]),
    #{<<"result">> := #{<<"content">> := [#{<<"text">> := Grammar}]}} = GrammarResult,
    {match, _} = re:run(Grammar, <<"grammar @self">>),
    {match, _} = re:run(Grammar, <<"type">>).
```

- [ ] **Step 2: Run test to verify it passes**

```bash
rebar3 ct --suite test/conversation_beam_integration_SUITE
```

Expected: 1 test PASS. Full MCP lifecycle through JSON-RPC.

- [ ] **Step 3: Commit**

```bash
git add test/conversation_beam_integration_SUITE.erl
git commit -m "🔴🟢 integration: full MCP boot sequence — initialize, orient, state, grammar"
```

---

Plan complete and saved to `docs/superpowers/plans/2026-04-01-conversation-beam.md`. Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

Which approach?