//! `NiXenonRenderer/run_00e7acb0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The four-character codes `00e7acb0` answers 0 for above `"MET1"` (their bytes read as ASCII:
/// `"DXT1"` to `"DXT5"`, `"UYVY"`, `"YUY2"`, `"RGBG"`, `"GRGB"`).
const CODES_ANSWERING_0: [u32; 9] = [
    0x3154_5844,
    0x3254_5844,
    0x3354_5844,
    0x3454_5844,
    0x3554_5844,
    0x5956_5955,
    0x3259_5559,
    0x4247_5247,
    0x4742_4752,
];
/// `"MET1"`, the highest code of the compare chain's lower half.
const CODE_MET1: i32 = 0x3154_454d;

// Translated from 00e7acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7acb0`), `cdecl`, one word (a format code; the renderer
/// passes Direct3D format numbers): the bit size it answers. Codes above `"MET1"` (signed) give
/// 0 for the nine four-character codes of [`CODES_ANSWERING_0`] and -1 otherwise; `"MET1"`
/// gives 0; below it, codes above 0x75 (unsigned, so negative ones too) give -1, and 0 to 0x75
/// go through the switch's byte map at `00e7ad64` and jump table at `00e7ad44` (read in
/// Ghidra): 0x14 gives 0x18 (24); 0x15, 0x16, 0x1f to 0x21, 0x23, 0x3e to 0x40, 0x43, 0x47,
/// 0x4b, 0x4d, 0x4f, 0x52, 0x53, 0x66, 0x70 and 0x72 give 0x20 (32); 0x17 to 0x1a, 0x1d, 0x1e,
/// 0x22, 0x28, 0x33, 0x3c, 0x3d, 0x46, 0x49, 0x50, 0x51, 0x65, 0x6f and 0x75 give 0x10 (16);
/// 0x1b, 0x1c, 0x29, 0x32 and 0x34 give 8; 0x24, 0x6e, 0x71 and 0x73 give 0x40 (64); 0x74
/// gives 0x80 (128); 0 and 0x64 give 0; the rest -1.
pub fn fn_00e7acb0(_e: &mut Engine, format: i32) -> i32 {
    if format > CODE_MET1 {
        return if CODES_ANSWERING_0.contains(&(format as u32)) {
            0
        } else {
            -1
        };
    }
    if format == CODE_MET1 {
        return 0;
    }
    match format as u32 {
        0 | 0x64 => 0,
        0x14 => 0x18,
        0x15
        | 0x16
        | 0x1f..=0x21
        | 0x23
        | 0x3e..=0x40
        | 0x43
        | 0x47
        | 0x4b
        | 0x4d
        | 0x4f
        | 0x52
        | 0x53
        | 0x66
        | 0x70
        | 0x72 => 0x20,
        0x17..=0x1a
        | 0x1d
        | 0x1e
        | 0x22
        | 0x28
        | 0x33
        | 0x3c
        | 0x3d
        | 0x46
        | 0x49
        | 0x50
        | 0x51
        | 0x65
        | 0x6f
        | 0x75 => 0x10,
        0x1b | 0x1c | 0x29 | 0x32 | 0x34 => 8,
        0x24 | 0x6e | 0x71 | 0x73 => 0x40,
        0x74 => 0x80,
        _ => -1,
    }
}

/// `00e7ade0`'s callees: the hash map lookup `00a53c30` (`thiscall` on the map, key and the
/// address of the found value; answers a byte), the nine Direct3D depth/stencil formats it
/// tries (`011bf140`, nine words) and the format's depth and stencil bit counts `00e7a8b0`
/// (unnamed; `cdecl`: the format, the addresses of the two counts).
const MAP_GET_AT_00A53C30: u32 = 0x00a5_3c30;
const DEPTH_FORMATS_011BF140: u32 = 0x011b_f140;
const FORMAT_BITS_00E7A8B0: u32 = 0x00e7_a8b0;

