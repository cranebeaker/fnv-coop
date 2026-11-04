//! `fallout shared/sky/run_0063fb70` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The sky's texture load request (`0063fb70`, which `Clouds::Update` uses for the cloud
//! transition textures) and the `Stars` constructor (`STARS_CONSTRUCT` in `sky.rs`; the
//! Xbox PDB's `Stars` is 0x10 bytes, as `Sky::Initialize` allocates).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_sky::skyobject::{pointer_assign, NI_POINTER_FROM_RAW};

/// `00af4200` (unit `archive.cpp`, `cdecl`: the path, a 0x104-byte buffer, its size).
const ARCHIVE_PATH_00AF4200: u32 = 0x00af_4200;
/// `operator new` (`00401000`, `cdecl`, size) and the request's constructor `0063fa10`
/// (`thiscall`: the `NiPointer` to fill, the path; 0x38 bytes).
const OPERATOR_NEW: u32 = 0x0040_1000;
const REQUEST_CONSTRUCT: u32 = 0x0063_fa10;
/// The pointer global (`01202d98`) whose object queues the request through its vtable slot
/// +0x48.
const QUEUE_01202D98: u32 = 0x0120_2d98;
/// `SkyObject::SkyObject` (`0063f830`) and `Stars`' vtable (`0104f1d8`).
const SKY_OBJECT_CONSTRUCT: u32 = 0x0063_f830;
const STARS_VTABLE: u32 = 0x0104_f1d8;

layout! {
    /// `Stars` (Xbox PDB), 0x10 bytes.
    pub struct Stars: 0x10 {
        /// `SkyObject::spRoot` (Xbox PDB).
        0x04 spRoot: u32,
        /// `spStars` (Xbox PDB): `NiPointer<NiNode>`.
        0x08 spStars: u32,
        /// `fAlpha` (Xbox PDB).
        0x0C fAlpha: f32,
    }
}

// Translated from 0063fb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: the `NiPointer` that receives the
/// texture, the path). With a path, `00af4200` writes it into a local 0x104-byte buffer
/// (which nothing reads afterwards). Makes a 0x38-byte request (`0063fa10` with the pointer
/// and the path; 0 when the allocation fails) and hands it to the object at `01202d98`
/// through its vtable slot +0x48.
// Translation incomplete: 0063fb70 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) and the /GS cookie check (00ec408c) are not translated.
pub fn fn_0063fb70(e: &mut Engine, slot: u32, path: u32) {
    e.with_stack(0x104, |e, buffer| {
        if path != 0 {
            e.call(ARCHIVE_PATH_00AF4200, &args![path, buffer.addr(), 0x104u32]);
        }
    });
    let block = e.call(OPERATOR_NEW, &args![0x38u32]).u32();
    let request = if block != 0 {
        e.call(REQUEST_CONSTRUCT, &args![block, slot, path]).u32()
    } else {
        0
    };
    let queue = e.global::<u32>(QUEUE_01202D98);
    e.vcall(queue, 0x48, &args![request]);
}

// Translated from 0063fc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Stars::Stars` (by the vtable it stores; `thiscall`, no stack words):
/// `SkyObject::SkyObject`, the vtable, `spStars` constructed from 0 and assigned 0, alpha 0.
/// Answers `this`.
// Translation incomplete: 0063fc70 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and spStars) is not translated.
pub fn fn_0063fc70(e: &mut Engine, this: Ptr<Stars>) -> Ptr<Stars> {
    let base = this.addr();
    e.call(SKY_OBJECT_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, STARS_VTABLE);
    e.call(NI_POINTER_FROM_RAW, &args![base + 8, 0u32]);
    pointer_assign(e, base + 8, 0);
    e.set(this, Stars::fAlpha, 0.0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_fb70, fn_0063fb70(u32, u32)),
        entry!(0x0063_fc70, fn_0063fc70(Ptr<Stars>) -> Ptr<Stars>),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::fallout_shared_sky::skyobject::test_support::*;

    #[test]
    fn texture_request_is_queued() {
        let mut e = Engine::new();
        quiet(&mut e, &[ARCHIVE_PATH_00AF4200, 0x0000_7b48]);
        e.register(OPERATOR_NEW, |_, _| answer(0x0000_3800));
        e.register(REQUEST_CONSTRUCT, |_, a| answer(a[0]));
        let queue = object_with(&mut e, 4, &[(0x48, 0x0000_7b48)]);
        e.map(QUEUE_01202D98, 4);
        e.set_global(QUEUE_01202D98, queue);
        e.call_log = Some(vec![]);
        e.call(0x0063_fb70, &args![0x1110u32, 0x2220u32]);
        let archive = calls(&e, ARCHIVE_PATH_00AF4200);
        assert_eq!(archive.len(), 1);
        assert_eq!(archive[0][0], 0x2220);
        assert_eq!(archive[0][2], 0x104);
        assert_eq!(
            calls(&e, REQUEST_CONSTRUCT),
            vec![vec![0x3800, 0x1110, 0x2220]]
        );
        assert_eq!(calls(&e, 0x0000_7b48), vec![vec![queue, 0x3800]]);
        // No path, no allocation: a null request is still queued.
        e.register(OPERATOR_NEW, |_, _| answer(0));
        e.call_log = Some(vec![]);
        e.call(0x0063_fb70, &args![0x1110u32, 0u32]);
        assert!(calls(&e, ARCHIVE_PATH_00AF4200).is_empty());
        assert!(calls(&e, REQUEST_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, 0x0000_7b48), vec![vec![queue, 0]]);
    }

    #[test]
    fn stars_constructor() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[SKY_OBJECT_CONSTRUCT]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        e.mem.set_f32(this + 0xc, 3.0);
        assert_eq!(e.call(0x0063_fc70, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), STARS_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.f32(this + 0xc), 0.0);
    }
}
