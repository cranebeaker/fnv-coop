//! A length decoder for the x86-32 instructions found at the start of
//! functions, plus trampoline building for entry hooks.
//!
//! This is deliberately a small subset, not a general disassembler. To hook
//! a function the probe must copy its first few instructions elsewhere and
//! run them from there, so it needs to know where instructions end and which
//! ones depend on their own address. Anything outside the subset is an
//! error, never a guess:
//!
//! - one-byte opcodes: `push`/`pop`/`inc`/`dec` of a register, `push imm`,
//!   the ALU group (`add or adc sbb and sub xor cmp`, all operand forms),
//!   `test`, `xchg`, `mov` (ModRM forms, `moffs`, `B0..BF`, `C6/C7`),
//!   `lea`, `imul` with immediate, shifts, `nop`, `cwde`, `cdq`, group 3
//!   (`F6/F7`), `inc/dec/call/jmp/push` through `FE/FF`, the x87 opcodes
//!   `D8..DF` with any ModRM, and `E8`/`E9` with a 32-bit displacement;
//! - two-byte `0F` opcodes: `jcc rel32`, `setcc`, `cmovcc`, `imul`,
//!   `movzx`, `movsx`, multi-byte `nop` and the common SSE/SSE2 moves and
//!   arithmetic (`10/11 12..17 28..2F 51..5F 6E/6F 7E/7F D6 EF`, and
//!   `70 C2 C6` which carry an immediate);
//! - prefixes `64`/`65` (segment), `66` (operand size), `F2`/`F3` (SSE
//!   only), `F0` and the unused segment overrides.
//!
//! Relative branches: `E8`, `E9` and `0F 8x` with a 32-bit displacement can
//! be relocated and are flagged as [`Kind::Rel32`]. Short branches (`EB`,
//! `70..7F`, `E0..E3`) cannot be moved without rewriting them, so they are
//! rejected with [`DecodeError::ShortBranch`].

use std::fmt;

