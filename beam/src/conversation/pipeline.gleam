//// Pipeline — wire a conversation spec as a GenStage topology.
////
//// Runs a spec through a single producer→consumer pipeline and returns
//// the list of deltas needed to converge BEAM state toward the spec.

import conversation/protocol.{type Spec}
import conversation/runtime.{type Delta}
import gleam/erlang/process
import gleam/list
import stage
import stage/consumer
import stage/producer

/// Run a conversation spec through a producer→consumer pipeline.
/// Returns the list of deltas needed to converge to desired state.
pub fn run(spec: Spec) -> List(Delta) {
  // Producer: emits the spec once on first demand, then goes quiet.
  let prod_config =
    producer.new_config(
      init_state: #(spec, False),
      on_demand: fn(demand, state) {
        let #(s, emitted) = state
        case demand > 0 && !emitted {
          True -> #([s], #(s, True))
          False -> #([], #(s, emitted))
        }
      },
    )
  let assert Ok(prod) = stage.start_producer(prod_config)

  // Consumer: converge each spec event, accumulate deltas.
  let cons_config =
    consumer.new_config(
      init_state: [],
      on_events: fn(events: List(Spec), acc: List(Delta)) {
        list.append(acc, list.flat_map(events, runtime.converge))
      },
    )
  let assert Ok(cons) = stage.start_consumer(cons_config)

  let assert Ok(Nil) =
    stage.subscribe(
      cons,
      to: prod,
      with: stage.auto_subscribe_opts(max_demand: 1, min_demand: 0),
    )

  // Allow async event delivery to complete before reading state.
  process.sleep(100)
  stage.consumer_state(cons, 500)
}
