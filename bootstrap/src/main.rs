//! mirror — the native binary entry point.
//!
//! The dispatch surface lives in `bootstrap/src/lib.rs`. This file
//! is a thin wrapper: collect argv, invoke `mirror::dispatch`, and
//! exit with the returned code. Per Taut #286 Win 2: extracting the
//! library entry point `mirror::kintsugi_main` lets integration tests
//! call the same dispatch in-process (skipping the 200-800ms dyld +
//! Accelerate startup tax) while the binary path still streams
//! stdout / stderr natively without capture overhead.

fn main() {
    let args: Vec<String> = std::env::args().collect();
    std::process::exit(mirror::dispatch(&args));
}
