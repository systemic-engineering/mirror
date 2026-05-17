in @prism
in @prism/rust
in @prism/compose
in @epistemologic
in @epistemologic/property
in @fate/connectome
in @ai/fate
in @ai/abyss
in @ai/introject
in @ai/cartographer
in @ai/explorer
in @compose/weighted
in @kintsugi
in @kintsugi/translate
in @kintsugi/migrate
in @kintsugi/lift
in @craft
in @code/rust
in @code/llvm
in @code/llvm/emit
in @io
in @fragmentation
in @mirror/evaluate
in @mirror/resolve
in @mirror/check
in @mirror/interpreter
in @mirror/runtime
in @nl
in @git/hooks
in @cli

# mirror.spec: the mirror binary describes itself.

type target = boot | cargo | binary

# all CLI flags from all imported grammars
out @cli/*

cli = @mirror/cli {
  kintsugi = @kintsugi {
    collapse(ast, ast) -> imperfect { \ }
    translate(ast, grammar) -> imperfect { \ }
    migrate(ast) -> imperfect { \ }
  }
  craft = @craft {
    craft(target) -> crystal {
      focus(target) |> split |> zoom |> refract |> project
    }
  }
}

# the two targets. kintsugi collapses them.
collapse(target(boot), target(cargo)) -> imperfect { \ }

# the self-hosting target.
target binary <| @code/llvm <| std
