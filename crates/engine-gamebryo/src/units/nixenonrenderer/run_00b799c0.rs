//! `NiXenonRenderer/run_00b799c0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShader::BSShader` (engine map, `00be2200`, `thiscall`, no stack words).
const BS_SHADER_CONSTRUCT: u32 = 0x00be_2200;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The shader's vtable (`010af2f8`; no RTTI locator before it).
const SHADER_VTABLE_010AF2F8: u32 = 0x010a_f2f8;
/// The `float` whose reciprocal is stored (`012023e0`) and the four words set (`011fd954`
/// the reciprocal, `011fd958`, `011fd95c`, `011fd960` 0.0).
const DIVISOR_012023E0: u32 = 0x0120_23e0;
const FLOATS_011FD954: u32 = 0x011f_d954;

/// A reference-counted object released: `InterlockedDecrement` on its count at +4, deleted
/// through its vtable slot +4 when that answers 0.
fn release_reference(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `NiPointer<T>::operator=(T *)` expanded inline: nothing when the slot holds `object`; else
/// the old object released (when set), the slot set, the new object's count incremented
/// (when set).
fn ni_pointer_assign(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        release_reference(e, old);
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// The slot's object (when set) released and the slot cleared.
fn ni_pointer_clear(e: &mut Engine, slot: u32) {
    let old = e.mem.u32(slot);
    if old != 0 {
        release_reference(e, old);
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 00b79b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b799c0`): a shader constructor that the constructors
/// `00bd44c0`, `00bcd390` and `00bcf7f0` start with, `thiscall` with four stack words (`RET
/// 0x10`), each an object kept by an `NiPointer`. Runs `BSShader::BSShader` (`00be2200`),
/// stores the vtable `010af2f8`, clears the words +0x6c to +0x88, sets +0x68 to 1; assigns the
/// first word to the `NiPointer`s at +0x28 and +0x6c, the others to +0x70, +0x74 and +0x78;
/// clears the `NiPointer`s at +0x80, +0x7c, +0x88 and +0x84 (just cleared, so nothing is
/// released); stores `1.0 / ` the `float` at `012023e0` (x87 `FDIVRP`) in `011fd954` and 0.0 in
/// `011fd958`, `011fd95c`, `011fd960`; clears the word +0x60. Answers `this`.
// Translation incomplete: 00b79b00 - the C++ exception frame (FS:[0] chain, unwind state 8 for the base and the pointers) is not translated.
pub fn fn_00b79b00(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    fourth: Ptr,
) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SHADER_VTABLE_010AF2F8);
    for offset in (0x6cu32..=0x88).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x68, 1);
    ni_pointer_assign(e, base + 0x28, first.addr());
    ni_pointer_assign(e, base + 0x6c, first.addr());
    ni_pointer_assign(e, base + 0x70, second.addr());
    ni_pointer_assign(e, base + 0x74, third.addr());
    ni_pointer_assign(e, base + 0x78, fourth.addr());
    for offset in [0x80u32, 0x7c, 0x88, 0x84] {
        ni_pointer_clear(e, base + offset);
    }
    let divisor = x87_load32(e.global::<u32>(DIVISOR_012023E0));
    let reciprocal = x87_store32(x87_div(1.0, divisor));
    e.set_global::<u32>(FLOATS_011FD954, reciprocal);
    for offset in [8u32, 4, 0xc] {
        e.set_global::<f32>(FLOATS_011FD954 + offset, 0.0);
    }
    e.mem.set_u32(base + 0x60, 0);
    this
}

// Translated from 00b799c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b799c0`): `RemoveAll` of an `NiTObjectArray` of
/// `NiPointer`s (base +4, size +0xa, element count +0xc), `thiscall`, no stack words. Each slot
/// up to the size (read again each time) holding an object releases it (`InterlockedDecrement`
/// on its +4, deleted through vtable slot +4 at 0) and is cleared; then the size and the element
/// count become 0.
// Translation incomplete: 00b799c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for each slot's pointer) is not translated.
pub fn fn_00b799c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u16;
    while index < e.mem.u16(base + 0xa) {
        let slot = e.mem.u32(base + 4).wrapping_add(u32::from(index) * 4);
        ni_pointer_clear(e, slot);
        index = index.wrapping_add(1);
    }
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
}

/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element size, count,
/// constructor, destructor), the `NiPointer` element constructor (`00b54620`) and the element
/// destructor the array is given (`0045cec0`).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const NI_POINTER_CONSTRUCT_00B54620: u32 = 0x00b5_4620;
const NI_POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;

// Translated from 00b79a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b799c0`): `new NiPointer<T>[count]`, `cdecl`, one word
/// (the count). Allocates `count * 4 + 4` bytes from the memory manager (all ones when the
/// product or the sum overflows: `MUL`/`SETO` and `ADD`/`SETC`), stores the count in the first
/// word and constructs the `count` pointers after it (`eh vector constructor iterator`,
/// element size 4, `00b54620`, destructor `0045cec0`); answers the address after the count, or
/// 0 when the allocation failed.
// Translation incomplete: 00b79a60 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b79a60(e: &mut Engine, count: u32) -> Ptr {
    let product = u64::from(count) * 4;
    let bytes = if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    };
    let bytes = bytes.checked_add(4).unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    let array = block.wrapping_add(4);
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            array,
            4u32,
            count,
            NI_POINTER_CONSTRUCT_00B54620,
            NI_POINTER_DESTRUCT_0045CEC0
        ],
    );
    Ptr::new(array)
}

// Translated from 00b79d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the byte at +0x10.
pub fn fn_00b79d70(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x10))
}

