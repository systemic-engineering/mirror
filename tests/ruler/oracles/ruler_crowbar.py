"""Seam 2026-10-29: counterexamples against docs/math/2026-10-29-mara-ruler-and-pact.md
and docs/specs/mirror-ruler.md.

Ground (propositional) positive programs suffice for every law in math §5: a rule is
(head, [body atoms]); facts are rules with an empty body. Two independent restricted
runners: NAIVE (recompute T_P on the whole model each tick) and SEMI (join only rule
instances that use at least one atom new at the previous tick). Bans are a function
R(M_t) of the snapshot, as math Def 5.1 says.

Run: python3 ruler_crowbar.py
"""
import random, itertools


def TP(rules, M):
    return {h for h, body in rules if all(b in M for b in body)}


def lfp(rules, start=frozenset()):
    M = set(start)
    while True:
        N = M | TP(rules, M)
        if N == M:
            return M
        M = N


def run(rules, R, mode, budget=None):
    """Restricted run M_{t+1} = M_t u (new_t \\ B_t), B_t = R(M_t). Returns model,
    withheld [(atom, tick)], late [(atom, tick_deposited, tick_banned)], ticks, bans per tick."""
    M, delta = set(), None
    dep_tick, withheld, late, bans = {}, [], [], []
    t = 0
    while True:
        if budget is not None and t >= budget:
            break
        B = R(frozenset(M))
        bans.append(B)
        for a in B:
            if a in M and not any(x[0] == a for x in late):
                late.append((a, dep_tick[a], t))
        if mode == "naive":
            cand = TP(rules, M) - M
        else:  # semi-naive: some body atom in the previous delta (all rules at t = 0)
            cand = {h for h, body in rules
                    if all(b in M for b in body)
                    and (delta is None or any(b in delta for b in body))} - M
        new = cand - B
        withheld += [(a, t) for a in sorted(cand & B)]
        if not new:
            break
        for a in new:
            dep_tick[a] = t
        M |= new
        delta = new
        t += 1
    return M, withheld, late, t, bans


def show(name, rules, R, budget=None):
    out = {}
    for mode in ("naive", "semi"):
        M, W, L, t, _ = run(rules, R, mode, budget)
        out[mode] = (sorted(M), W, L, t)
    return out


# ---------------- 1. RR5: a lifted ban (non-monotone ruler) ----------------

def lifted_ban():
    rules = [("a", []), ("p", ["a"]), ("q", ["a"])]
    R = lambda M: frozenset({"p"}) if "q" not in M else frozenset()   # ban p until q is seen
    o = show("lifted", rules, R)
    Mn, Ms = set(o["naive"][0]), set(o["semi"][0])
    Bfinal = R(frozenset(Ms))
    fix = lfp([(h, b) for h, b in rules if h not in Bfinal])
    print("1. Lifted ban (R bans p while q is absent; facts a; p :- a; q :- a):")
    print(f"   naive model {sorted(Mn)}   semi-naive model {sorted(Ms)}")
    print(f"   withheld (semi) {o['semi'][1]}  late_bans {o['semi'][2]}")
    print(f"   final ban set {sorted(Bfinal)}; lfp under the final ruler {sorted(fix)}")
    print("   => semi-naive stops on an empty delta that is not a fixpoint of T^{B_final};")
    print("      the model depends on the evaluation strategy, `withheld` names a ruler whose")
    print("      ban is no longer in force, and late_bans is empty: the timing dependence is unreported.")


# ---------------- 2. RR4 / RR6 on random programs and random snapshot-dependent bans ----------------

def rand_prog(rng, n_atoms=8, n_rules=12):
    atoms = [f"x{i}" for i in range(n_atoms)]
    rules = [(rng.choice(atoms), [])] + [(rng.choice(atoms), [])]
    for _ in range(n_rules):
        rules.append((rng.choice(atoms), rng.sample(atoms, rng.randint(1, 2))))
    return atoms, rules


