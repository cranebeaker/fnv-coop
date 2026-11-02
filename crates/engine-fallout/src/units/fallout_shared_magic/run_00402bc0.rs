//! `fallout shared/magic/run_00402bc0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `0048e270` (a `BSStringT` at `+4`, per the Xbox PDB's
/// `TESTexture`).
const BASE_CONSTRUCT: u32 = 0x0048_e270;
/// This class's vtable.
const VTABLE: u32 = 0x0101_1cd4;

// Translated from 00402c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESIcon` (the engine map gives it no name; the Xbox
/// PDB's `TESIcon` is a `TESTexture` and `TESQuest` constructs it at +0x24):
/// runs the base constructor `0048e270`, then sets its own vtable. Returns
/// `this`.
pub fn fn_00402c20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

// Translated from 00402c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00402bc0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00402c60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_2bc0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The member destructor body `0040a140` (unnamed in the engine map; `thiscall`, no stack
/// words), run on +0x30.
const MEMBER_DESTRUCT_0040A140: u32 = 0x0040_a140;
/// `TESBoundObject`'s destructor body (`0050e9f0`, `thiscall`, no stack words).
const BOUND_OBJECT_DESTRUCT: u32 = 0x0050_e9f0;
/// The `TESTexture`-layout destructor `0048e2e0` (`thiscall`, no stack words).
const TEXTURE_DESTRUCT: u32 = 0x0048_e2e0;

// Translated from 00402bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the code is shared by several
/// classes, so the class is unconfirmed): a destructor body that runs `0040a140` on the
/// member at +0x30, then `TESBoundObject`'s destructor body (`0050e9f0`) on `this`.
// Translation incomplete: 00402bc0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the member) is not translated.
pub fn fn_00402bc0(e: &mut Engine, this: Ptr) {
    e.call(MEMBER_DESTRUCT_0040A140, &args![this.byte_add(0x30)]);
    e.call(BOUND_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00402c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): a forwarder that runs the
/// destructor `0048e2e0` (of the layout the base constructor `0048e270` builds) on
/// `this`.
pub fn fn_00402c40(e: &mut Engine, this: Ptr) {
    e.call(TEXTURE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00402c20, fn_00402c20(Ptr) -> Ptr),
        entry!(0x0040_2c60, fn_00402c60(Ptr, u32) -> Ptr),
        entry!(0x0040_2bc0, fn_00402bc0(Ptr)),
        entry!(0x0040_2c40, fn_00402c40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_the_base_constructor_then_sets_the_vtable() {
        let mut e = Engine::new();
        // The base constructor sets its own vtable first; the derived one wins.
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.mem.alloc(0x10);
        let back = e.call(0x0040_2c20, &args![object]).u32();
        assert_eq!(back, object);
        assert_eq!(e.mem.u32(object), VTABLE);
    }
    #[test]
    fn at_00402c60_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_2bc0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_2c60, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_2c60)
                .collect();
            let mut expected = vec![(0x0040_2bc0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;

    #[test]
    fn fn_00402bc0_destroys_the_member_then_the_base() {
        let mut e = Engine::new();
        e.register(MEMBER_DESTRUCT_0040A140, |_, _| Ret::default());
        e.register(BOUND_OBJECT_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_2bc0, &args![0x0e00_1000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (MEMBER_DESTRUCT_0040A140, vec![0x0e00_1030]),
                (BOUND_OBJECT_DESTRUCT, vec![0x0e00_1000]),
            ]
        );
    }

    #[test]
    fn fn_00402c40_forwards_this_to_the_texture_destructor() {
        let mut e = Engine::new();
        e.register(TEXTURE_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_2c40, &args![0x0e00_2000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(TEXTURE_DESTRUCT, vec![0x0e00_2000])]
        );
    }
}
