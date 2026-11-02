//! `(unplaced)/run_004b4a10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004b4ad0`: thiscall with two stack words (the body keeps the stack
/// 16-byte aligned itself and returns nothing).
const IMPLEMENTATION: u32 = 0x004b_4ad0;

// Translated from 004b4ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall wrapper (`RET 8`): forwards `this` and both stack
/// words, in order, to `004b4ad0`.
pub fn fn_004b4ab0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(IMPLEMENTATION, &args![this, first, second]);
}

/// The function `fn_004b4f20` calls on the member (`006815c0`).
const CALLEE_006815C0: u32 = 0x0068_15c0;

// Translated from 004b4f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006815c0` with `ECX` = `this + 0x10` and passes on its pointer result.
pub fn fn_004b4f20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_006815C0, &args![this.byte_add(0x10)]).ptr()
}

/// One lane of an SSE arithmetic instruction (`MULPS`, `ADDPS`): a NaN in the destination
/// operand wins (quieted), then a NaN in the source operand, otherwise the result of `op`.
fn sse_lane(dest: u32, source: u32, op: fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

/// `MULPS dest, source` on the four lanes (an XMM register as four float bit patterns).
fn mulps(dest: [u32; 4], source: [u32; 4]) -> [u32; 4] {
    let mul: fn(f32, f32) -> f32 = |a, b| a * b;
    [0, 1, 2, 3].map(|lane| sse_lane(dest[lane], source[lane], mul))
}

/// `ADDPS dest, source` on the four lanes.
fn addps(dest: [u32; 4], source: [u32; 4]) -> [u32; 4] {
    let add: fn(f32, f32) -> f32 = |a, b| a + b;
    [0, 1, 2, 3].map(|lane| sse_lane(dest[lane], source[lane], add))
}

/// `MOVAPS xmm, [address]`: four words. (The instruction faults on an address that is not
/// 16-byte aligned; the engine's heap is only 8-byte aligned, so the fault is not modelled.)
fn load_lanes(e: &Engine, address: u32) -> [u32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.u32(address.wrapping_add(offset)))
}

// Translated from 004b4cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall with one stack word: the address of element `index` of an array of
/// 16-byte rows starting at `this` (`SHL EAX, 4` then `ADD EAX, this`, 32-bit wrap-around). It
/// reads no memory.
pub fn fn_004b4cf0(_e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    Ptr::new((index << 4).wrapping_add(this.addr()))
}

// Translated from 004b4ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004b4ad0 - the /GS stack cookie check (00ec408c) at the end is not translated.
/// No map name. Thiscall with two stack words (`RET 8`; `fn_004b4ab0` is its wrapper): `this` is a
/// 16-byte destination, `matrix` points at three 16-byte rows (`fn_004b4cf0` gives each row's
/// address) and `vector` at 16 bytes whose first three floats are used. It multiplies the vector
/// by the matrix's rows the way a 3x3 matrix times a vector does: the rows' lanes 0..2 are
/// transposed into columns by `UNPCKLPS`/`UNPCKHPS`/`SHUFPS`/`MOVLHPS`/`MOVHLPS`, each column is
/// multiplied (`MULPS`) by the vector's x, y or z broadcast to all lanes, and the three products
/// are added, `(x column + y column) + z column` (`ADDPS`), then stored with `MOVAPS` to `this`.
/// Lanes 0..2 are the three dot products; lane 3 is `row2[1] * x + row2[3] * y + row2[3] * z`,
/// whatever those words hold (the columns' fourth lanes come from the shuffles, as in the exe).
pub fn fn_004b4ad0(e: &mut Engine, this: Ptr, matrix: Ptr, vector: Ptr) {
    let row0_address = fn_004b4cf0(e, matrix, 0);
    let row0 = load_lanes(e, row0_address.addr());
    let row1_address = fn_004b4cf0(e, matrix, 1);
    let row1 = load_lanes(e, row1_address.addr());
    let row2_address = fn_004b4cf0(e, matrix, 2);
    let row2 = load_lanes(e, row2_address.addr());
    // UNPCKLPS / UNPCKHPS / SHUFPS 0xe4 / MOVLHPS / MOVHLPS / SHUFPS 0xd4, in the exe's order.
    let low_pairs = [row0[0], row1[0], row0[1], row1[1]];
    let high_pairs = [row0[2], row1[2], row0[3], row1[3]];
    let column2 = [high_pairs[0], high_pairs[1], row2[2], row2[3]];
    let column0 = [low_pairs[0], low_pairs[1], row2[0], row2[1]];
    let mixed = [low_pairs[2], low_pairs[3], column0[2], column0[3]];
    let column1 = [mixed[0], mixed[1], row2[1], row2[3]];
    let v = load_lanes(e, vector.addr());
    let x = mulps(column0, [v[0]; 4]);
    let y = mulps(column1, [v[1]; 4]);
    let z = mulps(column2, [v[2]; 4]);
    let sum = addps(addps(x, y), z);
    for (lane, word) in sum.iter().enumerate() {
        e.mem
            .set_u32(this.addr().wrapping_add(4 * lane as u32), *word);
    }
}

