//! `(unplaced)/run_008528d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004b9ba0` (`thiscall`, no stack word; unit `fallout shared/globalfunc.cpp`): the map's
/// first position.
const FIRST_POSITION: u32 = 0x004b_9ba0;
/// `006b7f20` (`thiscall`, `RET 0xc`; unit `fallout shared/pathfinding/navmeshinfomap.cpp`):
/// advances the position at the first address and stores the key and the value at the other
/// two.
const GET_NEXT: u32 = 0x006b_7f20;
/// `00438af0` (`thiscall`, no stack word): empties the map.
const EMPTY_MAP: u32 = 0x0043_8af0;

// Translated from 008528d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): deletes every value of the map
/// `this` and empties it. From the first position (`004b9ba0`), while the position is not
/// null: the key and value locals are set to 0, `006b7f20` takes the three addresses
/// (position, key, value), and a non-null value gets its vtable slot 0 called with 1. Then
/// `00438af0` empties the map.
pub fn fn_008528d0(e: &mut Engine, this: Ptr) {
    let first = e.call(FIRST_POSITION, &args![this]).u32();
    // The three stack words: the position (+0), the key (+4), the value (+8).
    e.with_stack(0xc, |e, locals| {
        let position = locals.addr();
        let key = position + 4;
        let value = position + 8;
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(GET_NEXT, &args![this, position, key, value]);
            let object = e.mem.u32(value);
            if object != 0 {
                e.vcall(object, 0, &args![1u32]);
            }
        }
    });
    e.call(EMPTY_MAP, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0085_28d0, fn_008528d0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deletes_every_value_then_empties_the_map() {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0150_0000);
        e.register(FIRST_POSITION, |_, _| 0x10u32.into_ret());
        // Position 0x10 gives the object and moves to 0x20; 0x20 gives null and ends.
        e.register_double(GET_NEXT, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            assert_eq!(e.mem.u32(a[3]), 0);
            let (next, value) = if e.mem.u32(a[1]) == 0x10 {
                (0x20, object)
            } else {
                (0, 0)
            };
            e.mem.set_u32(a[1], next);
            e.mem.set_u32(a[3], value);
            Ret::default()
        });
        e.register(EMPTY_MAP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0085_28d0, &args![0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == GET_NEXT).count(), 2);
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == 0x0150_1000)
                .collect::<Vec<_>>(),
            [&(0x0150_1000, vec![object, 1])]
        );
        assert_eq!(log.last().unwrap(), &(EMPTY_MAP, vec![0x7000]));
        // An empty map: only the emptying.
        e.register(FIRST_POSITION, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0085_28d0, &args![0x7000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }
}
