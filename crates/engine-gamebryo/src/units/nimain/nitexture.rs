//! `NiMain/nitexture.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The function pointer `00a62a10` calls through (`011a9600`; set to
/// `00a62690` on first use when null).
const BUILDER_POINTER: u32 = 0x011a_9600;
/// `00a62690` itself, the pointer's default.
const DEFAULT_BUILDER: u32 = 0x00a6_2690;
/// The ten cached objects (`011f4514`, one word each, indexed by the low
/// 31 bits of the request).
const CACHE: u32 = 0x011f_4514;
/// `00a62330` (cdecl: two four-`float` colours, the word table at
/// `011f4600`, a count; returns the new object) and the cache slot's
/// assignment (`00a5f860`, thiscall on the slot, the object).
const BUILD_00A62330: u32 = 0x00a6_2330;
const BUILD_TABLE: u32 = 0x011f_4600;
const SLOT_ASSIGN_00A5F860: u32 = 0x00a5_f860;

/// `1.0` and `0.0` as the code stores them (`FLD1`/`FLDZ`).
const ONE: u32 = 0x3f80_0000;
const ZERO: u32 = 0;

/// The two colours (the second and first arguments of `00a62330`, four
/// `float`s each) and the count for request `request`: the code's jump
/// table at `00a629ec` (requests `0x80000001` to `0x80000009`), any other
/// request taking the table's default block.
fn colours(request: u32) -> ([u32; 4], [u32; 4], u32) {
    match request {
        0x8000_0001 => ([ONE, ONE, ZERO, ONE], [ZERO, ZERO, ONE, ONE], 1),
        0x8000_0002 => ([ONE, ZERO, ZERO, ONE], [ZERO, ONE, ZERO, ONE], 6),
        0x8000_0003 => ([ONE, ONE, ZERO, ONE], [ZERO, ONE, ONE, ONE], 1),
        0x8000_0004 => ([ONE, ONE, ZERO, ONE], [ONE, ZERO, ZERO, ONE], 1),
        0x8000_0006 => ([ZERO, ONE, ONE, ONE], [ZERO, ONE, ONE, ONE], 1),
        0x8000_0007 => ([ONE, ONE, ZERO, ONE], [ZERO, ONE, ZERO, ONE], 6),
        0x8000_0008 => ([ONE, ZERO, ONE, ONE], [ZERO, ONE, ZERO, ONE], 6),
        0x8000_0009 => ([ZERO, ONE, ONE, ONE], [ZERO, ONE, ZERO, ONE], 6),
        // 0x80000005 and every request outside the table.
        _ => ([ONE, ZERO, ONE, ONE], [ONE, ZERO, ONE, ONE], 1),
    }
}

// Translated from 00a62690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cached object for `request` (cdecl, two words, the first unused;
/// unnamed in the engine map): 0 when the low 31 bits of the request are
/// 10 or more; otherwise the cache slot `011f4514[request & 0x7fffffff]`,
/// filled on first use with `00a62330(second colour, first colour,
/// 011f4600, count)` (the colours and count of the request, see
/// [`colours`]) through `00a5f860`. Returns the slot's object.
pub fn fn_00a62690(e: &mut Engine, _unused_1: u32, request: u32) -> u32 {
    let index = request & 0x7fff_ffff;
    if index >= 10 {
        return 0;
    }
    let slot = CACHE + index * 4;
    if e.mem.u32(slot) == 0 {
        let (first, second, count) = colours(request);
        e.with_stack(0x20, |e, frame| {
            let first_colour = frame.addr();
            let second_colour = frame.addr() + 0x10;
            for i in 0..4 {
                e.mem.set_u32(first_colour + i * 4, first[i as usize]);
                e.mem.set_u32(second_colour + i * 4, second[i as usize]);
            }
            let object = e
                .call(
                    BUILD_00A62330,
                    &args![second_colour, first_colour, BUILD_TABLE, count],
                )
                .u32();
            e.call(SLOT_ASSIGN_00A5F860, &args![slot, object]);
        });
    }
    e.mem.u32(slot)
}

// Translated from 00a62a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives 0 for a non-negative `request` (cdecl, two words; unnamed in the
/// engine map); otherwise calls the function at `011a9600` (set to
/// `00a62690` first when null) with both words and returns its result.
pub fn fn_00a62a10(e: &mut Engine, first: u32, request: u32) -> u32 {
    if (request as i32) >= 0 {
        return 0;
    }
    let mut builder = e.global::<u32>(BUILDER_POINTER);
    if builder == 0 {
        builder = DEFAULT_BUILDER;
        e.set_global(BUILDER_POINTER, builder);
    }
    e.call(builder, &args![first, request]).u32()
}

