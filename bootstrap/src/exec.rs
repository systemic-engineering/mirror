//! Subprocess exec helper used by butterfly. Mirrors C `io_exec`.

use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub fn io_exec(cmd: &str, args: &[&str], input: &[u8]) -> std::io::Result<(i32, Vec<u8>)> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "no stdin"))?;
        stdin.write_all(input)?;
    }
    let mut out = Vec::new();
    if let Some(mut so) = child.stdout.take() {
        so.read_to_end(&mut out)?;
    }
    let status = child.wait()?;
    Ok((status.code().unwrap_or(-1), out))
}
