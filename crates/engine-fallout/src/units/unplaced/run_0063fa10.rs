//! `(unplaced)/run_0063fa10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `QueuedFile` base constructor (`00c3ce60`, `this` and the task
/// argument).
const QUEUED_FILE_CONSTRUCT: u32 = 0x00c3_ce60;
/// Sets the queued file's name (`00c3cee0`, `this` and the name).
const QUEUED_FILE_SET_NAME: u32 = 0x00c3_cee0;
/// Looks the named file up in an archive (`00c3cf60`, `this` and the
/// archive argument).
const QUEUED_FILE_FIND_ENTRY: u32 = 0x00c3_cf60;
/// The `SkyTask` vtable (its RTTI locator names `.?AVSkyTask@@`).
const SKY_TASK_VTABLE: u32 = 0x0104_f118;
/// The task argument the base constructor gets.
const SKY_TASK_BASE_ARGUMENT: u32 = 3;
/// The archive argument of the file look-up.
const SKY_TASK_ARCHIVE_ARGUMENT: u32 = 1;
/// The `SkyTask` field the first argument goes to (+0x30).
const SKY_TASK_OWNER: u32 = 0x30;

// Translated from 0063fa10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0063fa10 - C++ exception unwinding (FS:[0] frame) not translated
/// A `SkyTask` constructor: the `QueuedFile` base with argument 3, the
/// `SkyTask` vtable, `owner` at +0x30, then the file name and the archive
/// look-up with argument 1. Returns `this`.
pub fn fn_0063fa10(e: &mut Engine, this: Ptr, owner: u32, name: u32) -> Ptr {
    e.call(QUEUED_FILE_CONSTRUCT, &args![this, SKY_TASK_BASE_ARGUMENT]);
    e.mem.set_u32(this.addr(), SKY_TASK_VTABLE);
    e.mem
        .set_u32(this.addr().wrapping_add(SKY_TASK_OWNER), owner);
    e.call(QUEUED_FILE_SET_NAME, &args![this, name]);
    e.call(
        QUEUED_FILE_FIND_ENTRY,
        &args![this, SKY_TASK_ARCHIVE_ARGUMENT],
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0063fa10, fn_0063fa10(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sky_task_constructor_builds_the_base_and_names_the_file() {
        let mut e = Engine::new();
        e.register(QUEUED_FILE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(QUEUED_FILE_SET_NAME, |e, a| {
            assert_eq!(e.mem.u32(a[0]), SKY_TASK_VTABLE);
            Ret::default()
        });
        e.register(QUEUED_FILE_FIND_ENTRY, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0063_fa10, &args![this, 0x55u32, 0x66u32]).u32(),
            this
        );
        let log = e.call_log.take().unwrap();
        let callees: Vec<_> = log.iter().skip(1).map(|(a, w)| (*a, w.clone())).collect();
        assert_eq!(
            callees,
            vec![
                (QUEUED_FILE_CONSTRUCT, vec![this, 3]),
                (QUEUED_FILE_SET_NAME, vec![this, 0x66]),
                (QUEUED_FILE_FIND_ENTRY, vec![this, 1]),
            ]
        );
        assert_eq!(e.mem.u32(this + SKY_TASK_OWNER), 0x55);
    }
}
