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

echo "::group::kintsugi (substrate-native)"
mirror kintsugi --ci --shatter "$SHATTER" "$TARGET" | tee "$MIRROR_VERDICT"
echo "::endgroup::"

echo "::group::kintsugi (@io crossing: --format=json)"
mirror kintsugi --ci --format=json --shatter "$SHATTER" "$TARGET" > "$JSON_VERDICT"
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
