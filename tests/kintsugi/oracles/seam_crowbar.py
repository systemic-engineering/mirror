"""Seam 2026-10-29: counterexamples against the compile/kintsugi math (2026-10-29).

Independent of Mara's cech_shards.py in the parts that matter: the comma among all is
recomputed here by its own union-find + Euler count (no import of her `build`), and the
linear-algebra checks use exact integer arithmetic over F2 (no numpy, no SVD).

Run: python3 seam_crowbar.py
"""
import itertools, random


# ---------- independent primitives ----------

def components(vertices, edges):
    parent = {v: v for v in vertices}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x
    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra != rb:
            parent[ra] = rb
    return {v: find(v) for v in vertices}


def b0(vertices, edges):
    return len(set(components(vertices, edges).values()))


def b1(vertices, edges):
    # multigraph: parallel edges count
    return len(edges) - len(vertices) + b0(vertices, edges)


def cover(owner, refs):
    """Mara Def 1.1/1.2 read as a multigraph: one edge per ordered reference (d, r)."""
    shards = sorted(set(owner.values()))
    K = {}
    for s in shards:
        Es = [(d, r) for (d, r) in refs if owner[d] == s]
        Vs = {d for d in owner if owner[d] == s} | {r for (_, r) in Es}
        K[s] = (sorted(Vs), Es)
    return sorted(owner), list(refs), K


def among(owner, refs):
    V, E, K = cover(owner, refs)
    return b1(V, E) - sum(b1(*K[s]) for s in K)


def star(owner, refs, i):
    _, _, K = cover(owner, refs)
    Vi = set(K[i][0])
    return sorted(j for j in K if j != i and Vi & set(K[j][0]))


def rank_f2(rows):
    basis = {}  # leading bit -> vector
    for r in rows:
        while r:
            lead = r.bit_length() - 1
            if lead in basis:
                r ^= basis[lead]
            else:
                basis[lead] = r
                break
    return len(basis)


# ---------- 1. R7: first use of a foreign symbol is not star-local ----------

def r7():
    # i declares d and already uses X.r2 (so the line i -> X exists).
    # W.w refs i.d and Q1.u1;  Q1.u1 refs Q2.u2 (case A only);  Q2.u2 refs X.r.
    owner = {"d": "i", "w": "W", "u1": "Q1", "u2": "Q2", "r": "X", "r2": "X"}
    base = [("d", "r2"), ("w", "d"), ("w", "u1"), ("u2", "r")]
    A = base + [("u1", "u2")]
    B = base
    edit = ("d", "r")  # first use of foreign X.r by i: R7
    out = []
    for name, refs in (("A (Q1.u1 refs Q2.u2)", A), ("B (it does not)", B)):
        before = among(owner, refs)
        after = among(owner, refs + [edit])
        st = star(owner, refs + [edit], "i")
        _, _, K = cover(owner, refs + [edit])
        stardata = {j: K[j] for j in ["i"] + st}
        out.append((name, before, after, st, stardata))
    same_star = out[0][4] == out[1][4]
    print("R7 counterexample (first use of foreign X.r by shard i):")
    for name, bf, af, st, _ in out:
        print(f"  case {name}: among {bf} -> {af} (delta {af - bf}); star(i) after edit = {st}")
    print(f"  star data (K_j for j in star(i) + i) identical in both cases: {same_star}")
    print("  => the update is not a function of star data: R7 is not star-local.")
    # the true update rule, checked on random covers
    rng = random.Random(7)
    ok = tot = 0
    for _ in range(3000):
        owner, refs = random_cover(rng)
        d = rng.choice(sorted(owner))
        i = owner[d]
        V, E, K = cover(owner, refs)
        Vi = set(K[i][0])
        cands = [x for x in owner if owner[x] != i and x not in Vi]
        if not cands:
            continue
        r = rng.choice(cands)
        comp = components(V, E)
        pred = 1 if comp[d] == comp[r] else 0  # delta b1(Sigma); b1(K_i) unchanged (pendant)
        got = among(owner, refs + [(d, r)]) - among(owner, refs)
        tot += 1
        ok += (got == pred)
    print(f"  rule 'delta = [d ~ r in Sigma]' holds {ok}/{tot} random R7 edits (global connectivity, not star data)")


