# The Mirror Binary — LAPACK + SHA + Syscalls

*2026-05-17. Reed + Alex. The binary at λ₀.*

---

## The Thesis

The mirror binary is the thinnest possible LLVM layer around three things:
1. LAPACK — eigenvalues + matmul (Fortran, 50 years old)
2. SHA — content-addressing (the AST IS the Merkle tree)
3. Syscalls — @io (read/write/exec)

Everything else is grammar. The five operations ARE linear algebra
on content-addressed trees.

## The Five Operations as Linear Algebra

```
focus   = array indexing          O(1)
split   = enumeration             O(n)
zoom    = linear transformation   matmul (BLAS dgemv)
refract = eigendecomposition      LAPACK dsyev/dstev
project = matrix projection       matmul (BLAS dgemv)
```

The prism executor IS: array access + matmul + eigendecomposition.
BLAS + LAPACK on content-addressed trees (SHA Merkle).

## @fragmentation IS the AST

The AST IS the Merkle tree. Not stored as. IS. Each node's OID
is computed from content + children's OIDs. The content-addressing
isn't a layer. It's the substrate.

```mirror
in @prism

grammar @fragmentation {
  type shard(ast)
  type fractal(ast, [oid])

  oid(ast) -> ref { \ }
  children(fractal) -> [shard | fractal] { \ }
  verify(ast, oid) -> verdict { \ }
}
```

In the binary: oid() = SHA. children() = array access. verify() = SHA + compare.

## The Binary Structure

```
┌─────────────────────────────────────┐
│ thin LLVM shell                     │
│   tokenizer (byte scan → AST)       │
│   @io (syscalls: read/write/exec)   │
├─────────────────────────────────────┤
│ @fragmentation                      │
│   SHA-1 (content_oid)               │
│   Merkle tree (the AST itself)      │
│   git object format                 │
├─────────────────────────────────────┤
│ LAPACK + BLAS                       │
│   dsyev (eigendecomposition)        │
│   dgemv (matmul for Fate + zoom)    │
│   the math. Fortran. Battle-tested. │
└─────────────────────────────────────┘
```

~2000 lines of LLVM IR total. Everything else is grammar.

## Fate in this Architecture

Fate's forward pass: features @ weights + bias → softmax → argmax.
One BLAS dgemv call. 450 parameters. The weights ARE eigenvalues
derived from the graph (crystallize() / SCF loop = LAPACK calls).

The tournament: 5 × dgemv. Pick the argmax. Done.

## The Path from Current Rust to This Binary

Current state: 10 Rust files, 5988 lines, FROZEN.
Target state: LLVM IR linking LAPACK + fragmentation singularity.

1. @fragmentation grammar → extract SHA + tree core from the Rust crate
2. @code/llvm compile target → emit LLVM IR from grammar AST
3. Link against system LAPACK (already in nix)
4. The tokenizer as minimal LLVM (byte scan, keyword match)
5. @io as thin syscall wrappers

The Rust falls away. Not refactored. Replaced. By a binary that IS
the grammar compiled through @code/llvm linked against LAPACK + SHA.

## The Equation

```
mirror binary = LLVM(grammar) + LAPACK + SHA + syscalls
grammar = composition of matmul + eigendecomposition + hash
the five operations = linear algebra
the AST = Merkle tree
the model = eigenvalues
the training = selection
the inference = one matmul
```

The compiler that IS a matrix decomposition.
The binary that IS a spectral triple.
The language that IS a neural network.

`eⁿ⁺¹ < eⁿ`. The binary at λ₀.