/// The `float` factor `fn_004b4e50` multiplies each component by.
const COMPONENT_SCALE: u32 = 0x011c_6294;

// Translated from 004b4e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: writes the three `float`s at `source`, each
/// multiplied by the `float` global at `011c6294`, to `this[0..3]` and `0.0` to `this[3]`;
/// returns `this`. The global is loaded again for every component (`FLD`, then `FMUL dword`),
/// so a NaN is chosen as the x87 does.
pub fn fn_004b4e50(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    for lane in 0..3u32 {
        let scale = e.mem.u32(COMPONENT_SCALE);
        let component = e.mem.u32(source.addr().wrapping_add(4 * lane));
        let product = x87_store32(x87_mul(x87_load32(scale), x87_m32(component)));
        e.mem.set_u32(this.addr().wrapping_add(4 * lane), product);
    }
    e.mem.set_u32(this.addr().wrapping_add(0xc), 0);
    this
}

/// `GetUserNameA` import slot (`advapi32`, stdcall: buffer, address of the size).
const GET_USER_NAME: u32 = 0x00fd_f014;
/// The 0x104-byte user-name buffer.
const USER_NAME: u32 = 0x011c_5fa0;
/// `Error` (`0040fbe0`, engine map; cdecl, variadic): empty in this build.
const ERROR: u32 = 0x0040_fbe0;
/// `"%s"`.
const PERCENT_S: u32 = 0x0101_9f08;
/// `"Computer Name: %s"`.
const COMPUTER_NAME_FORMAT: u32 = 0x0101_ffa8;
/// `00464f30` (cdecl: a string and 0; its result is formatted as a string).
const NAME_LOOKUP: u32 = 0x0046_4f30;
/// `_sprintf` (`00ec623a`, cdecl).
const SPRINTF: u32 = 0x00ec_623a;
/// `009d9f40` (thiscall, returns a pointer).
const OBJECT_GET: u32 = 0x009d_9f40;
/// `hkpMotion::getMass` (`00c86430`, engine map; thiscall, its result in ST0).
const MOTION_GET_MASS: u32 = 0x00c8_6430;
/// `00436aa0` (unit `fallout shared/extradataobjects.cpp` in the engine map; thiscall).
const CALLEE_00436AA0: u32 = 0x0043_6aa0;
/// `00537bb0` (unit `fallout shared/tesland.cpp` in the engine map; thiscall).
const BASE_CONSTRUCTOR_00537BB0: u32 = 0x0053_7bb0;
/// `00458700` (unit `fallout shared/tes.cpp` in the engine map; thiscall, `RET 8`: an out
/// block and an index; returns a pointer).
const ELEMENT_GET: u32 = 0x0045_8700;
/// `004586d0` (thiscall; its result in ST0).
const ELEMENT_VALUE: u32 = 0x0045_86d0;
/// The four floats `004b4f40` copies (0.5, 0.3, 250.0 and 0.95 in the exe's `.rdata`).
const DEFAULT_30: u32 = 0x0101_6248;
const DEFAULT_34: u32 = 0x0101_7870;
const DEFAULT_38: u32 = 0x0101_b2d0;
const DEFAULT_3C: u32 = 0x0101_ffc0;

