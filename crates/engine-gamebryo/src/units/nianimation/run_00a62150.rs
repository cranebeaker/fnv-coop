//! `NiAnimation/run_00a62150` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPixelFormat` (Xbox PDB): `m_uFlags` (+0x00; bit 0 set when the components are kept in
/// little-endian order), `m_ucBitsPerPixel` (+0x01) and `m_akComponents` (+0x14), entries of
/// 0xc bytes whose component kind is the word at +0 and bit count the byte at +8.
const FORMAT_BITS_PER_PIXEL: u32 = 0x01;
const FORMAT_COMPONENTS: u32 = 0x14;
const COMPONENT_SIZE: u32 = 0xc;
const COMPONENT_BITS: u32 = 0x8;
const COMPONENT_COUNT: u32 = 4;
const FLAG_LITTLE_ENDIAN: u8 = 0x1;

/// `NiTexture::RendererData::ms_aspReplacementData` (Xbox PDB name; ten `NiPointer`s from
/// `011f4514` on PC) and `ms_kReplacementFormat` (an `NiPixelFormat` at `011f4600`).
const REPLACEMENT_DATA: u32 = 0x011f_4514;
const REPLACEMENT_DATA_COUNT: u32 = 10;
const REPLACEMENT_FORMAT: u32 = 0x011f_4600;
const PIXEL_FORMAT_WORDS: u32 = 0x11;
/// The double 255.0 in `.rdata` the colour components are scaled by.
const COMPONENT_SCALE: u32 = 0x0101_e568;
/// The side of the checkerboard (32 pixels) and of its squares (16).
const BOARD_SIDE: u32 = 0x20;

/// `NiPixelData` fields the checkerboard fills (Xbox PDB names): `m_pucPixels` +0x50,
/// `m_puiOffsetInBytes` +0x5c, `m_uiMipmapLevels` +0x60.
const PIXEL_DATA_PIXELS: u32 = 0x50;
const PIXEL_DATA_OFFSETS: u32 = 0x5c;
const PIXEL_DATA_LEVELS: u32 = 0x60;

// Callees outside this unit.
/// `00aa13e0` (cdecl, a size): the object allocation.
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// `NiPixelData::NiPixelData` (`00a7c190`; `__thiscall`, five words: width, height, format,
/// mipmap levels, faces).
const PIXEL_DATA_CONSTRUCTOR: u32 = 0x00a7_c190;
/// `_memcpy` (`00ec44d0`, cdecl, three words).
const MEMCPY: u32 = 0x00ec_44d0;
/// `InterlockedDecrement` import slot and `DeleteThis` (virtual +0x4) of a reference.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const DELETE_THIS_SLOT: u32 = 0x4;

/// The component entry for `kind`, as its index and the bits of the components before it,
/// or `None` when the format has no such component among its four.
fn find_component(e: &Engine, format: u32, kind: u32) -> Option<(u32, u32)> {
    let mut before = 0u32;
    for index in 0..COMPONENT_COUNT {
        let entry = format + FORMAT_COMPONENTS + index * COMPONENT_SIZE;
        if e.mem.u32(entry) == kind {
            return Some((index, before));
        }
        before = before.wrapping_add(u32::from(e.mem.u8(entry + COMPONENT_BITS)));
    }
    None
}

