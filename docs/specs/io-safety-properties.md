# IO Safety Properties

*2026-05-19. Reed. Spec.*

Status: **Red**

Depends on: @io (socket layer), @epistemologic/property (verdict type, check runner),
@code/kernel (syscall surface)

---

## 1. What the Hang Bug Teaches Us

The bootstrap compiler (`~/.local/bin/mirror`, 51KB C binary) hangs when
compiling uncached files. The root cause:

```c
static char *git_store(const char *content) {
    FILE *f = popen("git hash-object -w --stdin", "w");
    fwrite(content, 1, strlen(content), f);
    pclose(f);
    // DEAD CODE: opens git hash-object --stdin for READING
    // but git hash-object reads from stdin, waiting for data
    // pclose blocks forever waiting for git to exit
    FILE *f2 = popen("git hash-object --stdin", "r");
    pclose(f2);  // <-- HANGS HERE
    ...
}
```

The pattern: `popen` opens a pipe to a process that reads from stdin. The C code
opens the pipe for reading (stdout). Git's stdin is inherited from the parent or
connected to nothing useful. Git waits for EOF on stdin. `pclose` waits for git
to exit. Deadlock.

This is not a rare edge case. This is the most common form of IO hang:
**a process that waits for input that will never arrive.**

The compiler should have caught this. Not at runtime -- at compile time. The
grammar for `@io.exec` should carry a property that says: "if you open a process
that reads stdin, you must either feed it or close its stdin." The property layer
should verify this before the binary exists.

---

## 2. The Class of Bug

The hang is one instance of a general class: **unbounded IO without termination
guarantee.** The class includes:

| Bug | Pattern | Symptom |
|-----|---------|---------|
| stdin deadlock | `exec(proc)` where proc reads stdin, caller reads stdout | hang |
| missing EOF | pipe opened for writing, never closed | hang |
| missing error path | `exec(proc)` returns nonzero, caller reads stdout expecting data | hang or garbage |
| unbounded read | `read(stream)` with no timeout, server never sends EOF | hang |
| cache miss block | cache lookup calls process, process has no data, caller waits | hang |

Every one of these is detectable from the grammar alone. The `@io` grammar
declares what IO operations exist. The `@epistemologic/property/io_safety`
grammar declares what properties those operations must satisfy. The compiler
verifies the properties hold before emitting code.

---

## 3. Compile-Time Properties

### 3.1 bounded_io: every @io.exec has a termination guarantee

```
bounded_io(ast) -> verdict
```

Walks the AST. For every `@io.exec` or `@io.read` node:
- Check that the call has an associated timeout, EOF handler, or non-blocking flag.
- If a process is opened for reading (`popen(..., "r")`), verify that the process
  does not read from stdin (or that stdin is explicitly closed/fed).
- If a process is opened for writing, verify that the write end is closed (EOF sent).

**What it catches:** The hang bug. `popen("git hash-object --stdin", "r")` opens
a stdin-reading process for stdout reading. No stdin feed. No timeout. Verdict: fail.

**Verdict on the hang bug:**
```
fail(diagnostic {
  error: {
    grammar: @epistemologic/property/io_safety,
    name: bounded_io,
    message: "exec opens stdin-reading process for stdout capture without feeding stdin",
    location: { file: "native/mirror.c", line: 1226, column: 15 },
    loss: { bits: 1.0, source: <node>, measurement: shannon },
  },
  severity: fatal,
  suggestion: "feed stdin and close, or use a temp file, or add timeout",
})
```

### 3.2 error_path: every @io.exec handles nonzero exit

```
error_path(ast) -> verdict
```

Walks the AST. For every `@io.exec` node:
- Check that the return value is pattern-matched or checked for error.
- The `imperfect` return type carries errors -- verify they are inspected.

**What it catches:** The second half of the hang bug. `git cat-file -p` on a
missing ref returns exit 128. If the caller ignores the exit code and reads
stdout expecting data, it gets empty or garbage. The `exec_capture` function
happens to handle this (fread returns 0 bytes), but only by accident. The
property ensures it's handled by design.

### 3.3 cache_nonblocking: cache miss returns immediately

```
cache_nonblocking(ast) -> verdict
```

Walks the AST. For every cache lookup pattern (identified by `@io.exec` calling
a content-addressed store):
- Check that cache miss (process exits nonzero or returns empty) is handled
  as an immediate return, not a blocking wait.
- Check that no secondary blocking call follows a cache miss.

**What it catches:** The specific pattern where cache miss triggers a store
operation that itself hangs. In the C binary, `git_crystal_exists` (cache check)
works fine on miss. But `git_store_crystal` (cache store on miss) calls `git_store`
which has the dead popen. The property checks the whole cache-miss path, not just
the lookup.

