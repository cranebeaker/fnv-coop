//! `Havok SDK/run_00d46410` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d46410 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `table`; descriptions, not confirmed): calls `00cfbf60` (thiscall
/// on `table`, two numbers) 19 times with these pairs, in this order: `(1, 0x12)`, `(3, 0x12)`,
/// `(4, 0x12)`, `(5, 0x12)`, `(2, 0x12)`, `(6, 0x12)`, `(0xb, 0x12)`, `(0xc, 0x12)`, `(0x14, 0x12)`,
/// `(0x17, 7)`, `(9, 7)`, `(0xe, 7)`, `(0x10, 7)`, `(0x1e, 7)`, `(0x1f, 7)`, `(0xa, 8)`,
/// `(0x12, 0x1a)`, `(0x1c, 0x19)`, `(0xd, 0x19)`.
pub fn fn_00d46410(e: &mut Engine, table: Ptr) {
    for (first, second) in [
        (1u32, 0x12u32),
        (3, 0x12),
        (4, 0x12),
        (5, 0x12),
        (2, 0x12),
        (6, 0x12),
        (0xb, 0x12),
        (0xc, 0x12),
        (0x14, 0x12),
        (0x17, 7),
        (9, 7),
        (0xe, 7),
        (0x10, 7),
        (0x1e, 7),
        (0x1f, 7),
        (0xa, 8),
        (0x12, 0x1a),
        (0x1c, 0x19),
        (0xd, 0x19),
    ] {
        e.call(0x00cf_bf60, &args![table, first, second]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_6410, fn_00d46410(Ptr))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d46410_registers_the_nineteen_pairs() {
        let mut e = Engine::new();
        e.register(0x00cf_bf60, |_, _| Ret::default());
        let table = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00d4_6410, &args![table]);
        let log = e.call_log.take().unwrap();
        let pairs: Vec<(u32, u32)> = log[1..].iter().map(|(_, a)| (a[1], a[2])).collect();
        assert_eq!(pairs.len(), 19);
        assert!(log[1..]
            .iter()
            .all(|(address, a)| *address == 0x00cf_bf60 && a[0] == table));
        assert_eq!(pairs[0], (1, 0x12));
        assert_eq!(pairs[4], (2, 0x12));
        assert_eq!(pairs[9], (0x17, 7));
        assert_eq!(pairs[15], (0xa, 8));
        assert_eq!(pairs[16], (0x12, 0x1a));
        assert_eq!(pairs[18], (0xd, 0x19));
    }
}
