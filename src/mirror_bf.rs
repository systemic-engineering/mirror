//! Mirror-BF: A Brainfuck derivative mapped to the five Prism operations.
//!
//! | Mirror-BF | Brainfuck | Operation           | Prism mapping                  |
//! |-----------|-----------|---------------------|--------------------------------|
//! | `f`       | `>`       | Move right          | **Fold**: advance the read head |
//! | `F`       | `<`       | Move left           | **Fold**: retreat, re-examine   |
//! | `p`       | `+`       | Increment           | **Prism**: sharpen              |
//! | `P`       | `-`       | Decrement           | **Prism**: relax                |
//! | `t`       | `.`       | Output              | **Traversal**: emit             |
//! | `T`       | `,`       | Input               | **Traversal**: receive          |
//! | `l`       | `[`       | Loop start          | **Lens**: begin inspection      |
//! | `L`       | `]`       | Loop end            | **Lens**: close inspection      |
//!
//! **Iso** is the fifth operation, implicit: a program that halts HAS reached its
//! fixed point. The halting state IS the iso. The crystal. You don't write iso —
//! iso is what happens when the program stops.

/// The 8 Mirror-BF instructions, named after Prism operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// `f` — move right (fold forward: advance the read head, accumulate)
    FoldForward,
    /// `F` — move left (fold back: retreat, re-examine)
    FoldBack,
    /// `p` — increment (prism up: increase precision, sharpen)
    PrismUp,
    /// `P` — decrement (prism down: decrease precision, relax)
    PrismDown,
    /// `t` — output (traverse out: emit, visit an external node)
    TraverseOut,
    /// `T` — input (traverse in: receive, accept an external signal)
    TraverseIn,
    /// `l` — loop start (lens open: begin focused inspection)
    LensOpen,
    /// `L` — loop end (lens close: end inspection, put back if nonzero)
    LensClose,
}

/// The operational signature of a Mirror-BF program: counts of each operation class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub fold: usize,
    pub prism: usize,
    pub traversal: usize,
    pub lens: usize,
    pub total: usize,
}

impl Signature {
    /// Compute the operational signature from a slice of instructions.
    pub fn from_instructions(program: &[Instruction]) -> Self {
        let mut fold = 0;
        let mut prism = 0;
        let mut traversal = 0;
        let mut lens = 0;

        for inst in program {
            match inst {
                Instruction::FoldForward | Instruction::FoldBack => fold += 1,
                Instruction::PrismUp | Instruction::PrismDown => prism += 1,
                Instruction::TraverseOut | Instruction::TraverseIn => traversal += 1,
                Instruction::LensOpen | Instruction::LensClose => lens += 1,
            }
        }

        let total = fold + prism + traversal + lens;
        Signature {
            fold,
            prism,
            traversal,
            lens,
            total,
        }
    }

