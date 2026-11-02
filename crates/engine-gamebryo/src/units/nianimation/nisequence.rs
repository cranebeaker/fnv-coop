//! `NiAnimation/nisequence.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSequence::ms_RTTI` (the class's `NiRTTI` record).
const NI_SEQUENCE_RTTI: u32 = 0x011f_3f48;

// Translated from 00a47a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequence::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_sequence_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SEQUENCE_RTTI)
}

// Translated from 00a47cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequence::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a478f0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x34 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_sequence_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_78f0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x34u32]);
    }
    this
}

layout! {
    /// `NiSequence` (Xbox PDB), 0x34 bytes on PC (the same): the name, the object names
    /// (`NiTPrimitiveArray<char*>` at +0xC: vtable, pointers +0x10, capacity +0x14, size
    /// +0x16, non-null count +0x18, step +0x1A), the controllers (`NiTObjectArray` of
    /// `NiPointer`s at +0x1C: vtable, pointers +0x20, capacity +0x24, size +0x26, non-null
    /// count +0x28, step +0x2A), the text keys and their link reference.
    pub struct NiSequence: 0x34 {
        /// `m_pcName` (Xbox PDB): `char*`.
        0x08 m_pcName: u32,
        /// `m_kObjectName` (Xbox PDB).
        0x0C m_kObjectName: Inline<NiTArray>,
        /// `m_kCtrl` (Xbox PDB).
        0x1C m_kCtrl: Inline<NiTArray>,
        /// `m_spTextKeys` (Xbox PDB): `NiPointer<NiTextKeyExtraData>`.
        0x2C m_spTextKeys: u32,
        /// `m_uiTextKeyReference` (Xbox PDB).
        0x30 m_uiTextKeyReference: u32,
    }
}

use crate::types::NiTArray;

/// `NiSequence`'s vtable, and the vtables its constructor gives the two arrays (the names'
/// `01021024`, the controllers' `010984e4`) and its destructor (`0102102c`, `0109844c`).
const NI_SEQUENCE_VTABLE: u32 = 0x0109_8454;
const NAMES_VTABLE: u32 = 0x0102_1024;
const CONTROLLERS_VTABLE: u32 = 0x0109_84e4;
const NAMES_BASE_VTABLE: u32 = 0x0102_102c;
const CONTROLLERS_BASE_VTABLE: u32 = 0x0109_844c;
/// The function-local `static` null `NiPointer` the controllers' `SetAt` compares with
/// (`011f3f40`), its guard word (bit 0, `011f3f44`), its `atexit` destructor (`00fdae10`)
/// and `_atexit` (`00ec658f`, `cdecl`).
const NULL_POINTER: u32 = 0x011f_3f40;
const NULL_POINTER_GUARD: u32 = 0x011f_3f44;
const NULL_POINTER_DESTRUCTOR: u32 = 0x00fd_ae10;
const ATEXIT: u32 = 0x00ec_658f;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The `char` allocation (`00aa1070`, `cdecl`, size) and free (`00aa10f0`, `cdecl`),
/// `strcpy_s` (`00ec65a6`, `cdecl`, (destination, size, source)), the memory manager
/// (`011f6238`) and its `Deallocate` (`00aa4060`, `thiscall`, the block).
const CHAR_ALLOCATE: u32 = 0x00aa_1070;
const CHAR_FREE: u32 = 0x00aa_10f0;
const STRCPY_S: u32 = 0x00ec_65a6;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `NiObject`'s constructor (`00a5d3a0`), destructor body (`00a5d3d0`) and `LoadBinary`
/// (`00a5d610`, the stream); the base `LinkObject` `00a89af0` (the stream).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const OBJECT_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream::LoadCString` (`00a64ce0`, `thiscall` on the stream, the address of the
/// pointer), the link-ID read `00a645b0` and `NiStream::GetObjectFromLinkID` (`00a63430`;
/// both `thiscall` on the stream, no arguments).
const STREAM_LOAD_CSTRING: u32 = 0x00a6_4ce0;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// The names array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`, (index, address of the
/// element)); the controllers array's `SetSize` (`00a47770`), its element getter (`00a47540`,
/// `thiscall`, (address of the result pointer, index); answers the element's address) and
/// its `delete[]` of the pointers (`00a47600`, `cdecl`).
const NAMES_SET_SIZE: u32 = 0x0096_ad30;
const NAMES_SET_AT: u32 = 0x0096_ae90;
const CONTROLLERS_SET_SIZE: u32 = 0x00a4_7770;
const CONTROLLERS_GET_AT: u32 = 0x00a4_7540;
const CONTROLLERS_DELETE: u32 = 0x00a4_7600;

