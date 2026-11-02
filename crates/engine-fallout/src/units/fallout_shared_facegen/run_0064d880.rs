//! `fallout shared/facegen/run_0064d880` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A face-generation helper that keeps a texture (`NiPointer` at `+8`) and, from `+0xc`, a
//! vector of `NiColorA`s it fills from the texture's pixels the first time they are asked
//! for (`0064d880`), with the vector's and the pixel data's small accessors the compiler
//! placed after it. The code is compiled without optimisation; every accessor is a call.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPointer<T>::get` (`00559450`), the `NiPointer` constructor from a raw pointer
/// (`00633c90`) and destructor (`0045cec0`) (`thiscall`).
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_POINTER_FROM_RAW: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `00b6bbd0` (`cdecl`: the texture, the format record `011aa2a0`; answers the converted
/// texture).
const CONVERT_TEXTURE_00B6BBD0: u32 = 0x00b6_bbd0;
const FORMAT_011AA2A0: u32 = 0x011a_a2a0;
/// The texture's pixel data (`004b70d0`, `thiscall`), its width and height (`0064da80`,
/// `0064daa0`; `thiscall`, two words).
const PIXEL_DATA: u32 = 0x004b_70d0;
const PIXEL_WIDTH: u32 = 0x0064_da80;
const PIXEL_HEIGHT: u32 = 0x0064_daa0;
/// The colour vector: its size (`0064dc30`), element address (`0064db20`, one word) and
/// resize-with-value (`0064dcf0`, two words) (`thiscall`).
const VECTOR_SIZE: u32 = 0x0064_dc30;
const VECTOR_ELEMENT: u32 = 0x0064_db20;
const VECTOR_RESIZE: u32 = 0x0064_dcf0;

// Translated from 0064d880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the colour vector
/// (`this + 0xc`). When it is empty and a texture is held, converts the texture
/// (`00b6bbd0` with `011aa2a0`) into a local `NiPointer`; with a result, resizes the vector
/// to width * height of the result's pixel data (filled with `(0, 0, 0, 0)`) and stores
/// each pixel's four bytes of level 0 as `float`s into its colour.
// Translation incomplete: 0064d880 - the C++ exception frame (FS:[0] chain, unwind state 0 for the local NiPointer) is not translated.
pub fn fn_0064d880(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let colors = base + 0xc;
    if !fn_0064db00(e, Ptr::new(colors)) {
        return colors;
    }
    if e.call(NI_POINTER_GET, &args![base + 8]).u32() == 0 {
        return colors;
    }
    let texture = e.call(NI_POINTER_GET, &args![base + 8]).u32();
    let converted = e
        .call(CONVERT_TEXTURE_00B6BBD0, &args![texture, FORMAT_011AA2A0])
        .u32();
    let pointer = e.mem.alloc(4);
    e.call(NI_POINTER_FROM_RAW, &args![pointer, converted]);
    if e.call(NI_POINTER_GET, &args![pointer]).u32() != 0 {
        let texture = e.call(NI_POINTER_GET, &args![pointer]).u32();
        let data = e.call(PIXEL_DATA, &args![texture]).u32();
        let pixels = fn_0064dac0(e, data, 0, 0);
        let width = e.call(PIXEL_WIDTH, &args![data, 0u32, 0u32]).u32();
        let height = e.call(PIXEL_HEIGHT, &args![data, 0u32, 0u32]).u32();
        let count = width.wrapping_mul(height);
        e.with_stack(16, |e, zero| {
            for word in 0..4 {
                e.mem.set_f32(zero.addr() + 4 * word, 0.0);
            }
            fn_0064db50(e, Ptr::new(colors), count, zero.addr());
        });
        let mut index = 0u32;
        while index < count {
            for channel in 0..4u32 {
                let byte = e.mem.u8(pixels
                    .wrapping_add(index.wrapping_mul(4))
                    .wrapping_add(channel));
                let value = x87_store32(f64::from(byte));
                let element = e.call(VECTOR_ELEMENT, &args![colors, index]).u32();
                e.mem.set_u32(element + 4 * channel, value);
            }
            index += 1;
        }
    }
    e.call(NI_POINTER_DESTRUCT, &args![pointer]);
    e.mem.free(pointer);
    colors
}

// Translated from 0064dac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on pixel data, two words: `level`, `face`): the
/// address `+0x50` plus `face` times the offset table's (`+0x5c`) entry at the level count
/// (`+0x60`) plus the table's entry for `level`.
pub fn fn_0064dac0(e: &mut Engine, data: u32, level: u32, face: u32) -> u32 {
    let table = e.mem.u32(data + 0x5c);
    let levels = e.mem.u32(data + 0x60);
    let face_size = e.mem.u32(table.wrapping_add(levels.wrapping_mul(4)));
    let start = e
        .mem
        .u32(data + 0x50)
        .wrapping_add(face.wrapping_mul(face_size));
    let table = e.mem.u32(data + 0x5c);
    start.wrapping_add(e.mem.u32(table.wrapping_add(level.wrapping_mul(4))))
}

// Translated from 0064db00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the colour vector): whether its size
/// (`0064dc30`) is 0.
pub fn fn_0064db00(e: &mut Engine, this: Ptr) -> bool {
    e.call(VECTOR_SIZE, &args![this]).u32() == 0
}

// Translated from 0064db50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the colour vector, two words: the count, the
/// address of the fill value): `0064dcf0` with the same words.
pub fn fn_0064db50(e: &mut Engine, this: Ptr, count: u32, value: u32) {
    e.call(VECTOR_RESIZE, &args![this, count, value]);
}

/// `_invalid_parameter_noinfo` (`00ec7c56`).
const INVALID_PARAMETER: u32 = 0x00ec_7c56;

// Translated from 0064da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on pixel data, two words: the level and one not
/// read): the width of `level` (the word table at `+0x54`).
pub fn fn_0064da80(e: &mut Engine, this: u32, level: u32, _unused_1: u32) -> u32 {
    let widths = e.mem.u32(this.wrapping_add(0x54));
    e.mem.u32(widths.wrapping_add(level.wrapping_mul(4)))
}

// Translated from 0064daa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on pixel data, two words: the level and one not
/// read): the height of `level` (the word table at `+0x58`).
pub fn fn_0064daa0(e: &mut Engine, this: u32, level: u32, _unused_1: u32) -> u32 {
    let heights = e.mem.u32(this.wrapping_add(0x58));
    e.mem.u32(heights.wrapping_add(level.wrapping_mul(4)))
}

// Translated from 0064db20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the colour vector's checked `operator[]` (`thiscall`, one
/// word): `_invalid_parameter_noinfo` when `index` is not below the size (`0064dc30`), then
/// the element address `first + 16 * index` (the first pointer at `+0xc`).
pub fn fn_0064db20(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let size = e.call(VECTOR_SIZE, &args![this]).u32();
    if index >= size {
        e.call(INVALID_PARAMETER, &args![]);
    }
    (index << 4).wrapping_add(e.mem.u32(this.addr() + 0xc))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_d880, fn_0064d880(Ptr) -> u32),
        entry!(0x0064_dac0, fn_0064dac0(u32, u32, u32) -> u32),
        entry!(0x0064_db00, fn_0064db00(Ptr) -> bool),
        entry!(0x0064_db50, fn_0064db50(Ptr, u32, u32)),
        entry!(0x0064_da80, fn_0064da80(u32, u32, u32) -> u32),
        entry!(0x0064_daa0, fn_0064daa0(u32, u32, u32) -> u32),
        entry!(0x0064_db20, fn_0064db20(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
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

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Pixel data whose level 0 starts at `pixels`.
    fn pixel_data(e: &mut Engine, pixels: u32) -> u32 {
        let data = e.mem.alloc(0x64);
        let table = e.mem.alloc(8);
        e.mem.set_u32(data + 0x50, pixels);
        e.mem.set_u32(data + 0x5c, table);
        e.mem.set_u32(data + 0x60, 1);
        e.mem.set_u32(table, 0);
        e.mem.set_u32(table + 4, 0x100);
        data
    }

    #[test]
    fn pixel_address_uses_the_offset_table() {
        let mut e = Engine::new();
        let data = pixel_data(&mut e, 0x1000);
        assert_eq!(e.call(0x0064_dac0, &args![data, 0u32, 0u32]).u32(), 0x1000);
        assert_eq!(e.call(0x0064_dac0, &args![data, 1u32, 2u32]).u32(), 0x1300);
    }

    #[test]
    fn empty_test_and_resize_forward_to_the_vector() {
        let mut e = Engine::new();
        e.register(VECTOR_SIZE, |e, a| answer(e.mem.u32(a[0])));
        e.register(VECTOR_RESIZE, |_, _| Ret::default());
        let vector = e.mem.alloc(16);
        assert!(e.call(0x0064_db00, &args![vector]).bool());
        e.mem.set_u32(vector, 3);
        assert!(!e.call(0x0064_db00, &args![vector]).bool());
        e.call_log = Some(vec![]);
        e.call(0x0064_db50, &args![vector, 7u32, 0x40u32]);
        assert_eq!(calls(&e, VECTOR_RESIZE), vec![vec![vector, 7, 0x40]]);
    }

    #[test]
    fn colours_are_filled_from_the_texture_once() {
        let mut e = Engine::new();
        e.register(NI_POINTER_GET, |e, a| answer(e.mem.u32(a[0])));
        e.register(NI_POINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            answer(a[0])
        });
        e.register(NI_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(CONVERT_TEXTURE_00B6BBD0, |_, _| answer(0x0000_7000));
        let pixels = e.mem.alloc(8);
        for (i, byte) in [1u8, 2, 3, 4, 250, 0, 128, 255].into_iter().enumerate() {
            e.mem.set_u8(pixels + i as u32, byte);
        }
        let data = pixel_data(&mut e, pixels);
        e.register_double(PIXEL_DATA, move |_, _| answer(data));
        e.register(PIXEL_WIDTH, |_, _| answer(2));
        e.register(PIXEL_HEIGHT, |_, _| answer(1));
        // The vector: a size word and its elements' storage at +4.
        e.register(VECTOR_SIZE, |e, a| answer(e.mem.u32(a[0])));
        e.register(VECTOR_RESIZE, |e, a| {
            let storage = e.mem.alloc(16 * a[1]);
            for i in 0..4 * a[1] {
                let value = e.mem.u32(a[2] + 4 * (i % 4));
                e.mem.set_u32(storage + 4 * i, value);
            }
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, storage);
            Ret::default()
        });
        e.register(VECTOR_ELEMENT, |e, a| {
            answer(e.mem.u32(a[0] + 4) + 16 * a[1])
        });
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x0064_d880, &args![this]).u32(), this + 0xc);
        // No texture yet: nothing.
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.mem.set_u32(this + 8, 0x0000_6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_d880, &args![this]).u32(), this + 0xc);
        assert_eq!(
            calls(&e, CONVERT_TEXTURE_00B6BBD0),
            vec![vec![0x6000, FORMAT_011AA2A0]]
        );
        assert_eq!(calls(&e, VECTOR_RESIZE)[0][1], 2);
        let storage = e.mem.u32(this + 0x10);
        let values: Vec<f32> = (0..8).map(|i| e.mem.f32(storage + 4 * i)).collect();
        assert_eq!(values, vec![1.0, 2.0, 3.0, 4.0, 250.0, 0.0, 128.0, 255.0]);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);
        // Filled: asked again, nothing is converted.
        e.call_log = Some(vec![]);
        e.call(0x0064_d880, &args![this]);
        assert!(calls(&e, CONVERT_TEXTURE_00B6BBD0).is_empty());
        // A failed conversion leaves the vector empty.
        e.mem.set_u32(this + 0xc, 0);
        e.register(CONVERT_TEXTURE_00B6BBD0, |_, _| answer(0));
        e.call_log = Some(vec![]);
        e.call(0x0064_d880, &args![this]);
        assert!(calls(&e, VECTOR_RESIZE).is_empty());
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);
    }
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    #[test]
    fn level_sizes_read_the_width_and_height_tables() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x60);
        let widths = e.mem.alloc(8);
        let heights = e.mem.alloc(8);
        e.mem.set_u32(data + 0x54, widths);
        e.mem.set_u32(data + 0x58, heights);
        e.mem.set_u32(widths + 4, 64);
        e.mem.set_u32(heights + 4, 32);
        assert_eq!(e.call(0x0064_da80, &args![data, 1u32, 9u32]).u32(), 64);
        assert_eq!(e.call(0x0064_daa0, &args![data, 1u32, 9u32]).u32(), 32);
    }

    #[test]
    fn element_address_checks_the_index_against_the_size() {
        let mut e = Engine::new();
        e.register(VECTOR_SIZE, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector + 0xc, 0x4000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_db20, &args![vector, 1u32]).u32(), 0x4010);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != INVALID_PARAMETER));
        // At the size: reported, and the address is still answered.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_db20, &args![vector, 2u32]).u32(), 0x4020);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == INVALID_PARAMETER).count(), 1);
    }
}
