//! `fallout shared/run_005e0830` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two methods of an object with a name buffer at `+4` (set through
//! `004f8ac0`) and a `BSSimpleList` at `+0x10C`; the class is not
//! confirmed.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e0830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the name (`004f8ac0` with `name`), then calls the object's vtable
/// +0x18 with `flag` and answers what it answers: the call is the last
/// instruction before the return (`005e0853`), so its `EAX` is this
/// function's (`005e0200` tests `AL`: the setting file opened).
pub fn fn_005e0830(e: &mut Engine, this: Ptr, name: u32, flag: u8) -> u32 {
    e.call(0x004f_8ac0, &args![this, name]);
    e.vcall(this.addr(), 0x18, &args![u32::from(flag)]).u32()
}

// Translated from 005e0860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the item of the `BSSimpleList` at `+0x10C` whose name (the word
/// at the item's `+8`, through `0044ddc0`) compares equal to `name`
/// (`00404dc0` returns 0): stores it in `*found` and returns true. False for
/// a null `name`, or when the walk reaches an empty node (`008256d0`) or the
/// end (`00726070` gives the next node).
pub fn fn_005e0860(e: &mut Engine, this: Ptr, name: u32, found: Ptr) -> bool {
    if name == 0 {
        return false;
    }
    let mut node = this.addr().wrapping_add(0x10c);
    while node != 0 {
        if e.call(0x0082_56d0, &args![node]).u8() != 0 {
            break;
        }
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        let item_name = e.call(0x0044_ddc0, &args![item]).u32();
        if e.call(0x0040_4dc0, &args![item_name, name]).u32() == 0 {
            e.mem.set_u32(found.addr(), item);
            return true;
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_0830, fn_005e0830(Ptr, u32, u8) -> u32),
        entry!(0x005e_0860, fn_005e0860(Ptr, u32, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn sets_the_name_then_calls_the_slot() {
        let mut e = Engine::new();
        e.register(0x004f_8ac0, |_, _| Ret::default());
        e.put_vtable(0x0060_0000, &[0; 8]);
        e.mem.set_u32(0x0060_0018, 0x0061_0000);
        e.register(0x0061_0000, |_, _| 0x1234_5601u32.into_ret());
        let this = e.mem.alloc(0x120);
        e.mem.set_u32(this, 0x0060_0000);
        e.call_log = Some(vec![]);
        // The slot's answer is passed on whole (005e0853).
        assert_eq!(
            e.call(0x005e_0830, &args![this, 0x5000u32, 0x101u32]).u32(),
            0x1234_5601
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x004f_8ac0, vec![this, 0x5000]));
        assert_eq!(log[2], (0x0061_0000, vec![this, 1]));
    }

    /// A list at `+0x10C` of nodes (item, next) whose items hold their name
    /// at `+8`; the doubles walk it as the game's helpers do.
    fn list_engine() -> (Engine, u32, [u32; 2]) {
        let mut e = Engine::new();
        e.register(0x0082_56d0, |e, a| {
            u8::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0040_4dc0, |_, a| u32::from(a[0] != a[1]).into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let this = e.mem.alloc(0x120);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0] + 8, 0x7000);
        e.mem.set_u32(items[1] + 8, 0x7100);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, items[1]);
        e.mem.set_u32(this + 0x10c, items[0]);
        e.mem.set_u32(this + 0x110, second);
        (e, this, items)
    }

    #[test]
    fn finds_an_item_by_name() {
        let (mut e, this, items) = list_engine();
        let found = e.mem.alloc(4);
        assert!(e.call(0x005e_0860, &args![this, 0x7100u32, found]).bool());
        assert_eq!(e.mem.u32(found), items[1]);
        e.mem.set_u32(found, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_0860, &args![this, 0x7200u32, found]).bool());
        assert_eq!(e.mem.u32(found), 0);
        assert_eq!(calls_to(&e, 0x0040_4dc0).len(), 2);
        assert!(!e.call(0x005e_0860, &args![this, 0u32, found]).bool());
        // An empty list stops at once.
        let empty = e.mem.alloc(0x120);
        assert!(!e.call(0x005e_0860, &args![empty, 0x7000u32, found]).bool());
    }
}