def rand_ruler(rng, atoms, monotone):
    # trigger -> banned atom pairs
    pairs = [(rng.choice(atoms), rng.choice(atoms)) for _ in range(rng.randint(1, 3))]
    always = frozenset(rng.sample(atoms, rng.randint(0, 1)))
    if monotone:
        return lambda M: always | frozenset(b for (t, b) in pairs if t in M)
    return lambda M: always | frozenset(b for (t, b) in pairs if t not in M)


def random_laws(trials=4000, seed=1029):
    rng = random.Random(seed)
    rr3 = rr6 = rr4 = rr4n = sn_eq = sn_n = nonmono = 0
    for _ in range(trials):
        atoms, rules = rand_prog(rng)
        mono = rng.random() < 0.5
        R = rand_ruler(rng, atoms, mono)
        full = lfp(rules)
        Mn, Wn, Ln, _, _ = run(rules, R, "naive")
        Ms, Ws, Ls, _, _ = run(rules, R, "semi")
        X = {a for a, _ in Ws}
        rr3 += Ms <= full
        rr6 += lfp(rules, Ms | X) == full
        sn_n += 1
        sn_eq += Mn == Ms
        if not mono:
            nonmono += Mn != Ms
        if mono and not Ls:
            Binf = R(frozenset(Ms))
            rr4n += 1
            rr4 += Ms == lfp([(h, b) for h, b in rules if h not in Binf])
    print(f"2. Random programs ({trials}), bans R(M_t) monotone or not:")
    print(f"   RR3  M_R subset lfp:                 {rr3}/{trials}")
    print(f"   RR6  lfp(M_R u withheld) = lfp(F):    {rr6}/{trials}")
    print(f"   RR4  monotone, no late ban => lfp(T^Binf): {rr4}/{rr4n}")
    print(f"   naive model = semi-naive model:      {sn_eq}/{sn_n} (differs in {nonmono} non-monotone runs)")


# ---------------- 3. R9 / RR3: a Choice ruler on a split grammar picks the meaning ----------------

def split_choice():
    """S <- A B ; A <- 'a' | 'a' 'a' ; B <- 'a' 'b' | 'b'  (split: unordered, both kept), input 'aab'.
    Ground clauses: m(X,i,j)."""
    c = [("c0a", []), ("c1a", []), ("c2b", [])]
    r = [("A01", ["c0a"]), ("A02", ["c0a", "c1a"]),
         ("B13", ["c1a", "c2b"]), ("B23", ["c2b"]),
         ("S03_via_A01_B13", ["A01", "B13"]), ("S03_via_A02_B23", ["A02", "B23"])]
    rules = c + r
    full = lfp(rules)
    parses = lambda M: sorted(x for x in M if x.startswith("S03"))
    print("3. Split grammar S <- A B, A <- a | aa, B <- ab | b on 'aab':")
    print(f"   unrestricted parses of S: {parses(full)}")
    for ban in (["A02"], ["A01"]):
        M, W, _, _, _ = run(rules, lambda _m, b=ban: frozenset(b), "semi")
        print(f"   Choice ruler bans {ban}: parses {parses(M)}; M_R subset lfp: {M <= full}")
    print("   => RR3 (M_R subset lfp) holds and the ruler still decides WHICH tree N is.")


# ---------------- 4. Bootstrap Prop 6.1: both directions of the iff ----------------

def bootstrap():
    """Grammar S <- 'a' ; T <- 'b' (T unused by S). Input 'a'. Bottom-up derives items for T
    at position 0 too. N is read off the start cone: the items S and its children."""
    rules = [("char0a", []), ("m_a_0_1", ["char0a"]), ("m_S_0_1", ["m_a_0_1"]),
             ("f_b_0", ["char0a"]), ("f_T_0", ["f_b_0"])]
    cone = {"m_S_0_1", "m_a_0_1", "char0a"}
    N = lambda M: sorted(M & cone)
    off, _, _, _, _ = run(rules, lambda m: frozenset(), "semi")
    on, W, _, _, _ = run(rules, lambda m: frozenset({"f_T_0"}), "semi")
    print("4. Bootstrap iff:")
    print(f"   (=>) ban f(T,0), outside the start cone: erasure {W}; N_off {N(off)}; N_on {N(on)}; equal: {N(off) == N(on)}")
    on2, W2, _, t2, _ = run(rules, lambda m: frozenset(), "semi", budget=2)
    print(f"   (<=) a Budget ruler of 2 ticks: erasure {W2}; N_on {N(on2)}; equal: {N(off) == N(on2)}")
    print("   => non-empty erasure with equal N, and empty erasure with different N.")