// Translated from 00e7ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7acb0`), `thiscall` on the display-format record with
/// three words: the back buffer format, the requested depth bits and the requested stencil
/// bits. Looks the back buffer format up in the map at +8 (`00a53c30`; not found: 0). In the
/// found entry, nine flag bytes every four bytes from +0x24 say which of the nine formats at
/// `011bf140` it allows; for each allowed one, `00e7a8b0` gives its depth and stencil bits, and
/// it becomes the answer when its depth distance to the request is not above the best one's
/// and its stencil distance is not above the best one's either (both distances are the
/// wrapping 32-bit absolute differences, compared signed; the best starts at 0x7fffffff for
/// both). Answers the chosen format, 0 when none was chosen.
pub fn fn_00e7ade0(
    e: &mut Engine,
    this: Ptr,
    back_buffer: u32,
    depth_bits: i32,
    stencil_bits: i32,
) -> u32 {
    let entry = e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        let found = e
            .call(
                MAP_GET_AT_00A53C30,
                &args![this.addr().wrapping_add(8), back_buffer, out],
            )
            .u8();
        if found != 0 {
            Some(e.mem.u32(out.addr()))
        } else {
            None
        }
    });
    let Some(entry) = entry else {
        return 0;
    };
    let distance = |value: i32, wanted: i32| value.wrapping_sub(wanted).wrapping_abs();
    let mut flag = entry.wrapping_add(0x24);
    let mut chosen = 0u32;
    let mut best_depth = 0x7fff_ffffi32;
    let mut best_stencil = 0x7fff_ffffi32;
    for slot in 0..9u32 {
        if e.mem.u8(flag) != 0 {
            let format = e.mem.u32(DEPTH_FORMATS_011BF140 + 4 * slot);
            let (depth, stencil) = e.with_stack(8, |e, counts| {
                e.mem.set_u32(counts.addr(), 0);
                e.mem.set_u32(counts.addr() + 4, 0);
                e.call(
                    FORMAT_BITS_00E7A8B0,
                    &args![format, counts, counts.addr() + 4],
                );
                (
                    e.mem.u32(counts.addr()) as i32,
                    e.mem.u32(counts.addr() + 4) as i32,
                )
            });
            if distance(best_depth, depth_bits) >= distance(depth, depth_bits)
                && distance(best_stencil, stencil_bits) >= distance(stencil, stencil_bits)
            {
                best_depth = depth;
                best_stencil = stencil;
                chosen = format;
            }
        }
        flag = flag.wrapping_add(4);
    }
    chosen
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_acb0, fn_00e7acb0(i32) -> i32),
        entry!(0x00e7_ade0, fn_00e7ade0(Ptr, u32, i32, i32) -> u32),
    ]
}

#[cfg(test)]
mod b000s_tests {
    use super::*;

    fn size(e: &mut Engine, format: u32) -> i32 {
        e.call(0x00e7_acb0, &args![format]).i32()
    }

    #[test]
    fn fn_00e7acb0_answers_the_bit_sizes() {
        let mut e = Engine::new();
        let cases: [(u32, i32); 16] = [
            (0, 0),
            (0x64, 0),
            (0x14, 24),
            (0x15, 32),
            (0x72, 32),
            (0x17, 16),
            (0x75, 16),
            (0x1b, 8),
            (0x34, 8),
            (0x24, 64),
            (0x73, 64),
            (0x74, 128),
            (0x01, -1),
            (0x76, -1),
            (0x8000_0000, -1),
            (0x2a, -1),
        ];
        for (format, expected) in cases {
            assert_eq!(size(&mut e, format), expected, "format {format:#x}");
        }
    }

    #[test]
    fn fn_00e7acb0_four_character_codes() {
        let mut e = Engine::new();
        for code in CODES_ANSWERING_0 {
            assert_eq!(size(&mut e, code), 0, "code {code:#x}");
        }
        assert_eq!(size(&mut e, 0x3154_454d), 0);
        assert_eq!(size(&mut e, 0x3154_454e), -1);
        assert_eq!(size(&mut e, 0x3154_454c), -1);
        assert_eq!(size(&mut e, 0x7fff_ffff), -1);
    }
}

#[cfg(test)]
mod b000x_tests {
    use super::*;

    /// The nine formats at `011bf140` are 0x100 + slot; `00e7a8b0` gives each its
    /// (depth, stencil) pair from `bits`.
    fn setup(e: &mut Engine, allowed: &[u32]) -> u32 {
        e.map(DEPTH_FORMATS_011BF140 & !0xfff, 0x1000);
        for slot in 0..9u32 {
            e.mem
                .set_u32(DEPTH_FORMATS_011BF140 + 4 * slot, 0x100 + slot);
        }
        let entry = e.mem.alloc(0x48);
        for slot in allowed {
            e.mem.set_u8(entry + 0x24 + 4 * slot, 1);
        }
        e.register_double(MAP_GET_AT_00A53C30, move |e, a| {
            assert_eq!(a[1], 0x16);
            e.mem.set_u32(a[2], entry);
            1u32.into_ret()
        });
        e.register(FORMAT_BITS_00E7A8B0, |e, a| {
            let (depth, stencil) = match a[0] - 0x100 {
                0 => (16u32, 0u32),
                1 => (24, 8),
                2 => (32, 0),
                3 => (24, 8),
                4 => (15, 1),
                5 => (24, 0),
                _ => (0, 0),
            };
            e.mem.set_u32(a[1], depth);
            e.mem.set_u32(a[2], stencil);
            Ret::default()
        });
        e.mem.alloc(0x10)
    }

