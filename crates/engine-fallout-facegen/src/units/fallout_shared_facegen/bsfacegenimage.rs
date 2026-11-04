//! `fallout shared/facegen/bsfacegenimage.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0064cdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenImage` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `0064ce20` and frees 0x2c bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_image_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0064_ce20, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x2cu32]);
    }
    this
}

/// `BSFaceGenImage`'s vtable (RTTI).
const BS_FACE_GEN_IMAGE_VTABLE: u32 = 0x0106_53c4;
/// The texture pointer at +8: its get (`00559450`, `thiscall`), assignment from a raw pointer
/// (`0066b0d0`, `thiscall`: the pointer), assignment from another (`006e5cc0`, `thiscall`: the
/// other's address), constructor from a raw pointer (`00633c90`, `thiscall`: the pointer) and
/// destructor (`0045cec0`, `thiscall`).
const TEXTURE_GET: u32 = 0x0055_9450;
const TEXTURE_SET: u32 = 0x0066_b0d0;
const TEXTURE_ASSIGN: u32 = 0x006e_5cc0;
const TEXTURE_CONSTRUCT: u32 = 0x0063_3c90;
const TEXTURE_DESTRUCT: u32 = 0x0045_cec0;
/// The image's values (+0xC, a container of four-float values): its emptiness test
/// (`0064db00`, `thiscall`), its begin and end iterators (`0044a170` and `0065f550`,
/// `thiscall`: the out iterator; each answers the address of its result), its range erase
/// (`0064dc50`, `thiscall`: result, first and last iterators), the iterator's value address
/// (`0064dbd0`, `thiscall`), its step (`0064ddf0`, `thiscall`) and the container's destructor
/// (`0065c7c0`, `thiscall`).
const VALUES_EMPTY: u32 = 0x0064_db00;
const VALUES_BEGIN: u32 = 0x0044_a170;
const VALUES_END: u32 = 0x0065_f550;
const VALUES_ERASE: u32 = 0x0064_dc50;
const ITERATOR_VALUE: u32 = 0x0064_dbd0;
const ITERATOR_STEP: u32 = 0x0064_ddf0;
const VALUES_DESTRUCT: u32 = 0x0065_c7c0;
/// The base destructor `BSFaceGenImage` ends with (`00496910`, `thiscall`).
const BASE_DESTRUCT: u32 = 0x0049_6910;
/// The byte the debug texture hook tests (`0064d870`'s answer).
const DEBUG_TEXTURE_FLAG: u32 = 0x011f_94b0;
/// `NiDevImageConverter`'s constructor (`00a7b110`), destructor (`00a7aff0`) and
/// `GenerateMipmapLevels` (`00a78090`: the pixel data and 0), all `thiscall`.
const CONVERTER_CONSTRUCT: u32 = 0x00a7_b110;
const CONVERTER_DESTRUCT: u32 = 0x00a7_aff0;
const CONVERTER_MIPMAPS: u32 = 0x00a7_8090;
/// The size query of the size argument (`00726070`, `thiscall`; 0 picks the three-channel
/// format), the game's allocation (`00aa13e0`, `cdecl`: the size) and
/// `NiPixelData::NiPixelData` (`00a7c190`, `thiscall`: width, height, format, 1, 1) with the
/// three- and four-channel formats (`011aa2e8`, `011aa2a0`); the pixel data's byte address
/// (`0064dac0`, `thiscall`: two zeros).
const SIZE_QUERY: u32 = 0x0072_6070;
const ALLOCATE: u32 = 0x00aa_13e0;
const PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
const FORMAT_THREE_CHANNELS: u32 = 0x011a_a2e8;
const FORMAT_FOUR_CHANNELS: u32 = 0x011a_a2a0;
const PIXEL_BYTES: u32 = 0x0064_dac0;
/// The float floor (`00404040`, `cdecl`, answers in ST0: the truncation of `00404090`, one
/// less when that is above the value).
const FLOAT_FLOOR: u32 = 0x0040_4040;
/// The mipmap setting's test (`00454af0`, `thiscall` on `011d59ac`).
const MIPMAP_SETTING: u32 = 0x0045_4af0;
const MIPMAP_SETTING_OBJECT: u32 = 0x011d_59ac;
/// The debug texture hook (`00b6b940`, `cdecl`: the pixel data), `NiSourceTexture::Create`
/// (`00a5ff10`, `cdecl`: pixel data, 0, the format preferences `011a9598`) and the resize of
/// a texture for the size argument (`00b6bbd0`, `cdecl`: texture, the size argument).
const DEBUG_TEXTURE: u32 = 0x00b6_b940;
const SOURCE_TEXTURE_CREATE: u32 = 0x00a5_ff10;
const TEXTURE_PREFERENCES: u32 = 0x011a_9598;
const TEXTURE_RESIZE: u32 = 0x00b6_bbd0;

