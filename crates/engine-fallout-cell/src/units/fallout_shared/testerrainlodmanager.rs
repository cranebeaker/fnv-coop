//! `fallout shared/testerrainlodmanager.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Scope guard constructor / destructor (`this`, tag, 1, source file, line).
const SCOPE_GUARD_CTOR: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DTOR: u32 = 0x0040_4ee0;
/// `"D:\\_Fallout3\\Platforms\\Common\\Code\\Fallout Shared\\TESTerrainLODManager.cpp"`.
const SOURCE_FILE: u32 = 0x0103_10d8;
/// Setting getters: pointer to a setting's `float` value (`00403e20`) and to its `int` value
/// (`0043d4d0`), both `thiscall` on the setting.
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// The three `float` settings the grass distance is made of (`011ca450`, `011ca4ac`,
/// `011ca498`) and the `int` setting that caps it (`011c63cc`, shifted left by 12).
const SETTING_011CA450: u32 = 0x011c_a450;
const SETTING_011CA4AC: u32 = 0x011c_a4ac;
const SETTING_011CA498: u32 = 0x011c_a498;
const SETTING_011C63CC: u32 = 0x011c_63cc;
/// The `double` added to the second setting before the comparison (`01011588`).
const MARGIN_01011588: u32 = 0x0101_1588;
/// The globals the function sets: the grass distance (`float`), the grass node, the done
/// flag.
const GRASS_DISTANCE: u32 = 0x011c_a434;
const GRASS_NODE: u32 = 0x011c_a438;
const GRASS_READY: u32 = 0x011c_a430;
/// `00450b80(0)` (`cdecl`): the object the node is attached to through its vtable `+0xf8`.
const ATTACH_TARGET: u32 = 0x0045_0b80;
/// The allocation `00aa13e0` (`cdecl`, size) and `NiNode`'s constructor (`00a5ecb0`).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `0057d090` (no arguments): the value `00a5b950` is called with on the node.
const NODE_ARGUMENT: u32 = 0x0057_d090;
const NODE_SET: u32 = 0x00a5_b950;
/// `NiAVObject::UpdateProperties` (engine map, `00a5a040`).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;

