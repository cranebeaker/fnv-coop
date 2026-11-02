//! `BSShader/run_00b815e0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The two per-pass flag tables `00b815e0` reads (one word per pass).
const PASS_FLAGS_011FF248: u32 = 0x011f_f248;
const PASS_FLAGS_011FF120: u32 = 0x011f_f120;
/// The flag bytes it writes: bits 1 to 3 of the first table go to
/// `011ff118`..`011ff11a`, bits 1 to 6 of the second to `011ff110`..`011ff115`.
const FIRST_TABLE_BYTES: u32 = 0x011f_f118;
const SECOND_TABLE_BYTES: u32 = 0x011f_f110;
/// The nine objects whose byte at `+8` it sets (pointers at `011ff4a0`..`011ff4c0`).
const TOGGLE_OBJECTS: u32 = 0x011f_f4a0;

fn table_bit(e: &Engine, table: u32, pass: u32, bit: u32) -> u8 {
    let word = e.mem.u32(table.wrapping_add(pass.wrapping_mul(4)));
    ((word >> bit) & 1) as u8
}

fn toggle_object(e: &Engine, index: u32) -> u32 {
    e.mem.u32(TOGGLE_OBJECTS + 4 * index)
}

// Translated from 00b815e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(pass)`: copies the pass's flag bits
/// into the flag bytes and the toggle objects. The first three objects
/// get 1 (each when it exists, before the matching byte gets bit 1, 2 or 3
/// of the first table); the fourth gets bit 1 of the second table; then
/// bytes `011ff110`..`011ff115` get bits 1 to 6 of the second table, the
/// objects five to nine (when they exist) bits 2 to 6. Every bit is read
/// from the table again, as the code does.
pub fn fn_00b815e0(e: &mut Engine, pass: u32) {
    for (index, bit) in [(0u32, 1u32), (1, 2), (2, 3)] {
        let object = toggle_object(e, index);
        if object != 0 {
            e.mem.set_u8(object + 8, 1);
        }
        let value = table_bit(e, PASS_FLAGS_011FF248, pass, bit);
        e.mem.set_u8(FIRST_TABLE_BYTES + index, value);
    }
    let object = toggle_object(e, 3);
    if object != 0 {
        let value = table_bit(e, PASS_FLAGS_011FF120, pass, 1);
        e.mem.set_u8(object + 8, value);
    }
    for index in 0..6u32 {
        let value = table_bit(e, PASS_FLAGS_011FF120, pass, index + 1);
        e.mem.set_u8(SECOND_TABLE_BYTES + index, value);
        if index < 5 {
            let object = toggle_object(e, 4 + index);
            if object != 0 {
                let value = table_bit(e, PASS_FLAGS_011FF120, pass, index + 2);
                e.mem.set_u8(object + 8, value);
            }
        }
    }
}

