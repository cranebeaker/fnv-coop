//! `fallout shared/region system/run_004f4280` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESRegionData::TESRegionData` (`004f1400`).
const TES_REGION_DATA_CTOR: u32 = 0x004f_1400;
/// The vtable of `TESRegionDataSound` (RTTI `.?AVTESRegionDataSound@@`, read at the word before it).
const TES_REGION_DATA_SOUND_VTABLE: u32 = 0x0102_3da4;
/// `004f43c0` (unnamed in the engine map): the constructor run on +8.
const MEMBER_CTOR_004F43C0: u32 = 0x004f_43c0;
/// `00470470` (unnamed in the engine map): run on +0x18.
const MEMBER_INIT_00470470: u32 = 0x0047_0470;

// Translated from 004f42a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f42a0 - its FS:[0] exception frame (the unwind states that destroy the base and the member at +8 when a later step throws) is not translated
/// `TESRegionDataSound` constructor (unnamed in the engine map; the
/// vtable's RTTI names the class): runs the `TESRegionData` constructor,
/// stores the `TESRegionDataSound` vtable, constructs the member at +8
/// (`004f43c0`), then clears the words at +8 and +0x14 and runs `00470470`
/// on +0x18. Returns `this`.
pub fn fn_004f42a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TES_REGION_DATA_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_SOUND_VTABLE);
    e.call(MEMBER_CTOR_004F43C0, &args![this.addr() + 8]);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0x14, 0);
    e.call(MEMBER_INIT_00470470, &args![this.addr() + 0x18]);
    this
}

/// `004048e0` (`thiscall`, no stack word; engine-map unit `effectitem.cpp`).
const COUNT_004048E0: u32 = 0x0040_48e0;

// Translated from 004f4280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): `004048e0` on `this` plus 1
/// (wrapping, as `ADD EAX,1`).
pub fn fn_004f4280(e: &mut Engine, this: Ptr) -> u32 {
    e.call(COUNT_004048E0, &args![this]).u32().wrapping_add(1)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_42a0, fn_004f42a0(Ptr) -> Ptr),
        entry!(0x004f_4280, fn_004f4280(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f42a0_builds_the_members_in_order() {
        let mut e = Engine::new();
        e.register(TES_REGION_DATA_CTOR, |_, _| Ret::default());
        e.register(MEMBER_CTOR_004F43C0, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 4, 0x2222);
            Ret::default()
        });
        e.register(MEMBER_INIT_00470470, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x3333);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_42a0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_SOUND_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 12), 0x2222);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x004f_42a0).collect();
        assert_eq!(
            calls,
            vec![
                (TES_REGION_DATA_CTOR, vec![this]),
                (MEMBER_CTOR_004F43C0, vec![this + 8]),
                (MEMBER_INIT_00470470, vec![this + 0x18]),
            ]
        );
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn fn_004f4280_adds_one() {
        let mut e = Engine::new();
        e.register(COUNT_004048E0, |_, a| (a[0] + 4).into_ret());
        assert_eq!(e.call(0x004f_4280, &args![0x10u32]).u32(), 0x15);
        e.register(COUNT_004048E0, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(e.call(0x004f_4280, &args![0x10u32]).u32(), 0);
    }
}
