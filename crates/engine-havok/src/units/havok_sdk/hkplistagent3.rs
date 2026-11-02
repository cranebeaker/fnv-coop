//! `Havok SDK/hkplistagent3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00d97250` (cdecl, `(word, word, collector)`): adds the statistics of the agent's contents.
const CALC_STATISTICS: u32 = 0x00d9_7250;
/// The string `010d50a8` handed to the collector's slot 0x18 (`beginObject`-like; unconfirmed).
const STATISTICS_NAME: u32 = 0x010d_50a8;

// Translated from 00da6de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkListAgent3::calcStatistics` (Xbox PDB), four stack words (the first is never read): calls
/// slot 0x18 of the collector (the fourth word) with the string `010d50a8`, then `00d97250`
/// with the second and third words and the collector, and ends with a jump to slot 0x1C of
/// the collector (the result of that call is the function's own).
pub fn hk_list_agent3_calc_statistics(
    e: &mut Engine,
    _unused_1: u32,
    first: u32,
    second: u32,
    collector: Ptr,
) {
    e.vcall(collector.addr(), 0x18, &args![STATISTICS_NAME]);
    e.call(CALC_STATISTICS, &args![first, second, collector]);
    e.vcall(collector.addr(), 0x1c, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00da6de0,
        hk_list_agent3_calc_statistics(u32, u32, u32, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const BEGIN_DOUBLE: u32 = 0x0900_0011;
    const END_DOUBLE: u32 = 0x0900_0012;

    #[test]
    fn the_collector_is_bracketed_around_the_statistics_call() {
        let mut e = Engine::new();
        let order = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
        for (addr, tag) in [(BEGIN_DOUBLE, 1u32), (END_DOUBLE, 3), (CALC_STATISTICS, 2)] {
            let sink = Rc::clone(&order);
            e.register_double(addr, move |_, a| {
                let mut row = vec![tag];
                row.extend_from_slice(a);
                sink.borrow_mut().push(row);
                Ret::default()
            });
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x18, BEGIN_DOUBLE);
        e.mem.set_u32(vtable + 0x1c, END_DOUBLE);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, vtable);
        e.call(0x00da6de0, &args![1u32, 2u32, 3u32, collector]);
        assert_eq!(
            *order.borrow(),
            vec![
                vec![1, collector, STATISTICS_NAME],
                vec![2, 2, 3, collector],
                vec![3, collector],
            ]
        );
    }
}