# ---------- 2. K3 / spec section 5: '+1 when it joins two components' needs Sigma connected ----------

def k3():
    # fractured = 1 (i uses X.n), but i.b is an isolated declaration, so Sigma is disconnected.
    owner = {"a": "i", "b": "i", "n": "X"}
    refs = [("a", "n")]
    V, E, K = cover(owner, refs)
    print("K3 counterexample (private edit joining two components of K_i, Sigma disconnected):")
    print(f"  b0(Sigma) = {b0(V, E)}, b0(K_i) = {b0(*K['i'])}, line graph connected (i uses X.n): fractured = 1")
    before, after = among(owner, refs), among(owner, refs + [("b", "a")])
    print(f"  add i.b -> i.a (joins K_i's components {{a,n}} and {{b}}): among {before} -> {after} (spec: +1)")
    rng = random.Random(29)
    joins = plus = joins_conn = plus_conn = 0
    for _ in range(4000):
        owner, refs = random_cover(rng)
        V, E, K = cover(owner, refs)
        i = rng.choice(sorted(K))
        Vi, Ei = K[i]
        decls = [x for x in Vi if owner[x] == i]
        comp = components(Vi, Ei)
        d = rng.choice(decls)
        cands = [v for v in Vi if v != d and comp[v] != comp[d]]
        if not cands:
            continue
        r = rng.choice(cands)
        delta = among(owner, refs + [(d, r)]) - among(owner, refs)
        conn = b0(V, E) == 1
        joins += 1
        plus += (delta == 1)
        if conn:
            joins_conn += 1
            plus_conn += (delta == 1)
    print(f"  random joins: +1 in {plus}/{joins} overall; {plus_conn}/{joins_conn} when Sigma connected")
    print("  true rule for a private edge (d, r): delta = [d ~ r in Sigma] - [d ~ r in K_i]")


def random_cover(rng, n_max=5):
    n = rng.randint(2, n_max)
    owner = {}
    for s in range(n):
        for k in range(rng.randint(1, 4)):
            owner[f"s{s}.d{k}"] = f"S{s}"
    D = sorted(owner)
    refs = set()
    for d in D:
        for _ in range(rng.randint(0, 3)):
            r = rng.choice(D) if rng.random() < 0.4 else rng.choice([x for x in D if owner[x] == owner[d]])
            if r != d:
                refs.add((d, r))
    return owner, sorted(refs)


# ---------- 3. mutual reference: Def 1.1 multigraph vs simple graph ----------

def mutual():
    owner = {"a": "A", "b": "B"}
    refs = [("a", "b"), ("b", "a")]
    multi = among(owner, refs)
    # simple-graph reading: one undirected edge {a,b}, owned by both shards
    V = ["a", "b"]; E = [("a", "b")]
    KA = (["a", "b"], [("a", "b")]); KB = (["a", "b"], [("a", "b")])
    simple = b1(V, E) - b1(*KA) - b1(*KB)
    print("Mutual reference A.a -> B.b, B.b -> A.a:")
    print(f"  multigraph reading (Prop 1.4 holds): among = {multi}")
    print(f"  simple-graph reading: the edge {{a,b}} lies in K_A and K_B (Prop 1.4 fails); Euler count = {simple}")


# ---------- 4. Q1: a single line comma IS an H^1 class when a shard is frozen ----------

