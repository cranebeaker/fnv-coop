//! `NiXenonRenderer/run_00e7bcd0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size),
/// `NiPixelFormat::NiPixelFormat` (engine map, `00a7cca0`; `thiscall`, no stack words) and the
/// format filler `00e7c1e0` (unnamed; `cdecl`: Direct3D format, pixel format).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const NI_PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;
const FILL_PIXEL_FORMAT_00E7C1E0: u32 = 0x00e7_c1e0;

// Translated from 00e7c500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7bcd0`), `cdecl`, a Direct3D format: makes an
/// `NiPixelFormat` (0x44 bytes from the memory manager; null when that fails), fills it from
/// the format (`00e7c1e0`, also given null) and answers it.
pub fn fn_00e7c500(e: &mut Engine, format: u32) -> u32 {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x44u32])
        .u32();
    let pixel_format = if block != 0 {
        e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(FILL_PIXEL_FORMAT_00E7C1E0, &args![format, pixel_format]);
    pixel_format
}

/// `00e7bd20`'s callees on the pixel format (both unnamed, `thiscall` with one word): the
/// channel mask `00a62190` (the word picks the channel) and the channel's bit count
/// `00a77980` (answers a byte).
const CHANNEL_MASK_00A62190: u32 = 0x00a6_2190;
const CHANNEL_BITS_00A77980: u32 = 0x00a7_7980;

