# Minimum Binary Surface

The mirror binary's external call set. Everything else is grammar.

## The Complete External Call Set

### LAPACK/BLAS

**None.**

The mirror binary does NOT link LAPACK or BLAS. This is the biggest surprise.

- `dirac.rs` implements its own **Jacobi eigenvalue solver** in pure Rust (lines 274-422)
- The `prism-core` crate has an optional `lapack` feature but mirror does NOT enable it
  (`features = ["bundle", "lambda"]` in Cargo.toml)
- The `coincidence` crate links LAPACK via Fortran FFI, but mirror only uses `coincidence`
  in test code (`tests/crypto_break.rs`), never in `src/`
- The `fate` crate has an optional `lapack` feature but mirror does not depend on fate at all

Alex's starting assumption (`dsyev`, `dgemv`, `dgemm`) was wrong. The binary is LAPACK-free.

### SHA / Hash

Two hash functions, both from the `sha2`/`sha1` crates (pure Rust, no FFI):

| Function | Source | Used For |
|----------|--------|----------|
| **SHA-256** | `sha2::Sha256` | Content addressing via CoincidenceHash<3> → eigenvalue → SHA-256 compression. Final OID is 64 hex chars. |
| **SHA-1** | `sha1::Sha1` | Git-compatible tree/blob OIDs in fragmentation (`blob {len}\0{data}` format). |

Both are pure Rust implementations. No external C library calls.

The hash pipeline for `Oid::hash(bytes)`:
1. Encode bytes into 16-dimensional StateVector via SHA-256 seeds
2. Apply 3 rank-1 projections (each derived from SHA-256 seeds)
3. Concatenate projection results into eigenvalue record
4. SHA-256 compress eigenvalue record → 64 hex chars

