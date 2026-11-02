//! `(unplaced)/run_00844fa0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UOffsetData@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0107ED50: u32 = 0x0107_ed50;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 00844fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UOffsetData@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00844fa0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0107ED50);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 00844fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable slot, `RET 4`): allocates `count` 8-byte items through
/// `00401000(count * 8)` and returns the block. `this` is not read.
pub fn fn_00844fc0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(0x0040_1000, &args![count.wrapping_shl(3)]).u32()
}

/// The vtable of the derived instance with RTTI name
/// `.?AV?$BSScrapArray@UOffsetData@@$0EAA@@@` (read the same way).
const SCRAP_ARRAY_VTABLE_0107ED3C: u32 = 0x0107_ed3c;

// Translated from 00844ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00844ff0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f129c8, state variable around the release) is not translated
/// Destructor of the `BSScrapArray` instance with RTTI name
/// `.?AV?$BSScrapArray@UOffsetData@@$0EAA@@@` (`thiscall`, no stack word): stores its
/// vtable, releases the buffer (`008454f0(this, 1)`), then runs the `BSSimpleArray` base
/// destructor ([`fn_00844fa0`]).
pub fn fn_00844ff0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE_0107ED3C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_00844fa0(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0084_4ff0, fn_00844ff0(Ptr)),
        entry!(0x0084_4fa0, fn_00844fa0(Ptr)),
        entry!(0x0084_4fc0, fn_00844fc0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_00844fa0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0084_4fa0, ARRAY_VTABLE_0107ED50, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    #[test]
    fn fn_00844fc0_allocates_eight_bytes_per_item() {
        let mut e = Engine::new();
        e.register(0x0040_1000, |_, a| (a[0] + 0x5000).into_ret());
        assert_eq!(
            e.call(0x0084_4fc0, &args![0x10u32, 6u32]).u32(),
            0x5000 + 48
        );
        // The count is shifted as a word.
        assert_eq!(
            e.call(0x0084_4fc0, &args![0x10u32, 0x2000_0001u32]).u32(),
            0x5008
        );
    }
}

#[cfg(test)]
mod tests_n077 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00844ff0_releases_then_runs_the_base_destructor() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(ARRAY_RELEASE_008454F0, move |e, a| {
            record.borrow_mut().push((a[0], a[1], e.mem.u32(a[0])));
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call(0x0084_4ff0, &args![this]);
        assert_eq!(
            *seen.borrow(),
            [
                (this, 1, SCRAP_ARRAY_VTABLE_0107ED3C),
                (this, 1, ARRAY_VTABLE_0107ED50),
            ]
        );
    }
}