// Translated from 0064ce20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSFaceGenImage`'s destructor (`thiscall`): its vtable, the
/// values erased (`0064db70` on +0xC), the texture pointer (+8) cleared (`0066b0d0(0)`), the
/// values' container destroyed (`0065c7c0`), the texture pointer destroyed (`0045cec0`), then
/// the base destructor (`00496910`).
// Translation incomplete: 0064ce20 - the C++ exception-unwinding frame (FS:[0] chain, handler 00efe77e, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_0064ce20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BS_FACE_GEN_IMAGE_VTABLE);
    fn_0064db70(e, Ptr::new(this.addr() + 0xc));
    e.call(TEXTURE_SET, &args![this.addr() + 8, 0u32]);
    e.call(VALUES_DESTRUCT, &args![this.addr() + 0xc]);
    e.call(TEXTURE_DESTRUCT, &args![this.addr() + 8]);
    e.call(BASE_DESTRUCT, &args![this]);
}

/// One channel of a compressed pixel: the value at `offset` of the iterator's four floats
/// (through the x87 stack), clamped to `[min, max]` (below `min` or unordered gives `min`;
/// above `max` gives `max`), floored (`00404040`), less `min`, times `scale`, truncated to an
/// integer; its low byte.
fn compress_channel(
    e: &mut Engine,
    iterator: u32,
    offset: u32,
    min: f32,
    max: f32,
    scale: f32,
) -> u8 {
    let values = e.call(ITERATOR_VALUE, &args![iterator]).u32();
    let value = x87_quiet_bits(e.mem.u32(values + offset));
    let clamped = if !(x87_load32(min.to_bits()) <= x87_load32(value)) {
        x87_quiet_bits(min.to_bits())
    } else if !(x87_load32(max.to_bits()) >= x87_load32(value)) {
        x87_quiet_bits(max.to_bits())
    } else {
        value
    };
    let floor = e.call(FLOAT_FLOOR, &args![x87_quiet_bits(clamped)]).f64();
    let shifted = x87_sub(floor, x87_m32(min.to_bits()));
    let scaled = x87_mul(shifted, x87_m32(scale.to_bits()));
    engine_core::x87::fistp_chop(scaled) as u8
}

