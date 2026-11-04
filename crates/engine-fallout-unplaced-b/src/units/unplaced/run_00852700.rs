//! `(unplaced)/run_00852700` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The save/load object the callbacks here work on (`011de134`; fields read at their
/// offsets).
const SAVE_LOAD_GLOBAL: u32 = 0x011d_e134;
/// `00850330` (`thiscall`, one word): the scalar deleting destructor of the object at +0x20.
const DELETE_00850330: u32 = 0x0085_0330;
/// `00af2e50` (no arguments; `BSSystemUtilities`): the object `008527d0` asks.
const OBJECT_00AF2E50: u32 = 0x00af_2e50;
/// The text `008527d0` passes (`0107fabc`).
const TEXT_0107FABC: u32 = 0x0107_fabc;

// Translated from 00852700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): whether bit 0x20 or bit 0x80 of the
/// word at +4 is set.
pub fn fn_00852700(e: &mut Engine, this: Ptr) -> bool {
    let flags = e.mem.u32(this.addr() + 4);
    flags & 0x20 != 0 || flags & 0x80 != 0
}

// Translated from 00852740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words; a copy-done callback): marks the save/load
/// object (`011de134`, byte +0x24 = 1 when it exists), deletes its object at +0x20
/// (`00850330` with 1) and clears it, then passes both words to its callback at +0x28
/// (`cdecl`) when set. The word at +0x20 is read whether or not the object exists, as the
/// exe does.
pub fn fn_00852740(e: &mut Engine, first: u32, second: u32) {
    if e.mem.u32(SAVE_LOAD_GLOBAL) != 0 {
        let object = e.mem.u32(SAVE_LOAD_GLOBAL);
        e.mem.set_u8(object + 0x24, 1);
    }
    let object = e.mem.u32(SAVE_LOAD_GLOBAL);
    if e.mem.u32(object.wrapping_add(0x20)) != 0 {
        let owned = e.mem.u32(object.wrapping_add(0x20));
        if owned != 0 {
            e.call(DELETE_00850330, &args![owned, 1u32]);
        }
        let object = e.mem.u32(SAVE_LOAD_GLOBAL);
        e.mem.set_u32(object + 0x20, 0);
    }
    if e.mem.u32(SAVE_LOAD_GLOBAL) != 0 {
        let object = e.mem.u32(SAVE_LOAD_GLOBAL);
        let callback = e.mem.u32(object + 0x28);
        if callback != 0 {
            e.call(callback, &args![first, second]);
        }
    }
}

// Translated from 008527d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments; a callback): byte +0x1C of the
/// save/load object (`011de134`) = whether `00af2e50`'s object answers above 0 (unsigned)
/// from its vtable slot +0xC with `0107fabc`, 0, 0.
pub fn fn_008527d0(e: &mut Engine) {
    let object = e.call(OBJECT_00AF2E50, &[]).u32();
    let answer = e
        .vcall(object, 0xc, &args![TEXT_0107FABC, 0u32, 0u32])
        .u32();
    let save_load = e.mem.u32(SAVE_LOAD_GLOBAL);
    e.mem.set_u8(save_load + 0x1c, u8::from(answer > 0));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0085_2700, fn_00852700(Ptr) -> bool),
        entry!(0x0085_2740, fn_00852740(u32, u32)),
        entry!(0x0085_27d0, fn_008527d0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00852700_tests_either_bit() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        for (flags, expected) in [(0u32, 0u32), (0x20, 1), (0x80, 1), (0x5f, 0)] {
            e.mem.set_u32(this + 4, flags);
            assert_eq!(e.call(0x0085_2700, &args![this]).u32() & 0xff, expected);
        }
    }

    #[test]
    fn fn_00852740_marks_deletes_and_calls_back() {
        let mut e = Engine::new();
        e.map(SAVE_LOAD_GLOBAL & !0xfff, 0x1000);
        e.register(DELETE_00850330, |_, _| Ret::default());
        e.register(0x2fff_0000, |_, _| Ret::default());
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(SAVE_LOAD_GLOBAL, object);
        e.mem.set_u32(object + 0x20, 0x6000);
        e.mem.set_u32(object + 0x28, 0x2fff_0000);
        e.call_log = Some(vec![]);
        e.call(0x0085_2740, &args![1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (DELETE_00850330, vec![0x6000, 1]),
                (0x2fff_0000, vec![1, 2])
            ]
        );
        assert_eq!(e.mem.u8(object + 0x24), 1);
        assert_eq!(e.mem.u32(object + 0x20), 0);
        // Without a callback or an owned object nothing is called.
        e.mem.set_u32(object + 0x28, 0);
        e.call_log = Some(vec![]);
        e.call(0x0085_2740, &args![1u32, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_008527d0_stores_whether_the_answer_is_positive() {
        let mut e = Engine::new();
        e.map(SAVE_LOAD_GLOBAL & !0xfff, 0x1000);
        let save_load = e.mem.alloc(0x30);
        e.mem.set_u32(SAVE_LOAD_GLOBAL, save_load);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 0xc, vtable + 0xc);
        e.register_double(vtable + 0xc, |e, a| {
            assert_eq!(a[1..], [TEXT_0107FABC, 0, 0]);
            e.mem.u32(a[0] + 4).into_ret()
        });
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.register_double(OBJECT_00AF2E50, move |_, _| object.into_ret());
        e.mem.set_u32(object + 4, 3);
        e.call(0x0085_27d0, &[]);
        assert_eq!(e.mem.u8(save_load + 0x1c), 1);
        e.mem.set_u32(object + 4, 0);
        e.call(0x0085_27d0, &[]);
        assert_eq!(e.mem.u8(save_load + 0x1c), 0);
    }
}