/// Releases a counted object (count at +4; `DeleteThis`, slot +4, at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The inlined `strlen`.
fn string_length(e: &Engine, text: u32) -> u32 {
    let mut length = 0u32;
    while e.mem.u8(text.wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length
}

/// The 16-bit non-null count of an array, changed by `delta` (wrapping).
fn count_held(e: &mut Engine, array: u32, delta: u16) {
    let held = e.mem.u16(array + 0xc);
    e.mem.set_u16(array + 0xc, held.wrapping_add(delta));
}

/// One 4-byte read from the stream's input (`NiStream::m_pkIstr` +0x24C, `m_pfnRead` +0x8):
/// the component size word at `size_word` set to 4, the bytes read added to the position.
fn read_word(e: &mut Engine, stream: u32, destination: u32, size_word: u32) {
    let input = e.mem.u32(stream.wrapping_add(0x24c));
    let read = e.mem.u32(input.wrapping_add(8));
    e.mem.set_u32(size_word, 4);
    let done = e
        .call(read, &args![input, destination, 4u32, size_word, 1u32])
        .u32();
    let position = e.mem.u32(input.wrapping_add(4));
    e.mem
        .set_u32(input.wrapping_add(4), position.wrapping_add(done));
}

// Translated from 00a47470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The controllers array's `SetAt` (unnamed in the map; `NiTArray<NiPointer<..>>`,
/// `thiscall`, (index, address of the pointer), `RET 8`): on first use the function's
/// `static` null pointer is made (guard bit, null, its destructor given to `_atexit`). An
/// index at or past the size makes the size index + 1 and counts a non-null pointer; below
/// the size the non-null count follows the replacement. Then the element is assigned as a
/// counted pointer (the old one released, the new one counted, nothing when they are the
/// same).
pub fn fn_00a47470(e: &mut Engine, this: u32, index: u32, value: u32) {
    if e.mem.u8(NULL_POINTER_GUARD) & 1 == 0 {
        let guard = e.mem.u32(NULL_POINTER_GUARD);
        e.mem.set_u32(NULL_POINTER_GUARD, guard | 1);
        e.mem.set_u32(NULL_POINTER, 0);
        e.call(ATEXIT, &args![NULL_POINTER_DESTRUCTOR]);
    }
    let offset = index.wrapping_mul(4);
    if index >= u32::from(e.mem.u16(this + 0xa)) {
        e.mem.set_u16(this + 0xa, index.wrapping_add(1) as u16);
        if e.mem.u32(value) != e.mem.u32(NULL_POINTER) {
            count_held(e, this, 1);
        }
    } else {
        let null = e.mem.u32(NULL_POINTER);
        let array = e.mem.u32(this + 4);
        let current = e.mem.u32(array.wrapping_add(offset));
        if e.mem.u32(value) != null {
            if current == null {
                count_held(e, this, 1);
            }
        } else if current != null {
            count_held(e, this, 0xffff);
        }
    }
    let element = e.mem.u32(this + 4).wrapping_add(offset);
    let old = e.mem.u32(element);
    if old == e.mem.u32(value) {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    let new = e.mem.u32(value);
    e.mem.set_u32(element, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
    }
}

// Translated from 00a47660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequence::LoadBinary` (Xbox PDB; `thiscall`, the stream): `NiObject::LoadBinary`, the
/// name (`NiStream::LoadCString`, the pointer cleared first), the text-key reference (4
/// bytes), the text keys' link ID (`00a645b0`), the object count (4 bytes, into a local),
/// the names array sized to it (`0096ad30`), then for each object its name (`LoadCString`
/// into the stream argument's own stack slot, cleared first) stored by the inlined `SetAt`
/// and its controller's link ID read. The component size word of the reads is the stream
/// argument's slot.
pub fn ni_sequence_load_binary(e: &mut Engine, this: Ptr<NiSequence>, stream: u32) {
    let t = this.addr();
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.set(this, NiSequence::m_pcName, 0);
    e.call(STREAM_LOAD_CSTRING, &args![stream, t + 8]);
    e.with_stack(8, |e, slots| {
        let local = slots.addr();
        let argument = local + 4;
        read_word(e, stream, t + 0x30, argument);
        e.call(STREAM_READ_LINK_ID, &args![stream]);
        read_word(e, stream, local, argument);
        let names = t + 0xc;
        let count = e.mem.u32(local);
        e.call(NAMES_SET_SIZE, &args![names, count]);
        let mut index = 0u32;
        while index < e.mem.u32(local) {
            e.mem.set_u32(argument, 0);
            e.call(STREAM_LOAD_CSTRING, &args![stream, argument]);
            let name = e.mem.u32(argument);
            let offset = index.wrapping_mul(4);
            if index >= u32::from(e.mem.u16(names + 0xa)) {
                e.mem.set_u16(names + 0xa, index.wrapping_add(1) as u16);
                if name != 0 {
                    count_held(e, names, 1);
                }
            } else {
                let current = e.mem.u32(e.mem.u32(names + 4).wrapping_add(offset));
                if name != 0 {
                    if current == 0 {
                        count_held(e, names, 1);
                    }
                } else if current != 0 {
                    count_held(e, names, 0xffff);
                }
            }
            let array = e.mem.u32(names + 4);
            e.mem.set_u32(array.wrapping_add(offset), name);
            e.call(STREAM_READ_LINK_ID, &args![stream]);
            index += 1;
        }
    });
}

// Translated from 00a478f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequence`'s destructor body (unnamed in the map; `thiscall`): the vtable, the name
/// freed; for each controller slot (up to the controllers' size) the element fetched
/// (`00a47540`, into a local pointer that starts holding `this`) is released and cleared and
/// the local released, and the matching object name taken out of its array (cleared, the
/// non-null count and, for the last one, the size lowered) and freed; then the text keys
/// released, the controllers array's base (vtable `0109844c`, `delete[]` `00a47600`) and the
/// names array's (vtable `0102102c`, pointers freed), and the `NiObject` destructor body (a
/// tail jump; its `EAX` is answered).
pub fn fn_00a478f0(e: &mut Engine, this: Ptr<NiSequence>) -> u32 {
    let t = this.addr();
    let name = e.get(this, NiSequence::m_pcName);
    e.mem.set_u32(t, NI_SEQUENCE_VTABLE);
    e.call(CHAR_FREE, &args![name]);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, t);
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(t + 0x26)) {
            let element = e
                .call(CONTROLLERS_GET_AT, &args![t + 0x1c, local, index])
                .u32();
            let object = e.mem.u32(element);
            if object != 0 {
                release(e, object);
                e.mem.set_u32(element, 0);
            }
            let held = e.mem.u32(local);
            if held != 0 {
                release(e, held);
            }
            let name = if index < u32::from(e.mem.u16(t + 0x16)) {
                let at = e.mem.u32(t + 0x10).wrapping_add(index.wrapping_mul(4));
                let name = e.mem.u32(at);
                e.mem.set_u32(at, 0);
                if name != 0 {
                    count_held(e, t + 0xc, 0xffff);
                }
                let last = e.mem.u16(t + 0x16).wrapping_sub(1);
                if index == u32::from(last) {
                    e.mem.set_u16(t + 0x16, last);
                }
                name
            } else {
                0
            };
            e.call(CHAR_FREE, &args![name]);
            index += 1;
        }
    });
    let text_keys = e.get(this, NiSequence::m_spTextKeys);
    if text_keys != 0 {
        release(e, text_keys);
    }
    let controllers = e.mem.u32(t + 0x20);
    e.mem.set_u32(t + 0x1c, CONTROLLERS_BASE_VTABLE);
    e.call(CONTROLLERS_DELETE, &args![controllers]);
    let names = e.mem.u32(t + 0x10);
    e.mem.set_u32(t + 0xc, NAMES_BASE_VTABLE);
    e.call(CHAR_FREE, &args![names]);
    e.call(OBJECT_DESTRUCT, &args![this]).u32()
}

