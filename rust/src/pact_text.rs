//! `mirror pact eval <file>`: read one pact-text file, run the engine's
//! well-founded wrapper `W` once, print one mirror-text record.
//!
//! Admitted by `docs/audits/2026-10-29-seam-pact-eval-and-ruler-envelope.md`
//! §1. The `@io` boundary: it reads one file (its argument) and prints one
//! record to stdout. No other file, no write, no store, no git, no socket,
//! no process, no why-not trees, no JSON.
//!
//! The reader implements exactly the grammar of audit §1.2 (with Mara's
//! 2026-10-29 amendment, `functional p(1, 2) -> 3`), and it is FROZEN: it
//! retires when `E(G_pact)` reads the same fixtures, and is never extended
//! in place. Anything outside the grammar is `dark` at its line and column.
//!
//! Verdicts and exit codes (audit §1.4, amended by Alex 2026-10-29:
//! "refused is Dark. The illumination was declined. … The discoball is
//! selective."): `clear` 0, `opaque` 1, `dark` 2 (the bytes or the program
//! don't read; `shaped_by none`), and `dark` 255, a *selected* Dark: a
//! definite functional violation on `true`, whose `shaped_by` names the
//! declining declaration(s) by surface text. `refused` is never printed.
//! Usage errors print to stderr, no record, exit 2.

use engine::{
    Atom, Const, Dark, Fact, Facts, Functional, Instance, Literal, NormalProgram, NormalRule,
    Outcome, Position, Predicate, RuleId, Status, Sym, Term, Var,
};
use std::collections::{BTreeMap, BTreeSet};
use std::process::ExitCode;

/// The altitude of the facts this verb reads.
const ALTITUDE: &str = "@pact/fact";

const USAGE: &str = "usage: mirror pact eval <file>";

/// `mirror pact <rest…>`.
pub(crate) fn cmd_pact(rest: &[String]) -> ExitCode {
    let path = match rest {
        [sub, path] if sub == "eval" && !path.starts_with('-') => path,
        [sub, ..] if sub != "eval" => return usage(&format!("unknown pact verb `{sub}`")),
        [_] => return usage("missing <file>"),
        [] => return usage("missing `eval <file>`"),
        _ => return usage("expected exactly one <file> and no flags"),
    };
    let record = match std::fs::read(path) {
        Ok(bytes) => evaluate(&bytes),
        Err(e) => Record::Dark(format!("io: cannot read the file ({})", e.kind())),
    };
    let (text, exit) = record.render(path);
    print!("{text}");
    ExitCode::from(exit)
}

fn usage(why: &str) -> ExitCode {
    eprintln!("mirror pact: {why}");
    eprintln!("{USAGE}");
    ExitCode::from(2)
}

// ── The record ───────────────────────────────────────────────────────────

enum Record {
    Dark(String),
    Done(Summary),
}

struct Summary {
    true_lines: Vec<String>,
    undefined_lines: Vec<String>,
    false_lines: Vec<String>,
    findings: Vec<String>,
    /// The declarations that declined the illumination: those whose
    /// predicate has a finding on `true`, as surface text, sorted.
    declined: Vec<String>,
    outcome: Outcome,
}

/// One `<key> <value>` line, the key padded to 12 columns.
fn kv(out: &mut String, key: &str, value: &str) {
    out.push_str(&format!("{key:<12}{value}\n"));
}

fn quote(path: &str) -> String {
    let mut q = String::from("\"");
    for ch in path.chars() {
        match ch {
            '"' => q.push_str("\\\""),
            '\\' => q.push_str("\\\\"),
            c => q.push(c),
        }
    }
    q.push('"');
    q
}

