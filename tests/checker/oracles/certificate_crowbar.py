"""Seam 2026-10-29: certificates that pass the checks of docs/specs/mirror-certificate.md §4
while the result is wrong. Ground programs; a rule is (id, head, [pos], [neg]).

The checker below implements §4's list literally:
  1 each derivation is an instance of its rule; 2 (S) each non-fact atom has a derivation with
  strictly earlier premises; 3 (C) one naive round adds nothing; 4 layers = longest-path depth;
  6 W: gamma(under) = over and gamma(over) = under; 7 Dark 255: the violating atoms are derivable.
It does NOT do what the spec requires elsewhere but omits from §4 (all first-tick derivations,
functionality on every result, leastness of W's contexts). Each case shows the gap.

Run: python3 certificate_crowbar.py
"""


def tp(rules, M, ctx=None):
    ctx = M if ctx is None else ctx
    return {h for _, h, pos, neg in rules if all(p in M for p in pos) and not any(n in ctx for n in neg)}


def lfp(rules, facts, ctx=None):
    M = set(facts)
    while True:
        N = M | tp(rules, M, ctx)
        if N == M:
            return M
        M = N


def gamma(rules, facts, S):
    return lfp(rules, facts, ctx=S)


def wf(rules, facts):
    under = set(facts)
    while True:
        over = gamma(rules, facts, under)
        nxt = gamma(rules, facts, over)
        if nxt == under:
            return under, over
        under = nxt


def check(rules, facts, cert, functional=()):
    """cert = {model, layers{atom: t}, derivations[(head, rule_id, premises)], ctx: (under, over) | None,
    dark255: [atoms] | None}. Returns the list of failed check numbers (empty = passes)."""
    R = {rid: (h, pos, neg) for rid, h, pos, neg in rules}
    M, L, D = cert["model"], cert["layers"], cert["derivations"]
    ctx = cert.get("ctx")
    fails = []
    for head, rid, prem in D:                                           # 1
        h, pos, neg = R[rid]
        if h != head or sorted(prem) != sorted(pos):
            fails.append(1)
        if ctx and any(n in ctx[0] for n in neg):
            fails.append(1)
    for a in M - set(facts):                                            # 2 (S)
        if not any(h == a and all(L[p] < L[a] for p in prem) for h, _, prem in D):
            fails.append(2)
    if not tp(rules, M, ctx[0] if ctx else None) <= M:                  # 3 (C)
        fails.append(3)
    for a in M:                                                         # 4 layers = depth
        ders = [prem for h, _, prem in D if h == a]
        want = 0 if a in facts else (1 + max(max((L[p] for p in prem), default=0) for prem in ders) if ders else None)
        if L.get(a) != want:
            fails.append(4)
    if ctx:                                                             # 6 period-2
        u, o = ctx
        if gamma(rules, facts, u) != o or gamma(rules, facts, o) != u:
            fails.append(6)
    if cert.get("dark255"):                                             # 7
        if not set(cert["dark255"]) <= M:
            fails.append(7)
    return sorted(set(fails))


def depth_layers(rules, facts, M, D):
    L = {f: 0 for f in facts}
    changed = True
    while changed:
        changed = False
        for a in M:
            if a in L:
                continue
            ders = [prem for h, _, prem in D if h == a]
            if ders and all(all(p in L for p in prem) for prem in ders):
                L[a] = 1 + max(max((L[p] for p in prem), default=0) for prem in ders)
                changed = True
    return L