// Translated from 00a47a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, (name, controller), `RET 8`; by its body the sequence's
/// add): at the names array's size, a copy of `name` (`00aa1070` of its length + 1,
/// `strcpy_s`) is appended to the names (grown by the step when full, `SetAt` with the
/// address of the name argument's own stack slot, which holds the copy) and `controller`
/// (counted while held in that slot) to the controllers (grown when full, `fn_00a47470`).
/// Answers the index.
pub fn fn_00a47a20(e: &mut Engine, this: Ptr<NiSequence>, name: u32, controller: u32) -> u32 {
    let t = this.addr();
    let index = u32::from(e.mem.u16(t + 0x16));
    let size = string_length(e, name).wrapping_add(1);
    let copy = e.call(CHAR_ALLOCATE, &args![size]).u32();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, copy);
        e.call(STRCPY_S, &args![copy, size, name]);
        let names = t + 0xc;
        if index >= u32::from(e.mem.u16(t + 0x14)) {
            let step = u32::from(e.mem.u16(names + 0xe));
            e.call(NAMES_SET_SIZE, &args![names, step + index]);
        }
        e.call(NAMES_SET_AT, &args![names, index, slot]);
        e.mem.set_u32(slot, controller);
        if controller != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![controller.wrapping_add(4)]);
        }
        let controllers = t + 0x1c;
        if index >= u32::from(e.mem.u16(t + 0x24)) {
            let step = u32::from(e.mem.u16(controllers + 0xe));
            e.call(CONTROLLERS_SET_SIZE, &args![controllers, step + index]);
        }
        fn_00a47470(e, controllers, index, slot);
        if controller != 0 {
            release(e, controller);
        }
    });
    index
}

