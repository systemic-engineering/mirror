in @prism
in @kintsugi
in @craft

-- mirror.spec: the mirror binary describes itself.
-- Two targets. Kintsugi collapses them.
-- The loss measures drift between grammar and implementation.

type target = boot | cargo | binary

craft(target) -> crystal {
  focus(target) |> split |> zoom |> refract |> project
}

target binary <| @code/llvm <| std

collapse(target(boot), target(cargo)) -> imperfect { \ }
