//! `Havok SDK/hkpsolverexport.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `exportImpulsesAndRhs` mixes SSE (the vector products: single precision, lane by lane, a
//! NaN in the destination operand winning) with x87 (the scalar arithmetic: through the
//! prelude's `x87_*` helpers, one call per instruction, in the exe's operand order).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00d51760::{
    load_lanes, mulps, sse_add, store_lanes, subps, Lanes,
};

/// `hkSolverExport::exportImpulsesAndRhs` (`00def570`, cdecl, four stack words).
const EXPORT_IMPULSES_AND_RHS: u32 = 0x00de_f570;

/// The solver body copy (`00d72030`, `this` = the destination, one stack word = the source;
/// 0x80 bytes).
const BODY_COPY: u32 = 0x00d7_2030;
/// `hkVector4::setMul3` (`00c7f750`, `this` = the result, `(matrix, vector)`).
const SET_MUL3: u32 = 0x00c7_f750;
/// The size in bytes of each Jacobian schema command, by command byte (`011b9414`).
const COMMAND_SIZES: u32 = 0x011b_9414;
/// The number of impulse floats each schema command consumes, by command byte (`011b9434`).
const COMMAND_IMPULSES: u32 = 0x011b_9434;
/// The `double` 1.5 the export keeps on the x87 stack (`01016ff0`).
const ONE_AND_A_HALF: u32 = 0x0101_6ff0;
/// The `double` the impulse limits are scaled by (`0106d508`: the float 0.99 widened).
const LIMIT_FRACTION: u32 = 0x0106_d508;

/// `hkpSolverInfo::m_dampDivTau` (Xbox PDB) +0x50.
const INFO_DAMP_DIV_TAU: u32 = 0x50;
/// `hkpSolverInfo::m_dampDivFrictionTau` (Xbox PDB) +0x58.
const INFO_DAMP_DIV_FRICTION_TAU: u32 = 0x58;
/// `hkpSolverInfo::m_deltaTime` (Xbox PDB) +0x10C.
const INFO_DELTA_TIME: u32 = 0x10c;
/// `hkpSolverInfo::m_numSteps` (Xbox PDB) +0x114.
const INFO_NUM_STEPS: u32 = 0x114;

/// The export's own stack block (`AND ESP,-16; SUB ESP,0x204` and three saved registers):
/// the two body copies sit at +0x110 and +0x190, the four `setMul3` results at +0xD0 ..
/// +0x100.
const FRAME_SIZE: u32 = 0x210;

/// The values the export keeps on the x87 stack (and in one stack slot) for the whole walk.
struct Constants {
    /// `[ESP+0x60]`: `m_dampDivTau * m_deltaTime`, as stored (a `float` memory operand).
    tau_dt: u32,
    /// `ST3` at the loop head: `m_dampDivFrictionTau * m_deltaTime`, stored and reloaded.
    friction_dt: f64,
    /// `ST2`: `m_numSteps * m_deltaTime`, stored and reloaded.
    steps_dt: f64,
    /// `ST1`: `m_deltaTime`, stored and reloaded.
    delta_time: f64,
    /// `ST0`: the `double` at `01016ff0`.
    one_and_a_half: f64,
}

