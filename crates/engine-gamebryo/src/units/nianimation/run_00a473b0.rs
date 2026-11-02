//! `NiAnimation/run_00a473b0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a473b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar-deleting-destructor-shaped function (no name in the engine map; class
/// unconfirmed): runs the destructor body at `00a5c240`, then the sized delete
/// (`00aa1460`, `(this, size)`) of 0x18 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00a473b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_c240;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

/// `00aa13e0` (`cdecl`, the size): the Gamebryo allocation the objects are made with; the
/// string storage's allocation (`00aa1070`) and free (`00aa10f0`, both `cdecl`).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const STRING_ALLOCATE: u32 = 0x00aa_1070;
const STRING_FREE: u32 = 0x00aa_10f0;
/// `_strcpy_s` (`cdecl`, (destination, size, source)).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `00a5c200` (unit `niobjectnet.cpp`, `thiscall`, no stack words): the base constructor
/// `00a47380` runs, and `NiSequenceStreamHelper`'s vtable (`010983bc`, by its RTTI type
/// descriptor).
const OBJECT_NET_CONSTRUCT: u32 = 0x00a5_c200;
const SEQUENCE_STREAM_HELPER_VTABLE: u32 = 0x0109_83bc;
/// `InterlockedIncrement` and `InterlockedDecrement` (KERNEL32.DLL import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a47380 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments; by its body
/// `NiSequenceStreamHelper`'s creation function): 0x18 bytes from `00aa13e0`, built by
/// `00a5c200` with the vtable `010983bc`. Answers the object, null when the allocation fails.
pub fn fn_00a47380(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(OBJECT_NET_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, SEQUENCE_STREAM_HELPER_VTABLE);
    Ptr::new(object)
}

// Translated from 00a473d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, a C string, `RET 4`): frees the string at +8
/// (`00aa10f0`) and clears it; for a non-null `text`, its length plus one bytes from
/// `00aa1070` (stored at +8) get a copy (`_strcpy_s`).
pub fn fn_00a473d0(e: &mut Engine, this: Ptr, text: u32) {
    let old = e.mem.u32(this.addr() + 8);
    e.call(STRING_FREE, &args![old]);
    e.mem.set_u32(this.addr() + 8, 0);
    if text == 0 {
        return;
    }
    let mut end = text;
    while e.mem.u8(end) != 0 {
        end = end.wrapping_add(1);
    }
    let size = end.wrapping_sub(text).wrapping_add(1);
    let copy = e.call(STRING_ALLOCATE, &args![size]).u32();
    e.mem.set_u32(this.addr() + 8, copy);
    e.call(STRCPY_S, &args![copy, size, text]);
}

// Translated from 00a47420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the object, `RET 4`): the smart-pointer store at
/// +0x2c: unless it holds `object` already, the old object's count (+4) goes down (vtable +4
/// at zero), `object` is stored and, when not null, counted up.
pub fn fn_00a47420(e: &mut Engine, this: Ptr, object: u32) {
    let old = e.mem.u32(this.addr() + 0x2c);
    if old == object {
        return;
    }
    if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
        e.vcall(old, 4, &[]);
    }
    e.mem.set_u32(this.addr() + 0x2c, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a473b0, fn_00a473b0(Ptr, u32) -> Ptr),
        entry!(0x00a4_7380, fn_00a47380() -> Ptr),
        entry!(0x00a4_73d0, fn_00a473d0(Ptr, u32)),
        entry!(0x00a4_7420, fn_00a47420(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a473b0_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_c240, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a473b0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a473b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, 0x00a5_c240), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n007_tests {
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

    #[test]
    fn creation_function_allocates_and_constructs() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_NET_CONSTRUCT, |_, a| a[0].into_ret());
        let object = e.call(0x00a4_7380, &[]).u32();
        assert_eq!(e.mem.u32(object), SEQUENCE_STREAM_HELPER_VTABLE);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_7380, &[]).u32(), 0);
    }

    #[test]
    fn name_is_freed_and_copied() {
        let mut e = Engine::new();
        e.register(STRING_FREE, |_, _| Ret::default());
        e.register(STRING_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(STRCPY_S, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x1230);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"abc\0");
        e.call(0x00a4_73d0, &args![this, text]);
        let copy = e.mem.u32(this + 8);
        assert_eq!(calls_to(&e, STRING_FREE), vec![vec![0x1230]]);
        assert_eq!(calls_to(&e, STRING_ALLOCATE), vec![vec![4]]);
        assert_eq!(calls_to(&e, STRCPY_S), vec![vec![copy, 4, text]]);
        // A null text: freed and cleared only.
        e.call(0x00a4_73d0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(calls_to(&e, STRING_ALLOCATE).len(), 1);
    }

    #[test]
    fn counted_store_at_0x2c() {
        let mut e = Engine::new();
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
        e.register(0x0348_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0348_0000);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        let this = e.mem.alloc(0x30);
        e.call(0x00a4_7420, &args![this, object]);
        e.call(0x00a4_7420, &args![this, object]);
        assert_eq!((e.mem.u32(this + 0x2c), e.mem.i32(object + 4)), (object, 1));
        e.call(0x00a4_7420, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(calls_to(&e, 0x0348_0000), vec![vec![object]]);
    }
}
