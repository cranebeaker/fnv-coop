//! `NiAnimation/run_00a47ce0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, the size): the Gamebryo allocation the objects are made with.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `InterlockedIncrement` and `InterlockedDecrement` (KERNEL32.DLL import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00653290` (`thiscall`, an `NiRTTI`): the object when its class is the RTTI's or derives
/// from it, else 0 (by its body: `006532c0`'s answer masks `this`).
const KIND_OF: u32 = 0x0065_3290;
/// The `NiRTTI` records (static, built at start-up) of `NiSequenceStreamHelper`
/// (`011f3f38`), `NiTextKeyExtraData` (`011f3f28`), `NiStringExtraData` (`011f4a38`),
/// `NiTransformController` (`011f36ec`) and `NiSequence` (`011f3f48`): their initializers
/// name them.
const SEQUENCE_STREAM_HELPER_RTTI: u32 = 0x011f_3f38;
const TEXT_KEY_EXTRA_DATA_RTTI: u32 = 0x011f_3f28;
const STRING_EXTRA_DATA_RTTI: u32 = 0x011f_4a38;
const TRANSFORM_CONTROLLER_RTTI: u32 = 0x011f_36ec;
const SEQUENCE_RTTI: u32 = 0x011f_3f48;
/// The sequence's constructor (`00a47bd0`, unit `nisequence.cpp`; `thiscall`, (name, two
/// array sizes)) and `00a47a20` (same unit; `thiscall`, (name, controller)), which the
/// conversion adds each controller with.
const SEQUENCE_CONSTRUCT: u32 = 0x00a4_7bd0;
const SEQUENCE_ADD: u32 = 0x00a4_7a20;
/// The smart-pointer store at +0x2c (`00a47420`) and the C-string store at +8 (`00a473d0`),
/// both `thiscall`, one word.
const SET_TEXT_KEYS: u32 = 0x00a4_7420;
const SET_NAME: u32 = 0x00a4_73d0;
/// `00a5bf50` (`thiscall`, an index) and `00a5bfa0` (`thiscall`, no stack words), unit
/// `niobjectnet.cpp`: run on the stream helper after its extra data is taken.
const HELPER_DROP_AT: u32 = 0x00a5_bf50;
const HELPER_DROP_ALL: u32 = 0x00a5_bfa0;
/// The smart-pointer assignment `00a5f860` (`thiscall` on the pointer, the object) and
/// `00c663d0` (`thiscall` on the result pointer, the address of another pointer; a pointer
/// to pointer assignment by its use).
const POINTER_ASSIGN: u32 = 0x00a5_f860;
const POINTER_COPY: u32 = 0x00c6_63d0;
/// `NiStream`'s constructor (`00a66150`), `NiStream::Load_ov3` (`00a63be0`, the file name;
/// answers `AL`) and destructor (`00a65300`), all `thiscall`.
const STREAM_CONSTRUCT: u32 = 0x00a6_6150;
const STREAM_LOAD: u32 = 0x00a6_3be0;
const STREAM_DESTRUCT: u32 = 0x00a6_5300;
/// `NiObject`'s constructor (`00a5d3a0`, `thiscall`) and `NiRotData`'s vtable (`010984ec`,
/// by its RTTI type descriptor).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const ROT_DATA_VTABLE: u32 = 0x0109_84ec;

/// Whether `object`'s class is `rtti`'s or derives from it: its `GetRTTI` record (vtable +8)
/// or one of that record's bases (`NiRTTI` +4).
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, 8, &[]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record + 4);
    }
    false
}

/// A reference-counted object's count (+4) down, its deleting function (vtable +4) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// The smart pointer at `at` released and cleared (nothing when it is null).
fn release_and_clear(e: &mut Engine, at: u32) {
    let object = e.mem.u32(at);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(at, 0);
    }
}

