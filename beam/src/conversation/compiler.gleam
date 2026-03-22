//// Compiler — @compiler actor.
////
//// The @compiler receives .conv source, compiles the grammar block via
//// the Rust NIF, loads the compiled module onto the BEAM, and returns a
//// witnessed Trace(CompiledDomain).
////
//// Identity is deterministic: sha256("compiler") → Ed25519 keypair.
////
//// Two start modes:
//// - start()       — imperative path. Starts domain supervisor, manages
////                    domain server lifecycle on compile. Backwards compatible.
//// - start_named() — supervised path. Pure compilation only. The garden
////                    factory supervisor handles domain server lifecycle.

import conversation/domain
import conversation/grammar
import conversation/key
import conversation/loader
import conversation/nif
import conversation/oid
import conversation/ref.{type ScopedOid}
import conversation/trace.{type Trace}
import gleam/erlang/process.{type Subject}
import gleam/option.{None}
import gleam/otp/actor

/// A compiled domain grammar.
pub type CompiledDomain {
  CompiledDomain(domain: String, source_oid: oid.Oid, module: String)
}

/// Messages the @compiler actor accepts.
pub type Message {
  CompileGrammar(
    source: String,
    reply: Subject(Result(Trace(CompiledDomain), String)),
  )
  Shutdown
}

type State {
  State(
    kp: key.KeyPair,
    actor_oid: ScopedOid(key.Key),
    manage_domains: Bool,
  )
}

/// The @compiler actor's deterministic public key.
pub fn public_key() -> key.Key {
  key.from_seed(do_sha256(<<"compiler":utf8>>))
  |> key.public_key
}

/// Start the @compiler actor (imperative path).
/// Starts the domain supervisor and manages domain server lifecycle
/// on each compile. Use this for backwards compatibility with the
/// existing boot path.
pub fn start() -> actor.StartResult(Subject(Message)) {
  // Ensure the domain supervisor is running so compiled grammars
  // can start supervised domain servers.
  let _ = domain.start_supervisor()
  do_start(True)
}

/// Start the @compiler actor with a registered name (supervised path).
/// Does NOT start a domain supervisor or manage domain servers.
/// Pure compilation: grammar → NIF → ETF → BEAM module → trace.
/// The garden factory supervisor handles domain server lifecycle.
pub fn start_named(
  name: process.Name(Message),
) -> actor.StartResult(Subject(Message)) {
  do_start_named(name)
}

fn do_start(manage_domains: Bool) -> actor.StartResult(Subject(Message)) {
  let kp = key.from_seed(do_sha256(<<"compiler":utf8>>))
  let actor_oid = key.oid(key.public_key(kp))
  let state =
    State(kp: kp, actor_oid: actor_oid, manage_domains: manage_domains)
  actor.new(state)
  |> actor.on_message(handle_message)
  |> actor.start
}

fn do_start_named(
  name: process.Name(Message),
) -> actor.StartResult(Subject(Message)) {
  let kp = key.from_seed(do_sha256(<<"compiler":utf8>>))
  let actor_oid = key.oid(key.public_key(kp))
  let state = State(kp: kp, actor_oid: actor_oid, manage_domains: False)
  actor.new(state)
  |> actor.on_message(handle_message)
  |> actor.named(name)
  |> actor.start
}

fn handle_message(state: State, msg: Message) -> actor.Next(State, Message) {
  case msg {
    CompileGrammar(source, reply) -> {
      let source_oid = oid.from_bytes(<<source:utf8>>)
      case nif.compile_grammar(source) {
        Ok(etf) -> {
          let domain_name = case grammar.from_source(source) {
            Ok(g) -> grammar.domain(g)
            Error(_) -> "unknown"
          }
          case loader.load_etf_module(etf) {
            Ok(module) -> {
              // Only manage domain servers in imperative mode.
              // In supervised mode, the garden handles this.
              case state.manage_domains {
                True ->
                  case domain.is_running(domain_name) {
                    False -> {
                      let _ = domain.start_supervised(domain_name)
                      Nil
                    }
                    True -> Nil
                  }
                False -> Nil
              }
              let compiled =
                CompiledDomain(
                  domain: domain_name,
                  source_oid: source_oid,
                  module: module,
                )
              let t = trace.new(state.actor_oid, state.kp, compiled, None)
              process.send(reply, Ok(t))
            }
            Error(e) -> process.send(reply, Error(e))
          }
        }
        Error(e) -> process.send(reply, Error(e))
      }
      actor.continue(state)
    }
    Shutdown -> actor.stop()
  }
}

@external(erlang, "crypto_ffi", "sha256")
fn do_sha256(data: BitArray) -> BitArray