// Translated from 00b81730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x124 of the vtable `010af9e0` (the one `WaterShader`'s
/// constructor stores at `00b86c1a`; `thiscall`, no stack words). For each of the 0x4a words
/// from `+0x6c` in order, calls the object's own slot +0x128 with the word.
pub fn fn_00b81730(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in 0..0x4au32 {
        let word = e.mem.u32(base + 0x6c + 4 * index);
        e.vcall(base, 0x128, &args![word]);
    }
}
/// The pass state word (`0126f74c`) the slot +0xf0 functions set.
const PASS_STATE_0126F74C: u32 = 0x0126_f74c;
/// `00b81760`'s jump table for the passes 0x23f to 0x2ce (the byte table at `00b81ccc` into the
/// 74 code addresses at `00b81ba4`), read from the exe.
const TARGETS_00B81BA4: [u32; 74] = [
    0x00b8_1975,
    0x00b8_1983,
    0x00b8_1957,
    0x00b8_1b76,
    0x00b8_1782,
    0x00b8_1791,
    0x00b8_17a0,
    0x00b8_17ae,
    0x00b8_17bd,
    0x00b8_17cc,
    0x00b8_17da,
    0x00b8_17e9,
    0x00b8_17f8,
    0x00b8_1806,
    0x00b8_1815,
    0x00b8_1824,
    0x00b8_1832,
    0x00b8_1841,
    0x00b8_1850,
    0x00b8_185e,
    0x00b8_186d,
    0x00b8_1992,
    0x00b8_19be,
    0x00b8_19af,
    0x00b8_19a1,
    0x00b8_19cd,
    0x00b8_19f9,
    0x00b8_19ea,
    0x00b8_19db,
    0x00b8_1a07,
    0x00b8_1a33,
    0x00b8_1a25,
    0x00b8_1a16,
    0x00b8_1a42,
    0x00b8_1a6e,
    0x00b8_1a5f,
    0x00b8_1a51,
    0x00b8_1a7d,
    0x00b8_187c,
    0x00b8_1887,
    0x00b8_1893,
    0x00b8_189f,
    0x00b8_18aa,
    0x00b8_18b9,
    0x00b8_18c8,
    0x00b8_18d6,
    0x00b8_18e5,
    0x00b8_18f1,
    0x00b8_18ff,
    0x00b8_190e,
    0x00b8_191d,
    0x00b8_192b,
    0x00b8_193a,
    0x00b8_1949,
    0x00b8_1966,
    0x00b8_1b93,
    0x00b8_1a8b,
    0x00b8_1a9a,
    0x00b8_1aa9,
    0x00b8_1ab7,
    0x00b8_1ac6,
    0x00b8_1ad5,
    0x00b8_1ae3,
    0x00b8_1af2,
    0x00b8_1b01,
    0x00b8_1b0f,
    0x00b8_1b1e,
    0x00b8_1b2d,
    0x00b8_1b3b,
    0x00b8_1b4a,
    0x00b8_1b59,
    0x00b8_1b67,
    0x00b8_1b85,
    0x00b8_1b9f,
];
const BYTES_00B81CCC: [u8; 144] = [
    0, 1, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72,
];
/// The case at each code address of `00b81760`: the shader word (offset) it puts in `0126f74c`.
const CASES_00B81760: [(u32, u32); 73] = [
    (0x00b8_1782, 0xb0),
    (0x00b8_1791, 0xbc),
    (0x00b8_17a0, 0xb8),
    (0x00b8_17ae, 0xb4),
    (0x00b8_17bd, 0xc0),
    (0x00b8_17cc, 0xcc),
    (0x00b8_17da, 0xc8),
    (0x00b8_17e9, 0xc4),
    (0x00b8_17f8, 0xd0),
    (0x00b8_1806, 0xdc),
    (0x00b8_1815, 0xd8),
    (0x00b8_1824, 0xd4),
    (0x00b8_1832, 0xe0),
    (0x00b8_1841, 0xec),
    (0x00b8_1850, 0xe8),
    (0x00b8_185e, 0xe4),
    (0x00b8_186d, 0x100),
    (0x00b8_187c, 0x6c),
    (0x00b8_1887, 0x78),
    (0x00b8_1893, 0x74),
    (0x00b8_189f, 0x70),
    (0x00b8_18aa, 0x8c),
    (0x00b8_18b9, 0x98),
    (0x00b8_18c8, 0x94),
    (0x00b8_18d6, 0x90),
    (0x00b8_18e5, 0x7c),
    (0x00b8_18f1, 0x80),
    (0x00b8_18ff, 0x84),
    (0x00b8_190e, 0x88),
    (0x00b8_191d, 0x9c),
    (0x00b8_192b, 0xa8),
    (0x00b8_193a, 0xa4),
    (0x00b8_1949, 0xa0),
    (0x00b8_1957, 0xf8),
    (0x00b8_1966, 0xfc),
    (0x00b8_1975, 0xf0),
    (0x00b8_1983, 0xf4),
    (0x00b8_1992, 0x148),
    (0x00b8_19a1, 0x154),
    (0x00b8_19af, 0x150),
    (0x00b8_19be, 0x14c),
    (0x00b8_19cd, 0x158),
    (0x00b8_19db, 0x164),
    (0x00b8_19ea, 0x160),
    (0x00b8_19f9, 0x15c),
    (0x00b8_1a07, 0x168),
    (0x00b8_1a16, 0x174),
    (0x00b8_1a25, 0x170),
    (0x00b8_1a33, 0x16c),
    (0x00b8_1a42, 0x178),
    (0x00b8_1a51, 0x184),
    (0x00b8_1a5f, 0x180),
    (0x00b8_1a6e, 0x17c),
    (0x00b8_1a7d, 0x190),
    (0x00b8_1a8b, 0x104),
    (0x00b8_1a9a, 0x110),
    (0x00b8_1aa9, 0x10c),
    (0x00b8_1ab7, 0x108),
    (0x00b8_1ac6, 0x124),
    (0x00b8_1ad5, 0x130),
    (0x00b8_1ae3, 0x12c),
    (0x00b8_1af2, 0x128),
    (0x00b8_1b01, 0x114),
    (0x00b8_1b0f, 0x118),
    (0x00b8_1b1e, 0x11c),
    (0x00b8_1b2d, 0x120),
    (0x00b8_1b3b, 0x134),
    (0x00b8_1b4a, 0x140),
    (0x00b8_1b59, 0x13c),
    (0x00b8_1b67, 0x138),
    (0x00b8_1b76, 0x188),
    (0x00b8_1b85, 0x18c),
    (0x00b8_1b93, 0xac),
];
/// `00b81d60`'s jump table for the passes 0x23f to 0x2ce (the byte table at `00b82418` into the
/// 74 code addresses at `00b822f0`), read from the exe.
const TARGETS_00B822F0: [u32; 74] = [
    0x00b8_201b,
    0x00b8_202e,
    0x00b8_1ff5,
    0x00b8_22b4,
    0x00b8_1d86,
    0x00b8_1d99,
    0x00b8_1dac,
    0x00b8_1dbf,
    0x00b8_1dd2,
    0x00b8_1de5,
    0x00b8_1df8,
    0x00b8_1e0b,
    0x00b8_1e1e,
    0x00b8_1e31,
    0x00b8_1e44,
    0x00b8_1e57,
    0x00b8_1e6a,
    0x00b8_1e7d,
    0x00b8_1e90,
    0x00b8_1ea3,
    0x00b8_1eb6,
    0x00b8_2041,
    0x00b8_207a,
    0x00b8_2067,
    0x00b8_2054,
    0x00b8_208d,
    0x00b8_20c6,
    0x00b8_20b3,
    0x00b8_20a0,
    0x00b8_20d9,
    0x00b8_2112,
    0x00b8_20ff,
    0x00b8_20ec,
    0x00b8_2125,
    0x00b8_215e,
    0x00b8_214b,
    0x00b8_2138,
    0x00b8_2171,
    0x00b8_1ec9,
    0x00b8_1ed8,
    0x00b8_1eeb,
    0x00b8_1efe,
    0x00b8_1f11,
    0x00b8_1f24,
    0x00b8_1f37,
    0x00b8_1f4a,
    0x00b8_1f5d,
    0x00b8_1f70,
    0x00b8_1f83,
    0x00b8_1f96,
    0x00b8_1fa9,
    0x00b8_1fbc,
    0x00b8_1fcf,
    0x00b8_1fe2,
    0x00b8_2008,
    0x00b8_22da,
    0x00b8_2184,
    0x00b8_2197,
    0x00b8_21aa,
    0x00b8_21bd,
    0x00b8_21d0,
    0x00b8_21e3,
    0x00b8_21f6,
    0x00b8_2209,
    0x00b8_221c,
    0x00b8_222f,
    0x00b8_2242,
    0x00b8_2255,
    0x00b8_2268,
    0x00b8_227b,
    0x00b8_228e,
    0x00b8_22a1,
    0x00b8_22c7,
    0x00b8_22df,
];
const BYTES_00B82418: [u8; 144] = [
    0, 1, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72,
];
/// The case at each code address of `00b81d60`: the pass index it hands `00b815e0`.
const CASES_00B81D60: [(u32, u32); 72] = [
    (0x00b8_1d86, 0x11),
    (0x00b8_1d99, 0x14),
    (0x00b8_1dac, 0x13),
    (0x00b8_1dbf, 0x12),
    (0x00b8_1dd2, 0x15),
    (0x00b8_1de5, 0x18),
    (0x00b8_1df8, 0x17),
    (0x00b8_1e0b, 0x16),
    (0x00b8_1e1e, 0x19),
    (0x00b8_1e31, 0x1c),
    (0x00b8_1e44, 0x1b),
    (0x00b8_1e57, 0x1a),
    (0x00b8_1e6a, 0x1d),
    (0x00b8_1e7d, 0x20),
    (0x00b8_1e90, 0x1f),
    (0x00b8_1ea3, 0x1e),
    (0x00b8_1eb6, 0x25),
    (0x00b8_1ed8, 0x3),
    (0x00b8_1eeb, 0x2),
    (0x00b8_1efe, 0x1),
    (0x00b8_1f11, 0x8),
    (0x00b8_1f24, 0xb),
    (0x00b8_1f37, 0xa),
    (0x00b8_1f4a, 0x9),
    (0x00b8_1f5d, 0x4),
    (0x00b8_1f70, 0x5),
    (0x00b8_1f83, 0x6),
    (0x00b8_1f96, 0x7),
    (0x00b8_1fa9, 0xc),
    (0x00b8_1fbc, 0xf),
    (0x00b8_1fcf, 0xe),
    (0x00b8_1fe2, 0xd),
    (0x00b8_1ff5, 0x23),
    (0x00b8_2008, 0x24),
    (0x00b8_201b, 0x21),
    (0x00b8_202e, 0x22),
    (0x00b8_2041, 0x37),
    (0x00b8_2054, 0x3a),
    (0x00b8_2067, 0x39),
    (0x00b8_207a, 0x38),
    (0x00b8_208d, 0x3b),
    (0x00b8_20a0, 0x3e),
    (0x00b8_20b3, 0x3d),
    (0x00b8_20c6, 0x3c),
    (0x00b8_20d9, 0x3f),
    (0x00b8_20ec, 0x42),
    (0x00b8_20ff, 0x41),
    (0x00b8_2112, 0x40),
    (0x00b8_2125, 0x43),
    (0x00b8_2138, 0x46),
    (0x00b8_214b, 0x45),
    (0x00b8_215e, 0x44),
    (0x00b8_2171, 0x49),
    (0x00b8_2184, 0x26),
    (0x00b8_2197, 0x29),
    (0x00b8_21aa, 0x28),
    (0x00b8_21bd, 0x27),
    (0x00b8_21d0, 0x2e),
    (0x00b8_21e3, 0x31),
    (0x00b8_21f6, 0x30),
    (0x00b8_2209, 0x2f),
    (0x00b8_221c, 0x2a),
    (0x00b8_222f, 0x2b),
    (0x00b8_2242, 0x2c),
    (0x00b8_2255, 0x2d),
    (0x00b8_2268, 0x32),
    (0x00b8_227b, 0x35),
    (0x00b8_228e, 0x34),
    (0x00b8_22a1, 0x33),
    (0x00b8_22b4, 0x47),
    (0x00b8_22c7, 0x48),
    (0x00b8_22da, 0x10),
];

