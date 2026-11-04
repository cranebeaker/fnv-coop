//! `fallout shared/run_0066e910` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The registered `ActorValueInfo*` of each of the 0x4D actor values
/// (filled by `ActorValue::RegisterActorValues`).
const ACTOR_VALUE_INFOS: u32 = 0x011d_61c8;

// Translated from 0066e920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The info of actor value `index` (`cdecl`; no Xbox PDB name): the entry
/// of the table at `011d61c8` for `0 <= index < 0x4d` (signed), 0 for any
/// other index.
pub fn fn_0066e920(e: &mut Engine, index: i32) -> u32 {
    if (0..0x4d).contains(&index) {
        e.mem.u32(ACTOR_VALUE_INFOS + 4 * index as u32)
    } else {
        0
    }
}

// Translated from 0066e910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00470470` on the object at `011d6124` (`cdecl`, no arguments; the
/// engine map gives it no name).
pub fn fn_0066e910(e: &mut Engine) {
    e.call(0x0047_0470, &args![0x011d_6124u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066e920, fn_0066e920(i32) -> u32),
        entry!(0x0066_e910, fn_0066e910()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_table_only_inside_its_bounds() {
        let mut e = Engine::new();
        e.map(0x011d_6000, 0x1000);
        e.set_global(ACTOR_VALUE_INFOS, 0x1234u32);
        e.set_global(ACTOR_VALUE_INFOS + 4 * 0x4c, 0x5678u32);
        e.set_global(ACTOR_VALUE_INFOS + 4 * 0x4d, 0x9999u32);
        assert_eq!(e.call(0x0066_e920, &args![0u32]).u32(), 0x1234);
        assert_eq!(e.call(0x0066_e920, &args![0x4cu32]).u32(), 0x5678);
        assert_eq!(e.call(0x0066_e920, &args![0x4du32]).u32(), 0);
        assert_eq!(e.call(0x0066_e920, &args![0xffff_ffffu32]).u32(), 0);
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn runs_the_callee_on_the_global() {
        let mut e = Engine::new();
        e.register(0x0047_0470, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0066_e910, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0047_0470, vec![0x011d_6124])]);
    }
}
