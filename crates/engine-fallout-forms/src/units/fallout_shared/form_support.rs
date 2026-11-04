//! PC helpers needed by the reference/form construction dependency chain.
//!
//! The engine map assigns these to address ranges rather than named source
//! units. This module records dependencies without claiming a source unit.

use crate::prelude::*;
use crate::types::TESForm;

// Translated from 00402d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the `BaseFormComponent` base (Xbox PDB): writes its vtable
/// `01011d68` and returns `this`, with no stack arguments or other stores.
pub fn construct_base_component(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_1d68);
    this
}

// Translated from 004f15a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `TESForm::cFormType` (Xbox PDB) at +4. PC reads only the low byte
/// of the one stack word (`RET 4`) and leaves `this` in EAX.
pub fn set_form_type(e: &mut Engine, this: Ptr<TESForm>, kind: u8) -> Ptr<TESForm> {
    e.set(this, TESForm::cFormType, kind);
    this
}

// Translated from 00486ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the form registry's editor-ID map; the second stack word is
/// narrowed to a byte before the base call. Both stack words are consumed.
pub fn construct_editor_id_map(e: &mut Engine, this: Ptr, buckets: u32, flag: u8) -> Ptr {
    construct_case_insensitive_map(e, this, buckets, flag);
    e.mem.set_u32(this.addr(), 0x0101_cab8);
    this
}

// Translated from 00486ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the string-map base, then installs vtable `0101cb38`.
pub fn construct_case_insensitive_map(e: &mut Engine, this: Ptr, buckets: u32, flag: u8) -> Ptr {
    construct_string_map(e, this, buckets, flag);
    e.mem.set_u32(this.addr(), 0x0101_cb38);
    this
}

// Translated from 00486f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the pointer-map base, installs `0101cad8`, and writes only
/// the flag byte at +0x10 (not the adjacent padding).
pub fn construct_string_map(e: &mut Engine, this: Ptr, buckets: u32, flag: u8) -> Ptr {
    construct_pointer_map(e, this, buckets);
    e.mem.set_u32(this.addr(), 0x0101_cad8);
    e.mem.set_u8(this.addr() + 0x10, flag);
    this
}

// Translated from 00486f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the bucket map, then installs vtable `0101caf8`.
pub fn construct_pointer_map(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    construct_bucket_map(e, this, buckets);
    e.mem.set_u32(this.addr(), 0x0101_caf8);
    this
}

// Translated from 00486fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes count and bucket storage. The PC reloads the bucket count
/// after allocation for memset, and shifts in 32 bits in both places.
pub fn construct_bucket_map(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_cb18);
    e.mem.set_u32(this.addr() + 4, buckets);
    e.mem.set_u32(this.addr() + 0xc, 0);
    let bytes = buckets.wrapping_shl(2);
    let buffer = e.call(0x00aa_1070, &args![bytes]).u32();
    e.mem.set_u32(this.addr() + 8, buffer);
    let bytes = e.mem.u32(this.addr() + 4).wrapping_shl(2);
    e.call(0x0040_3d30, &args![buffer, 0u32, bytes]);
    this
}

// Translated from 004701d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the form registry's array base, then installs `01019a3c`.
pub fn construct_form_array(e: &mut Engine, this: Ptr, capacity: u32, grow_by: u32) -> Ptr {
    construct_form_array_base(e, this, capacity, grow_by);
    e.mem.set_u32(this.addr(), 0x0101_9a3c);
    this
}

// Translated from 004705e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores capacity/growth, clears both used counts, and allocates capacity
/// words only when capacity is nonzero. Both arguments are full words.
pub fn construct_form_array_base(e: &mut Engine, this: Ptr, capacity: u32, grow_by: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_9a34);
    e.mem.set_u32(this.addr() + 8, capacity);
    e.mem.set_u32(this.addr() + 0x14, grow_by);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    let buffer = if capacity > 0 {
        allocate_words(e, capacity)
    } else {
        Ptr::NULL
    };
    e.mem.set_u32(this.addr() + 4, buffer.addr());
    this
}

// Translated from 0096afc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl allocation wrapper: shifts the word count left twice in EAX,
/// forwards the wrapped byte count to `00aa1070`, and returns its result.
pub fn allocate_words(e: &mut Engine, count: u32) -> Ptr {
    e.call(0x00aa_1070, &args![count.wrapping_shl(2)]).ptr()
}