// Translated from 00a47ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (stream, index, name)): makes a sequence from an old
/// file's `NiSequenceStreamHelper`: none (null) for files from version `0x04010003`
/// (`NiStream` +0xd8), an index past the top objects (+0x228) or a top object (+0x220) that
/// is not a stream helper. The sequence (0x34 bytes, `00a47bd0` with the name, or the
/// helper's +8 without one, and 0xc, 0xc; +0x30 cleared) takes the helper's first
/// `NiTextKeyExtraData` (extra data array +0x10, count +0x14; `00a47420`), which the helper
/// drops (`00a5bf50`). Then for each extra data entry, walking the helper's controllers
/// (+0xc, next +0x30, held counted and unlinked as it goes), `00a47a20` adds the entry's
/// +0xc when it is an `NiStringExtraData` (0 otherwise) with the controller when it is an
/// `NiTransformController` (0 otherwise). The helper then runs `00a5bfa0`. Answers the
/// sequence.
pub fn fn_00a47ce0(e: &mut Engine, stream: Ptr, index: u32, name: u32) -> Ptr {
    let s = stream.addr();
    if e.mem.u32(s + 0xd8) >= 0x0401_0003 || index >= e.mem.u32(s + 0x228) {
        return Ptr::NULL;
    }
    let top = e
        .mem
        .u32(e.mem.u32(s + 0x220).wrapping_add(index.wrapping_mul(4)));
    if top == 0 {
        return Ptr::NULL;
    }
    let helper = e
        .call(KIND_OF, &args![top, SEQUENCE_STREAM_HELPER_RTTI])
        .u32();
    if helper == 0 {
        return Ptr::NULL;
    }
    let name = if name == 0 {
        e.mem.u32(helper + 8)
    } else {
        name
    };
    let object = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    let sequence = if object != 0 {
        e.call(SEQUENCE_CONSTRUCT, &args![object, name, 0xcu32, 0xcu32])
            .u32()
    } else {
        0
    };
    e.mem.set_u32(sequence + 0x30, 0);
    let entry = |e: &Engine, i: u32| {
        let entries = e.mem.u32(helper + 0x10);
        e.mem.u32(entries.wrapping_add((i & 0xffff) * 4))
    };
    let mut i = 0u32;
    while i < u32::from(e.mem.u16(helper + 0x14)) {
        let extra = entry(e, i);
        if extra != 0 {
            let keys = e
                .call(KIND_OF, &args![extra, TEXT_KEY_EXTRA_DATA_RTTI])
                .u32();
            if keys != 0 {
                e.call(SET_TEXT_KEYS, &args![sequence, keys]);
                e.call(HELPER_DROP_AT, &args![helper, i]);
                break;
            }
        }
        i += 1;
    }
    let mut controller = e.mem.u32(helper + 0xc);
    if controller != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![controller + 4]);
    }
    let mut j = 0u32;
    while j < u32::from(e.mem.u16(helper + 0x14)) {
        let extra = entry(e, j);
        let text = if extra != 0 && is_kind_of(e, extra, STRING_EXTRA_DATA_RTTI) {
            extra
        } else {
            0
        };
        let transform = if controller != 0 && is_kind_of(e, controller, TRANSFORM_CONTROLLER_RTTI) {
            controller
        } else {
            0
        };
        let text_name = e.mem.u32(text + 0xc);
        e.call(SEQUENCE_ADD, &args![sequence, text_name, transform]);
        let next = e.mem.u32(controller + 0x30);
        if next != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![next + 4]);
        }
        release_and_clear(e, controller + 0x30);
        if controller != next {
            release(e, controller);
            controller = next;
            if next != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![next + 4]);
                release(e, next);
            }
        } else if next != 0 {
            release(e, next);
        }
        j += 1;
    }
    e.call(HELPER_DROP_ALL, &args![helper]);
    if controller != 0 {
        release(e, controller);
    }
    Ptr::new(sequence)
}

// Translated from 00a47f20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments; a creation
/// function): 0x34 bytes from `00aa13e0` built by the sequence constructor (`00a47bd0` with
/// 0, 0xc, 0xc). Answers what the constructor answers, null when the allocation fails.
pub fn fn_00a47f20(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(SEQUENCE_CONSTRUCT, &args![object, 0u32, 0xcu32, 0xcu32])
        .ptr()
}

