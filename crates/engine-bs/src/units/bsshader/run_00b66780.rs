//! `BSShader/run_00b66780` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00b6f460` (`thiscall`, one word: the stream; a jump to `00ba8a70`; the
/// engine map's `NiAlphaProperty::LinkObject` name is the decompiler's): the
/// base link step.
const BASE_LINK_OBJECT: u32 = 0x00b6_f460;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall`, no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::ms_uiObjects` (`011f4400`): the live object count.
const OBJECT_COUNT: u32 = 0x011f_4400;
/// The `NiRefObject` vtable and the one `00b66810` stores over it.
const REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const VTABLE_010AE0C0: u32 = 0x010a_e0c0;

// Translated from 00b66780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the stream; a
/// `LinkObject`): the base step (`00b6f460`), then the next linked object goes
/// into the `NiPointer` at `+0xA4` (the old one released, `DeleteThis` at a
/// zero count; the new one's count incremented).
pub fn fn_00b66780(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let slot = this.addr() + 0xa4;
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
        e.vcall(old, 4, &[]);
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00b66810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word whose low byte is used): a
/// constructor of an `NiRefObject`-derived object: the `NiRefObject` part
/// (vtable `0101dce4`, count 0, `ms_uiObjects` incremented), the byte at `+8`,
/// the vtable `010ae0c0`, then two zero words (`+0xC`, `+0x10`). Answers
/// `this`.
pub fn fn_00b66810(e: &mut Engine, this: Ptr, flag: u8) -> Ptr {
    let a = this.addr();
    e.mem.set_u32(a, REF_OBJECT_VTABLE);
    e.mem.set_u32(a + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![OBJECT_COUNT]);
    e.mem.set_u8(a + 8, flag);
    e.mem.set_u32(a, VTABLE_010AE0C0);
    e.mem.set_u32(a + 0xc, 0);
    e.mem.set_u32(a + 0x10, 0);
    this
}

/// The base `RegisterStreamables` (`00bc3770`, a jump chain; `thiscall`, one word: the
/// stream; `bool` in `AL`).
const BASE_REGISTER_STREAMABLES: u32 = 0x00bc_3770;

// Translated from 00b667e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x54 of `BSShaderPPLightingProperty`'s vtable `010ae0d0`
/// and two others (`RegisterStreamables` in the Xbox PDB's slot list; `thiscall`, one word:
/// the stream). False when the base (`00bc3770`) answers false; otherwise the object at
/// `+0xa4`, when there is one, registers itself too (its slot +0x54 with the stream, the
/// result not used), and the answer is true.
pub fn fn_00b667e0(e: &mut Engine, this: Ptr, stream: u32) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let object = e.mem.u32(this.addr().wrapping_add(0xa4));
    if object != 0 {
        e.vcall(object, 0x54, &args![stream]);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_6780, fn_00b66780(Ptr, Ptr)),
        entry!(0x00b6_6810, fn_00b66810(Ptr, u8) -> Ptr),
        entry!(0x00b6_67e0, fn_00b667e0(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn with_refcounts(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
    }

    #[test]
    fn fn_00b66780_links_the_object_at_0xa4() {
        const DELETE_THIS: u32 = 0x0100_0004;
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.register(DELETE_THIS, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, vtable);
        e.mem.set_u32(old + 4, 1);
        let linked = e.mem.alloc(8);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| word(linked));
        let this = e.mem.alloc(0xa8);
        e.mem.set_u32(this + 0xa4, old);
        e.call_log = Some(vec![]);
        e.call(0x00b6_6780, &args![this, 0x6000u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_LINK_OBJECT, vec![this, 0x6000])));
        assert!(log.contains(&(DELETE_THIS, vec![old])));
        assert_eq!(e.mem.u32(this + 0xa4), linked);
        assert_eq!(e.mem.u32(linked + 4), 1);
        // The same object again: nothing changes.
        e.call(0x00b6_6780, &args![this, 0x6000u32]);
        assert_eq!(e.mem.u32(linked + 4), 1);
    }

    #[test]
    fn fn_00b66810_constructs_the_object() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.map(OBJECT_COUNT, 4);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 0x99);
        assert_eq!(e.call(0x00b6_6810, &args![this, 0x0201u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010AE0C0);
        assert_eq!(e.mem.u8(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 1);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00b667e0_registers_the_object_at_0xa4() {
        let mut e = Engine::new();
        e.register(BASE_REGISTER_STREAMABLES, |_, a| (a[1] != 0).into_ret());
        e.put_vtable(0x0f00_1000, &[0; 0x16]);
        e.mem.set_u32(0x0f00_1054, 0x0f00_0054);
        e.register(0x0f00_0054, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0f00_1000);
        let this = e.mem.alloc(0xa8);
        e.mem.set_u32(this + 0xa4, object);
        e.call_log = Some(vec![]);
        // The base refuses: nothing else runs.
        assert!(!e.call(0x00b6_67e0, &args![this, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == 0x0f00_0054));
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b6_67e0, &args![this, 0x700u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0f00_0054, vec![object, 0x700])));
        // Without an object the answer is still true.
        e.mem.set_u32(this + 0xa4, 0);
        assert!(e.call(0x00b6_67e0, &args![this, 0x700u32]).bool());
    }
}