// Translated from 004b4a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004b4a30 - the /GS stack-cookie check (00ec408c) is not translated
/// Reports the user and computer: with the size word set to 0x104, `GetUserNameA` into
/// `011c5fa0`; when that succeeds, `Error("%s", text)`, then `"Computer Name: %s"` with
/// `00464f30(011c5fa0, 0)` formatted into a 0x108-byte local (`_sprintf`) and given to
/// `Error`.
pub fn fn_004b4a30(e: &mut Engine, text: u32) {
    // The frame: the text at EBP-0x110, the size word at EBP-4.
    e.with_stack(0x110, |e, frame| {
        let line = frame.addr();
        let size = frame.addr() + 0x10c;
        e.mem.set_u32(size, 0x104);
        if e.call(GET_USER_NAME, &args![USER_NAME, size]).u32() != 0 {
            e.call(ERROR, &args![PERCENT_S, text]);
            let name = e.call(NAME_LOOKUP, &args![USER_NAME, 0u32]).u32();
            e.call(SPRINTF, &args![line, COMPUTER_NAME_FORMAT, name]);
            e.call(ERROR, &args![line]);
        }
    });
}

// Translated from 004b4d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004b4d50 - the /GS stack-cookie check (00ec408c) is not translated
/// Stores four floats into the 16 bytes at `this` (`MOVSS` loads, `UNPCKLPS` into one
/// register, `MOVAPS` through an aligned local: the words are copied unchanged). Returns
/// `this`.
pub fn fn_004b4d50(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, w: f32) -> Ptr {
    for (lane, value) in [x, y, z, w].into_iter().enumerate() {
        e.mem
            .set_u32(this.addr().wrapping_add(4 * lane as u32), value.to_bits());
    }
    this
}

// Translated from 004b4d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four floats at `this` into `out` through `004b4d50` (each pushed with
/// `FLD`/`FSTP`, so a signalling NaN arrives quiet). Returns `out`.
pub fn fn_004b4d10(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let lane = |e: &Engine, i: u32| {
        f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(4 * i))))
    };
    let (w, z, y, x) = (lane(e, 3), lane(e, 2), lane(e, 1), lane(e, 0));
    fn_004b4d50(e, out, x, y, z, w);
    out
}

// Translated from 004b4db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004b4db0 - the /GS stack-cookie check (00ec408c) is not translated
/// For `i` in 0..4: `004586d0` on the pointer `00458700(source, local_i, i)` returns a float
/// in ST0, stored (`FSTP`) as the `i`th float of `this`; each `local_i` is a 16-byte block of
/// the aligned frame. Returns `this`.
pub fn fn_004b4db0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    // The locals at EBP-0x20, -0x30, -0x40, -0x50 for i = 0..3.
    e.with_stack(0x50, |e, frame| {
        for i in 0..4u32 {
            let local = frame.addr() + 0x30 - 0x10 * i;
            let element = e.call(ELEMENT_GET, &args![source, local, i]).u32();
            let value = e.call(ELEMENT_VALUE, &args![element]).f64();
            e.mem
                .set_u32(this.addr().wrapping_add(4 * i), x87_store32(value));
        }
    });
    this
}

// Translated from 004b4ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::getMass` (`00c86430`) on `009d9f40(this)`; its result (ST0) is passed on.
pub fn fn_004b4ea0(e: &mut Engine, this: Ptr) -> Ret {
    let object = e.call(OBJECT_GET, &args![this]).u32();
    e.call(MOTION_GET_MASS, &args![object])
}

// Translated from 004b4ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00436aa0` on `006815c0(this + 0x10)`; its result is passed on.
pub fn fn_004b4ee0(e: &mut Engine, this: Ptr) -> Ret {
    let member = e
        .call(CALLEE_006815C0, &args![this.addr().wrapping_add(0x10)])
        .u32();
    e.call(CALLEE_00436AA0, &args![member])
}

// Translated from 004b4ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004b4ee0` on `009d9f40(this)`; its result is passed on.
pub fn fn_004b4ec0(e: &mut Engine, this: Ptr) -> Ret {
    let object = e.call(OBJECT_GET, &args![this]).u32();
    fn_004b4ee0(e, Ptr::new(object))
}

