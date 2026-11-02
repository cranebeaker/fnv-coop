//! `SpeedTree/run_00b0dae0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT's invalid-parameter report (`00ec7c56`, no arguments).
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// The fill of uninitialised elements (`00b10cf0`, cdecl: the place, the
/// count, the value's address) and the insert (`00b0e3e0`, thiscall,
/// `RET 0x10`: the result iterator's address, the position iterator (the
/// container word and the element pointer), the value's address).
const FILL_00B10CF0: u32 = 0x00b1_0cf0;
const INSERT_00B0E3E0: u32 = 0x00b0_e3e0;
/// The vector's container word (+0) and first, last and end pointers
/// (+0xc, +0x10, +0x14); its elements are one byte each.
const CONTAINER: u32 = 0x00;
const FIRST: u32 = 0x0c;
const LAST: u32 = 0x10;
const END: u32 = 0x14;

// Translated from 00b0dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A byte vector's `push_back` (thiscall, `RET 4`; unnamed in the engine
/// map): with room left (the size below the capacity, 0 for no storage)
/// the value is filled in at the last pointer (`00b10cf0(last, 1, value)`)
/// and the last pointer moves one byte on; otherwise the end iterator
/// (checked, the invalid-parameter report when out of range) and the value
/// go to the insert (`00b0e3e0`). The code's allocator bytes and a
/// `do { } while (0)` around the check leave no trace.
pub fn fn_00b0dae0(e: &mut Engine, this: Ptr, value: Ptr) {
    let base = this.addr();
    let size = e.mem.u32(base + LAST).wrapping_sub(e.mem.u32(base + FIRST));
    let capacity = if e.mem.u32(base + FIRST) == 0 {
        0
    } else {
        e.mem.u32(base + END).wrapping_sub(e.mem.u32(base + FIRST))
    };
    if size < capacity {
        let last = e.mem.u32(base + LAST);
        e.call(FILL_00B10CF0, &args![last, 1u32, value]);
        e.mem.set_u32(base + LAST, last.wrapping_add(1));
    } else {
        let last = e.mem.u32(base + LAST);
        if base == 0 || e.mem.u32(base + FIRST) > last || last > e.mem.u32(base + LAST) {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let container = e.mem.u32(base + CONTAINER);
        e.with_stack(8, |e, result| {
            e.call(
                INSERT_00B0E3E0,
                &args![this, result, container, last, value],
            );
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b0_dae0, fn_00b0dae0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(e: &mut Engine) -> u32 {
        e.register(FILL_00B10CF0, |_, _| Ret::default());
        e.register(INSERT_00B0E3E0, |_, _| Ret::default());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + CONTAINER, 0x0777_0000);
        e.mem.set_u32(this + FIRST, 0x0600_0000);
        e.mem.set_u32(this + LAST, 0x0600_0002);
        e.mem.set_u32(this + END, 0x0600_0004);
        this
    }

    #[test]
    fn fills_in_place_while_there_is_room() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b0_dae0, &args![this, 0x0abcu32]);
        assert_eq!(e.mem.u32(this + LAST), 0x0600_0003);
        let log = e.call_log.take().unwrap();
        let fills: Vec<_> = log.iter().filter(|(a, _)| *a == FILL_00B10CF0).collect();
        assert_eq!(fills[0].1, vec![0x0600_0002, 1, 0x0abc]);
        assert!(!log.iter().any(|(a, _)| *a == INSERT_00B0E3E0));
    }

    #[test]
    fn inserts_at_the_end_when_full_or_without_storage() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        e.mem.set_u32(this + LAST, 0x0600_0004);
        e.call_log = Some(vec![]);
        e.call(0x00b0_dae0, &args![this, 0x0abcu32]);
        let log = e.call_log.take().unwrap();
        let inserts: Vec<_> = log.iter().filter(|(a, _)| *a == INSERT_00B0E3E0).collect();
        assert_eq!(inserts[0].1[0], this);
        assert_eq!(&inserts[0].1[2..], &[0x0777_0000, 0x0600_0004, 0x0abc]);
        assert!(!log.iter().any(|(a, _)| *a == INVALID_PARAMETER));
        assert_eq!(e.mem.u32(this + LAST), 0x0600_0004);
        // No storage: no room, the insert again.
        for offset in [FIRST, LAST, END] {
            e.mem.set_u32(this + offset, 0);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b0_dae0, &args![this, 0x0abcu32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, _)| *a == INSERT_00B0E3E0));
    }
}
