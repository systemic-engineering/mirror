//! `ruler`: mirror's ruler envelope (`docs/specs/mirror-ruler.md` §1,
//! §1.1), admitted by `docs/audits/2026-10-29-seam-pact-eval-and-ruler-envelope.md`
//! §2.
//!
//! A narrowing parameter is a [`Ruler`]: an addressed record of a parsed
//! body. Its address is `H("ruler\0" ‖ τ ‖ enc(canonical body))` with
//! `τ = "@mirror/ruler" ‖ 0x00 ‖ 0x00` (spec §1.1). Nothing builds a ruler
//! from an address; the address is computed from the body, always.
//!
//! A result a ruler narrowed is a [`Shaped`]: it names the rulers that
//! shaped it (`shaped_by`), what they cut away (`erasure`), and its
//! approval. In this slice the only approval is [`Approval::Unsigned`]: the
//! ruler is real and addressed, and nobody has signed it yet. Signing,
//! `Approved` and `verify` wait for their own audit (D1, D7).
//!
//! The only body is [`Count`] (a budget). [`Body`] is sealed.
//!
//! ```compile_fail
//! // No ruler from an address, and no struct literal: the fields are private.
//! let r = ruler::Ruler { addr: todo!(), reader: todo!(), body: todo!() };
//! ```
//!
//! ```compile_fail
//! #![deny(unused_must_use)]
//! // A shaped result can't be dropped unread.
//! fn f(i: &engine::Instance<'_, engine::PositiveProgram>, r: &ruler::Ruler<ruler::Count>) {
//!     ruler::run_bounded(i, r).unwrap();
//! }
//! ```
//!
//! Pure: no I/O, no keys, no clock, no randomness, no `unsafe`.

#![forbid(unsafe_code)]

use engine::{Addr, Bounded, Budget, Instance, PositiveProgram, Run, Tick};
use std::num::NonZeroU64;

// ── Bodies ───────────────────────────────────────────────────────────────

/// What a [`Count`] counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Unit {
    /// Core rounds of `E` (engine `Tick`).
    Ticks,
    /// Outer rounds of `W` (engine `Round`).
    Rounds,
}

/// The Budget family's only body: at most `n` units.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Count {
    pub n: NonZeroU64,
    pub unit: Unit,
}

/// Whether a ruler's reading depends on a reader (spec R3). Part of the
/// address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReaderDependence {
    Independent,
    /// Reader-dependent, naming the reader. Refused for [`Count`].
    Dependent(Addr),
}

mod sealed {
    pub trait Sealed {
        /// The canonical body bytes (spec §1.1), or `None` if this body
        /// doesn't admit the reader dependence.
        fn canonical(&self, reader: &super::ReaderDependence) -> Option<Vec<u8>>;
    }
}

/// A ruler body. Sealed: the body set is closed in this slice.
pub trait Body: sealed::Sealed {}

const FAMILY_BUDGET: u8 = 0x01;
const BODY_COUNT: u8 = 0x01;
const READER_INDEPENDENT: u8 = 0x00;
const UNIT_TICKS: u8 = 0x01;
const UNIT_ROUNDS: u8 = 0x02;

impl sealed::Sealed for Count {
    /// `family ‖ body ‖ reader ‖ u64_be(n) ‖ unit`: 12 bytes.
    fn canonical(&self, reader: &ReaderDependence) -> Option<Vec<u8>> {
        if *reader != ReaderDependence::Independent {
            return None; // spec §1 table: Count admits only Independent
        }
        let mut b = vec![FAMILY_BUDGET, BODY_COUNT, READER_INDEPENDENT];
        b.extend_from_slice(&self.n.get().to_be_bytes());
        b.push(match self.unit {
            Unit::Ticks => UNIT_TICKS,
            Unit::Rounds => UNIT_ROUNDS,
        });
        Some(b)
    }
}

impl Body for Count {}

// ── The ruler ────────────────────────────────────────────────────────────

/// An addressed narrowing parameter. Built only by [`Ruler::new`], which
/// computes the address from the body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ruler<B: Body> {
    addr: Addr,
    reader: ReaderDependence,
    body: B,
}