/// `FLD dword`.
fn fld(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// A `float` memory operand of an x87 arithmetic instruction.
fn m32(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// `FLD dword [source]; FSTP dword [dest]`: the float copied, a signalling NaN quieted.
fn copy_float(e: &mut Engine, dest: u32, source: u32) {
    let bits = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(dest, bits);
}

/// `FSTP dword` then `FLD dword` of the same slot.
fn spill(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `ADDPS dest, source`.
fn addps(dest: Lanes, source: Lanes) -> Lanes {
    std::array::from_fn(|lane| sse_add(dest[lane], source[lane]))
}

/// The horizontal sum of the first three lanes the exe forms with `SHUFPS 0x55`, `ADDSS`,
/// `SHUFPS 0xAA`, `ADDSS`: `z + (y + x)`, as the bits `MOVSS` stores.
fn sum3(lanes: Lanes) -> u32 {
    sse_add(lanes[2], sse_add(lanes[1], lanes[0]))
}

/// `FLD sum; FMUL ST(steps); FSTP tmp; FSUB tmp`: `head - round(sum * steps_dt)`.
fn minus_rounded(head: f64, sum: u32, steps_dt: f64) -> f64 {
    let product = x87_store32(x87_mul(x87_load32(sum), steps_dt));
    x87_sub(head, x87_m32(product))
}

/// `FLD sum; FMUL ST(steps); FSUBP`: `head - sum * steps_dt`, the product kept on the stack.
fn minus_on_stack(head: f64, sum: u32, steps_dt: f64) -> f64 {
    x87_sub(head, x87_mul(x87_load32(sum), steps_dt))
}

/// The x87 condition bits `FCOM` leaves in `AH` (`C0` = 0x01, `C2` = 0x04, `C3` = 0x40).
fn fcom(st0: f64, source: f64) -> u8 {
    match st0.partial_cmp(&source) {
        None => 0x45,
        Some(std::cmp::Ordering::Less) => 0x01,
        Some(std::cmp::Ordering::Equal) => 0x40,
        Some(std::cmp::Ordering::Greater) => 0x00,
    }
}

/// `TEST AH,0x5` then `JP`: the parity flag of `C0 | C2` (set for none or both).
fn parity_even(status: u8) -> bool {
    matches!(status & 0x05, 0x00 | 0x05)
}

/// The common row (`00def84f`): the velocity of `row` (`+0` linear, `+0x10` with body A's
/// angular, `+0x20` with body B's angular, `+0xC` the target) against the bodies; writes the
/// impulse and `target * tau_dt - velocity * steps_dt` to `out`.
fn plain_row(e: &mut Engine, k: &Constants, row: u32, a: u32, b: u32, impulse: u32, out: u32) {
    let head = x87_mul(fld(e, row + 0xc), x87_m32(k.tau_dt));
    let by_b = mulps(load_lanes(e, b + 0x50), load_lanes(e, row + 0x20));
    let by_a = mulps(load_lanes(e, a + 0x50), load_lanes(e, row + 0x10));
    let head = spill(head);
    let angular = addps(by_b, by_a);
    let linear = subps(load_lanes(e, a + 0x40), load_lanes(e, b + 0x40));
    let linear = mulps(linear, load_lanes(e, row));
    copy_float(e, out, impulse);
    let sum = sum3(addps(angular, linear));
    let value = minus_rounded(head, sum, k.steps_dt);
    e.mem.set_u32(out + 4, x87_store32(value));
}

/// A record (`[ESP+0x58]`): a 16-bit zero, the 16-bit kind, the tag word of the current
/// header and one more word.
fn put_record(e: &mut Engine, record: u32, kind: u16, tag: u32, word: u32) {
    e.mem.set_u16(record, 0);
    e.mem.set_u16(record + 2, kind);
    e.mem.set_u32(record + 4, tag);
    e.mem.set_u32(record + 8, word);
}

/// One limit of command `0x1f` (`00df04b7` / `00df0517`):
/// `round(round(limit / denominator) * m_deltaTime / divisor * 1.5)`.
fn friction_limit(
    e: &Engine,
    k: &Constants,
    info: u32,
    limit: u32,
    denominator: u32,
    divisor: u32,
) -> f64 {
    let value = spill(x87_div(fld(e, limit), m32(e, denominator)));
    let value = x87_mul(value, m32(e, info + INFO_DELTA_TIME));
    let value = x87_div(value, m32(e, divisor));
    spill(x87_mul(value, k.one_and_a_half))
}

// Translated from 00def570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverExport::exportImpulsesAndRhs` (Xbox PDB), cdecl `(info, impulses, schema,
/// bodies)`: walks the Jacobian schema at `schema` and, for every constraint row, writes the
/// row's accumulated impulse (from `impulses`, consumed in order) and its right-hand side to
/// the output the schema's headers name; it also writes 16-byte records from `schema + 0x10`
/// on, and finally their number to the first word of `schema`.
///
/// `info` is read at the `hkpSolverInfo` offsets (Xbox PDB names): `m_dampDivTau *
/// m_deltaTime` (`tau_dt`), `m_dampDivFrictionTau * m_deltaTime` (`friction_dt`) and
/// `m_numSteps * m_deltaTime` (`steps_dt`) are formed once. The right-hand side of a row is
/// `target * scale - velocity * steps_dt`, the velocity being the row's vectors against the
/// two bodies' 0x80-byte records (`+0x40`, `+0x50`). Commands (the first byte, sign-extended,
/// through the jump table at `00df05f8`):
/// - `0`: end. `1` (0x10 bytes): a header: the output stride (byte `+1`), the two body
///   indices (words `+4`, `+6`, into `bodies`), the output (`+8`) and the records' tag
///   (`+0xC`); with no output, the commands `5` .. `0x1b` that follow are skipped by the size
///   and impulse tables (`011b9414`, `011b9434`). `2`: jump by the word at `+4`. `3`: the
///   output moves by byte `+4` times the stride (as a signed 16-bit value). `0x19`: skip 0x20.
/// - `5`, `0x10`, `0x11` (0x30), `7`, `8` (0x40): a plain row with `tau_dt`. `6` (0x30) as
///   well, after remembering whether `|impulse|` exceeded `0.99 * [+0x2C]`; when it has and
///   the next command is not `6`, a record of kind 0. `0x12`, `0x13` (0x30): a record of kind
///   1 with the output address when the impulse exceeds `0.99 * [+0x2C]`, then a plain row.
/// - `9` (0x40), `0xA` (0x50, which first adds the third impulse to the target): a plain row
///   scaled by the second impulse; the impulses advance by the table. `0xB` (0x40): a row with
///   both bodies' linear vectors. `0xC` (0x20), `0xD` (0x30): an angular row with
///   `m_deltaTime`. `0xE` (0x30), `0xF` (0x40, adding the third impulse to the target at
///   `+0x1C`): the same scaled by the second impulse.
/// - `0x14`, `0x15` (0x70): two plain rows. `0x16` (0x70), `0x17` (0x90), `0x18` (0x50): two,
///   three and two rows written to the output the command names (`+0x68`, `+0x8C`, `+0x44`),
///   scaled by the last impulse, with `friction_dt` (`0x16`, `0x17`) or `m_deltaTime`
///   (`0x18`).
/// - `0x1A` (0x20): one body (B when the word at `+0x10` is nonzero, else A) is copied to the
///   stack and the vector at `+0` added to its `+0x10` and `+0x40`. `0x1B` (0x60): both bodies
///   copied, and `matrix * [+0x20]` / `matrix * [+0x50]` of each added to its `+0x10` / `+0x40`
///   (A's matrix at `+0`, B's at `+0x30`).
/// - `0x1D`, `0x1E`: runs of one and three rows per body pair; `0x1F`: runs of six rows per
///   pair (two per step), the pairs' mode bytes copied out first and each step's second row
///   bounded below (mode 3) and above (mode 1) by its limits.
///
/// The commands `4` and `0x1c` and any byte above `0x1f` (also negative ones) make the exe
/// jump back to the same read for ever (`00def5ff`); they panic here.
pub fn hk_solver_export_export_impulses_and_rhs(
    e: &mut Engine,
    info: u32,
    impulses: u32,
    schema: u32,
    bodies: u32,
) {
    let tau_dt = x87_store32(x87_mul(
        fld(e, info + INFO_DAMP_DIV_TAU),
        m32(e, info + INFO_DELTA_TIME),
    ));
    let friction_dt = x87_store32(x87_mul(
        fld(e, info + INFO_DAMP_DIV_FRICTION_TAU),
        m32(e, info + INFO_DELTA_TIME),
    ));
    let steps = e.mem.u32(info + INFO_NUM_STEPS) as i32;
    let steps_dt = x87_store32(x87_mul(f64::from(steps), m32(e, info + INFO_DELTA_TIME)));
    let delta_time = x87_store32(fld(e, info + INFO_DELTA_TIME));
    let mut k = Constants {
        tau_dt,
        friction_dt: x87_load32(friction_dt),
        steps_dt: x87_load32(steps_dt),
        delta_time: x87_load32(delta_time),
        one_and_a_half: x87_load64(e.mem.u64(ONE_AND_A_HALF)),
    };
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let copy_a = frame + 0x110;
        let copy_b = frame + 0x190;
        // ESI, EBX, EDI, ECX, EDX and the stack slots of the exe.
        let mut cursor = schema;
        let mut impulses = impulses;
        let mut out = 0u32;
        let mut body_a = 0u32;
        let mut body_b = 0u32;
        let mut stride = 0u32;
        let mut count = 0u32;
        let mut tag = 0u32;
        let mut record = schema.wrapping_add(0x10);
        let mut over_limit = false;
        let table = |e: &Engine, base: u32, command: i8| -> u32 {
            u32::from(e.mem.u8(base.wrapping_add(i32::from(command) as u32)))
        };
        loop {
            let command = e.mem.i8(cursor);
            let row = cursor;
            match command {
                // 00df05df
                0 => break,
                // 00def614
                1 => {
                    stride = u32::from(e.mem.u8(row + 1));
                    out = e.mem.u32(row + 8);
                    let next_tag = e.mem.u32(row + 0xc);
                    body_a = (u32::from(e.mem.u16(row + 4)) << 7).wrapping_add(bodies);
                    body_b = (u32::from(e.mem.u16(row + 6)) << 7).wrapping_add(bodies);
                    cursor = row + 0x10;
                    tag = next_tag;
                    if out == 0 {
                        let mut next = e.mem.i8(cursor);
                        while next > 4 && next < 0x1c {
                            let consumed = table(e, COMMAND_IMPULSES, next);
                            let size = table(e, COMMAND_SIZES, next);
                            next = e.mem.i8(cursor.wrapping_add(size));
                            cursor = cursor.wrapping_add(size);
                            impulses = impulses.wrapping_add(consumed * 4);
                        }
                    }
                }
                // 00df0171
                2 => cursor = cursor.wrapping_add(e.mem.u32(row + 4)),
                // 00def67e
                3 => {
                    let steps = u32::from(e.mem.u8(row + 4));
                    let stride_word = i32::from(stride as u16 as i16) as u32;
                    out = out.wrapping_add(steps.wrapping_mul(stride_word));
                    cursor = row + 0x10;
                }
                // 00def848, 00def7d8, 00def7e1, 00def784, then 00def84f
                5..=8 | 0x10..=0x13 => {
                    match command {
                        7 | 8 => cursor = row + 0x40,
                        6 => {
                            let magnitude = x87_store32(fld(e, impulses).abs());
                            cursor = row + 0x30;
                            let fraction = f64::from_bits(e.mem.u64(LIMIT_FRACTION));
                            let limit = x87_mul(fld(e, row + 0x2c), fraction);
                            if !parity_even(fcom(limit, x87_load32(magnitude))) {
                                over_limit = true;
                            }
                            if e.mem.i8(cursor) != 6 && over_limit {
                                let at = record;
                                count = count.wrapping_add(1);
                                record = record.wrapping_add(0x10);
                                put_record(e, at, 0, tag, 0);
                                over_limit = false;
                            }
                        }
                        0x12 | 0x13 => {
                            let impulse = fld(e, impulses);
                            let fraction = f64::from_bits(e.mem.u64(LIMIT_FRACTION));
                            let limit = x87_mul(fld(e, row + 0x2c), fraction);
                            cursor = row + 0x30;
                            if !parity_even(fcom(spill(limit), impulse)) {
                                let at = record;
                                count = count.wrapping_add(1);
                                record = record.wrapping_add(0x10);
                                put_record(e, at, 1, tag, out);
                            }
                        }
                        _ => cursor = row + 0x30,
                    }
                    plain_row(e, &k, row, body_a, body_b, impulses, out);
                    out = out.wrapping_add(stride);
                    impulses = impulses.wrapping_add(4);
                }
                // 00def8ef, 00def8d2, then 00def902
                9 | 0xa => {
                    let scale = x87_quiet_bits(e.mem.u32(impulses + 4));
                    let kind = if command == 0xa {
                        cursor = row + 0x50;
                        let target = x87_add(fld(e, impulses + 8), m32(e, row + 0xc));
                        e.mem.set_u32(row + 0xc, x87_store32(target));
                        e.mem.i8(row)
                    } else {
                        cursor = row + 0x40;
                        command
                    };
                    let next_impulses =
                        impulses.wrapping_add(table(e, COMMAND_IMPULSES, kind) * 4);
                    let head = x87_mul(fld(e, row + 0xc), x87_m32(k.tau_dt));
                    let by_b = mulps(load_lanes(e, body_b + 0x50), load_lanes(e, row + 0x20));
                    let by_a = mulps(load_lanes(e, body_a + 0x50), load_lanes(e, row + 0x10));
                    let head = spill(head);
                    let angular = addps(by_b, by_a);
                    let linear =
                        subps(load_lanes(e, body_a + 0x40), load_lanes(e, body_b + 0x40));
                    let linear = mulps(linear, load_lanes(e, row));
                    copy_float(e, out, impulses);
                    impulses = next_impulses;
                    let sum = sum3(addps(angular, linear));
                    let value = spill(minus_rounded(head, sum, k.steps_dt));
                    let value = x87_mul(value, x87_m32(scale));
                    e.mem.set_u32(out + 4, x87_store32(value));
                    out = out.wrapping_add(stride);
                }
                // 00defab9
                0xb => {
                    let head = fld(e, row + 0xc);
                    let by_b = mulps(load_lanes(e, body_b + 0x50), load_lanes(e, row + 0x30));
                    let by_a = mulps(load_lanes(e, body_a + 0x50), load_lanes(e, row + 0x20));
                    let angular = addps(by_b, by_a);
                    let linear_b =
                        mulps(load_lanes(e, body_b + 0x40), load_lanes(e, row + 0x10));
                    let linear_a = mulps(load_lanes(e, body_a + 0x40), load_lanes(e, row));
                    copy_float(e, out, impulses);
                    let head = spill(x87_mul(spill(head), x87_m32(k.tau_dt)));
                    let linear = subps(linear_a, linear_b);
                    let sum = sum3(addps(angular, linear));
                    cursor = row + 0x40;
                    let value = minus_rounded(head, sum, k.steps_dt);
                    e.mem.set_u32(out + 4, x87_store32(value));
                    out = out.wrapping_add(stride);
                    impulses = impulses.wrapping_add(4);
                }
                // 00def99a, 00def9a1, then 00def9a6
                0xc | 0xd => {
                    cursor = row + if command == 0xc { 0x20 } else { 0x30 };
                    let head = spill(fld(e, row + 0x1c));
                    let by_b = mulps(load_lanes(e, body_b + 0x50), load_lanes(e, row + 0x10));
                    copy_float(e, out, impulses);
                    let by_a = mulps(load_lanes(e, body_a + 0x50), load_lanes(e, row));
                    let sum = sum3(addps(by_b, by_a));
                    let head = x87_mul(head, k.delta_time);
                    let value = minus_on_stack(head, sum, k.steps_dt);
                    e.mem.set_u32(out + 4, x87_store32(value));
                    out = out.wrapping_add(stride);
                    impulses = impulses.wrapping_add(4);
                }
                // 00defa25, 00defa08, then 00defa38
                0xe | 0xf => {
                    let scale = x87_quiet_bits(e.mem.u32(impulses + 4));
                    let kind = if command == 0xf {
                        cursor = row + 0x40;
                        let target = x87_add(fld(e, impulses + 8), m32(e, row + 0x1c));
                        e.mem.set_u32(row + 0x1c, x87_store32(target));
                        e.mem.i8(row)
                    } else {
                        cursor = row + 0x30;
                        command
                    };
                    let next_impulses =
                        impulses.wrapping_add(table(e, COMMAND_IMPULSES, kind) * 4);
                    let head = spill(fld(e, row + 0x1c));
                    let by_b = mulps(load_lanes(e, body_b + 0x50), load_lanes(e, row + 0x10));
                    copy_float(e, out, impulses);
                    let by_a = mulps(load_lanes(e, body_a + 0x50), load_lanes(e, row));
                    impulses = next_impulses;
                    let sum = sum3(addps(by_b, by_a));
                    let head = x87_mul(head, k.delta_time);
                    let value = spill(minus_on_stack(head, sum, k.steps_dt));
                    let value = x87_mul(value, x87_m32(scale));
                    e.mem.set_u32(out + 4, x87_store32(value));
                    out = out.wrapping_add(stride);
                }
                // 00def698
                0x14 | 0x15 => {
                    let b_angular = load_lanes(e, body_b + 0x50);
                    let a_angular = load_lanes(e, body_a + 0x50);
                    let target = fld(e, row + 0xc);
                    let tau = x87_load32(k.tau_dt);
                    let linear =
                        subps(load_lanes(e, body_a + 0x40), load_lanes(e, body_b + 0x40));
                    let first_head = x87_mul(target, tau);
                    let by_b = mulps(load_lanes(e, row + 0x20), b_angular);
                    let by_a = mulps(load_lanes(e, row + 0x10), a_angular);
                    let first = addps(by_a, by_b);
                    let first_head = spill(first_head);
                    let first = addps(first, mulps(load_lanes(e, row), linear));
                    let second_head = spill(x87_mul(tau, m32(e, row + 0x3c)));
                    let first_sum = sum3(first);
                    let second_a = mulps(load_lanes(e, row + 0x40), a_angular);
                    let second_b = mulps(load_lanes(e, row + 0x50), b_angular);
                    let second_linear = load_lanes(e, row + 0x30);
                    copy_float(e, out, impulses);
                    let value = minus_rounded(first_head, first_sum, k.steps_dt);
                    let second = addps(second_a, second_b);
                    let second = addps(second, mulps(linear, second_linear));
                    e.mem.set_u32(out + 4, x87_store32(value));
                    out = out.wrapping_add(stride);
                    copy_float(e, out, impulses + 4);
                    let second_sum = sum3(second);
                    cursor = row + 0x70;
                    let value = minus_rounded(second_head, second_sum, k.steps_dt);
                    e.mem.set_u32(out + 4, x87_store32(value));
                    out = out.wrapping_add(stride);
                    impulses = impulses.wrapping_add(8);
                }
                // 00defb4c
                0x16 => {
                    let scale = x87_quiet_bits(e.mem.u32(impulses + 8));
                    let b_angular = load_lanes(e, body_b + 0x50);
                    let a_angular = load_lanes(e, body_a + 0x50);
                    let linear =
                        subps(load_lanes(e, body_a + 0x40), load_lanes(e, body_b + 0x40));
                    let first_head = spill(x87_mul(fld(e, row + 0xc), k.friction_dt));
                    let by_b = mulps(load_lanes(e, row + 0x20), b_angular);
                    let by_a = mulps(load_lanes(e, row + 0x10), a_angular);
                    let first = addps(by_a, by_b);
                    let first = addps(first, mulps(load_lanes(e, row), linear));
                    let second_head = x87_store32(x87_mul(fld(e, row + 0x3c), k.friction_dt));
                    let second_impulse = x87_quiet_bits(e.mem.u32(impulses + 4));
                    let first_sum = sum3(first);
                    let second_a = mulps(load_lanes(e, row + 0x40), a_angular);
                    let second_b = mulps(load_lanes(e, row + 0x50), b_angular);
                    let second_linear = load_lanes(e, row + 0x30);
                    let dest = e.mem.u32(row + 0x68);
                    copy_float(e, dest, impulses);
                    let first_value = spill(minus_rounded(first_head, first_sum, k.steps_dt));
                    let second = addps(second_a, second_b);
                    let second = addps(second, mulps(linear, second_linear));
                    let second_sum = sum3(second);
                    let scale_value = x87_load32(scale);
                    cursor = row + 0x70;
                    impulses = impulses.wrapping_add(0xc);
                    let first_scaled = x87_store32(x87_mul(first_value, scale_value));
                    e.mem.set_u32(dest + 4, first_scaled);
                    e.mem.set_u32(dest + 8, second_impulse);
                    let second_value =
                        minus_rounded(x87_load32(second_head), second_sum, k.steps_dt);
                    let second_value = x87_mul(scale_value, x87_m32(x87_store32(second_value)));
                    e.mem.set_u32(dest + 0xc, x87_store32(second_value));
                }
                // 00defc5b
                0x17 => {
                    let scale = x87_quiet_bits(e.mem.u32(impulses + 0xc));
                    let dest = e.mem.u32(row + 0x8c);
                    let a_angular = load_lanes(e, body_a + 0x50);
                    let second_head = x87_store32(x87_mul(fld(e, row + 0x3c), k.friction_dt));
                    let b_angular = load_lanes(e, body_b + 0x50);
                    let linear =
                        subps(load_lanes(e, body_a + 0x40), load_lanes(e, body_b + 0x40));
                    let third_target = x87_quiet_bits(e.mem.u32(row + 0x7c));
                    let first_head = fld(e, row + 0xc);
                    let by_a = mulps(load_lanes(e, row + 0x10), a_angular);
                    let first_head = spill(x87_mul(first_head, k.friction_dt));
                    let by_b = mulps(load_lanes(e, row + 0x20), b_angular);
                    let first = addps(by_a, by_b);
                    let first = addps(first, mulps(load_lanes(e, row), linear));
                    let first_sum = sum3(first);
                    let second_a = mulps(load_lanes(e, row + 0x40), a_angular);
                    let second_b = mulps(load_lanes(e, row + 0x50), b_angular);
                    let second = addps(second_a, second_b);
                    let first_value = spill(minus_rounded(first_head, first_sum, k.steps_dt));
                    let second = addps(second, mulps(load_lanes(e, row + 0x30), linear));
                    let second_sum = sum3(second);
                    let scale_value = x87_load32(scale);
                    let first_scaled = x87_store32(x87_mul(first_value, scale_value));
                    let second_impulse = x87_quiet_bits(e.mem.u32(impulses + 4));
                    let third_a = mulps(a_angular, load_lanes(e, row + 0x60));
                    let third_b = mulps(load_lanes(e, row + 0x70), b_angular);
                    let third = addps(third_b, third_a);
                    copy_float(e, dest, impulses);
                    e.mem.set_u32(dest + 4, first_scaled);
                    let third_sum = sum3(third);
                    cursor = row + 0x90;
                    let third_impulse = impulses.wrapping_add(8);
                    impulses = impulses.wrapping_add(0x10);
                    e.mem.set_u32(dest + 8, second_impulse);
                    let second_value = spill(minus_rounded(
                        x87_load32(second_head),
                        second_sum,
                        k.steps_dt,
                    ));
                    let second_value = x87_mul(second_value, scale_value);
                    e.mem.set_u32(dest + 0xc, x87_store32(second_value));
                    copy_float(e, dest + 0x10, third_impulse);
                    let third_head = x87_mul(x87_load32(third_target), k.friction_dt);
                    let third_value =
                        x87_store32(minus_on_stack(third_head, third_sum, k.steps_dt));
                    let third_value = x87_mul(scale_value, x87_m32(third_value));
                    e.mem.set_u32(dest + 0x14, x87_store32(third_value));
                }
                // 00defddb
                0x18 => {
                    let consumed = table(e, COMMAND_IMPULSES, command);
                    let scale = x87_quiet_bits(e.mem.u32(impulses + 8));
                    let b_angular = load_lanes(e, body_b + 0x50);
                    let a_angular = load_lanes(e, body_a + 0x50);
                    let first_target = x87_quiet_bits(e.mem.u32(row + 0x1c));
                    let second_target = x87_quiet_bits(e.mem.u32(row + 0x3c));
                    let by_b = mulps(load_lanes(e, row + 0x10), b_angular);
                    let by_a = mulps(load_lanes(e, row), a_angular);
                    let second_impulse = x87_quiet_bits(e.mem.u32(impulses + 4));
                    let first_sum = sum3(addps(by_b, by_a));
                    let second_b = mulps(load_lanes(e, row + 0x30), b_angular);
                    let second_a = mulps(a_angular, load_lanes(e, row + 0x20));
                    let dest = e.mem.u32(row + 0x44);
                    copy_float(e, dest, impulses);
                    let first_head = x87_mul(x87_load32(first_target), k.delta_time);
                    let second_sum = sum3(addps(second_b, second_a));
                    let first_value = spill(minus_on_stack(first_head, first_sum, k.steps_dt));
                    cursor = row + 0x50;
                    let scale_value = x87_load32(scale);
                    let first_scaled = x87_store32(x87_mul(first_value, scale_value));
                    e.mem.set_u32(dest + 4, first_scaled);
                    e.mem.set_u32(dest + 8, second_impulse);
                    let second_head = x87_mul(x87_load32(second_target), k.delta_time);
                    let second_value =
                        x87_store32(minus_on_stack(second_head, second_sum, k.steps_dt));
                    let second_value = x87_mul(scale_value, x87_m32(second_value));
                    e.mem.set_u32(dest + 0xc, x87_store32(second_value));
                    impulses = impulses.wrapping_add(consumed * 4);
                }
                // 00df0086
                0x19 => cursor = row + 0x20,
                // 00defedd
                0x1a => {
                    cursor = row + 0x20;
                    let target = if e.mem.u32(row + 0x10) == 0 {
                        e.call(BODY_COPY, &args![copy_a, body_a]);
                        body_a = copy_a;
                        copy_a
                    } else {
                        e.call(BODY_COPY, &args![copy_b, body_b]);
                        body_b = copy_b;
                        copy_b
                    };
                    let sum = addps(load_lanes(e, row), load_lanes(e, target + 0x10));
                    store_lanes(e, target + 0x10, sum);
                    let sum = addps(load_lanes(e, row), load_lanes(e, target + 0x40));
                    store_lanes(e, target + 0x40, sum);
                    // 00df0067: the x87 constants reloaded (the 1.5 from the exe's data again).
                    k.one_and_a_half = x87_load64(e.mem.u64(ONE_AND_A_HALF));
                }
                // 00deff4d
                0x1b => {
                    cursor = row + 0x60;
                    e.call(BODY_COPY, &args![copy_a, body_a]);
                    body_a = copy_a;
                    e.call(BODY_COPY, &args![copy_b, body_b]);
                    body_b = copy_b;
                    e.call(SET_MUL3, &args![frame + 0xd0, row, copy_a + 0x20]);
                    e.call(SET_MUL3, &args![frame + 0xf0, row, copy_a + 0x50]);
                    e.call(SET_MUL3, &args![frame + 0x100, row + 0x30, copy_b + 0x20]);
                    e.call(SET_MUL3, &args![frame + 0xe0, row + 0x30, copy_b + 0x50]);
                    for (field, product) in [
                        (copy_a + 0x10, frame + 0xd0),
                        (copy_a + 0x40, frame + 0xf0),
                        (copy_b + 0x10, frame + 0x100),
                        (copy_b + 0x40, frame + 0xe0),
                    ] {
                        let sum = addps(load_lanes(e, field), load_lanes(e, product));
                        store_lanes(e, field, sum);
                    }
                    // 00df0067.
                    k.one_and_a_half = x87_load64(e.mem.u64(ONE_AND_A_HALF));
                }
                // 00df008e
                0x1d => {
                    let pairs_count = u32::from(e.mem.u16(row + 2));
                    if out == 0 {
                        // 00df0179
                        impulses = impulses.wrapping_add(pairs_count * 4);
                        cursor = cursor.wrapping_add(e.mem.u32(row + 4));
                        continue;
                    }
                    let pairs = (pairs_count << 6).wrapping_add(row).wrapping_add(0x14);
                    let mut rows = row + 0x10;
                    let mut index = 0u32;
                    if pairs_count != 0 {
                        loop {
                            let head = x87_mul(fld(e, rows + 0xc), k.delta_time);
                            let a = e.mem.u32(pairs + index * 4).wrapping_add(bodies);
                            let b = e.mem.u32(pairs + index * 4 + 4).wrapping_add(bodies);
                            let head = spill(head);
                            let by_b = mulps(load_lanes(e, b + 0x50), load_lanes(e, rows + 0x20));
                            let by_a = mulps(load_lanes(e, a + 0x50), load_lanes(e, rows + 0x10));
                            let angular = addps(by_b, by_a);
                            let linear = subps(load_lanes(e, a + 0x40), load_lanes(e, b + 0x40));
                            let linear = mulps(linear, load_lanes(e, rows));
                            copy_float(e, out, impulses);
                            let sum = sum3(addps(angular, linear));
                            index += 1;
                            impulses = impulses.wrapping_add(4);
                            rows = rows.wrapping_add(0x30);
                            let value = minus_rounded(head, sum, k.steps_dt);
                            e.mem.set_u32(out + 4, x87_store32(value));
                            out = out.wrapping_add(stride);
                            if (index as i32) >= i32::from(e.mem.u16(row + 2)) {
                                break;
                            }
                        }
                    }
                    cursor = cursor.wrapping_add(e.mem.u32(row + 4));
                }
                // 00df0188
                0x1e => {
                    let pairs_count = u32::from(e.mem.u16(row + 2));
                    if out == 0 {
                        // 00df028b
                        impulses = impulses.wrapping_add(pairs_count * 3 * 4);
                        cursor = cursor.wrapping_add(e.mem.u32(row + 4));
                        continue;
                    }
                    let pairs = pairs_count
                        .wrapping_mul(0x130)
                        .wrapping_add(row)
                        .wrapping_add(0x20);
                    let mut rows = row + 0x10;
                    let mut index = 0u32;
                    if pairs_count != 0 {
                        loop {
                            let a = e.mem.u32(pairs + index * 4).wrapping_add(bodies);
                            let b = e.mem.u32(pairs + index * 4 + 4).wrapping_add(bodies);
                            for _ in 0..3 {
                                let head = x87_mul(fld(e, rows + 0xc), k.delta_time);
                                let by_a =
                                    mulps(load_lanes(e, rows + 0x10), load_lanes(e, a + 0x50));
                                let head = spill(head);
                                let by_b =
                                    mulps(load_lanes(e, rows + 0x20), load_lanes(e, b + 0x50));
                                let angular = addps(by_a, by_b);
                                let linear =
                                    subps(load_lanes(e, a + 0x40), load_lanes(e, b + 0x40));
                                let linear = mulps(linear, load_lanes(e, rows));
                                copy_float(e, out, impulses);
                                let sum = sum3(addps(angular, linear));
                                impulses = impulses.wrapping_add(4);
                                rows = rows.wrapping_add(0x30);
                                let value = minus_rounded(head, sum, k.steps_dt);
                                e.mem.set_u32(out + 4, x87_store32(value));
                                out = out.wrapping_add(stride);
                            }
                            index += 1;
                            if (index as i32) >= i32::from(e.mem.u16(row + 2)) {
                                break;
                            }
                        }
                    }
                    cursor = cursor.wrapping_add(e.mem.u32(row + 4));
                }
                // 00df029d
                0x1f => {
                    // The pairs' mode bytes copied to the array at `+8`.
                    if e.mem.u16(row + 4) != 0 {
                        let mut index = 0u32;
                        let mut offset = 0u32;
                        loop {
                            let source = (u32::from(e.mem.u16(row + 4)) * 0x4f4)
                                .wrapping_add(offset)
                                .wrapping_add(row)
                                .wrapping_add(0x44);
                            offset = offset.wrapping_add(0x40);
                            let byte = e.mem.u8(source);
                            let dest = e.mem.u32(row + 8);
                            e.mem.set_u8(index.wrapping_add(dest), byte);
                            index += 1;
                            if (index as i32) >= i32::from(e.mem.u16(row + 4)) {
                                break;
                            }
                        }
                    }
                    let pairs_count = u32::from(e.mem.u16(row + 4));
                    let second_rows = (pairs_count * 0x90).wrapping_add(row).wrapping_add(0x20);
                    if out == 0 {
                        // 00df05d1
                        impulses = impulses.wrapping_add(pairs_count * 3 * 8);
                        cursor = cursor.wrapping_add(e.mem.u32(row + 0x10));
                        continue;
                    }
                    let pairs = (pairs_count * 0x4f0).wrapping_add(row).wrapping_add(0x40);
                    if pairs_count != 0 {
                        let out_step = stride.wrapping_mul(3).wrapping_mul(2);
                        let mut rows = row + 0x30;
                        let mut seconds = second_rows + 0xc;
                        let mut pair = 0u32;
                        loop {
                            let a = e.mem.u32(pairs + pair * 4).wrapping_add(bodies);
                            let b = e.mem.u32(pairs + pair * 4 + 4).wrapping_add(bodies);
                            let pair_offset = pair << 6;
                            let mut limit_offset = 0u32;
                            let mut shift = 0u32;
                            let mut out2 = out.wrapping_add(0x1c);
                            let mut impulse = impulses;
                            loop {
                                // 00df03a7, the first row: `rows - 0x10` linear, `rows` with
                                // A, `rows + 0x10` with B, the target at `rows - 4`; it always
                                // goes to the pair's first output slot.
                                let head = spill(x87_mul(fld(e, rows - 4), k.delta_time));
                                let by_b =
                                    mulps(load_lanes(e, rows + 0x10), load_lanes(e, b + 0x50));
                                let by_a = mulps(load_lanes(e, rows), load_lanes(e, a + 0x50));
                                let angular = addps(by_b, by_a);
                                let linear =
                                    subps(load_lanes(e, a + 0x40), load_lanes(e, b + 0x40));
                                copy_float(e, out, impulse);
                                let linear = mulps(linear, load_lanes(e, rows - 0x10));
                                let sum = sum3(addps(angular, linear));
                                let value = minus_rounded(head, sum, k.steps_dt);
                                e.mem.set_u32(out + 4, x87_store32(value));
                                // The second row: `seconds - 0xC` with A, `seconds + 4` with
                                // B, the target at `seconds + 0x10`.
                                let by_b =
                                    mulps(load_lanes(e, seconds + 4), load_lanes(e, b + 0x50));
                                let second_head = x87_mul(fld(e, seconds + 0x10), k.delta_time);
                                let by_a =
                                    mulps(load_lanes(e, seconds - 0xc), load_lanes(e, a + 0x50));
                                let second_sum = sum3(addps(by_b, by_a));
                                let second = x87_store32(minus_on_stack(
                                    second_head,
                                    second_sum,
                                    k.steps_dt,
                                ));
                                copy_float(e, out2 - 4, impulse + 0xc);
                                let mut current = x87_load32(second);
                                e.mem.set_u32(out2, x87_store32(current));
                                let modes = (u32::from(e.mem.u16(row + 4)) * 0x4f4)
                                    .wrapping_add(pair_offset)
                                    .wrapping_add(row)
                                    .wrapping_add(0x44);
                                let divisor = modes + limit_offset + 0x10;
                                if (e.mem.u8(modes) >> shift) & 3 == 3 {
                                    let limit = friction_limit(
                                        e,
                                        &k,
                                        info,
                                        modes + limit_offset + 8,
                                        seconds,
                                        divisor,
                                    );
                                    // FCOM; JNP 00df04f9: the value stays when the limit is
                                    // below it (ordered), else the limit.
                                    if parity_even(fcom(limit, current)) {
                                        current = limit;
                                    }
                                    e.mem.set_u32(out2, x87_store32(current));
                                }
                                if (e.mem.u8(modes) >> shift) & 3 == 1 {
                                    let limit = friction_limit(
                                        e,
                                        &k,
                                        info,
                                        modes + limit_offset + 4,
                                        seconds,
                                        divisor,
                                    );
                                    // FCOM; TEST AH,0x41; JZ 00df055f: the value stays when
                                    // the limit is above it (ordered), else the limit.
                                    let kept = if fcom(limit, current) & 0x41 == 0 {
                                        current
                                    } else {
                                        limit
                                    };
                                    e.mem.set_u32(out2, x87_store32(kept));
                                }
                                impulse = impulse.wrapping_add(4);
                                seconds = seconds.wrapping_add(0x20);
                                limit_offset += 0x14;
                                out2 = out2.wrapping_add(8);
                                shift += 2;
                                rows = rows.wrapping_add(0x30);
                                if shift >= 6 {
                                    break;
                                }
                            }
                            out = out.wrapping_add(out_step);
                            pair += 1;
                            impulses = impulses.wrapping_add(0x18);
                            if (pair as i32) >= i32::from(e.mem.u16(row + 4)) {
                                break;
                            }
                        }
                    }
                    cursor = cursor.wrapping_add(e.mem.u32(row + 0x10));
                }
                _ => panic!(
                    "00def5ff: schema command {command:#x} at {cursor:#010x}: the exe spins here for ever"
                ),
            }
        }
        // 00df05df
        e.mem.set_u32(schema, count);
    });
}

// Translated from 00df0680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkExportImpulsesAndRhs` (Xbox PDB): a jump to `hkSolverExport::exportImpulsesAndRhs`
/// (`00def570`) with the caller's four stack words unchanged (that function reads exactly
/// four and returns nothing).
pub fn hk_export_impulses_and_rhs(
    e: &mut Engine,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
) {
    e.call(
        EXPORT_IMPULSES_AND_RHS,
        &args![word_1, word_2, word_3, word_4],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00de_f570,
            hk_solver_export_export_impulses_and_rhs(u32, u32, u32, u32)
        ),
        entry!(0x00df_0680, hk_export_impulses_and_rhs(u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn the_export_jumps_to_the_solver_export_with_the_same_words() {
        let mut e = Engine::new();
        let log = Rc::new(RefCell::new(Vec::new()));
        let double_log = Rc::clone(&log);
        e.register_double(EXPORT_IMPULSES_AND_RHS, move |_, a| {
            double_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00df_0680, &args![1u32, 2u32, 3u32, 4u32]);
        assert_eq!(*log.borrow(), vec![vec![1, 2, 3, 4]]);
    }

    /// The command sizes the exe keeps at `011b9414` and the impulse counts at `011b9434`.
    const SIZES: [u8; 32] = [
        16, 16, 16, 16, 255, 48, 48, 64, 64, 64, 80, 64, 32, 48, 48, 64, 48, 48, 48, 48, 112, 112,
        112, 144, 80, 32, 32, 96, 255, 255, 255, 255,
    ];
    const IMPULSES: [u8; 28] = [
        255, 255, 255, 255, 255, 1, 1, 1, 1, 2, 3, 1, 1, 1, 2, 3, 1, 1, 1, 1, 2, 2, 3, 4, 3, 0, 0,
        0,
    ];

    type Vec4 = [f32; 4];
    const X: Vec4 = [0.0, 1.0, 0.0, 0.0];
    const Z: Vec4 = [0.0, 0.0, 1.0, 0.0];

    struct Setup {
        e: Engine,
        info: u32,
        bodies: u32,
        impulses: u32,
    }

    /// The tables and constants, a solver info with `tau_dt` 1, `friction_dt` 1.5,
    /// `steps_dt` 2 and `m_deltaTime` 0.5, two bodies (A: linear `(0,1,0)`, angular
    /// `(0,2,0)`; B: linear zero, angular `(0,0,3)`) and 64 impulses `100 + i`.
    fn setup() -> Setup {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x0106_d000, 0x1000);
        e.map(0x011b_9000, 0x1000);
        e.mem.set_u64(ONE_AND_A_HALF, 1.5f64.to_bits());
        e.mem.set_u64(LIMIT_FRACTION, f64::from(0.99f32).to_bits());
        for (index, size) in SIZES.into_iter().enumerate() {
            e.mem.set_u8(COMMAND_SIZES + index as u32, size);
        }
        for (index, count) in IMPULSES.into_iter().enumerate() {
            e.mem.set_u8(COMMAND_IMPULSES + index as u32, count);
        }
        let info = e.mem.alloc(0x130);
        e.mem.set_f32(info + INFO_DAMP_DIV_TAU, 2.0);
        e.mem.set_f32(info + INFO_DAMP_DIV_FRICTION_TAU, 3.0);
        e.mem.set_f32(info + INFO_DELTA_TIME, 0.5);
        e.mem.set_u32(info + INFO_NUM_STEPS, 4);
        let bodies = e.mem.alloc(0x100);
        put(&mut e, bodies + 0x40, X);
        put(&mut e, bodies + 0x50, [0.0, 2.0, 0.0, 0.0]);
        put(&mut e, bodies + 0x80 + 0x50, [0.0, 0.0, 3.0, 0.0]);
        let impulses = e.mem.alloc(0x100);
        for index in 0..64 {
            e.mem.set_f32(impulses + index * 4, 100.0 + index as f32);
        }
        Setup {
            e,
            info,
            bodies,
            impulses,
        }
    }

    fn put(e: &mut Engine, at: u32, v: Vec4) {
        for (index, value) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, value);
        }
    }

    /// A vector whose lane 3 is `w`.
    fn with_w(v: Vec4, w: f32) -> Vec4 {
        [v[0], v[1], v[2], w]
    }

    /// The command byte goes into the low byte of the command's first word (the x lane of its
    /// first vector, zero here), so it is written after the vectors.
    fn header(e: &mut Engine, at: u32, stride: u8, out: u32, tag: u32) {
        e.mem.set_u8(at, 1);
        e.mem.set_u8(at + 1, stride);
        e.mem.set_u16(at + 4, 0);
        e.mem.set_u16(at + 6, 1);
        e.mem.set_u32(at + 8, out);
        e.mem.set_u32(at + 0xc, tag);
    }

    /// The standard row: linear `(0,1,0)`, A angular `(0,1,0)`, B angular `(0,0,1)`, against
    /// the bodies a velocity of 1 + 2 + 3 = 6.
    fn plain(e: &mut Engine, at: u32, command: u8, target: f32) {
        put(e, at, with_w(X, target));
        put(e, at + 0x10, X);
        put(e, at + 0x20, Z);
        e.mem.set_u8(at, command);
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|index| e.mem.f32(at + index * 4)).collect()
    }

    fn run(s: &mut Setup, schema: u32) {
        let (info, impulses, bodies) = (s.info, s.impulses, s.bodies);
        s.e.call(
            EXPORT_IMPULSES_AND_RHS,
            &args![info, impulses, schema, bodies],
        );
    }

    #[test]
    fn plain_rows_write_impulse_and_rhs_and_the_limit_records() {
        let mut s = setup();
        let out = s.e.mem.alloc(0x40);
        let schema = s.e.mem.alloc(0x100);
        let e = &mut s.e;
        header(e, schema, 8, out, 0x77);
        plain(e, schema + 0x10, 5, 4.0);
        // 0x12: impulse 101 exceeds 0.99 * 10, a record of kind 1 with the output address.
        plain(e, schema + 0x40, 0x12, 4.0);
        put(e, schema + 0x60, with_w(Z, 10.0));
        // 6 with a high limit, then 6 whose |impulse| exceeds 0.99 * 1 before the end.
        plain(e, schema + 0x70, 6, 4.0);
        put(e, schema + 0x90, with_w(Z, 200.0));
        plain(e, schema + 0xa0, 6, 4.0);
        put(e, schema + 0xc0, with_w(Z, 1.0));
        e.mem.set_u8(schema + 0xd0, 0);
        run(&mut s, schema);
        let e = &s.e;
        // target * tau_dt - 6 * steps_dt = 4 - 12.
        assert_eq!(
            floats(e, out, 8),
            [100.0, -8.0, 101.0, -8.0, 102.0, -8.0, 103.0, -8.0]
        );
        assert_eq!(e.mem.u32(schema), 2);
        assert_eq!(e.mem.u16(schema + 0x10), 0);
        assert_eq!(e.mem.u16(schema + 0x12), 1);
        assert_eq!(e.mem.u32(schema + 0x14), 0x77);
        assert_eq!(e.mem.u32(schema + 0x18), out + 8);
        assert_eq!(e.mem.u16(schema + 0x22), 0);
        assert_eq!(e.mem.u32(schema + 0x24), 0x77);
        assert_eq!(e.mem.u32(schema + 0x28), 0);
    }

    #[test]
    fn scaled_angular_and_linear_rows_and_the_moves() {
        let mut s = setup();
        let out = s.e.mem.alloc(0x80);
        let schema = s.e.mem.alloc(0x200);
        let e = &mut s.e;
        header(e, schema, 8, out, 0);
        // 9: (4 - 12) * impulse 101; two impulses.
        plain(e, schema + 0x10, 9, 4.0);
        // 0xA: target 4 + impulse 104 = 108; (108 - 12) * 103; three impulses.
        plain(e, schema + 0x50, 0xa, 4.0);
        // 0xC: A angular (0,1,0) -> 2, B (0,0,1) -> 3: 6 * 0.5 - 5 * 2 = -7.
        put(e, schema + 0xa0, X);
        put(e, schema + 0xb0, with_w(Z, 6.0));
        e.mem.set_u8(schema + 0xa0, 0xc);
        // 0xF: target 6 + impulse 108 = 114: (57 - 10) * 107.
        put(e, schema + 0xc0, X);
        put(e, schema + 0xd0, with_w(Z, 6.0));
        e.mem.set_u8(schema + 0xc0, 0xf);
        // 0xB: A linear . (0,1,0) = 1, B linear zero, A angular . (0,1,0) = 2, B angular .
        // (0,0,1) = 3: 5 - 12.
        put(e, schema + 0x100, with_w(X, 5.0));
        put(e, schema + 0x110, X);
        put(e, schema + 0x120, X);
        put(e, schema + 0x130, Z);
        e.mem.set_u8(schema + 0x100, 0xb);
        e.mem.set_u8(schema + 0x140, 0x19);
        // 3 moves the output by 2 strides; 2 jumps over a command that would spin.
        e.mem.set_u8(schema + 0x160, 3);
        e.mem.set_u8(schema + 0x164, 2);
        e.mem.set_u8(schema + 0x170, 2);
        e.mem.set_u32(schema + 0x174, 0x20);
        e.mem.set_u8(schema + 0x180, 4);
        // 0xD: 2 * 0.5 - 10.
        put(e, schema + 0x190, X);
        put(e, schema + 0x1a0, with_w(Z, 2.0));
        e.mem.set_u8(schema + 0x190, 0xd);
        e.mem.set_u8(schema + 0x1c0, 0);
        run(&mut s, schema);
        let e = &s.e;
        assert_eq!(
            floats(e, out, 16),
            [
                100.0,
                -8.0 * 101.0,
                102.0,
                96.0 * 103.0,
                105.0,
                -7.0,
                106.0,
                47.0 * 107.0,
                109.0,
                -7.0,
                0.0,
                0.0,
                0.0,
                0.0,
                110.0,
                -9.0
            ]
        );
        assert_eq!(e.mem.f32(schema + 0x5c), 108.0);
        assert_eq!(e.mem.f32(schema + 0xdc), 114.0);
        assert_eq!(e.mem.u32(schema), 0);
    }

    #[test]
    fn row_pairs_and_the_rows_written_where_the_command_says() {
        let mut s = setup();
        let out = s.e.mem.alloc(0x40);
        let dest = s.e.mem.alloc(0x80);
        let schema = s.e.mem.alloc(0x300);
        let e = &mut s.e;
        // A header without output skips 5 and 0x14 (three impulses).
        header(e, schema, 8, 0, 0);
        e.mem.set_u8(schema + 0x10, 5);
        e.mem.set_u8(schema + 0x40, 0x14);
        header(e, schema + 0xb0, 8, out, 0);
        // 0x14: 4 - 12, then the second row (target 2 at +0x3C): 2 - 12.
        let c = schema + 0xc0;
        plain(e, c + 0x30, 0, 2.0);
        plain(e, c, 0x14, 4.0);
        // 0x16: (4 * 1.5 - 12) * 2, (2 * 1.5 - 12) * 2 (impulse 107 is the scale).
        let c = schema + 0x130;
        plain(e, c + 0x30, 0, 2.0);
        e.mem.set_u32(c + 0x68, dest);
        plain(e, c, 0x16, 4.0);
        e.mem.set_f32(s.impulses + 7 * 4, 2.0);
        // 0x17: the third row A (0,1,0), B (0,0,1), target 4 at +0x7C: 6 - 10; scale 111.
        let c = schema + 0x1a0;
        plain(e, c + 0x30, 0, 2.0);
        put(e, c + 0x60, X);
        put(e, c + 0x70, with_w(Z, 4.0));
        e.mem.set_u32(c + 0x8c, dest + 0x10);
        plain(e, c, 0x17, 4.0);
        e.mem.set_f32(s.impulses + 11 * 4, 0.5);
        // 0x18: (6 * 0.5 - 10) * 2, (2 * 0.5 - 10) * 2.
        let c = schema + 0x230;
        put(e, c, X);
        put(e, c + 0x10, with_w(Z, 6.0));
        put(e, c + 0x20, X);
        put(e, c + 0x30, with_w(Z, 2.0));
        e.mem.set_u32(c + 0x44, dest + 0x30);
        e.mem.set_u8(c, 0x18);
        e.mem.set_f32(s.impulses + 14 * 4, 2.0);
        e.mem.set_u8(schema + 0x280, 0);
        run(&mut s, schema);
        let e = &s.e;
        assert_eq!(floats(e, out, 4), [103.0, -8.0, 104.0, -10.0]);
        assert_eq!(floats(e, dest, 4), [105.0, -12.0, 106.0, -18.0]);
        assert_eq!(
            floats(e, dest + 0x10, 6),
            [108.0, -3.0, 109.0, -4.5, 110.0, -2.0]
        );
        assert_eq!(floats(e, dest + 0x30, 4), [112.0, -14.0, 113.0, -18.0]);
    }

    #[test]
    fn body_copies_change_the_velocities_of_the_rows_after_them() {
        let mut s = setup();
        let copies = Rc::new(RefCell::new(Vec::new()));
        let copy_log = Rc::clone(&copies);
        s.e.register_double(BODY_COPY, move |e, a| {
            copy_log.borrow_mut().push(a.to_vec());
            for offset in (0..0x80).step_by(4) {
                let word = e.mem.u32(a[1] + offset);
                e.mem.set_u32(a[0] + offset, word);
            }
            Ret::default()
        });
        let products = Rc::new(RefCell::new(Vec::new()));
        let product_log = Rc::clone(&products);
        s.e.register_double(SET_MUL3, move |e, a| {
            let mut log = product_log.borrow_mut();
            log.push(a.to_vec());
            e.mem.set_f32(a[0], 0.0);
            e.mem.set_f32(a[0] + 4, log.len() as f32);
            e.mem.set_f32(a[0] + 8, 0.0);
            e.mem.set_f32(a[0] + 12, 0.0);
            Ret::default()
        });
        let out = s.e.mem.alloc(0x40);
        let schema = s.e.mem.alloc(0x200);
        let bodies = s.bodies;
        let e = &mut s.e;
        header(e, schema, 8, out, 0);
        // 0x1A on A: its linear becomes (0,2,0): 2 + 2 + 3 = 7, 4 - 14.
        put(e, schema + 0x10, X);
        e.mem.set_u8(schema + 0x10, 0x1a);
        plain(e, schema + 0x30, 5, 4.0);
        // 0x1A on B: its linear becomes (0,-1,0): 3 + 2 + 3 = 8, 4 - 16.
        put(e, schema + 0x60, [0.0, -1.0, 0.0, 0.0]);
        e.mem.set_u32(schema + 0x70, 1);
        e.mem.set_u8(schema + 0x60, 0x1a);
        plain(e, schema + 0x80, 5, 4.0);
        // 0x1B: A's linear + (0,2,0) = (0,4,0), B's + (0,4,0) = (0,3,0): 1 + 2 + 3, 4 - 12.
        e.mem.set_u8(schema + 0xb0, 0x1b);
        plain(e, schema + 0x110, 5, 4.0);
        e.mem.set_u8(schema + 0x140, 0);
        run(&mut s, schema);
        let e = &s.e;
        assert_eq!(floats(e, out, 6), [100.0, -10.0, 101.0, -12.0, 102.0, -8.0]);
        // The bodies themselves are untouched.
        assert_eq!(floats(e, bodies + 0x40, 2), [0.0, 1.0]);
        let copies = copies.borrow();
        assert_eq!(copies.len(), 4);
        assert_eq!(copies[0][1], bodies);
        assert_eq!(copies[1][1], bodies + 0x80);
        let (copy_a, copy_b) = (copies[0][0], copies[1][0]);
        assert_eq!(copy_b, copy_a + 0x80);
        assert_eq!(copies[2], vec![copy_a, copy_a]);
        assert_eq!(copies[3], vec![copy_b, copy_b]);
        let frame = copy_a - 0x110;
        let row = schema + 0xb0;
        assert_eq!(
            *products.borrow(),
            vec![
                vec![frame + 0xd0, row, copy_a + 0x20],
                vec![frame + 0xf0, row, copy_a + 0x50],
                vec![frame + 0x100, row + 0x30, copy_b + 0x20],
                vec![frame + 0xe0, row + 0x30, copy_b + 0x50],
            ]
        );
    }

    /// Command 0x1f for one pair (A, B) at `c` (0x580 bytes): first rows with targets 8 at
    /// every step (only the last stays), second rows with targets 4, 40, 6 and modes 3, 1, 3.
    fn friction_run(e: &mut Engine, c: u32, mode_copy: u32) {
        e.mem.set_u16(c + 4, 1);
        e.mem.set_u32(c + 8, mode_copy);
        e.mem.set_u32(c + 0x10, 0x580);
        for step in 0..3 {
            let rows = c + 0x30 + step * 0x30;
            put(e, rows - 0x10, with_w(X, 8.0));
            put(e, rows, X);
            put(e, rows + 0x10, Z);
            let seconds = c + 0xbc + step * 0x20;
            put(e, seconds - 0xc, with_w(X, 1.0));
            put(e, seconds + 4, Z);
        }
        e.mem.set_f32(c + 0xcc, 4.0);
        e.mem.set_f32(c + 0xec, 40.0);
        e.mem.set_f32(c + 0x10c, 6.0);
        e.mem.set_u32(c + 0x534, 0x80);
        let modes = c + 0x538;
        e.mem.set_u8(modes, 0x37);
        for divisor in [0x10, 0x24, 0x38] {
            e.mem.set_f32(modes + divisor, 1.0);
        }
        e.mem.set_f32(modes + 8, 4.0);
        e.mem.set_f32(modes + 0x14 + 4, 8.0);
        e.mem.set_f32(modes + 0x28 + 8, -20.0);
        e.mem.set_u8(c, 0x1f);
    }

    #[test]
    fn row_runs_per_body_pair_and_the_friction_limits() {
        let mut s = setup();
        let out = s.e.mem.alloc(0x40);
        let out2 = s.e.mem.alloc(0x40);
        let mode_copy = s.e.mem.alloc(0x10);
        let schema = s.e.mem.alloc(0x1000);
        let e = &mut s.e;
        header(e, schema, 8, out, 0);
        // 0x1D with two pairs (A, B) and (B, A): 2 - 12, then B against A: -1 * ... 2 + 2.
        let c = schema + 0x10;
        e.mem.set_u8(c, 0x1d);
        e.mem.set_u16(c + 2, 2);
        e.mem.set_u32(c + 4, 0xa0);
        for row in 0..2 {
            put(e, c + 0x10 + row * 0x30, with_w(X, 4.0));
            put(e, c + 0x20 + row * 0x30, X);
            put(e, c + 0x30 + row * 0x30, Z);
        }
        e.mem.set_u32(c + 0x94, 0);
        e.mem.set_u32(c + 0x98, 0x80);
        e.mem.set_u32(c + 0x9c, 0);
        // 0x1E with one pair, three rows with targets 2, 4, 6.
        let c = schema + 0xb0;
        e.mem.set_u8(c, 0x1e);
        e.mem.set_u16(c + 2, 1);
        e.mem.set_u32(c + 4, 0x160);
        for row in 0..3 {
            put(e, c + 0x10 + row * 0x30, with_w(X, 2.0 + 2.0 * row as f32));
            put(e, c + 0x20 + row * 0x30, X);
            put(e, c + 0x30 + row * 0x30, Z);
        }
        e.mem.set_u32(c + 0x150, 0);
        e.mem.set_u32(c + 0x154, 0x80);
        // Without output: 0x1D (3), 0x1E (3 rows of 1) and 0x1F (6) skip 12 impulses.
        header(e, schema + 0x210, 8, 0, 0);
        let c = schema + 0x220;
        e.mem.set_u8(c, 0x1d);
        e.mem.set_u16(c + 2, 3);
        e.mem.set_u32(c + 4, 0x10);
        e.mem.set_u8(c + 0x10, 0x1e);
        e.mem.set_u16(c + 0x12, 1);
        e.mem.set_u32(c + 0x14, 0x10);
        let skipped = c + 0x20;
        let skipped_copy = mode_copy + 8;
        friction_run(e, skipped, skipped_copy);
        header(e, skipped + 0x580, 8, out2, 0);
        friction_run(e, skipped + 0x590, mode_copy);
        e.mem.set_u8(skipped + 0x590 + 0x580, 0);
        run(&mut s, schema);
        let e = &s.e;
        assert_eq!(
            floats(e, out, 10),
            [100.0, -10.0, 101.0, 4.0, 102.0, -11.0, 103.0, -10.0, 104.0, -9.0]
        );
        // Impulses 105 .. 116 skipped. First rows: 8 * 0.5 - 12 (the last step's impulse).
        // Second rows: 4 * 0.5 - 10 = -8 raised to the limit 4 * 0.5 * 1.5 = 3; 40 * 0.5 - 10
        // = 10 lowered to 8 * 0.5 * 1.5 = 6; 6 * 0.5 - 10 = -7 kept above -15.
        assert_eq!(floats(e, out2, 2), [119.0, -8.0]);
        assert_eq!(
            floats(e, out2 + 0x18, 6),
            [120.0, 3.0, 121.0, 6.0, 122.0, -7.0]
        );
        assert_eq!(e.mem.u8(mode_copy), 0x37);
        assert_eq!(e.mem.u8(skipped_copy), 0x37);
    }

    #[test]
    #[should_panic(expected = "spins here for ever")]
    fn command_four_spins() {
        let mut s = setup();
        let schema = s.e.mem.alloc(0x20);
        s.e.mem.set_u8(schema, 4);
        run(&mut s, schema);
    }
}