// Translated from 00a62190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `NiPixelFormat`, one word: a component kind; the shape of
/// `NiPixelFormat::GetMask`): 0 when the format lacks the component or it has no bits.
/// Otherwise its mask (`bits` ones, summed as the code sums `1 << k` for `k` from `bits - 1`
/// down to 1, plus one) shifted up by the bits of the components before it; for a format whose
/// components are not little-endian the mask's bytes are swapped and shifted down by
/// `32 - bits per pixel` (shift counts taken modulo 32, as `SHL`/`SHR` do).
pub fn fn_00a62190(e: &mut Engine, this: Ptr, kind: u32) -> u32 {
    let format = this.addr();
    let Some((index, before)) = find_component(e, format, kind) else {
        return 0;
    };
    let bits = e
        .mem
        .u8(format + FORMAT_COMPONENTS + index * COMPONENT_SIZE + COMPONENT_BITS);
    if bits == 0 {
        return 0;
    }
    let mut sum = 0u32;
    let mut k = bits - 1;
    while k != 0 {
        sum = sum.wrapping_add(1u32.wrapping_shl(u32::from(k)));
        k -= 1;
    }
    let mask = sum.wrapping_add(1).wrapping_shl(before);
    if e.mem.u8(format) & FLAG_LITTLE_ENDIAN != 0 {
        return mask;
    }
    let bits_per_pixel = u32::from(e.mem.u8(format + FORMAT_BITS_PER_PIXEL));
    mask.swap_bytes()
        .wrapping_shr(0x20u32.wrapping_sub(bits_per_pixel))
}

// Translated from 00a62230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `NiPixelFormat`, one word: a component kind; the shape of
/// `NiPixelFormat::GetShift`): 0 when the format lacks the component; otherwise the bits of
/// the components before it (a byte sum) for a little-endian format, else `bits per pixel -
/// (its bits + those before)` (the byte the code returns in `AL`).
pub fn fn_00a62230(e: &mut Engine, this: Ptr, kind: u32) -> u8 {
    let format = this.addr();
    let Some((index, before)) = find_component(e, format, kind) else {
        return 0;
    };
    let before = before as u8;
    if e.mem.u8(format) & FLAG_LITTLE_ENDIAN != 0 {
        return before;
    }
    let bits = e
        .mem
        .u8(format + FORMAT_COMPONENTS + index * COMPONENT_SIZE + COMPONENT_BITS);
    let used = bits.wrapping_add(before);
    let bits_per_pixel = u32::from(e.mem.u8(format + FORMAT_BITS_PER_PIXEL));
    bits_per_pixel.wrapping_sub(u32::from(used)) as u8
}

/// Gives up the ten replacement data pointers (each deleted at its last reference) and
/// clears them.
fn release_replacement_data(e: &mut Engine) {
    for i in 0..REPLACEMENT_DATA_COUNT {
        let slot = REPLACEMENT_DATA + 4 * i;
        let data = e.mem.u32(slot);
        if data != 0 {
            let count = e.call(INTERLOCKED_DECREMENT, &args![data + 4]).u32();
            if count == 0 {
                e.vcall(data, DELETE_THIS_SLOT, &args![]);
            }
            e.mem.set_u32(slot, 0);
        }
    }
}

// Translated from 00a62280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no words): releases the replacement data (`011f4514`, ten pointers).
pub fn fn_00a62280(e: &mut Engine) {
    release_replacement_data(e);
}

// Translated from 00a622d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word: an `NiPixelFormat`): copies the format into the replacement
/// format (`011f4600`, 0x11 words) and releases the replacement data made for the old one.
pub fn fn_00a622d0(e: &mut Engine, format: Ptr) {
    for i in 0..PIXEL_FORMAT_WORDS {
        let word = e.mem.u32(format.addr() + 4 * i);
        e.mem.set_u32(REPLACEMENT_FORMAT + 4 * i, word);
    }
    release_replacement_data(e);
}

/// `FISTP qword` with the rounding control set to chop (`FNSTCW`, `OR 0xc00`, `FLDCW`): the
/// value truncated toward zero; out of range or NaN gives the integer indefinite
/// (`0x8000000000000000`). The code keeps the low word.
fn fistp_chop_low(value: f64) -> u32 {
    let truncated = value.trunc();
    let low = i64::MIN as f64;
    let as_integer = if value.is_nan() || !(low..-low).contains(&truncated) {
        i64::MIN
    } else {
        truncated as i64
    };
    as_integer as u32
}