/// A body that doesn't admit the declared reader dependence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReaderRefused {
    pub reader: ReaderDependence,
}

const TAU: &[u8] = b"@mirror/ruler\0\0";

impl<B: Body> Ruler<B> {
    /// `addr = H("ruler\0" ‖ τ ‖ enc(canonical body))` (spec §1.1).
    pub fn new(body: B, reader: ReaderDependence) -> Result<Ruler<B>, ReaderRefused> {
        let canonical = body.canonical(&reader).ok_or(ReaderRefused { reader })?;
        let mut h = blake3::Hasher::new();
        h.update(b"ruler\0");
        h.update(TAU);
        h.update(&(canonical.len() as u64).to_be_bytes());
        h.update(&canonical);
        let hex: String = h.finalize().as_bytes().iter().map(|b| format!("{b:02x}")).collect();
        // engine::Addr's only public door for outside bytes is its checked parse.
        let addr = Addr::from_hex(&hex).expect("64 hex digits from a BLAKE3 digest");
        Ok(Ruler { addr, reader, body })
    }

    pub fn addr(&self) -> Addr {
        self.addr
    }

    pub fn reader(&self) -> &ReaderDependence {
        &self.reader
    }

    pub fn body(&self) -> &B {
        &self.body
    }
}

// ── Shaped results ───────────────────────────────────────────────────────

/// One or more.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonEmpty<T> {
    head: T,
    tail: Vec<T>,
}

impl<T> NonEmpty<T> {
    pub fn one(head: T) -> NonEmpty<T> {
        NonEmpty { head, tail: Vec::new() }
    }

    pub fn first(&self) -> &T {
        &self.head
    }

    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }

    /// Never true; here for the `len` convention.
    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.head).chain(self.tail.iter())
    }
}

/// Who approved a ruler. In this slice nobody can: there is no signing,
/// no `Approved` and no `verify` (audit §2.4, pending D1 and D7).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Approval {
    Unsigned,
}

/// A result some ruler narrowed: the value, the rulers that shaped it,
/// the observers that fed them, what was cut away, and the approval.
#[must_use = "a shaped result names what narrowed it; read it"]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shaped<T, E> {
    value: T,
    shaped_by: NonEmpty<Addr>,
    observed_by: Vec<Addr>,
    erasure: Option<E>,
    approval: Approval,
}

impl<T, E> Shaped<T, E> {
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn shaped_by(&self) -> &NonEmpty<Addr> {
        &self.shaped_by
    }

    /// Always empty in this slice (no `Observe`).
    pub fn observed_by(&self) -> &[Addr] {
        &self.observed_by
    }

    /// What the rulers cut away, if anything.
    pub fn erasure(&self) -> Option<&E> {
        self.erasure.as_ref()
    }

    pub fn approval(&self) -> Approval {
        self.approval
    }
}

/// [`Count`]'s erasure: the run stopped at its budget, at this tick, with
/// a non-empty delta still pending.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Truncated {
    pub at: Tick,
}

/// A [`Count`] in another unit than the call counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnitMismatch {
    pub expected: Unit,
    pub found: Unit,
}

/// `E(P, F, k_max)` with the budget as a ruler: the engine's unshaped
/// `run_bounded`, wrapped. The result names the ruler (`shaped_by`) and,
/// iff the run stopped at its budget, records `Truncated { at }`.
pub fn run_bounded(
    inst: &Instance<'_, PositiveProgram>,
    budget: &Ruler<Count>,
) -> Result<Shaped<Run<Bounded>, Truncated>, UnitMismatch> {
    let count = budget.body();
    if count.unit != Unit::Ticks {
        return Err(UnitMismatch { expected: Unit::Ticks, found: count.unit });
    }
    let k = Budget::new(count.n.get()).expect("NonZeroU64");
    let run = engine::run_bounded(inst, k);
    let erasure = match run.stop {
        Bounded::Budget(b) => Some(Truncated { at: Tick(b.get()) }),
        Bounded::Fixed(_) => None,
    };
    Ok(Shaped {
        value: run,
        shaped_by: NonEmpty::one(budget.addr()),
        observed_by: Vec::new(),
        erasure,
        approval: Approval::Unsigned,
    })
}