# ---- 1. W: a stable model passes the period-2 witness; the WF model has p, q undefined ----
def case_period2():
    rules = [("p", "p", [], ["q"]), ("q", "q", [], ["p"]), ("r", "r", [], ["s"]), ("t", "t", ["p"], [])]
    facts = set()
    u, o = wf(rules, facts)
    bad = {"q", "r"}                 # a stable model: q true, p false, nothing undefined
    D = [("q", "q", []), ("r", "r", [])]
    cert = {"model": bad, "layers": {"q": 1, "r": 1}, "derivations": D, "ctx": (bad, bad)}
    print("1. Vector 04 (p :- not q; q :- not p; r :- not s; t :- p):")
    print(f"   true WF model: under={sorted(u)} over={sorted(o)} -> undefined {sorted(o - u)}")
    print(f"   certificate 'q, r true; nothing undefined' with contexts ({sorted(bad)}, {sorted(bad)}):"
          f" failed checks {check(rules, facts, cert)}")
    print("   => the period-2 witness holds for every fixpoint of gamma^2; it does not certify the LEAST one.")


# ---- 2. CT3: an atom with two first-tick derivations, certificate carries one ----
def case_one_derivation():
    rules = [("ab", "b", ["a"], []), ("ac", "c", ["a"], []), ("bd", "d", ["b"], []), ("cd", "d", ["c"], [])]
    facts = {"a"}
    M = lfp(rules, facts)
    D = [("b", "ab", ["a"]), ("c", "ac", ["a"]), ("d", "bd", ["b"])]          # cd omitted
    cert = {"model": M, "layers": depth_layers(rules, facts, M, D), "derivations": D}
    print("2. Diamond a->b, a->c, b->d, c->d; certificate omits d's derivation via c:")
    print(f"   failed checks {check(rules, facts, cert)}  (CT3 expects a failure; §4 has no check for it)")


# ---- 3. A clear result that should be Dark 255: functionality is checked only on 255 ----
def case_hidden_violation():
    rules = [("o1", "owner(doc,alex)", ["claim(doc,alex)"], []), ("o2", "owner(doc,reed)", ["claim(doc,reed)"], [])]
    facts = {"claim(doc,alex)", "claim(doc,reed)"}
    M = lfp(rules, facts)
    D = [("owner(doc,alex)", "o1", ["claim(doc,alex)"]), ("owner(doc,reed)", "o2", ["claim(doc,reed)"])]
    cert = {"model": M, "layers": depth_layers(rules, facts, M, D), "derivations": D, "dark255": None}
    print("3. Vector-05 shape (functional owner(1) -> 2), certificate says verdict clear:")
    print(f"   failed checks {check(rules, facts, cert)}  => a wrong 'clear' passes; check 7 runs only when 255 is claimed")


# ---- 4. Facts read from the certificate instead of the source ----
def case_trusted_facts():
    rules = [("pb", "path(a,z)", ["edge(a,z)"], [])]
    src_facts = {"edge(a,b)"}
    cert_facts = {"edge(a,b)", "edge(a,z)"}
    M = lfp(rules, cert_facts)
    D = [("path(a,z)", "pb", ["edge(a,z)"])]
    cert = {"model": M, "layers": depth_layers(rules, cert_facts, M, D), "derivations": D}
    print("4. Certificate's own fact list includes edge(a,z), the source has only edge(a,b):")
    print(f"   checked against the certificate's facts: failed {check(rules, cert_facts, cert)};"
          f" against the source's facts: failed {check(rules, src_facts, cert)}")


# ---- 5. Order-freedom is a theorem about sets: replay can't fail on an acyclic certificate ----
def case_order():
    import itertools
    D = [("b", "ab", ["a"]), ("c", "ac", ["a"]), ("d", "bd", ["b"]), ("d", "cd", ["c"])]
    facts = {"a"}
    outs = set()
    n = 0
    for perm in itertools.permutations(D):
        M, ok = set(facts), True
        for h, _, prem in perm:
            if not all(p in M for p in prem):
                ok = False
                break
            M.add(h)
        if ok:
            n += 1
            outs.add(frozenset(M))
    print(f"5. Replay of the diamond in all {n} linear extensions: {len(outs)} distinct final set(s).")
    print("   The result is the union of heads and facts in every extension; one acyclicity check covers all.")


if __name__ == "__main__":
    case_period2(); print()
    case_one_derivation(); print()
    case_hidden_violation(); print()
    case_trusted_facts(); print()
    case_order()
