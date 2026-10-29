"""Seam 2026-10-29: the `undeclared` count as a function of the lexical rule.

Mara's undeclared.py fixes one lexer (`#` comments, per-line `"` toggling, `in @X`
only when nothing follows on the line, declares = explicit out/prism/grammar/glass/pact).
The spec (fixpoint §1.3) says "code line (not a comment, not inside a string literal)"
and "declares reads N, so the out that reversed R3 inserts counts". This script
varies each lexical choice independently and prints the exact/prefix counts, so the
fixture can pin the rule instead of a number.

Toggles:
  M   Mara's oracle as committed (baseline; must reproduce 20 / 35)
  I   `in @X  # trailing comment` is an import (2 corpus lines)
  C   `//` and `--` line comments are comments (2 + 32 files use them)
  E   `\"` inside a string does not close it (11 corpus lines)
  R   reversed R3: a shard with no explicit `out` declares @<path under shards/>
  T   Taut-like: string literals are NOT stripped (references inside strings count)
Usage: python3 undeclared_variants.py <shards-root>
"""
import re, sys, pathlib, itertools

REF = re.compile(r"@[A-Za-z_][\w-]*(?:/[\w-]+)*")
DECL = re.compile(r"^\s*(?:out|prism|grammar|glass|pact)\s+(@[A-Za-z_][\w-]*(?:/[\w-]+)*)")
IMP_STRICT = re.compile(r"^\s*in\s+(@[A-Za-z_][\w-]*(?:/[\w-]+)*)\s*$")
IMP_LOOSE = re.compile(r"^\s*in\s+(@[A-Za-z_][\w-]*(?:/[\w-]+)*)\s*(?:#.*)?$")


def code_part(line, esc, strip_strings, extra_comments):
    s = line.lstrip()
    if extra_comments and (s.startswith("//") or s.startswith("--")):
        return ""
    out, in_str, prev = [], False, ""
    for ch in line:
        if ch == '"' and not (esc and in_str and prev == "\\"):
            in_str = not in_str
            out.append(" " if strip_strings else ch)
            prev = ch
            continue
        if not in_str and ch == "#":
            break
        out.append(" " if (in_str and strip_strings) else ch)
        prev = ch if prev != "\\" else ""
    return "".join(out)


def scan(path, root, opt):
    imp_re = IMP_LOOSE if "I" in opt else IMP_STRICT
    imports, declares, refs = set(), set(), {}
    has_out = False
    for n, raw in enumerate(path.read_text(errors="replace").splitlines(), 1):
        m = imp_re.match(raw)
        if m:
            imports.add(m.group(1)); continue
        m = DECL.match(raw)
        if m:
            declares.add(m.group(1))
        s = raw.lstrip()
        if s.startswith("out "):
            has_out = True
        if s.startswith(("out ", "source ", "in ")):
            continue
        code = code_part(raw, "E" in opt, "T" not in opt, "C" in opt)
        for r in REF.findall(code):
            refs.setdefault(r, n)
    if "R" in opt and not has_out:
        rel = path.relative_to(root).with_suffix("")
        declares.add("@" + "/".join(rel.parts))
    return imports, declares, refs


def covered(x, names, prefix):
    return x in names or (prefix and any(x.startswith(y + "/") for y in names))


def count(root, opt):
    files = sorted(root.rglob("*.mirror"))
    res = {"exact": set(), "prefix": set()}
    pairs = {"exact": set(), "prefix": set()}
    for f in files:
        imports, declares, refs = scan(f, root, opt)
        names = imports | declares
        for mode in ("exact", "prefix"):
            bad = [x for x in refs if not covered(x, names, mode == "prefix")]
            if bad:
                res[mode].add(str(f.relative_to(root)))
                pairs[mode] |= {(str(f.relative_to(root)), x) for x in bad}
    return res, pairs


def main(root):
    root = pathlib.Path(root)
    base, base_pairs = count(root, "M")
    print(f"{'rule':8} {'prefix':>7} {'exact':>6} {'(S,X) exact':>12}")
    for k in range(0, 6):
        for combo in itertools.combinations("ICERT", k):
            opt = "".join(combo) or "M"
            if k > 1 and opt not in ("ICER", "ICERT", "ICE"):
                continue
            r, p = count(root, opt)
            print(f"{opt:8} {len(r['prefix']):7} {len(r['exact']):6} {len(p['exact']):12}")
    r, _ = count(root, "ICER")
    print("\nshards that leave the exact list under ICER (lexer false positives in M):")
    for s in sorted(base["exact"] - r["exact"]):
        print("  -", s)
    print("shards that join the exact list under ICER:")
    for s in sorted(r["exact"] - base["exact"]):
        print("  +", s)


main(sys.argv[1])