// Translated from 0040b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl wrapper around the PC InterlockedIncrement import; returns EAX.
pub fn increment_counter(e: &mut Engine, target: Ptr<i32>) -> i32 {
    e.call(0x00fd_f1c0, &args![target]).i32()
}

// Translated from 004019a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl wrapper around the PC InterlockedDecrement import; returns EAX.
pub fn decrement_counter(e: &mut Engine, target: Ptr<i32>) -> i32 {
    e.call(0x00fd_f1c4, &args![target]).i32()
}

// Translated from 0044dee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded constructor: clears one word and returns the receiver. Used by
/// the save-form header at its unaligned +3 field; no original class asserted.
pub fn construct_zero_word(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    this
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044dee0, construct_zero_word(Ptr) -> Ptr),
        entry!(0x00402d30, construct_base_component(Ptr) -> Ptr),
        entry!(0x004f15a0, set_form_type(Ptr<TESForm>, u8) -> Ptr<TESForm>),
        entry!(0x00486ab0, construct_editor_id_map(Ptr, u32, u8) -> Ptr),
        entry!(0x00486ee0, construct_case_insensitive_map(Ptr, u32, u8) -> Ptr),
        entry!(0x00486f40, construct_string_map(Ptr, u32, u8) -> Ptr),
        entry!(0x00486f70, construct_pointer_map(Ptr, u32) -> Ptr),
        entry!(0x00486fa0, construct_bucket_map(Ptr, u32) -> Ptr),
        entry!(0x004701d0, construct_form_array(Ptr, u32, u32) -> Ptr),
        entry!(0x004705e0, construct_form_array_base(Ptr, u32, u32) -> Ptr),
        entry!(0x0096afc0, allocate_words(u32) -> Ptr),
        entry!(0x0040b460, increment_counter(Ptr<i32>) -> i32),
        entry!(0x004019a0, decrement_counter(Ptr<i32>) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_wrappers_use_real_imports_and_return_wrapped_new_value() {
        let mut e = Engine::new();
        let imports = [
            (0x00fdf1c0, "InterlockedIncrement"),
            (0x00fdf1c4, "InterlockedDecrement"),
        ]
        .map(|(slot, name)| crate::exe::Import {
            slot,
            dll: "KERNEL32.dll".into(),
            name: name.into(),
        });
        assert_eq!(e.register_imports(&imports), 2);
        let counter = e.mem.alloc(8);
        e.mem.set_i32(counter, i32::MAX);
        e.mem.set_u32(counter + 4, 0xaabbccdd);
        assert_eq!(e.call(0x0040b460, &[counter]).i32(), i32::MIN);
        assert_eq!(e.mem.i32(counter), i32::MIN);
        assert_eq!(e.call(0x004019a0, &[counter]).i32(), i32::MAX);
        assert_eq!(e.mem.i32(counter), i32::MAX);
        assert_eq!(e.mem.u32(counter + 4), 0xaabbccdd);
    }

    // Generated allocator fixture: keep the translated 00aa1070 wrapper
    // real, replacing only the allocator object's virtual allocation method.
    fn engine_with_allocator() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        let holder = e.mem.alloc(4);
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(0x011f_6080, holder);
        e.mem.set_u32(holder, allocator);
        e.put_vtable(0x0300_0000, &[0, 0x0070_0000]);
        e.mem.set_u32(allocator, 0x0300_0000);
        e.register_double(0x0070_0000, |e, args| {
            assert_eq!(e.mem.u32(args[2]), 4);
            assert_eq!(&args[3..], &[5, 0, 0, u32::MAX, 0]);
            let size = e.mem.u32(args[1]);
            e.mem.alloc(size).into_ret()
        });
        e
    }

    // Generated sentinels checked with nv-call on FalloutNV.exe 1.4.0.525,
    // SHA256 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
    // Ryzen AI 9 HX 370, FCW 027F / MXCSR 1F80. No game-function doubles.
    #[test]
    fn base_component_constructor_matches_native_guarded_store() {
        let mut e = Engine::new();
        let object = e.mem.alloc(28);
        e.mem.write(object, &[0xaa; 28]);
        assert_eq!(e.call(0x00402d30, &[object]).u32(), object);
        let mut expected = [0xaa; 28];
        expected[..4].copy_from_slice(&0x0101_1d68u32.to_le_bytes());
        assert_eq!(e.mem.bytes(object, 28), expected);
    }

    #[test]
    fn form_type_setter_matches_native_truncation_and_guard_bytes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(28);
        for kind in [0u32, 58, 255, 511] {
            e.mem.write(object, &[0xaa; 28]);
            assert_eq!(e.call(0x004f15a0, &[object, kind]).u32(), object);
            let mut expected = [0xaa; 28];
            expected[4] = kind as u8;
            assert_eq!(e.mem.bytes(object, 28), expected);
        }
    }

    #[test]
    fn map_constructor_chain_preserves_padding_and_zeroes_real_storage() {
        for (address, vtable, has_flag) in [
            (0x00486ab0, 0x0101cab8, true),
            (0x00486ee0, 0x0101cb38, true),
            (0x00486f40, 0x0101cad8, true),
            (0x00486f70, 0x0101caf8, false),
            (0x00486fa0, 0x0101cb18, false),
        ] {
            let mut e = engine_with_allocator();
            let object = e.mem.alloc(24);
            e.mem.write(object, &[0xaa; 24]);
            let args = if has_flag {
                vec![object, 3, 0x1ff]
            } else {
                vec![object, 3]
            };
            assert_eq!(e.call(address, &args).u32(), object);
            assert_eq!(e.mem.u32(object), vtable);
            assert_eq!(e.mem.u32(object + 4), 3);
            let buffer = e.mem.u32(object + 8);
            assert_ne!(buffer, 0);
            assert_eq!(e.mem.bytes(buffer, 12), [0; 12]);
            assert_eq!(e.mem.u32(object + 0xc), 0);
            assert_eq!(e.mem.u8(object + 0x10), if has_flag { 0xff } else { 0xaa });
            assert_eq!(e.mem.bytes(object + 0x11, 7), [0xaa; 7]);
        }
    }

    #[test]
    fn bucket_map_reloads_count_after_allocator_and_wraps_native_word_arithmetic() {
        let mut e = Engine::new();
        let object = e.mem.alloc(20);
        let buffer = e.mem.alloc(12);
        e.mem.write(buffer, &[0xcc; 12]);
        e.register_double(0x00aa1070, move |e, args| {
            assert_eq!(args, &[4]); // 0x40000001 << 2
            e.mem.set_u32(object + 4, 3);
            buffer.into_ret()
        });
        assert_eq!(e.call(0x00486fa0, &[object, 0x40000001]).u32(), object);
        assert_eq!(e.mem.u32(object + 4), 3);
        assert_eq!(e.mem.u32(object + 8), buffer);
        assert_eq!(e.mem.bytes(buffer, 12), [0; 12]);
    }

    #[test]
    fn form_array_constructors_cover_zero_and_nonzero_capacity_with_full_growth_word() {
        // Both zero-capacity cases were also run natively with nv-call
        // (same build/CPU above): these bytes and RET 8 matched; no fault.
        // Nonzero cases use the generated allocator fixture above.
        for (address, vtable) in [(0x004701d0, 0x01019a3c), (0x004705e0, 0x01019a34)] {
            for capacity in [0, 3] {
                let mut e = engine_with_allocator();
                let object = e.mem.alloc(28);
                e.mem.write(object, &[0xaa; 28]);
                e.call_log = Some(vec![]);
                assert_eq!(
                    e.call(address, &[object, capacity, 0x12345678]).u32(),
                    object
                );
                assert_eq!(e.mem.u32(object), vtable);
                assert_eq!(e.mem.u32(object + 8), capacity);
                assert_eq!(e.mem.u32(object + 0xc), 0);
                assert_eq!(e.mem.u32(object + 0x10), 0);
                assert_eq!(e.mem.u32(object + 0x14), 0x12345678);
                assert_eq!(e.mem.u32(object + 0x18), 0xaaaaaaaa);
                let allocations: Vec<_> = e
                    .call_log
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|(entry, _)| *entry == 0x00aa1070)
                    .collect();
                if capacity == 0 {
                    assert_eq!(e.mem.u32(object + 4), 0);
                    assert!(allocations.is_empty());
                } else {
                    assert_ne!(e.mem.u32(object + 4), 0);
                    assert_eq!(allocations.len(), 1);
                    assert_eq!(allocations[0].1, vec![12]);
                }
            }
        }
    }

    #[test]
    fn word_allocator_wraps_shift_and_forwards_null_result() {
        for (words, bytes) in [(0, 0), (3, 12), (0x40000001, 4), (u32::MAX, 0xfffffffc)] {
            let mut e = Engine::new();
            e.register_double(0x00aa1070, move |_, args| {
                assert_eq!(args, &[bytes]);
                0u32.into_ret()
            });
            assert_eq!(e.call(0x0096afc0, &[words]).u32(), 0);
        }
    }
}