# ---------------- 5. R5: approval by reachability lets a ruler drop its constraint ----------------

def approval():
    """Nodes: H (human signature over G0's address), G0 (governance pact: 'changes need H'),
    G1 (new version of G0: 'changes need G1'), R (a ruler G1 approves).
    Edges X -> Y: Y approves X."""
    E = {"G0": ["H"], "G1": ["G0"], "G2": ["G1"], "R": ["G2"]}
    nodes = {"H", "G0", "G1", "G2", "R"}
    human = {"H"}

    def reach(x):
        seen, st = set(), [x]
        while st:
            y = st.pop()
            if y in seen:
                continue
            seen.add(y)
            st += E.get(y, [])
        return seen

    def terminal_sccs():
        sccs = []
        for v in nodes:
            comp = {u for u in reach(v) if v in reach(u)}
            if comp not in sccs:
                sccs.append(comp)
        return [c for c in sccs if all(w in c for u in c for w in E.get(u, []))]

    def r5(x):
        return all(c & human for c in terminal_sccs() if c <= reach(x))
    print("5. R5 by reachability:")
    for x in ("G1", "G2", "R"):
        print(f"   {x}: R5 satisfied = {r5(x)}")
    print("   H signed G0 only. G0 approved G1, whose content drops 'changes need H'; G1 approved G2")
    print("   and R. Every chain still ends in H's sink, so R5 holds while the constraint is gone.")


# ---------------- 6. R6 shape: Polycentric bought with a redundant check ----------------

def shape():
    # three copies of one ruler checking each other in a triangle: b1 = 1, and their readings are
    # equal by construction, so every holonomy is the identity.
    readings = {"R1": 5, "R2": 5, "R3": 5}
    checks = [("R1", "R2"), ("R2", "R3"), ("R3", "R1")]
    b1 = len(checks) - len(readings) + 1
    hol = sum(readings[b] - readings[a] for a, b in checks)
    commas = [(a, b) for a, b in checks if readings[a] != readings[b]]
    print("6. Shape: a triangle of identical rulers:")
    print(f"   b1 = {b1} => Polycentric; commas = {commas}; holonomy = {hol}")
    print("   => 'Polycentric' with no disagreement possible. b1 counts declared check edges, not capacity.")


# ---------------- 7. Two Choice rulers through the ban mechanism: union is not composition ----------------

def choice_compose():
    """Two parses x, y of one span. Choice A keeps the lexicographically least (bans y);
    Choice B keeps the heavier, w(y) > w(x) (bans x). Bans from several rulers are united
    (BanSet per Restrict, one run)."""
    rules = [("tok", []), ("x", ["tok"]), ("y", ["tok"]), ("S_x", ["x"]), ("S_y", ["y"])]
    A = lambda M: frozenset({"y"})
    B = lambda M: frozenset({"x"})
    S = lambda M: sorted(a for a in M if a.startswith("S_"))
    union, _, _, _, _ = run(rules, lambda M: A(M) | B(M), "semi")
    a_then_b = ["S_x"]   # A leaves {x}; B has nothing left to choose between
    b_then_a = ["S_y"]   # B leaves {y}; A has nothing left to choose between
    print("7. Two Choice rulers (A: least name; B: heavier):")
    print(f"   union of bans (the RR mechanism): parses {S(union)}  (a hole)")
    print(f"   A then B: {a_then_b};  B then A: {b_then_a}")
    print("   => for Choice, composition is ordered; united bans give a third answer that is neither.")


if __name__ == "__main__":
    choice_compose(); print()
    lifted_ban(); print()
    random_laws(); print()
    split_choice(); print()
    bootstrap(); print()
    approval(); print()
    shape()
