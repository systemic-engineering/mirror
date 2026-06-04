// Minimal cargo target for the kintsugi spec walker test.
// `cargo check` succeeds → @io/cargo lifts to transparency::success
// per shards/io/cargo.mirror's cargo_exit_to_transparency contract.

pub fn settled() -> &'static str {
    "settled"
}
