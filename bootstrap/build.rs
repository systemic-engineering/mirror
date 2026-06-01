//! Build integration for the flang-FFI numerical-substrate pathway.
//!
//! Proof tick (docs/specs/numerical-substrate-via-fortran.md): compile the
//! Fortran source at `fortran/dot.f90` with flang, archive the object into a
//! static library, and emit the cargo link directives so Rust FFI can call it.
//!
//! ## Real constraints surfaced by this tick
//!
//! 1. flang is NOT in the warm devShell (only gfortran is). It IS in nixpkgs
//!    as `llvmPackages.flang` (21.1.8). The repo `flake.nix` has no devShell,
//!    so there is nothing to add flang to; we resolve flang at build time
//!    instead (FLANG env var → PATH → `nix build nixpkgs#llvmPackages.flang`).
//!
//! 2. nixpkgs `llvmPackages.flang` on aarch64-darwin ships the compiler but
//!    NOT `libflang_rt.runtime.a`. The full flang driver link fails with
//!    `ld: library not found for -lflang_rt.runtime`, and any runtime-backed
//!    intrinsic (e.g. SUM → __FortranASumReal8) is unresolved. This is spec
//!    open-decision #5 (runtime dependency), now answered with evidence: the
//!    runtime must be supplied separately before real numerics land.
//!
//! 3. We sidestep (2) for the proof: `dot.f90` uses an explicit DO loop, so
//!    its object has ZERO Fortran-runtime dependencies. We compile to an
//!    object with `flang -c`, archive it with `ar`, and link it directly —
//!    no flang runtime, no flang driver at link time. This isolates the FFI
//!    pathway (the thing this tick proves) from the runtime question (a
//!    documented follow-up blocker).
//!
//! Provisional choices (PROVISIONAL — to be confirmed when the spec's open
//! decisions are resolved):
//!   - #1 substrate path: `bootstrap/fortran/dot.f90`.
//!   - #2 FFI shape: static-link (not the spec's LLVM-IR target) for the proof.
//!   - #5 runtime lib: none needed yet (runtime-free object); the real runtime
//!     dependency is flagged above for the eigendecomposition tick.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let src = manifest_dir.join("fortran/dot.f90");

    // Rebuild if the Fortran source changes.
    println!("cargo:rerun-if-changed={}", src.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=FLANG");

    let flang = locate_flang();
    let obj = out_dir.join("dot.o");

    // Compile: flang -c fortran/dot.f90 -o $OUT_DIR/dot.o
    let status = Command::new(&flang)
        .arg("-c")
        .arg(&src)
        .arg("-o")
        .arg(&obj)
        .status()
        .unwrap_or_else(|e| panic!("failed to invoke flang ({}): {e}", flang.display()));
    assert!(
        status.success(),
        "flang failed to compile {}",
        src.display()
    );

    // Archive: ar rcs $OUT_DIR/libdot.a $OUT_DIR/dot.o
    let lib = out_dir.join("libdot.a");
    let _ = std::fs::remove_file(&lib);
    let ar = std::env::var("AR").unwrap_or_else(|_| "ar".to_string());
    let status = Command::new(&ar)
        .arg("rcs")
        .arg(&lib)
        .arg(&obj)
        .status()
        .unwrap_or_else(|e| panic!("failed to invoke ar ({ar}): {e}"));
    assert!(status.success(), "ar failed to archive {}", obj.display());

    // Link directives. The object is runtime-free (DO loop, no intrinsics),
    // so a bare static lib with no Fortran runtime suffices for this proof.
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=dot");
}

/// Resolve a flang binary: FLANG env var, then `flang`/`flang-new` on PATH,
/// then realize `nixpkgs#llvmPackages.flang` into the store as a last resort.
fn locate_flang() -> PathBuf {
    if let Ok(p) = std::env::var("FLANG") {
        let p = PathBuf::from(p);
        if p.exists() {
            return p;
        }
    }
    for name in ["flang", "flang-new", "flang-21"] {
        if let Ok(out) = Command::new("which").arg(name).output() {
            if out.status.success() {
                let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !path.is_empty() && Path::new(&path).exists() {
                    return PathBuf::from(path);
                }
            }
        }
    }
    // Last resort: realize flang from nixpkgs. This is what makes the proof
    // reproducible without a devShell change. Note for Alex: a flake.nix
    // devShell entry (llvmPackages.flang) would make this unnecessary, but
    // would require `direnv reload` to take effect.
    let out = Command::new("nix")
        .args([
            "build",
            "--no-link",
            "--print-out-paths",
            "nixpkgs#llvmPackages.flang",
        ])
        .output()
        .expect("flang not found on PATH and `nix build nixpkgs#llvmPackages.flang` failed to run");
    assert!(
        out.status.success(),
        "could not realize flang from nixpkgs: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let store_path = String::from_utf8_lossy(&out.stdout)
        .lines()
        .last()
        .unwrap_or("")
        .trim()
        .to_string();
    let flang = PathBuf::from(store_path).join("bin/flang");
    assert!(
        flang.exists(),
        "realized flang store path but {} does not exist",
        flang.display()
    );
    flang
}
