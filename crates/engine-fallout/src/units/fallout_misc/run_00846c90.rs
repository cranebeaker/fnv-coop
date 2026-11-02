//! `fallout/misc/run_00846c90` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00846c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gets or allocates an index, retaining both directions of the map.
/// PC layout: forward map at +0, reverse map at +0x10, next index at +0x20.
/// The engine map does not assign this body a class or method name.
pub fn get_or_add_index(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    let data_handler = e.mem.u32(0x011c3f2c);
    if e.call(0x00469860, &args![data_handler, id]).u8() != 0 {
        return id;
    }
    e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        if e.call(0x00853130, &args![this, id, out]).u8() != 0 {
            return e.mem.u32(out.addr());
        }
        let index = e.mem.u32(this.addr().wrapping_add(0x20));
        e.mem
            .set_u32(this.addr().wrapping_add(0x20), index.wrapping_add(1));
        e.call(0x00844700, &args![this, id, index]);
        e.call(
            0x00844700,
            &args![this.addr().wrapping_add(0x10), index, id],
        );
        index
    })
}

// Translated from 00846d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves an index through the reverse map at +0x10; absent entries
/// return zero even if the failed lookup changed its output word.
pub fn resolve_index(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let data_handler = e.mem.u32(0x011c3f2c);
    if e.call(0x00469860, &args![data_handler, index]).u8() != 0 {
        return index;
    }
    e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        if e.call(
            0x00853130,
            &args![this.addr().wrapping_add(0x10), index, out],
        )
        .u8()
            != 0
        {
            e.mem.u32(out.addr())
        } else {
            0
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00846c90, get_or_add_index(Ptr, u32) -> u32),
        entry!(0x00846d20, resolve_index(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011c3f2c, 4);
        e.mem.set_u32(0x011c3f2c, 0x1234);
        let map = e.mem.alloc(0x24);
        e.mem.set_u32(map + 0x20, 7);
        (e, map)
    }

    #[test]
    fn passthrough_uses_low_byte_and_never_touches_map() {
        let (mut e, map) = fixture();
        e.register_double(0x00469860, |_, a| {
            assert_eq!(a, &[0x1234, 0xff001234]);
            0x101u32.into_ret()
        });
        for address in [0x00846c90, 0x00846d20] {
            assert_eq!(
                e.call(address, &args![0u32, 0xff001234u32]).u32(),
                0xff001234
            );
        }
        assert_eq!(e.mem.u32(map + 0x20), 7);
    }

    #[test]
    fn existing_index_and_reverse_lookup_use_output_only_on_success() {
        let (mut e, map) = fixture();
        e.register_double(0x00469860, |_, _| 0x100u32.into_ret());
        for (address, receiver, success, expected) in [
            (0x00846c90, map, 0x101u32, 0xfedcba98),
            (0x00846d20, map + 0x10, 0x101, 0xfedcba98),
            (0x00846d20, map + 0x10, 0x100, 0),
        ] {
            e.register_double(0x00853130, move |e, a| {
                assert_eq!(&a[..2], &[receiver, 0x314159]);
                assert_eq!(a.len(), 3);
                assert_eq!(e.mem.u32(a[2]), 0);
                e.mem.set_u32(a[2], 0xfedcba98);
                success.into_ret()
            });
            assert_eq!(e.call(address, &args![map, 0x314159u32]).u32(), expected);
            assert_eq!(e.mem.u32(map + 0x20), 7);
        }
    }

    #[test]
    fn missing_index_reloads_counter_wraps_and_keeps_captured_id_across_inserts() {
        let (mut e, map) = fixture();
        e.register_double(0x00469860, |_, _| 0u32.into_ret());
        e.register_double(0x00853130, move |e, a| {
            assert_eq!(&a[..2], &[map, 0x102030]);
            e.mem.set_u32(map + 0x20, u32::MAX);
            e.mem.set_u32(a[2], 99);
            0x100u32.into_ret()
        });
        e.register_double(0x00844700, move |e, a| {
            if a[0] == map {
                assert_eq!(a, &[map, 0x102030, u32::MAX]);
                assert_eq!(e.mem.u32(map + 0x20), 0);
                e.mem.set_u32(map + 0x20, 66);
            } else {
                assert_eq!(a, &[map + 0x10, u32::MAX, 0x102030]);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00846c90, &args![map, 0x102030u32]).u32(), u32::MAX);
        let inserts: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00844700)
            .map(|(_, arguments)| arguments.clone())
            .collect();
        assert_eq!(
            inserts,
            [
                vec![map, 0x102030, u32::MAX],
                vec![map + 0x10, u32::MAX, 0x102030]
            ]
        );
        assert_eq!(e.mem.u32(map + 0x20), 66);
    }
}