/// `NiTexture::m_pkRendererData` (Xbox PDB) +0x24: `NiTexture::RendererData*`.
const RENDERER_DATA: u32 = 0x24;
/// `NiTexture::RendererData` (Xbox PDB): `m_uiWidth` +0x0C, `m_uiHeight` +0x10,
/// the bits-per-pixel byte of `m_kPixelFormat` (+0x14, `NiPixelFormat` +1) at +0x15.
const RENDERER_DATA_WIDTH: u32 = 0x0c;
const RENDERER_DATA_HEIGHT: u32 = 0x10;
const RENDERER_DATA_BITS_PER_PIXEL: u32 = 0x15;
/// The byte of the renderer data `GetViewerStrings` shows as `"State"` (+0x5B;
/// the Xbox PDB's `RendererData` puts `cFlags` at +0x5C, so the PC field is unnamed).
const RENDERER_DATA_STATE: u32 = 0x5b;

/// `NiObjectNET::GetViewerStrings` (thiscall, the strings array).
const OBJECT_NET_GET_VIEWER_STRINGS: u32 = 0x00a5_ca80;
/// `NiGetViewerString` (cdecl, the class name) and `NiGetViewerString_ov7`
/// (cdecl, a name and an `int`, `"%s = %d"`): each returns a new string.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_BYTE: u32 = 0x00a7_d4d0;
/// The strings array's `SetSize` (thiscall, the new size) and `SetAt`
/// (thiscall, the index and the address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `NiTexture::ms_RTTI` (`011f450c`): its first word is the class name.
const NI_TEXTURE_RTTI: u32 = 0x011f_450c;
/// `"State"`.
const TEXT_STATE: u32 = 0x0104_4a7c;

