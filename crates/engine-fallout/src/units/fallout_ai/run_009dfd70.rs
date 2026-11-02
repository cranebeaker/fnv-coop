//! `fallout/ai/run_009dfd70` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ActorPathHandler`'s vtable (`0109231c`; RTTI `.?AVActorPathHandler@@`).
const VTABLE_ACTOR_PATH_HANDLER: u32 = 0x0109_231c;
/// `00633c90`: an `NiPointer` constructor (`thiscall`, the object): stores
/// it and adds a reference when it is not null.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;

layout! {
    /// `ActorPathHandler` (Xbox PDB), 0x14 bytes; the PC constructor writes
    /// the same offsets.
    pub struct ActorPathHandler: 0x14 {
        /// `pActor` (Xbox PDB): `Actor *`.
        0x04 pActor: u32,
        /// `pActorMover` (Xbox PDB): `ActorMover *`.
        0x08 pActorMover: u32,
        /// `spPath` (Xbox PDB): `NiPointer<PathingSolution>`.
        0x0C spPath: u32,
        /// `spRequest` (Xbox PDB): `NiPointer<PathingRequest>`.
        0x10 spRequest: u32,
    }
}

// Translated from 009dfd70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dfd70 - the C++ exception frame (FS:[0]) is not translated
/// `ActorPathHandler::ActorPathHandler` (constructor, named by the vtable it
/// sets): stores the actor and the mover and constructs both `NiPointer`s
/// empty. Returns `this`.
pub fn actor_path_handler_construct(
    e: &mut Engine,
    this: Ptr<ActorPathHandler>,
    actor: u32,
    mover: u32,
) -> Ptr<ActorPathHandler> {
    e.mem.set_u32(this.addr(), VTABLE_ACTOR_PATH_HANDLER);
    e.set(this, ActorPathHandler::pActor, actor);
    e.set(this, ActorPathHandler::pActorMover, mover);
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 0x0c, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 0x10, 0u32]);
    this
}

// Translated from 009dfdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPathHandler::NoPathUpdate` (Xbox PDB vtable slot `+0x10`; the
/// `ActorPathHandler` vtable holds this body there): returns 5 and reads
/// nothing.
pub fn actor_path_handler_no_path_update(
    _e: &mut Engine,
    _this: Ptr<ActorPathHandler>,
    _unused_1: u32,
) -> u32 {
    5
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009d_fd70,
            actor_path_handler_construct(Ptr<ActorPathHandler>, u32, u32) -> Ptr<ActorPathHandler>
        ),
        entry!(
            0x009d_fdf0,
            actor_path_handler_no_path_update(Ptr<ActorPathHandler>, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_sets_vtable_fields_and_empty_pointers() {
        let mut e = Engine::new();
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = e.new_object::<ActorPathHandler>();
        e.mem.set_u32(this.addr() + 0x0c, 0xffff);
        e.mem.set_u32(this.addr() + 0x10, 0xffff);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x009d_fd70, &args![this, 0x1111u32, 0x2222u32])
            .u32();
        assert_eq!(r, this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ACTOR_PATH_HANDLER);
        assert_eq!(e.get(this, ActorPathHandler::pActor), 0x1111);
        assert_eq!(e.get(this, ActorPathHandler::pActorMover), 0x2222);
        assert_eq!(e.get(this, ActorPathHandler::spPath), 0);
        assert_eq!(e.get(this, ActorPathHandler::spRequest), 0);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == NI_POINTER_CONSTRUCT).collect();
        assert_eq!(inner[0].1, vec![this.addr() + 0x0c, 0]);
        assert_eq!(inner[1].1, vec![this.addr() + 0x10, 0]);
    }

    #[test]
    fn no_path_update_returns_5() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009d_fdf0, &args![0x4000u32, 7u32]).u32(), 5);
    }
}