/// One colour packed for `format`: each of its four floats times 255.0 (`FMUL double`),
/// chopped to an integer, shifted to the component's place and masked.
fn pack_colour(e: &mut Engine, format: Ptr, colour: u32) -> u32 {
    let scale = e.mem.f64(COMPONENT_SCALE);
    let mut packed = 0u32;
    for kind in 0..COMPONENT_COUNT {
        let shift = fn_00a62230(e, format, kind);
        let value = x87_mul(x87_load32(e.mem.u32(colour + 4 * kind)), scale);
        let placed = fistp_chop_low(value).wrapping_shl(u32::from(shift));
        let mask = fn_00a62190(e, format, kind);
        packed |= mask & placed;
    }
    packed
}

// Translated from 00a62330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, four words: two RGBA colours (four floats each), an `NiPixelFormat`, the
/// face count): a 32x32 checkerboard. New pixel data (`00aa13e0` of 0x74 bytes and
/// `NiPixelData::NiPixelData(32, 32, format, 1, faces)`; null when the allocation fails) gets,
/// on every face, the first colour where the 16-pixel row and column squares match and the
/// second elsewhere, each pixel copied (`_memcpy` of `bits per pixel / 8` bytes) from the
/// colours packed for the format. Answers the pixel data.
pub fn fn_00a62330(e: &mut Engine, first: Ptr, second: Ptr, format: Ptr, faces: u32) -> Ptr {
    e.with_stack(8, |e, colours| {
        let colours = colours.addr();
        let memory = e.call(NEW_OBJECT, &args![0x74u32]).u32();
        let data = if memory != 0 {
            e.call(
                PIXEL_DATA_CONSTRUCTOR,
                &args![memory, BOARD_SIDE, BOARD_SIDE, format, 1u32, faces],
            )
            .u32()
        } else {
            0
        };
        let packed = pack_colour(e, format, first.addr());
        e.mem.set_u32(colours, packed);
        let packed = pack_colour(e, format, second.addr());
        e.mem.set_u32(colours + 4, packed);
        for face in 0..faces {
            let offsets = e.mem.u32(data + PIXEL_DATA_OFFSETS);
            let levels = e.mem.u32(data + PIXEL_DATA_LEVELS);
            let mut pixel = e
                .mem
                .u32(offsets + 4 * levels)
                .wrapping_mul(face)
                .wrapping_add(e.mem.u32(offsets))
                .wrapping_add(e.mem.u32(data + PIXEL_DATA_PIXELS));
            let bytes = u32::from(e.mem.u8(format.addr() + FORMAT_BITS_PER_PIXEL) >> 3);
            for row in 0..BOARD_SIDE {
                for column in 0..BOARD_SIDE {
                    let colour = colours + 4 * u32::from(row >> 4 != column >> 4);
                    e.call(MEMCPY, &args![pixel, colour, bytes]);
                    pixel = pixel.wrapping_add(bytes);
                }
            }
        }
        Ptr::new(data)
    })
}

