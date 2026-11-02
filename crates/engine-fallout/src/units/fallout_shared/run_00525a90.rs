//! `fallout shared/run_00525a90` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00525a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the answer of `TESObjectWEAP::GetCurrentAmmo` (`00525980`, `ECX` = this, the argument
/// passed through) and, when it is not null, the answer of
/// `DialoguePackage::GetTargetOfConversation` (`008d80e0`, `ECX` = that object). Returns the
/// second answer when it is not null, otherwise the pointer at `+0x118` of `this`. The engine
/// map gives the function no name, so its class is unconfirmed.
pub fn fn_00525a90(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let first = e.call(0x0052_5980, &args![this, source]).u32();
    if first != 0 {
        let second = e.call(0x008d_80e0, &args![first]).u32();
        if second != 0 {
            return Ptr::new(second);
        }
    }
    Ptr::new(e.mem.u32(this.addr() + 0x118))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0052_5a90, fn_00525a90(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    /// Runs the function with `ammo` answering `00525980` and `target` answering `008d80e0`.
    fn run(ammo: u32, target: u32) -> (u32, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x120);
        e.mem.set_u32(this + 0x118, 0x7777);
        e.register_double(0x0052_5980, move |_, _| Ret {
            eax: ammo,
            ..Ret::default()
        });
        e.register_double(0x008d_80e0, move |_, _| Ret {
            eax: target,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(
            0x0052_5a90,
            &args![Ptr::<()>::new(this), Ptr::<()>::new(0x55)],
        );
        let log = e.call_log.take().unwrap();
        let calls = log[1..]
            .iter()
            .map(|(a, w)| (*a, w[1..].to_vec()))
            .collect();
        (result.u32(), calls)
    }

    #[test]
    fn fn_00525a90_prefers_the_conversation_target() {
        let (result, calls) = run(0x1234, 0x9999);
        assert_eq!(result, 0x9999);
        assert_eq!(calls, [(0x0052_5980, vec![0x55]), (0x008d_80e0, vec![])]);
    }

    #[test]
    fn fn_00525a90_falls_back_to_the_field_at_0x118() {
        // No target: the field.
        let (result, calls) = run(0x1234, 0);
        assert_eq!(result, 0x7777);
        assert_eq!(calls.len(), 2);
        // No ammo: the target getter is not called.
        let (result, calls) = run(0, 0x9999);
        assert_eq!(result, 0x7777);
        assert_eq!(calls, [(0x0052_5980, vec![0x55])]);
    }
}