def frozen_line():
    """Linearised interface sheaf on one line Y - X over F2, in the 'variation' model:
    vertex stalk = admissible edits of that shard, edge stalk = facts on the line.
    Compiled sections are fixed; the defect w = rho_X(s_X) - rho_Y(s_Y) is a 1-cochain b,
    and the question 'can edits fix it' is: is b in im(delta)?"""
    # facts on the line: f0 = 'n exported as carrier', f1 = 'n used as action/2'. Bitmask over 2 facts.
    w = 0b11  # the defect: symmetric difference (X says carrier, Y says action/2)
    cases = {
        "both shards editable": {"X": [0b01, 0b10], "Y": [0b01, 0b10]},
        "X frozen (published / other repo), Y editable": {"X": [], "Y": [0b10]},
        "both frozen": {"X": [], "Y": []},
    }
    print("Q1: one line comma in the frozen-vertex (affine) model, F2:")
    for name, gens in cases.items():
        img = gens["X"] + gens["Y"]   # delta = rho_X - rho_Y, over F2 a sum
        r0 = rank_f2(img)
        r1 = rank_f2(img + [w])
        h1 = 2 - r0
        print(f"  {name}: dim H^1(G;F_var) = {h1}; defect class [w] {'= 0' if r1 == r0 else '!= 0'}")
    print("  => on a single edge (no cycle, owned by one pair) the defect is a non-zero H^1 class")
    print("     as soon as a vertex stalk cannot reach it. Prop 3.3 is a theorem about given sections;")
    print("     'a line comma is not H^1' is a choice of coefficient sheaf.")


# ---------- 5. Q5: non-constant mend sheaf: trivial holonomy everywhere, class non-trivial ----------

def mend_tree():
    """Tree of lines Y1 - X - Y2. Both Y1 and Y2 use X.f. Mends are renamings of f.
    Gamma = Z/2 = {id, swap f<->g}. Gauge groups: G(X) = Z/2 acting on BOTH lines at once
    (renaming X's export renames it everywhere: restriction G(X) -> G(e1) x G(e2) is the
    diagonal), G(Y1) = G(Y2) = 1 (published consumers; their uses are frozen).
    Mend family g_e1 = swap, g_e2 = id."""
    G_X = [(0, 0), (1, 1)]   # diagonal image in G(e1) x G(e2)
    G_Y1 = [0]; G_Y2 = [0]
    g = (1, 0)
    trivial = any(((hx[0] ^ hy1) == g[0]) and ((hx[1] ^ hy2) == g[1])
                  for hx in G_X for hy1 in G_Y1 for hy2 in G_Y2)
    print("Q5: mend family on a TREE of lines (no cycles, so every holonomy is trivial):")
    print(f"  g = (swap on Y1-X, id on Y2-X); gauge-equivalent to identity: {trivial}")
    print("  => non-trivial non-abelian H^1 with no cycle: X would have to export f as g to Y1")
    print("     and as f to Y2. Prop 3.5(c) needs G constant (restrictions G(S) -> G(e) iso);")
    print("     the interface mend sheaf is not constant, so 'holonomy trivial on all cycles'")
    print("     does not imply 'glues'.")


# ---------- 6. Def 1.3 vs Def 3.2: a line whose only use is absent is 'unused', not a comma ----------

def absent_line():
    # Y has `in @X` and uses X.f; X declares only g. Vertices are declarations, so (X, f) does
    # not exist; the reference has no edge; K_Y meets K_X nowhere.
    owner = {"y": "Y", "g": "X"}
    refs = []   # y -> X.f has no target vertex
    _, _, K = cover(owner, refs)
    overlap = set(K["Y"][0]) & set(K["X"][0])
    print("Absent export: Y `in @X`, uses X.f, X does not declare f:")
    print(f"  K_Y n K_X = {overlap or 'empty'} => Def 1.3 classes the line as UNUSED (a question, verdict partial),")
    print("  while spec K5 expects `comma line ... exports absent` (verdict comma). Same line, two verdicts.")


if __name__ == "__main__":
    r7(); print()
    k3(); print()
    mutual(); print()
    frozen_line(); print()
    mend_tree(); print()
    absent_line()
