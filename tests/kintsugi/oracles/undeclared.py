"""undeclared(S, X) :- references(S, X), not imports(S, X), not declares(S, X).
Code lines only (comments and string literals stripped; in/out/source lines are
not references). Two coverage rules: exact, and prefix (in @Y covers @Y/...)."""
import re, sys, pathlib

REF = re.compile(r"@[A-Za-z_][\w-]*(?:/[\w-]+)*")
DECL = re.compile(r"^\s*(?:out|prism|grammar|glass|pact)\s+(@[A-Za-z_][\w-]*(?:/[\w-]+)*)")
IMP = re.compile(r"^\s*in\s+(@[A-Za-z_][\w-]*(?:/[\w-]+)*)\s*$")

def code_part(line):
    out, in_str = [], False
    for ch in line:
        if ch == '"':
            in_str = not in_str
            out.append(" ")
            continue
        if not in_str and ch == "#":
            break
        out.append(" " if in_str else ch)
    return "".join(out)

def scan(path):
    imports, declares, refs, prose = set(), set(), {}, set()
    for n, raw in enumerate(path.read_text(errors="replace").splitlines(), 1):
        m = IMP.match(raw)
        if m:
            imports.add(m.group(1)); continue
        m = DECL.match(raw)
        if m:
            declares.add(m.group(1))
        s = raw.lstrip()
        if s.startswith(("out ", "source ", "in ")):
            continue
        code = code_part(raw)
        for r in REF.findall(code):
            refs.setdefault(r, n)
        for r in REF.findall(raw[len(code):] if len(code) < len(raw) else ""):
            prose.add(r)
    return imports, declares, refs, prose

def covered(x, names, prefix):
    if x in names:
        return True
    return prefix and any(x.startswith(y + "/") for y in names)

def main(root):
    files = sorted(pathlib.Path(root).rglob("*.mirror"))
    res = {"exact": [], "prefix": []}
    prose_only = 0
    for f in files:
        imports, declares, refs, prose = scan(f)
        names = imports | declares
        for mode in ("exact", "prefix"):
            bad = sorted(x for x in refs if not covered(x, names, mode == "prefix"))
            if bad:
                res[mode].append((str(f), bad))
        if not [x for x in refs if not covered(x, names, True)] and \
           [x for x in prose if not covered(x, names, True)]:
            prose_only += 1
    print(f"files {len(files)}")
    for mode in ("prefix", "exact"):
        print(f"{mode}: {len(res[mode])} shards with undeclared code-line refs")
    print(f"prose-only uncovered (prefix rule): {prose_only}")
    for mode in ("prefix", "exact"):
        for f, bad in res[mode]:
            print(" ", mode[0].upper(), f.split("/shards/")[-1], " ".join(bad[:6]) + (" ..." if len(bad) > 6 else ""))

main(sys.argv[1])
