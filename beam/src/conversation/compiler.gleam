//// Compiler — @compiler actor.
////
//// The @compiler receives compilation requests and produces witnessed
//// Trace(Artifact) results. Compilation targets are defined by garden
//// packages (@fortran, @elixir, @gleam, @rust) which inherit from
//// @compiler via `in @compiler`.
////
//// Current state: actor infrastructure ready, codegen routes through
//// Rust NIF (compile.rs → EAF). Gleam codegen modules superseded by
//// garden .conv packages.

import conversation/key
import conversation/oid
import conversation/ref.{type ScopedOid}
import conversation/trace.{type Trace}
import gleam/erlang/process.{type Subject}
import gleam/option.{None}
import gleam/otp/actor

/// Compile target — the language to emit.
pub type Target {
  Gleam
  Elixir
  Erlang
  Fortran
  Eaf
}

/// A compiled artifact produced by the @compiler actor.
pub type Artifact {
  Source(target: Target, source: String, oid: oid.Oid)
}

/// Messages the @compiler actor accepts.
pub type Message {
  Compile(
    source: String,
    target: Target,
    reply: Subject(Result(Trace(Artifact), String)),
  )
  Shutdown
}

type State {
  State(kp: key.KeyPair, actor_oid: ScopedOid(key.Key))
}

/// The @compiler actor's deterministic public key.
pub fn public_key() -> key.Key {
  key.from_seed(do_sha256(<<"compiler":utf8>>))
  |> key.public_key
}

/// Start the @compiler actor.
pub fn start() -> actor.StartResult(Subject(Message)) {
  let kp = key.from_seed(do_sha256(<<"compiler":utf8>>))
  let actor_oid = key.oid(key.public_key(kp))
  let state = State(kp: kp, actor_oid: actor_oid)
  actor.new(state)
  |> actor.on_message(handle_message)
  |> actor.start
}

fn handle_message(state: State, msg: Message) -> actor.Next(State, Message) {
  case msg {
    Compile(source, target, reply) -> {
      let source_oid = oid.from_bytes(<<source:utf8>>)
      let artifact = Source(target: target, source: source, oid: source_oid)
      let t = trace.new(state.actor_oid, state.kp, artifact, None)
      process.send(reply, Ok(t))
      actor.continue(state)
    }
    Shutdown -> actor.stop()
  }
}

@external(erlang, "crypto_ffi", "sha256")
fn do_sha256(data: BitArray) -> BitArray
