import conversation/compiler
import conversation/trace
import gleam/erlang/process
import gleeunit/should

pub fn compile_grammar_returns_trace_test() {
  let assert Ok(started) = compiler.start()
  let subject = started.data
  let reply = process.new_subject()
  process.send(
    subject,
    compiler.CompileGrammar("grammar @test_compile {\n  type = a | b\n}\n", reply),
  )
  let assert Ok(Ok(t)) = process.receive(reply, 5000)
  case trace.value(t) {
    compiler.CompiledDomain(domain: "test_compile", ..) -> should.be_true(True)
    _ -> should.be_true(False)
  }
  process.send(subject, compiler.Shutdown)
}

pub fn compile_grammar_loads_module_test() {
  let assert Ok(started) = compiler.start()
  let subject = started.data
  let reply = process.new_subject()
  process.send(
    subject,
    compiler.CompileGrammar("grammar @test_loaded {\n  type = x | y\n}\n", reply),
  )
  let assert Ok(Ok(t)) = process.receive(reply, 5000)
  let compiled = trace.value(t)
  // Module name should be conv_test_loaded (conv_ prefix)
  should.equal(compiled.module, "conv_test_loaded")
  process.send(subject, compiler.Shutdown)
}

pub fn trace_is_verifiable_test() {
  let assert Ok(started) = compiler.start()
  let subject = started.data
  let reply = process.new_subject()
  process.send(
    subject,
    compiler.CompileGrammar("grammar @test_verify {\n  type = p | q\n}\n", reply),
  )
  let assert Ok(Ok(t)) = process.receive(reply, 5000)
  trace.verify(t, compiler.public_key()) |> should.be_true()
  process.send(subject, compiler.Shutdown)
}

pub fn compile_grammar_error_test() {
  let assert Ok(started) = compiler.start()
  let subject = started.data
  let reply = process.new_subject()
  // Source with no grammar block should error
  process.send(
    subject,
    compiler.CompileGrammar("template $t {\n  slug\n}\n", reply),
  )
  let assert Ok(Error(_msg)) = process.receive(reply, 5000)
  should.be_true(True)
  process.send(subject, compiler.Shutdown)
}
