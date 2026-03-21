-module(conversation_sup).
-behaviour(supervisor).

%% Supervisor for domain servers.
%%
%% simple_one_for_one: each domain server is a child started dynamically
%% via start_domain/1. If a domain server crashes, the supervisor restarts
%% it — the domain atom re-registers and the compiled module's
%% gen_server:call routes work again.

-export([start_link/0, start_domain/1, init/1]).

%% Start the supervisor. Registered as 'conversation_sup'.
start_link() ->
    supervisor:start_link({local, ?MODULE}, ?MODULE, []).

%% Dynamically start a supervised domain server.
%% Returns {ok, Pid} or {error, Reason}.
start_domain(Domain) when is_binary(Domain) ->
    supervisor:start_child(?MODULE, [Domain]).

init([]) ->
    ChildSpec = #{
        id => domain_server,
        start => {domain_server, start_link, []},
        restart => transient,
        type => worker
    },
    {ok, {#{strategy => simple_one_for_one, intensity => 5, period => 60}, [ChildSpec]}}.
