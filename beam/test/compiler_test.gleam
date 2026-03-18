import conversation/compiler
import conversation/trace
import gleam/erlang/process
import gleeunit/should

pub fn compile_returns_trace_test() {
  let assert Ok(started) = compiler.start()
  let subject = started.data
  let reply = process.new_subject()
  process.send(
    subject,
    compiler.Compile("type X = a | b", compiler.Gleam, reply),
  )
  let assert Ok(Ok(t)) = process.receive(reply, 1000)
  case trace.value(t) {
    compiler.Source(compiler.Gleam, _, _) -> should.be_true(True)
    _ -> should.be_true(False)
  }
  process.send(subject, compiler.Shutdown)
}

pub fn compile_is_deterministic_test() {
  let assert Ok(started) = compiler.start()
  let subject = started.data
  let reply1 = process.new_subject()
  let reply2 = process.new_subject()
  process.send(
    subject,
    compiler.Compile("type X = a | b", compiler.Gleam, reply1),
  )
  process.send(
    subject,
    compiler.Compile("type X = a | b", compiler.Gleam, reply2),
  )
  let assert Ok(Ok(t1)) = process.receive(reply1, 1000)
  let assert Ok(Ok(t2)) = process.receive(reply2, 1000)
  let compiler.Source(_, src1, _) = trace.value(t1)
  let compiler.Source(_, src2, _) = trace.value(t2)
  should.equal(src1, src2)
  process.send(subject, compiler.Shutdown)
}

pub fn trace_is_verifiable_test() {
  let assert Ok(started) = compiler.start()
  let subject = started.data
  let reply = process.new_subject()
  process.send(
    subject,
    compiler.Compile("type X = a | b", compiler.Gleam, reply),
  )
  let assert Ok(Ok(t)) = process.receive(reply, 1000)
  trace.verify(t, compiler.public_key()) |> should.be_true()
  process.send(subject, compiler.Shutdown)
}

pub fn compile_fortran_target_test() {
  let assert Ok(started) = compiler.start()
  let subject = started.data
  let reply = process.new_subject()
  process.send(
    subject,
    compiler.Compile("type X = a | b", compiler.Fortran, reply),
  )
  let assert Ok(Ok(t)) = process.receive(reply, 1000)
  case trace.value(t) {
    compiler.Source(compiler.Fortran, _, _) -> should.be_true(True)
    _ -> should.be_true(False)
  }
  process.send(subject, compiler.Shutdown)
}

pub fn different_targets_different_oids_test() {
  let assert Ok(started) = compiler.start()
  let subject = started.data
  let reply_gleam = process.new_subject()
  let reply_fortran = process.new_subject()
  process.send(
    subject,
    compiler.Compile("type X = a", compiler.Gleam, reply_gleam),
  )
  process.send(
    subject,
    compiler.Compile("type X = a", compiler.Fortran, reply_fortran),
  )
  let assert Ok(Ok(t_gleam)) = process.receive(reply_gleam, 1000)
  let assert Ok(Ok(t_fortran)) = process.receive(reply_fortran, 1000)
  let compiler.Source(_, _, oid_gleam) = trace.value(t_gleam)
  let compiler.Source(_, _, oid_fortran) = trace.value(t_fortran)
  // Same source → same oid (content-addressed on source text)
  should.equal(oid_gleam, oid_fortran)
  process.send(subject, compiler.Shutdown)
}