### 3.4 eof_handling: every @io.read has EOF handling

```
eof_handling(ast) -> verdict
```

Walks the AST. For every `@io.read` node:
- Check that EOF (stream closed, no more data) is handled.
- `fread` returning 0 is not sufficient -- the code must check the return value.
- `pclose` after fread must check the exit code.

**What it catches:** Silent data loss. If `fread` returns 0 and the code treats
the empty buffer as valid data, it produces garbage output. The property ensures
that EOF is an explicit code path, not an ignored edge case.

---

## 4. How the Properties Fit into @epistemologic/property

The property hierarchy:

```
@epistemologic/property              # verdict type, check runner
  /duplicate_variant                 # E001: type system
  /unresolved_import                 # E002: module system
  /io_safety                         # IO soundness (this spec)
    bounded_io                       #   every exec terminates
    error_path                       #   every exec handles failure
    cache_nonblocking                #   cache miss is non-blocking
    eof_handling                     #   every read handles EOF
```

`io_safety` is a compound property. It declares four checks. Each is a named
lambda returning verdict. The parent `@epistemologic/property` runner collects
all four verdicts when `--reflect` is passed.

### Grammar structure

```mirror
in @prism
in @epistemologic/property
in @io

grammar @epistemologic/property/io_safety {
  # the four properties are named checks.
  # each takes an AST and returns a verdict.
  # the \ hole means: the implementation is deferred to the interpreter.
  # when the interpreter lands, these become executable.
  # until then, they are declarations that --reflect reports.

  bounded_io(ast) -> verdict { \ }
  error_path(ast) -> verdict { \ }
  cache_nonblocking(ast) -> verdict { \ }
  eof_handling(ast) -> verdict { \ }
}

out bounded_io
out error_path
out cache_nonblocking
out eof_handling
```

---

## 5. How --reflect Shows These Verdicts

`mirror compile --reflect` runs the property layer after tokenization and reports
verdicts. For IO safety properties:

```
$ mirror compile --reflect boot/std/io.mirror

@epistemologic/property/io_safety:
  bounded_io:        pass
  error_path:        pass
  cache_nonblocking: pass
  eof_handling:      pass

verdict: pass
oid: 5b4178705fe449cc95b08e26cd2665c3ce3aea9562e82a5aa9a20d80cfef23b8
```

When a violation is found:

```
$ mirror compile --reflect native/mirror.c

@epistemologic/property/io_safety:
  bounded_io:        fail
    error[IO-001]: exec opens stdin-reading process without feeding stdin
      --> native/mirror.c:1226:15
       |
    1226 |     FILE *f2 = popen("git hash-object --stdin", "r");
         |                ^^^^^ process reads stdin; pipe opened for stdout only
       |
       = loss: 1.0 (shannon)
       = suggestion: feed stdin and close, use temp file, or add timeout

  error_path:        partial(0.8)
    warning[IO-002]: exec return value not checked for error
      --> native/mirror.c:1227:5
       |
    1227 |     pclose(f2);
         |     ^^^^^^ exit code ignored
       |
       = loss: 0.3 (shannon)

  cache_nonblocking: fail
    error[IO-003]: cache store path contains blocking exec
      --> native/mirror.c:1219:15
       |
    1219 |     FILE *f = popen("git hash-object -w --stdin", "w");
         |               ^^^^^ followed by blocking popen on line 1226
       |
       = loss: 1.0 (shannon)

  eof_handling:      pass

verdict: fail (2 errors, 1 warning)
total loss: 2.3 (shannon)
```

### What --reflect does NOT do

`--reflect` does not execute the `\ ` holes. The properties are declarations.
Until the interpreter resolves holes through Fate, `--reflect` reports the
property structure and any statically derivable verdicts. Full verdict computation
requires the interpreter loop.

Phase 1 (now): `--reflect` lists properties and their types. Reports `\ ` (unknown).
Phase 2 (interpreter): `--reflect` executes properties and reports verdicts.

---

## 6. The Proof

The hang bug exists because the C code has no compile-time check for IO soundness.
The properties in this spec would catch it at three levels:

1. `bounded_io` catches the dead popen (stdin never fed)
2. `error_path` catches the ignored exit code
3. `cache_nonblocking` catches the blocking call in the cache-store path

Any one of the three would prevent the hang. All three together make the class
of bug structurally impossible in compiled mirror code.

The grammar IS the firewall. `@io.exec` is the only door to reality. Properties
on `@io.exec` are the lock. The compiler checks the lock before opening the door.

```
e^(n+1) < e^(n)
```

The hang is e^n. The property layer is e^(n+1). The error gets smaller.
