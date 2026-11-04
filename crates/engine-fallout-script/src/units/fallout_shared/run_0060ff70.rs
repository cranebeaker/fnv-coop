//! `fallout shared/run_0060ff70` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::tesquesttarget::TESQuestTarget;

/// The `TESCondition` constructor (`00680890`).
const CONDITION_CONSTRUCT: u32 = 0x0068_0890;
/// `TeleportPath::TeleportPath` (`006f48b0`).
const TELEPORT_PATH_CONSTRUCT: u32 = 0x006f_48b0;

// Translated from 0060ff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `TESQuestTarget` (0x48 bytes; `TESQuest`'s loader
/// allocates 0x48 for a `QSTA` chunk and calls it): constructs the
/// conditions (+4) and the target path (+0x10), clears the target
/// reference (+0xc) and the flags byte (+0). Returns `this`.
// Translation incomplete: 0060ff70 - SEH exception unwinding (the frame around the member constructors) is not translated.
pub fn fn_0060ff70(e: &mut Engine, this: Ptr<TESQuestTarget>) -> Ptr<TESQuestTarget> {
    e.call(CONDITION_CONSTRUCT, &args![this.addr() + 4]);
    e.call(TELEPORT_PATH_CONSTRUCT, &args![this.addr() + 0x10]);
    e.set(this, TESQuestTarget::m_pTargetRef, Ptr::NULL);
    e.set(this, TESQuestTarget::cFlags, 0);
    this
}

/// `TeleportPath::~TeleportPath` (`006f4930`, `thiscall`).
const TELEPORT_PATH_DESTRUCT: u32 = 0x006f_4930;
/// The `TESCondition` destructor (`006808b0`, `thiscall`).
const CONDITION_DESTRUCT: u32 = 0x0068_08b0;

// Translated from 0060ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060ffe0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00efb33b, state variable) is not translated
/// Destructor of a `TESQuestTarget` (`thiscall`, no stack word; the counterpart of
/// [`fn_0060ff70`]; Ghidra matches the body to a library destructor pattern): destroys the
/// target path (+0x10, `006f4930`) and then the conditions (+4, `006808b0`).
pub fn fn_0060ffe0(e: &mut Engine, this: Ptr<TESQuestTarget>) {
    e.call(TELEPORT_PATH_DESTRUCT, &args![this.addr() + 0x10]);
    e.call(CONDITION_DESTRUCT, &args![this.addr() + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0060ff70,
            fn_0060ff70(Ptr<TESQuestTarget>) -> Ptr<TESQuestTarget>
        ),
        entry!(0x0060_ffe0, fn_0060ffe0(Ptr<TESQuestTarget>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_constructor_builds_members_and_clears_fields() {
        let mut e = Engine::new();
        e.register(CONDITION_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(TELEPORT_PATH_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let target: Ptr<TESQuestTarget> = e.new_object();
        e.mem.set_u8(target.addr(), 0xff);
        e.mem.set_u32(target.addr() + 0xc, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(fn_0060ff70(&mut e, target), target);
        assert_eq!(e.get(target, TESQuestTarget::cFlags), 0);
        assert_eq!(e.get(target, TESQuestTarget::m_pTargetRef), Ptr::NULL);
        let calls: Vec<(u32, Vec<u32>)> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| [CONDITION_CONSTRUCT, TELEPORT_PATH_CONSTRUCT].contains(a))
            .collect();
        assert_eq!(
            calls,
            vec![
                (CONDITION_CONSTRUCT, vec![target.addr() + 4]),
                (TELEPORT_PATH_CONSTRUCT, vec![target.addr() + 0x10]),
            ]
        );
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn target_destructor_destroys_the_path_then_the_conditions() {
        let mut e = Engine::new();
        e.register(TELEPORT_PATH_DESTRUCT, |_, _| Ret::default());
        e.register(CONDITION_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x0060_ffe0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (TELEPORT_PATH_DESTRUCT, vec![this + 0x10]),
                (CONDITION_DESTRUCT, vec![this + 4]),
            ]
        );
    }
}
