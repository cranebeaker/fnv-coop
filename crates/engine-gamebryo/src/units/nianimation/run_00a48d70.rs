//! `NiAnimation/run_00a48d70` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Gamebryo allocation.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (engine map, `00a3ca10`; `thiscall`, no
/// stack words).
const KEY_BASED_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_ca10;
/// `NiQuaternionInterpolator`'s vtable (RTTI `.?AVNiQuaternionInterpolator@@` at the
/// locator `01123d40`).
const QUATERNION_INTERPOLATOR_VTABLE: u32 = 0x0109_86f4;
/// The four words at `011f4208` copied into the new object's value (an `NiQuaternion`
/// static of the exe).
const QUATERNION_SOURCE: u32 = 0x011f_4208;
/// The object's size and its fields: the quaternion value at +0xC (four words), then two
/// words cleared at +0x1C and +0x20.
const OBJECT_SIZE: u32 = 0x24;
const VALUE: u32 = 0xc;
const CLEARED_1C: u32 = 0x1c;
const CLEARED_20: u32 = 0x20;

// Translated from 00a48d70 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments; its address is pushed at `00a2db96`):
/// the `NiQuaternionInterpolator` factory. Allocates 0x24 bytes (`00aa13e0`); on success it
/// runs the `NiKeyBasedInterpolator` constructor (`00a3ca10`), stores the
/// `NiQuaternionInterpolator` vtable, copies the four words at `011f4208` to +0xC..+0x18 and
/// clears +0x1C and +0x20. Returns the object, or 0 when the allocation fails.
pub fn fn_00a48d70(e: &mut Engine) -> u32 {
    let object = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, QUATERNION_INTERPOLATOR_VTABLE);
    for word in 0..4 {
        let value = e.mem.u32(QUATERNION_SOURCE + word * 4);
        e.mem.set_u32(object + VALUE + word * 4, value);
    }
    e.mem.set_u32(object + CLEARED_1C, 0);
    e.mem.set_u32(object + CLEARED_20, 0);
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a4_8d70, fn_00a48d70() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig(succeeds: bool) -> Engine {
        let mut e = Engine::new();
        e.register_double(NI_ALLOCATE, move |e, a| {
            assert_eq!(a[0], OBJECT_SIZE);
            let block = if succeeds { e.mem.alloc(a[0]) } else { 0 };
            block.into_ret()
        });
        e.register(KEY_BASED_INTERPOLATOR_CONSTRUCT, |e, a| {
            for offset in (0..OBJECT_SIZE).step_by(4) {
                e.mem.set_u32(a[0] + offset, 0xdddd_dddd);
            }
            Ret::default()
        });
        e.map(QUATERNION_SOURCE, 16);
        for (i, word) in [0x3f80_0000u32, 0x1111_1111, 0x2222_2222, 0x3333_3333]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(QUATERNION_SOURCE + i as u32 * 4, word);
        }
        e
    }

    #[test]
    fn the_factory_builds_a_quaternion_interpolator() {
        let mut e = rig(true);
        e.call_log = Some(vec![]);
        let block = e.call(0x00a4_8d70, &args![]).u32();
        assert_ne!(block, 0);
        assert_eq!(e.mem.u32(block), QUATERNION_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0x3f80_0000);
        assert_eq!(e.mem.u32(block + 0x10), 0x1111_1111);
        assert_eq!(e.mem.u32(block + 0x14), 0x2222_2222);
        assert_eq!(e.mem.u32(block + 0x18), 0x3333_3333);
        assert_eq!(e.mem.u32(block + 0x1c), 0);
        assert_eq!(e.mem.u32(block + 0x20), 0);
        // The base constructor's own fields (+4, +8) are left as it set them.
        assert_eq!(e.mem.u32(block + 4), 0xdddd_dddd);
        let log = e.call_log.take().unwrap();
        let construct = log
            .iter()
            .find(|(a, _)| *a == KEY_BASED_INTERPOLATOR_CONSTRUCT)
            .unwrap();
        assert_eq!(construct.1, vec![block]);
    }

    #[test]
    fn a_failed_allocation_answers_null() {
        let mut e = rig(false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_8d70, &args![]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .all(|(a, _)| *a != KEY_BASED_INTERPOLATOR_CONSTRUCT));
    }
}