// Translated from 0057cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTerrainLODManager::InitGrass` (Xbox PDB), no arguments. Under a scope guard (tag 0x1c,
/// line 0x90): the grass distance is the sum of the `float` settings `011ca450` and `011ca498`
/// when `011ca450` is at least `011ca4ac` plus the `double` at `01011588` (0 otherwise, and
/// when either is a NaN), capped by the `int` setting `011c63cc` shifted left by 12 (as an
/// unsigned 32-bit value) when it is larger; stored at `011ca434`. Then makes the grass node
/// (an `NiNode` of 0xac bytes with no children, null when the allocation fails) at
/// `011ca438`, calls `00a5b950` on it with `0057d090`'s value, attaches it to `00450b80(0)`
/// through that object's vtable `+0xf8` (with 6), updates its properties and sets the byte at
/// `011ca430`.
// Translation incomplete: 0057cf00 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) is not translated.
pub fn tes_terrain_lod_manager_init_grass(e: &mut Engine) {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0x1cu32, 1u32, SOURCE_FILE, 0x90u32],
        );
        let mut distance = 0.0f32.to_bits();
        let first = e
            .call(SETTING_FLOAT_POINTER, &args![SETTING_011CA450])
            .u32();
        let second = e
            .call(SETTING_FLOAT_POINTER, &args![SETTING_011CA4AC])
            .u32();
        let margin: f64 = e.global(MARGIN_01011588);
        let threshold = x87_add(x87_load32(e.mem.u32(second)), margin);
        let value = x87_load32(e.mem.u32(first));
        // `FCOMPP; TEST AH,1; JNZ`: skipped when below or unordered.
        if value >= threshold {
            let first = e
                .call(SETTING_FLOAT_POINTER, &args![SETTING_011CA450])
                .u32();
            let third = e
                .call(SETTING_FLOAT_POINTER, &args![SETTING_011CA498])
                .u32();
            let sum = x87_add(x87_load32(e.mem.u32(first)), x87_m32(e.mem.u32(third)));
            distance = x87_store32(sum);
        }
        let cap = e.call(SETTING_INT_POINTER, &args![SETTING_011C63CC]).u32();
        let cap = f64::from(e.mem.u32(cap).wrapping_shl(12));
        // `FCOMPP; TEST AH,0x41; JNZ`: replaced only when strictly above (ordered).
        if x87_load32(distance) > cap {
            let cap = e.call(SETTING_INT_POINTER, &args![SETTING_011C63CC]).u32();
            let cap = f64::from(e.mem.u32(cap).wrapping_shl(12));
            distance = x87_store32(cap);
        }
        e.set_global(GRASS_DISTANCE, x87_quiet_bits(distance));

        let target = e.call(ATTACH_TARGET, &args![0u32]).u32();
        let memory = e.call(NEW_OBJECT, &args![0xacu32]).u32();
        let node = if memory != 0 {
            e.call(NI_NODE_CONSTRUCT, &args![memory, 0u32]).u32()
        } else {
            0
        };
        e.set_global(GRASS_NODE, node);
        let argument = e.call(NODE_ARGUMENT, &args![]).u32();
        let node = e.global::<u32>(GRASS_NODE);
        e.call(NODE_SET, &args![node, argument]);
        let node = e.global::<u32>(GRASS_NODE);
        e.vcall(target, 0xf8, &args![6u32, node]);
        let node = e.global::<u32>(GRASS_NODE);
        e.call(UPDATE_PROPERTIES, &args![node]);
        e.set_global(GRASS_READY, 1u8);
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0057_cf00, tes_terrain_lod_manager_init_grass())]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_VTABLE: u32 = 0x0f00_1000;
    const ATTACH: u32 = 0x0f00_2000;

    /// Settings keep their value at `+4`; the getters answer that address.
    fn rig(first: f32, second: f32, third: f32, cap: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x011c_6000, 0x011c_a000] {
            e.map(page, 0x1000);
        }
        e.set_global(MARGIN_01011588, 0.5f64);
        e.set_global(SETTING_011CA450 + 4, first);
        e.set_global(SETTING_011CA4AC + 4, second);
        e.set_global(SETTING_011CA498 + 4, third);
        e.set_global(SETTING_011C63CC + 4, cap);
        e.register(SETTING_FLOAT_POINTER, |_, a| (a[0] + 4).into_ret());
        e.register(SETTING_INT_POINTER, |_, a| (a[0] + 4).into_ret());
        e.register(SCOPE_GUARD_CTOR, |_, a| a[0].into_ret());
        e.register(SCOPE_GUARD_DTOR, |_, _| Ret::default());
        let target = e.mem.alloc(8);
        let mut slots = vec![0u32; 0x40];
        slots[0xf8 / 4] = ATTACH;
        e.put_vtable(TARGET_VTABLE, &slots);
        e.mem.set_u32(target, TARGET_VTABLE);
        e.register(ATTACH, |_, _| Ret::default());
        e.register_double(ATTACH_TARGET, move |_, _| target.into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NODE_ARGUMENT, |_, _| 0x0555_0000u32.into_ret());
        e.register(NODE_SET, |_, _| Ret::default());
        e.register(UPDATE_PROPERTIES, |_, _| Ret::default());
        (e, target)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn init_grass_sums_the_settings_and_makes_the_node() {
        let (mut e, target) = rig(4000.0, 3000.0, 1000.0, 10);
        e.call_log = Some(vec![]);
        e.call(0x0057_cf00, &args![]);
        assert_eq!(e.global::<f32>(GRASS_DISTANCE), 5000.0);
        let node = e.global::<u32>(GRASS_NODE);
        assert_ne!(node, 0);
        assert_eq!(calls(&e, NEW_OBJECT), vec![vec![0xac]]);
        assert_eq!(calls(&e, NI_NODE_CONSTRUCT), vec![vec![node, 0]]);
        assert_eq!(calls(&e, NODE_SET), vec![vec![node, 0x0555_0000]]);
        assert_eq!(calls(&e, ATTACH), vec![vec![target, 6, node]]);
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![node]]);
        assert_eq!(calls(&e, ATTACH_TARGET), vec![vec![0]]);
        assert_eq!(e.global::<u8>(GRASS_READY), 1);
        let guard = calls(&e, SCOPE_GUARD_CTOR);
        assert_eq!(&guard[0][1..], &[0x1c, 1, SOURCE_FILE, 0x90]);
        assert_eq!(calls(&e, SCOPE_GUARD_DTOR), vec![vec![guard[0][0]]]);
    }

    #[test]
    fn init_grass_distance_is_zero_below_the_threshold_or_for_a_nan() {
        // 3000 < 3000 + 0.5: zero.
        let (mut e, _) = rig(3000.0, 3000.0, 1000.0, 10);
        e.call(0x0057_cf00, &args![]);
        assert_eq!(e.global::<u32>(GRASS_DISTANCE), 0);
        let (mut e, _) = rig(f32::NAN, 3000.0, 1000.0, 10);
        e.call(0x0057_cf00, &args![]);
        assert_eq!(e.global::<u32>(GRASS_DISTANCE), 0);
        // Exactly at the threshold: summed.
        let (mut e, _) = rig(3000.5, 3000.0, 1.0, 10);
        e.call(0x0057_cf00, &args![]);
        assert_eq!(e.global::<f32>(GRASS_DISTANCE), 3001.5);
    }

    #[test]
    fn init_grass_caps_the_distance_by_the_int_setting() {
        // 1 << 12 = 4096 < 5000.
        let (mut e, _) = rig(4000.0, 3000.0, 1000.0, 1);
        e.call(0x0057_cf00, &args![]);
        assert_eq!(e.global::<f32>(GRASS_DISTANCE), 4096.0);
        // The shift is 32-bit: 0x100001 << 12 wraps to 0x1000 (4096).
        let (mut e, _) = rig(4000.0, 3000.0, 1000.0, 0x0010_0001);
        e.call(0x0057_cf00, &args![]);
        assert_eq!(e.global::<f32>(GRASS_DISTANCE), 4096.0);
        // Read as unsigned: 0x80000 << 12 is 2^31, not negative: no cap.
        let (mut e, _) = rig(4000.0, 3000.0, 1000.0, 0x0008_0000);
        e.call(0x0057_cf00, &args![]);
        assert_eq!(e.global::<f32>(GRASS_DISTANCE), 5000.0);
        // A NaN sum is kept (unordered: not replaced), quiet.
        let (mut e, _) = rig(f32::INFINITY, 3000.0, f32::NEG_INFINITY, 1);
        e.call(0x0057_cf00, &args![]);
        assert!(e.global::<f32>(GRASS_DISTANCE).is_nan());
    }

    #[test]
    fn init_grass_stores_a_null_node_when_the_allocation_fails() {
        let (mut e, target) = rig(4000.0, 3000.0, 1000.0, 10);
        e.register(NEW_OBJECT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0057_cf00, &args![]);
        assert_eq!(e.global::<u32>(GRASS_NODE), 0);
        assert!(calls(&e, NI_NODE_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, ATTACH), vec![vec![target, 6, 0]]);
    }
}
