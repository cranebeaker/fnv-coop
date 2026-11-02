//! `BSMain/bsocclusionquery.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The flag `End` checks and clears (a byte global).
const QUERY_ISSUED: u32 = 0x0120_30a4;

// Translated from 00c4edb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionQuery::End` (Xbox PDB): when the byte at `012030a4` is set, it is cleared and
/// the query object held at `this + 0` gets its COM method at vtable +0x18 with (query, 1).
pub fn bs_occlusion_query_end(e: &mut Engine, this: Ptr) {
    if e.global::<u8>(QUERY_ISSUED) != 0 {
        e.set_global::<u8>(QUERY_ISSUED, 0);
        let query = e.mem.u32(this.addr());
        e.vcall(query, 0x18, &args![1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c4edb0, bs_occlusion_query_end(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_issues_once_while_the_flag_is_set() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(QUERY_ISSUED, 1);
        e.register(0x0090_0018, |_, _| Ret::default());
        e.put_vtable(0x0090_1000, &[0, 0, 0, 0, 0, 0, 0x0090_0018]);
        let query = e.mem.alloc(8);
        e.mem.set_u32(query, 0x0090_1000);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, query);
        e.call(0x00c4edb0, &args![this]);
        e.set_global::<u8>(QUERY_ISSUED, 1);
        e.call(0x00c4edb0, &args![this]);
        e.call(0x00c4edb0, &args![this]);
        assert_eq!(e.global::<u8>(QUERY_ISSUED), 0);
        let issued: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x0090_0018)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(issued, vec![vec![query, 1]]);
    }
}
