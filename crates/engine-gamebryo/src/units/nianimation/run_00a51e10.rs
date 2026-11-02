//! `NiAnimation/run_00a51e10` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00a51e10` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv),
//! read from its bytes: the object factory of `NiBSplineData` (the RTTI name of the vtable
//! `01099e54` it stores).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nianimation::nibsplinedata::NiBSplineData;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The base constructor `00a5d3a0` (unit `niobject.cpp`, unnamed in the engine map;
/// `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiBSplineData`'s vtable (`01099e54`, RTTI `.?AVNiBSplineData@@`).
const VTABLE: u32 = 0x0109_9e54;

// Translated from 00a51e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, no arguments): allocates 0x18 bytes, and when the
/// allocation is not null runs the base constructor `00a5d3a0` on it, clears
/// `m_pafControlPoints`, `m_pasCompactControlPoints`, `m_uiControlPointCount` and
/// `m_uiCompactControlPointCount` and stores `NiBSplineData`'s vtable. Answers the object, or
/// 0 when the allocation failed.
pub fn fn_00a51e10(e: &mut Engine) -> Ptr<NiBSplineData> {
    let object = Ptr::<NiBSplineData>::new(e.call(NI_NEW, &args![0x18u32]).u32());
    if !object.is_null() {
        e.call(BASE_CONSTRUCT, &args![object]);
        e.set(object, NiBSplineData::m_pafControlPoints, 0);
        e.set(object, NiBSplineData::m_pasCompactControlPoints, 0);
        e.set(object, NiBSplineData::m_uiControlPointCount, 0);
        e.set(object, NiBSplineData::m_uiCompactControlPointCount, 0);
        e.mem.set_u32(object.addr(), VTABLE);
    }
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_1e10, fn_00a51e10() -> Ptr<NiBSplineData>)]
}

#[cfg(test)]
mod tests {
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
    fn fn_00a51e10_builds_the_spline_data_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0x18]);
            block.into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00a5_1e10, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x18]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0xcdcd_cdcd);
        for offset in [0x08u32, 0x0c, 0x10, 0x14] {
            assert_eq!(e.mem.u32(object + offset), 0, "{offset:#x}");
        }

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_1e10, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }
}
