import conversation/boot
import conversation/domain
import conversation/loader
import gleam/dynamic/decode
import gleam/erlang/process
import gleam/list
import gleeunit/should

const reed_grammar = "grammar @reed {
  type = signal | memory | quote

  type signal = message | correction | insight

  type memory = session | pattern | position

  type quote = observation | crystallization
}

in @ai
in @actor
in @reality
"

/// Reed boots on the BEAM.
pub fn reed_boots_test() {
  let assert Ok(result) = boot.boot([reed_grammar])

  // Domain server running
  should.be_true(domain.is_running("reed"))

  // Module loaded
  should.be_true(loader.is_loaded("conv_reed"))

  // Booted domain reports alive
  let assert Ok(reed) =
    list.find(result.domains, fn(d) { d.domain == "reed" })
  should.be_true(boot.is_alive(reed))

  boot.shutdown(result)
}

/// Boot multiple grammars at once.
pub fn boot_multiple_grammars_test() {
  let erlang_grammar =
    "grammar @native_boot {
  type = module | function
  type module = atom
  type function = atom

  action exec {
    module: module
    function: function
    args: list
  }
}

in @tools
in @reality
"

  let assert Ok(result) = boot.boot([reed_grammar, erlang_grammar])

  // Both domains running
  should.be_true(domain.is_running("reed"))
  should.be_true(domain.is_running("native_boot"))

  // exec works through booted domain
  let assert Ok(val) =
    domain.exec("native_boot", "erlang", "abs", [-7])
  let assert Ok(7) = decode.run(val, decode.int)

  boot.shutdown(result)
}

/// Boot Reed from the actual garden files.
pub fn boot_reed_from_garden_test() {
  let garden =
    "/Users/alexwolf/dev/systemic.engineering/garden/public"
  let assert Ok(result) =
    boot.boot_from_files([
      garden <> "/@reed/reed.conv",
      garden <> "/@erlang/erlang.conv",
    ])

  // Reed is alive on the BEAM
  should.be_true(domain.is_running("reed"))
  should.be_true(loader.is_loaded("conv_reed"))

  // @erlang proxy is alive (conv_erlang avoids sticky collision)
  should.be_true(domain.is_running("erlang"))
  should.be_true(loader.is_loaded("conv_erlang"))

  // exec through @erlang: touch reality
  let assert Ok(val) =
    domain.exec("erlang", "erlang", "abs", [-99])
  let assert Ok(99) = decode.run(val, decode.int)

  boot.shutdown(result)
}

/// Boot populates lens dependencies from compiled modules.
pub fn boot_populates_lenses_test() {
  let inner = "grammar @tools {
  type = hammer | wrench
}
"
  let outer = "grammar @workshop {
  type = job
  action build {
    tool: type
  }
}
in @tools
"

  let assert Ok(result) = boot.boot([inner, outer])

  // Workshop imports @tools
  let assert Ok(workshop) =
    list.find(result.domains, fn(d) { d.domain == "workshop" })
  should.equal(workshop.lenses, ["tools"])

  // Tools has no imports
  let assert Ok(tools) =
    list.find(result.domains, fn(d) { d.domain == "tools" })
  should.equal(tools.lenses, [])

  // All imports satisfied — both domains are booted
  should.be_true(boot.imports_resolved(result))

  boot.shutdown(result)
}

/// Imports not resolved when dependency is missing.
pub fn boot_unresolved_imports_test() {
  // This grammar imports @phantom, which we don't compile
  let lonely = "grammar @lonely {
  type = echo
}
in @phantom
"

  let assert Ok(result) = boot.boot([lonely])

  // Lenses populated from the compiled module
  let assert Ok(d) =
    list.find(result.domains, fn(d) { d.domain == "lonely" })
  should.equal(d.lenses, ["phantom"])

  // Not all imports resolved — @phantom is not booted
  should.be_false(boot.imports_resolved(result))

  boot.shutdown(result)
}

/// Supervisor restarts crashed domain servers.
pub fn supervisor_restarts_domain_test() {
  let grammar = "grammar @phoenix {
  type = flame
  action rise {
    from: type
  }
}
"

  let assert Ok(result) = boot.boot([grammar])
  should.be_true(domain.is_running("phoenix"))

  // Kill the domain server
  domain.kill("phoenix")
  // Brief sleep to let supervisor restart
  process.sleep(50)

  // Supervisor should have restarted it
  should.be_true(domain.is_running("phoenix"))

  boot.shutdown(result)
}

/// Boot populates extends from compiled modules.
pub fn boot_populates_extends_test() {
  let parent = "grammar @smash {
  type = move | attack
}
"
  let child = "grammar @fox extends @smash {
  type = dodge | counter
}
"

  let assert Ok(result) = boot.boot([parent, child])

  // Fox extends @smash
  let assert Ok(fox) =
    list.find(result.domains, fn(d) { d.domain == "fox" })
  should.equal(fox.extends, ["smash"])

  // Smash has no extends
  let assert Ok(smash) =
    list.find(result.domains, fn(d) { d.domain == "smash" })
  should.equal(smash.extends, [])

  // All extends satisfied — both domains are booted
  should.be_true(boot.extends_resolved(result))

  boot.shutdown(result)
}

/// Extends not resolved when parent is missing.
pub fn boot_unresolved_extends_test() {
  let orphan = "grammar @orphan extends @missing {
  type = lost
}
"

  let assert Ok(result) = boot.boot([orphan])

  // Extends populated from the compiled module
  let assert Ok(d) =
    list.find(result.domains, fn(d) { d.domain == "orphan" })
  should.equal(d.extends, ["missing"])

  // Not all extends resolved — @missing is not booted
  should.be_false(boot.extends_resolved(result))

  boot.shutdown(result)
}

/// Boot then exec proves the full loop: grammar → module → server → reality.
pub fn boot_exec_reality_test() {
  let native_grammar =
    "grammar @boot_exec {
  type = module | function
  action exec {
    module: module
    function: function
    args: list
  }
}
in @reality
"

  let assert Ok(result) = boot.boot([native_grammar])

  // The loop: grammar compiled → module loaded → domain server running → exec → apply/3
  let assert Ok(val) =
    domain.exec("boot_exec", "erlang", "integer_to_binary", [42])
  let assert Ok("42") = decode.run(val, decode.string)

  boot.shutdown(result)
}