// Translated from 00a47ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequence::LinkObject` (Xbox PDB; `thiscall`, the stream): the base `LinkObject`
/// (`00a89af0`), the text keys from the stream's next linked object (counted assignment),
/// the controllers array sized to the names' size (`00a47770`), and each controller from
/// the next linked object stored by `fn_00a47470` (counted while held in a local that
/// starts holding `this`).
pub fn ni_sequence_link_object(e: &mut Engine, this: Ptr<NiSequence>, stream: u32) {
    let t = this.addr();
    e.call(OBJECT_LINK_OBJECT, &args![this, stream]);
    let keys = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let old = e.get(this, NiSequence::m_spTextKeys);
    if old != keys {
        if old != 0 {
            release(e, old);
        }
        e.set(this, NiSequence::m_spTextKeys, keys);
        if keys != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![keys.wrapping_add(4)]);
        }
    }
    let count = u32::from(e.mem.u16(t + 0x16));
    e.call(CONTROLLERS_SET_SIZE, &args![t + 0x1c, count]);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, t);
        for index in 0..count {
            let controller = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(local, controller);
            if controller != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![controller.wrapping_add(4)]);
            }
            fn_00a47470(e, t + 0x1c, index, local);
            if controller != 0 {
                release(e, controller);
            }
        }
    });
}

