import conversation/runtime
import conversation/protocol.{Arm, Case, Cmp, DesiredState, Gt, Pass, Wildcard}

pub fn main() {
  // Example: case dispatch with wildcard fallback
  let spec =
    Case("error.rate", [
      Arm(Cmp(Gt, "0.1"), DesiredState("health_monitor", "critical")),
      Arm(Wildcard, Pass),
    ])

  let deltas = runtime.converge(spec)
  deltas
}