/// Bytes a `jmp rel32` occupies; the minimum number of bytes to steal.
pub const JMP_LEN: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Position independent: can be copied byte for byte.
    Plain,
    /// Ends in a 32-bit displacement relative to the next instruction.
    /// `field_offset` is where that displacement starts in the instruction.
    Rel32 { field_offset: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Insn {
    pub len: usize,
    pub kind: Kind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// The bytes end in the middle of an instruction.
    Truncated,
    /// A branch with an 8-bit displacement.
    ShortBranch {
        opcode: u8,
    },
    /// An opcode outside the supported subset. Two-byte opcodes are
    /// reported as `0x0F00 | second_byte`.
    Unsupported {
        opcode: u16,
    },
    TooManyPrefixes,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::Truncated => write!(f, "bytes end inside an instruction"),
            DecodeError::ShortBranch { opcode } => {
                write!(f, "short branch (opcode {opcode:#04x}) cannot be relocated")
            }
            DecodeError::Unsupported { opcode } => {
                write!(f, "opcode {opcode:#06x} is not in the supported subset")
            }
            DecodeError::TooManyPrefixes => write!(f, "too many prefixes"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Length of the ModRM byte, optional SIB and displacement.
fn modrm_len(b: &[u8], at: usize) -> Result<usize, DecodeError> {
    let m = *b.get(at).ok_or(DecodeError::Truncated)?;
    let (md, rm) = (m >> 6, m & 7);
    if md == 3 {
        return Ok(1);
    }
    let mut len = 1;
    let mut sib_base_is_5 = false;
    if rm == 4 {
        let sib = *b.get(at + 1).ok_or(DecodeError::Truncated)?;
        len += 1;
        sib_base_is_5 = sib & 7 == 5;
    }
    match md {
        0 => {
            if rm == 5 || (rm == 4 && sib_base_is_5) {
                len += 4;
            }
        }
        1 => len += 1,
        _ => len += 4,
    }
    Ok(len)
}

/// Decode one instruction at the start of `b`.
pub fn decode(b: &[u8]) -> Result<Insn, DecodeError> {
    let mut p = 0usize;
    let mut opsize16 = false;
    let mut rep = false;
    let mut prefixes = 0;
    let op = loop {
        let c = *b.get(p).ok_or(DecodeError::Truncated)?;
        match c {
            0x26 | 0x2e | 0x36 | 0x3e | 0x64 | 0x65 | 0xf0 => {}
            0x66 => opsize16 = true,
            0xf2 | 0xf3 => rep = true,
            0x67 => return Err(DecodeError::Unsupported { opcode: 0x67 }),
            _ => break c,
        }
        p += 1;
        prefixes += 1;
        if prefixes > 4 {
            return Err(DecodeError::TooManyPrefixes);
        }
    };
    p += 1;
    let imm32 = if opsize16 { 2 } else { 4 };
    let unsupported = |opcode: u16| Err(DecodeError::Unsupported { opcode });
    // `p` is now at the byte after the opcode, which is the ModRM byte for
    // every opcode below that has one (two-byte opcodes advance it first).
    let reg_at = |at: usize| b.get(at).map(|m| (m >> 3) & 7);
    let mut sse = false;

    // (has ModRM, immediate bytes)
    let (has_modrm, imm) = match op {
        0x0f => {
            let op2 = *b.get(p).ok_or(DecodeError::Truncated)?;
            p += 1;
            match op2 {
                0x80..=0x8f => {
                    if opsize16 || rep {
                        return unsupported(0x0f00 | u16::from(op2));
                    }
                    if b.len() < p + 4 {
                        return Err(DecodeError::Truncated);
                    }
                    return Ok(Insn {
                        len: p + 4,
                        kind: Kind::Rel32 { field_offset: p },
                    });
                }
                0x40..=0x4f | 0x90..=0x9f | 0xaf | 0xb6 | 0xb7 | 0xbe | 0xbf | 0x1f => (true, 0),
                0x10..=0x17
                | 0x28..=0x2f
                | 0x51..=0x5f
                | 0x6e
                | 0x6f
                | 0x7e
                | 0x7f
                | 0xd6
                | 0xef => {
                    sse = true;
                    (true, 0)
                }
                0x70 | 0xc2 | 0xc6 => {
                    sse = true;
                    (true, 1)
                }
                _ => return unsupported(0x0f00 | u16::from(op2)),
            }
        }
        0x00..=0x3f if op & 7 < 4 => (true, 0),
        0x00..=0x3f if op & 7 == 4 => (false, 1),
        0x00..=0x3f if op & 7 == 5 => (false, imm32),
        0x40..=0x5f | 0x90 | 0x98 | 0x99 => (false, 0),
        0x68 => (false, imm32),
        0x6a => (false, 1),
        0x69 => (true, imm32),
        0x6b => (true, 1),
        0x70..=0x7f | 0xe0..=0xe3 | 0xeb => return Err(DecodeError::ShortBranch { opcode: op }),
        0x80 | 0x83 => (true, 1),
        0x81 => (true, imm32),
        0x84..=0x8b => (true, 0),
        0x8d => {
            // lea with a register operand is invalid.
            if b.get(p).is_some_and(|m| m >> 6 == 3) {
                return unsupported(u16::from(op));
            }
            (true, 0)
        }
        0xa0..=0xa3 => (false, 4),
        0xa8 => (false, 1),
        0xa9 => (false, imm32),
        0xb0..=0xb7 => (false, 1),
        0xb8..=0xbf => (false, imm32),
        0xc0 | 0xc1 => (true, 1),
        0xc6 | 0xc7 => match reg_at(p) {
            Some(0) => (true, if op == 0xc6 { 1 } else { imm32 }),
            Some(_) => return unsupported(u16::from(op)),
            None => return Err(DecodeError::Truncated),
        },
        0xd0..=0xd3 | 0xd8..=0xdf => (true, 0),
        0xe8 | 0xe9 => {
            if opsize16 || rep {
                return unsupported(u16::from(op));
            }
            if b.len() < p + 4 {
                return Err(DecodeError::Truncated);
            }
            return Ok(Insn {
                len: p + 4,
                kind: Kind::Rel32 { field_offset: p },
            });
        }
        0xf6 | 0xf7 => match reg_at(p) {
            Some(r) => (
                true,
                if r < 2 {
                    if op == 0xf6 {
                        1
                    } else {
                        imm32
                    }
                } else {
                    0
                },
            ),
            None => return Err(DecodeError::Truncated),
        },
        0xfe => match reg_at(p) {
            Some(0 | 1) => (true, 0),
            Some(_) => return unsupported(u16::from(op)),
            None => return Err(DecodeError::Truncated),
        },
        0xff => match reg_at(p) {
            Some(0 | 1 | 2 | 4 | 6) => (true, 0),
            Some(_) => return unsupported(u16::from(op)),
            None => return Err(DecodeError::Truncated),
        },
        _ => return unsupported(u16::from(op)),
    };

    if rep && !sse {
        return unsupported(0xf3);
    }
    let mut len = p;
    if has_modrm {
        len += modrm_len(b, p)?;
    }
    len += imm;
    if len > b.len() {
        return Err(DecodeError::Truncated);
    }
    Ok(Insn {
        len,
        kind: Kind::Plain,
    })
}

/// A decode failure at a known offset inside a run of instructions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunError {
    pub offset: usize,
    pub cause: DecodeError,
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at byte offset {}: {}", self.offset, self.cause)
    }
}

impl std::error::Error for RunError {}

/// Decode instructions from the start of `bytes` until at least `min_len`
/// bytes are covered. Returns each instruction with its start offset.
pub fn decode_run(bytes: &[u8], min_len: usize) -> Result<Vec<(usize, Insn)>, RunError> {
    let mut out = Vec::new();
    let mut offset = 0;
    while offset < min_len {
        let insn = decode(&bytes[offset.min(bytes.len())..])
            .map_err(|cause| RunError { offset, cause })?;
        out.push((offset, insn));
        offset += insn.len;
    }
    Ok(out)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrampolineError {
    /// Fewer than [`JMP_LEN`] bytes were requested.
    TooShort {
        steal: usize,
    },
    Decode(RunError),
    /// The requested count splits an instruction. `before` and `after` are
    /// the nearest instruction boundaries on either side.
    NotOnBoundary {
        steal: usize,
        before: usize,
        after: usize,
    },
    /// A relocated branch targets the middle of the bytes being replaced.
    BranchIntoStolen {
        insn_offset: usize,
        target: u32,
    },
}

impl fmt::Display for TrampolineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrampolineError::TooShort { steal } => {
                write!(f, "stealing {steal} bytes cannot hold a {JMP_LEN}-byte jump")
            }
            TrampolineError::Decode(e) => write!(f, "cannot decode the function start: {e}"),
            TrampolineError::NotOnBoundary { steal, before, after } => write!(
                f,
                "stolen byte count {steal} is not an instruction boundary (nearest are {before} and {after})"
            ),
            TrampolineError::BranchIntoStolen { insn_offset, target } => write!(
                f,
                "the branch at offset {insn_offset} targets {target:#010x}, inside the bytes being replaced"
            ),
        }
    }
}