/// The value of the case at `target` in a (code address, value) list.
fn case_value(cases: &[(u32, u32)], target: u32) -> Option<u32> {
    cases
        .iter()
        .find(|(address, _)| *address == target)
        .map(|(_, value)| *value)
}

// Translated from 00b81760 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00b81760 to the `RET 4` at 00b81b9f, its jump tables after
/// it): slot 60 (+0xf0) of the water shader's vtable `010af9e0`, `thiscall` with one stack word:
/// the pass. For the passes 0x23f to 0x2ce with a case in the jump table (read from the exe) puts
/// the shader's word for that pass (+0x6c to +0x190) in `0126f74c`; other passes: nothing.
pub fn fn_00b81760(e: &mut Engine, this: Ptr, pass: u32) {
    let case = pass.wrapping_sub(0x23f);
    if case > 0x8f {
        return;
    }
    let target = TARGETS_00B81BA4[usize::from(BYTES_00B81CCC[case as usize])];
    // 00b81b9f, the default, only returns.
    if let Some(offset) = case_value(&CASES_00B81760, target) {
        let word = e.mem.u32(this.addr() + offset);
        e.set_global::<u32>(PASS_STATE_0126F74C, word);
    }
}

// Translated from 00b81d60 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00b81d60 to the `RET 4` at 00b822ea, its jump tables after
/// it): slot 63 (+0xfc) of the water shader's vtable `010af9e0`, `thiscall` with one stack word
/// (the pass; `this` is not read). For the passes 0x23f to 0x2ce the jump table (read from the
/// exe) picks a pass index (1 to 0x48), any other pass (and the table's default `00b822df`) 0;
/// [`fn_00b815e0`] runs with the index and a 0 word (`cdecl`, both pushed).
pub fn fn_00b81d60(e: &mut Engine, _this: Ptr, pass: u32) {
    let case = pass.wrapping_sub(0x23f);
    let index = if case > 0x8f {
        0
    } else {
        let target = TARGETS_00B822F0[usize::from(BYTES_00B82418[case as usize])];
        case_value(&CASES_00B81D60, target).unwrap_or(0)
    };
    e.call(0x00b8_15e0, &args![index, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b815e0, fn_00b815e0(u32)),
        entry!(0x00b8_1730, fn_00b81730(Ptr)),
        entry!(0x00b8_1760, fn_00b81760(Ptr, u32)),
        entry!(0x00b8_1d60, fn_00b81d60(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_the_pass_bits_into_the_bytes_and_objects() {
        let mut e = Engine::new();
        e.map(0x011f_f000, 0x1000);
        let objects: Vec<u32> = (0..9).map(|_| e.mem.alloc(0xc)).collect();
        for (index, &object) in objects.iter().enumerate() {
            if index != 5 {
                e.set_global(TOGGLE_OBJECTS + 4 * index as u32, object);
            }
        }
        e.set_global(PASS_FLAGS_011FF248 + 8, 0b1010u32);
        e.set_global(PASS_FLAGS_011FF120 + 8, 0b101_0100u32);
        e.call(0x00b8_15e0, &args![2u32]);
        let bytes = |e: &Engine, at: u32, count: u32| -> Vec<u8> {
            (0..count).map(|index| e.mem.u8(at + index)).collect()
        };
        assert_eq!(bytes(&e, FIRST_TABLE_BYTES, 3), vec![1, 0, 1]);
        assert_eq!(bytes(&e, SECOND_TABLE_BYTES, 6), vec![0, 1, 0, 1, 0, 1]);
        let flags: Vec<u8> = objects.iter().map(|&object| e.mem.u8(object + 8)).collect();
        assert_eq!(flags, vec![1, 1, 1, 0, 1, 0, 1, 0, 1]);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00b81730_passes_each_word_to_slot_0x128() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x4b];
        slots[0x4a] = 0x0f00_0128;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0128, |_, _| Ret::default());
        let this = e.mem.alloc(0x6c + 0x4a * 4);
        e.mem.set_u32(this, 0x0f00_1000);
        for i in 0..0x4a {
            e.mem.set_u32(this + 0x6c + 4 * i, 0x100 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b8_1730, &args![this]);
        let seen: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0f00_0128)
            .map(|(_, w)| w)
            .collect();
        let expected: Vec<Vec<u32>> = (0..0x4a).map(|i| vec![this, 0x100 + i]).collect();
        assert_eq!(seen, expected);
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00b81760_puts_the_pass_word_in_the_state() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let this = e.mem.alloc(0x194);
        for offset in (0x6cu32..0x194).step_by(4) {
            e.mem.set_u32(this + offset, 0x1000 + offset);
        }
        for (pass, offset) in [
            (0x23fu32, 0xf0u32),
            (0x240, 0xf4),
            (0x28c, 0xb8),
            (0x2a3, 0x168),
            (0x2ce, 0x18c),
        ] {
            e.set_global::<u32>(PASS_STATE_0126F74C, 0);
            e.call(0x00b8_1760, &args![this, pass]);
            assert_eq!(
                e.global::<u32>(PASS_STATE_0126F74C),
                0x1000 + offset,
                "{pass:#x}"
            );
        }
        for pass in [0x241u32, 0x267, 0x23e, 0x2cf, 0] {
            e.set_global::<u32>(PASS_STATE_0126F74C, 7);
            e.call(0x00b8_1760, &args![this, pass]);
            assert_eq!(e.global::<u32>(PASS_STATE_0126F74C), 7, "{pass:#x}");
        }
    }

    #[test]
    fn fn_00b81d60_hands_the_pass_index_to_00b815e0() {
        let mut e = Engine::new();
        e.register(0x00b8_15e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let passes = [
            0x23fu32, 0x240, 0x28c, 0x2a3, 0x2ce, 0x241, 0x267, 0x23e, 0x2cf,
        ];
        for pass in passes {
            e.call(0x00b8_1d60, &args![0u32, pass]);
        }
        let seen: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x00b8_15e0)
            .map(|(_, w)| w)
            .collect();
        let expected: Vec<Vec<u32>> = [0x21u32, 0x22, 0x13, 0x3f, 0x48, 0, 0, 0, 0]
            .iter()
            .map(|index| vec![*index, 0])
            .collect();
        assert_eq!(seen, expected);
    }
}