/// Adds the string word at `slot` at the end of the strings array (`SetSize`
/// to the size plus the grow-by step when the array is full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = u32::from(e.get(strings, NiTArray::m_usSize));
    if index >= u32::from(e.get(strings, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a62670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexture::GetMemorySize` (Xbox PDB; thiscall, no arguments): 0 without
/// renderer data; otherwise its bits per pixel shifted right by 3 (bytes per
/// pixel) times its height times its width, as a 32-bit product.
pub fn ni_texture_get_memory_size(e: &mut Engine, this: Ptr) -> u32 {
    let data = e.mem.u32(this.addr() + RENDERER_DATA);
    if data == 0 {
        return 0;
    }
    let bytes_per_pixel = u32::from(e.mem.u8(data + RENDERER_DATA_BITS_PER_PIXEL) >> 3);
    bytes_per_pixel
        .wrapping_mul(e.mem.u32(data + RENDERER_DATA_HEIGHT))
        .wrapping_mul(e.mem.u32(data + RENDERER_DATA_WIDTH))
}

// Translated from 00a62a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexture::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiObjectNET::GetViewerStrings`, then the class name (the first word of
/// `NiTexture::ms_RTTI`) and `"State"` with the renderer data's byte at +0x5B
/// (0 without renderer data), each held in the argument's stack word and
/// added at the array's end.
pub fn ni_texture_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr<NiTArray>) {
    e.call(OBJECT_NET_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_TEXTURE_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let data = e.mem.u32(this.addr() + RENDERER_DATA);
        let state = if data == 0 {
            0
        } else {
            u32::from(e.mem.u8(data + RENDERER_DATA_STATE))
        };
        let text = e.call(VIEWER_STRING_BYTE, &args![TEXT_STATE, state]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_2690, fn_00a62690(u32, u32) -> u32),
        entry!(0x00a6_2a10, fn_00a62a10(u32, u32) -> u32),
        entry!(0x00a6_2670, ni_texture_get_memory_size(Ptr) -> u32),
        entry!(
            0x00a6_2a40,
            ni_texture_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Where the build double records the colours it is given.
    const SEEN: u32 = 0x0343_0000;

    fn setup(e: &mut Engine) {
        e.map(CACHE, 0x28);
        e.map(BUILDER_POINTER, 4);
        e.map(SEEN, 0x40);
        e.register(BUILD_00A62330, |e, a| {
            assert_eq!(a[2], BUILD_TABLE);
            let first = e.mem.bytes(a[0], 16);
            let second = e.mem.bytes(a[1], 16);
            e.mem.write(SEEN, &first);
            e.mem.write(SEEN + 0x10, &second);
            e.mem.set_u32(SEEN + 0x20, a[3]);
            0x0abc_0000u32.into_ret()
        });
        e.register(SLOT_ASSIGN_00A5F860, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    fn words(e: &Engine, address: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(address + i * 4)).collect()
    }

    #[test]
    fn builds_a_cache_slot_once_with_the_requests_colours() {
        let mut e = Engine::new();
        setup(&mut e);
        assert_eq!(
            e.call(0x00a6_2690, &args![0u32, 0x8000_0002u32]).u32(),
            0x0abc_0000
        );
        assert_eq!(e.mem.u32(CACHE + 8), 0x0abc_0000);
        assert_eq!(words(&e, SEEN), vec![ZERO, ONE, ZERO, ONE]);
        assert_eq!(words(&e, SEEN + 0x10), vec![ONE, ZERO, ZERO, ONE]);
        assert_eq!(e.mem.u32(SEEN + 0x20), 6);
        // Cached: no second build.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a6_2690, &args![0u32, 0x8000_0002u32]).u32(),
            0x0abc_0000
        );
        assert!(!e
            .call_log
            .iter()
            .flatten()
            .any(|(a, _)| *a == BUILD_00A62330));
        // A request outside the table takes the default colours.
        e.call(0x00a6_2690, &args![0u32, 0x8000_0000u32]);
        assert_eq!(words(&e, SEEN), vec![ONE, ZERO, ONE, ONE]);
        assert_eq!(e.mem.u32(SEEN + 0x20), 1);
        // Out of range.
        assert_eq!(e.call(0x00a6_2690, &args![0u32, 0x8000_000au32]).u32(), 0);
    }

    #[test]
    fn memory_size_is_bytes_per_pixel_times_height_times_width() {
        let mut e = Engine::new();
        let texture = e.mem.alloc(0x30);
        assert_eq!(e.call(0x00a6_2670, &args![texture]).u32(), 0);
        let data = e.mem.alloc(0x60);
        e.mem.set_u32(texture + RENDERER_DATA, data);
        e.mem.set_u32(data + RENDERER_DATA_WIDTH, 64);
        e.mem.set_u32(data + RENDERER_DATA_HEIGHT, 32);
        e.mem.set_u8(data + RENDERER_DATA_BITS_PER_PIXEL, 37);
        assert_eq!(e.call(0x00a6_2670, &args![texture]).u32(), 4 * 64 * 32);
    }

    #[test]
    fn viewer_strings_add_the_class_name_and_the_state() {
        let mut e = Engine::new();
        e.map(NI_TEXTURE_RTTI, 4);
        e.set_global(NI_TEXTURE_RTTI, 0x0104_0000u32);
        e.register(OBJECT_NET_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_BYTE, |_, a| {
            assert_eq!(a[0], TEXT_STATE);
            (0x0700_0000 + a[1]).into_ret()
        });
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(SEEN + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.map(SEEN, 0x40);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 1);
        let texture = e.mem.alloc(0x30);
        e.call(0x00a6_2a40, &args![texture, strings]);
        assert_eq!(e.mem.u32(SEEN), 0x0104_0001);
        assert_eq!(e.mem.u32(SEEN + 4), 0x0700_0000);
        assert_eq!(e.mem.u16(strings + 8), 2);
        let data = e.mem.alloc(0x60);
        e.mem.set_u8(data + RENDERER_DATA_STATE, 3);
        e.mem.set_u32(texture + RENDERER_DATA, data);
        e.call(0x00a6_2a40, &args![texture, strings]);
        assert_eq!(e.mem.u32(SEEN + 12), 0x0700_0003);
    }

    #[test]
    fn negative_requests_go_through_the_pointer_set_on_first_use() {
        let mut e = Engine::new();
        setup(&mut e);
        assert_eq!(e.call(0x00a6_2a10, &args![0u32, 5u32]).u32(), 0);
        assert_eq!(e.mem.u32(BUILDER_POINTER), 0);
        assert_eq!(
            e.call(0x00a6_2a10, &args![7u32, 0x8000_0009u32]).u32(),
            0x0abc_0000
        );
        assert_eq!(e.mem.u32(BUILDER_POINTER), DEFAULT_BUILDER);
        // A pointer already set is used as it is.
        e.register(0x0055_0000, |_, a| (a[0] + a[1]).into_ret());
        e.mem.set_u32(BUILDER_POINTER, 0x0055_0000);
        assert_eq!(
            e.call(0x00a6_2a10, &args![1u32, 0x8000_0001u32]).u32(),
            0x8000_0002
        );
    }
}