// Translated from 0064ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenImage::GetCompressedImage` (Xbox PDB; `thiscall`, four words: the float range
/// `min`, `max`, the float `scale` and a size argument). Without a texture yet (+8), with
/// values (+0xC not empty), a width (+0x24) and height (+0x28) and `max > min`: builds
/// `NiPixelData` of the image's size (`0x74` bytes, three channels when the size argument's
/// query `00726070` answers 0, else four) and fills it from the values in order, each channel
/// compressed to a byte (`(floor(clamp(value)) - min) * scale`, truncated), stepping the
/// iterator after each pixel (`0064dbf0`). With the mipmap setting (`00454af0`) the converter's
/// `GenerateMipmapLevels` makes the texture's pixel data and the original is deleted (its
/// virtual 0 with 1); the debug hook (`00b6b940`) sees the pixel data when `0064d870` is set;
/// `NiSourceTexture::Create` makes the texture. When the size query answers 1 or 0 the texture
/// becomes the image's, else the image gets its resize (`00b6bbd0`); an image still without a
/// texture then takes the made one. Answers the image's texture (`00559450` on +8).
// Translation incomplete: 0064ceb0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00efe7da, state variable) and the /GS cookie check (00ec408c) are not translated.
pub fn bs_face_gen_image_get_compressed_image(
    e: &mut Engine,
    this: Ptr,
    min: f32,
    max: f32,
    scale: f32,
    size: u32,
) -> u32 {
    let image = this.addr();
    let texture = image + 8;
    let values = image + 0xc;
    let ready = e.call(TEXTURE_GET, &args![texture]).u32() == 0
        && e.call(VALUES_EMPTY, &args![values]).u8() == 0
        && e.mem.u32(image + 0x24) != 0
        && e.mem.u32(image + 0x28) != 0
        && x87_load32(max.to_bits()) > x87_load32(min.to_bits());
    if ready {
        // The converter (0xA7C bytes), the made texture's pointer, the iterator.
        e.with_stack(0xa7c + 4 + 8 + 8, |e, frame| {
            let converter = frame.addr();
            let made = converter + 0xa7c;
            let iterator = made + 4;
            let scratch = iterator + 8;
            e.call(CONVERTER_CONSTRUCT, &args![converter]);
            e.call(TEXTURE_CONSTRUCT, &args![made, 0u32]);
            let begin = e.call(VALUES_BEGIN, &args![values, scratch]).u32();
            let word = e.mem.u32(begin);
            e.mem.set_u32(iterator, word);
            let word = e.mem.u32(begin + 4);
            e.mem.set_u32(iterator + 4, word);
            let pixels_count = e
                .mem
                .u32(image + 0x24)
                .wrapping_mul(e.mem.u32(image + 0x28));
            let three = e.call(SIZE_QUERY, &args![size]).u32() == 0;
            let (format, channels) = if three {
                (FORMAT_THREE_CHANNELS, 3u32)
            } else {
                (FORMAT_FOUR_CHANNELS, 4u32)
            };
            let block = e.call(ALLOCATE, &args![0x74u32]).u32();
            let pixel_data = if block == 0 {
                0
            } else {
                let width = e.mem.u32(image + 0x24);
                let height = e.mem.u32(image + 0x28);
                e.call(
                    PIXEL_DATA_CONSTRUCT,
                    &args![block, width, height, format, 1u32, 1u32],
                )
                .u32()
            };
            let mut bytes = e.call(PIXEL_BYTES, &args![pixel_data, 0u32, 0u32]).u32();
            let first_byte = bytes;
            let mut pixel = 0u32;
            while pixel < pixels_count {
                for channel in 0..channels {
                    let byte = compress_channel(e, iterator, 4 * channel, min, max, scale);
                    e.mem.set_u8(bytes + channel, byte);
                }
                bytes = bytes.wrapping_add(channels);
                fn_0064dbf0(e, Ptr::new(iterator), scratch, 0);
                pixel = pixel.wrapping_add(1);
            }
            if super::bsfacegenmodel::face_check::enabled() {
                // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                let width = e.mem.u32(image + 0x24);
                let height = e.mem.u32(image + 0x28);
                super::bsfacegenmodel::face_check::image(
                    e, first_byte, width, height, channels, pixel_data,
                );
            }
            let source = if e.call(MIPMAP_SETTING, &args![MIPMAP_SETTING_OBJECT]).u8() != 0 {
                let mipmaps = e
                    .call(CONVERTER_MIPMAPS, &args![converter, pixel_data, 0u32])
                    .u32();
                if pixel_data != 0 {
                    e.vcall(pixel_data, 0, &args![1u32]);
                }
                mipmaps
            } else {
                pixel_data
            };
            if fn_0064d870(e) != 0 {
                e.call(DEBUG_TEXTURE, &args![source]);
            }
            let created = e
                .call(
                    SOURCE_TEXTURE_CREATE,
                    &args![source, 0u32, TEXTURE_PREFERENCES],
                )
                .u32();
            e.call(TEXTURE_SET, &args![made, created]);
            if super::bsfacegenmodel::face_check::enabled() {
                // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
                super::bsfacegenmodel::face_check::line(format_args!(
                    "image texture {created:#x} from pixel data {source:#x} (mipmapped {})",
                    source != pixel_data
                ));
            }
            let query = e.call(SIZE_QUERY, &args![size]).u32();
            if query == 1 || e.call(SIZE_QUERY, &args![size]).u32() == 0 {
                e.call(TEXTURE_ASSIGN, &args![texture, made]);
            } else {
                let raw = e.call(TEXTURE_GET, &args![made]).u32();
                let resized = e.call(TEXTURE_RESIZE, &args![raw, size]).u32();
                e.call(TEXTURE_SET, &args![texture, resized]);
            }
            if e.call(TEXTURE_GET, &args![texture]).u32() == 0 {
                e.call(TEXTURE_ASSIGN, &args![texture, made]);
            }
            e.call(TEXTURE_DESTRUCT, &args![made]);
            e.call(CONVERTER_DESTRUCT, &args![converter]);
        });
    }
    e.call(TEXTURE_GET, &args![texture]).u32()
}

