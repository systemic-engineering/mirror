#!/usr/bin/env bash
# make-reformatted.sh — generate tests/fixpoint/mirror.reformatted.mirror.
#
# Spec: docs/specs/mirror-fixpoint-parser.md §13 T1 item 3. The fixture is
# mirror.mirror with its layout changed and its declarations reordered. Both
# are free moves (spec §2 items 5–6, §12 decision 3: rule names are meaning,
# declaration order is not), so `mirror compile` must print the same `root`
# for both files.
#
# Usage:
#   tests/fixpoint/make-reformatted.sh            # writes the checked-in fixture
#   tests/fixpoint/make-reformatted.sh IN OUT     # OUT may be `-` for stdout
#
# rust/tests/fixpoint_t1.rs runs this script with OUT=`-` and compares the
# result with the checked-in fixture, so a stale fixture fails T1.3 by name.
#
# What the transformation does (deterministic, pure text, no parser):
#   1. Finds the outermost `grammar … {` block and its closing brace.
#   2. Splits the block's body into units at blank lines. A unit never ends
#      inside an open bracket or string: blank lines inside a multi-line
#      declaration are kept inside its unit.
#   3. Reverses the order of the units (the declaration reorder).
#   4. Doubles every line's leading indentation, strips trailing whitespace
#      and separates units by two blank lines (the layout change). Lines that
#      start inside a string literal keep their bytes.
#   Lines before the grammar block and after its closing brace stay in place.
#
# Bracket and string tracking: `"` opens and closes strings, `\` escapes the
# next character inside a string, an unquoted `#` starts a comment to the end
# of the line. This matches the comment and string forms in
# boot/std/mirror/grammar.mirror. It has not been checked against mirror.mirror,
# which does not exist yet (it lands with the seed/engine work, spec §14).
# When it lands: run this script, read the diff, and confirm every unit is a
# whole declaration (or a comment block) before checking in the fixture.
#
# Exit codes: 0 written; 2 input missing; 3 no grammar block or unbalanced;
# 4 fewer than two units to reorder, or the output equals the input.

set -euo pipefail
export LC_ALL=C

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
in="${1:-$root/mirror.mirror}"
out="${2:-$here/mirror.reformatted.mirror}"

if [ ! -f "$in" ]; then
    echo "make-reformatted.sh: input missing: $in" >&2
    echo "  mirror.mirror is the grammar (spec §2); it lands with the seed/engine" >&2
    echo "  work (spec §14). This script does not author it." >&2
    exit 2
fi

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

awk '
function scan(line,    i, n, c, esc) {
    # Updates depth and in_str across the line. Comments end the scan.
    n = length(line)
    esc = 0
    for (i = 1; i <= n; i++) {
        c = substr(line, i, 1)
        if (in_str) {
            if (esc) { esc = 0 }
            else if (c == "\\") { esc = 1 }
            else if (c == "\"") { in_str = 0 }
        } else if (c == "\"") { in_str = 1 }
        else if (c == "#") { return }
        else if (c == "{" || c == "(" || c == "[") { depth++ }
        else if (c == "}" || c == ")" || c == "]") { depth-- }
    }
}
function rstrip(s) { sub(/[ \t]+$/, "", s); return s }
function reindent(s,    lead) {
    match(s, /^[ \t]*/)
    lead = substr(s, 1, RLENGTH)
    return lead lead substr(s, RLENGTH + 1)
}
{
    raw[NR] = $0
    starts_in_str[NR] = in_str
    before = depth
    scan($0)
    depth_after[NR] = depth
    if (gopen == 0 && before == 0 && depth > 0 && $0 ~ /^[ \t]*grammar[ \t]/) gopen = NR
    if (gopen > 0 && gclose == 0 && NR > gopen && depth == 0) gclose = NR
}
END {
    if (gopen == 0 || gclose == 0) {
        print "make-reformatted.sh: no balanced top-level `grammar … {` block found" > "/dev/stderr"
        exit 3
    }
    # Units inside the body (gopen+1 .. gclose-1), split at blank lines that
    # sit at body depth (1) and outside strings.
    nu = 0; cur = ""
    for (i = gopen + 1; i < gclose; i++) {
        blank = (raw[i] ~ /^[ \t]*$/) && !starts_in_str[i]
        at_body = (depth_after[i - 1] == 1)
        if (blank && at_body) {
            if (cur != "") { unit[++nu] = cur; cur = "" }
            continue
        }
        line = starts_in_str[i] ? raw[i] : reindent(rstrip(raw[i]))
        cur = (cur == "") ? line : cur "\n" line
    }
    if (cur != "") unit[++nu] = cur
    if (nu < 2) {
        print "make-reformatted.sh: fewer than two units in the grammar body; nothing to reorder" > "/dev/stderr"
        exit 4
    }
    for (i = 1; i < gopen; i++) print rstrip(raw[i])
    print rstrip(raw[gopen])
    for (u = nu; u >= 1; u--) {
        print ""
        print unit[u]
        if (u > 1) print ""
    }
    print ""
    for (i = gclose; i <= NR; i++) print rstrip(raw[i])
}
' "$in" > "$tmp"

if cmp -s "$in" "$tmp"; then
    echo "make-reformatted.sh: output equals input; the fixture would be vacuous" >&2
    exit 4
fi

if [ "$out" = "-" ]; then
    cat "$tmp"
else
    cp "$tmp" "$out"
fi
