//! `Havok SDK/run_00d942f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00d07120` (`hkpsimulationisland.cpp`): `thiscall`, no stack words.
const ISLAND_CALL: u32 = 0x00d0_7120;
/// `hkpWorldOperationUtil::splitSimulationIsland` (`00cb6060`, engine map name): cdecl, four
/// words.
const SPLIT_SIMULATION_ISLAND: u32 = 0x00cb_6060;

// Translated from 00d942f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(island, argument)`, the body of a split-island job (`hkCpuSplitSimulationIslandJob`
/// calls it): clears the two low bits of the byte at `island + 0x25`, runs `00d07120` on the
/// island, then `hkpWorldOperationUtil::splitSimulationIsland` (`00cb6060`) with `(island,
/// [island + 0x18], argument, &array)` where `array` is a local empty `hkArray` (data 0, size 0,
/// capacity word `0x80000000`) that is never released here.
pub fn fn_00d942f0(e: &mut Engine, island: u32, argument: u32) {
    let flags = e.mem.u8(island.wrapping_add(0x25));
    e.mem.set_u8(island.wrapping_add(0x25), flags & 0xfc);
    e.with_stack(0xc, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        e.call(ISLAND_CALL, &args![island]);
        let world = e.mem.u32(island.wrapping_add(0x18));
        e.call(
            SPLIT_SIMULATION_ISLAND,
            &args![island, world, argument, array],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9_42f0, fn_00d942f0(u32, u32))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn the_island_flags_are_masked_and_the_split_gets_an_empty_array() {
        let mut e = Engine::new();
        e.register(ISLAND_CALL, |_, _| Ret::default());
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(SPLIT_SIMULATION_ISLAND, move |e, a| {
            let array = [e.mem.u32(a[3]), e.mem.u32(a[3] + 4), e.mem.u32(a[3] + 8)];
            log.borrow_mut()
                .push((a.to_vec(), array, e.mem.u8(a[0] + 0x25)));
            Ret::default()
        });
        let island = e.mem.alloc(0x40);
        e.mem.set_u8(island + 0x25, 0xff);
        e.mem.set_u32(island + 0x18, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00d9_42f0, &args![island, 0x77u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[..3], [island, 0x6000, 0x77]);
        assert_eq!(seen[0].1, [0, 0, 0x8000_0000]);
        assert_eq!(seen[0].2, 0xfc);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(order, [0x00d9_42f0, ISLAND_CALL, SPLIT_SIMULATION_ISLAND]);
        assert_eq!(log[1].1, vec![island]);
    }
}
