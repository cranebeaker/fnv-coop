//! `fallout shared/run_00479790` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004797c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0101aa98` (RTTI `.?AVBGSIdleCollection@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00479810` (named `BGSIdleCollection::ClearDataComponent` in the engine map). One word, plain `RET`, returns nothing.
pub fn fn_004797c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_aa98);
    e.call(0x0047_9810, &args![this]);
}

// Translated from 00479790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSIdleCollection` constructor (unnamed in the engine map; the
/// vtable `0101aa98` is `BGSIdleCollection`'s by its RTTI): the
/// `BaseFormComponent` base (`00402d30`), the vtable, then
/// `BGSIdleCollection::InitializeDataComponent` (`004797e0`). Answers `this`.
pub fn fn_00479790(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_aa98);
    e.call(0x0047_97e0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_97c0, fn_004797c0(Ptr)),
        entry!(0x0047_9790, fn_00479790(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_004797c0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0047_9810, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0047_97c0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0047_9810)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0101_aa98);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0101_aa98,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    #[test]
    fn fn_00479790_builds_the_base_then_initializes() {
        let mut e = Engine::new();
        e.register(0x0040_2d30, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        // The initializer sees the collection's vtable already stored.
        e.register(0x0047_97e0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_9790, &args![this]).u32(), this);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x0047_9790, 0x0040_2d30, 0x0047_97e0]);
        assert_eq!(e.mem.u32(this), 0x0101_aa98);
        assert_eq!(e.mem.u32(this + 4), 0x0101_aa98);
    }
}
