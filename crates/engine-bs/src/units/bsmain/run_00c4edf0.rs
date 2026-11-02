//! `BSMain/run_00c4edf0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c4ee40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The occlusion query's result read (unnamed in the map; `thiscall`, `(&result, flush)`,
/// `RET 8`): the query object held at `this + 0` gets its COM method at vtable +0x1C with
/// (query, `result`, 4, 1 when `flush` is set else 0). Answers whether that returned 0 (the
/// low byte; the rest of EAX keeps the returned value's upper bytes, not modelled).
pub fn fn_00c4ee40(e: &mut Engine, this: Ptr, result: Ptr, flush: u8) -> bool {
    let query = e.mem.u32(this.addr());
    let flags = u32::from(flush != 0);
    let returned = e.vcall(query, 0x1c, &args![result, 4u32, flags]).u32();
    returned == 0
}

// Translated from 00c4edf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map; the same code as [`fn_00c4ee40`] at another address (`thiscall`,
/// `(&result, flush)`, `RET 8`): the query object held at `this + 0` gets its COM method at
/// vtable +0x1C with (query, `result`, 4, 1 when `flush` is set else 0). Answers whether that
/// returned 0 (the low byte; the rest of EAX keeps the returned value's upper bytes, not
/// modelled).
pub fn fn_00c4edf0(e: &mut Engine, this: Ptr, result: Ptr, flush: u8) -> bool {
    let query = e.mem.u32(this.addr());
    let flags = u32::from(flush != 0);
    let returned = e.vcall(query, 0x1c, &args![result, 4u32, flags]).u32();
    returned == 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4ee40, fn_00c4ee40(Ptr, Ptr, u8) -> bool),
        entry!(0x00c4_edf0, fn_00c4edf0(Ptr, Ptr, u8) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c4ee40_reads_the_query_result() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0090_001c, |_, a| Ret {
            eax: a[3],
            ..Ret::default()
        });
        e.put_vtable(0x0090_1000, &[0, 0, 0, 0, 0, 0, 0, 0x0090_001c]);
        let query = e.mem.alloc(8);
        e.mem.set_u32(query, 0x0090_1000);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, query);
        // The double answers the flags word: 0 (S_OK) without flush, 1 with.
        assert!(e.call(0x00c4ee40, &args![this, 0x7000u32, 0u32]).bool());
        assert!(!e
            .call(0x00c4ee40, &args![this, 0x7000u32, 0x100u32 | 2])
            .bool());
        let reads: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x0090_001c)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            reads,
            vec![vec![query, 0x7000, 4, 0], vec![query, 0x7000, 4, 1]]
        );
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00c4edf0_reads_the_query_result() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        // The double answers the flags word: 0 (S_OK) without flush, 1 with.
        e.register(0x0090_001c, |_, a| Ret {
            eax: a[3],
            ..Ret::default()
        });
        e.put_vtable(0x0090_1000, &[0, 0, 0, 0, 0, 0, 0, 0x0090_001c]);
        let query = e.mem.alloc(8);
        e.mem.set_u32(query, 0x0090_1000);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, query);
        assert!(e.call(0x00c4_edf0, &args![this, 0x7000u32, 0u32]).bool());
        // Only the flag's low byte counts.
        assert!(!e
            .call(0x00c4_edf0, &args![this, 0x7000u32, 0x100u32 | 2])
            .bool());
        assert!(e
            .call(0x00c4_edf0, &args![this, 0x7000u32, 0x100u32])
            .bool());
        let reads: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x0090_001c)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            reads,
            vec![
                vec![query, 0x7000, 4, 0],
                vec![query, 0x7000, 4, 1],
                vec![query, 0x7000, 4, 0]
            ]
        );
    }
}
