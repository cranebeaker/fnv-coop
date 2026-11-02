//! `(unplaced)/run_00844c90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006f3170` (`thiscall`, `bool`): whether the array should shrink.
const SHOULD_SHRINK_006F3170: u32 = 0x006f_3170;
/// `00869600` (`thiscall`): the new capacity.
const NEW_CAPACITY_00869600: u32 = 0x0086_9600;
/// `00692480` (`thiscall`: destination, source, count): moves elements.
const MOVE_00692480: u32 = 0x0069_2480;
/// `0072ba80` (`thiscall`: an element, a count): destroys elements.
const DESTROY_0072BA80: u32 = 0x0072_ba80;
/// `006a8500` (`thiscall`): frees the old buffer.
const FREE_BUFFER_006A8500: u32 = 0x006a_8500;

// Translated from 00844c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words): removes element `index` of an array of
/// 8-byte elements (buffer +4, count +8, capacity +0xC). With `shrink` set and `006f3170`
/// answering true, a new buffer of `00869600`'s capacity (vtable +4) takes the elements
/// before the index and, after the element is destroyed (`0072ba80` with 1), the count - 1
/// elements after it (as the exe passes them), the old buffer is freed (`006a8500`) and the
/// new one and its capacity are kept; otherwise the element is destroyed and the following
/// ones moved down in place. The count drops by one.
pub fn fn_00844c90(e: &mut Engine, this: Ptr, index: u32, shrink: u8) {
    let at = this.addr();
    if shrink != 0 && e.call(SHOULD_SHRINK_006F3170, &args![at]).bool() {
        let capacity = e.call(NEW_CAPACITY_00869600, &args![at]).u32();
        let buffer = e.vcall(at, 0x4, &args![capacity]).u32();
        let old = e.mem.u32(at + 4);
        e.call(MOVE_00692480, &args![at, buffer, old, index]);
        let element = e.mem.u32(at + 4).wrapping_add(index.wrapping_mul(8));
        e.call(DESTROY_0072BA80, &args![at, element, 1u32]);
        let count = e.mem.u32(at + 8).wrapping_sub(1);
        let source = e
            .mem
            .u32(at + 4)
            .wrapping_add(index.wrapping_mul(8))
            .wrapping_add(8);
        let destination = buffer.wrapping_add(index.wrapping_mul(8));
        e.call(MOVE_00692480, &args![at, destination, source, count]);
        e.call(FREE_BUFFER_006A8500, &args![at]);
        e.mem.set_u32(at + 4, buffer);
        e.mem.set_u32(at + 0xc, capacity);
    } else {
        let element = e.mem.u32(at + 4).wrapping_add(index.wrapping_mul(8));
        e.call(DESTROY_0072BA80, &args![at, element, 1u32]);
        let count = e.mem.u32(at + 8).wrapping_sub(index).wrapping_sub(1);
        let source = e
            .mem
            .u32(at + 4)
            .wrapping_add(index.wrapping_mul(8))
            .wrapping_add(8);
        let destination = e.mem.u32(at + 4).wrapping_add(index.wrapping_mul(8));
        e.call(MOVE_00692480, &args![at, destination, source, count]);
    }
    let count = e.mem.u32(at + 8).wrapping_sub(1);
    e.mem.set_u32(at + 8, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_4c90, fn_00844c90(Ptr, u32, u8))]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = [(u32, Vec<u32>)];

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [MOVE_00692480, DESTROY_0072BA80, FREE_BUFFER_006A8500] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(NEW_CAPACITY_00869600, |_, _| 6u32.into_ret());
        e
    }

    /// The array with its vtable (slot +4 answers the new buffer 0x9000).
    fn array(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, vtable + 4);
        e.register_double(vtable + 4, |_, a| {
            assert_eq!(a[1], 6);
            0x9000u32.into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 8, 5);
        e.mem.set_u32(this + 0xc, 10);
        this
    }

    #[test]
    fn removes_in_place() {
        let mut e = engine();
        let this = array(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0084_4c90, &args![this, 1u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SHOULD_SHRINK_006F3170).is_empty());
        assert_eq!(
            calls_to(&log, DESTROY_0072BA80),
            vec![vec![this, 0x5008, 1]]
        );
        assert_eq!(
            calls_to(&log, MOVE_00692480),
            vec![vec![this, 0x5008, 0x5010, 3]]
        );
        assert_eq!(e.mem.u32(this + 8), 4);
    }

    #[test]
    fn shrinks_into_a_new_buffer() {
        let mut e = engine();
        e.register(SHOULD_SHRINK_006F3170, |_, _| 1u32.into_ret());
        let this = array(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0084_4c90, &args![this, 2u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MOVE_00692480),
            vec![vec![this, 0x9000, 0x5000, 2], vec![this, 0x9010, 0x5018, 4]]
        );
        assert_eq!(
            calls_to(&log, DESTROY_0072BA80),
            vec![vec![this, 0x5010, 1]]
        );
        assert_eq!(calls_to(&log, FREE_BUFFER_006A8500), vec![vec![this]]);
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0x9000, 4, 6)
        );
        // Shrinking not wanted: in place.
        e.register(SHOULD_SHRINK_006F3170, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0084_4c90, &args![this, 0u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MOVE_00692480),
            vec![vec![this, 0x9000, 0x9008, 3]]
        );
    }
}