// Translated from 00e7bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7bcd0`), `cdecl`, one word (an `NiPixelFormat`): its
/// Direct3D format. A word at +0xc other than -1 is answered as it is. Otherwise the format
/// kind at +4 and the bits per pixel byte at +1 decide (the jump tables at `00e7c0d0`,
/// `00e7c110` with byte map `00e7c124` and `00e7c140` with byte map `00e7c15c`, read in
/// Ghidra), with the channel masks (`00a62190`) and channel bit counts (`00a77980`) where the
/// kind needs them:
/// - 0 (RGB): 8 bits: mask 0 0xe0 gives 0x1b, else a mask 0 of 0 gives 0x32; 16 bits: mask 0
///   0xf800 0x17, 0x7c00 0x18, 0xf00 0x1e, 0 0x51; 24 bits 0x14; 32 bits: mask 0 0xff0000
///   0x16, 0xff 0x21, 0xffff0000 0x22.
/// - 1 (RGBA): 8 bits: mask 9 of 0 gives 0x1c, 0xf 0x34; 16 bits: mask 0 0x7c00 0x19, 0xf00
///   0x1a, 0xe0 0x1d, 0 0x33; 32 bits: mask 0 0xff0000 0x15, 0x3ff 0x1f, 0xff 0x20,
///   0x3ff00000 0x23; 64 bits 0x71; 128 bits 0x74.
/// - 2: 0x29; 4, 5, 6: `"DXT1"`, `"DXT3"`, `"DXT5"`; 8: 0x3c; 9: 16 bits 0x3d, 32 bits 0x3e;
///   0xb: 8 bits 0x1c when mask 3 is nonzero else 0x32, 16 bits 0x6f, 32 bits 0x72; 0xc: 16
///   bits 0x33, 32 bits 0x70, 64 bits 0x73; 0xe: 32 bits 0x15, 64 bits 0x71, 128 bits 0x74;
///   0xf: 16 bits 0x49 when channel 0x12 has 1 bit else 0x50, 32 bits 0x4f when channel 0x12
///   has 4 bits, 0x4b when 8, else 0x4d when channel 0x11 has 0x18 bits and 0x47 otherwise.
/// - Everything else (kinds 3, 7, 0xa, 0xd and above 0xf, other sizes, masks matching
///   nothing): 0.
///
/// Each mask test calls `00a62190` again, as the exe does.
pub fn fn_00e7bd20(e: &mut Engine, pixel_format: Ptr) -> u32 {
    let format = pixel_format.addr();
    let hint = e.mem.u32(format.wrapping_add(0xc));
    if hint != 0xffff_ffff {
        return hint;
    }
    let bits = e.mem.u8(format.wrapping_add(1));
    let mask =
        |e: &mut Engine, channel: u32| e.call(CHANNEL_MASK_00A62190, &args![format, channel]).u32();
    let channel_bits =
        |e: &mut Engine, channel: u32| e.call(CHANNEL_BITS_00A77980, &args![format, channel]).u8();
    match e.mem.u32(format.wrapping_add(4)) {
        0 => match bits {
            8 => {
                if mask(e, 0) == 0xe0 {
                    0x1b
                } else if mask(e, 0) == 0 {
                    0x32
                } else {
                    0
                }
            }
            0x10 => {
                if mask(e, 0) == 0xf800 {
                    0x17
                } else if mask(e, 0) == 0x7c00 {
                    0x18
                } else if mask(e, 0) == 0xf00 {
                    0x1e
                } else if mask(e, 0) == 0 {
                    0x51
                } else {
                    0
                }
            }
            0x18 => 0x14,
            0x20 => {
                if mask(e, 0) == 0x00ff_0000 {
                    0x16
                } else if mask(e, 0) == 0xff {
                    0x21
                } else if mask(e, 0) == 0xffff_0000 {
                    0x22
                } else {
                    0
                }
            }
            _ => 0,
        },
        1 => match bits {
            8 => {
                if mask(e, 9) == 0 {
                    0x1c
                } else if mask(e, 9) == 0xf {
                    0x34
                } else {
                    0
                }
            }
            0x10 => {
                if mask(e, 0) == 0x7c00 {
                    0x19
                } else if mask(e, 0) == 0xf00 {
                    0x1a
                } else if mask(e, 0) == 0xe0 {
                    0x1d
                } else if mask(e, 0) == 0 {
                    0x33
                } else {
                    0
                }
            }
            0x20 => {
                if mask(e, 0) == 0x00ff_0000 {
                    0x15
                } else if mask(e, 0) == 0x3ff {
                    0x1f
                } else if mask(e, 0) == 0xff {
                    0x20
                } else if mask(e, 0) == 0x3ff0_0000 {
                    0x23
                } else {
                    0
                }
            }
            0x40 => 0x71,
            0x80 => 0x74,
            _ => 0,
        },
        2 => 0x29,
        4 => 0x3154_5844,
        5 => 0x3354_5844,
        6 => 0x3554_5844,
        8 => 0x3c,
        9 => match bits {
            0x10 => 0x3d,
            0x20 => 0x3e,
            _ => 0,
        },
        0xb => match bits {
            // NEG/SBB/AND 0xffffffea/ADD 0x32.
            8 => {
                let high = if mask(e, 3) != 0 { 0xffff_ffeau32 } else { 0 };
                high.wrapping_add(0x32)
            }
            0x10 => 0x6f,
            0x20 => 0x72,
            _ => 0,
        },
        0xc => match bits {
            0x10 => 0x33,
            0x20 => 0x70,
            0x40 => 0x73,
            _ => 0,
        },
        0xe => match bits {
            0x20 => 0x15,
            0x40 => 0x71,
            0x80 => 0x74,
            _ => 0,
        },
        0xf => match bits {
            // DEC AL/MOVZX/NEG/SBB/AND 7/ADD 0x49.
            0x10 => {
                if channel_bits(e, 0x12) != 1 {
                    0x50
                } else {
                    0x49
                }
            }
            0x20 => match channel_bits(e, 0x12) {
                4 => 0x4f,
                8 => 0x4b,
                // SUB AL,0x18/MOVZX/NEG/SBB/AND 0xfffffffa/ADD 0x4d.
                _ => {
                    if channel_bits(e, 0x11) != 0x18 {
                        0x47
                    } else {
                        0x4d
                    }
                }
            },
            _ => 0,
        },
        // 3, 7, 0xa, 0xd and above 0xf.
        _ => 0,
    }
}