// Translated from 00a62150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (words, count)): reverses the byte order of `count`
/// 4-byte words in place.
pub fn fn_00a62150(e: &mut Engine, words: Ptr, count: u32) {
    let mut at = words.addr();
    for _ in 0..count {
        let (b0, b1, b2, b3) = (
            e.mem.u8(at),
            e.mem.u8(at + 1),
            e.mem.u8(at + 2),
            e.mem.u8(at + 3),
        );
        e.mem.set_u8(at, b3);
        e.mem.set_u8(at + 3, b0);
        e.mem.set_u8(at + 1, b2);
        e.mem.set_u8(at + 2, b1);
        at = at.wrapping_add(4);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_2150, fn_00a62150(Ptr, u32)),
        entry!(0x00a62190, fn_00a62190(Ptr, u32) -> u32),
        entry!(0x00a62230, fn_00a62230(Ptr, u32) -> u8),
        entry!(0x00a62280, fn_00a62280()),
        entry!(0x00a622d0, fn_00a622d0(Ptr)),
        entry!(0x00a62330, fn_00a62330(Ptr, Ptr, Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A 32-bit format with components (kind, bits) in this order.
    fn format(e: &mut Engine, little_endian: bool, components: &[(u32, u8)]) -> u32 {
        let f = e.mem.alloc(0x44);
        e.mem.set_u8(f, u8::from(little_endian));
        e.mem.set_u8(f + 1, 32);
        for (i, &(kind, bits)) in components.iter().enumerate() {
            let entry = f + FORMAT_COMPONENTS + COMPONENT_SIZE * i as u32;
            e.mem.set_u32(entry, kind);
            e.mem.set_u8(entry + COMPONENT_BITS, bits);
        }
        for i in components.len()..4 {
            e.mem
                .set_u32(f + FORMAT_COMPONENTS + COMPONENT_SIZE * i as u32, 0x10);
        }
        f
    }

    #[test]
    fn masks_and_shifts_of_components() {
        let mut e = Engine::new();
        let le = format(&mut e, true, &[(0, 8), (1, 8), (2, 8), (3, 8)]);
        assert_eq!(e.call(0x00a62190, &args![le, 0u32]).u32(), 0xff);
        assert_eq!(e.call(0x00a62190, &args![le, 2u32]).u32(), 0x00ff_0000);
        assert_eq!(e.call(0x00a62230, &args![le, 2u32]).u32(), 16);
        assert_eq!(e.call(0x00a62190, &args![le, 9u32]).u32(), 0);
        assert_eq!(e.call(0x00a62230, &args![le, 9u32]).u32(), 0);
        let be = format(&mut e, false, &[(0, 5), (1, 6), (2, 5), (3, 0)]);
        e.mem.set_u8(be + 1, 16);
        // Red: 0x1f, byte-swapped to 0x1f000000, then down by 16.
        assert_eq!(e.call(0x00a62190, &args![be, 0u32]).u32(), 0x1f00);
        assert_eq!(e.call(0x00a62230, &args![be, 0u32]).u32(), 11);
        assert_eq!(e.call(0x00a62230, &args![be, 1u32]).u32(), 5);
        // No bits: no mask.
        assert_eq!(e.call(0x00a62190, &args![be, 3u32]).u32(), 0);
    }

    const DELETE_DOUBLE: u32 = 0x0070_5000;

    #[test]
    fn replacement_data_is_released() {
        let mut e = Engine::new();
        e.map(REPLACEMENT_DATA, 0x200);
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        let data = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem.set_u32(data + 4, 2);
        e.mem.set_u32(REPLACEMENT_DATA + 8, data);
        e.mem.set_u32(REPLACEMENT_DATA + 0x24, data);
        e.call_log = Some(vec![]);
        e.call(0x00a62280, &args![]);
        assert_eq!(e.mem.u32(REPLACEMENT_DATA + 8), 0);
        assert_eq!(e.mem.u32(REPLACEMENT_DATA + 0x24), 0);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![data]]);
        let format = e.mem.alloc(0x44);
        for i in 0..0x11 {
            e.mem.set_u32(format + 4 * i, 0x900 + i);
        }
        e.mem.set_u32(data + 4, 1);
        e.mem.set_u32(REPLACEMENT_DATA, data);
        e.call(0x00a622d0, &args![format]);
        assert_eq!(e.mem.u32(REPLACEMENT_FORMAT), 0x900);
        assert_eq!(e.mem.u32(REPLACEMENT_FORMAT + 0x40), 0x910);
        assert_eq!(e.mem.u32(REPLACEMENT_DATA), 0);
        assert_eq!(calls_to(&e, DELETE_DOUBLE).len(), 2);
    }

    #[test]
    fn fistp_chop_truncates_toward_zero() {
        assert_eq!(fistp_chop_low(254.9), 254);
        assert_eq!(fistp_chop_low(-1.5), u32::MAX);
        assert_eq!(fistp_chop_low(f64::NAN), 0);
        assert_eq!(fistp_chop_low(1e30), 0);
    }

    #[test]
    fn fn_00a62330_paints_a_checkerboard() {
        let mut e = Engine::new();
        e.map(COMPONENT_SCALE, 8);
        e.mem.set_u64(COMPONENT_SCALE, 255.0f64.to_bits());
        let f = format(&mut e, true, &[(0, 8), (1, 8), (2, 8), (3, 8)]);
        let data = e.mem.alloc(0x74);
        let offsets = e.mem.alloc(8);
        e.mem.set_u32(offsets + 4, 0x1000);
        e.mem.set_u32(data + PIXEL_DATA_OFFSETS, offsets);
        e.mem.set_u32(data + PIXEL_DATA_LEVELS, 1);
        e.mem.set_u32(data + PIXEL_DATA_PIXELS, 0x8000);
        e.register_double(NEW_OBJECT, move |_, _| Ret {
            eax: data,
            ..Ret::default()
        });
        e.register(PIXEL_DATA_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.map(0x8000, 0x2000);
        e.register(MEMCPY, |e, a| {
            assert_eq!(a[2], 4);
            let colour = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], colour);
            Ret::default()
        });
        let red = e.mem.alloc(16);
        let green = e.mem.alloc(16);
        for (i, v) in [1.0f32, 0.0, 0.0, 1.0].iter().enumerate() {
            e.mem.set_u32(red + 4 * i as u32, v.to_bits());
        }
        for (i, v) in [0.0f32, 1.0, 0.0, 0.5].iter().enumerate() {
            e.mem.set_u32(green + 4 * i as u32, v.to_bits());
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a62330, &args![red, green, f, 2u32]).u32(), data);
        assert_eq!(
            calls_to(&e, PIXEL_DATA_CONSTRUCTOR),
            vec![vec![data, 32, 32, f, 1, 2]]
        );
        let copies = calls_to(&e, MEMCPY);
        assert_eq!(copies.len(), 2 * 32 * 32);
        // Row 0: columns 0..16 the first colour, 16..32 the second; face 1 starts at 0x1000.
        assert_eq!(copies[0][0], 0x8000);
        assert_eq!(copies[16][1], copies[0][1] + 4);
        assert_eq!(copies[16 * 32 + 16][1], copies[0][1]);
        assert_eq!(copies[1024][0], 0x9000);
        assert_eq!(e.mem.u32(0x8000), 0xff00_00ff);
        // 0.5 * 255 chops to 127.
        assert_eq!(e.mem.u32(0x8000 + 16 * 4), 0x7f00_ff00);
        assert_eq!(e.mem.u32(0x8000 + 16 * 128 + 16 * 4), 0xff00_00ff);
        assert_eq!(e.mem.u32(0x9000 + 16 * 128), 0x7f00_ff00);
    }
}

#[cfg(test)]
mod n007_tests {
    use super::*;

    #[test]
    fn swaps_each_word() {
        let mut e = Engine::new();
        let words = e.mem.alloc(12);
        e.mem.set_u32(words, 0x1122_3344);
        e.mem.set_u32(words + 4, 0xaabb_ccdd);
        e.mem.set_u32(words + 8, 0x5566_7788);
        e.call(0x00a6_2150, &args![words, 2u32]);
        assert_eq!(e.mem.u32(words), 0x4433_2211);
        assert_eq!(e.mem.u32(words + 4), 0xddcc_bbaa);
        assert_eq!(e.mem.u32(words + 8), 0x5566_7788);
        e.call(0x00a6_2150, &args![words, 0u32]);
        assert_eq!(e.mem.u32(words), 0x4433_2211);
    }
}
