//! `fallout/ai/run_009baee0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004fd420` (`thiscall`, `RET 4`): a flag test.
const HAS_FLAG: u32 = 0x004f_d420;
/// `006ab360` (`thiscall`, no stack word).
const GET_ENTRY: u32 = 0x006a_b360;
/// `0048cee0` (`thiscall`, no stack word).
const ENTRY_HAS_MODEL: u32 = 0x0048_cee0;
/// The model loader singleton pointer (`011c3b3c`) and its load (`00447080`, `thiscall`,
/// `RET 0x18`; engine map `ModelLoader::LoadFile`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const LOAD_FILE: u32 = 0x0044_7080;
/// `00a5d680` (`thiscall`, no stack word; engine map `NiObject::Clone_ov2`).
const CLONE: u32 = 0x00a5_d680;
/// `0066b0d0` (`thiscall`, `RET 4`): stores the node in the holder at +0xc.
const SET_NODE: u32 = 0x0066_b0d0;
/// `00559450` (`thiscall`, no stack word): the node held at +0xc.
const GET_NODE: u32 = 0x0055_9450;
/// `0062bc10` (`cdecl`, one word; engine map `TESHavokUtilities::RemoveHavokFromSceneGraph`).
const REMOVE_HAVOK: u32 = 0x0062_bc10;
/// `009bb240` (`thiscall`, `RET 4`; engine map `MuzzleFlash::Position3D`).
const POSITION_3D: u32 = 0x009b_b240;
/// `009bb7f0` (`thiscall`, no stack word; unit `fallout/ai/combat/muzzleflash.cpp`).
const AFTER_POSITION: u32 = 0x009b_b7f0;
/// `00a6d2d0` (`cdecl`, one word; engine map `NiTimeController::StartAnimations`).
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
/// `00a5a040` (`thiscall`, no stack word; engine map `NiAVObject::UpdateProperties`).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `0043d410` (`thiscall`, `RET 0xc`: a float and two words): builds the update local.
const MAKE_UPDATE: u32 = 0x0043_d410;
/// `00a59c60` (`thiscall`, `RET 4`): updates the node with the local.
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// The game time `float` (`011c3c08`) and the `double` subtracted from it (`01090008`).
const GAME_TIME: u32 = 0x011c_3c08;
const TIME_STEP: u32 = 0x0109_0008;

// Translated from 009baee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): when the object at +0x14 is set and
/// [`fn_009baf30`] on it answers nonzero (low byte), what `006ab360` gives for it; otherwise
/// null.
pub fn fn_009baee0(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.mem.u32(this.addr() + 0x14);
    if object != 0 && fn_009baf30(e, Ptr::new(object)) as u8 != 0 {
        let object = e.mem.u32(this.addr() + 0x14);
        return e.call(GET_ENTRY, &args![object]).u32();
    }
    0
}

// Translated from 009baf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `004fd420(8)` on `this`; its `EAX`
/// is passed on (callers test the low byte).
pub fn fn_009baf30(e: &mut Engine, this: Ptr) -> u32 {
    e.call(HAS_FLAG, &args![this, 8u32]).u32()
}