/// The 0x44-byte pixel format records `00e7c1e0` copies from, by Direct3D format (the byte
/// map at `00e7c490` and jump table at `00e7c3e0`, read in Ghidra), and the one every other
/// format gets.
fn pixel_format_record_00e7c1e0(format: u32) -> u32 {
    match format {
        0x3154_5844 => 0x011a_a498,
        0x3354_5844 => 0x011a_a4e0,
        0x3554_5844 => 0x011a_a528,
        0x14 => 0x011a_aa80,
        0x15 => 0x011a_aac8,
        0x16 => 0x011a_ab58,
        0x17 => 0x011a_a8d0,
        0x18 => 0x011a_a9a8,
        0x19 => 0x011a_a960,
        0x1a => 0x011a_a9f0,
        0x1b => 0x011a_a7b0,
        0x1c => 0x011a_a768,
        0x1d => 0x011a_a7f8,
        0x1e => 0x011a_aa38,
        0x1f => 0x011a_abe8,
        0x20 => 0x011a_a2a0,
        0x21 => 0x011a_aba0,
        0x22 => 0x011a_acc0,
        0x23 => 0x011a_ac30,
        0x24 | 0x71 => 0x011a_a5b8,
        0x28 => 0x011a_ad50,
        0x29 => 0x011a_a210,
        0x32 => 0x011a_ad98,
        0x33 => 0x011a_ae28,
        0x34 => 0x011a_ae70,
        0x3c => 0x011a_a408,
        0x3d => 0x011a_ad08,
        0x3e => 0x011a_af90,
        0x3f => 0x011a_aeb8,
        0x40 => 0x011a_af00,
        0x43 => 0x011a_af48,
        0x46 | 0x50 => 0x011a_b140,
        0x47 => 0x011a_b188,
        0x49 => 0x011a_b1d0,
        0x4b => 0x011a_b218,
        0x4d => 0x011a_b2a8,
        0x4f => 0x011a_b2f0,
        0x51 => 0x011a_ade0,
        0x52 | 0x53 => 0x011a_b0b0,
        0x6e => 0x011a_b0f8,
        0x6f => 0x011a_a6d8,
        0x70 => 0x011a_a648,
        0x72 => 0x011a_a600,
        0x73 => 0x011a_a690,
        0x74 => 0x011a_a570,
        0x75 => 0x011a_b068,
        _ => 0x011a_b020,
    }
}

// Translated from 00e7c1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7bcd0`), `cdecl`, two words: a Direct3D format and an
/// `NiPixelFormat` to fill (no null check). Copies the format's 0x44-byte record (see
/// [`pixel_format_record_00e7c1e0`]; `REP MOVSD`, 0x11 words) over the pixel format, then
/// stores the format at +0xc and 0 at +0x10.
pub fn fn_00e7c1e0(e: &mut Engine, format: u32, pixel_format: Ptr) {
    let target = pixel_format.addr();
    let record = pixel_format_record_00e7c1e0(format);
    for word in 0..0x11u32 {
        let value = e.mem.u32(record + 4 * word);
        e.mem.set_u32(target.wrapping_add(4 * word), value);
    }
    e.mem.set_u32(target.wrapping_add(0xc), format);
    e.mem.set_u32(target.wrapping_add(0x10), 0);
}

/// What the two destructors below use: the adapter description clean-up `00e7bb50` (unnamed;
/// `thiscall`, no stack words), the sized free `00aa1460` (`cdecl`: block, size), the free
/// `00aa10f0` (`cdecl`, a block), the array table stored at +4 (RTTI
/// `NiTArray<NiDX9AdapterDesc *, NiTMallocInterface<NiDX9AdapterDesc *>>`), the
/// `NiD3DDefaultShader` table and the shader base destructor `00e806c0` (`nid3dshader.cpp`;
/// `thiscall`, no stack words).
const ADAPTER_DESC_CLEAN_UP_00E7BB50: u32 = 0x00e7_bb50;
const FREE_00AA1460: u32 = 0x00aa_1460;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const ADAPTER_DESC_ARRAY_VTABLE: u32 = 0x010e_ec4c;
const NI_D3D_DEFAULT_SHADER_VTABLE: u32 = 0x010e_ecac;
const SHADER_DOWN_00E806C0: u32 = 0x00e8_06c0;

