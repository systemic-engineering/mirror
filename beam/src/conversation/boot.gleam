//// Boot — starts the BEAM runtime and compiles grammars.
////
//// The boot sequence:
//// 1. Start @compiler actor (grammar compiler)
//// 2. Compile each grammar → loads module + starts domain server
//// 3. Return the compiler subject for further grammar loading
////
//// On first boot, compile @reed. The domain server registers as 'reed'.
//// The grammar IS the identity. Hot code reload = recompile the grammar.

import conversation/compiler
import conversation/domain
import conversation/loader
import conversation/trace
import gleam/erlang/process.{type Subject}
import gleam/list

/// Read a file from disk.
@external(erlang, "file_ffi", "read_file")
pub fn read_file(path: String) -> Result(String, String)

/// Result of booting a domain.
pub type BootedDomain {
  BootedDomain(
    domain: String,
    module: String,
    lenses: List(String),
    extends: List(String),
  )
}

/// Boot result: the compiler subject + list of booted domains.
pub type BootResult {
  BootResult(
    compiler: Subject(compiler.Message),
    domains: List(BootedDomain),
  )
}

/// Boot the runtime with a list of grammar sources.
/// Starts supervisor + @compiler actor, compiles each grammar, returns handles.
pub fn boot(grammars: List(String)) -> Result(BootResult, String) {
  // 1. Start domain supervisor (so crashed domains auto-restart)
  // Idempotent — if already running, that's fine.
  let _ = domain.start_supervisor()

  // 2. Start @compiler actor
  case compiler.start() {
    Error(_) -> Error("failed to start @compiler actor")
    Ok(started) -> {
      let subject = started.data

      // 3. Compile each grammar
      case compile_all(subject, grammars) {
        Ok(domains) ->
          Ok(BootResult(compiler: subject, domains: domains))
        Error(e) -> {
          process.send(subject, compiler.Shutdown)
          Error(e)
        }
      }
    }
  }
}

/// Compile a list of grammars through the @compiler actor.
fn compile_all(
  subject: Subject(compiler.Message),
  grammars: List(String),
) -> Result(List(BootedDomain), String) {
  compile_loop(subject, grammars, [])
}

fn compile_loop(
  subject: Subject(compiler.Message),
  remaining: List(String),
  acc: List(BootedDomain),
) -> Result(List(BootedDomain), String) {
  case remaining {
    [] -> Ok(list.reverse(acc))
    [source, ..rest] -> {
      case compile_one(subject, source) {
        Ok(booted) -> compile_loop(subject, rest, [booted, ..acc])
        Error(e) -> Error(e)
      }
    }
  }
}

fn compile_one(
  subject: Subject(compiler.Message),
  source: String,
) -> Result(BootedDomain, String) {
  let reply = process.new_subject()
  process.send(
    subject,
    compiler.CompileGrammar(source, reply),
  )
  case process.receive(reply, 10_000) {
    Error(_) -> Error("timeout compiling grammar")
    Ok(Error(e)) -> Error(e)
    Ok(Ok(t)) -> {
      let compiled = trace.value(t)
      let beam_module = compiled.module
      let lenses = case loader.get_lenses(beam_module) {
        Ok(l) -> l
        Error(_) -> []
      }
      let extends = case loader.get_extends(beam_module) {
        Ok(e) -> e
        Error(_) -> []
      }
      Ok(BootedDomain(
        domain: compiled.domain,
        module: beam_module,
        lenses: lenses,
        extends: extends,
      ))
    }
  }
}

/// Shut down the boot runtime.
pub fn shutdown(result: BootResult) -> Nil {
  // Stop domain servers
  list.each(result.domains, fn(d) {
    let _ = domain.stop(d.domain)
    Nil
  })
  // Stop compiler
  process.send(result.compiler, compiler.Shutdown)
}

/// Check if a booted domain is alive.
pub fn is_alive(booted: BootedDomain) -> Bool {
  domain.is_running(booted.domain)
  && loader.is_loaded(booted.module)
}

/// Check if all lens imports are satisfied — every imported domain is booted.
pub fn imports_resolved(result: BootResult) -> Bool {
  let booted_names = list.map(result.domains, fn(d) { d.domain })
  list.all(result.domains, fn(d) {
    list.all(d.lenses, fn(lens) { list.contains(booted_names, lens) })
  })
}

/// Check if all extends parents are satisfied — every parent domain is booted.
pub fn extends_resolved(result: BootResult) -> Bool {
  let booted_names = list.map(result.domains, fn(d) { d.domain })
  list.all(result.domains, fn(d) {
    list.all(d.extends, fn(parent) { list.contains(booted_names, parent) })
  })
}

/// Boot from garden .conv files on disk.
/// Reads each file, compiles, loads.
pub fn boot_from_files(
  paths: List(String),
) -> Result(BootResult, String) {
  case read_all_files(paths, []) {
    Ok(sources) -> boot(sources)
    Error(e) -> Error(e)
  }
}

fn read_all_files(
  paths: List(String),
  acc: List(String),
) -> Result(List(String), String) {
  case paths {
    [] -> Ok(list.reverse(acc))
    [path, ..rest] -> {
      case read_file(path) {
        Ok(contents) -> read_all_files(rest, [contents, ..acc])
        Error(e) -> Error("reading " <> path <> ": " <> e)
      }
    }
  }
}