    /// Returns (fold%, prism%, traversal%, lens%) as f64 percentages.
    pub fn percentages(&self) -> (f64, f64, f64, f64) {
        if self.total == 0 {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let t = self.total as f64;
        (
            self.fold as f64 / t * 100.0,
            self.prism as f64 / t * 100.0,
            self.traversal as f64 / t * 100.0,
            self.lens as f64 / t * 100.0,
        )
    }
}

/// Parse a Mirror-BF source string into instructions.
///
/// All characters except `f F p P t T l L` are treated as comments and ignored.
pub fn parse(source: &str) -> Vec<Instruction> {
    source
        .chars()
        .filter_map(|c| match c {
            'f' => Some(Instruction::FoldForward),
            'F' => Some(Instruction::FoldBack),
            'p' => Some(Instruction::PrismUp),
            'P' => Some(Instruction::PrismDown),
            't' => Some(Instruction::TraverseOut),
            'T' => Some(Instruction::TraverseIn),
            'l' => Some(Instruction::LensOpen),
            'L' => Some(Instruction::LensClose),
            _ => None,
        })
        .collect()
}

/// Execute a Mirror-BF program.
///
/// Standard Brainfuck semantics: 30,000-cell tape, wrapping bytes (u8),
/// bracket matching for loops.
///
/// Returns `(output_bytes, final_tape_state)`. The tape is trimmed to the
/// highest cell that was ever written to (or cell 0 if nothing was written).
pub fn execute(program: &[Instruction], input: &[u8]) -> (Vec<u8>, Vec<u8>) {
    const TAPE_SIZE: usize = 30_000;

    let mut tape = vec![0u8; TAPE_SIZE];
    let mut dp: usize = 0; // data pointer
    let mut ip: usize = 0; // instruction pointer
    let mut input_pos: usize = 0;
    let mut output = Vec::new();
    let mut max_dp: usize = 0;

    // Pre-compute bracket matching for O(1) jumps.
    let brackets = match_brackets(program);

    while ip < program.len() {
        match program[ip] {
            Instruction::FoldForward => {
                dp += 1;
                if dp >= TAPE_SIZE {
                    dp = 0; // wrap
                }
                if dp > max_dp {
                    max_dp = dp;
                }
            }
            Instruction::FoldBack => {
                if dp == 0 {
                    dp = TAPE_SIZE - 1; // wrap
                } else {
                    dp -= 1;
                }
            }
            Instruction::PrismUp => {
                tape[dp] = tape[dp].wrapping_add(1);
            }
            Instruction::PrismDown => {
                tape[dp] = tape[dp].wrapping_sub(1);
            }
            Instruction::TraverseOut => {
                output.push(tape[dp]);
            }
            Instruction::TraverseIn => {
                if input_pos < input.len() {
                    tape[dp] = input[input_pos];
                    input_pos += 1;
                } else {
                    tape[dp] = 0; // EOF = 0
                }
            }
            Instruction::LensOpen => {
                if tape[dp] == 0 {
                    ip = brackets[ip];
                }
            }
            Instruction::LensClose => {
                if tape[dp] != 0 {
                    ip = brackets[ip];
                }
            }
        }
        ip += 1;
    }

    // Trim tape to highest used cell.
    let tape_end = max_dp + 1;
    tape.truncate(tape_end);

    (output, tape)
}

/// Pre-compute bracket matching: for each `[` store index of matching `]` and vice versa.
fn match_brackets(program: &[Instruction]) -> Vec<usize> {
    let mut brackets = vec![0usize; program.len()];
    let mut stack = Vec::new();

    for (i, inst) in program.iter().enumerate() {
        match inst {
            Instruction::LensOpen => {
                stack.push(i);
            }
            Instruction::LensClose => {
                if let Some(open) = stack.pop() {
                    brackets[open] = i;
                    brackets[i] = open;
                }
            }
            _ => {}
        }
    }

    brackets
}

/// Translate Mirror-BF source to standard Brainfuck.
pub fn to_brainfuck(source: &str) -> String {
    source
        .chars()
        .filter_map(|c| match c {
            'f' => Some('>'),
            'F' => Some('<'),
            'p' => Some('+'),
            'P' => Some('-'),
            't' => Some('.'),
            'T' => Some(','),
            'l' => Some('['),
            'L' => Some(']'),
            _ => None,
        })
        .collect()
}

/// Translate standard Brainfuck source to Mirror-BF.
pub fn from_brainfuck(source: &str) -> String {
    source
        .chars()
        .filter_map(|c| match c {
            '>' => Some('f'),
            '<' => Some('F'),
            '+' => Some('p'),
            '-' => Some('P'),
            '.' => Some('t'),
            ',' => Some('T'),
            '[' => Some('l'),
            ']' => Some('L'),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- (a) Hello World in Mirror-BF ----

    #[test]
    fn hello_world_mirror_bf() {
        // Classic BF hello world:
        let bf_hello = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let mirror_hello = from_brainfuck(bf_hello);
        let program = parse(&mirror_hello);
        let (output, _tape) = execute(&program, &[]);
        assert_eq!(String::from_utf8(output).unwrap(), "Hello World!\n");
    }

    // ---- (b) Round-trip: BF → Mirror-BF → BF preserves semantics ----

    #[test]
    fn round_trip_preserves_semantics() {
        let original_bf = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

        // BF → Mirror-BF → BF
        let mirror = from_brainfuck(original_bf);
        let back_to_bf = to_brainfuck(&mirror);

        // The round-tripped BF should be the same (comments stripped).
        let stripped_original: String = original_bf
            .chars()
            .filter(|c| "><+-.,[]".contains(*c))
            .collect();
        assert_eq!(back_to_bf, stripped_original);

        // And both produce the same output.
        let program_original = parse_bf(original_bf);
        let program_mirror = parse(&mirror);

        let (out1, tape1) = execute(&program_original, &[]);
        let (out2, tape2) = execute(&program_mirror, &[]);

        assert_eq!(out1, out2);
        assert_eq!(tape1, tape2);
    }

    /// Helper: parse standard BF by converting to Mirror-BF first.
    fn parse_bf(source: &str) -> Vec<Instruction> {
        parse(&from_brainfuck(source))
    }

    // ---- (c) Fate translation produces same output ----

    #[test]
    fn fate_bf_translated_to_mirror_bf_same_output() {
        // fate.bf: reads 17 input bytes, outputs 1 byte (winning model index).
        let fate_bf = include_str!("../fixtures/fate.bf");
        let fate_mirror = from_brainfuck(fate_bf);
        let fate_mirror_parsed = parse(&fate_mirror);
        let fate_bf_parsed = parse_bf(fate_bf);

        // Test with model=0, all features=0 → should select Cartographer (1) due to bias.
        let input_case_0: Vec<u8> = vec![0; 16].into_iter().chain(std::iter::once(0)).collect();
        let (out_bf, tape_bf) = execute(&fate_bf_parsed, &input_case_0);
        let (out_mirror, tape_mirror) = execute(&fate_mirror_parsed, &input_case_0);
        assert_eq!(out_bf, out_mirror, "case 0: outputs differ");
        assert_eq!(tape_bf, tape_mirror, "case 0: tapes differ");

        // Test with model=2, feature[0]=5 → Pathfinder gets bias(10)+feature(5)=15.
        let mut input_case_2 = vec![0u8; 17];
        input_case_2[0] = 5; // feature 0
        input_case_2[16] = 2; // model index
        let (out_bf2, tape_bf2) = execute(&fate_bf_parsed, &input_case_2);
        let (out_mirror2, tape_mirror2) = execute(&fate_mirror_parsed, &input_case_2);
        assert_eq!(out_bf2, out_mirror2, "case 2: outputs differ");
        assert_eq!(tape_bf2, tape_mirror2, "case 2: tapes differ");

        // Test with model=4 → Fate biases Abyss(0), but feature 0 can tip Pathfinder.
        let mut input_case_4 = vec![0u8; 17];
        input_case_4[16] = 4; // model index = Fate
        let (out_bf4, tape_bf4) = execute(&fate_bf_parsed, &input_case_4);
        let (out_mirror4, tape_mirror4) = execute(&fate_mirror_parsed, &input_case_4);
        assert_eq!(out_bf4, out_mirror4, "case 4: outputs differ");
        assert_eq!(tape_bf4, tape_mirror4, "case 4: tapes differ");
    }

    // ---- (d) The mapping is semantic ----

    #[test]
    fn fold_instructions_only_move_head() {
        // `f` and `F` should only change the data pointer, not the cell values.
        let program = parse("fffFFF");
        let (output, tape) = execute(&program, &[]);
        assert!(
            output.is_empty(),
            "fold instructions should not produce output"
        );
        // Tape should be all zeros — fold never modifies values.
        assert!(
            tape.iter().all(|&b| b == 0),
            "fold should not change cell values"
        );
    }

    #[test]
    fn prism_instructions_only_change_values() {
        // `p` and `P` should only change the current cell, not move the pointer.
        let program = parse("pppPP"); // net +1 at cell 0
        let (output, tape) = execute(&program, &[]);
        assert!(output.is_empty(), "prism should not produce output");
        assert_eq!(tape, vec![1], "prism should only change current cell value");
    }

    #[test]
    fn traversal_instructions_only_do_io() {
        // `t` outputs the current cell. `T` reads input.
        let program_out =
            parse("pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppt"); // 65 increments then output = 'A'
        let (output, _tape) = execute(&program_out, &[]);
        assert_eq!(output, vec![65], "traverse out should emit current cell");

        let program_in = parse("Tt"); // read one byte, output it
        let (output2, _tape2) = execute(&program_in, &[42]);
        assert_eq!(output2, vec![42], "traverse in should receive input byte");
    }

    #[test]
    fn lens_instructions_only_control_flow() {
        // `l` and `L` implement loops. They don't move the pointer or change values directly.
        // A loop on a zero cell skips entirely.
        let program = parse("lppppppppppL"); // cell is 0, so [++++++++++] should skip
        let (output, tape) = execute(&program, &[]);
        assert!(output.is_empty());
        assert_eq!(
            tape,
            vec![0],
            "lens on zero cell should skip loop body entirely"
        );

        // A loop that decrements to zero.
        let program2 = parse("pppppplPLt"); // cell=6, then loop: decrement until 0, then output 0
        let (output2, tape2) = execute(&program2, &[]);
        assert_eq!(tape2, vec![0], "lens should loop until cell is zero");
        assert_eq!(output2, vec![0]);
    }

    // ---- (e) Iso is halting ----

    #[test]
    fn iso_is_halting_decrement_until_zero() {
        // A convergent program: set cell to 5, then decrement until zero.
        // The program halts — the halting state IS the iso, the crystal.
        let program = parse("ppppp lPL"); // cell=5, loop: decrement
        let (output, tape) = execute(&program, &[]);
        assert!(output.is_empty());
        // The tape has crystallized: the fixed point is zero.
        assert_eq!(
            tape[0], 0,
            "program reached its fixed point (iso): all motion resolved to zero"
        );
    }

    #[test]
    fn iso_crystallized_tape_state() {
        // A more complex convergent program: two cells, transfer value from cell 0 to cell 1.
        // cell0=3, loop: dec cell0, inc cell1
        let program = parse("ppp l P f p F L"); // cell0=3, [dec0 right inc1 left]
        let (output, tape) = execute(&program, &[]);
        assert!(output.is_empty());
        assert_eq!(tape[0], 0, "source cell drained");
        assert_eq!(tape[1], 3, "destination cell received");
        // The crystal: the value has been conserved through transformation.
        // Iso = the program halted. The tape is the fixed point.
    }

    // ---- Operational signature ----

    #[test]
    fn operational_signature_counts() {
        let program = parse("ffFppPtTlL");
        let sig = Signature::from_instructions(&program);
        assert_eq!(sig.fold, 3); // ff F
        assert_eq!(sig.prism, 3); // pp P
        assert_eq!(sig.traversal, 2); // t T
        assert_eq!(sig.lens, 2); // l L
        assert_eq!(sig.total, 10);
    }

    #[test]
    fn empty_program_signature() {
        let program = parse("");
        let sig = Signature::from_instructions(&program);
        assert_eq!(sig.total, 0);
        let (f, p, t, l) = sig.percentages();
        assert_eq!(f, 0.0);
        assert_eq!(p, 0.0);
        assert_eq!(t, 0.0);
        assert_eq!(l, 0.0);
    }
}