// Translated from 00e7bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7bcd0`), `thiscall`: frees the adapter descriptions of
/// the array at +4 (data at +8, the 16-bit count at +0xe read again for each step): each one
/// set gets `00e7bb50` and its 0x468 bytes freed (`00aa1460`). Then the array's table
/// (`NiTArray<NiDX9AdapterDesc *, ...>`) goes to +4 and its data is freed (`00aa10f0`).
pub fn fn_00e7bcd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(base.wrapping_add(0xe))) {
        let data = e.mem.u32(base.wrapping_add(8));
        let desc = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
        if desc != 0 {
            e.call(ADAPTER_DESC_CLEAN_UP_00E7BB50, &args![desc]);
            e.call(FREE_00AA1460, &args![desc, 0x468u32]);
        }
        index += 1;
    }
    let data = e.mem.u32(base.wrapping_add(8));
    e.mem
        .set_u32(base.wrapping_add(4), ADAPTER_DESC_ARRAY_VTABLE);
    e.call(FREE_00AA10F0, &args![data]);
}

// Translated from 00e7c540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7bcd0`): the `NiD3DDefaultShader` destructor (the RTTI
/// of the table it stores), `thiscall`: its table, the block at +0x50 freed (`00aa10f0`), then
/// the shader base destructor (`00e806c0`, a tail jump).
pub fn fn_00e7c540(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let block = e.mem.u32(base.wrapping_add(0x50));
    e.mem.set_u32(base, NI_D3D_DEFAULT_SHADER_VTABLE);
    e.call(FREE_00AA10F0, &args![block]);
    e.call(SHADER_DOWN_00E806C0, &args![base]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_c500, fn_00e7c500(u32) -> u32),
        entry!(0x00e7_bd20, fn_00e7bd20(Ptr) -> u32),
        entry!(0x00e7_c1e0, fn_00e7c1e0(u32, Ptr)),
        entry!(0x00e7_bcd0, fn_00e7bcd0(Ptr)),
        entry!(0x00e7_c540, fn_00e7c540(Ptr)),
    ]
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e7c500_makes_and_fills_a_pixel_format() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FILL_PIXEL_FORMAT_00E7C1E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let made = e.call(0x00e7_c500, &args![0x16u32]).u32();
        assert_ne!(made, 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(FILL_PIXEL_FORMAT_00E7C1E0, vec![0x16, made])));
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_c500, &args![0x16u32]).u32(), 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(FILL_PIXEL_FORMAT_00E7C1E0, vec![0x16, 0])));
    }
}

#[cfg(test)]
mod b000x_tests {
    use super::*;

    /// A pixel format of `kind` and `bits` with no cached format; masks 0 and 9 and 3 and
    /// the channel bit counts come from the words after the format (+0x50 mask 0, +0x54 mask
    /// 9, +0x58 mask 3, +0x5c channel 0x12 bits, +0x60 channel 0x11 bits).
    fn format(e: &mut Engine, kind: u32, bits: u8) -> u32 {
        let p = e.mem.alloc(0x64);
        e.mem.set_u8(p + 1, bits);
        e.mem.set_u32(p + 4, kind);
        e.mem.set_u32(p + 0xc, 0xffff_ffff);
        e.register(CHANNEL_MASK_00A62190, |e, a| {
            let offset = match a[1] {
                0 => 0x50,
                9 => 0x54,
                3 => 0x58,
                other => panic!("mask {other}"),
            };
            e.mem.u32(a[0] + offset).into_ret()
        });
        e.register(CHANNEL_BITS_00A77980, |e, a| {
            let offset = match a[1] {
                0x12 => 0x5c,
                0x11 => 0x60,
                other => panic!("channel {other:#x}"),
            };
            e.mem.u32(a[0] + offset).into_ret()
        });
        p
    }

    fn d3d(e: &mut Engine, kind: u32, bits: u8, words: &[(u32, u32)]) -> u32 {
        let p = format(e, kind, bits);
        for (offset, value) in words {
            e.mem.set_u32(p + offset, *value);
        }
        e.call(0x00e7_bd20, &args![p]).u32()
    }

    #[test]
    fn fn_00e7bd20_answers_a_cached_format() {
        let mut e = Engine::new();
        let p = format(&mut e, 0, 0x18);
        e.mem.set_u32(p + 0xc, 0x15);
        assert_eq!(e.call(0x00e7_bd20, &args![p]).u32(), 0x15);
    }