impl Record {
    fn render(&self, path: &str) -> (String, u8) {
        let mut out = String::new();
        match self {
            Record::Dark(why) => {
                // Unreadable: nothing selected it, so nothing shaped it.
                kv(&mut out, "verdict", "dark");
                kv(&mut out, "file", &quote(path));
                kv(&mut out, "shaped_by", "none");
                kv(&mut out, "dark", why);
                (out, 2)
            }
            Record::Done(s) => {
                let (verdict, exit) = if !s.findings.is_empty() {
                    ("dark", 255) // a selected Dark: the illumination was declined
                } else if s.outcome == Outcome::Opaque {
                    ("opaque", 1)
                } else {
                    ("clear", 0)
                };
                kv(&mut out, "verdict", verdict);
                kv(&mut out, "file", &quote(path));
                if s.findings.is_empty() {
                    // W runs to its fixed point: no ruler shaped this result.
                    kv(&mut out, "shaped_by", "none");
                } else {
                    for d in &s.declined {
                        kv(&mut out, "shaped_by", d);
                    }
                }
                kv(&mut out, "true", &s.true_lines.len().to_string());
                kv(&mut out, "undefined", &s.undefined_lines.len().to_string());
                kv(&mut out, "findings", &s.findings.len().to_string());
                let details: Vec<(&str, &String)> = s
                    .true_lines
                    .iter()
                    .map(|l| ("true", l))
                    .chain(s.undefined_lines.iter().map(|l| ("undefined", l)))
                    .chain(s.false_lines.iter().map(|l| ("false", l)))
                    .chain(s.findings.iter().map(|l| ("finding", l)))
                    .collect();
                if !details.is_empty() {
                    out.push('\n');
                    for (k, v) in details {
                        kv(&mut out, k, v);
                    }
                }
                (out, exit)
            }
        }
    }
}

// ── Evaluation ───────────────────────────────────────────────────────────

fn evaluate(bytes: &[u8]) -> Record {
    let read = match read(bytes) {
        Ok(r) => r,
        Err(d) => return Record::Dark(d.to_string()),
    };
    let rules: Vec<NormalRule> = read.rules.iter().map(|(r, _)| r.clone()).collect();
    let decls: Vec<Functional> = read.decls.iter().map(|(d, _)| d.clone()).collect();
    let program = match NormalProgram::new(rules, decls) {
        Ok(p) => p,
        Err(d) => return Record::Dark(engine_dark(&d, &read)),
    };
    let altitude = engine::Altitude::new(ALTITUDE).expect("a constant altitude");
    let facts = Facts {
        altitude,
        atoms: read.facts.clone(),
    };
    let instance = match Instance::new(&program, facts) {
        Ok(i) => i,
        Err(d) => return Record::Dark(engine_dark(&d, &read)),
    };
    let w = engine::well_founded(&instance);

    // `true` and `undefined` list every intensional atom (a rule head's
    // predicate); `false` lists queried atoms only (oracles.md).
    let intensional: BTreeSet<&str> = read
        .rules
        .iter()
        .map(|(r, _)| r.head.pred.as_str())
        .collect();
    let listed = |set: &BTreeMap<engine::Addr, Fact>| -> Vec<String> {
        let mut v: Vec<String> = set
            .values()
            .filter(|f| intensional.contains(f.pred.as_str()))
            .map(render_fact)
            .collect();
        v.sort();
        v
    };
    let mut false_lines: Vec<String> = read
        .queries
        .iter()
        .filter(|(q, _)| w.status(q) == Status::False)
        .map(|(q, _)| render_fact(q))
        .collect();
    false_lines.sort();
    false_lines.dedup();
    // A finding names a predicate, not a declaration, so every declaration
    // on a violated predicate is listed as declining.
    let violated: BTreeSet<&str> = w.findings.iter().map(|v| v.pred().as_str()).collect();
    let mut declined: Vec<String> = read
        .decls
        .iter()
        .filter(|(d, _)| violated.contains(d.pred.as_str()))
        .map(|(d, _)| render_decl(d))
        .collect();
    declined.sort();
    declined.dedup();
    let findings = w
        .findings
        .iter()
        .map(|v| {
            let key: Vec<String> = v.key().iter().map(render_const).collect();
            format!(
                "functional {} key({}) {} {}",
                v.pred().as_str(),
                key.join(", "),
                render_const(v.least()),
                render_const(v.other())
            )
        })
        .collect();
    Record::Done(Summary {
        true_lines: listed(&w.true_atoms),
        undefined_lines: listed(&w.undefined),
        false_lines,
        findings,
        declined,
        outcome: w.outcome,
    })
}

/// A declaration's surface text: `functional p(1, 2) -> 3`.
fn render_decl(d: &Functional) -> String {
    let key: Vec<String> = d.key.iter().map(|p| p.get().to_string()).collect();
    format!(
        "functional {}({}) -> {}",
        d.pred.as_str(),
        key.join(", "),
        d.value.get()
    )
}

