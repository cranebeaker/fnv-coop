//! `fallout/ai/combat/combatprocedure.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A combat procedure keeps its combat controller at `+4` (every accessor below reads it
//! there) and a saved word at `+8`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00865e50` (save buffer, `RET 0xc`: data, size, 0) and `00864980` (load buffer, `RET 8`:
/// data, size).
const SAVE_DATA: u32 = 0x0086_5e50;
const LOAD_DATA: u32 = 0x0086_4980;
/// `CombatController::GetLookAtTargetIndex` (engine-map name of `00981690`; `thiscall`, `RET 4`:
/// the procedure).
const LOOK_AT_INDEX: u32 = 0x0098_1690;
/// `operator new` (`00401000`, `cdecl(size)`) and the debug print `005b5e40` (`cdecl`), with
/// `"COMBAT: Could not create combat procedure of type %i"` (`0108ddac`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const DEBUG_PRINT: u32 = 0x005b_5e40;
const UNKNOWN_KIND_FORMAT: u32 = 0x0108_ddac;

/// The procedure's combat controller (`+4`).
fn controller_of(e: &Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(4))
}

// Translated from 00996de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedure::GetAttacker` (Xbox PDB; `thiscall`, plain `RET`): `0097ae90` on the
/// controller, its `EAX` passed on.
pub fn combat_procedure_get_attacker(e: &mut Engine, this: Ptr) -> u32 {
    let controller = controller_of(e, this);
    e.call(0x0097_ae90, &args![controller]).u32()
}

// Translated from 00996e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedure::GetTarget` (Xbox PDB; `thiscall`, plain `RET`): `004030b0` on the
/// controller, its `EAX` passed on.
pub fn combat_procedure_get_target(e: &mut Engine, this: Ptr) -> u32 {
    let controller = controller_of(e, this);
    e.call(0x0040_30b0, &args![controller]).u32()
}

// Translated from 00996e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedure::SetMoveMode` (Xbox PDB; `thiscall`, `RET 4`): the controller's
/// `SetMoveMode` (`00981570`) with `mode`.
pub fn combat_procedure_set_move_mode(e: &mut Engine, this: Ptr, mode: u32) {
    let controller = controller_of(e, this);
    e.call(0x0098_1570, &args![controller, mode]);
}

// Translated from 00996e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedure::ClearMoveMode` (Xbox PDB; `thiscall`, plain `RET`): the controller's
/// `ClearMoveMode` (`00981660`).
pub fn combat_procedure_clear_move_mode(e: &mut Engine, this: Ptr) {
    let controller = controller_of(e, this);
    e.call(0x0098_1660, &args![controller]);
}

// Translated from 00996e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedure::SetLookAtTarget` (Xbox PDB; `thiscall`, `RET 8`): the controller's
/// `SetLookAtTarget` (`009816d0`) with the procedure's look-at index (`00981690(this)`),
/// `target` and the byte `priority`.
pub fn combat_procedure_set_look_at_target(e: &mut Engine, this: Ptr, target: Ptr, priority: u8) {
    let controller = controller_of(e, this);
    let index = e.call(LOOK_AT_INDEX, &args![controller, this]).u32();
    let controller = controller_of(e, this);
    e.call(
        0x0098_16d0,
        &args![controller, index, target, u32::from(priority)],
    );
}

// Translated from 00996ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedure::ClearLookAtTarget` (Xbox PDB; `thiscall`, plain `RET`): the controller's
/// `ClearLookAtTarget` (`00981720`) with the procedure's look-at index (`00981690(this)`).
pub fn combat_procedure_clear_look_at_target(e: &mut Engine, this: Ptr) {
    let controller = controller_of(e, this);
    let index = e.call(LOOK_AT_INDEX, &args![controller, this]).u32();
    let controller = controller_of(e, this);
    e.call(0x0098_1720, &args![controller, index]);
}

// Translated from 00996ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedure::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): the 4 bytes at `+8` into the
/// save buffer (`00865e50`).
pub fn combat_procedure_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(SAVE_DATA, &args![buffer, this.byte_add(8), 4u32, 0u32]);
}

// Translated from 00996ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedure::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): the 4 bytes at `+8` from the
/// load buffer (`00864980`).
pub fn combat_procedure_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(LOAD_DATA, &args![buffer, this.byte_add(8), 4u32]);
}

