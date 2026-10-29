"""K0 golden-test oracle (Mara 2026-10-29), independent of the compiler.

Reproduces the data findings of Seam's crowbar
(docs/audits/2026-10-29-seam-crowbar-compile-kintsugi.md §7.3, §7.1, §10)
from the corpus at a given tree:

  claim     @X claimed (out|grammar|prism @X) by more than one file
  name?     a shard with no `out @X` line (compile: hole `name?`)
  undeclared (S, X) pairs, exact rule, comment tokens  #  //  --  (rule C)

Usage: python3 golden_k0.py <tree-root> [shards|all]
  shards: shards/ only;  all: boot/ + shards/
Output is mirror-text records, sorted, one finding per record.
"""
import re, sys, pathlib, collections

PATH = r"@[A-Za-z_][\w-]*(?:/[\w-]+)*"
REF = re.compile(PATH)
CLAIM = re.compile(r"^\s*(?:out|grammar|prism)\s+(" + PATH + r")")
OUT = re.compile(r"^\s*out\s+(" + PATH + r")")
IMP = re.compile(r"^\s*in\s+(" + PATH + r")\s*(?:#.*)?$")
COMMENTS = ("#", "//", "--")  # rule C; must equal mu_mirror's comment tokens


def code_part(line):
    out, in_str, i = [], False, 0
    while i < len(line):
        ch = line[i]
        if in_str and ch == "\\" and i + 1 < len(line):
            out.append("  "); i += 2; continue
        if ch == '"':
            in_str = not in_str; out.append(" "); i += 1; continue
        if not in_str and any(line.startswith(t, i) for t in COMMENTS):
            break
        out.append(" " if in_str else ch); i += 1
    return "".join(out)


def scan(f):
    imports, outs, claims, refs = set(), set(), set(), set()
    for raw in f.read_text(errors="replace").splitlines():
        m = IMP.match(raw)
        if m:
            imports.add(m.group(1)); continue
        m = CLAIM.match(raw)
        if m:
            claims.add(m.group(1))
        m = OUT.match(raw)
        if m:
            outs.add(m.group(1))
        if raw.lstrip().startswith(("out ", "source ", "in ")):
            continue
        refs.update(REF.findall(code_part(raw)))
    return imports, outs, claims, refs


def main(root, scope):
    root = pathlib.Path(root)
    dirs = ["shards"] if scope == "shards" else ["boot", "shards"]
    files = sorted(p for d in dirs for p in (root / d).rglob("*.mirror"))
    claimants = collections.defaultdict(set)
    records = []
    for f in files:
        rel = str(f.relative_to(root))
        imports, outs, claims, refs = scan(f)
        for c in claims:
            claimants[c].add(rel)
        if not outs:
            records.append(("name?", rel, ""))
        for x in sorted(refs - imports - claims):
            records.append(("undeclared", rel, x))
    for name, fs in claimants.items():
        if len(fs) > 1:
            records.append(("claim", name, " ".join(sorted(fs))))
    records.sort()
    counts = collections.Counter(k for k, _, _ in records)
    print(f"scope      {scope}\nfiles      {len(files)}")
    for k in ("claim", "name?", "undeclared"):
        print(f"{k:<10} {counts[k]}")
    print(f"undeclared_shards {len({s for k, s, _ in records if k == 'undeclared'})}")
    for k, a, b in records:
        print(f"\n{k:<10} {a}" + (f"\n{'by' if k == 'claim' else 'ref':<10} {b}" if b else ""))


main(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else "shards")