/// An engine `Dark`, named in the file's terms where the file says where.
fn engine_dark(d: &Dark, read: &Read) -> String {
    match d {
        Dark::Unsafe { rule, var } => {
            let at = read
                .rules
                .iter()
                .find(|(r, _)| &r.id == rule)
                .map(|(_, at)| *at);
            let what = format!(
                "unsafe: ?{} in rule {} is not bound by a positive literal",
                var.as_str(),
                rule.as_str()
            );
            match at {
                Some(at) => format!("{at} {what}"),
                None => what,
            }
        }
        Dark::Arity {
            pred,
            expected,
            found,
        } => {
            format!(
                "arity: {} expected {} found {}",
                pred.as_str(),
                expected.0,
                found.0
            )
        }
        Dark::Functional { pred, position } => {
            let at = read
                .decls
                .iter()
                .find(|(f, _)| &f.pred == pred)
                .map(|(_, at)| *at);
            let what = format!(
                "functional: {} has no position {} for this declaration",
                pred.as_str(),
                position.get()
            );
            match at {
                Some(at) => format!("{at} {what}"),
                None => what,
            }
        }
    }
}

fn render_const(c: &Const) -> String {
    match c {
        Const::Sym(s) => s.as_str().to_string(),
        Const::Nat(n) => n.0.to_string(),
        Const::Addr(a) => a.as_bytes().iter().map(|b| format!("{b:02x}")).collect(),
    }
}

/// The oracle's surface form: `p` or `p(a, b)`.
fn render_fact(f: &Fact) -> String {
    if f.args.is_empty() {
        return f.pred.as_str().to_string();
    }
    let args: Vec<String> = f.args.iter().map(render_const).collect();
    format!("{}({})", f.pred.as_str(), args.join(", "))
}

// ── The reader (audit §1.2, frozen) ──────────────────────────────────────

/// A 1-based line and byte column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct At {
    line: usize,
    col: usize,
}

impl std::fmt::Display for At {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

struct ReadError {
    at: At,
    why: String,
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.at, self.why)
    }
}

struct Read {
    /// Each rule with the position of its id.
    rules: Vec<(NormalRule, At)>,
    facts: Vec<Fact>,
    /// Each query with the position of its predicate.
    queries: Vec<(Fact, At)>,
    /// Each declaration with the position of its predicate.
    decls: Vec<(Functional, At)>,
}

fn read(bytes: &[u8]) -> Result<Read, ReadError> {
    let mut r = Read {
        rules: vec![],
        facts: vec![],
        queries: vec![],
        decls: vec![],
    };
    let mut ids: BTreeMap<String, At> = BTreeMap::new();
    // Each predicate's arity, from the rules and facts, for the query check.
    let mut lines: Vec<&[u8]> = bytes.split(|&b| b == b'\n').collect();
    if lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop(); // the final LF
    }
    for (n, line) in lines.iter().enumerate() {
        let mut c = Cur {
            s: line,
            i: 0,
            line: n + 1,
        };
        if let Some(i) = line.iter().position(|&b| !(0x20..=0x7e).contains(&b)) {
            c.i = i;
            return Err(c.err(format!("byte 0x{:02x} is outside ASCII 0x20-0x7E", line[i])));
        }
        if line.iter().all(|&b| b == b' ') {
            continue; // blank
        }
        if line[0] == b'#' {
            continue; // comment, column 1 only
        }
        if line[0] == b' ' {
            return Err(c.err("a record starts at column 1".into()));
        }
        let key_end = line
            .iter()
            .position(|b| !b.is_ascii_lowercase())
            .unwrap_or(line.len());
        let key = &line[..key_end];
        match key {
            b"rule" | b"fact" | b"query" | b"functional" => {}
            _ => return Err(c.err("unknown key (rule, fact, query, functional)".into())),
        }
        c.i = key_end;
        c.sp()?;
        match key {
            b"rule" => {
                let id_at = c.at();
                let id = c.rule_id()?;
                if ids.contains_key(&id) {
                    return Err(ReadError {
                        at: id_at,
                        why: format!("duplicate rule id `{id}`"),
                    });
                }
                ids.insert(id.clone(), id_at);
                c.sp()?;
                let head = c.atom(Mode::Open)?;
                c.sp()?;
                c.word("requires")?;
                c.sp()?;
                let mut body = vec![c.literal()?];
                while c.peek() == Some(b',') {
                    c.i += 1;
                    c.sp0();
                    body.push(c.literal()?);
                }
                c.end()?;
                let id = RuleId::new(&id).ok_or_else(|| ReadError {
                    at: id_at,
                    why: "rule id".into(),
                })?;
                r.rules.push((NormalRule { id, head, body }, id_at));
            }
            b"fact" => {
                let a = c.atom(Mode::Ground("a fact is ground"))?;
                c.end()?;
                r.facts.push(ground(a));
            }
            b"query" => {
                let at = c.at();
                let a = c.atom(Mode::Ground("a query is ground"))?;
                c.end()?;
                r.queries.push((ground(a), at));
            }
            _ => {
                let at = c.at();
                let pred = c.pred()?;
                c.byte(b'(')?;
                let mut key = vec![c.position()?];
                while c.peek() == Some(b',') {
                    c.i += 1;
                    c.sp0();
                    key.push(c.position()?);
                }
                c.byte(b')')?;
                c.sp()?;
                c.word("->")?;
                c.sp()?;
                let value = c.position()?;
                c.end()?;
                r.decls.push((Functional { pred, key, value }, at));
            }
        }
    }

    // Queries: arity agrees with the program's signature (Instance::new
    // checks facts, not queries).
    let mut arity: BTreeMap<&str, usize> = BTreeMap::new();
    for (rule, _) in &r.rules {
        let atoms = std::iter::once(&rule.head).chain(rule.body.iter().map(|l| match l {
            Literal::Pos(a) | Literal::Not(a) => a,
        }));
        for a in atoms {
            arity.entry(a.pred.as_str()).or_insert(a.args.len());
        }
    }
    for f in &r.facts {
        arity.entry(f.pred.as_str()).or_insert(f.args.len());
    }
    for (q, at) in &r.queries {
        if let Some(&n) = arity.get(q.pred.as_str()) {
            if n != q.args.len() {
                return Err(ReadError {
                    at: *at,
                    why: format!(
                        "query arity {} but `{}` has arity {n}",
                        q.args.len(),
                        q.pred.as_str()
                    ),
                });
            }
        }
    }
    Ok(r)
}

