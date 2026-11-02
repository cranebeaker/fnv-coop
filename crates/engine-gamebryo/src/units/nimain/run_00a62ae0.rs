//! `NiMain/run_00a62ae0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRenderer::ms_pkRenderer` (the renderer global `011f4748`; Xbox PDB name of the static).
const RENDERER_INSTANCE: u32 = 0x011f_4748;
/// `NiRenderer`'s virtual table (RTTI `.?AVNiRenderer@@`).
const NI_RENDERER_VTABLE: u32 = 0x0109_bb94;
/// `NiRenderer` fields (Xbox PDB, the PC code's offsets): `m_spAccum` +0x08,
/// `m_spErrorShader` +0x14, the critical sections `m_kRendererLock` +0x80,
/// `m_kPrecacheCriticalSection` +0x100 and `m_kSourceDataCriticalSection` +0x180,
/// `m_eFrameState` +0x200, `m_uiFrameID` +0x204, `m_bRenderTargetGroupActive` +0x208 and
/// `m_bBatchRendering` +0x209.
const RENDERER_ACCUMULATOR: u32 = 0x08;
const RENDERER_ERROR_SHADER: u32 = 0x14;
const RENDERER_LOCKS: [u32; 3] = [0x80, 0x100, 0x180];
const RENDERER_FRAME_STATE: u32 = 0x200;
const RENDERER_FRAME_ID: u32 = 0x204;
const RENDERER_TARGET_GROUP_ACTIVE: u32 = 0x208;
const RENDERER_BATCH_RENDERING: u32 = 0x209;
/// `00a5d3a0`: the `NiObject` constructor body (`__thiscall`).
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// The `InitializeCriticalSection` import slot (stdcall: the section).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;

// Translated from 00a62ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no words; called from `00a8a5b9`): clears the renderer global
/// (`NiRenderer::ms_pkRenderer`, `011f4748`).
pub fn fn_00a62ae0(e: &mut Engine) {
    e.mem.set_u32(RENDERER_INSTANCE, 0);
}

// Translated from 00a62af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiRenderer` constructor (unnamed in the map; it stores `NiRenderer`'s virtual table):
/// the `NiObject` constructor, the virtual table, no accumulator or error shader, its three
/// critical sections initialised (`InitializeCriticalSection` on +0x80, +0x100, +0x180), the
/// frame state, frame id and both flags cleared, and the renderer global set to `this`.
/// Answers `this`.
pub fn fn_00a62af0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, NI_RENDERER_VTABLE);
    e.mem.set_u32(base + RENDERER_ACCUMULATOR, 0);
    e.mem.set_u32(base + RENDERER_ERROR_SHADER, 0);
    for offset in RENDERER_LOCKS {
        e.call(INITIALIZE_CRITICAL_SECTION, &args![base + offset]);
    }
    e.mem.set_u32(base + RENDERER_FRAME_STATE, 0);
    e.mem.set_u32(base + RENDERER_FRAME_ID, 0);
    e.mem.set_u8(base + RENDERER_TARGET_GROUP_ACTIVE, 0);
    e.mem.set_u8(base + RENDERER_BATCH_RENDERING, 0);
    e.mem.set_u32(RENDERER_INSTANCE, base);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a62ae0, fn_00a62ae0()),
        entry!(0x00a62af0, fn_00a62af0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00a62af0_builds_the_renderer_and_fn_00a62ae0_forgets_it() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        e.map(RENDERER_INSTANCE, 4);
        let r = e.mem.alloc(0x280);
        for off in (0..0x280).step_by(4) {
            e.mem.set_u32(r + off, 0x3333_3333);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a62af0, &args![r]).u32(), r);
        assert_eq!(calls_to(&e, NI_OBJECT_CONSTRUCTOR), vec![vec![r]]);
        assert_eq!(
            calls_to(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![r + 0x80], vec![r + 0x100], vec![r + 0x180]]
        );
        assert_eq!(e.mem.u32(r), NI_RENDERER_VTABLE);
        for off in [0x08, 0x14, 0x200, 0x204] {
            assert_eq!(e.mem.u32(r + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(r + 0x208), 0x3333_0000);
        assert_eq!(e.mem.u32(r + 0x0c), 0x3333_3333);
        assert_eq!(e.mem.u32(RENDERER_INSTANCE), r);
        e.call(0x00a62ae0, &args![]);
        assert_eq!(e.mem.u32(RENDERER_INSTANCE), 0);
    }
}