// Translated from 004b4f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004b4f20` on `009d9f40(this)`: the pointer `006815c0` gives for its member at +0x10.
pub fn fn_004b4f00(e: &mut Engine, this: Ptr) -> Ptr {
    let object = e.call(OBJECT_GET, &args![this]).u32();
    fn_004b4f20(e, Ptr::new(object))
}

// Translated from 004b4ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores four floats at `this` (each through `FLD`/`FSTP`, so a signalling NaN is stored
/// quiet). Returns `this` (`EAX`, which the decompiler drops).
pub fn fn_004b4ff0(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, w: f32) -> Ptr {
    for (lane, value) in [x, y, z, w].into_iter().enumerate() {
        e.mem.set_u32(
            this.addr().wrapping_add(4 * lane as u32),
            x87_quiet_bits(value.to_bits()),
        );
    }
    this
}

// Translated from 004b4fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the four floats at `this` to 0.0 (`FLDZ` pushed four times, `004b4ff0`). Returns
/// `004b4ff0`'s result, `this`.
pub fn fn_004b4fc0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_004b4ff0(e, this, 0.0, 0.0, 0.0, 0.0)
}

// Translated from 004b4fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default constructor of four floats: all 0.0 (`004b4fc0`). Returns `this`.
pub fn fn_004b4fa0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_004b4fc0(e, this);
    this
}

// Translated from 004b4f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor: the base `00537bb0`, the four-float members at +0x10 and +0x20 zeroed
/// (`004b4fa0`), and the floats at `01016248`, `01017870`, `0101b2d0` and `0101ffc0`
/// (0.5, 0.3, 250.0, 0.95) copied through `FLD`/`FSTP` to +0x30, +0x34, +0x38 and +0x3c.
/// Returns `this`.
pub fn fn_004b4f40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCTOR_00537BB0, &args![this]);
    fn_004b4fa0(e, Ptr::new(base.wrapping_add(0x10)));
    fn_004b4fa0(e, Ptr::new(base.wrapping_add(0x20)));
    for (offset, constant) in [
        (0x30u32, DEFAULT_30),
        (0x34, DEFAULT_34),
        (0x38, DEFAULT_38),
        (0x3c, DEFAULT_3C),
    ] {
        let bits = x87_quiet_bits(e.mem.u32(constant));
        e.mem.set_u32(base.wrapping_add(offset), bits);
    }
    this
}

// Translated from 004b5020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `006815c0` on `this` and clears the word at +0. Returns `this`.
pub fn fn_004b5020(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_006815C0, &args![this]);
    e.mem.set_u32(this.addr(), 0);
    this
}

/// The function pointer `011aa050` (a variable in `.data`): transforms an array of points
/// (`cdecl`: count, source points, destination points, transform).
const TRANSFORM_POINTS_POINTER: u32 = 0x011a_a050;

