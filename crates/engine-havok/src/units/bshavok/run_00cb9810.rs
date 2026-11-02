//! `BSHavok/run_00cb9810` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::enter_scope;

/// `hkpAllCdBodyPairCollector::addCdBodyPair` (Xbox PDB, `00cb7550`, `this` plus two bodies).
const ADD_BODY_PAIR: u32 = 0x00cb_7550;
/// The routine `fn_00cb9900` continues in (`0062d4b0`, `this` only).
const BASE_DESTRUCT: u32 = 0x0062_d4b0;
/// The vtable `fn_00cb9900` stores.
const COLLECTOR_VTABLE: u32 = 0x0108_4e54;

/// Follows the chain of words at `+0xC` from `start` to its last link.
pub(crate) fn chain_end(e: &Engine, start: u32) -> u32 {
    let mut link = start;
    loop {
        let next = e.mem.u32(link + 0xc);
        if next == 0 {
            return link;
        }
        link = next;
    }
}

// Translated from 00cb9810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb9810 - the FS:[0] exception frame is not modeled.
/// `fn_00cb9810` (unnamed in the Xbox PDB): takes two bodies. With the allocation-scope word
/// set to 0x10: looks through the `+0xC` signed count of 16-byte records at `+0x8` for one
/// whose word at `+8` equals the end of the `+0xC` chain of the second body; when none does
/// (or there are no records) it runs `hkpAllCdBodyPairCollector::addCdBodyPair` (`00cb7550`)
/// on `(this, first, second)`. The scope word is then put back.
pub fn fn_00cb9810(e: &mut Engine, this: u32, first: u32, second: u32) {
    let (word, saved) = enter_scope(e);
    let count = e.mem.i32(this + 0xc);
    let mut found = false;
    let mut index = 0i32;
    while index < count && !found {
        let root = chain_end(e, second);
        let records = e.mem.u32(this + 8);
        let record_word = e
            .mem
            .u32(records.wrapping_add(index as u32 * 16).wrapping_add(8));
        found = root == record_word;
        index += 1;
    }
    if !found {
        e.call(ADD_BODY_PAIR, &args![this, first, second]);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cb9900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cb9900` (unnamed in the Xbox PDB): stores the vtable 0x01084e54 and continues in
/// `0062d4b0` on the same object (a tail jump). The shape is that of a destructor; the class is
/// not confirmed.
pub fn fn_00cb9900(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, COLLECTOR_VTABLE);
    e.call(BASE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb9810, fn_00cb9810(u32, u32, u32)),
        entry!(0x00cb9900, fn_00cb9900(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};
    use crate::units::bshavok::bhkentity::SCOPE_WORD;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A collector with `count` records, record `slot` holding the end of the chain of the
    /// second body when `matching`; the second body is a chain of two links.
    fn setup(count: i32, slot: u32, matching: bool) -> (Engine, u32, u32) {
        let mut e = logging_engine();
        e.register_double(ADD_BODY_PAIR, |e, _| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            Ret::default()
        });
        let end = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(second + 0xc, end);
        let this = e.mem.alloc(0x20);
        let records = e.mem.alloc(16 * count.max(1) as u32);
        e.mem.set_u32(this + 8, records);
        e.mem.set_i32(this + 0xc, count);
        if matching {
            e.mem.set_u32(records + 16 * slot + 8, end);
        }
        (e, this, second)
    }

    #[test]
    fn a_known_body_is_not_added() {
        let (mut e, this, second) = setup(3, 2, true);
        e.call(0x00cb_9810, &args![this, 0x11u32, second]);
        assert!(calls_to(&e, ADD_BODY_PAIR).is_empty());
        let word = e.tls() + SCOPE_WORD;
        assert_eq!(e.mem.u32(word), 0);
    }

    #[test]
    fn an_unknown_body_is_added_inside_the_scope() {
        let (mut e, this, second) = setup(3, 2, false);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 5);
        e.call(0x00cb_9810, &args![this, 0x11u32, second]);
        assert_eq!(calls_to(&e, ADD_BODY_PAIR), [[this, 0x11, second]]);
        assert_eq!(e.mem.u32(word), 5);
    }

    #[test]
    fn no_records_means_the_body_is_added() {
        let (mut e, this, second) = setup(0, 0, false);
        e.call(0x00cb_9810, &args![this, 0x11u32, second]);
        assert_eq!(calls_to(&e, ADD_BODY_PAIR).len(), 1);
    }

    #[test]
    fn the_second_routine_stores_the_vtable_and_continues() {
        let mut e = logging_engine();
        e.register(BASE_DESTRUCT, nothing);
        let this = e.mem.alloc(0x10);
        e.call(0x00cb_9900, &args![this]);
        assert_eq!(e.mem.u32(this), COLLECTOR_VTABLE);
        assert_eq!(calls_to(&e, BASE_DESTRUCT), [[this]]);
    }
}