    fn choose(e: &mut Engine, this: u32, depth: u32, stencil: u32) -> u32 {
        e.call(0x00e7_ade0, &args![this, 0x16u32, depth, stencil])
            .u32()
    }

    #[test]
    fn fn_00e7ade0_picks_the_closest_allowed_format() {
        let mut e = Engine::new();
        let this = setup(&mut e, &[0, 1, 2, 3, 4]);
        e.call_log = Some(vec![]);
        // 24/8 exactly: slot 1, then slot 3 ties and wins (not above the best).
        assert_eq!(choose(&mut e, this, 24, 8), 0x103);
        let log = e.call_log.take().unwrap();
        let lookups: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == MAP_GET_AT_00A53C30)
            .collect();
        assert_eq!(lookups[0].1[0], this + 8);
        let asked: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == FORMAT_BITS_00E7A8B0)
            .map(|(_, a)| a[0])
            .collect();
        assert_eq!(asked, vec![0x100, 0x101, 0x102, 0x103, 0x104]);
        // 16/0: slot 0 exactly; every later one is farther in depth.
        assert_eq!(choose(&mut e, this, 16, 0), 0x100);
        // 32/0: slot 2; nothing after it is as close in depth.
        assert_eq!(choose(&mut e, this, 32, 0), 0x102);
    }

    #[test]
    fn fn_00e7ade0_needs_both_distances_not_above() {
        let mut e = Engine::new();
        // Slot 5 (24/0) is closer in depth than slot 0 (16/0) for 24/8 and as close in
        // stencil: it wins. Slot 4 (15/1) is farther in depth: skipped.
        let this = setup(&mut e, &[0, 4, 5]);
        assert_eq!(choose(&mut e, this, 24, 8), 0x105);
        // Nothing allowed: 0.
        let mut e = Engine::new();
        let this = setup(&mut e, &[]);
        assert_eq!(choose(&mut e, this, 24, 8), 0);
    }

    #[test]
    fn fn_00e7ade0_answers_0_when_the_format_is_not_found() {
        let mut e = Engine::new();
        e.register(MAP_GET_AT_00A53C30, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x10);
        assert_eq!(choose(&mut e, this, 24, 8), 0);
    }
}

#[cfg(test)]
mod b048_tests {
    use super::*;

    #[test]
    fn fn_00e7ade0_picks_the_closest_usable_depth_format() {
        let mut e = Engine::new();
        e.map(DEPTH_FORMATS_011BF140, 0x24);
        for i in 0..9u32 {
            e.set_global(DEPTH_FORMATS_011BF140 + 4 * i, 0x50 + i);
        }
        let record = e.mem.alloc(0x48);
        // Usable: formats 1, 3 and 5.
        for i in [1u32, 3, 5] {
            e.mem.set_u8(record + 0x24 + 4 * i, 1);
        }
        e.register_double(MAP_GET_AT_00A53C30, move |e, a| {
            if a[1] == 7 {
                e.mem.set_u32(a[2], record);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        // Sizes: format 0x51 (24, 8), 0x53 (32, 0), 0x55 (16, 8).
        e.register(FORMAT_BITS_00E7A8B0, |e, a| {
            let (first, second) = match a[0] {
                0x51 => (24, 8),
                0x53 => (32, 0),
                _ => (16, 8),
            };
            e.mem.set_u32(a[1], first);
            e.mem.set_u32(a[2], second);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00e7_ade0, &args![this, 7u32, 24i32, 8i32]).u32(),
            0x51
        );
        assert_eq!(
            e.call(0x00e7_ade0, &args![this, 7u32, 32i32, 0i32]).u32(),
            0x53
        );
        // A tie keeps the later format.
        assert_eq!(
            e.call(0x00e7_ade0, &args![this, 7u32, 20i32, 8i32]).u32(),
            0x55
        );
        assert_eq!(
            e.call(0x00e7_ade0, &args![this, 8u32, 24i32, 8i32]).u32(),
            0
        );
    }
}
