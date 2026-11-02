//! `(unplaced)/run_00a854c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSkinData`'s vtable (its RTTI names the class).
const SKIN_DATA_VTABLE: u32 = 0x0109_e574;
/// The base constructor `00a5d3a0` (thiscall; the engine map puts it in
/// `NiMain/niobject.cpp`).
const BASE_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// `NiSkinData::ComputeBoneBounds` (engine map), `00a853c0` (thiscall: the
/// vertices).
const COMPUTE_BONE_BOUNDS: u32 = 0x00a8_53c0;

// Translated from 00a854c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiSkinData` (the class its vtable's RTTI names; called
/// by `00687d90`): the base constructor `00a5d3a0`, the vtable, the skin
/// partition pointer (+8) cleared, the 0x34-byte root-parent-to-skin
/// transform copied from `transform` to +0x0C, `bones` at +0x44 and
/// `bone_data` at +0x40, then `NiSkinData::ComputeBoneBounds(vertices)`.
/// Returns `this`.
pub fn fn_00a854c0(
    e: &mut Engine,
    this: u32,
    bones: u32,
    bone_data: u32,
    transform: u32,
    vertices: u32,
) -> u32 {
    e.call(BASE_CONSTRUCTOR, &[this]);
    e.mem.set_u32(this, SKIN_DATA_VTABLE);
    e.mem.set_u32(this + 8, 0);
    for i in 0..13 {
        let word = e.mem.u32(transform + 4 * i);
        e.mem.set_u32(this + 0xc + 4 * i, word);
    }
    e.mem.set_u32(this + 0x44, bones);
    e.mem.set_u32(this + 0x40, bone_data);
    e.call(COMPUTE_BONE_BOUNDS, &[this, vertices]);
    this
}

// Translated from 00a85510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method taking eight words it does not read (thiscall, `RET 0x20`;
/// called by `00683c10`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00a85510(
    _e: &mut Engine,
    _this: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
) {
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_54c0, fn_00a854c0(u32, u32, u32, u32, u32) -> u32),
        entry!(
            0x00a8_5510,
            fn_00a85510(u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skin_data_takes_its_bones_and_transform() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 8, 0x2222);
            a[0].into_ret()
        });
        e.register(COMPUTE_BONE_BOUNDS, |_, _| 0u32.into_ret());
        let transform = e.mem.alloc(0x34);
        for i in 0..13 {
            e.mem.set_u32(transform + 4 * i, 0x100 + i);
        }
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a8_54c0, &[this, 5, 0x6000, transform, 0x7000])
                .u32(),
            this
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_CONSTRUCTOR, vec![this]));
        assert_eq!(log[2], (COMPUTE_BONE_BOUNDS, vec![this, 0x7000]));
        assert_eq!(e.mem.u32(this), SKIN_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x100);
        assert_eq!(e.mem.u32(this + 0x3c), 0x10c);
        assert_eq!(e.mem.u32(this + 0x40), 0x6000);
        assert_eq!(e.mem.u32(this + 0x44), 5);
    }

    #[test]
    fn empty_method_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00a8_5510, &[0x1000, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
