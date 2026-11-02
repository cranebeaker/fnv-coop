//! `fallout/ai/run_009dfe40` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `ActorPathHandler` vtable.
const ACTOR_PATH_HANDLER_VTABLE: u32 = 0x0109_231c;

// Translated from 009dfe40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dfe40 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `ActorPathHandler` constructor (Xbox PDB class; no name in the
/// engine map): the vtable, `pActor` (+4), `pActorMover` (+8), and the two
/// smart pointers `spPath` (+0xc, `00559a40`) and `spRequest` (+0x10,
/// `00633c90`) set from their arguments. Returns `this`.
pub fn fn_009dfe40(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    mover: u32,
    path: u32,
    request: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, ACTOR_PATH_HANDLER_VTABLE);
    e.mem.set_u32(base + 4, actor);
    e.mem.set_u32(base + 8, mover);
    e.call(0x0055_9a40, &args![base + 0xc, path]);
    e.call(0x0063_3c90, &args![base + 0x10, request]);
    this
}

// Translated from 009dfed0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dfed0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `ActorPathHandler` destructor body: the vtable, then the smart
/// pointers `spRequest` (+0x10) and `spPath` (+0xc) released
/// (`0045cec0`).
pub fn fn_009dfed0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, ACTOR_PATH_HANDLER_VTABLE);
    e.call(0x0045_cec0, &args![base + 0x10]);
    e.call(0x0045_cec0, &args![base + 0xc]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d_fe40, fn_009dfe40(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x009d_fed0, fn_009dfed0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, callees: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| callees.contains(address))
            .cloned()
            .collect()
    }

    #[test]
    fn constructor_stores_the_parts() {
        let mut e = Engine::new();
        e.register(0x0055_9a40, |_, _| Ret::default());
        e.register(0x0063_3c90, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x009d_fe40,
            &args![this, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), ACTOR_PATH_HANDLER_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x11);
        assert_eq!(e.mem.u32(this + 8), 0x22);
        assert_eq!(
            calls(&e, &[0x0055_9a40, 0x0063_3c90]),
            vec![
                (0x0055_9a40, vec![this + 0xc, 0x33]),
                (0x0063_3c90, vec![this + 0x10, 0x44]),
            ]
        );
    }

    #[test]
    fn destructor_body_releases_request_then_path() {
        let mut e = Engine::new();
        e.register(0x0045_cec0, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x009d_fed0, &args![this]);
        assert_eq!(e.mem.u32(this), ACTOR_PATH_HANDLER_VTABLE);
        assert_eq!(
            calls(&e, &[0x0045_cec0]),
            vec![
                (0x0045_cec0, vec![this + 0x10]),
                (0x0045_cec0, vec![this + 0xc])
            ]
        );
    }
}
