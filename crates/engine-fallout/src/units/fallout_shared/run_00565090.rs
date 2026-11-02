//! `fallout shared/run_00565090` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00527080` (thiscall, type byte): the reference's extra data of that
/// type (`00410220` on its extra data list).
const GET_EXTRA: u32 = 0x0052_7080;
/// The extra data type these functions handle (0x92).
const EXTRA_TYPE: u32 = 0x92;
/// `0084e3a0` (thiscall): the word at +0xc (the extra data's flags).
const EXTRA_FLAGS: u32 = 0x0084_e3a0;
/// `0044ddc0` (thiscall): the word at +8 (a form's flags).
const FORM_FLAGS: u32 = 0x0044_ddc0;
/// `00401000` (cdecl, byte count): allocates a block.
const ALLOCATE_BLOCK: u32 = 0x0040_1000;
/// `00411e40` (thiscall): the constructor of the type-0x92 extra data
/// (vtable `01014458`, flags 0).
const EXTRA_CONSTRUCT: u32 = 0x0041_1e40;
/// `005d43c0` (thiscall): `this + 0x44` (the reference's extra data list).
const EXTRA_LIST: u32 = 0x005d_43c0;
/// `0040ff60` `BaseExtraList::AddExtra` (Xbox PDB name in the engine map;
/// thiscall, extra data).
const ADD_EXTRA: u32 = 0x0040_ff60;
/// `0041fd00` (thiscall, word): stores the flags (the engine map's name
/// `NonActorMagicCaster::SetCurrentSpell` is a folded body).
const SET_EXTRA_FLAGS: u32 = 0x0041_fd00;
/// `00410140` `BaseExtraList::RemoveExtra_ov2` (Xbox PDB name in the engine
/// map; thiscall, type byte).
const REMOVE_EXTRA: u32 = 0x0041_0140;

// Translated from 00565090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the reference has the type-0x92 extra
/// data with flag 8 set.
pub fn fn_00565090(e: &mut Engine, this: Ptr) -> bool {
    let extra = e.call(GET_EXTRA, &args![this, EXTRA_TYPE]).u32();
    extra != 0 && e.call(EXTRA_FLAGS, &args![extra]).u32() & 8 != 0
}

// Translated from 005651e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether flag 0x80000 of the form's flags
/// (`0044ddc0`) is set.
pub fn fn_005651e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_FLAGS, &args![this]).u32() & 0x0008_0000 != 0
}

// Translated from 005650d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets (`set` non-zero) or clears flag 8 of the
/// reference's type-0x92 extra data, creating it (0x14 bytes, `00411e40`,
/// added to the extra list) when `set` is exactly 1 and it is missing, and
/// removing it once its flags are 0; then vtable +0x48 with 0x80000000.
// Translation incomplete: 005650d0 - SEH exception unwinding (the allocation's cleanup on unwind) is unsupported.
pub fn fn_005650d0(e: &mut Engine, this: Ptr, set: u8) {
    let mut extra = e.call(GET_EXTRA, &args![this, EXTRA_TYPE]).u32();
    if set == 1 && extra == 0 {
        let block = e.call(ALLOCATE_BLOCK, &args![0x14u32]).u32();
        let data = if block != 0 {
            e.call(EXTRA_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        let list = e.call(EXTRA_LIST, &args![this]).u32();
        extra = e.call(ADD_EXTRA, &args![list, data]).u32();
    }
    if extra != 0 {
        let mut flags = e.call(EXTRA_FLAGS, &args![extra]).u32();
        if set != 0 {
            flags |= 8;
        } else {
            flags &= 0xffff_fff7;
        }
        e.call(SET_EXTRA_FLAGS, &args![extra, flags]);
        if flags == 0 {
            let list = e.call(EXTRA_LIST, &args![this]).u32();
            e.call(REMOVE_EXTRA, &args![list, EXTRA_TYPE]);
        }
        e.vcall(this.addr(), 0x48, &args![0x8000_0000u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0056_5090, fn_00565090(Ptr) -> bool),
        entry!(0x0056_51e0, fn_005651e0(Ptr) -> bool),
        entry!(0x0056_50d0, fn_005650d0(Ptr, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A reference whose extra data (if any) is the word at +0x50; the
    /// extra's flags are its word at +0xc.
    fn extra_doubles(e: &mut Engine) {
        e.register(GET_EXTRA, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x50),
            ..Ret::default()
        });
        e.register(EXTRA_FLAGS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        e.register(SET_EXTRA_FLAGS, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        e.register(ALLOCATE_BLOCK, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(EXTRA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(EXTRA_LIST, |_, a| Ret {
            eax: a[0] + 0x44,
            ..Ret::default()
        });
        e.register(ADD_EXTRA, |e, a| {
            e.mem.set_u32(a[0] - 0x44 + 0x50, a[1]);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(REMOVE_EXTRA, |e, a| {
            e.mem.set_u32(a[0] - 0x44 + 0x50, 0);
            Ret::default()
        });
        e.register(0x7000_0048, |_, _| Ret::default());
        e.put_vtable(0x9100_0000, &[0; 0x13]);
        e.mem.set_u32(0x9100_0048, 0x7000_0048);
    }

    fn new_reference(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this, 0x9100_0000);
        this
    }

    #[test]
    fn fn_00565090_tests_flag_8_of_the_extra() {
        let mut e = Engine::new();
        extra_doubles(&mut e);
        let this = new_reference(&mut e);
        assert!(!e.call(0x0056_5090, &args![this]).bool());
        let extra = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x50, extra);
        e.mem.set_u32(extra + 0xc, 7);
        assert!(!e.call(0x0056_5090, &args![this]).bool());
        e.mem.set_u32(extra + 0xc, 8);
        assert!(e.call(0x0056_5090, &args![this]).bool());
    }

    #[test]
    fn fn_005651e0_tests_form_flag_80000() {
        let mut e = Engine::new();
        e.register(FORM_FLAGS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0xfff7_ffff);
        assert!(!e.call(0x0056_51e0, &args![this]).bool());
        e.mem.set_u32(this + 8, 0x0008_0000);
        assert!(e.call(0x0056_51e0, &args![this]).bool());
    }

    #[test]
    fn fn_005650d0_creates_sets_clears_and_removes_the_extra() {
        let mut e = Engine::new();
        extra_doubles(&mut e);
        let this = new_reference(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0056_50d0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x50), 0, "clearing creates nothing");
        e.call(0x0056_50d0, &args![this, 2u32]);
        assert_eq!(e.mem.u32(this + 0x50), 0, "only 1 creates");
        e.call(0x0056_50d0, &args![this, 1u32]);
        let extra = e.mem.u32(this + 0x50);
        assert_ne!(extra, 0);
        assert_eq!(e.mem.u32(extra + 0xc), 8);
        e.mem.set_u32(extra + 0xc, 0x18);
        e.call(0x0056_50d0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(extra + 0xc), 0x10, "still present");
        assert_eq!(e.mem.u32(this + 0x50), extra);
        e.mem.set_u32(extra + 0xc, 8);
        e.call(0x0056_50d0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x50), 0, "removed at flags 0");
        let log = e.call_log.take().unwrap();
        let notified: Vec<_> = log.iter().filter(|c| c.0 == 0x7000_0048).collect();
        assert_eq!(notified.len(), 3);
        assert_eq!(notified[0].1, vec![this, 0x8000_0000]);
        let removed: Vec<_> = log.iter().filter(|c| c.0 == REMOVE_EXTRA).collect();
        assert_eq!(removed[0].1, vec![this + 0x44, 0x92]);
    }
}
