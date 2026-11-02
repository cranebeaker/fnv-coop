//! `Havok SDK/run_00de2fa0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkadeltacompressedanimation::profile_record;

// Translated from 00de3040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x28` (`SUB ECX,0x28`) and jumps to `00d68910`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00de3040(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8910, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

/// The profile labels `"TthkpActiveContactPointViewer::postSimulationCallback"` (`010d164c`)
/// and `"Et"` (`010c41f8`) of the records `00de2fa0` writes around its work.
const LABEL_POST_SIMULATION: u32 = 0x010d_164c;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkpContactPointViewer::drawAllContactPointsInIsland` (`00de2da0`, thiscall on the viewer,
/// one word).
const DRAW_ALL_CONTACT_POINTS: u32 = 0x00de_2da0;
/// The vtable stored by `00de3060` (RTTI `hkpConvexRadiusBuilder`).
const CONVEX_RADIUS_BUILDER_VTABLE: u32 = 0x010d_8214;
/// The `float` `00de3050` stores: `0.001` (`01017d00`) in the exe.
const SETUP_DEFAULT: u32 = 0x0101_7d00;

// Translated from 00de2fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Post simulation callback of the contact point viewer (`this` = the listener sub-object at
/// the viewer's `+0x28`, `arg_1` the stepping argument; the profile label names
/// `hkpActiveContactPointViewer::postSimulationCallback`, a body this class's vtable shares):
/// between two profiling records (`TthkpActiveContactPointViewer::postSimulationCallback` and
/// `Et`) asks the viewer (`this - 0x28`, vtable slot `0xc`, `arg_1`) for an array (data at
/// `+0`, signed count at `+4`) and calls `drawAllContactPointsInIsland` (`00de2da0`) with every
/// element in turn, the count read again after each call. `RDTSC` is the host clock.
pub fn fn_00de2fa0(e: &mut Engine, this: Ptr, arg_1: u32) {
    profile_record(e, LABEL_POST_SIMULATION);
    let viewer = this.addr().wrapping_sub(0x28);
    let islands = e.vcall(viewer, 0xc, &args![arg_1]).u32();
    let mut index: i32 = 0;
    while index < e.mem.i32(islands.wrapping_add(4)) {
        let data = e.mem.u32(islands);
        let island = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        e.call(DRAW_ALL_CONTACT_POINTS, &args![viewer, island]);
        index = index.wrapping_add(1);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00de3050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the setup record of `hkpConvexRadiusBuilder` (the engine map has no name;
/// `hkpConvexRadiusViewer` makes the record with it and passes it to `00de3060`): stores the
/// float at `01017d00` through the x87 stack (`FLD`/`FSTP`: a signalling NaN comes out quiet)
/// at `this + 0`. Returns `this`.
pub fn fn_00de3050(e: &mut Engine, this: Ptr) -> Ptr {
    let value = x87_quiet_bits(e.mem.u32(SETUP_DEFAULT));
    e.mem.set_u32(this.addr(), value);
    this
}

// Translated from 00de3060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexRadiusBuilder::hkpConvexRadiusBuilder` (constructor; class confirmed by the RTTI
/// of the vtable it stores, `010d8214`), thiscall with the setup record: the 16-bit word at
/// `+6` is 1, the vtable pointer goes to `+0`, the first word of `setup` (copied as an integer)
/// to `+8` and 0 to `+0xc`. Returns `this`.
pub fn fn_00de3060(e: &mut Engine, this: Ptr, setup: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me.wrapping_add(6), 1);
    e.mem.set_u32(me, CONVEX_RADIUS_BUILDER_VTABLE);
    let first = e.mem.u32(setup.addr());
    e.mem.set_u32(me.wrapping_add(8), first);
    e.mem.set_u32(me.wrapping_add(0xc), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_2fa0, fn_00de2fa0(Ptr, u32)),
        entry!(0x00de_3040, fn_00de3040(Ptr, u32) -> u32),
        entry!(0x00de_3050, fn_00de3050(Ptr) -> Ptr),
        entry!(0x00de_3060, fn_00de3060(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00de3040() {
        assert_eq!(check(0x00de_3040, 0x00d6_8910, 0x28), 0xdead_beef);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const PROFILE_SLOT: u32 = 0x0126_835c;
    const TLS_GET_VALUE: u32 = 0x00fd_f044;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A profiling stream with room for `records` records; the TLS lookup answers it. Returns
    /// the stream and the address of its first record.
    fn profile(e: &mut Engine, records: u32) -> (u32, u32) {
        e.map(0x0126_8000, 0x1000);
        let stream = e.mem.alloc(0x20);
        let first = e.mem.alloc(12 * records.max(1));
        e.mem.set_u32(stream + 4, first);
        e.mem.set_u32(stream + 0xc, first + 12 * records);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, words| {
            assert_eq!(words[0], 3);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        (stream, first)
    }

    /// A viewer whose vtable slot `0xc` answers an array of `islands`; calls of the slot and
    /// of the drawing function are logged. The listener sub-object is at `viewer + 0x28`.
    fn viewer(e: &mut Engine, islands: &[u32]) -> (u32, Calls) {
        let calls: Calls = Rc::default();
        let viewer = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(viewer, vtable);
        e.mem.set_u32(vtable + 0xc, 0x0900_000c);
        let array = e.mem.alloc(16);
        let data = e.mem.alloc(4 * islands.len().max(1) as u32);
        for (i, island) in islands.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *island);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, islands.len() as u32);
        for (address, eax) in [(0x0900_000c, array), (DRAW_ALL_CONTACT_POINTS, 0)] {
            let calls = calls.clone();
            e.register_double(address, move |_, words| {
                calls.borrow_mut().push((address, words.to_vec()));
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }
        (viewer, calls)
    }

    #[test]
    fn the_callback_draws_every_island_between_two_records() {
        let mut e = Engine::new();
        let (stream, first) = profile(&mut e, 4);
        let (viewer, calls) = viewer(&mut e, &[0x111, 0x222]);
        e.call(0x00de_2fa0, &args![viewer + 0x28, 0x77u32]);
        assert_eq!(
            *calls.borrow(),
            vec![
                (0x0900_000c, vec![viewer, 0x77]),
                (DRAW_ALL_CONTACT_POINTS, vec![viewer, 0x111]),
                (DRAW_ALL_CONTACT_POINTS, vec![viewer, 0x222]),
            ]
        );
        assert_eq!(e.mem.u32(stream + 4), first + 24);
        assert_eq!(e.mem.u32(first), LABEL_POST_SIMULATION);
        assert_eq!(e.mem.u32(first + 12), LABEL_END);
    }

    #[test]
    fn the_callback_with_no_islands_only_writes_the_records() {
        let mut e = Engine::new();
        let (stream, first) = profile(&mut e, 4);
        let (viewer, calls) = viewer(&mut e, &[]);
        e.call(0x00de_2fa0, &args![viewer + 0x28, 0u32]);
        assert_eq!(calls.borrow().len(), 1);
        assert_eq!(e.mem.u32(stream + 4), first + 24);
    }

    #[test]
    fn a_full_profile_stream_takes_no_record() {
        let mut e = Engine::new();
        let (stream, first) = profile(&mut e, 0);
        let (viewer, calls) = viewer(&mut e, &[5]);
        e.call(0x00de_2fa0, &args![viewer + 0x28, 0u32]);
        assert_eq!(calls.borrow().len(), 2);
        assert_eq!(e.mem.u32(stream + 4), first);
    }

    #[test]
    fn the_setup_record_gets_the_default_float() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.mem.set_u32(SETUP_DEFAULT, 0x3a83_126f);
        let setup = e.mem.alloc(8);
        let result = e.call(0x00de_3050, &args![setup]);
        assert_eq!(result.u32(), setup);
        assert_eq!(e.mem.u32(setup), 0x3a83_126f);
        e.mem.set_u32(SETUP_DEFAULT, 0x7f80_0001);
        e.call(0x00de_3050, &args![setup]);
        assert_eq!(e.mem.u32(setup), 0x7fc0_0001);
    }

    #[test]
    fn the_builder_constructor_stores_the_vtable_and_the_setup_word() {
        let mut e = Engine::new();
        let setup = e.mem.alloc(8);
        e.mem.set_u32(setup, 0x3a83_126f);
        let this = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem.set_u32(this + 4 * word, 0xdead_0000 + word);
        }
        let result = e.call(0x00de_3060, &args![this, setup]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_8214);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x3a83_126f);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 4), 0x0001_0001);
    }
}