// Translated from 00b79d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a byte): stores the
/// byte at +0x24 and answers true.
pub fn fn_00b79d80(e: &mut Engine, this: Ptr, value: u8) -> bool {
    e.mem.set_u8(this.addr().wrapping_add(0x24), value);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_9d70, fn_00b79d70(Ptr) -> u8),
        entry!(0x00b7_9d80, fn_00b79d80(Ptr, u8) -> bool),
        entry!(0x00b7_9a60, fn_00b79a60(u32) -> Ptr),
        entry!(0x00b7_9b00, fn_00b79b00(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00b7_99c0, fn_00b799c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE: u32 = 0x0e80_0000;
    const VTABLE: u32 = 0x0e81_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(BS_SHADER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.map(0x011f_d000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn fn_00b79b00_keeps_the_four_objects() {
        let mut e = engine();
        e.set_global::<f32>(DIVISOR_012023E0, 4.0);
        e.mem.write(FLOATS_011FD954, &[0xff; 0x10]);
        let this = e.mem.alloc(0x90);
        e.mem.write(this, &[0xcc; 0x90]);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x28, old);
        let words = [object(&mut e, 0), object(&mut e, 0), object(&mut e, 0), 0];
        let made = e
            .call(
                0x00b7_9b00,
                &args![this, words[0], words[1], words[2], words[3]],
            )
            .u32();
        assert_eq!(made, this);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010AF2F8);
        assert_eq!(e.mem.u32(this + 0x68), 1);
        assert_eq!(e.mem.u32(this + 0x28), words[0]);
        assert_eq!(e.mem.u32(this + 0x6c), words[0]);
        assert_eq!(e.mem.u32(words[0] + 4), 2);
        assert_eq!(e.mem.u32(this + 0x70), words[1]);
        assert_eq!(e.mem.u32(this + 0x74), words[2]);
        assert_eq!(e.mem.u32(words[2] + 4), 1);
        for offset in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x60] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        let log = e.call_log.take().unwrap();
        let deleted: Vec<_> = log.iter().filter(|c| c.0 == DELETE).collect();
        assert_eq!(deleted.len(), 1);
        assert_eq!(deleted[0].1, vec![old]);
        assert_eq!(e.global::<f32>(FLOATS_011FD954), 0.25);
        for offset in [4u32, 8, 0xc] {
            assert_eq!(e.global::<u32>(FLOATS_011FD954 + offset), 0);
        }
    }

    #[test]
    fn fn_00b79b00_reciprocal_of_zero_is_infinite() {
        let mut e = engine();
        let this = e.mem.alloc(0x90);
        e.call(0x00b7_9b00, &args![this, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.global::<f32>(FLOATS_011FD954), f32::INFINITY);
    }
}

#[cfg(test)]
mod remove_all_tests {
    use super::*;

    const DELETE: u32 = 0x0e00_0004;
    const VTABLE: u32 = 0x0e01_0000;

    #[test]
    fn fn_00b799c0_releases_every_element() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        let objects = [e.mem.alloc(8), e.mem.alloc(8)];
        e.mem.set_u32(objects[0], VTABLE);
        e.mem.set_u32(objects[0] + 4, 1);
        e.mem.set_u32(objects[1], VTABLE);
        e.mem.set_u32(objects[1] + 4, 2);
        let array = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0xc);
        e.mem.set_u32(elements, objects[0]);
        e.mem.set_u32(elements + 8, objects[1]);
        e.mem.set_u32(array + 4, elements);
        e.mem.set_u16(array + 0xa, 3);
        e.mem.set_u16(array + 0xc, 2);
        e.call(0x00b7_99c0, &args![array]);
        assert_eq!(e.mem.bytes(elements, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(objects[1] + 4), 1);
        assert_eq!(e.mem.u16(array + 0xa), 0);
        assert_eq!(e.mem.u16(array + 0xc), 0);
        let log = e.call_log.take().unwrap();
        let deleted: Vec<_> = log.iter().filter(|c| c.0 == DELETE).collect();
        assert_eq!(deleted, vec![&(DELETE, vec![objects[0]])]);
    }
}

#[cfg(test)]
mod new_array_tests {
    use super::*;

    fn rig(answer: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(MEMORY_ALLOCATE, move |_, _| answer.into_ret());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00b79a60_stores_the_count_and_constructs_the_pointers() {
        let mut e = rig(0);
        let block = e.mem.alloc(0x20);
        e.register_double(MEMORY_ALLOCATE, move |_, _| block.into_ret());
        let array = e.call(0x00b7_9a60, &args![3u32]).u32();
        assert_eq!(array, block + 4);
        assert_eq!(e.mem.u32(block), 3);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x10]]);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![
                block + 4,
                4,
                3,
                NI_POINTER_CONSTRUCT_00B54620,
                NI_POINTER_DESTRUCT_0045CEC0
            ]]
        );
    }

    #[test]
    fn fn_00b79a60_saturates_the_size_and_answers_0_without_memory() {
        let mut e = rig(0);
        assert_eq!(e.call(0x00b7_9a60, &args![0x4000_0000u32]).u32(), 0);
        e.call(0x00b7_9a60, &args![0x3fff_ffffu32]);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![
                vec![MEMORY_MANAGER, u32::MAX],
                vec![MEMORY_MANAGER, u32::MAX]
            ]
        );
        assert!(calls(&e, VECTOR_CONSTRUCT).is_empty());
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn the_byte_accessors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.mem.set_u8(this + 0x10, 0x5a);
        assert_eq!(e.call(0x00b7_9d70, &args![this]).u8(), 0x5a);
        let r = e.call(0x00b7_9d80, &args![this, 0x1c3u32]);
        assert_eq!(r.u8(), 1);
        assert_eq!(e.mem.u8(this + 0x24), 0xc3);
    }
}