// Translated from 00a47ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The controllers array base's scalar deleting destructor (unnamed in the map; `thiscall`,
/// the flags): the vtable `0109844c`, the pointers' `delete[]` (`00a47600`), and with bit 0
/// of `flags` the block given back to the memory manager. Answers `this`.
pub fn fn_00a47ba0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let pointers = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), CONTROLLERS_BASE_VTABLE);
    e.call(CONTROLLERS_DELETE, &args![pointers]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a47bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequence`'s constructor (unnamed in the map; `thiscall`, (name, array size, step),
/// `RET 0xC`): `NiObject`'s constructor, the vtable, both arrays empty with step 1 and
/// their vtables, no text keys; the name copied (`00aa1070` of its length + 1, `strcpy_s`)
/// or null; both arrays sized (`0096ad30`, `00a47770`) with the step's low 16 bits as
/// their step; the text keys released and cleared when present; the text-key reference 0.
/// Answers `this`.
pub fn fn_00a47bd0(
    e: &mut Engine,
    this: Ptr<NiSequence>,
    name: u32,
    size: u32,
    step: u32,
) -> Ptr<NiSequence> {
    let t = this.addr();
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(t, NI_SEQUENCE_VTABLE);
    for (array, vtable) in [(t + 0xc, NAMES_VTABLE), (t + 0x1c, CONTROLLERS_VTABLE)] {
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xc, 0);
        e.mem.set_u16(array + 0xe, 1);
        e.mem.set_u32(array, vtable);
        e.mem.set_u16(array + 0xa, 0);
        e.mem.set_u32(array + 4, 0);
    }
    e.set(this, NiSequence::m_spTextKeys, 0);
    if name != 0 {
        let length = string_length(e, name).wrapping_add(1);
        let copy = e.call(CHAR_ALLOCATE, &args![length]).u32();
        e.set(this, NiSequence::m_pcName, copy);
        e.call(STRCPY_S, &args![copy, length, name]);
    } else {
        e.set(this, NiSequence::m_pcName, 0);
    }
    e.call(NAMES_SET_SIZE, &args![t + 0xc, size]);
    e.mem.set_u16(t + 0x1a, step as u16);
    e.call(CONTROLLERS_SET_SIZE, &args![t + 0x1c, size]);
    e.mem.set_u16(t + 0x2a, step as u16);
    let keys = e.get(this, NiSequence::m_spTextKeys);
    if keys != 0 {
        release(e, keys);
        e.set(this, NiSequence::m_spTextKeys, 0);
    }
    e.set(this, NiSequence::m_uiTextKeyReference, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a47a10, ni_sequence_get_rtti() -> Ptr),
        entry!(0x00a47cc0, ni_sequence_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_7470, fn_00a47470(u32, u32, u32)),
        entry!(0x00a4_7660, ni_sequence_load_binary(Ptr<NiSequence>, u32)),
        entry!(0x00a4_78f0, fn_00a478f0(Ptr<NiSequence>) -> u32),
        entry!(0x00a4_7a20, fn_00a47a20(Ptr<NiSequence>, u32, u32) -> u32),
        entry!(0x00a4_7ae0, ni_sequence_link_object(Ptr<NiSequence>, u32)),
        entry!(0x00a4_7ba0, fn_00a47ba0(Ptr, u32) -> Ptr),
        entry!(
            0x00a4_7bd0,
            fn_00a47bd0(Ptr<NiSequence>, u32, u32, u32) -> Ptr<NiSequence>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_sequence_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a47a10, &args![]).u32(), NI_SEQUENCE_RTTI);
    }

    #[test]
    fn ni_sequence_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_78f0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a47cc0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a47cc0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x34]]);
        assert_eq!(calls_to(&e, 0x00a4_78f0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n005_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn counting(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x0099_0004, |_, _| Ret::default());
        e.register(ATEXIT, |_, _| Ret::default());
        e.map(0x011f_3000, 0x1000);
    }

    /// A counted object (vtable slot +4 at `0099_0004`) holding `count` references.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table + 4, 0x0099_0004);
        e.mem.set_u32(object, table);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// An array (`NiTArray` layout) of `capacity` pointers with the given size.
    fn array(e: &mut Engine, capacity: u32, size: u16) -> u32 {
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(4 * capacity);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, capacity as u16);
        e.mem.set_u16(array + 0xa, size);
        array
    }

    #[test]
    fn set_at_00a47470_counts_and_assigns() {
        let mut e = Engine::new();
        counting(&mut e);
        let this = array(&mut e, 4, 1);
        let base = e.mem.u32(this + 4);
        let first = counted(&mut e, 0);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, first);
        e.call_log = Some(vec![]);
        // Past the size: the size grows and the pointer is counted.
        e.call(0x00a4_7470, &[this, 2, value]);
        assert_eq!(e.mem.u16(this + 0xa), 3);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u32(base + 8), first);
        assert_eq!(e.mem.u32(first + 4), 1);
        // The static null is made once.
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![NULL_POINTER_DESTRUCTOR]]);
        assert_eq!(e.mem.u32(NULL_POINTER_GUARD) & 1, 1);
        // The same pointer again: nothing changes but the count stays.
        e.call(0x00a4_7470, &[this, 2, value]);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u32(first + 4), 1);
        assert_eq!(calls_to(&e, ATEXIT).len(), 1);
        // Below the size, a null over a held pointer: counted down, released.
        e.mem.set_u32(value, 0);
        e.call(0x00a4_7470, &[this, 2, value]);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(calls_to(&e, 0x0099_0004), vec![vec![first]]);
        // Below the size, a pointer over a null: counted up.
        let second = counted(&mut e, 0);
        e.mem.set_u32(value, second);
        e.call(0x00a4_7470, &[this, 0, value]);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u16(this + 0xa), 3);
        assert_eq!(e.mem.u32(second + 4), 1);
    }

    /// A stream whose input reads answer the words in `words`, in order.
    fn reading_stream(e: &mut Engine, words: Vec<u32>) -> u32 {
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, 0x0300_0000);
        e.mem.set_u32(stream + 0x24c, input);
        let mut words = words;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], words.remove(0));
            a[2].into_ret()
        });
        stream
    }

    #[test]
    fn load_binary_reads_name_reference_and_object_names() {
        let mut e = Engine::new();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        e.register(NAMES_SET_SIZE, |_, _| Ret::default());
        let mut names = vec![0x0500_0000u32, 0x0500_0001, 0];
        e.register_double(STREAM_LOAD_CSTRING, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], names.remove(0));
            Ret::default()
        });
        let stream = reading_stream(&mut e, vec![7, 2]);
        let this = e.new_object::<NiSequence>();
        let t = this.addr();
        let base = e.mem.alloc(0x10);
        e.mem.set_u32(t + 0x10, base);
        e.mem.set_u32(t + 8, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00a4_7660, &[t, stream]);
        assert_eq!(e.get(this, NiSequence::m_pcName), 0x0500_0000);
        assert_eq!(e.get(this, NiSequence::m_uiTextKeyReference), 7);
        assert_eq!(calls_to(&e, NAMES_SET_SIZE), vec![vec![t + 0xc, 2]]);
        assert_eq!(e.mem.u32(base), 0x0500_0001);
        assert_eq!(e.mem.u32(base + 4), 0);
        assert_eq!(e.mem.u16(t + 0x16), 2);
        assert_eq!(e.mem.u16(t + 0x18), 1);
        // The text keys' link and one per object.
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID).len(), 3);
        assert_eq!(calls_to(&e, STREAM_LOAD_CSTRING)[0], vec![stream, t + 8]);
    }

    #[test]
    fn destructor_body_releases_controllers_and_frees_names() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(CHAR_FREE, |_, _| Ret::default());
        e.register(CONTROLLERS_DELETE, |_, _| Ret::default());
        e.register(OBJECT_DESTRUCT, |_, _| 0x55u32.into_ret());
        let this = e.new_object::<NiSequence>();
        let t = this.addr();
        let controllers = e.mem.alloc(8);
        let first = counted(&mut e, 1);
        e.mem.set_u32(controllers, first);
        e.mem.set_u32(t + 0x20, controllers);
        e.mem.set_u16(t + 0x26, 2);
        // GetAt hands out the element's address and a counted copy in the local.
        e.register(CONTROLLERS_GET_AT, |e, a| {
            let element = e.mem.u32(a[0] + 4) + 4 * a[2];
            let object = e.mem.u32(element);
            e.mem.set_u32(a[1], object);
            if object != 0 {
                let count = e.mem.u32(object + 4) + 1;
                e.mem.set_u32(object + 4, count);
            }
            element.into_ret()
        });
        let names = e.mem.alloc(8);
        e.mem.set_u32(names, 0x0600_0000);
        e.mem.set_u32(t + 0x10, names);
        e.mem.set_u16(t + 0x16, 1);
        e.mem.set_u16(t + 0x18, 1);
        e.mem.set_u32(t + 8, 0x0700_0000);
        let keys = counted(&mut e, 2);
        e.set(this, NiSequence::m_spTextKeys, keys);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_78f0, &[t]).u32(), 0x55);
        assert_eq!(e.mem.u32(t), NI_SEQUENCE_VTABLE);
        assert_eq!(e.mem.u32(controllers), 0);
        assert_eq!(calls_to(&e, 0x0099_0004), vec![vec![first]]);
        assert_eq!(e.mem.u32(names), 0);
        assert_eq!(e.mem.u16(t + 0x16), 0);
        assert_eq!(e.mem.u16(t + 0x18), 0);
        assert_eq!(e.mem.u32(keys + 4), 1);
        assert_eq!(
            calls_to(&e, CHAR_FREE),
            vec![vec![0x0700_0000], vec![0x0600_0000], vec![0], vec![names]]
        );
        assert_eq!(calls_to(&e, CONTROLLERS_DELETE), vec![vec![controllers]]);
        assert_eq!(e.mem.u32(t + 0x1c), CONTROLLERS_BASE_VTABLE);
        assert_eq!(e.mem.u32(t + 0xc), NAMES_BASE_VTABLE);
    }

    /// A NUL-terminated copy of `text` in the engine's memory.
    fn c_string(e: &mut Engine, text: &str) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(at, text.as_bytes());
        e.mem.set_u8(at + text.len() as u32, 0);
        at
    }

    #[test]
    fn add_00a47a20_appends_a_name_copy_and_the_controller() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(CHAR_ALLOCATE, |_, _| 0x0800_0000u32.into_ret());
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(NAMES_SET_SIZE, |_, _| Ret::default());
        e.register(CONTROLLERS_SET_SIZE, |_, _| Ret::default());
        e.register(NAMES_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0800_0000);
            Ret::default()
        });
        let this = e.new_object::<NiSequence>();
        let t = this.addr();
        let controllers = e.mem.alloc(0x10);
        e.mem.set_u32(t + 0x20, controllers);
        e.mem.set_u16(t + 0x16, 1);
        e.mem.set_u16(t + 0x14, 1);
        e.mem.set_u16(t + 0x1a, 3);
        e.mem.set_u16(t + 0x24, 4);
        let name = c_string(&mut e, "Bip01");
        let controller = counted(&mut e, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_7a20, &[t, name, controller]).u32(), 1);
        assert_eq!(calls_to(&e, CHAR_ALLOCATE), vec![vec![6]]);
        assert_eq!(calls_to(&e, STRCPY_S), vec![vec![0x0800_0000, 6, name]]);
        // The names array was full (capacity 1): grown by its step.
        assert_eq!(calls_to(&e, NAMES_SET_SIZE), vec![vec![t + 0xc, 4]]);
        assert_eq!(calls_to(&e, NAMES_SET_AT)[0][..2], [t + 0xc, 1]);
        assert!(calls_to(&e, CONTROLLERS_SET_SIZE).is_empty());
        // The controller is held by the array (and once by its owner).
        assert_eq!(e.mem.u32(controllers + 4), controller);
        assert_eq!(e.mem.u32(controller + 4), 2);
    }

    #[test]
    fn link_object_links_text_keys_and_controllers() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(OBJECT_LINK_OBJECT, |_, _| Ret::default());
        e.register(CONTROLLERS_SET_SIZE, |_, _| Ret::default());
        let keys = counted(&mut e, 0);
        let first = counted(&mut e, 0);
        let mut linked = vec![keys, first, 0];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            linked.remove(0).into_ret()
        });
        let this = e.new_object::<NiSequence>();
        let t = this.addr();
        let controllers = e.mem.alloc(0x10);
        e.mem.set_u32(t + 0x20, controllers);
        e.mem.set_u16(t + 0x16, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a4_7ae0, &[t, 0x9000]);
        assert_eq!(e.get(this, NiSequence::m_spTextKeys), keys);
        assert_eq!(e.mem.u32(keys + 4), 1);
        assert_eq!(calls_to(&e, CONTROLLERS_SET_SIZE), vec![vec![t + 0x1c, 2]]);
        assert_eq!(e.mem.u32(controllers), first);
        assert_eq!(e.mem.u32(controllers + 4), 0);
        assert_eq!(e.mem.u32(first + 4), 1);
        assert_eq!(e.mem.u16(t + 0x28), 1);
    }

    #[test]
    fn array_destructor_00a47ba0_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(CONTROLLERS_DELETE, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x4000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_7ba0, &[this, 0]).u32(), this);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00a4_7ba0, &[this, 1]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
        assert_eq!(calls_to(&e, CONTROLLERS_DELETE), vec![vec![0x4000]; 2]);
        assert_eq!(e.mem.u32(this), CONTROLLERS_BASE_VTABLE);
    }

    #[test]
    fn constructor_00a47bd0_sets_up_arrays_and_copies_the_name() {
        let mut e = Engine::new();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(CHAR_ALLOCATE, |_, _| 0x0800_0000u32.into_ret());
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(NAMES_SET_SIZE, |_, _| Ret::default());
        e.register(CONTROLLERS_SET_SIZE, |_, _| Ret::default());
        let this = Ptr::<NiSequence>::new(e.mem.alloc(0x34));
        let t = this.addr();
        for offset in (0..0x34).step_by(4) {
            e.mem.set_u32(t + offset, 0xffff_ffff);
        }
        // The text-key slot is cleared before it is checked: no release happens.
        let name = c_string(&mut e, "Walk");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_7bd0, &[t, name, 5, 0x0001_0002]).u32(), t);
        assert_eq!(e.mem.u32(t), NI_SEQUENCE_VTABLE);
        assert_eq!(e.mem.u32(t + 0xc), NAMES_VTABLE);
        assert_eq!(e.mem.u32(t + 0x1c), CONTROLLERS_VTABLE);
        assert_eq!(e.get(this, NiSequence::m_pcName), 0x0800_0000);
        assert_eq!(calls_to(&e, STRCPY_S), vec![vec![0x0800_0000, 5, name]]);
        assert_eq!(calls_to(&e, NAMES_SET_SIZE), vec![vec![t + 0xc, 5]]);
        assert_eq!(calls_to(&e, CONTROLLERS_SET_SIZE), vec![vec![t + 0x1c, 5]]);
        assert_eq!(e.mem.u16(t + 0x1a), 2);
        assert_eq!(e.mem.u16(t + 0x2a), 2);
        for offset in [0x10u32, 0x20] {
            assert_eq!(e.mem.u32(t + offset), 0);
        }
        assert_eq!(e.mem.u16(t + 0x16), 0);
        assert_eq!(e.mem.u16(t + 0x26), 0);
        assert_eq!(e.get(this, NiSequence::m_spTextKeys), 0);
        assert_eq!(e.get(this, NiSequence::m_uiTextKeyReference), 0);
        // Without a name: no copy.
        e.call_log = Some(vec![]);
        e.call(0x00a4_7bd0, &[t, 0, 5, 1]);
        assert_eq!(e.get(this, NiSequence::m_pcName), 0);
        assert!(calls_to(&e, CHAR_ALLOCATE).is_empty());
    }
}
