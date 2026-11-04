//! `fallout shared/pathfinding/run_006c7850` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSTaskletData` vtable (RTTI).
const VTABLE_TASKLET_DATA: u32 = 0x0106_c814;
/// `BSWin32TaskletData` vtable (RTTI).
const VTABLE_WIN32_TASKLET_DATA: u32 = 0x0106_c800;
/// The constructor of the member at +0x10 (`0044dee0`, thiscall, no stack words).
const MEMBER_CONSTRUCTOR: u32 = 0x0044_dee0;

// Translated from 006c78d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTaskletData` constructor (class by the vtable's RTTI): stores the vtable and clears the
/// byte at +4. Returns `this`.
pub fn bs_tasklet_data_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_TASKLET_DATA);
    e.mem.set_u8(this.addr() + 4, 0);
    this
}

// Translated from 006c7850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c7850 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f03d68, state variable) is not translated
/// `BSWin32TaskletData` constructor (class by the vtable's RTTI): runs the `BSTaskletData`
/// constructor (`006c78d0`), stores its own vtable, clears the word at +8, sets the byte at
/// +0xc to 1 and constructs the member at +0x10 (`0044dee0`). Returns `this`.
pub fn bs_win32_tasklet_data_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    bs_tasklet_data_constructor(e, this);
    e.mem.set_u32(base, VTABLE_WIN32_TASKLET_DATA);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u8(base + 0xc, 1);
    e.call(MEMBER_CONSTRUCTOR, &args![base + 0x10]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c_78d0, bs_tasklet_data_constructor(Ptr) -> Ptr),
        entry!(0x006c_7850, bs_win32_tasklet_data_constructor(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    #[test]
    fn tasklet_data_constructor_sets_the_vtable_and_clears_the_flag() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0xffff_ffff);
        assert_eq!(e.call(0x006c_78d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_TASKLET_DATA);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ff00);
    }

    #[test]
    fn win32_tasklet_data_constructor_fills_its_fields_and_constructs_the_member() {
        let mut e = Engine::new();
        e.register(MEMBER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x5151);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x20);
        for i in 0..8 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c_7850, &args![this]).u32(), this);
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == MEMBER_CONSTRUCTOR)
            .collect();
        assert_eq!(calls, vec![(MEMBER_CONSTRUCTOR, vec![this + 0x10])]);
        assert_eq!(e.mem.u32(this), VTABLE_WIN32_TASKLET_DATA);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0x10), 0x5151);
    }
}