    #[test]
    fn fn_00e7bd20_rgb_and_rgba() {
        let mut e = Engine::new();
        let cases: [(u32, u8, u32, u32, u32); 24] = [
            (0, 8, 0x50, 0xe0, 0x1b),
            (0, 8, 0x50, 0, 0x32),
            (0, 8, 0x50, 0x1, 0),
            (0, 0x10, 0x50, 0xf800, 0x17),
            (0, 0x10, 0x50, 0x7c00, 0x18),
            (0, 0x10, 0x50, 0xf00, 0x1e),
            (0, 0x10, 0x50, 0, 0x51),
            (0, 0x10, 0x50, 0x1, 0),
            (0, 0x18, 0x50, 0x1, 0x14),
            (0, 0x20, 0x50, 0x00ff_0000, 0x16),
            (0, 0x20, 0x50, 0xff, 0x21),
            (0, 0x20, 0x50, 0xffff_0000, 0x22),
            (0, 0x40, 0x50, 0, 0),
            (1, 8, 0x54, 0, 0x1c),
            (1, 8, 0x54, 0xf, 0x34),
            (1, 0x10, 0x50, 0x7c00, 0x19),
            (1, 0x10, 0x50, 0xf00, 0x1a),
            (1, 0x10, 0x50, 0xe0, 0x1d),
            (1, 0x10, 0x50, 0, 0x33),
            (1, 0x20, 0x50, 0x00ff_0000, 0x15),
            (1, 0x20, 0x50, 0x3ff, 0x1f),
            (1, 0x20, 0x50, 0xff, 0x20),
            (1, 0x20, 0x50, 0x3ff0_0000, 0x23),
            (1, 0x20, 0x50, 0x1, 0),
        ];
        for (kind, bits, offset, value, expected) in cases {
            assert_eq!(
                d3d(&mut e, kind, bits, &[(offset, value)]),
                expected,
                "kind {kind} bits {bits:#x} mask {value:#x}"
            );
        }
        assert_eq!(d3d(&mut e, 1, 0x40, &[]), 0x71);
        assert_eq!(d3d(&mut e, 1, 0x80, &[]), 0x74);
        assert_eq!(d3d(&mut e, 1, 0x18, &[]), 0);
    }

    #[test]
    fn fn_00e7bd20_other_kinds() {
        let mut e = Engine::new();
        let cases: [(u32, u8, u32); 27] = [
            (2, 0, 0x29),
            (3, 0x20, 0),
            (4, 0, 0x3154_5844),
            (5, 0, 0x3354_5844),
            (6, 0, 0x3554_5844),
            (7, 0x20, 0),
            (8, 0, 0x3c),
            (9, 0x10, 0x3d),
            (9, 0x20, 0x3e),
            (9, 8, 0),
            (0xa, 0x20, 0),
            (0xb, 0x10, 0x6f),
            (0xb, 0x20, 0x72),
            (0xb, 0x40, 0),
            (0xc, 0x10, 0x33),
            (0xc, 0x20, 0x70),
            (0xc, 0x40, 0x73),
            (0xc, 8, 0),
            (0xd, 0x20, 0),
            (0xe, 0x20, 0x15),
            (0xe, 0x40, 0x71),
            (0xe, 0x80, 0x74),
            (0xe, 0x10, 0),
            (0xf, 8, 0),
            (0x10, 0x20, 0),
            (0xffff_ffff, 0x20, 0),
            (0xb, 0x18, 0),
        ];
        for (kind, bits, expected) in cases {
            assert_eq!(
                d3d(&mut e, kind, bits, &[]),
                expected,
                "kind {kind:#x} bits {bits:#x}"
            );
        }
        // Kind 0xb, 8 bits: mask 3 decides.
        assert_eq!(d3d(&mut e, 0xb, 8, &[(0x58, 7)]), 0x1c);
        assert_eq!(d3d(&mut e, 0xb, 8, &[(0x58, 0)]), 0x32);
    }

