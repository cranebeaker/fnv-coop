//! `fallout shared/havok/run_0061fba0` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0061fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0104d9cc` (RTTI `.?AVZoneEntry@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `0061fea0`. One word, plain `RET`, returns nothing.
pub fn fn_0061fc50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_d9cc);
    e.call(0x0061_fea0, &args![this]);
}

/// `00620720` (`run_00620230.rs`): `thiscall`, `RET 8`; the constructor of the list node the
/// entries embed (stores its two words at `+0xC` and `+8`, clears `+0` and `+4`).
const ENTRY_NODE_CONSTRUCT: u32 = 0x0062_0720;
/// `006815c0` (unnamed in the engine map): `thiscall`, plain `RET`, no stack words.
const MEMBER_SETUP: u32 = 0x0068_15c0;
/// `00538f40` (unnamed in the engine map): `thiscall`, plain `RET`, no stack words; run on each
/// member right after `006815c0`.
const MEMBER_SETUP_AFTER: u32 = 0x0053_8f40;

// Translated from 0061fba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TargetEntry` (RTTI `.?AVTargetEntry@@`, vtable `0104d9c4`): constructs
/// the embedded node at `+0x10` with the two arguments (`00620720`), stores the vtable, runs
/// `006815c0` on the members at `+0x20` and `+0x30` and then `00538f40` on both, and sets the
/// word at `+0x40` to `0x24`. `thiscall`, `RET 8`; answers `this`.
pub fn fn_0061fba0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.call(
        ENTRY_NODE_CONSTRUCT,
        &args![base.wrapping_add(0x10), first, second],
    );
    e.mem.set_u32(base, 0x0104_d9c4);
    e.call(MEMBER_SETUP, &args![base.wrapping_add(0x20)]);
    e.call(MEMBER_SETUP, &args![base.wrapping_add(0x30)]);
    e.call(MEMBER_SETUP_AFTER, &args![base.wrapping_add(0x20)]);
    e.call(MEMBER_SETUP_AFTER, &args![base.wrapping_add(0x30)]);
    e.mem.set_u32(base.wrapping_add(0x40), 0x24);
    this
}

// Translated from 0061fc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `ZoneEntry` (RTTI `.?AVZoneEntry@@`, vtable `0104d9cc`): constructs the
/// embedded node at `+4` with the two arguments (`00620720`), stores the vtable and clears the
/// word at `+0x14`. `thiscall`, `RET 8`; answers `this`.
pub fn fn_0061fc10(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.call(
        ENTRY_NODE_CONSTRUCT,
        &args![base.wrapping_add(4), first, second],
    );
    e.mem.set_u32(base, 0x0104_d9cc);
    e.mem.set_u32(base.wrapping_add(0x14), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_fc50, fn_0061fc50(Ptr)),
        entry!(0x0061_fba0, fn_0061fba0(Ptr, u32, u32) -> Ptr),
        entry!(0x0061_fc10, fn_0061fc10(Ptr, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_0061fc50_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0061_fea0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0061_fc50, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0061_fea0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0104_d9cc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0104_d9cc,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod havok_glue_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ENTRY_NODE_CONSTRUCT, |_, _| Ret::default());
        e.register(MEMBER_SETUP, |_, _| Ret::default());
        e.register(MEMBER_SETUP_AFTER, |_, _| Ret::default());
        e
    }

    #[test]
    fn at_0061fba0_builds_a_target_entry() {
        let mut e = rig();
        // The node constructor leaves a marker word in the first word of the entry.
        e.register(ENTRY_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] - 0x10, 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x44);
        e.mem.write(this, &[0xcd; 0x44]);
        assert_eq!(e.call(0x0061_fba0, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(
            calls(&e, ENTRY_NODE_CONSTRUCT),
            vec![vec![this + 0x10, 5, 6]]
        );
        assert_eq!(e.mem.u32(this), 0x0104_d9c4, "the vtable follows the node");
        assert_eq!(e.mem.u32(this + 0x40), 0x24);
        assert_eq!(
            calls(&e, MEMBER_SETUP),
            vec![vec![this + 0x20], vec![this + 0x30]]
        );
        assert_eq!(
            calls(&e, MEMBER_SETUP_AFTER),
            vec![vec![this + 0x20], vec![this + 0x30]]
        );
    }

    #[test]
    fn at_0061fc10_builds_a_zone_entry() {
        let mut e = rig();
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xcd; 0x18]);
        assert_eq!(e.call(0x0061_fc10, &args![this, 7u32, 8u32]).u32(), this);
        assert_eq!(calls(&e, ENTRY_NODE_CONSTRUCT), vec![vec![this + 4, 7, 8]]);
        assert_eq!(e.mem.u32(this), 0x0104_d9cc);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 8), 0xcdcd_cdcd);
    }
}