/// An atom the reader produced in `Mode::Ground` holds only constants.
fn ground(a: Atom) -> Fact {
    Fact {
        pred: a.pred,
        args: a
            .args
            .into_iter()
            .filter_map(|t| match t {
                Term::Const(c) => Some(c),
                Term::Var(_) => None, // unreachable: Mode::Ground refuses variables
            })
            .collect(),
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Open,
    Ground(&'static str),
}

/// A cursor over one line.
struct Cur<'a> {
    s: &'a [u8],
    i: usize,
    line: usize,
}

impl Cur<'_> {
    fn at(&self) -> At {
        At {
            line: self.line,
            col: self.i + 1,
        }
    }

    fn err(&self, why: String) -> ReadError {
        ReadError { at: self.at(), why }
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }

    fn found(&self) -> String {
        match self.peek() {
            None => "end of line".into(),
            Some(b) => format!("`{}`", b as char),
        }
    }

    /// `sp = " " , { " " }`.
    fn sp(&mut self) -> Result<(), ReadError> {
        if self.peek() != Some(b' ') {
            return Err(self.err(format!("expected a space, found {}", self.found())));
        }
        self.sp0();
        Ok(())
    }

    fn sp0(&mut self) {
        while self.peek() == Some(b' ') {
            self.i += 1;
        }
    }

    fn byte(&mut self, b: u8) -> Result<(), ReadError> {
        if self.peek() != Some(b) {
            return Err(self.err(format!("expected `{}`, found {}", b as char, self.found())));
        }
        self.i += 1;
        Ok(())
    }

    fn word(&mut self, w: &str) -> Result<(), ReadError> {
        if !self.s[self.i..].starts_with(w.as_bytes()) {
            return Err(self.err(format!("expected `{w}`, found {}", self.found())));
        }
        self.i += w.len();
        Ok(())
    }

    /// `{ " " }` then end of line: no trailing comments, nothing else.
    fn end(&mut self) -> Result<(), ReadError> {
        self.sp0();
        match self.peek() {
            None => Ok(()),
            Some(_) => Err(self.err(format!("expected end of line, found {}", self.found()))),
        }
    }

    /// `name = lower , { lower | digit | "_" }`.
    fn name(&mut self) -> Result<String, ReadError> {
        match self.peek() {
            Some(b) if b.is_ascii_lowercase() => {}
            Some(b) if b.is_ascii_digit() => {
                return Err(self.err("numeric constants are not in the grammar".into()));
            }
            _ => {
                return Err(self.err(format!(
                    "expected a name [a-z][a-z0-9_]*, found {}",
                    self.found()
                )))
            }
        }
        let start = self.i;
        while self
            .peek()
            .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        {
            self.i += 1;
        }
        Ok(String::from_utf8_lossy(&self.s[start..self.i]).into_owned())
    }

    /// A name that may be a predicate or a constant: not `not`, not `requires`.
    fn unreserved(&mut self) -> Result<String, ReadError> {
        let at = self.at();
        let n = self.name()?;
        if n == "not" || n == "requires" {
            return Err(ReadError {
                at,
                why: format!("`{n}` is reserved, not a name"),
            });
        }
        Ok(n)
    }

    fn pred(&mut self) -> Result<Predicate, ReadError> {
        let at = self.at();
        let n = self.unreserved()?;
        Predicate::new(&n).ok_or(ReadError {
            at,
            why: "predicate".into(),
        })
    }

    /// `rule-id = lower , { lower | digit | "_" | "-" }`.
    fn rule_id(&mut self) -> Result<String, ReadError> {
        if !self.peek().is_some_and(|b| b.is_ascii_lowercase()) {
            return Err(self.err(format!(
                "expected a rule id [a-z][a-z0-9_-]*, found {}",
                self.found()
            )));
        }
        let start = self.i;
        while self
            .peek()
            .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
        {
            self.i += 1;
        }
        Ok(String::from_utf8_lossy(&self.s[start..self.i]).into_owned())
    }

    /// `term = var | sym`.
    fn term(&mut self, mode: Mode) -> Result<Term, ReadError> {
        if self.peek() == Some(b'?') {
            if let Mode::Ground(why) = mode {
                return Err(self.err(format!("{why}: no variables")));
            }
            let at = self.at();
            self.i += 1;
            let n = self.name()?;
            return Var::new(&n).map(Term::Var).ok_or(ReadError {
                at,
                why: "variable".into(),
            });
        }
        let at = self.at();
        let n = self.unreserved()?;
        Sym::new(&n)
            .map(|s| Term::Const(Const::Sym(s)))
            .ok_or(ReadError {
                at,
                why: "constant".into(),
            })
    }

    /// `atom = pred | pred , "(" , term , { "," , sp0 , term } , ")"`.
    fn atom(&mut self, mode: Mode) -> Result<Atom, ReadError> {
        let pred = self.pred()?;
        self.atom_args(pred, mode)
    }

    fn atom_args(&mut self, pred: Predicate, mode: Mode) -> Result<Atom, ReadError> {
        let mut args = Vec::new();
        if self.peek() == Some(b'(') {
            self.i += 1;
            if self.peek() == Some(b')') {
                return Err(self.err("`p()` is dark: a zero-ary atom takes no parentheses".into()));
            }
            args.push(self.term(mode)?);
            loop {
                match self.peek() {
                    Some(b',') => {
                        self.i += 1;
                        self.sp0();
                        args.push(self.term(mode)?);
                    }
                    Some(b')') => {
                        self.i += 1;
                        break;
                    }
                    _ => {
                        return Err(self.err(format!("expected `,` or `)`, found {}", self.found())))
                    }
                }
            }
        }
        Ok(Atom { pred, args })
    }

    /// `literal = atom | "not" , sp , atom`.
    fn literal(&mut self) -> Result<Literal, ReadError> {
        let at = self.at();
        let n = self.name()?;
        if n == "not" {
            if self.peek() == Some(b' ') {
                self.sp0();
                return Ok(Literal::Not(self.atom(Mode::Open)?));
            }
            return Err(ReadError {
                at,
                why: "`not` is reserved: write `not <atom>`".into(),
            });
        }
        if n == "requires" {
            return Err(ReadError {
                at,
                why: "`requires` is reserved, not a name".into(),
            });
        }
        let pred = Predicate::new(&n).ok_or(ReadError {
            at,
            why: "predicate".into(),
        })?;
        Ok(Literal::Pos(self.atom_args(pred, Mode::Open)?))
    }

    /// `position = nonzero-digit , { digit }`: a 1-based argument index.
    fn position(&mut self) -> Result<Position, ReadError> {
        let at = self.at();
        if !self.peek().is_some_and(|b| (b'1'..=b'9').contains(&b)) {
            return Err(self.err(format!(
                "expected a position [1-9][0-9]*, found {}",
                self.found()
            )));
        }
        let start = self.i;
        while self.peek().is_some_and(|b| b.is_ascii_digit()) {
            self.i += 1;
        }
        std::str::from_utf8(&self.s[start..self.i])
            .ok()
            .and_then(|d| d.parse::<u32>().ok())
            .and_then(Position::new)
            .ok_or(ReadError {
                at,
                why: "position out of range".into(),
            })
    }
}