Alex's assumption about SHA-512 for key derivation: **not present in the binary**.
(The `coincidence` crate uses `hkdf` + `chacha20poly1305` for crypto, but mirror
doesn't exercise that path in production code.)

### Syscalls

The mirror binary touches the OS through exactly these libc/kernel interfaces:

| Syscall | Used By | Purpose |
|---------|---------|---------|
| `posix_spawn` / `fork+execve` | `std::process::Command` in `io_exec` | Spawn external processes (git) |
| `pipe` | `std::process::Stdio::piped()` | Connect stdin/stdout to child |
| `read` | Implicit in `wait_with_output()` | Read child stdout |
| `write` | `child.stdin.write_all()` | Write to child stdin |
| `waitpid` | `child.wait_with_output()` | Wait for child termination |
| `open` / `openat` | `std::fs::read_to_string()` | Read .mirror source files |
| `read` | `std::fs::read_to_string()` | Read file contents |
| `close` | Implicit in File drop | Close file descriptors |
| `fstat` / `stat` | `std::fs::metadata()` | Check if path is directory |
| `getdents64` / `readdir` | `std::fs::read_dir()` | List directory entries (craft) |
| `write` | `eprintln!` / `println!` | Write to stdout/stderr |
| `clock_gettime` | `std::time::Instant::now()` | Benchmarking timestamps |
| `brk` / `mmap` | Rust allocator (jemalloc/system) | Heap allocation |
| `getenv` | `std::env::args()` | Command line arguments |

**NOT needed:**
- `socket` — no network. Git runs as a subprocess.
- `mmap` for files — uses `read` via `read_to_string`
- `ioctl` — no terminal control
- `signal` — no signal handlers

### libc

The binary links libc for:

1. **Memory allocation** — `malloc`/`free`/`realloc` (Rust's global allocator)
2. **Process management** — `posix_spawn`/`fork`/`execve`/`waitpid`/`pipe`
3. **File I/O** — `open`/`read`/`write`/`close`/`stat`/`readdir`
4. **String operations** — `memcpy`/`memmove`/`memset`/`memcmp` (compiler intrinsics)
5. **Math** — `sqrt`/`exp`/`ln`/`cos`/`sin`/`atan` (used by Jacobi solver + softmax in tests)
6. **Process exit** — `exit`

## What Each External Call Does in Mirror

| External Call | Mirror Operation |
|---------------|-----------------|
| SHA-256 | Oid::hash() → content address. Every AST node, every grammar, every crystal. |
| SHA-1 | Git tree OID compatibility (fragmentation). `blob {len}\0{data}` format. |
| posix_spawn + pipe + read + write + waitpid | `io_exec()` — the ONE door to reality. Runs `git hash-object`, `git cat-file`, `git update-ref`. |
| open + read + close | Reading `.mirror` source files from disk |
| write (stdout) | Printing compilation results (OIDs) |
| write (stderr) | Diagnostics, cache status |
| stat | `is_dir()` check for bench dispatch |
| readdir | `craft` target scanning (find all .mirror files) |
| malloc/free | All heap allocation (Vec, String, HashMap, BTreeMap) |
| sqrt | Jacobi solver rotation angles, Connes distance, vector norms |
| exp | Softmax (in tests via coincidence heat kernel; not in binary hot path) |
| clock_gettime | Benchmark timing |

## The Call Count

**External function signatures the binary imports:**

| Category | Count | Functions |
|----------|-------|-----------|
| Hash (pure Rust, no FFI) | 0 | SHA-256 and SHA-1 are pure Rust — no external symbols |
| libc memory | 3 | `malloc`, `free`, `realloc` |
| libc process | 5 | `posix_spawn`, `pipe`, `waitpid`, `read`, `write` |
| libc file | 5 | `open`, `close`, `stat`, `fstat`, `getdents64` |
| libc string | 4 | `memcpy`, `memmove`, `memset`, `memcmp` |
| libc math | 4 | `sqrt`, `cos`, `sin`, `atan` |
| libc other | 3 | `exit`, `getenv`, `clock_gettime` |
| **Total** | **24** | (plus Rust runtime internals like `__rust_alloc`) |

But from the LLVM IR perspective, the true external surface is just **libc**.
No other shared library. No LAPACK. No OpenSSL. No libgit2 in the minimal path
(though `git2` is currently linked via `fragmentation-git` — see "What's NOT Needed").

## What's NOT Needed

| Thing | Why Not |
|-------|---------|
| **LAPACK** (`dsyev`, `dgemv`, `dgemm`) | Jacobi solver is pure Rust. Mirror never enables `lapack` feature. |
| **BLAS** | No matrix-matrix multiply calls. Sparse matvec is hand-rolled. |
| **dpotrf** (Cholesky) | Not used anywhere in mirror's dependency tree. |
| **dstev** (tridiagonal eigenvalue) | Not used. The Lanczos method mentioned in cosmos spec is not in mirror. |
| **dsyevd** (divide-and-conquer) | Not used. Jacobi is sufficient for grammar-sized matrices. |
| **SHA-512** | Not used by mirror. Coincidence crate uses hkdf but mirror doesn't exercise it. |
| **libgit2** | Currently linked through `fragmentation-git`, but mirror's `io_exec` already shells out to `git`. Redundant. Removable. |
| **OpenSSL/libssh2** | Currently pulled transitively by `git2`. Not needed if `git2` is removed. |
| **age** (encryption) | Listed in Cargo.toml but never imported in `src/`. Dead dependency. |
| **eetf** (Erlang term format) | Listed in Cargo.toml but never imported in `src/`. Dead dependency. |
| **base64** | Listed in Cargo.toml but never imported in `src/`. Dead dependency. |
| **unicode-segmentation** | Listed in Cargo.toml but never imported in `src/`. Dead dependency. |
| **rust-stemmers** | Listed in Cargo.toml but never imported in `src/`. Dead dependency. |
| **coincidence** (crate) | Listed in Cargo.toml, only used in `tests/`. Dead for the binary. |
| **socket/network** | No network calls. Git subprocess handles transport. |
| **mmap** | No memory-mapped files. Everything through `read_to_string`. |

## The True Minimum

If we strip dead dependencies and the `git2`/`fragmentation-git` redundancy:

```
mirror links:
  1. libc          — allocation, process, file I/O, math
  2. (nothing else)
```

The hash functions (SHA-256, SHA-1) are pure Rust. The eigenvalue solver is pure Rust.
The only FFI boundary is libc. Everything above is grammar.

### The Absolute Minimum for Self-Hosting

For a self-hosting compiler that can compile itself:

```
Syscalls (via libc):
  read(fd, buf, len)       — read source files + child stdout
  write(fd, buf, len)      — write to stdout/stderr + child stdin
  open(path, flags)        — open source files
  close(fd)                — cleanup
  execve(path, argv, envp) — spawn git subprocess
  pipe(fds)                — connect to child process
  waitpid(pid, status, 0)  — wait for child
  exit(code)               — terminate
  brk/mmap                 — heap (could be bump allocator)
  stat(path, buf)          — directory detection

Math (could be grammar):
  sqrt                     — Jacobi rotations, norms, Connes distance
  cos, sin, atan           — Jacobi rotation angles

Hash (pure Rust, compilable to grammar):
  SHA-256 block transform  — content addressing
  SHA-1 block transform    — git compatibility
```

**Total: 10 syscalls + 4 math functions + 2 hash block transforms = 16 primitives.**

If sqrt/cos/sin/atan are implemented in grammar (Taylor series), and SHA is
implemented in grammar (bitwise ops), then the absolute minimum is:

**10 syscalls. That's it. The rest is grammar.**

## The Binary Size Estimate

With dead dependencies removed and only libc linked:

| Component | Estimated LLVM IR |
|-----------|------------------|
| Tokenizer (tokenize.rs) | ~3K lines |
| AST (mirror_ast.rs) | ~2K lines |
| Kernel (kernel.rs) | ~1K lines |
| Interpreter (interpreter.rs) | ~500 lines |
| Dirac (dirac.rs) | ~1K lines |
| Prism (prism.rs) | ~500 lines |
| SHA-256 (inlined from sha2) | ~500 lines |
| SHA-1 (inlined from sha1) | ~400 lines |
| Coincidence hash (from prism-core) | ~800 lines |
| CLI + bench | ~500 lines |
| **Total LLVM IR** | **~10K lines** |
| **Estimated binary** | **~200KB stripped** |

The current binary is larger due to `git2`, `age`, `eetf`, and other dead weight.
The minimum binary — the one that compiles .mirror files and produces content-addressed
OIDs — needs none of that.

## Architecture Implication

The mirror binary is already closer to the minimum than expected:

```
mirror = libc + pure Rust
       = 10 syscalls + SHA-256 + SHA-1 + Jacobi eigenvalues
       = @io + @hash + @eigen

Three abstract operations. Everything else is grammar.
```

The thesis "LAPACK/BLAS + SHA + Syscalls" simplifies to just "SHA + Syscalls" because
the eigenvalue computation is already self-contained. And if SHA is implemented in
grammar (as it could be — it's just bitwise arithmetic), it simplifies further to
just syscalls.

The binary IS the proof that computation reduces to IO.
