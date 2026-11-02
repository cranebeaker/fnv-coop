//! Cross-check the length decoder against a real disassembler.
//!
//! usage: decode_check <code.bin> <boundaries.txt>
//!
//! `code.bin` is raw code bytes. `boundaries.txt` has one instruction start
//! offset per line (from `objdump`), ending with the end of the code. Every
//! instruction the decoder accepts must have exactly the length the
//! disassembler found. Instructions outside the supported subset are counted,
//! not compared, and a window that is only zero padding after the decoded
//! instruction is skipped (disassemblers split padding differently).
//!
//! Exits with status 1 on any disagreement.

use nv_oracle_core::decode::{decode, DecodeError, Kind};
use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: decode_check <code.bin> <boundaries.txt>");
        std::process::exit(2);
    }
    let bytes = std::fs::read(&args[1]).expect("read code");
    let bounds: Vec<usize> = std::fs::read_to_string(&args[2])
        .expect("read boundaries")
        .lines()
        .map(|l| l.trim().parse().expect("offset"))
        .collect();
    let (mut agree, mut relative, mut padding, mut mismatches) = (0usize, 0usize, 0usize, 0usize);
    let mut unsupported: BTreeMap<String, usize> = BTreeMap::new();
    for w in bounds.windows(2) {
        let (a, b) = (w[0], w[1]);
        match decode(&bytes[a..(a + 16).min(bytes.len())]) {
            Ok(i) if i.len == b - a => {
                agree += 1;
                if matches!(i.kind, Kind::Rel32 { .. }) {
                    relative += 1;
                }
            }
            Ok(i) if i.len < b - a && bytes[a + i.len..b].iter().all(|&x| x == 0) => padding += 1,
            Ok(i) => {
                mismatches += 1;
                if mismatches <= 20 {
                    println!(
                        "MISMATCH at {a:#x}: decoder {} disassembler {}: {:02x?}",
                        i.len,
                        b - a,
                        &bytes[a..b.min(a + 16)]
                    );
                }
            }
            Err(DecodeError::Unsupported { opcode }) => {
                *unsupported.entry(format!("{opcode:#06x}")).or_default() += 1
            }
            Err(DecodeError::ShortBranch { .. }) => {
                *unsupported.entry("short branch".into()).or_default() += 1
            }
            Err(e) => *unsupported.entry(format!("{e}")).or_default() += 1,
        }
    }
    let skipped: usize = unsupported.values().sum();
    println!(
        "{} instructions: {agree} agree ({relative} relocatable), {skipped} outside the subset, {padding} padding windows skipped, {mismatches} MISMATCHES",
        bounds.len().saturating_sub(1)
    );
    if mismatches > 0 {
        std::process::exit(1);
    }
}
