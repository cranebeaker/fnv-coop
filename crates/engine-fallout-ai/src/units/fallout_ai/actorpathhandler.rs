//! `fallout/ai/movement/actorpathhandler.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `ActorPathHandler` destructor body (`009dfed0`, unit `run_009dfe40`; `thiscall`, no
/// stack words).
const DESTRUCTOR_BODY: u32 = 0x009d_fed0;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009dfe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPathHandler::_scalar_deleting_destructor_` (engine map; `thiscall`, one stack word:
/// the flags, `RET 4`): the destructor body (`009dfed0`), then `this` freed (`00401030`) when
/// bit 0 of `flags` is set. Returns `this`.
pub fn actor_path_handler_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x009d_fe10,
        actor_path_handler_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_zero() {
        for (flags, frees) in [(1u32, 1usize), (0, 0), (2, 0), (3, 1)] {
            let mut e = Engine::new();
            e.register(DESTRUCTOR_BODY, |_, _| Ret::default());
            e.register(OPERATOR_DELETE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009d_fe10, &args![0x2000u32, flags]).u32(), 0x2000);
            let log = e.call_log.take().unwrap();
            let bodies: Vec<_> = log.iter().filter(|c| c.0 == DESTRUCTOR_BODY).collect();
            assert_eq!(bodies.len(), 1);
            assert_eq!(bodies[0].1, vec![0x2000]);
            let deletes: Vec<_> = log.iter().filter(|c| c.0 == OPERATOR_DELETE).collect();
            assert_eq!(deletes.len(), frees);
        }
    }
}