// Translated from 009baf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; next to the `MuzzleFlash` functions):
/// with an entry ([`fn_009baee0`]) that `0048cee0` accepts, loads the model named by the
/// entry's vtable slot +0x14 (`ModelLoader::LoadFile` on `011c3b3c` with that name and
/// 0, 1, 0, 0, 0) and, when it loads, stores a clone of it (`00a5d680`) in the holder at +0xc
/// (`0066b0d0`). Then, when the holder has a node (`00559450`): removes Havok from it
/// (`0062bc10`), positions it (`009bb240` with `placement`), `009bb7f0`, starts its
/// animations (`00a6d2d0`), updates its properties (`00a5a040`), and updates it twice
/// (`00a59c60`) with a local built by `0043d410((float)(time - step), 1, 0)` from the game
/// time `011c3c08` and the `double` at `01090008`, the second time with the local's first
/// float set to the game time.
pub fn fn_009baf50(e: &mut Engine, this: Ptr, placement: u32) {
    let holder = this.addr() + 0xc;
    let entry = fn_009baee0(e, this);
    if entry != 0 && e.call(ENTRY_HAS_MODEL, &args![entry]).u32() != 0 {
        let name = e.vcall(entry, 0x14, &args![]).u32();
        let loader = e.mem.u32(MODEL_LOADER);
        let model = e
            .call(
                LOAD_FILE,
                &args![loader, name, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        if model != 0 {
            let clone = e.call(CLONE, &args![model]).u32();
            e.call(SET_NODE, &args![holder, clone]);
        }
    }
    if e.call(GET_NODE, &args![holder]).u32() == 0 {
        return;
    }
    let node = e.call(GET_NODE, &args![holder]).u32();
    e.call(REMOVE_HAVOK, &args![node]);
    e.call(POSITION_3D, &args![this, placement]);
    e.call(AFTER_POSITION, &args![this]);
    let node = e.call(GET_NODE, &args![holder]).u32();
    e.call(START_ANIMATIONS, &args![node]);
    let node = e.call(GET_NODE, &args![holder]).u32();
    e.call(UPDATE_PROPERTIES, &args![node]);
    let time = e.global::<u32>(GAME_TIME);
    let step: f64 = e.global(TIME_STEP);
    let start = x87_store32(x87_sub(x87_load32(time), step));
    e.with_stack(0xc, |e, local| {
        e.call(MAKE_UPDATE, &args![local, start, 1u32, 0u32]);
        let node = e.call(GET_NODE, &args![holder]).u32();
        e.call(NODE_UPDATE, &args![node, local]);
        let time = x87_quiet_bits(e.global::<u32>(GAME_TIME));
        e.mem.set_u32(local.addr(), time);
        let node = e.call(GET_NODE, &args![holder]).u32();
        e.call(NODE_UPDATE, &args![node, local]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009b_aee0, fn_009baee0(Ptr) -> u32),
        entry!(0x009b_af30, fn_009baf30(Ptr) -> u32),
        entry!(0x009b_af50, fn_009baf50(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn fn_009baee0_needs_the_object_and_its_flag() {
        let mut e = Engine::new();
        e.register(HAS_FLAG, |_, a| {
            (if a[0] == 0x5000 { 0x101u32 } else { 0x100 }).into_ret()
        });
        e.register(GET_ENTRY, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x009b_aee0, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x14, 0x5000);
        assert_eq!(e.call(0x009b_aee0, &args![this]).u32(), 0x5001);
        // Only the low byte of the flag answer counts.
        e.mem.set_u32(this + 0x14, 0x5100);
        assert_eq!(e.call(0x009b_aee0, &args![this]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_af30, &args![0x5000u32]).u32(), 0x101);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, HAS_FLAG), vec![vec![0x5000, 8]]);
    }

    fn fixture(model: u32, node: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(GAME_TIME & !0xfff, 0x1000);
        e.map(TIME_STEP & !0xfff, 0x1000);
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.mem.set_u32(GAME_TIME, 2.0f32.to_bits());
        e.mem.set_f64(TIME_STEP, 0.5);
        e.mem.set_u32(MODEL_LOADER, 0x6600);
        let entry = e.mem.alloc(4);
        e.put_vtable(0x0150_0000, &[0; 8]);
        e.mem.set_u32(0x0150_0014, 0x0150_1000);
        e.register(0x0150_1000, |_, _| 0x6700u32.into_ret());
        e.mem.set_u32(entry, 0x0150_0000);
        e.register(HAS_FLAG, |_, _| 1u32.into_ret());
        e.register_double(GET_ENTRY, move |_, _| entry.into_ret());
        e.register(ENTRY_HAS_MODEL, |_, _| 1u32.into_ret());
        e.register_double(LOAD_FILE, move |_, _| model.into_ret());
        e.register(CLONE, |_, a| (a[0] + 1).into_ret());
        e.register_double(GET_NODE, move |_, _| node.into_ret());
        for callee in [
            SET_NODE,
            REMOVE_HAVOK,
            POSITION_3D,
            AFTER_POSITION,
            START_ANIMATIONS,
            UPDATE_PROPERTIES,
            MAKE_UPDATE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(NODE_UPDATE, |e, a| e.mem.u32(a[1]).into_ret());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x14, 0x5000);
        (e, this)
    }

    #[test]
    fn fn_009baf50_loads_the_model_and_updates_the_node() {
        let (mut e, this) = fixture(0x6800, 0x6900);
        e.call_log = Some(vec![]);
        e.call(0x009b_af50, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOAD_FILE),
            vec![vec![0x6600, 0x6700, 0, 1, 0, 0, 0]]
        );
        assert_eq!(calls_to(&log, SET_NODE), vec![vec![this + 0xc, 0x6801]]);
        assert_eq!(calls_to(&log, REMOVE_HAVOK), vec![vec![0x6900]]);
        assert_eq!(calls_to(&log, POSITION_3D), vec![vec![this, 0x77]]);
        assert_eq!(calls_to(&log, START_ANIMATIONS), vec![vec![0x6900]]);
        assert_eq!(calls_to(&log, UPDATE_PROPERTIES), vec![vec![0x6900]]);
        let update = calls_to(&log, MAKE_UPDATE);
        assert_eq!(update[0][1..], [1.5f32.to_bits(), 1, 0]);
        let local = update[0][0];
        assert_eq!(
            calls_to(&log, NODE_UPDATE),
            vec![vec![0x6900, local], vec![0x6900, local]]
        );
    }

    #[test]
    fn fn_009baf50_without_a_model_or_node() {
        let (mut e, this) = fixture(0, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_af50, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SET_NODE).is_empty());
        assert_eq!(calls_to(&log, GET_NODE).len(), 1);
        assert!(calls_to(&log, REMOVE_HAVOK).is_empty());
    }
}
