//! `fractal` — the compiler's Mandelbrot set at rust/ altitude.
//!
//! Per Alex 2026-07-18 direct-transcript:
//!
//! > "fractal IS what does all of this. fractal is the mandelbrot set
//! > of the compiler. That takes the multidimensional inference and
//! > projects it into whatever. fractal.rs is the @io discharge.
//! > phone.rs connects fractal fibres."
//!
//! Migrated from `/Users/alexwolf/dev/projects/fragmentation/` per
//! Mara `2760c2a` canonical migration spec
//! (`docs/specs/2026-07-18-fragmentation-to-rust-fractal-migration.md`).
//!
//! ## Migration state (2026-07-18)
//!
//! - **Step 1 LANDED**: crate scaffold (`Cargo.toml`, `src/lib.rs`)
//! - **Step 2 LANDED**: `witnessed.rs` verbatim from fragmentation
//! - Steps 3-12 follow in sequential ticks per Mara §6 recipe:
//!   - Step 3: `keys.rs` slimmed (drop Encode/Decode/Fractal deps)
//!   - Step 4: `subject.rs` (Subject envelope + LiquidVoid impl)
//!   - Step 5: `git.rs` (read_witnessed + commit_signature + detect_keys)
//!   - Step 6-7: RED-first prop_tests
//!   - Step 8-10: mirror/rust/Cargo.toml wire + phone.rs refactor + subject.rs wrapper
//!   - Step 11-12: cargo build + test + sequential commits
//!
//! ## MARA doctrine (load-bearing)
//!
//! *"Different witness, different hash. My observation of this code is
//! part of what this documentation is."* — `MARA.md:13`.
//!
//! Encoded via Author ≠ Committer split in `witnessed.rs`. Every
//! commit's SHA byte-includes BOTH Author and Committer identities;
//! same Author + different Committer = different commit. This is the
//! crypto-floor form of SEL's identity-provenance discipline. Preserved
//! per Alex 2026-07-18 Q2 ratification.

pub mod crystal;
pub mod mandelbrot;
pub mod singularity;
pub mod subject;
pub mod witnessed;

pub use crystal::{crystallize, Crystal};
pub use mandelbrot::{Mandelbrot, MandelbrotProvenance, Oid};
pub use singularity::{OpticKind, Singularity, SingularityError, SingularityState};
pub use subject::{Subject, SubjectKind};
pub use witnessed::{Author, Committer, Message, Timestamp, Witnessed};
