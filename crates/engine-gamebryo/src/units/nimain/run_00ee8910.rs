//! `NiMain/run_00ee8910` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ee8990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual function (in several virtual tables, also called
/// directly) that answers `false` (`XOR AL,AL`; plain `RET`, `this` not read).
pub fn fn_00ee8990(_e: &mut Engine, _this: Ptr) -> bool {
    false
}

/// `NiSCMExtraData`'s vtable (its RTTI locator names `.?AVNiSCMExtraData@@`).
const NI_SCM_EXTRA_DATA_VTABLE: u32 = 0x0110_129c;
/// The `NiExtraData` constructor (`00a7b2e0`, unit `niextradata.cpp`, `thiscall`), the
/// name setter of unit `niobjectnet.cpp` (`00a5b950`, `thiscall`, one word; unnamed in the
/// engine map) and the array allocator (`00aa1070`, `cdecl`, the size).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const SET_NAME: u32 = 0x00a5_b950;
const ALLOCATE_ARRAY: u32 = 0x00aa_1070;

// Translated from 00ee8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: a name word and two entry counts): the
/// `NiSCMExtraData` constructor by its vtable: the `NiExtraData` base (`00a7b2e0`), the
/// vtable, `00a5b950` with the name word, then for each count (stored at +0xc and +0x14)
/// a used count cleared (+0x18, +0x20) and an array of `count * 8` bytes (`00aa1070`; null
/// for a zero count) at +0x24 and +0x2c. Answers `this`.
pub fn fn_00ee8910(e: &mut Engine, this: Ptr, name: u32, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.call(EXTRA_DATA_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, NI_SCM_EXTRA_DATA_VTABLE);
    e.call(SET_NAME, &args![base, name]);
    for (count, count_at, used_at, array_at) in [
        (first, 0xcu32, 0x18u32, 0x24u32),
        (second, 0x14, 0x20, 0x2c),
    ] {
        e.mem.set_u32(base + count_at, count);
        e.mem.set_u32(base + used_at, 0);
        let array = if count == 0 {
            0
        } else {
            e.call(ALLOCATE_ARRAY, &args![count.wrapping_mul(8)]).u32()
        };
        e.mem.set_u32(base + array_at, array);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_8990, fn_00ee8990(Ptr) -> bool),
        entry!(0x00ee_8910, fn_00ee8910(Ptr, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_false() {
        let mut e = Engine::new();
        assert!(!e.call(0x00ee_8990, &args![0x1000u32]).bool());
        assert!(!e.call(0x00ee_8990, &args![0u32]).bool());
    }
}

#[cfg(test)]
mod construct_tests {
    use super::*;

    #[test]
    fn constructs_with_two_arrays() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_NAME, |_, _| Ret::default());
        e.register(ALLOCATE_ARRAY, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x30);
        e.mem.write(this, &[0xaa; 0x30]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ee_8910, &args![this, 0x55u32, 3u32, 0u32]).u32(),
            this
        );
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(SET_NAME, vec![this, 0x55])));
        assert!(log.contains(&(ALLOCATE_ARRAY, vec![0x18])));
        assert_eq!(log.iter().filter(|c| c.0 == ALLOCATE_ARRAY).count(), 1);
        assert_eq!(e.mem.u32(this), NI_SCM_EXTRA_DATA_VTABLE);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x14)), (3, 0));
        assert_eq!((e.mem.u32(this + 0x18), e.mem.u32(this + 0x20)), (0, 0));
        assert_ne!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0xaaaa_aaaa);
    }
}
