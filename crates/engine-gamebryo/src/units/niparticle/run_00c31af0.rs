//! `NiParticle/run_00c31af0` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c21f60`: the jump target of `00c31b10`.
const JUMP_TARGET_00C31B10: u32 = 0x00c2_1f60;

// Translated from 00c31b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00c21f60`, passing on `this` and its two arguments. Return type (void): the exe's only callers of it are `CopyMembers`/`CreateClone` bodies (xrefs), it takes (this, destination, cloning process) and every `CopyMembers` in the Xbox PDB returns `void`; the class is unnamed in the engine map.
pub fn fn_00c31b10(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00C31B10, &args![this, first, second]);
}

/// The `NiPSysEmitter` constructor (`00c21f00`, `thiscall`).
const EMITTER_CONSTRUCTOR: u32 = 0x00c2_1f00;
/// The vtable of `NiPSysVolumeEmitter` (RTTI `.?AVNiPSysVolumeEmitter@@`).
const VOLUME_EMITTER_VTABLE: u32 = 0x010c_0a8c;

// Translated from 00c31af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysVolumeEmitter` constructor (by the
/// vtable it stores, `010c0a8c`): the `NiPSysEmitter` constructor, the
/// vtable and no emitter object (`m_pkEmitterObj`, +0x54). Returns `this`.
pub fn fn_00c31af0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(EMITTER_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VOLUME_EMITTER_VTABLE);
    e.mem.set_u32(this.addr() + 0x54, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_1af0, fn_00c31af0(Ptr) -> Ptr),
        entry!(0x00c3_1b10, fn_00c31b10(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c31b10_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c3_1b10;
        const TARGET: u32 = 0x00c2_1f60;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }

    #[test]
    fn volume_emitter_constructor_sets_the_vtable_and_no_object() {
        let mut e = Engine::new();
        e.register(EMITTER_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x58);
        e.mem.set_u32(this + 0x54, 0x99);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_1af0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == EMITTER_CONSTRUCTOR && c.1 == vec![this]));
        assert_eq!(e.mem.u32(this), VOLUME_EMITTER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x54), 0);
    }
}