// Translated from 00a47f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (stream, index, &result pointer, name)): the sequence
/// at `index` of a loaded stream into the smart pointer at `result`: for files before version
/// `0x04010003` the conversion `00a47ce0` (counted into a local pointer); otherwise, for an
/// index within the top objects, the top object when it is an `NiSequence` (`00a5f860` into
/// the local pointer), and for an index past them the result is released and cleared at once.
/// With a sequence, a non-null name is stored (`00a473d0`), the result takes the local
/// pointer (`00c663d0`) and the local's count goes down again; without one the result is
/// released and cleared.
pub fn fn_00a47f40(e: &mut Engine, stream: Ptr, index: u32, result: Ptr, name: u32) {
    let s = stream.addr();
    e.with_stack(4, |e, local| {
        let held_at = local.addr();
        e.mem.set_u32(held_at, 0);
        let held = if e.mem.u32(s + 0xd8) < 0x0401_0003 {
            let sequence = fn_00a47ce0(e, stream, index, name).addr();
            if sequence != 0 {
                e.mem.set_u32(held_at, sequence);
                e.call(INTERLOCKED_INCREMENT, &args![sequence + 4]);
            }
            sequence
        } else {
            if index >= e.mem.u32(s + 0x228) {
                release_and_clear(e, result.addr());
                return;
            }
            let top = e
                .mem
                .u32(e.mem.u32(s + 0x220).wrapping_add(index.wrapping_mul(4)));
            let sequence = if top == 0 {
                0
            } else {
                e.call(KIND_OF, &args![top, SEQUENCE_RTTI]).u32()
            };
            e.call(POINTER_ASSIGN, &args![held_at, sequence]);
            e.mem.u32(held_at)
        };
        if held == 0 {
            release_and_clear(e, result.addr());
            // The exe tests the (null) local pointer once more here; both ways in hold 0.
            if held != 0 {
                release(e, held);
            }
            return;
        }
        if name != 0 {
            e.call(SET_NAME, &args![held, name]);
        }
        e.call(POINTER_COPY, &args![result, held_at]);
        release(e, held);
    });
}

// Translated from 00a48070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a48070 - the /GS stack cookie (011c16bc XOR the stack pointer, checked by 00ec408c) is not modelled
/// Unnamed in the engine map (`cdecl`, (file name, index, &result pointer, name)): a local
/// `NiStream` (0x5c4 bytes; constructor `00a66150`) loads the file (`NiStream::Load_ov3`);
/// when it loads, `00a47f40` takes the sequence at `index`, otherwise the result is released
/// and cleared; then the stream is destroyed (`00a65300`). The stack cookie the exe sets and
/// checks around it is left out.
pub fn fn_00a48070(e: &mut Engine, file_name: u32, index: u32, result: Ptr, name: u32) {
    e.with_stack(0x5c4, |e, stream| {
        e.call(STREAM_CONSTRUCT, &args![stream]);
        if e.call(STREAM_LOAD, &args![stream, file_name]).bool() {
            fn_00a47f40(e, stream, index, result, name);
        } else {
            release_and_clear(e, result.addr());
        }
        e.call(STREAM_DESTRUCT, &args![stream]);
    });
}

