import conversation/boot
import conversation/coincidence
import conversation/domain
import conversation/loader
import gleam/list
import gleeunit/should

/// Inline grammar with requires — proves the full pipeline without garden files.
/// Grammar → Rust NIF → ETF → BEAM module → requires/0 → @coincidence.check_property → NIF → pass.
pub fn inline_grammar_with_requires_test() {
  let source =
    "grammar @inline_prop {
  type = a | b | c

  requires shannon_equivalence
}
"

  let assert Ok(result) =
    boot.boot_with_infrastructure([], [source])

  should.be_true(domain.is_running("inline_prop"))
  should.be_true(loader.is_loaded("conv_inline_prop"))

  // Verify the module actually exposes requires/0
  let assert Ok(requires) = loader.get_requires("conv_inline_prop")
  should.equal(requires, ["shannon_equivalence"])

  boot.shutdown(result)
  let _ = coincidence.stop_server()
}

/// Inline grammar with both requires and invariant.
pub fn inline_grammar_with_requires_and_invariant_test() {
  let source =
    "grammar @dual_prop {
  type = x | y | z

  requires shannon_equivalence
  invariant connected
}
"

  let assert Ok(result) =
    boot.boot_with_infrastructure([], [source])

  should.be_true(domain.is_running("dual_prop"))

  let assert Ok(requires) = loader.get_requires("conv_dual_prop")
  let assert Ok(invariants) = loader.get_invariants("conv_dual_prop")
  should.equal(requires, ["shannon_equivalence"])
  should.equal(invariants, ["connected"])

  boot.shutdown(result)
  let _ = coincidence.stop_server()
}

/// Full pipeline: boot infrastructure domains, then @training with
/// requires/invariant. Properties checked through @coincidence.
///
/// This is the capstone test proving the full pipeline works end-to-end:
/// 1. Grammar declares `requires shannon_equivalence` and `invariant connected`
/// 2. Rust compiler emits `requires/0` and `invariants/0` in BEAM module
/// 3. Compiler actor reads `requires/0` and `invariants/0`
/// 4. Compiler actor calls `@coincidence.check_property(source, property_name)`
/// 5. @coincidence domain server routes to NIF
/// 6. NIF evaluates shannon equivalence and connected
/// 7. Properties pass, compilation succeeds
pub fn full_property_pipeline_test() {
  // 1. Read infrastructure grammars from conv/ and application from garden
  let conv = "/Users/alexwolf/dev/projects/conversation/conv"
  let garden =
    "/Users/alexwolf/dev/systemic.engineering/garden/public"
  let assert Ok(property_source) =
    boot.read_file(conv <> "/property.conv")
  let assert Ok(topology_source) =
    boot.read_file(conv <> "/topology.conv")
  let assert Ok(training_source) =
    boot.read_file(garden <> "/@training/training.conv")

  // 2. Boot with ordering: infrastructure first, then application
  let assert Ok(result) =
    boot.boot_with_infrastructure(
      [property_source, topology_source],
      [training_source],
    )

  // 3. Verify all domains are running
  should.be_true(domain.is_running("property"))
  should.be_true(domain.is_running("topology"))
  should.be_true(domain.is_running("training"))

  // 4. Verify modules are loaded
  should.be_true(loader.is_loaded("conv_property"))
  should.be_true(loader.is_loaded("conv_topology"))
  should.be_true(loader.is_loaded("conv_training"))

  // 5. Verify @training has the expected property declarations
  let assert Ok(requires) = loader.get_requires("conv_training")
  let assert Ok(invariants) = loader.get_invariants("conv_training")
  should.equal(requires, ["shannon_equivalence"])
  should.equal(invariants, ["connected"])

  // 6. Verify @coincidence was started (by boot_with_infrastructure)
  should.be_true(coincidence.is_running())

  // 7. Verify infrastructure domains appear before application domains
  let names = list.map(result.domains, fn(d) { d.domain })
  should.equal(names, ["property", "topology", "training"])

  // 8. Cleanup
  boot.shutdown(result)
  let _ = coincidence.stop_server()
}