// Translated from 0064d870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers the byte at `011f94b0` (no arguments).
pub fn fn_0064d870(e: &mut Engine) -> u8 {
    e.mem.u8(DEBUG_TEXTURE_FLAG)
}

// Translated from 0064db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the values container's clear (`thiscall`): its end iterator
/// (`0065f550`) and begin iterator (`0044a170`), each copied out of its result, then the
/// range erase (`0064dc50`) from begin to end with a result temporary.
pub fn fn_0064db70(e: &mut Engine, this: Ptr) {
    e.with_stack(0x18, |e, locals| {
        let end_out = locals.addr();
        let begin_out = end_out + 8;
        let result = begin_out + 8;
        let end = e.call(VALUES_END, &args![this, end_out]).u32();
        let end = (e.mem.u32(end), e.mem.u32(end + 4));
        let begin = e.call(VALUES_BEGIN, &args![this, begin_out]).u32();
        let begin = (e.mem.u32(begin), e.mem.u32(begin + 4));
        e.call(
            VALUES_ERASE,
            &args![this, result, begin.0, begin.1, end.0, end.1],
        );
    });
}

// Translated from 0064dbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the values iterator's post-increment (`thiscall`, `RET 8`: the
/// out iterator and the unread `int` of `operator++(int)`): its two words saved, the step
/// (`0064ddf0`), the saved words written to `out`. Answers `out`.
pub fn fn_0064dbf0(e: &mut Engine, this: Ptr, out: u32, _unused_2: u32) -> u32 {
    let first = e.mem.u32(this.addr());
    let second = e.mem.u32(this.addr() + 4);
    e.call(ITERATOR_STEP, &args![this]);
    e.mem.set_u32(out, first);
    e.mem.set_u32(out + 4, second);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_cdf0, bs_face_gen_image_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0064_ce20, fn_0064ce20(Ptr)),
        entry!(
            0x0064_ceb0,
            bs_face_gen_image_get_compressed_image(Ptr, f32, f32, f32, u32) -> u32
        ),
        entry!(0x0064_d870, fn_0064d870() -> u8),
        entry!(0x0064_db70, fn_0064db70(Ptr)),
        entry!(0x0064_dbf0, fn_0064dbf0(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_face_gen_image_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0064_cdf0, 0x0064_ce20, 0x2c);
    }

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn image_destructor_erases_values_and_releases_the_texture() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                TEXTURE_SET,
                VALUES_DESTRUCT,
                TEXTURE_DESTRUCT,
                BASE_DESTRUCT,
                VALUES_ERASE,
            ],
        );
        e.register(VALUES_END, |e, a| {
            e.mem.set_u32(a[1], 0xe0);
            e.mem.set_u32(a[1] + 4, 0xe4);
            a[1].into_ret()
        });
        e.register(VALUES_BEGIN, |e, a| {
            e.mem.set_u32(a[1], 0xb0);
            e.mem.set_u32(a[1] + 4, 0xb4);
            a[1].into_ret()
        });
        e.call_log = Some(vec![]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x2c));
        fn_0064ce20(&mut e, this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr()), BS_FACE_GEN_IMAGE_VTABLE);
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                VALUES_END,
                VALUES_BEGIN,
                VALUES_ERASE,
                TEXTURE_SET,
                VALUES_DESTRUCT,
                TEXTURE_DESTRUCT,
                BASE_DESTRUCT
            ]
        );
        let erase = &calls(&log, VALUES_ERASE)[0];
        assert_eq!(erase[0], this.addr() + 0xc);
        assert_eq!(&erase[2..], &[0xb0, 0xb4, 0xe0, 0xe4]);
        assert_eq!(calls(&log, TEXTURE_SET), vec![vec![this.addr() + 8, 0]]);
    }

    /// An image of 2x1 pixels over the values at `values` (four floats each); the size query
    /// answers `query`; the mipmap setting is off; the made texture is 0x7700.
    fn image_engine(query: u32) -> (Engine, Ptr, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        quiet(
            &mut e,
            &[
                CONVERTER_CONSTRUCT,
                CONVERTER_DESTRUCT,
                TEXTURE_CONSTRUCT,
                TEXTURE_DESTRUCT,
                TEXTURE_ASSIGN,
                VALUES_EMPTY,
                MIPMAP_SETTING,
                DEBUG_TEXTURE,
            ],
        );
        e.register(TEXTURE_GET, |_, _| Ret::default());
        e.register(TEXTURE_SET, |_, _| Ret::default());
        e.register_double(SIZE_QUERY, move |_, _| query.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PIXEL_DATA_CONSTRUCT, |e, a| {
            let bytes = e.mem.alloc(8);
            e.mem.set_u32(a[0] + 0x40, bytes);
            a[0].into_ret()
        });
        e.register(PIXEL_BYTES, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(SOURCE_TEXTURE_CREATE, |_, _| 0x7700u32.into_ret());
        e.register(TEXTURE_RESIZE, |_, _| 0x7800u32.into_ret());
        e.register(FLOAT_FLOOR, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).floor(),
            ..Ret::default()
        });
        let values = e.mem.alloc(32);
        for (i, v) in [0.5f32, 3.7, -1.0, 9.0, 2.2, 1.0, f32::NAN, 4.0]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(values + 4 * i as u32, v.to_bits());
        }
        e.register(VALUES_BEGIN, |e, a| {
            let first = e.mem.u32(a[0] + 0x40);
            e.mem.set_u32(a[1], first);
            a[1].into_ret()
        });
        e.register(ITERATOR_VALUE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ITERATOR_STEP, |e, a| {
            let at = e.mem.u32(a[0]) + 16;
            e.mem.set_u32(a[0], at);
            Ret::default()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u32(this.addr() + 0xc + 0x40, values);
        e.mem.set_u32(this.addr() + 0x24, 2);
        e.mem.set_u32(this.addr() + 0x28, 1);
        e.call_log = Some(vec![]);
        (e, this, values)
    }

    #[test]
    fn compressed_image_packs_clamped_floored_channels() {
        // Three channels: range [0, 4], scale 10.
        let (mut e, this, _) = image_engine(0);
        bs_face_gen_image_get_compressed_image(&mut e, this, 0.0, 4.0, 10.0, 0x55);
        let log = e.call_log.take().unwrap();
        let construct = &calls(&log, PIXEL_DATA_CONSTRUCT)[0];
        assert_eq!(&construct[1..], &[2, 1, FORMAT_THREE_CHANNELS, 1, 1]);
        let block = construct[0];
        let bytes = e.mem.u32(block + 0x40);
        // 0.5 -> 0; 3.7 -> 30; -1 -> 0 (clamped); 2.2 -> 20; 1.0 -> 10; NaN -> min, 0.
        assert_eq!(e.mem.bytes(bytes, 6), vec![0, 30, 0, 20, 10, 0]);
        // Query 0: the made texture becomes the image's.
        assert_eq!(
            calls(&log, TEXTURE_ASSIGN)[0],
            vec![this.addr() + 8, calls(&log, TEXTURE_CONSTRUCT)[0][0]]
        );
        assert_eq!(calls(&log, ITERATOR_STEP).len(), 2);
        assert_eq!(
            calls(&log, SOURCE_TEXTURE_CREATE)[0],
            vec![block, 0, TEXTURE_PREFERENCES]
        );
        // Four channels and a size query of 2: the resize is stored, 9.0 clamps to 4.
        let (mut e, this, _) = image_engine(2);
        bs_face_gen_image_get_compressed_image(&mut e, this, 0.0, 4.0, 10.0, 0x55);
        let log = e.call_log.take().unwrap();
        let block = calls(&log, PIXEL_DATA_CONSTRUCT)[0][0];
        let bytes = e.mem.u32(block + 0x40);
        assert_eq!(e.mem.bytes(bytes, 8), vec![0, 30, 0, 40, 20, 10, 0, 40]);
        assert_eq!(calls(&log, TEXTURE_RESIZE), vec![vec![0, 0x55]]);
        assert!(calls(&log, TEXTURE_SET).contains(&vec![this.addr() + 8, 0x7800]));
    }

    #[test]
    fn compressed_image_needs_range_values_and_no_texture() {
        let (mut e, this, _) = image_engine(0);
        // An empty range: only the texture is read back.
        bs_face_gen_image_get_compressed_image(&mut e, this, 4.0, 4.0, 10.0, 0x55);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, CONVERTER_CONSTRUCT).is_empty());
        assert_eq!(calls(&log, TEXTURE_GET).len(), 2);
        // An existing texture: nothing is built.
        e.register(TEXTURE_GET, |_, _| 0x1234u32.into_ret());
        e.call_log = Some(vec![]);
        let texture = bs_face_gen_image_get_compressed_image(&mut e, this, 0.0, 4.0, 10.0, 0x55);
        assert_eq!(texture, 0x1234);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, VALUES_EMPTY).is_empty());
    }

    #[test]
    fn mipmaps_replace_and_delete_the_pixel_data() {
        let (mut e, this, _) = image_engine(1);
        e.register(MIPMAP_SETTING, |_, _| 1u32.into_ret());
        e.register(CONVERTER_MIPMAPS, |_, _| 0x6600u32.into_ret());
        e.register(0x0040_7700, |_, _| Ret::default());
        e.put_vtable(0x0090_7000, &[0x0040_7700]);
        e.register(PIXEL_DATA_CONSTRUCT, |e, a| {
            let bytes = e.mem.alloc(8);
            e.mem.set_u32(a[0] + 0x40, bytes);
            e.mem.set_u32(a[0], 0x0090_7000);
            a[0].into_ret()
        });
        e.mem.set_u8(DEBUG_TEXTURE_FLAG, 1);
        bs_face_gen_image_get_compressed_image(&mut e, this, 0.0, 4.0, 10.0, 0x55);
        let log = e.call_log.take().unwrap();
        let block = calls(&log, PIXEL_DATA_CONSTRUCT)[0][0];
        assert_eq!(calls(&log, 0x0040_7700), vec![vec![block, 1]]);
        assert_eq!(calls(&log, DEBUG_TEXTURE), vec![vec![0x6600]]);
        assert_eq!(calls(&log, SOURCE_TEXTURE_CREATE)[0][0], 0x6600);
        assert_eq!(calls(&log, SIZE_QUERY).len(), 2);
    }

    #[test]
    fn debug_flag_is_the_global_byte() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        assert_eq!(fn_0064d870(&mut e), 0);
        e.mem.set_u8(DEBUG_TEXTURE_FLAG, 5);
        assert_eq!(fn_0064d870(&mut e), 5);
    }

    #[test]
    fn values_clear_erases_begin_to_end() {
        let mut e = Engine::new();
        e.register(VALUES_ERASE, |_, _| Ret::default());
        e.register(VALUES_END, |e, a| {
            e.mem.set_u32(a[1], 3);
            e.mem.set_u32(a[1] + 4, 4);
            a[1].into_ret()
        });
        e.register(VALUES_BEGIN, |e, a| {
            e.mem.set_u32(a[1], 1);
            e.mem.set_u32(a[1] + 4, 2);
            a[1].into_ret()
        });
        e.call_log = Some(vec![]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        fn_0064db70(&mut e, this);
        let log = e.call_log.take().unwrap();
        assert_eq!(&calls(&log, VALUES_ERASE)[0][2..], &[1, 2, 3, 4]);
    }

    #[test]
    fn iterator_post_increment_answers_the_old_position() {
        let mut e = Engine::new();
        e.register(ITERATOR_STEP, |e, a| {
            let at = e.mem.u32(a[0]) + 16;
            e.mem.set_u32(a[0], at);
            Ret::default()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(this.addr(), 0x100);
        e.mem.set_u32(this.addr() + 4, 0x200);
        let out = e.mem.alloc(8);
        assert_eq!(fn_0064dbf0(&mut e, this, out, 0), out);
        assert_eq!(e.mem.u32(out), 0x100);
        assert_eq!(e.mem.u32(out + 4), 0x200);
        assert_eq!(e.mem.u32(this.addr()), 0x110);
    }
}
