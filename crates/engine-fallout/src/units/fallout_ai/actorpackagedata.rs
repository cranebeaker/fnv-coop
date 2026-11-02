//! `fallout/ai/actorpackagedata.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The game's `operator new` (`00401000`, cdecl, the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `SandBoxActorPackageData`'s constructor (`009f3900`).
const SANDBOX_DATA_CONSTRUCT: u32 = 0x009f_3900;
/// `PatrolActorPackageData`'s constructor (`009f29a0`).
const PATROL_DATA_CONSTRUCT: u32 = 0x009f_29a0;
/// `GuardActorPackageData`'s constructor (`009f27a0`).
const GUARD_DATA_CONSTRUCT: u32 = 0x009f_27a0;
/// The constructor of the 0x18-byte data of package type 2 (`009f0900`).
const TYPE_2_DATA_CONSTRUCT: u32 = 0x009f_0900;
/// The constructor of the 8-byte data of package type 16 (`0067d8f0`,
/// `fallout shared/packagedatauseweapon.cpp` by the engine map).
const TYPE_16_DATA_CONSTRUCT: u32 = 0x0067_d8f0;

/// Allocates `size` bytes and runs `construct` on them; null when the
/// allocation fails.
fn new_data(e: &mut Engine, size: u32, construct: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    e.call(construct, &args![block]).u32()
}

// Translated from 008c6ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c6ca0 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorPackageData::CreateActorPackageData` (Xbox PDB): new data for a
/// package type, or null. The compiler's `switch` (on `kind - 2`, through a
/// byte map) picks: type 2 a 0x18-byte object (`009f0900`), 12
/// `SandBoxActorPackageData` (0x64), 13 `PatrolActorPackageData` (0x38),
/// 14 `GuardActorPackageData` (0x14), 16 an 8-byte object (`0067d8f0`);
/// every other type, and a failed allocation, gives null.
pub fn actor_package_data_create_actor_package_data(e: &mut Engine, kind: u32) -> Ptr {
    let data = match kind.wrapping_sub(2) {
        0 => new_data(e, 0x18, TYPE_2_DATA_CONSTRUCT),
        10 => new_data(e, 0x64, SANDBOX_DATA_CONSTRUCT),
        11 => new_data(e, 0x38, PATROL_DATA_CONSTRUCT),
        12 => new_data(e, 0x14, GUARD_DATA_CONSTRUCT),
        14 => new_data(e, 0x08, TYPE_16_DATA_CONSTRUCT),
        _ => 0,
    };
    Ptr::new(data)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008c6ca0, actor_package_data_create_actor_package_data(u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_the_data_object_of_each_package_type() {
        let cases = [
            (2u32, 0x18u32, TYPE_2_DATA_CONSTRUCT),
            (12, 0x64, SANDBOX_DATA_CONSTRUCT),
            (13, 0x38, PATROL_DATA_CONSTRUCT),
            (14, 0x14, GUARD_DATA_CONSTRUCT),
            (16, 0x08, TYPE_16_DATA_CONSTRUCT),
        ];
        for (kind, size, construct) in cases {
            let mut e = Engine::new();
            e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
            for (_, _, c) in cases {
                e.register(c, |_, a| (a[0] + 1).into_ret());
            }
            e.call_log = Some(vec![]);
            let data = e.call(0x008c_6ca0, &args![kind]).u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (OPERATOR_NEW, vec![size]));
            assert_eq!(log[2].0, construct);
            assert_eq!(data, log[2].1[0] + 1);
        }
    }

    #[test]
    fn other_types_and_failed_allocations_give_null() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        for kind in [0u32, 1, 3, 11, 15, 17, 0xffff_ffff] {
            assert_eq!(e.call(0x008c_6ca0, &args![kind]).u32(), 0);
        }
        assert_eq!(e.call_log.as_ref().unwrap().len(), 7);
        assert_eq!(e.call(0x008c_6ca0, &args![12u32]).u32(), 0);
    }
}