// Translated from 004b4a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words; called by `004b4910`): calls the
/// function whose address is the word at `011aa050` with the count zero-extended from its
/// low 16 bits and the three other words, and answers its `EAX`.
pub fn fn_004b4a10(
    e: &mut Engine,
    count: u16,
    source: u32,
    destination: u32,
    transform: u32,
) -> u32 {
    let function: u32 = e.global(TRANSFORM_POINTS_POINTER);
    e.call(
        function,
        &args![u32::from(count), source, destination, transform],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b_4a10, fn_004b4a10(u16, u32, u32, u32) -> u32),
        entry!(0x004b_4e50, fn_004b4e50(Ptr, Ptr) -> Ptr),
        entry!(0x004b4ab0, fn_004b4ab0(Ptr, u32, u32)),
        entry!(0x004b_4f20, fn_004b4f20(Ptr) -> Ptr),
        entry!(0x004b_4ad0, fn_004b4ad0(Ptr, Ptr, Ptr)),
        entry!(0x004b_4cf0, fn_004b4cf0(Ptr, u32) -> Ptr),
        entry!(0x004b_4a30, fn_004b4a30(u32)),
        entry!(0x004b_4d50, fn_004b4d50(Ptr, f32, f32, f32, f32) -> Ptr),
        entry!(0x004b_4d10, fn_004b4d10(Ptr, Ptr) -> Ptr),
        entry!(0x004b_4db0, fn_004b4db0(Ptr, Ptr) -> Ptr),
        (
            0x004b_4ea0,
            (|e: &mut Engine, a: &[u32]| fn_004b4ea0(e, Ptr::new(a[0]))) as AbiFn,
        ),
        (
            0x004b_4ee0,
            (|e: &mut Engine, a: &[u32]| fn_004b4ee0(e, Ptr::new(a[0]))) as AbiFn,
        ),
        (
            0x004b_4ec0,
            (|e: &mut Engine, a: &[u32]| fn_004b4ec0(e, Ptr::new(a[0]))) as AbiFn,
        ),
        entry!(0x004b_4f00, fn_004b4f00(Ptr) -> Ptr),
        entry!(0x004b_4ff0, fn_004b4ff0(Ptr, f32, f32, f32, f32) -> Ptr),
        entry!(0x004b_4fc0, fn_004b4fc0(Ptr) -> Ptr),
        entry!(0x004b_4fa0, fn_004b4fa0(Ptr) -> Ptr),
        entry!(0x004b_4f40, fn_004b4f40(Ptr) -> Ptr),
        entry!(0x004b_5020, fn_004b5020(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_this_and_both_words() {
        let mut e = Engine::new();
        e.register(IMPLEMENTATION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004b_4ab0, &args![Ptr::<()>::new(0x100), 7u32, 9u32]);
        let log = e.call_log.take().unwrap();
        let forwarded = log
            .iter()
            .find(|(addr, _)| *addr == IMPLEMENTATION)
            .unwrap();
        assert_eq!(forwarded.1, vec![0x100, 7, 9]);
    }

    #[test]
    fn l68d_004b4f20_passes_this_plus_10_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006815C0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004b_4f20, &args![0x4100u32]).ptr::<()>().addr(),
            0x1445
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006815C0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x10]);
    }
}

#[cfg(test)]
mod physics_step_tests {
    use super::*;

