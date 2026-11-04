//! `(unplaced)/run_00486c80` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The destructor chain of `NiTStringTemplateMap<NiTMap<const char*,TESForm*>,TESForm*>`
//! (the RTTI names of the vtables `0101cad8`, `0101caf8` and `0101cb18`): the map keeps its
//! bucket count at +4, its bucket array at +8 and (the string map) whether it owns copies
//! of its keys in the byte at +0x10; each item is `next`, key, value.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTStringTemplateMap<NiTMap<const char*,TESForm*>,TESForm*>`'s vtable.
const STRING_MAP_VTABLE: u32 = 0x0101_cad8;
/// `NiTMap<const char*,TESForm*>`'s vtable.
const MAP_VTABLE: u32 = 0x0101_caf8;
/// `NiTMapBase<DFALL<NiTMapItem<const char*,TESForm*>>,const char*,TESForm*>`'s vtable.
const MAP_BASE_VTABLE: u32 = 0x0101_cb18;
/// `00438af0` (`ECX` = the map): removes every item.
const REMOVE_ALL: u32 = 0x0043_8af0;
/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x00aa_10f0;

// Translated from 00486c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, plain `RET`; called by `00486c60` and `00486f10`):
/// runs the string map's destructor [`fn_00486ca0`] on `this`.
pub fn fn_00486c80(e: &mut Engine, this: Ptr) {
    fn_00486ca0(e, this);
}

// Translated from 00486ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00486ca0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eef128, state variable) and the frame's cookie word are not translated.
/// The `NiTStringTemplateMap` destructor body (unnamed in the engine map; `ECX`, plain
/// `RET`): stores its vtable; when the map owns its keys (the byte at +0x10), frees the key
/// (+4) of every item of every bucket (`00aa10f0`); then the `NiTMap` destructor
/// [`fn_00486d60`].
pub fn fn_00486ca0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, STRING_MAP_VTABLE);
    if e.mem.u8(base + 0x10) != 0 {
        let mut bucket = 0u32;
        while bucket < e.mem.u32(base + 4) {
            let buckets = e.mem.u32(base + 8);
            let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
            while item != 0 {
                let current = item;
                item = e.mem.u32(current);
                let key = e.mem.u32(current.wrapping_add(4));
                e.call(FREE, &args![key]);
            }
            bucket += 1;
        }
    }
    fn_00486d60(e, this);
}

// Translated from 00486d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00486d60 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eef158, state variable) and the frame's cookie word are not translated.
/// The `NiTMap<const char*,TESForm*>` destructor body (`ECX`, plain `RET`): stores its
/// vtable, removes every item (`00438af0`), then the base destructor [`fn_00486dc0`].
pub fn fn_00486d60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    e.call(REMOVE_ALL, &args![this]);
    fn_00486dc0(e, this);
}

// Translated from 00486dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` destructor body (`ECX`, plain `RET`): stores its vtable, removes every
/// item (`00438af0`) and frees the bucket array at +8 (`00aa10f0`).
pub fn fn_00486dc0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE, &args![buckets]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_6c80, fn_00486c80(Ptr)),
        entry!(0x0048_6ca0, fn_00486ca0(Ptr)),
        entry!(0x0048_6d60, fn_00486d60(Ptr)),
        entry!(0x0048_6dc0, fn_00486dc0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// A map of two buckets: bucket 0 holds two items (keys 0x501, 0x502), bucket 1 none.
    fn engine(owns: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(REMOVE_ALL, |e, a| {
            // The vtable is the map's own when it runs.
            assert_ne!(e.mem.u32(a[0]), STRING_MAP_VTABLE);
            Ret::default()
        });
        e.register(FREE, |_, _| Ret::default());
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        let second = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x501);
        e.mem.set_u32(second + 4, 0x502);
        e.mem.set_u32(buckets, first);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u8(map + 0x10, owns);
        e.call_log = Some(vec![]);
        (e, map)
    }

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00486ca0_frees_owned_keys_then_the_buckets() {
        let (mut e, map) = engine(1);
        let buckets = e.mem.u32(map + 8);
        e.call(0x0048_6ca0, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, FREE),
            vec![vec![0x501], vec![0x502], vec![buckets]]
        );
        assert_eq!(calls(&log, REMOVE_ALL), vec![vec![map], vec![map]]);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
    }

    #[test]
    fn fn_00486c80_keeps_keys_it_does_not_own() {
        let (mut e, map) = engine(0);
        let buckets = e.mem.u32(map + 8);
        e.call(0x0048_6c80, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FREE), vec![vec![buckets]]);
    }

    #[test]
    fn fn_00486d60_and_00486dc0_unwind_the_bases() {
        let (mut e, map) = engine(1);
        e.call(0x0048_6d60, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, REMOVE_ALL).len(), 2);
        assert_eq!(calls(&log, FREE).len(), 1);
        let (mut e, map) = engine(1);
        e.call(0x0048_6dc0, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, REMOVE_ALL).len(), 1);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
    }
}
