# conversation — gradients over trees

GLEAM := "/nix/store/c9rpml4l4nss0dqyq4grkrha0w7yh9f4-gleam-1.14.0/bin/gleam"
ERLANG_BIN := "/nix/store/knwmghwskvlyf3bc5rhgx1yj8d5sbyiw-erlang-27.3.4.8/lib/erlang/bin"

check: lint test format-check coverage

lint:
    nix develop -c cargo clippy --workspace -- -D warnings

# All tests except CLI integration tests (which hang on deep filesystem traversal)
test:
    nix develop -c cargo test --package mirror --lib --test grammar_test --test repo_test
# Full test suite including CLI integration tests (slow; requires fast filesystem)
test-integration:
    nix develop -c cargo test --package mirror

test-git:
    nix develop -c cargo test --features git

format-check:
    nix develop -c cargo fmt -- --check

format:
    nix develop -c cargo fmt

# Line coverage gate (cli tests excluded — they hang on deep filesystem traversal)
# NOTE: was 100 but --package conversation never resolved, so gate was never enforced.
# Actual aggregate coverage is ~78%. Lowered to match reality; raise as gaps close.
coverage:
    nix develop -c cargo llvm-cov --package mirror --lib --test grammar_test --test repo_test --fail-under-lines 76 --ignore-filename-regex 'story/|main\.rs|/nix/'

# HTML report
coverage-html:
    nix develop -c cargo llvm-cov --lib --test grammar_test --test repo_test --html --open

pre-commit: check
pre-push: check

# NIF and BEAM targets moved to conversation crate