    #[test]
    fn fn_00e7bd20_kind_0xf_reads_the_channel_bits() {
        let mut e = Engine::new();
        assert_eq!(d3d(&mut e, 0xf, 0x10, &[(0x5c, 1)]), 0x49);
        assert_eq!(d3d(&mut e, 0xf, 0x10, &[(0x5c, 0x101)]), 0x49);
        assert_eq!(d3d(&mut e, 0xf, 0x10, &[(0x5c, 2)]), 0x50);
        assert_eq!(d3d(&mut e, 0xf, 0x20, &[(0x5c, 4)]), 0x4f);
        assert_eq!(d3d(&mut e, 0xf, 0x20, &[(0x5c, 8)]), 0x4b);
        assert_eq!(d3d(&mut e, 0xf, 0x20, &[(0x5c, 2), (0x60, 0x18)]), 0x4d);
        assert_eq!(d3d(&mut e, 0xf, 0x20, &[(0x5c, 2), (0x60, 0x10)]), 0x47);
    }

    #[test]
    fn fn_00e7bd20_asks_the_mask_again_for_each_test() {
        let mut e = Engine::new();
        let p = format(&mut e, 0, 0x10);
        e.mem.set_u32(p + 0x50, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_bd20, &args![p]).u32(), 0);
        let asked = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|(a, words)| *a == CHANNEL_MASK_00A62190 && words[..] == [p, 0])
            .count();
        assert_eq!(asked, 4);
    }

    #[test]
    fn fn_00e7c1e0_copies_the_record_and_sets_the_format() {
        let mut e = Engine::new();
        e.map(0x011a_a000, 0x2000);
        let cases: [(u32, u32); 8] = [
            (0x3154_5844, 0x011a_a498),
            (0x3354_5844, 0x011a_a4e0),
            (0x3554_5844, 0x011a_a528),
            (0x15, 0x011a_aac8),
            (0x71, 0x011a_a5b8),
            (0x53, 0x011a_b0b0),
            (0x75, 0x011a_b068),
            (0x25, 0x011a_b020),
        ];
        for (format, record) in cases {
            for word in 0..0x11u32 {
                e.mem.set_u32(record + 4 * word, record + word);
            }
            let target = e.mem.alloc(0x48);
            e.mem.set_u32(target + 0x44, 0xdead_beef);
            e.call(0x00e7_c1e0, &args![format, target]);
            for word in 0..0x11u32 {
                let expected = match word {
                    3 => format,
                    4 => 0,
                    _ => record + word,
                };
                assert_eq!(e.mem.u32(target + 4 * word), expected, "{format:#x}");
            }
            assert_eq!(e.mem.u32(target + 0x44), 0xdead_beef);
        }
        // Negative, small and above-table formats take the default record.
        for format in [0xffff_fff0u32, 0x13, 0x76, 0x3154_5845, 0x3454_5844] {
            assert_eq!(pixel_format_record_00e7c1e0(format), 0x011a_b020);
        }
    }
}

#[cfg(test)]
mod b051_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for no_op in [
            ADAPTER_DESC_CLEAN_UP_00E7BB50,
            FREE_00AA1460,
            FREE_00AA10F0,
            SHADER_DOWN_00E806C0,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e7bcd0_frees_each_adapter_description() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(0xc);
        e.mem.set_u32(data, 0x111);
        e.mem.set_u32(data + 8, 0x333);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u16(this + 0xe, 3);
        e.call(0x00e7_bcd0, &args![this]);
        assert_eq!(
            calls(&e, ADAPTER_DESC_CLEAN_UP_00E7BB50),
            vec![vec![0x111], vec![0x333]]
        );
        assert_eq!(
            calls(&e, FREE_00AA1460),
            vec![vec![0x111, 0x468], vec![0x333, 0x468]]
        );
        assert_eq!(e.mem.u32(this + 4), ADAPTER_DESC_ARRAY_VTABLE);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![data]]);
    }

    #[test]
    fn fn_00e7c540_frees_its_block_and_goes_down() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        e.mem.set_u32(this + 0x50, 0x777);
        e.call(0x00e7_c540, &args![this]);
        assert_eq!(e.mem.u32(this), NI_D3D_DEFAULT_SHADER_VTABLE);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x777]]);
        assert_eq!(calls(&e, SHADER_DOWN_00E806C0), vec![vec![this]]);
    }
}
