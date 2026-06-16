#!/usr/bin/env bash
# Run the kintsugi loop and write outputs for the composite action.
#
# Produces:
#   /tmp/kintsugi-verdict.mirror  (substrate-native; uploaded as artifact)
#   /tmp/kintsugi-verdict.json    (@io crossing for jq + $GITHUB_OUTPUT)
#
# Exits non-zero per --fail-on. The mirror binary itself is on $PATH from
# install-mirror.sh.
#
# Per kintsugi-ci-v0.1 §1.4 + §5.2.

set -euo pipefail

TARGET=""
THRESHOLD="0.8"
SHATTER="4"
FAIL_ON="failure"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)    TARGET="$2";    shift 2 ;;
    --threshold) THRESHOLD="$2"; shift 2 ;;
    --shatter)   SHATTER="$2";   shift 2 ;;
    --fail-on)   FAIL_ON="$2";   shift 2 ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

if [[ -z "$TARGET" ]]; then
  echo "--target is required" >&2
  exit 2
fi

MIRROR_VERDICT="/tmp/kintsugi-verdict.mirror"
JSON_VERDICT="/tmp/kintsugi-verdict.json"

# Self-host mode: caller built mirror inside a nix devShell. The nix
# cc-wrapper disables auto-rpath, so the binary needs gfortran/lapack/blas
# layered onto LD_LIBRARY_PATH at runtime. Wrap each invocation in
# `nix develop -c` so the FFI deps resolve from /nix/store.
run_mirror() {
  if [[ "${NIX_SHELL:-false}" == "true" ]]; then
    nix develop -c bash -c "
      export LD_LIBRARY_PATH=\"\$(dirname \$(gfortran -print-file-name=libgfortran.so.5)):\$LAPACK_DIR/lib:\$BLAS_DIR/lib:\$LD_LIBRARY_PATH\"
      mirror $*
    "
  else
    mirror "$@"
  fi
}

echo "::group::kintsugi (substrate-native)"
run_mirror kintsugi --ci --shatter "$SHATTER" "$TARGET" | tee "$MIRROR_VERDICT"
echo "::endgroup::"

echo "::group::kintsugi (@io crossing: --out=@data/json)"
run_mirror kintsugi --ci --out=@data/json --shatter "$SHATTER" "$TARGET" > "$JSON_VERDICT"
cat "$JSON_VERDICT"
echo "::endgroup::"

echo "::group::Writing \$GITHUB_OUTPUT"
verdict="$(jq -r '.verdict' "$JSON_VERDICT")"
confidence="$(jq -r '.confidence // 0' "$JSON_VERDICT")"
objective="$(jq -r '.objective // 0' "$JSON_VERDICT")"
opacities="$(jq -c '.opacities // []' "$JSON_VERDICT")"

{
  echo "verdict=$verdict"
  echo "confidence=$confidence"
  echo "objective=$objective"
  echo "opacities=$opacities"
} >> "$GITHUB_OUTPUT"
echo "::endgroup::"

echo "verdict: $verdict (objective=$objective, confidence=$confidence)"

# Gate per --fail-on.
case "$FAIL_ON" in
  failure)
    [[ "$verdict" != "failure" ]] || exit 1
    ;;
  partial)
    case "$verdict" in
      failure|partial) exit 1 ;;
    esac
    ;;
  none)
    : # never fail; produce verdict only
    ;;
  *)
    echo "invalid --fail-on: $FAIL_ON (expected: failure | partial | none)" >&2
    exit 2
    ;;
esac