/// How each procedure kind is made (`CreateProcedure`'s jump table at `00997360`, as arms):
/// the block size, its constructor and the words passed after `this` (floats as bits).
fn procedure_recipe(kind: u32) -> Option<(u32, u32, Vec<u32>)> {
    let zero = 0.0f32.to_bits();
    let recipe = match kind {
        0 => (0x48, 0x009d_0890, vec![]),
        1 => (0x88, 0x009c_c0c0, vec![]),
        2 => (0x3c, 0x009c_adf0, vec![]),
        3 => (0x20, 0x009c_bad0, vec![]),
        4 => (0x18, 0x009d_5ad0, vec![0]),
        5 => (0x18, 0x009d_a720, vec![0]),
        6 => (0x7c, 0x009d_69f0, vec![0, zero, 0]),
        7 => (0x6c, 0x009d_2440, vec![0]),
        8 => (0x1c, 0x009c_a5f0, vec![0, 0]),
        9 => (0x40, 0x009d_6030, vec![0, zero, 0, 0]),
        10 => (0x40, 0x009d_8b60, vec![]),
        11 => (0x18, 0x009d_aa10, vec![0]),
        12 => (0x64, 0x009d_3a00, vec![]),
        _ => return None,
    };
    Some(recipe)
}

// Translated from 00996f10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00996f10 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatProcedure::CreateProcedure` (Xbox PDB; `cdecl(kind)`): for kinds 0..=12 a block of
/// the kind's size (`00401000`) given to the kind's constructor (null when the allocation
/// fails): 0 `009d0890` (0x48 bytes), 1 `009cc0c0` (0x88), 2 `009cadf0` (0x3c), 3 `009cbad0`
/// (0x20), 4 `009d5ad0(0)` (0x18), 5 `009da720(0)` (0x18), 6 `009d69f0(0, 0.0, 0)` (0x7c),
/// 7 `009d2440(0)` (0x6c), 8 `009ca5f0(0, 0)` (0x1c), 9 `009d6030(0, 0.0, 0, 0)` (0x40),
/// 10 `009d8b60` (0x40), 11 `009daa10(0)` (0x18), 12 `009d3a00` (0x64). Any other kind calls
/// `005b5e40(1, "COMBAT: Could not create combat procedure of type %i", kind)` (the words as
/// pushed) and answers null.
pub fn combat_procedure_create_procedure(e: &mut Engine, kind: u32) -> u32 {
    let Some((size, constructor, words)) = procedure_recipe(kind) else {
        e.call(DEBUG_PRINT, &args![1u32, UNKNOWN_KIND_FORMAT, kind]);
        return 0;
    };
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let mut call = vec![block];
    call.extend(words);
    e.call(constructor, &call).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_6de0, combat_procedure_get_attacker(Ptr) -> u32),
        entry!(0x0099_6e00, combat_procedure_get_target(Ptr) -> u32),
        entry!(0x0099_6e20, combat_procedure_set_move_mode(Ptr, u32)),
        entry!(0x0099_6e40, combat_procedure_clear_move_mode(Ptr)),
        entry!(
            0x0099_6e60,
            combat_procedure_set_look_at_target(Ptr, Ptr, u8)
        ),
        entry!(0x0099_6ea0, combat_procedure_clear_look_at_target(Ptr)),
        entry!(0x0099_6ed0, combat_procedure_save_game(Ptr, Ptr)),
        entry!(0x0099_6ef0, combat_procedure_load_game(Ptr, Ptr)),
        entry!(0x0099_6f10, combat_procedure_create_procedure(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A procedure whose controller (at `+4`) is `0x6100`.
    fn procedure(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 4, 0x6100);
        this
    }

    #[test]
    fn the_attacker_comes_from_the_controller() {
        let mut e = Engine::new();
        e.register(0x0097_ae90, |_, a| (a[0] + 1).into_ret());
        let this = procedure(&mut e);
        assert_eq!(e.call(0x0099_6de0, &args![this]).u32(), 0x6101);
    }

    #[test]
    fn the_target_comes_from_the_controller() {
        let mut e = Engine::new();
        e.register(0x0040_30b0, |_, a| (a[0] + 2).into_ret());
        let this = procedure(&mut e);
        assert_eq!(e.call(0x0099_6e00, &args![this]).u32(), 0x6102);
    }

    #[test]
    fn set_move_mode_goes_to_the_controller() {
        let mut e = Engine::new();
        e.register(0x0098_1570, |_, _| Ret::default());
        let this = procedure(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0099_6e20, &args![this, 7u32]);
        assert_eq!(calls(&e, 0x0098_1570), vec![vec![0x6100, 7]]);
    }

    #[test]
    fn clear_move_mode_goes_to_the_controller() {
        let mut e = Engine::new();
        e.register(0x0098_1660, |_, _| Ret::default());
        let this = procedure(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0099_6e40, &args![this]);
        assert_eq!(calls(&e, 0x0098_1660), vec![vec![0x6100]]);
    }

    #[test]
    fn set_look_at_target_uses_the_procedures_index() {
        let mut e = Engine::new();
        e.register(LOOK_AT_INDEX, |_, a| (a[1] & 0xff).into_ret());
        e.register(0x0098_16d0, |_, _| Ret::default());
        let this = procedure(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0099_6e60, &args![this, 0x7000u32, 0x1204u32]);
        assert_eq!(calls(&e, LOOK_AT_INDEX), vec![vec![0x6100, this]]);
        assert_eq!(
            calls(&e, 0x0098_16d0),
            vec![vec![0x6100, this & 0xff, 0x7000, 4]]
        );
    }

    #[test]
    fn clear_look_at_target_uses_the_procedures_index() {
        let mut e = Engine::new();
        e.register(LOOK_AT_INDEX, |_, a| (a[1] & 0xff).into_ret());
        e.register(0x0098_1720, |_, _| Ret::default());
        let this = procedure(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0099_6ea0, &args![this]);
        assert_eq!(calls(&e, LOOK_AT_INDEX), vec![vec![0x6100, this]]);
        assert_eq!(calls(&e, 0x0098_1720), vec![vec![0x6100, this & 0xff]]);
    }

    #[test]
    fn save_game_writes_the_word_at_8() {
        let mut e = Engine::new();
        e.register(SAVE_DATA, |_, _| Ret::default());
        let this = procedure(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0099_6ed0, &args![this, 0x9000u32]);
        assert_eq!(calls(&e, SAVE_DATA), vec![vec![0x9000, this + 8, 4, 0]]);
    }

    #[test]
    fn load_game_reads_the_word_at_8() {
        let mut e = Engine::new();
        e.register(LOAD_DATA, |_, _| Ret::default());
        let this = procedure(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0099_6ef0, &args![this, 0x9100u32]);
        assert_eq!(calls(&e, LOAD_DATA), vec![vec![0x9100, this + 8, 4]]);
    }

    #[test]
    fn create_procedure_builds_each_kind_or_reports_it() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        let constructors = [
            0x009d_0890u32,
            0x009c_c0c0,
            0x009c_adf0,
            0x009c_bad0,
            0x009d_5ad0,
            0x009d_a720,
            0x009d_69f0,
            0x009d_2440,
            0x009c_a5f0,
            0x009d_6030,
            0x009d_8b60,
            0x009d_aa10,
            0x009d_3a00,
        ];
        for c in constructors {
            e.register(c, |_, a| (a[0] ^ 0x1).into_ret());
        }
        let sizes = [
            0x48u32, 0x88, 0x3c, 0x20, 0x18, 0x18, 0x7c, 0x6c, 0x1c, 0x40, 0x40, 0x18, 0x64,
        ];
        let extra: [usize; 13] = [0, 0, 0, 0, 1, 1, 3, 1, 2, 4, 0, 1, 0];
        for kind in 0..13u32 {
            e.call_log = Some(vec![]);
            let made = e.call(0x0099_6f10, &args![kind]).u32();
            let k = kind as usize;
            assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![sizes[k]]]);
            let built = calls(&e, constructors[k]);
            assert_eq!(built.len(), 1, "kind {kind}");
            assert_eq!(built[0].len(), 1 + extra[k], "kind {kind}");
            assert!(built[0][1..].iter().all(|w| *w == 0), "kind {kind}");
            assert_eq!(made, built[0][0] ^ 1);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_6f10, &args![13u32]).u32(), 0);
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![vec![1, UNKNOWN_KIND_FORMAT, 13]]
        );
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_6f10, &args![2u32]).u32(), 0);
        assert!(calls(&e, 0x009c_adf0).is_empty());
    }
}