    /// A 16-byte aligned block of `words`.
    fn block(e: &mut Engine, words: &[u32]) -> u32 {
        let raw = e.mem.alloc(4 * words.len() as u32 + 16);
        let aligned = (raw + 15) & !15;
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(aligned + 4 * i as u32, *w);
        }
        aligned
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    fn lanes(e: &Engine, at: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    #[test]
    fn row_address_is_this_plus_sixteen_times_the_index() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004b_4cf0, &args![0x2000u32, 0u32]).u32(), 0x2000);
        assert_eq!(e.call(0x004b_4cf0, &args![0x2000u32, 1u32]).u32(), 0x2010);
        assert_eq!(e.call(0x004b_4cf0, &args![0x2000u32, 2u32]).u32(), 0x2020);
        assert_eq!(
            e.call(0x004b_4cf0, &args![0x10u32, 0x1000_0001u32]).u32(),
            0x20,
            "the shift drops the high bits: 32-bit arithmetic"
        );
        assert_eq!(
            e.call(0x004b_4cf0, &args![0xffff_fff0u32, 2u32]).u32(),
            0x10
        );
    }

    #[test]
    fn matrix_times_vector_gives_the_three_dot_products_and_the_exe_fourth_lane() {
        let mut e = Engine::new();
        // Three rows of four floats; lane 3 of rows 0 and 1 is never used, row 2 supplies lane 3.
        let rows = [
            1.5f32, 2.0, 3.0, 99.0, //
            4.0, 5.0, 6.0, 98.0, //
            7.0, 8.0, 9.0, 0.5,
        ];
        let matrix = block(&mut e, &bits(&rows));
        let vector = block(&mut e, &bits(&[10.0, 100.0, 1000.0, 7777.0]));
        let this = block(&mut e, &[0xdead_beef; 4]);
        e.call(0x004b_4ad0, &args![this, matrix, vector]);
        let expected = [
            1.5f32 * 10.0 + 2.0 * 100.0 + 3.0 * 1000.0,
            4.0 * 10.0 + 5.0 * 100.0 + 6.0 * 1000.0,
            7.0 * 10.0 + 8.0 * 100.0 + 9.0 * 1000.0,
            // lane 3: row 2's lane 1 times x, lane 3 times y and z
            8.0 * 10.0 + 0.5 * 100.0 + 0.5 * 1000.0,
        ];
        assert_eq!(lanes(&e, this), bits(&expected));
    }

    #[test]
    fn wrapper_and_the_function_agree_through_the_registry() {
        let mut e = Engine::new();
        let matrix = block(
            &mut e,
            &bits(&[
                0.0, 1.0, 0.0, 0.0, // x row picks y
                0.0, 0.0, 1.0, 0.0, // y row picks z
                1.0, 0.0, 0.0, 0.0, // z row picks x
            ]),
        );
        let vector = block(&mut e, &bits(&[2.0, 3.0, 4.0, 5.0]));
        let this = block(&mut e, &[0; 4]);
        e.call(0x004b_4ab0, &args![this, matrix, vector]);
        let out: Vec<f32> = lanes(&e, this).iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(&out[..3], &[3.0, 4.0, 2.0]);
        // lane 3: row 2 lane 1 (0.0) * 2 + row 2 lane 3 (0.0) * 3 + 0.0 * 4
        assert_eq!(out[3], 0.0);
    }

    #[test]
    fn a_nan_in_the_matrix_column_wins_over_the_vector_nan_and_is_quieted() {
        let mut e = Engine::new();
        let snan = 0x7f80_0001u32; // a signalling NaN
        let vector_nan = 0x7fc0_0042u32;
        let mut rows = bits(&[
            1.0, 2.0, 3.0, 0.0, //
            4.0, 5.0, 6.0, 0.0, //
            7.0, 8.0, 9.0, 0.0,
        ]);
        rows[0] = snan; // row 0, lane 0 -> column 0, lane 0
        let matrix = block(&mut e, &rows);
        let vector = block(&mut e, &[vector_nan, 1.0f32.to_bits(), 1.0f32.to_bits(), 0]);
        let this = block(&mut e, &[0; 4]);
        e.call(0x004b_4ad0, &args![this, matrix, vector]);
        let out = lanes(&e, this);
        // x product lane 0: destination (matrix) NaN, quieted; the later adds keep a NaN dest.
        assert_eq!(out[0], snan | 0x0040_0000);
        // lanes 1 and 2: column 0 is finite, the vector's NaN x is the source: it wins, quieted.
        assert_eq!(out[1], vector_nan);
        assert_eq!(out[2], vector_nan);
    }

    #[test]
    fn only_this_is_written() {
        let mut e = Engine::new();
        let matrix = block(&mut e, &bits(&[1.0; 12]));
        let vector = block(&mut e, &bits(&[1.0; 4]));
        let this = block(&mut e, &[0x1111_1111; 8]);
        e.call(0x004b_4ad0, &args![this, matrix, vector]);
        assert_eq!(e.mem.u32(this + 16), 0x1111_1111);
        assert_eq!(lanes(&e, matrix)[0], 1.0f32.to_bits());
        assert_eq!(lanes(&e, this)[..3], bits(&[3.0, 3.0, 3.0])[..]);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_004b4e50_scales_three_components_and_clears_the_fourth() {
        let mut e = Engine::new();
        e.map(COMPONENT_SCALE, 4);
        e.mem.set_f32(COMPONENT_SCALE, 2.0);
        let source = e.mem.alloc(0x10);
        e.mem.set_f32(source, 1.5);
        e.mem.set_f32(source + 4, -4.0);
        e.mem.set_f32(source + 8, 0.25);
        e.mem.set_f32(source + 0xc, 99.0);
        let this = e.mem.alloc(0x10);
        e.mem.set_f32(this + 0xc, 7.0);
        assert_eq!(e.call(0x004b_4e50, &args![this, source]).u32(), this);
        assert_eq!(e.mem.f32(this), 3.0);
        assert_eq!(e.mem.f32(this + 4), -8.0);
        assert_eq!(e.mem.f32(this + 8), 0.5);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // The source is left alone.
        assert_eq!(e.mem.f32(source + 0xc), 99.0);
    }

    #[test]
    fn at_004b4e50_scaling_in_place_reads_each_component_before_writing_it() {
        let mut e = Engine::new();
        e.map(COMPONENT_SCALE, 4);
        e.mem.set_f32(COMPONENT_SCALE, 0.5);
        let vector = e.mem.alloc(0x10);
        e.mem.set_f32(vector, 8.0);
        e.mem.set_f32(vector + 4, 6.0);
        e.mem.set_f32(vector + 8, 4.0);
        e.call(0x004b_4e50, &args![vector, vector]);
        assert_eq!(e.mem.f32(vector), 4.0);
        assert_eq!(e.mem.f32(vector + 4), 3.0);
        assert_eq!(e.mem.f32(vector + 8), 2.0);
        assert_eq!(e.mem.u32(vector + 0xc), 0);
    }

    #[test]
    fn at_004b4e50_nan_components_come_out_quiet() {
        let mut e = Engine::new();
        e.map(COMPONENT_SCALE, 4);
        e.mem.set_f32(COMPONENT_SCALE, 2.0);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x7f80_0001);
        e.mem.set_u32(source + 4, 0xff80_0001);
        e.mem.set_f32(source + 8, 1.0);
        let this = e.mem.alloc(0x10);
        e.call(0x004b_4e50, &args![this, source]);
        assert_eq!(e.mem.u32(this), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 4), 0xffc0_0001);
        assert_eq!(e.mem.f32(this + 8), 2.0);
    }

    #[test]
    fn at_004b4e50_a_nan_scale_wins_over_a_number() {
        let mut e = Engine::new();
        e.map(COMPONENT_SCALE, 4);
        e.mem.set_u32(COMPONENT_SCALE, 0x7fc0_0123);
        let source = e.mem.alloc(0x10);
        e.mem.set_f32(source, 1.0);
        let this = e.mem.alloc(0x10);
        e.call(0x004b_4e50, &args![this, source]);
        assert_eq!(e.mem.u32(this), 0x7fc0_0123);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    fn words(e: &Engine, p: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(p + 4 * i)).collect()
    }

    #[test]
    fn user_report_runs_only_when_the_name_is_read() {
        for ok in [0u32, 1] {
            let mut e = Engine::new();
            e.register_double(GET_USER_NAME, move |e, a| {
                assert_eq!(e.mem.u32(a[1]), 0x104);
                ok.into_ret()
            });
            e.register(ERROR, |_, _| Ret::default());
            e.register(NAME_LOOKUP, |_, _| 0x7000u32.into_ret());
            e.register(SPRINTF, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x004b_4a30, &args![0x1234u32]);
            let log = log_of(&mut e, 0x004b_4a30);
            assert_eq!(log[0].0, GET_USER_NAME);
            assert_eq!(log[0].1[0], USER_NAME);
            if ok == 0 {
                assert_eq!(log.len(), 1);
                continue;
            }
            let line = log[3].1[0];
            assert_eq!(
                log[1..].to_vec(),
                vec![
                    (ERROR, vec![PERCENT_S, 0x1234]),
                    (NAME_LOOKUP, vec![USER_NAME, 0]),
                    (SPRINTF, vec![line, COMPUTER_NAME_FORMAT, 0x7000]),
                    (ERROR, vec![line]),
                ]
            );
        }
    }

    #[test]
    fn four_float_stores_copy_raw_or_quiet() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let r = e.call(0x004b_4d50, &[this, 0x7f80_0001, 2, 3, 4]);
        assert_eq!(r.u32(), this);
        assert_eq!(words(&e, this), vec![0x7f80_0001, 2, 3, 4]);
        let r = e.call(0x004b_4ff0, &[this, 0x7f80_0001, 2, 3, 4]);
        assert_eq!(r.u32(), this);
        assert_eq!(words(&e, this), vec![0x7fc0_0001, 2, 3, 4]);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x7f80_0005);
        assert_eq!(e.call(0x004b_4d10, &args![this, out]).u32(), out);
        assert_eq!(words(&e, out), vec![0x7fc0_0005, 2, 3, 4]);
        assert_eq!(e.call(0x004b_4fc0, &args![this]).u32(), this);
        assert_eq!(words(&e, this), vec![0; 4]);
        e.mem.set_u32(this + 8, 9);
        assert_eq!(e.call(0x004b_4fa0, &args![this]).u32(), this);
        assert_eq!(words(&e, this), vec![0; 4]);
    }

    #[test]
    fn element_values_fill_the_four_floats() {
        let mut e = Engine::new();
        e.register(ELEMENT_GET, |_, a| (0x100 + a[2]).into_ret());
        e.register(ELEMENT_VALUE, |_, a| Ret {
            st0: f64::from(a[0] - 0x100) + 0.5,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_4db0, &args![this, 0x9000u32]).u32(), this);
        let gets: Vec<_> = log_of(&mut e, 0x004b_4db0)
            .into_iter()
            .filter(|(a, _)| *a == ELEMENT_GET)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(gets.len(), 4);
        for (i, words) in gets.iter().enumerate() {
            assert_eq!(words[0], 0x9000);
            assert_eq!(words[2], i as u32);
        }
        // The four locals are distinct 16-byte blocks, one below the other.
        assert_eq!(gets[0][1] - gets[1][1], 0x10);
        let want: Vec<u32> = [0.5f32, 1.5, 2.5, 3.5]
            .iter()
            .map(|v| v.to_bits())
            .collect();
        assert_eq!(words(&e, this), want);
    }

    #[test]
    fn wrappers_pass_the_callee_result_on() {
        let mut e = Engine::new();
        e.register(OBJECT_GET, |_, a| (a[0] + 1).into_ret());
        e.register(MOTION_GET_MASS, |_, a| Ret {
            eax: a[0],
            st0: 2.25,
            ..Ret::default()
        });
        e.register(CALLEE_006815C0, |_, a| (a[0] + 2).into_ret());
        e.register(CALLEE_00436AA0, |_, a| Ret {
            eax: a[0] * 2,
            st0: 1.5,
            ..Ret::default()
        });
        let r = e.call(0x004b_4ea0, &args![0x100u32]);
        assert_eq!((r.eax, r.st0), (0x101, 2.25));
        let r = e.call(0x004b_4ee0, &args![0x100u32]);
        assert_eq!((r.eax, r.st0), (0x224, 1.5));
        let r = e.call(0x004b_4ec0, &args![0x100u32]);
        assert_eq!((r.eax, r.st0), (0x226, 1.5));
        assert_eq!(e.call(0x004b_4f00, &args![0x100u32]).u32(), 0x113);
    }

    #[test]
    fn constructor_zeroes_the_members_and_copies_the_defaults() {
        let mut e = Engine::new();
        for (constant, bits) in [
            (DEFAULT_30, 0.5f32.to_bits()),
            (DEFAULT_34, 0.3f32.to_bits()),
            (DEFAULT_38, 250.0f32.to_bits()),
            (DEFAULT_3C, 0x7f80_0003),
        ] {
            e.map(constant, 4);
            e.mem.set_u32(constant, bits);
        }
        e.register(BASE_CONSTRUCTOR_00537BB0, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x40);
        for i in 0..0x10 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_4f40, &args![this]).u32(), this);
        assert_eq!(
            log_of(&mut e, 0x004b_4f40),
            vec![(BASE_CONSTRUCTOR_00537BB0, vec![this])]
        );
        assert_eq!(words(&e, this + 0x10), vec![0; 4]);
        assert_eq!(words(&e, this + 0x20), vec![0; 4]);
        assert_eq!(
            words(&e, this + 0x30),
            vec![
                0.5f32.to_bits(),
                0.3f32.to_bits(),
                250.0f32.to_bits(),
                0x7fc0_0003
            ]
        );
        assert_eq!(e.mem.u32(this), 0xffff_ffff);
    }

    #[test]
    fn clears_the_first_word_after_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006815C0, |e, a| {
            e.mem.set_u32(a[0], 0x55);
            a[0].into_ret()
        });
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x004b_5020, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
    }
}

#[cfg(test)]
mod transform_points_tests {
    use super::*;

    #[test]
    fn fn_004b4a10_calls_the_function_pointer_with_a_16_bit_count() {
        let mut e = Engine::new();
        e.map(0x011a_a000, 0x1000);
        e.set_global(TRANSFORM_POINTS_POINTER, 0x0300_0001u32);
        e.register(0x0300_0001, |_, _| 0x77u32.into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(
            0x004b_4a10,
            &args![0xabcd_0005u32, 0x10u32, 0x20u32, 0x30u32],
        );
        assert_eq!(r.u32(), 0x77);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0300_0001, vec![5, 0x10, 0x20, 0x30]));
    }
}