// Translated from 00a48110 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments; by its body
/// `NiRotData`'s creation function): 0x18 bytes from `00aa13e0`, `NiObject`'s constructor,
/// +8, +0xc and +0x10 cleared and the byte +0x14, then the vtable `010984ec`. Answers the
/// object, null when the allocation fails.
pub fn fn_00a48110(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object + 8, 0);
    e.mem.set_u32(object + 0xc, 0);
    e.mem.set_u32(object + 0x10, 0);
    e.mem.set_u8(object + 0x14, 0);
    e.mem.set_u32(object, ROT_DATA_VTABLE);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4_7ce0, fn_00a47ce0(Ptr, u32, u32) -> Ptr),
        entry!(0x00a4_7f20, fn_00a47f20() -> Ptr),
        entry!(0x00a4_7f40, fn_00a47f40(Ptr, u32, Ptr, u32)),
        entry!(0x00a4_8070, fn_00a48070(u32, u32, Ptr, u32)),
        entry!(0x00a4_8110, fn_00a48110() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with counting interlocked doubles, a heap allocator, the RTTI pages and a
    /// call log.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x2000);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.i32(a[0]).wrapping_add(1);
            e.mem.set_i32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.i32(a[0]).wrapping_sub(1);
            e.mem.set_i32(a[0], count);
            count.into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// An object whose `GetRTTI` (the double at `get_rtti`) answers `rtti`; its deleting
    /// function (vtable +4) is the double `0x0349_0000`.
    fn object(e: &mut Engine, get_rtti: u32, rtti: u32) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0349_0000);
        e.mem.set_u32(vtable + 8, get_rtti);
        e.register_double(get_rtti, move |_, _| rtti.into_ret());
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A stream (file version `version`) with one top object.
    fn stream(e: &mut Engine, version: u32, top: u32) -> u32 {
        let stream = e.mem.alloc(0x230);
        e.mem.set_u32(stream + 0xd8, version);
        let tops = e.mem.alloc(4);
        e.mem.set_u32(tops, top);
        e.mem.set_u32(stream + 0x220, tops);
        e.mem.set_u32(stream + 0x228, 1);
        stream
    }

    #[test]
    fn conversion_builds_a_sequence_from_the_helper() {
        let mut e = engine();
        e.register(0x0349_0000, |_, _| Ret::default());
        e.register(KIND_OF, |e, a| {
            let object = a[0];
            let rtti = e.mem.u32(object + 0x3c);
            (if rtti == a[1] { object } else { 0 }).into_ret()
        });
        for address in [SET_TEXT_KEYS, HELPER_DROP_ALL, SEQUENCE_ADD] {
            e.register(address, |_, _| Ret::default());
        }
        // Dropping an entry: the ones after it move down, the count goes down.
        e.register(HELPER_DROP_AT, |e, a| {
            let count = u32::from(e.mem.u16(a[0] + 0x14));
            let entries = e.mem.u32(a[0] + 0x10);
            for k in a[1]..count - 1 {
                let next = e.mem.u32(entries + 4 * (k + 1));
                e.mem.set_u32(entries + 4 * k, next);
            }
            e.mem.set_u16(a[0] + 0x14, (count - 1) as u16);
            Ret::default()
        });
        e.register(SEQUENCE_CONSTRUCT, |_, a| a[0].into_ret());
        let helper_object = object(&mut e, 0x0349_0001, 0);
        e.mem
            .set_u32(helper_object + 0x3c, SEQUENCE_STREAM_HELPER_RTTI);
        e.mem.set_u32(helper_object + 8, 0x7777);
        // Extra data: a text-key entry and a string entry; one transform controller.
        let keys = object(&mut e, 0x0349_0002, 0);
        e.mem.set_u32(keys + 0x3c, TEXT_KEY_EXTRA_DATA_RTTI);
        let text = object(&mut e, 0x0349_0003, STRING_EXTRA_DATA_RTTI);
        e.mem.set_u32(text + 0xc, 0x5151);
        let entries = e.mem.alloc(8);
        e.mem.set_u32(entries, keys);
        e.mem.set_u32(entries + 4, text);
        e.mem.set_u32(helper_object + 0x10, entries);
        e.mem.set_u16(helper_object + 0x14, 2);
        e.mem.set_u32(keys + 0xc, 0x4141);
        let controller = object(&mut e, 0x0349_0004, TRANSFORM_CONTROLLER_RTTI);
        e.mem.set_i32(controller + 4, 1);
        e.mem.set_u32(helper_object + 0xc, controller);
        let s = stream(&mut e, 0x0401_0002, helper_object);
        let sequence = e.call(0x00a4_7ce0, &args![s, 0u32, 0u32]).u32();
        assert_eq!(
            calls_to(&e, SEQUENCE_CONSTRUCT),
            vec![vec![sequence, 0x7777, 0xc, 0xc]]
        );
        assert_eq!(calls_to(&e, SET_TEXT_KEYS), vec![vec![sequence, keys]]);
        assert_eq!(calls_to(&e, HELPER_DROP_AT), vec![vec![helper_object, 0]]);
        // The string entry is added with the transform controller.
        assert_eq!(
            calls_to(&e, SEQUENCE_ADD),
            vec![vec![sequence, 0x5151, controller]]
        );
        assert_eq!(calls_to(&e, HELPER_DROP_ALL), vec![vec![helper_object]]);
        // The controller's count is back where it was.
        assert_eq!(e.mem.i32(controller + 4), 1);
        // A new file, or an index past the tops: none.
        let s = stream(&mut e, 0x0401_0003, helper_object);
        assert_eq!(e.call(0x00a4_7ce0, &args![s, 0u32, 0u32]).u32(), 0);
        let s = stream(&mut e, 0x0401_0002, helper_object);
        assert_eq!(e.call(0x00a4_7ce0, &args![s, 1u32, 0u32]).u32(), 0);
    }

    #[test]
    fn sequence_lookup_fills_the_result_pointer() {
        let mut e = engine();
        e.register(0x0349_0000, |_, _| Ret::default());
        e.register(KIND_OF, |_, a| a[0].into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            if a[1] != 0 {
                let count = e.mem.i32(a[1] + 4) + 1;
                e.mem.set_i32(a[1] + 4, count);
            }
            Ret::default()
        });
        e.register(POINTER_COPY, |e, a| {
            let object = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], object);
            let count = e.mem.i32(object + 4) + 1;
            e.mem.set_i32(object + 4, count);
            Ret::default()
        });
        e.register(SET_NAME, |_, _| Ret::default());
        let sequence = object(&mut e, 0x0349_0010, SEQUENCE_RTTI);
        let result = e.mem.alloc(4);
        let s = stream(&mut e, 0x0401_0003, sequence);
        e.call(0x00a4_7f40, &args![s, 0u32, result, 0x9999u32]);
        assert_eq!(e.mem.u32(result), sequence);
        assert_eq!(calls_to(&e, SET_NAME), vec![vec![sequence, 0x9999]]);
        // Held by the result only.
        assert_eq!(e.mem.i32(sequence + 4), 1);
        // Past the tops: the result is released and cleared.
        let old = object(&mut e, 0x0349_0011, 0);
        e.mem.set_i32(old + 4, 1);
        e.mem.set_u32(result, old);
        e.call(0x00a4_7f40, &args![s, 1u32, result, 0u32]);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(calls_to(&e, 0x0349_0000), vec![vec![old]]);
        // A null top object: the result is released too.
        let s = stream(&mut e, 0x0401_0003, 0);
        e.mem.set_u32(result, 0);
        e.call(0x00a4_7f40, &args![s, 0u32, result, 0u32]);
        assert_eq!(e.mem.u32(result), 0);
    }

    #[test]
    fn file_load_runs_the_lookup_only_when_loaded() {
        let mut e = engine();
        e.register(STREAM_CONSTRUCT, |_, _| Ret::default());
        e.register(STREAM_DESTRUCT, |_, _| Ret::default());
        e.register(STREAM_LOAD, |_, _| 0u32.into_ret());
        e.register(0x0349_0000, |_, _| Ret::default());
        let old = object(&mut e, 0x0349_0020, 0);
        e.mem.set_i32(old + 4, 1);
        let result = e.mem.alloc(4);
        e.mem.set_u32(result, old);
        e.call(0x00a4_8070, &args![0x4000u32, 0u32, result, 0u32]);
        assert_eq!(e.mem.u32(result), 0);
        let stream = calls_to(&e, STREAM_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, STREAM_LOAD), vec![vec![stream, 0x4000]]);
        assert_eq!(calls_to(&e, STREAM_DESTRUCT), vec![vec![stream]]);
        // Loaded (a version past the old files, no tops): the lookup clears the result.
        e.register(STREAM_LOAD, |e, a| {
            e.mem.set_u32(a[0] + 0xd8, 0x0401_0003);
            1u32.into_ret()
        });
        e.call(0x00a4_8070, &args![0x4000u32, 0u32, result, 0u32]);
        assert_eq!(calls_to(&e, STREAM_DESTRUCT).len(), 2);
    }

    #[test]
    fn creation_functions() {
        let mut e = engine();
        e.register(SEQUENCE_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, a| a[0].into_ret());
        let sequence = e.call(0x00a4_7f20, &[]).u32();
        let object = calls_to(&e, SEQUENCE_CONSTRUCT)[0][0];
        assert_eq!(sequence, object + 1);
        assert_eq!(
            calls_to(&e, SEQUENCE_CONSTRUCT),
            vec![vec![object, 0, 0xc, 0xc]]
        );
        let data = e.call(0x00a4_8110, &[]).u32();
        assert_eq!(e.mem.u32(data), ROT_DATA_VTABLE);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_7f20, &[]).u32(), 0);
        assert_eq!(e.call(0x00a4_8110, &[]).u32(), 0);
    }
}
