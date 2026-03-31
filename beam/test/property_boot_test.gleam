import conversation/boot
import conversation/coincidence
import conversation/domain
import conversation/loader
import gleam/list
import gleam/string
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
/// requires/invariant. Properties enforced through @coincidence.
///
/// @training declares `invariant connected` but has 6 disconnected type
/// groups, so compilation correctly fails with property enforcement.
/// This test proves enforcement catches real violations end-to-end:
/// 1. Grammar declares `requires shannon_equivalence` and `invariant connected`
/// 2. Rust compiler emits `requires/0` and `invariants/0` in BEAM module
/// 3. Compiler actor reads `requires/0` and `invariants/0`
/// 4. Compiler actor calls `@coincidence.check_property(source, property_name)`
/// 5. @coincidence domain server routes to NIF
/// 6. NIF evaluates shannon equivalence (passes) and connected (fails)
/// 7. Compilation fails with property enforcement error
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
  let result =
    boot.boot_with_infrastructure(
      [property_source, topology_source],
      [training_source],
    )

  // 3. @training has disconnected type groups — enforcement correctly rejects
  should.be_error(result)
  let assert Error(reason) = result
  should.be_true(string.contains(reason, "property enforcement"))
  should.be_true(string.contains(reason, "connected"))

  // 4. Infrastructure domains compiled before the failure
  should.be_true(domain.is_running("property"))
  should.be_true(domain.is_running("topology"))

  // 5. Cleanup
  let _ = coincidence.stop_server()
}

/// Compilation fails when a required property is unknown.
pub fn enforcement_unknown_requires_fails_test() {
  let source =
    "grammar @bad_req {
  type = a | b

  requires nonexistent_property
}
"
  let _ = coincidence.start_server()
  let result = boot.boot_with_infrastructure([], [source])
  should.be_error(result)
  let _ = coincidence.stop_server()
}

/// Compilation succeeds when all required properties pass.
pub fn enforcement_valid_requires_passes_test() {
  let source =
    "grammar @good_req {
  type = a | b | c

  requires shannon_equivalence
}
"
  let assert Ok(result) =
    boot.boot_with_infrastructure([], [source])
  boot.shutdown(result)
  let _ = coincidence.stop_server()
}

/// Compilation fails when an invariant property is unknown.
pub fn enforcement_unknown_invariant_fails_test() {
  let source =
    "grammar @bad_inv {
  type = a | b

  invariant nonexistent_invariant
}
"
  let _ = coincidence.start_server()
  let result = boot.boot_with_infrastructure([], [source])
  should.be_error(result)
  let _ = coincidence.stop_server()
}