impl std::error::Error for TrampolineError {}

/// Smallest instruction boundary of `code` that is at least [`JMP_LEN`].
pub fn auto_steal(code: &[u8]) -> Result<usize, TrampolineError> {
    let run = decode_run(code, JMP_LEN).map_err(TrampolineError::Decode)?;
    let (off, last) = run.last().expect("at least one instruction");
    Ok(off + last.len)
}

/// Build the code that replaces the first `steal` bytes of a function after
/// they are overwritten by a `jmp rel32`: the original instructions (with
/// `E8`/`E9`/`0F 8x` displacements adjusted for their new address) followed
/// by a `jmp` back to the rest of the function.
///
/// `code` holds the function's first bytes (at least `steal`; more lets an
/// instruction that straddles the boundary be recognised). `src` is the
/// function address and `dst` the address the returned bytes will live at.
pub fn build_trampoline(
    code: &[u8],
    steal: usize,
    src: u32,
    dst: u32,
) -> Result<Vec<u8>, TrampolineError> {
    if steal < JMP_LEN {
        return Err(TrampolineError::TooShort { steal });
    }
    let run = decode_run(code, steal).map_err(TrampolineError::Decode)?;
    let end = run.last().map(|(o, i)| o + i.len).unwrap_or(0);
    if end != steal {
        let before = run.last().map(|(o, _)| *o).unwrap_or(0);
        return Err(TrampolineError::NotOnBoundary {
            steal,
            before,
            after: end,
        });
    }
    let mut out = Vec::with_capacity(steal + JMP_LEN);
    for (off, insn) in &run {
        let mut bytes = code[*off..*off + insn.len].to_vec();
        if let Kind::Rel32 { field_offset } = insn.kind {
            let rel = i32::from_le_bytes(
                bytes[field_offset..field_offset + 4]
                    .try_into()
                    .expect("4 bytes"),
            );
            let next = (*off + insn.len) as u32;
            let target = src.wrapping_add(next).wrapping_add(rel as u32);
            let inside = target.wrapping_sub(src);
            if inside >= 1 && (inside as usize) < steal {
                return Err(TrampolineError::BranchIntoStolen {
                    insn_offset: *off,
                    target,
                });
            }
            let new_rel = target.wrapping_sub(dst.wrapping_add(out.len() as u32 + insn.len as u32));
            bytes[field_offset..field_offset + 4].copy_from_slice(&new_rel.to_le_bytes());
        }
        out.extend_from_slice(&bytes);
    }
    let back = src.wrapping_add(steal as u32);
    let rel = back.wrapping_sub(dst.wrapping_add(out.len() as u32 + JMP_LEN as u32));
    out.push(0xe9);
    out.extend_from_slice(&rel.to_le_bytes());
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn len(bytes: &[u8]) -> usize {
        decode(bytes)
            .unwrap_or_else(|e| panic!("{bytes:02x?}: {e}"))
            .len
    }

    #[test]
    fn push_pop_and_frame_setup() {
        assert_eq!(len(&[0x55]), 1); // push ebp
        assert_eq!(len(&[0x8b, 0xec]), 2); // mov ebp, esp
        assert_eq!(len(&[0x56, 0x8b, 0xf1]), 1); // push esi
        assert_eq!(len(&[0x8b, 0xf1]), 2); // mov esi, ecx
        assert_eq!(len(&[0x5d]), 1); // pop ebp
        assert_eq!(len(&[0x6a, 0xff]), 2); // push -1
        assert_eq!(len(&[0x68, 0x78, 0x56, 0x34, 0x12]), 5); // push imm32
        assert_eq!(len(&[0x66, 0x68, 0x34, 0x12]), 4); // push imm16
    }

    #[test]
    fn msvc_sehs_prologue() {
        // push -1; push handler; mov eax, fs:[0]; push eax; ...
        let code = [
            0x6a, 0xff, 0x68, 0xa0, 0x11, 0x22, 0x00, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x50,
            0x83, 0xec, 0x10, 0x53, 0x56, 0x57, 0xa1, 0x30, 0x40, 0x50, 0x00, 0x33, 0xc5, 0x50,
        ];
        let lens: Vec<usize> = decode_run(&code, code.len())
            .unwrap()
            .iter()
            .map(|(_, i)| i.len)
            .collect();
        assert_eq!(lens, [2, 5, 6, 1, 3, 1, 1, 1, 5, 2, 1]);
    }

    #[test]
    fn aligned_stack_prologue() {
        // push ebp; mov ebp, esp; and esp, -16; sub esp, 0x18
        let code = [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf0, 0x83, 0xec, 0x18];
        let lens: Vec<usize> = decode_run(&code, 9)
            .unwrap()
            .iter()
            .map(|(_, i)| i.len)
            .collect();
        assert_eq!(lens, [1, 2, 3, 3]);
        assert_eq!(len(&[0x83, 0xec, 0x10]), 3); // sub esp, 0x10
        assert_eq!(len(&[0x81, 0xec, 0x00, 0x01, 0x00, 0x00]), 6); // sub esp, 0x100
    }

    #[test]
    fn hot_patch_and_frame_pointer_gcc() {
        assert_eq!(len(&[0x8b, 0xff]), 2); // mov edi, edi
        assert_eq!(len(&[0x55, 0x89, 0xe5, 0x83, 0xec, 0x18]), 1);
        assert_eq!(len(&[0x89, 0xe5]), 2); // mov ebp, esp (gcc form)
        assert_eq!(len(&[0x8b, 0x45, 0x08]), 3); // mov eax, [ebp+8]
        assert_eq!(len(&[0x89, 0x4d, 0xfc]), 3); // mov [ebp-4], ecx
        assert_eq!(len(&[0xc7, 0x45, 0xfc, 0x00, 0x00, 0x00, 0x00]), 7); // mov dword [ebp-4], 0
        assert_eq!(len(&[0x66, 0xc7, 0x45, 0xfc, 0x01, 0x00]), 6); // mov word [ebp-4], 1
        assert_eq!(len(&[0xc6, 0x45, 0xff, 0x01]), 4); // mov byte [ebp-1], 1
    }

    #[test]
    fn modrm_sib_and_displacements() {
        assert_eq!(len(&[0x8b, 0x44, 0x24, 0x04]), 4); // mov eax, [esp+4]
        assert_eq!(len(&[0x8b, 0x84, 0x24, 0x00, 0x01, 0x00, 0x00]), 7); // [esp+0x100]
        assert_eq!(len(&[0x8b, 0x04, 0x85, 0x00, 0x10, 0x40, 0x00]), 7); // [eax*4+disp32]
        assert_eq!(len(&[0x8b, 0x05, 0x00, 0x10, 0x40, 0x00]), 6); // [disp32]
        assert_eq!(len(&[0x8b, 0x45, 0x00]), 3); // [ebp+0]
        assert_eq!(len(&[0x8b, 0x04, 0x24]), 3); // [esp]
        assert_eq!(len(&[0x8b, 0x4c, 0x91, 0x08]), 4); // [ecx+edx*4+8]
        assert_eq!(len(&[0x8d, 0x85, 0x00, 0xff, 0xff, 0xff]), 6); // lea eax, [ebp-0x100]
        assert_eq!(len(&[0x8d, 0x44, 0x24, 0x10]), 4); // lea eax, [esp+0x10]
        assert_eq!(len(&[0x8b, 0x81, 0x70, 0x01, 0x00, 0x00]), 6); // mov eax, [ecx+0x170]
        assert_eq!(len(&[0x8b, 0x04, 0x25, 0x00, 0x10, 0x40, 0x00]), 7); // SIB, no base, no index
        assert_eq!(len(&[0x8b, 0x14, 0x2d, 0x00, 0x00, 0x00, 0x00]), 7);
        assert!(decode(&[0x8d, 0xc0]).is_err()); // lea with register operand
    }

    #[test]
    fn alu_forms() {
        assert_eq!(len(&[0x01, 0xd8]), 2); // add eax, ebx
        assert_eq!(len(&[0x2b, 0x45, 0x08]), 3); // sub eax, [ebp+8]
        assert_eq!(len(&[0x33, 0xc0]), 2); // xor eax, eax
        assert_eq!(len(&[0x3b, 0xc1]), 2); // cmp eax, ecx
        assert_eq!(len(&[0x85, 0xc0]), 2); // test eax, eax
        assert_eq!(len(&[0x84, 0xc0]), 2); // test al, al
        assert_eq!(len(&[0x05, 0x01, 0x00, 0x00, 0x00]), 5); // add eax, 1
        assert_eq!(len(&[0x3c, 0x20]), 2); // cmp al, 0x20
        assert_eq!(len(&[0x66, 0x05, 0x01, 0x00]), 4); // add ax, 1
        assert_eq!(len(&[0x83, 0x7d, 0x08, 0x00]), 4); // cmp dword [ebp+8], 0
        assert_eq!(len(&[0x80, 0x7d, 0x08, 0x00]), 4); // cmp byte [ebp+8], 0
        assert_eq!(len(&[0x81, 0xf9, 0x00, 0x01, 0x00, 0x00]), 6); // cmp ecx, 0x100
        assert_eq!(len(&[0xf7, 0xc1, 0x01, 0x00, 0x00, 0x00]), 6); // test ecx, 1
        assert_eq!(len(&[0xf6, 0xc1, 0x01]), 3); // test cl, 1
        assert_eq!(len(&[0xf7, 0xd8]), 2); // neg eax
        assert_eq!(len(&[0xf7, 0xf9]), 2); // idiv ecx
        assert_eq!(len(&[0xc1, 0xe0, 0x02]), 3); // shl eax, 2
        assert_eq!(len(&[0xd1, 0xf8]), 2); // sar eax, 1
        assert_eq!(len(&[0x6b, 0xc0, 0x0c]), 3); // imul eax, eax, 12
        assert_eq!(len(&[0x69, 0xc0, 0x00, 0x01, 0x00, 0x00]), 6);
        assert_eq!(len(&[0x0f, 0xaf, 0xc1]), 3); // imul eax, ecx
        assert_eq!(len(&[0x0f, 0xb6, 0x45, 0x08]), 4); // movzx eax, byte [ebp+8]
        assert_eq!(len(&[0x0f, 0xbf, 0xc0]), 3);
        assert_eq!(len(&[0x0f, 0x94, 0xc0]), 3); // sete al
        assert_eq!(len(&[0x0f, 0x44, 0xc1]), 3); // cmove eax, ecx
        assert_eq!(len(&[0x40]), 1);
        assert_eq!(len(&[0x4f]), 1);
        assert_eq!(len(&[0x99]), 1);
        assert_eq!(len(&[0x90]), 1);
        assert_eq!(len(&[0x0f, 0x1f, 0x44, 0x00, 0x00]), 5); // nop dword [eax+eax]
        assert_eq!(len(&[0xb8, 0x01, 0x00, 0x00, 0x00]), 5); // mov eax, 1
        assert_eq!(len(&[0xb1, 0x01]), 2); // mov cl, 1
        assert_eq!(len(&[0xa1, 0x00, 0x10, 0x40, 0x00]), 5); // mov eax, [moffs]
        assert_eq!(len(&[0xa3, 0x00, 0x10, 0x40, 0x00]), 5);
        assert_eq!(len(&[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]), 6); // mov eax, fs:[0]
        assert_eq!(len(&[0x64, 0x89, 0x25, 0x00, 0x00, 0x00, 0x00]), 7); // mov fs:[0], esp
    }

    #[test]
    fn indirect_calls_and_pushes_through_ff() {
        assert_eq!(len(&[0xff, 0x35, 0x00, 0x10, 0x40, 0x00]), 6); // push dword [abs]
        assert_eq!(len(&[0xff, 0x15, 0x00, 0x10, 0x40, 0x00]), 6); // call [abs]
        assert_eq!(len(&[0xff, 0x25, 0x00, 0x10, 0x40, 0x00]), 6); // jmp [abs]
        assert_eq!(len(&[0xff, 0xd1]), 2); // call ecx
        assert_eq!(len(&[0xff, 0x30]), 2); // push dword [eax]
        assert_eq!(len(&[0xff, 0x01]), 2); // inc dword [ecx]
        assert!(decode(&[0xff, 0x18]).is_err()); // far call
        assert!(decode(&[0xff, 0x28]).is_err()); // far jmp
        assert!(decode(&[0xff, 0x38]).is_err()); // reserved
        assert_eq!(len(&[0xfe, 0x01]), 2);
    }

    #[test]
    fn x87_with_modrm() {
        assert_eq!(len(&[0xd9, 0x44, 0x24, 0x04]), 4); // fld dword [esp+4]
        assert_eq!(len(&[0xdd, 0x44, 0x24, 0x04]), 4); // fld qword [esp+4]
        assert_eq!(len(&[0xd9, 0x45, 0x08]), 3); // fld dword [ebp+8]
        assert_eq!(len(&[0xdd, 0x45, 0x08]), 3);
        assert_eq!(len(&[0xd9, 0x5d, 0xfc]), 3); // fstp dword [ebp-4]
        assert_eq!(len(&[0xdd, 0x5d, 0xf8]), 3); // fstp qword [ebp-8]
        assert_eq!(len(&[0xd9, 0x05, 0x00, 0x10, 0x40, 0x00]), 6); // fld dword [abs]
        assert_eq!(len(&[0xdb, 0x2d, 0x00, 0x10, 0x40, 0x00]), 6); // fld tbyte [abs]
        assert_eq!(len(&[0xd9, 0xe8]), 2); // fld1
        assert_eq!(len(&[0xd9, 0xee]), 2); // fldz
        assert_eq!(len(&[0xd9, 0xfa]), 2); // fsqrt
        assert_eq!(len(&[0xd8, 0x4c, 0x24, 0x08]), 4); // fmul dword [esp+8]
        assert_eq!(len(&[0xdf, 0x6c, 0x24, 0x04]), 4); // fild qword [esp+4]
        assert_eq!(len(&[0xd9, 0x6c, 0x24, 0x04]), 4); // fldcw [esp+4]
    }

    #[test]
    fn sse_with_prefixes() {
        assert_eq!(len(&[0xf3, 0x0f, 0x10, 0x44, 0x24, 0x04]), 6); // movss xmm0, [esp+4]
        assert_eq!(len(&[0xf3, 0x0f, 0x11, 0x45, 0xfc]), 5); // movss [ebp-4], xmm0
        assert_eq!(len(&[0xf2, 0x0f, 0x10, 0x44, 0x24, 0x04]), 6); // movsd
        assert_eq!(len(&[0x0f, 0x28, 0xc1]), 3); // movaps xmm0, xmm1
        assert_eq!(len(&[0x0f, 0x29, 0x45, 0xe0]), 4); // movaps [ebp-0x20], xmm0
        assert_eq!(len(&[0x66, 0x0f, 0x28, 0xc1]), 4); // movapd
        assert_eq!(len(&[0xf3, 0x0f, 0x58, 0xc1]), 4); // addss
        assert_eq!(len(&[0xf3, 0x0f, 0x59, 0x41, 0x04]), 5); // mulss xmm0, [ecx+4]
        assert_eq!(len(&[0xf3, 0x0f, 0x5c, 0xc1]), 4); // subss
        assert_eq!(len(&[0xf3, 0x0f, 0x5e, 0xc1]), 4); // divss
        assert_eq!(len(&[0xf3, 0x0f, 0x51, 0xc0]), 4); // sqrtss
        assert_eq!(len(&[0xf3, 0x0f, 0x52, 0xc0]), 4); // rsqrtss
        assert_eq!(len(&[0x0f, 0x10, 0x01]), 3); // movups xmm0, [ecx]
        assert_eq!(len(&[0x0f, 0xc6, 0xc0, 0x00]), 4); // shufps xmm0, xmm0, 0
        assert_eq!(len(&[0x66, 0x0f, 0x70, 0xc0, 0x1b]), 5); // pshufd
        assert_eq!(len(&[0x66, 0x0f, 0xef, 0xc0]), 4); // pxor
        assert_eq!(len(&[0x66, 0x0f, 0x6e, 0xc0]), 4); // movd xmm0, eax
        assert_eq!(len(&[0xf3, 0x0f, 0x2a, 0xc0]), 4); // cvtsi2ss
        assert_eq!(len(&[0x0f, 0x2e, 0xc1]), 3); // ucomiss
                                                 // A repeat prefix on a non-SSE opcode is not allowed.
        assert!(decode(&[0xf3, 0xa5]).is_err());
        assert!(decode(&[0xf3, 0x8b, 0xc1]).is_err());
    }

    #[test]
    fn relative_branches_are_flagged() {
        let call = decode(&[0xe8, 0x10, 0x00, 0x00, 0x00]).unwrap();
        assert_eq!(
            call,
            Insn {
                len: 5,
                kind: Kind::Rel32 { field_offset: 1 }
            }
        );
        let jmp = decode(&[0xe9, 0xfb, 0xff, 0xff, 0xff]).unwrap();
        assert_eq!(
            jmp,
            Insn {
                len: 5,
                kind: Kind::Rel32 { field_offset: 1 }
            }
        );
        let jcc = decode(&[0x0f, 0x84, 0x00, 0x01, 0x00, 0x00]).unwrap();
        assert_eq!(
            jcc,
            Insn {
                len: 6,
                kind: Kind::Rel32 { field_offset: 2 }
            }
        );
        assert_eq!(decode(&[0xe8, 0x00, 0x00]), Err(DecodeError::Truncated));
        assert!(decode(&[0x66, 0xe8, 0x00, 0x00]).is_err()); // rel16 form
    }

    #[test]
    fn short_branches_are_rejected() {
        assert_eq!(
            decode(&[0xeb, 0x05]),
            Err(DecodeError::ShortBranch { opcode: 0xeb })
        );
        for op in 0x70..=0x7f {
            assert_eq!(
                decode(&[op, 0x05]),
                Err(DecodeError::ShortBranch { opcode: op })
            );
        }
        for op in 0xe0..=0xe3 {
            assert_eq!(
                decode(&[op, 0x05]),
                Err(DecodeError::ShortBranch { opcode: op })
            );
        }
        assert!(decode_run(&[0x85, 0xc9, 0x74, 0x06, 0xb8], 5).is_err());
    }

    #[test]
    fn unsupported_and_truncated() {
        assert_eq!(
            decode(&[0xc3]),
            Err(DecodeError::Unsupported { opcode: 0xc3 })
        );
        assert_eq!(
            decode(&[0xc2, 0x04, 0x00]),
            Err(DecodeError::Unsupported { opcode: 0xc2 })
        );
        assert_eq!(
            decode(&[0xcc]),
            Err(DecodeError::Unsupported { opcode: 0xcc })
        );
        assert_eq!(
            decode(&[0x0f, 0x0b]),
            Err(DecodeError::Unsupported { opcode: 0x0f0b })
        );
        assert_eq!(
            decode(&[0xc9]),
            Err(DecodeError::Unsupported { opcode: 0xc9 })
        );
        assert_eq!(
            decode(&[0x67, 0x8b, 0x00]),
            Err(DecodeError::Unsupported { opcode: 0x67 })
        );
        assert_eq!(
            decode(&[0xc7, 0xc8, 0, 0, 0, 0]),
            Err(DecodeError::Unsupported { opcode: 0xc7 })
        );
        assert_eq!(decode(&[]), Err(DecodeError::Truncated));
        assert_eq!(decode(&[0x8b]), Err(DecodeError::Truncated));
        assert_eq!(
            decode(&[0x8b, 0x84, 0x24, 0x00]),
            Err(DecodeError::Truncated)
        );
        assert_eq!(decode(&[0x68, 0x00]), Err(DecodeError::Truncated));
        assert_eq!(decode(&[0x64]), Err(DecodeError::Truncated));
        assert_eq!(decode(&[0x0f]), Err(DecodeError::Truncated));
        assert_eq!(
            decode(&[0x64, 0x64, 0x64, 0x64, 0x64, 0x90]),
            Err(DecodeError::TooManyPrefixes)
        );
    }

    #[test]
    fn every_supported_one_byte_register_op_has_length_one() {
        for op in 0x40..=0x5f {
            assert_eq!(len(&[op]), 1, "{op:#x}");
        }
    }

    #[test]
    fn decoder_never_reads_past_the_slice() {
        // Feed every prefix of a long instruction; the result must be an
        // error or exactly the full length, never a length past the end.
        let samples: [&[u8]; 4] = [
            &[0x8b, 0x84, 0x24, 0x00, 0x01, 0x00, 0x00],
            &[0xf3, 0x0f, 0x10, 0x84, 0xa4, 0x00, 0x01, 0x00, 0x00],
            &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00],
            &[
                0x81, 0x84, 0x24, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            ],
        ];
        for s in samples {
            for cut in 0..s.len() {
                assert!(decode(&s[..cut]).is_err(), "{:02x?}", &s[..cut]);
            }
            assert_eq!(decode(s).unwrap().len, s.len());
        }
    }

    #[test]
    fn run_decoding_reports_the_offset() {
        let err = decode_run(&[0x55, 0x8b, 0xec, 0xc3, 0x90], 5).unwrap_err();
        assert_eq!(err.offset, 3);
        assert_eq!(err.cause, DecodeError::Unsupported { opcode: 0xc3 });
        assert!(err.to_string().contains("offset 3"));
    }

    #[test]
    fn auto_steal_finds_the_first_boundary_of_five() {
        assert_eq!(
            auto_steal(&[0x55, 0x8b, 0xec, 0x83, 0xec, 0x10, 0x53]).unwrap(),
            6
        );
        assert_eq!(
            auto_steal(&[0x6a, 0xff, 0x68, 0, 0, 0, 0, 0x64, 0xa1]).unwrap(),
            7
        );
        assert_eq!(
            auto_steal(&[0x83, 0xec, 0x10, 0x56, 0x8b, 0xf1]).unwrap(),
            6
        );
        assert_eq!(auto_steal(&[0xb8, 1, 0, 0, 0, 0xc3]).unwrap(), 5);
        assert!(auto_steal(&[0x55, 0x8b]).is_err());
    }

    #[test]
    fn trampoline_copies_plain_instructions() {
        let code = [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf0, 0x83, 0xec, 0x18];
        let t = build_trampoline(&code, 6, 0x0040_1000, 0x1000_0000).unwrap();
        assert_eq!(&t[..6], &code[..6]);
        assert_eq!(t[6], 0xe9);
        // Jump from 0x1000_000B (end of the jmp) back to 0x0040_1006.
        let rel = i32::from_le_bytes(t[7..11].try_into().unwrap());
        assert_eq!(0x1000_000bu32.wrapping_add(rel as u32), 0x0040_1006);
        assert_eq!(t.len(), 11);
    }

    #[test]
    fn trampoline_relocates_call_and_jump() {
        // push ebp; call rel32 -> 0x00402000; mov ebp, esp ... at 0x00401000.
        let src = 0x0040_1000u32;
        let target = 0x0040_2000u32;
        let rel = target.wrapping_sub(src + 1 + 5);
        let mut code = vec![0x55, 0xe8];
        code.extend_from_slice(&rel.to_le_bytes());
        code.extend_from_slice(&[0x8b, 0xec]);
        let dst = 0x2000_0000u32;
        let t = build_trampoline(&code, 8, src, dst).unwrap();
        let new_rel = u32::from_le_bytes(t[2..6].try_into().unwrap());
        assert_eq!(dst.wrapping_add(6).wrapping_add(new_rel), target);
        assert_eq!(&t[6..8], &[0x8b, 0xec]);
        // Return jump.
        let back = u32::from_le_bytes(t[9..13].try_into().unwrap());
        assert_eq!(dst.wrapping_add(8 + 5).wrapping_add(back), src + 8);

        // An E9 first instruction (a jump thunk) is relocated the same way.
        let rel = target.wrapping_sub(src + 5);
        let mut code = vec![0xe9];
        code.extend_from_slice(&rel.to_le_bytes());
        code.extend_from_slice(&[0xcc; 4]); // padding after the thunk is not decoded
        let t = build_trampoline(&code, 5, src, dst).unwrap();
        let new_rel = u32::from_le_bytes(t[1..5].try_into().unwrap());
        assert_eq!(dst.wrapping_add(5).wrapping_add(new_rel), target);
    }

    #[test]
    fn trampoline_relocates_backwards_across_the_address_space() {
        // Target below the destination, destination below the source.
        let src = 0x0150_0000u32;
        let target = 0x0040_0100u32;
        let dst = 0x0010_0000u32;
        let mut code = vec![0xe8];
        code.extend_from_slice(&target.wrapping_sub(src + 5).to_le_bytes());
        let t = build_trampoline(&code, 5, src, dst).unwrap();
        let new_rel = u32::from_le_bytes(t[1..5].try_into().unwrap());
        assert_eq!(dst.wrapping_add(5).wrapping_add(new_rel), target);
    }

    #[test]
    fn trampoline_relocates_jcc_rel32() {
        let src = 0x0040_1000u32;
        let target = 0x0040_1200u32;
        // push ebp; jz rel32; (6 bytes) -> 7 stolen.
        let mut code = vec![0x55, 0x0f, 0x84];
        code.extend_from_slice(&target.wrapping_sub(src + 7).to_le_bytes());
        let t = build_trampoline(&code, 7, src, 0x3000_0000).unwrap();
        let new_rel = u32::from_le_bytes(t[3..7].try_into().unwrap());
        assert_eq!(0x3000_0007u32.wrapping_add(new_rel), target);
    }

    #[test]
    fn trampoline_refuses_branch_into_the_stolen_bytes() {
        // call whose target is src+3, in the middle of the 5 stolen bytes.
        let src = 0x0040_1000u32;
        let mut code = vec![0xe8];
        code.extend_from_slice(&(3u32.wrapping_sub(5)).to_le_bytes());
        match build_trampoline(&code, 5, src, 0x2000_0000) {
            Err(TrampolineError::BranchIntoStolen {
                insn_offset: 0,
                target,
            }) => assert_eq!(target, src + 3),
            other => panic!("{other:?}"),
        }
        // A call to the function itself (offset 0) is fine.
        let mut code = vec![0xe8];
        code.extend_from_slice(&(0u32.wrapping_sub(5)).to_le_bytes());
        assert!(build_trampoline(&code, 5, src, 0x2000_0000).is_ok());
    }

    #[test]
    fn trampoline_boundary_checks() {
        let code = [0x55, 0x8b, 0xec, 0x83, 0xec, 0x10, 0x53, 0x56];
        match build_trampoline(&code, 5, 0x401000, 0x2000_0000) {
            Err(TrampolineError::NotOnBoundary {
                steal: 5,
                before: 3,
                after: 6,
            }) => {}
            other => panic!("{other:?}"),
        }
        assert!(matches!(
            build_trampoline(&code, 4, 0x401000, 0x2000_0000),
            Err(TrampolineError::TooShort { .. })
        ));
        assert!(build_trampoline(&code, 6, 0x401000, 0x2000_0000).is_ok());
        // Not enough bytes to see the end of the last instruction.
        assert!(matches!(
            build_trampoline(&[0x55, 0x8b, 0xec, 0x83, 0xec], 5, 0x401000, 0x2000_0000),
            Err(TrampolineError::Decode(_))
        ));
        // A short branch inside the stolen bytes.
        assert!(matches!(
            build_trampoline(
                &[0x85, 0xc9, 0x74, 0x06, 0xb8, 1, 0, 0, 0],
                5,
                0x401000,
                0x2000_0000
            ),
            Err(TrampolineError::Decode(RunError {
                offset: 2,
                cause: DecodeError::ShortBranch { opcode: 0x74 }
            }))
        ));
    }
}
