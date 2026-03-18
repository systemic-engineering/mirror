# conversation — gradients over trees

GLEAM := "/nix/store/c9rpml4l4nss0dqyq4grkrha0w7yh9f4-gleam-1.14.0/bin/gleam"
ERLANG_BIN := "/nix/store/knwmghwskvlyf3bc5rhgx1yj8d5sbyiw-erlang-27.3.4.8/lib/erlang/bin"

check: lint test format-check coverage

lint:
    nix develop -c cargo clippy -- -D warnings

test:
    nix develop -c cargo test

test-git:
    nix develop -c cargo test --features git

format-check:
    nix develop -c cargo fmt -- --check

format:
    nix develop -c cargo fmt

# 100% line coverage or fail (scoped to conversation sources)
coverage:
    nix develop -c cargo llvm-cov --fail-under-lines 100 --ignore-filename-regex 'story/'

# HTML report
coverage-html:
    nix develop -c cargo llvm-cov --html --open

pre-commit: check
pre-push: check

# Build the Rustler NIF and place it in beam/priv/ for gleam test to pick up.
build-nif:
    nix develop -c cargo build -p conversation_nif --release
    mkdir -p beam/priv
    cp target/release/libconversation_nif.dylib beam/priv/conversation_nif.so 2>/dev/null || \
    cp target/release/libconversation_nif.so    beam/priv/conversation_nif.so

# Build the Fortran prism NIF.
build-prism-nif:
    nix develop -c make -C beam/native

# Build all NIFs then run gleam tests.
beam-test: build-nif build-prism-nif
    PATH={{ERLANG_BIN}}:$PATH {{GLEAM}} test --directory beam
