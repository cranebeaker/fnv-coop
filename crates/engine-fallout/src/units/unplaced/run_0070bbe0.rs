//! `(unplaced)/run_0070bbe0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions sit inside `fallout/interface/interfacemanager.cpp`
//! (between `InterfaceManager::PreIdleStuff` and `InterfaceManager::Idle`)
//! but the Xbox PDB names no unit for them. `0070bbe0` and `0070bc00` are
//! two wrappers around `BSAudio` pause calls, `0070bc20` is the
//! `InterfaceManager` function that works out which reference the player is
//! looking at (`pActivateRef`, `pTelekinesisRef`, `pActivatePickLocation`),
//! and the others are small `PlayerCharacter` accessors the compiler emitted
//! next to it.
//!
//! Notes for the next session:
//! - This unit is complete after `0070c440`.
//! - `0070bc20` keeps its stage number in the debug global `011d8a90`
//!   (1, 2, 3, `0x21`, `0x22`, `0x23`, 4, 5), as the game does; the
//!   translation writes it at the same places.
//! - Layouts: `InterfaceManager` is the one of
//!   `units/fallout_interface/interfacemanager.rs` (`pActivatePickLocation`
//!   at `+0x104` is read at its offset), `PlayerCharacter` the one of
//!   `units/fallout_ai/playercharacter.rs`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_ai::playercharacter::PlayerCharacter;
use crate::units::fallout_interface::interfacemanager::InterfaceManager;

/// The player character singleton pointer (`011dea3c`).
const PLAYER_SINGLETON: u32 = 0x011d_ea3c;
/// The `TES` singleton pointer (`011dea10`), the `this` of the ray picks.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// A flag word (`011e07d0`): when set, the eye position is the cached
/// first-person camera position.
const USE_CAMERA_POSITION: u32 = 0x011e_07d0;
/// The debug stage variable (`011d8a90`).
const STAGE: u32 = 0x011d_8a90;
/// Three INI settings (`bool`s) that select the pick-everything path.
const PICK_SETTINGS: [u32; 3] = [0x011d_8afc, 0x011c_a3ec, 0x011c_a394];
/// The float setting (`011d8b78`) given to `00631ab0`.
const VIEW_CASTER_SETTING: u32 = 0x011d_8b78;
/// `InterfaceManager::pActivatePickLocation` (Xbox PDB) `+0x104`: an `NiPoint3`.
const ACTIVATE_PICK_LOCATION: u32 = 0x104;
/// The argument passed through to `00631d60` (`+0x110`, `bFuzzyActivatePick`).
const FUZZY_ACTIVATE_PICK: u32 = 0x110;

/// Setting accessor returning the address of the value (`00408d60`, byte)
/// and `00403e20` (float).
const SETTING_BYTE_ADDRESS: u32 = 0x0040_8d60;
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// The identity function (`006815c0`), used by the exe to "construct" plain
/// vectors and matrices.
const IDENTITY: u32 = 0x0068_15c0;
/// `NiPick::NiPick(bool, unsigned)` (`00e98f20`) and its destructor
/// (`00e98fa0`).
const NI_PICK_CONSTRUCT: u32 = 0x00e9_8f20;
const NI_PICK_DESTRUCT: u32 = 0x00e9_8fa0;
/// The pick results (`00500940`: the result list inside the `NiPick`),
/// their count (`0044edb0`), one result (`00968670`), its `NiAVObject`
/// (`00458b50`) and the reference that owns it (`0056f930`, cdecl).
const PICK_RESULTS: u32 = 0x0050_0940;
const PICK_RESULT_COUNT: u32 = 0x0044_edb0;
const PICK_RESULT_AT: u32 = 0x0096_8670;
const PICK_RESULT_OBJECT: u32 = 0x0045_8b50;
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `TES` ray pick with the pick data (`004571c0`: the `NiPick`, start,
/// direction, distance, flags) and the plain one (`004570d0`: the
/// `NiPick`, start, direction, distance).
const TES_PICK_FLAGS: u32 = 0x0045_71c0;
const TES_PICK: u32 = 0x0045_70d0;
/// `float` helpers: `min(a, b)` (`0040ebd0`).
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// `HUDMainMenu::SetInfoForRef(ref, flag, 0)` (`00775a00`, cdecl).
const SET_INFO_FOR_REF: u32 = 0x0077_5a00;
/// `ContainerCanHoldType` (`00481f30`, cdecl, one word) and the form type
/// of a base form (`00401170`).
const CONTAINER_CAN_HOLD_TYPE: u32 = 0x0048_1f30;
const GET_FORM_TYPE: u32 = 0x0040_1170;
/// The word at `+0x20` (`007af430`: the base form of a reference).
const GET_BASE_FORM: u32 = 0x007a_f430;

// Translated from 0070bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00ad8510` (`BSAudio`) on `this` with `(6, 1)`.
pub fn fn_0070bbe0(e: &mut Engine, this: Ptr) {
    e.call(0x00ad_8510, &args![this, 6u32, 1u32]);
}

// Translated from 0070bc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00ad8540` (`BSAudio`) on `this` with `(6, 1)`.
pub fn fn_0070bc00(e: &mut Engine, this: Ptr) {
    e.call(0x00ad_8540, &args![this, 6u32, 1u32]);
}

fn copy_words(e: &mut Engine, source: u32, destination: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, word);
    }
}

// Translation incomplete: 0070bc20 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 0070bc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `InterfaceManager` function that finds the reference under the
/// crosshair. Does nothing when `007023a0` says so or the HUD main menu
/// (`Tile::GetMenuByClass(0x3ec)`, created when missing) is absent.
/// Otherwise: the eye position (the cached first-person camera position, or
/// the position the player's virtual at slot `0x1f4` returns, raised by
/// scale times `fEyeHeight`; the third-person shoulder offset is added by
/// `0063c8a0` when `00524d10` says so) and the view direction (the second column of the rotation made
/// from the player's `+0x2bc` virtual and `GetLooking`) are built. Then,
/// depending on three INI settings, either a ray pick over everything
/// (`004571c0`, flags `0x1f`: the first result whose reference is not the
/// player gives the reference and a distance) or the view caster
/// (`00631ab0`, `00631ca0`, `00631c50`, `00631d60`, then a plain pick
/// `004570d0` unless the reference is a type that cannot be activated:
/// `0x15`, `0x1b`, `0x1c`, `0x2a`, `0x2b`, or a container that can hold it)
/// finds the reference and its distance. `pActivateRef` /
/// `pTelekinesisRef` become the reference (the telekinesis one when the
/// player has telekinesis selected, the activate one only when within
/// `iPickDistance` then), `pActivatePickLocation` the point at that
/// distance on the ray, and `HUDMainMenu::SetInfoForRef` is called.
pub fn fn_0070bc20(e: &mut Engine, this: Ptr<InterfaceManager>) {
    if e.call(0x0070_23a0, &args![this]).bool() {
        return;
    }
    e.with_stack(0x300, |e, frame| {
        let f = frame.addr();
        let position = f;
        let direction = f + 0x10;
        let rotation = f + 0x20;
        let pitch = f + 0x50;
        let product = f + 0x80;
        let shoulder = f + 0xb0;
        let pick = f + 0x100;
        let second_pick = f + 0x180;
        let ignore_handle = f + 0x200;
        let player_handle = f + 0x208;
        let eye_handle = f + 0x218;
        let hit_point = f + 0x220;
        let pick_end = f + 0x230;
        let player = e.global::<u32>(PLAYER_SINGLETON);

        let mut menu = e.call(0x00a0_9030, &args![0x3ecu32]).u32();
        if menu == 0 {
            menu = e.call(0x0076_bfe0, &args![]).u32();
        }
        let mut telekinesis_ref = 0u32;
        let mut activate_ref = 0u32;
        for vector in [position, direction, rotation, pitch] {
            e.call(IDENTITY, &args![vector]);
        }

        if e.global::<u32>(USE_CAMERA_POSITION) != 0 {
            let camera = fn_0070c3f0(e, Ptr::new(player)).addr();
            copy_words(e, camera, position, 3);
        } else {
            let eye = e.vcall(player, 0x1f4, &args![]).u32();
            copy_words(e, eye, position, 3);
            let eye_height = fn_0070c3b0(e, Ptr::new(player)) as f64;
            let scale = e.call(0x0056_7400, &args![player]).f64();
            let z = e.mem.f32(position + 8) as f64;
            e.mem.set_f32(position + 8, (scale * eye_height + z) as f32);
        }
        if e.call(0x0052_4d10, &args![player]).bool() {
            let offset = fn_0070c410(e, Ptr::new(player), Ptr::new(shoulder));
            e.call(0x0063_c8a0, &args![position, offset]);
        }
        let angle = e.vcall(player, 0x2bc, &args![0u32]).f32();
        e.call(0x004a_0c90, &args![rotation, angle]);
        let looking = e.call(0x0093_1d70, &args![player]).f32();
        e.call(0x0052_4ac0, &args![pitch, looking]);
        let turned = e.call(0x0043_f8d0, &args![rotation, product, pitch]).u32();
        copy_words(e, turned, rotation, 9);
        e.call(0x0043_9f50, &args![rotation, 1u32, direction]);
        let pick_distance = e.get(this, InterfaceManager::iPickDistance);
        let max_distance = pick_distance as f32;
        if menu == 0 {
            return;
        }

        e.set_global(STAGE, 1u32);
        let mut reference = 0u32;
        let mut distance = 0.0f32;
        let mut any_setting = false;
        for setting in PICK_SETTINGS {
            let at = e.call(SETTING_BYTE_ADDRESS, &args![setting]).u32();
            if e.mem.u8(at) != 0 {
                any_setting = true;
                break;
            }
        }
        let tes = e.global::<u32>(TES_SINGLETON);
        if any_setting {
            e.call(NI_PICK_CONSTRUCT, &args![pick, 0u32, 8u32]);
            e.call(0x0093_1ed0, &args![player, player_handle]);
            e.call(0x008c_71b0, &args![ignore_handle, 0u32]);
            e.call(0x004a_39f0, &args![ignore_handle, 0x24u32]);
            let pointer = e.call(0x004a_3a20, &args![player_handle]).u32();
            e.call(0x0059_ce80, &args![ignore_handle, pointer]);
            if menu != 0
                && e.call(
                    TES_PICK_FLAGS,
                    &args![tes, pick, position, direction, max_distance, 0x1fu32],
                )
                .bool()
            {
                let mut index = 0u32;
                loop {
                    let results = e.call(PICK_RESULTS, &args![pick]).u32();
                    let count = e.call(PICK_RESULT_COUNT, &args![results]).u32();
                    if index >= count {
                        break;
                    }
                    let results = e.call(PICK_RESULTS, &args![pick]).u32();
                    let item = e.call(PICK_RESULT_AT, &args![results, index]).u32();
                    let object = e.call(PICK_RESULT_OBJECT, &args![item]).u32();
                    reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
                    if reference != player {
                        let results = e.call(PICK_RESULTS, &args![pick]).u32();
                        let item = e.call(PICK_RESULT_AT, &args![results, index]).u32();
                        distance = e.call(0x0047_c860, &args![item]).f32();
                        break;
                    }
                    reference = 0;
                    index += 1;
                }
            }
            e.call(NI_PICK_DESTRUCT, &args![pick]);
        } else {
            e.set_global(STAGE, 2u32);
            let view_caster = e.get(this, InterfaceManager::pViewCaster);
            if !e.call(0x004f_1540, &args![view_caster]).bool() && player != 0 {
                let controller = e.call(0x0093_06d0, &args![player]).u32();
                if controller != 0 {
                    fn_0070c440(e, Ptr::new(controller), Ptr::new(eye_handle));
                    let pointer = e.call(0x004a_3a20, &args![eye_handle]).u32();
                    let setting = e
                        .call(SETTING_FLOAT_ADDRESS, &args![VIEW_CASTER_SETTING])
                        .u32();
                    let value = e.mem.f32(setting);
                    e.call(0x0063_1ab0, &args![view_caster, value, pointer]);
                }
            }
            e.set_global(STAGE, 3u32);
            let view_caster = e.get(this, InterfaceManager::pViewCaster);
            if e.call(0x004f_1540, &args![view_caster]).bool() {
                e.set_global(STAGE, 0x21u32);
                let world_owner = e.call(0x008d_6f30, &args![player]).u32();
                if world_owner != 0 {
                    let current = e.call(0x0062_1440, &args![view_caster]).u32();
                    let owner = e.call(0x008d_6f30, &args![player]).u32();
                    let wanted = e.call(0x0045_43c0, &args![owner]).u32();
                    if current != wanted {
                        let owner = e.call(0x008d_6f30, &args![player]).u32();
                        let wanted = e.call(0x0045_43c0, &args![owner]).u32();
                        e.call(0x0063_1ca0, &args![view_caster, wanted]);
                    }
                }
                e.set_global(STAGE, 0x22u32);
                let handle = e.call(0x0093_1ed0, &args![player, eye_handle]).u32();
                let pointer = e.call(0x004a_3a20, &args![handle]).u32();
                let current = e.call(0x0044_ddc0, &args![view_caster]).u32();
                if current != pointer {
                    e.call(0x0063_1c50, &args![view_caster, pointer]);
                }
                e.set_global(STAGE, 0x23u32);
                e.mem.set_f32(pick_end, 0.0);
                reference = e
                    .call(
                        0x0063_1d60,
                        &args![
                            view_caster,
                            position,
                            direction,
                            max_distance,
                            pick_end,
                            this.addr() + FUZZY_ACTIVATE_PICK
                        ],
                    )
                    .u32();
                distance = e.mem.f32(pick_end);
                let mut accept = true;
                if reference != 0 {
                    let base = e.call(GET_BASE_FORM, &args![reference]).u32();
                    let form_type = e.call(GET_FORM_TYPE, &args![base]).u8();
                    match form_type {
                        0x15 | 0x1b | 0x1c | 0x2a | 0x2b => accept = false,
                        _ => {
                            let holds = e
                                .call(CONTAINER_CAN_HOLD_TYPE, &args![form_type as u32])
                                .bool();
                            accept = !holds;
                        }
                    }
                }
                if accept {
                    e.call(NI_PICK_CONSTRUCT, &args![second_pick, 0u32, 8u32]);
                    let nearest = e.call(FLOAT_MIN, &args![max_distance, distance]).f32();
                    if e.call(
                        TES_PICK,
                        &args![tes, second_pick, position, direction, nearest],
                    )
                    .bool()
                    {
                        let results = e.call(PICK_RESULTS, &args![second_pick]).u32();
                        let item = e.call(PICK_RESULT_AT, &args![results, 0u32]).u32();
                        let object = e.call(PICK_RESULT_OBJECT, &args![item]).u32();
                        let found = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
                        if found != 0 {
                            reference = found;
                        }
                    }
                    e.call(NI_PICK_DESTRUCT, &args![second_pick]);
                }
            }
        }

        e.set_global(STAGE, 4u32);
        if reference == 0 {
            activate_ref = 0;
            telekinesis_ref = 0;
        } else if fn_0070c3d0(e, Ptr::new(player)) != 0 {
            telekinesis_ref = reference;
            if distance as f64 <= pick_distance as f64 {
                activate_ref = reference;
            }
        } else {
            activate_ref = reference;
        }
        e.set(this, InterfaceManager::pTelekinesisRef, telekinesis_ref);
        e.set(this, InterfaceManager::pActivateRef, activate_ref);
        let scratch = f + 0x240;
        let point = e
            .call(0x0045_bb20, &args![direction, scratch, distance])
            .u32();
        let location = e
            .call(0x0043_9e90, &args![position, hit_point, point])
            .u32();
        copy_words(e, location, this.addr() + ACTIVATE_PICK_LOCATION, 3);
        e.set_global(STAGE, 5u32);
        let no_activate = (e.get(this, InterfaceManager::pActivateRef) == 0) as u32;
        e.call(SET_INFO_FOR_REF, &args![reference, no_activate, 0u32]);
    })
}

// Translated from 0070c3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::fEyeHeight` (Xbox PDB field), returned in `ST0`.
pub fn fn_0070c3b0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> f32 {
    e.get(this, PlayerCharacter::fEyeHeight)
}

// Translated from 0070c3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::bTelekinesisSelected` (Xbox PDB field, `+0x75e`): returns
/// the byte as stored (`MOV AL,[EAX+0x75e]`), not normalised to 0/1 (#198).
pub fn fn_0070c3d0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u8 {
    e.mem
        .u8(this.addr() + PlayerCharacter::bTelekinesisSelected.off)
}

// Translated from 0070c3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of `PlayerCharacter::Cached1stPersonCameraPos` (Xbox PDB
/// field, an `NiPoint3`).
pub fn fn_0070c3f0(_e: &mut Engine, this: Ptr<PlayerCharacter>) -> Ptr {
    this.byte_add(PlayerCharacter::Cached1stPersonCameraPos.off)
}

// Translated from 0070c410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `PlayerCharacter::kCamera3rdPersonShoulderOffset` (Xbox PDB field,
/// an `NiPoint3`) into `result` and returns `result`.
pub fn fn_0070c410(e: &mut Engine, this: Ptr<PlayerCharacter>, result: Ptr) -> Ptr {
    let source = this.addr() + PlayerCharacter::kCamera3rdPersonShoulderOffset.off;
    copy_words(e, source, result.addr(), 3);
    result
}

// Translated from 0070c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the first word of the handle built from the `NiPointer` at
/// `this + 0x594`: its pointee's (`00559450`) word through `0043b4f0`
/// when it is set, else a null handle (`008c71b0(0)`). Returns `result`.
pub fn fn_0070c440(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let slot = this.addr() + 0x594;
    let held = e.call(0x0055_9450, &args![slot]).u32();
    let handle = if held != 0 {
        e.with_stack(8, |e, scratch| {
            let held = e.call(0x0055_9450, &args![slot]).u32();
            e.call(0x0043_b4f0, &args![held, scratch]).u32()
        })
    } else {
        e.with_stack(8, |e, scratch| {
            e.call(0x008c_71b0, &args![scratch, 0u32]).u32()
        })
    };
    let word = e.mem.u32(handle);
    e.mem.set_u32(result.addr(), word);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070bbe0, fn_0070bbe0(Ptr)),
        entry!(0x0070bc00, fn_0070bc00(Ptr)),
        entry!(0x0070bc20, fn_0070bc20(Ptr<InterfaceManager>)),
        entry!(0x0070c3b0, fn_0070c3b0(Ptr<PlayerCharacter>) -> f32),
        entry!(0x0070c3d0, fn_0070c3d0(Ptr<PlayerCharacter>) -> u8),
        entry!(0x0070c3f0, fn_0070c3f0(Ptr<PlayerCharacter>) -> Ptr),
        entry!(0x0070c410, fn_0070c410(Ptr<PlayerCharacter>, Ptr) -> Ptr),
        entry!(0x0070c440, fn_0070c440(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Registers a double that logs its call and then answers with `f`.
    fn mock(
        e: &mut Engine,
        log: &Log,
        addr: u32,
        f: impl Fn(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        let log = log.clone();
        e.register_double(addr, move |e, w| {
            log.borrow_mut().push((addr, w.to_vec()));
            f(e, w)
        });
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn audio_wrappers_pass_six_and_one() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        mock(&mut e, &log, 0x00ad_8510, |_, _| Ret::default());
        mock(&mut e, &log, 0x00ad_8540, |_, _| Ret::default());
        e.call(0x0070_bbe0, &args![0x1234u32]);
        e.call(0x0070_bc00, &args![0x5678u32]);
        assert_eq!(calls_to(&log, 0x00ad_8510), vec![vec![0x1234, 6, 1]]);
        assert_eq!(calls_to(&log, 0x00ad_8540), vec![vec![0x5678, 6, 1]]);
    }

    #[test]
    fn player_accessors() {
        let mut e = Engine::new();
        let player = e.new_object::<PlayerCharacter>();
        e.set(player, PlayerCharacter::fEyeHeight, 1.75);
        e.set(player, PlayerCharacter::bTelekinesisSelected, true);
        assert_eq!(e.call(0x0070_c3b0, &args![player]).f32(), 1.75);
        assert_eq!(e.call(0x0070_c3d0, &args![player]).u8(), 1);
        assert_eq!(
            e.call(0x0070_c3f0, &args![player]).u32(),
            player.addr() + 0xdd4
        );
        let shoulder = player.addr() + 0xd58;
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(shoulder + 4 * i as u32, v);
        }
        let out = e.mem.alloc(16);
        e.mem.set_u32(out + 12, 0xdead);
        assert_eq!(e.call(0x0070_c410, &args![player, out]).u32(), out);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 8), 3.0);
        // Only three words are copied.
        assert_eq!(e.mem.u32(out + 12), 0xdead);
        e.set(player, PlayerCharacter::bTelekinesisSelected, false);
        assert_eq!(e.call(0x0070_c3d0, &args![player]).u8(), 0);
        // Any other byte comes back as stored (#198).
        e.mem.set_u8(player.addr() + 0x75e, 0x02);
        assert_eq!(e.call(0x0070_c3d0, &args![player]).u8(), 0x02);
    }

    #[test]
    fn handle_word_comes_from_the_held_object_or_a_null_handle() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let this = e.mem.alloc(0x600);
        let result = e.mem.alloc(8);
        // Nothing held: a null handle is built (`008c71b0(0)`) and its word
        // copied.
        mock(&mut e, &log, 0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        mock(&mut e, &log, 0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], 0x77);
            ret(a[0])
        });
        mock(&mut e, &log, 0x0043_b4f0, |e, a| {
            e.mem.set_u32(a[1], 0x99);
            ret(a[1])
        });
        assert_eq!(e.call(0x0070_c440, &args![this, result]).u32(), result);
        assert_eq!(e.mem.u32(result), 0x77);
        assert_eq!(calls_to(&log, 0x008c_71b0)[0][1], 0);
        assert!(calls_to(&log, 0x0043_b4f0).is_empty());
        // Something held: `0043b4f0(held, scratch)` supplies the handle.
        e.mem.set_u32(this + 0x594, 0x5000);
        assert_eq!(e.call(0x0070_c440, &args![this, result]).u32(), result);
        assert_eq!(e.mem.u32(result), 0x99);
        assert_eq!(calls_to(&log, 0x0043_b4f0)[0][0], 0x5000);
    }

    /// The state the `0070bc20` doubles answer from.
    struct Scene {
        e: Engine,
        log: Log,
        manager: Ptr<InterfaceManager>,
        player: u32,
    }

    /// Doubles for every callee of `0070bc20` with neutral answers: the
    /// player (with a vtable giving the eye position and angle), the three
    /// settings off, a view caster that is inactive.
    fn scene() -> Scene {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for addr in [PLAYER_SINGLETON, TES_SINGLETON, USE_CAMERA_POSITION, STAGE] {
            e.map(addr, 8);
        }
        let player = e.mem.alloc(0xe50);
        e.set_global(PLAYER_SINGLETON, player);
        e.set_global(TES_SINGLETON, 0x7e50_0000u32);
        let eye_position = e.mem.alloc(16);
        e.mem.set_f32(eye_position, 10.0);
        e.mem.set_f32(eye_position + 4, 20.0);
        e.mem.set_f32(eye_position + 8, 30.0);
        let mut slots = vec![0u32; 0x100];
        slots[0x1f4 / 4] = 0x0060_d1f4;
        slots[0x2bc / 4] = 0x0060_d2bc;
        e.put_vtable(0x0a00_5000, &slots);
        e.mem.set_u32(player, 0x0a00_5000);
        mock(&mut e, &log, 0x0060_d1f4, move |_, _| ret(eye_position));
        mock(&mut e, &log, 0x0060_d2bc, |_, _| float_ret(0.5));
        mock(&mut e, &log, 0x0070_23a0, |_, _| ret(0));
        mock(&mut e, &log, 0x00a0_9030, |_, _| ret(0x3333));
        mock(&mut e, &log, 0x0076_bfe0, |_, _| ret(0x4444));
        mock(&mut e, &log, IDENTITY, |_, a| ret(a[0]));
        mock(&mut e, &log, 0x0056_7400, |_, _| float_ret(2.0));
        mock(&mut e, &log, 0x0052_4d10, |_, _| ret(0));
        mock(&mut e, &log, 0x0063_c8a0, |_, _| Ret::default());
        mock(&mut e, &log, 0x004a_0c90, |_, _| Ret::default());
        mock(&mut e, &log, 0x0093_1d70, |_, _| float_ret(0.25));
        mock(&mut e, &log, 0x0052_4ac0, |_, _| Ret::default());
        mock(&mut e, &log, 0x0043_f8d0, |e, a| {
            // The product is the rotation itself (nine words copied back).
            for i in 0..9 {
                let w = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, w);
            }
            ret(a[1])
        });
        mock(&mut e, &log, 0x0043_9f50, |e, a| {
            for (i, v) in [0.0f32, 1.0, 0.0].into_iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * i as u32, v);
            }
            Ret::default()
        });
        let off = e.mem.alloc(8);
        mock(&mut e, &log, SETTING_BYTE_ADDRESS, move |_, _| ret(off));
        let float_setting = e.mem.alloc(8);
        e.mem.set_f32(float_setting, 7.5);
        mock(&mut e, &log, SETTING_FLOAT_ADDRESS, move |_, _| {
            ret(float_setting)
        });
        for addr in [
            NI_PICK_CONSTRUCT,
            NI_PICK_DESTRUCT,
            0x0093_1ed0,
            0x004a_39f0,
            0x0059_ce80,
            0x0063_1ab0,
            0x0063_1ca0,
            0x0063_1c50,
            SET_INFO_FOR_REF,
        ] {
            mock(&mut e, &log, addr, |_, _| Ret::default());
        }
        mock(&mut e, &log, 0x008c_71b0, |_, a| ret(a[0]));
        mock(&mut e, &log, 0x0055_9450, |_, _| ret(0));
        mock(&mut e, &log, 0x004a_3a20, |_, a| ret(a[0] + 1));
        mock(&mut e, &log, 0x004f_1540, |_, _| ret(0));
        mock(&mut e, &log, 0x0093_06d0, |_, _| ret(0));
        mock(&mut e, &log, 0x0045_bb20, |e, a| {
            // scratch = direction * distance
            let d = f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * d;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            ret(a[1])
        });
        mock(&mut e, &log, 0x0043_9e90, |e, a| {
            // out = position + scratch
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            ret(a[1])
        });
        mock(&mut e, &log, 0x0063_1d60, |_, _| ret(0));
        mock(&mut e, &log, 0x0045_43c0, |_, a| ret(a[0] + 0x10));
        mock(&mut e, &log, 0x008d_6f30, |_, _| ret(0));
        mock(&mut e, &log, 0x0062_1440, |_, _| ret(0));
        mock(&mut e, &log, 0x0044_ddc0, |_, _| ret(0));
        mock(&mut e, &log, 0x007a_f430, |_, a| ret(a[0] + 0x20));
        mock(&mut e, &log, GET_FORM_TYPE, |_, _| ret(0x20));
        mock(&mut e, &log, CONTAINER_CAN_HOLD_TYPE, |_, _| ret(0));
        mock(&mut e, &log, TES_PICK_FLAGS, |_, _| ret(0));
        mock(&mut e, &log, TES_PICK, |_, _| ret(0));
        mock(&mut e, &log, FLOAT_MIN, |_, a| {
            float_ret(f32::from_bits(a[0]).min(f32::from_bits(a[1])) as f64)
        });
        mock(&mut e, &log, PICK_RESULTS, |_, a| ret(a[0] + 0x10));
        mock(&mut e, &log, PICK_RESULT_COUNT, |_, _| ret(0));
        mock(&mut e, &log, PICK_RESULT_AT, |_, a| ret(a[1]));
        mock(&mut e, &log, PICK_RESULT_OBJECT, |_, a| ret(a[0]));
        mock(&mut e, &log, FIND_REFERENCE_FOR_3D, |_, a| ret(a[0]));
        mock(&mut e, &log, 0x0047_c860, |_, _| float_ret(0.0));
        let manager = e.new_object::<InterfaceManager>();
        e.set(manager, InterfaceManager::iPickDistance, 100);
        e.set(manager, InterfaceManager::pViewCaster, 0x6000);
        Scene {
            e,
            log,
            manager,
            player,
        }
    }

    fn set_pick_setting(scene: &mut Scene) {
        let on = scene.e.mem.alloc(8);
        scene.e.mem.set_u8(on, 1);
        mock(
            &mut scene.e,
            &scene.log,
            SETTING_BYTE_ADDRESS,
            move |_, _| ret(on),
        );
    }

    #[test]
    fn nothing_happens_when_the_interface_says_no() {
        let mut s = scene();
        mock(&mut s.e, &s.log, 0x0070_23a0, |_, _| ret(1));
        fn_0070bc20(&mut s.e, s.manager);
        assert!(calls_to(&s.log, 0x00a0_9030).is_empty());
        assert_eq!(s.e.global::<u32>(STAGE), 0);
    }

    #[test]
    fn without_the_hud_menu_the_function_ends_before_the_picks() {
        let mut s = scene();
        mock(&mut s.e, &s.log, 0x00a0_9030, |_, _| ret(0));
        mock(&mut s.e, &s.log, 0x0076_bfe0, |_, _| ret(0));
        fn_0070bc20(&mut s.e, s.manager);
        assert_eq!(calls_to(&s.log, 0x0076_bfe0).len(), 1);
        assert_eq!(s.e.global::<u32>(STAGE), 0);
        assert!(calls_to(&s.log, SET_INFO_FOR_REF).is_empty());
    }

    #[test]
    fn a_missing_menu_is_created() {
        let mut s = scene();
        mock(&mut s.e, &s.log, 0x00a0_9030, |_, _| ret(0));
        fn_0070bc20(&mut s.e, s.manager);
        // The created menu (0x4444) lets the function continue to the end.
        assert_eq!(s.e.global::<u32>(STAGE), 5);
        assert_eq!(calls_to(&s.log, SET_INFO_FOR_REF).len(), 1);
    }

    #[test]
    fn eye_position_is_raised_by_scale_times_eye_height() {
        let mut s = scene();
        s.e.set(
            Ptr::<PlayerCharacter>::new(s.player),
            PlayerCharacter::fEyeHeight,
            1.5,
        );
        fn_0070bc20(&mut s.e, s.manager);
        // The pick start is not used here; read it from the view-caster
        // call-free path: SET_INFO_FOR_REF ran, and the hit point is
        // position + direction * distance (distance 0 here).
        let location = s.manager.addr() + ACTIVATE_PICK_LOCATION;
        assert_eq!(s.e.mem.f32(location), 10.0);
        assert_eq!(s.e.mem.f32(location + 4), 20.0);
        // z = 30 + scale (2.0) * eye height (1.5) = 33.
        assert_eq!(s.e.mem.f32(location + 8), 33.0);
    }

    #[test]
    fn camera_position_is_used_when_the_flag_is_set() {
        let mut s = scene();
        s.e.set_global(USE_CAMERA_POSITION, 1u32);
        let camera = s.player + 0xdd4;
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            s.e.mem.set_f32(camera + 4 * i as u32, v);
        }
        fn_0070bc20(&mut s.e, s.manager);
        let location = s.manager.addr() + ACTIVATE_PICK_LOCATION;
        assert_eq!(s.e.mem.f32(location + 8), 3.0);
        assert!(calls_to(&s.log, 0x0060_d1f4).is_empty());
        assert!(calls_to(&s.log, 0x0056_7400).is_empty());
    }

    #[test]
    fn shoulder_offset_is_added_when_the_player_asks_for_it() {
        let mut s = scene();
        mock(&mut s.e, &s.log, 0x0052_4d10, |_, _| ret(1));
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            s.e.mem.set_f32(s.player + 0xd58 + 4 * i as u32, v);
        }
        let seen: Rc<RefCell<Vec<f32>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        mock(&mut s.e, &s.log, 0x0063_c8a0, move |e, a| {
            for i in 0..3 {
                sink.borrow_mut().push(e.mem.f32(a[1] + 4 * i));
            }
            Ret::default()
        });
        fn_0070bc20(&mut s.e, s.manager);
        assert_eq!(*seen.borrow(), vec![1.0, 2.0, 3.0]);
        // It is applied to the eye position (the first argument).
        assert_eq!(calls_to(&s.log, 0x0063_c8a0).len(), 1);
    }

    /// A pick result `n` stands for the reference `0x9000 + n`.
    fn pick_scene(player_first: bool) -> Scene {
        let mut s = scene();
        set_pick_setting(&mut s);
        let player = s.player;
        mock(&mut s.e, &s.log, TES_PICK_FLAGS, |_, _| ret(1));
        mock(&mut s.e, &s.log, PICK_RESULT_COUNT, |_, _| ret(2));
        mock(&mut s.e, &s.log, PICK_RESULT_OBJECT, |_, a| ret(a[0]));
        mock(&mut s.e, &s.log, FIND_REFERENCE_FOR_3D, move |_, a| {
            // Result 0 is the player (when asked for), result 1 a reference.
            ret(if a[0] == 0 && player_first {
                player
            } else {
                0x9000 + a[0]
            })
        });
        mock(&mut s.e, &s.log, 0x0047_c860, |_, a| {
            float_ret(a[0] as f64 + 2.5)
        });
        s
    }

    #[test]
    fn the_pick_skips_the_player_and_takes_the_next_reference() {
        let mut s = pick_scene(true);
        fn_0070bc20(&mut s.e, s.manager);
        // Result 1: reference 0x9001, distance 1 + 2.5.
        assert_eq!(s.e.get(s.manager, InterfaceManager::pActivateRef), 0x9001);
        assert_eq!(s.e.get(s.manager, InterfaceManager::pTelekinesisRef), 0);
        let info = &calls_to(&s.log, SET_INFO_FOR_REF)[0];
        assert_eq!(info, &vec![0x9001, 0, 0]);
        // The hit point: position + direction * 3.5.
        let location = s.manager.addr() + ACTIVATE_PICK_LOCATION;
        assert_eq!(s.e.mem.f32(location), 10.0);
        assert_eq!(s.e.mem.f32(location + 4), 23.5);
        // The pick ran with the filter `0x1f` and the flag objects were set up.
        let pick = &calls_to(&s.log, TES_PICK_FLAGS)[0];
        assert_eq!(pick[0], 0x7e50_0000);
        assert_eq!(pick[5], 0x1f);
        assert_eq!(f32::from_bits(pick[4]), 100.0);
        assert_eq!(calls_to(&s.log, 0x004a_39f0)[0][1], 0x24);
        assert_eq!(calls_to(&s.log, NI_PICK_DESTRUCT).len(), 1);
        // View caster paths were not touched.
        assert!(calls_to(&s.log, 0x0063_1d60).is_empty());
        assert_eq!(s.e.global::<u32>(STAGE), 5);
    }

    #[test]
    fn a_pick_of_only_the_player_finds_nothing() {
        let mut s = pick_scene(true);
        mock(&mut s.e, &s.log, PICK_RESULT_COUNT, |_, _| ret(1));
        fn_0070bc20(&mut s.e, s.manager);
        assert_eq!(s.e.get(s.manager, InterfaceManager::pActivateRef), 0);
        assert_eq!(s.e.get(s.manager, InterfaceManager::pTelekinesisRef), 0);
        let info = &calls_to(&s.log, SET_INFO_FOR_REF)[0];
        assert_eq!(info, &vec![0, 1, 0]);
    }

    #[test]
    fn a_failed_pick_finds_nothing() {
        let mut s = pick_scene(false);
        mock(&mut s.e, &s.log, TES_PICK_FLAGS, |_, _| ret(0));
        fn_0070bc20(&mut s.e, s.manager);
        assert_eq!(s.e.get(s.manager, InterfaceManager::pActivateRef), 0);
        assert!(calls_to(&s.log, PICK_RESULT_COUNT).is_empty());
    }

    #[test]
    fn telekinesis_keeps_the_far_reference_out_of_activation() {
        let mut s = pick_scene(false);
        s.e.set(
            Ptr::<PlayerCharacter>::new(s.player),
            PlayerCharacter::bTelekinesisSelected,
            true,
        );
        // Result 0 is hit at distance 2.5: within 100.
        fn_0070bc20(&mut s.e, s.manager);
        assert_eq!(s.e.get(s.manager, InterfaceManager::pActivateRef), 0x9000);
        assert_eq!(
            s.e.get(s.manager, InterfaceManager::pTelekinesisRef),
            0x9000
        );
        // Now beyond the pick distance.
        s.e.set(s.manager, InterfaceManager::iPickDistance, 1);
        fn_0070bc20(&mut s.e, s.manager);
        assert_eq!(s.e.get(s.manager, InterfaceManager::pActivateRef), 0);
        assert_eq!(
            s.e.get(s.manager, InterfaceManager::pTelekinesisRef),
            0x9000
        );
        let last = calls_to(&s.log, SET_INFO_FOR_REF).pop().unwrap();
        assert_eq!(last, vec![0x9000, 1, 0]);
    }

    /// The view caster is active and finds a reference of type `form_type`.
    fn caster_scene(form_type: u32) -> Scene {
        let mut s = scene();
        mock(&mut s.e, &s.log, 0x004f_1540, |_, _| ret(1));
        mock(&mut s.e, &s.log, 0x0063_1d60, |e, a| {
            e.mem.set_f32(a[4], 4.0);
            ret(0x9100)
        });
        mock(&mut s.e, &s.log, GET_FORM_TYPE, move |_, _| ret(form_type));
        mock(&mut s.e, &s.log, 0x008d_6f30, |_, _| ret(0xa000));
        mock(&mut s.e, &s.log, 0x0062_1440, |_, _| ret(0x31));
        mock(&mut s.e, &s.log, 0x0045_43c0, |_, a| ret(a[0] + 1));
        mock(&mut s.e, &s.log, 0x0044_ddc0, |_, _| ret(0x11));
        mock(&mut s.e, &s.log, 0x0093_1ed0, |_, a| ret(a[1]));
        mock(&mut s.e, &s.log, 0x004a_3a20, |_, _| ret(0x55));
        s
    }

    #[test]
    fn the_view_caster_path_syncs_the_caster_and_runs_the_second_pick() {
        let mut s = caster_scene(0x20);
        // The plain pick finds a closer reference.
        mock(&mut s.e, &s.log, TES_PICK, |_, _| ret(1));
        mock(&mut s.e, &s.log, PICK_RESULT_OBJECT, |_, _| ret(0x9200));
        mock(&mut s.e, &s.log, FIND_REFERENCE_FOR_3D, |_, a| ret(a[0]));
        fn_0070bc20(&mut s.e, s.manager);
        // World and object count differ: both are updated.
        assert_eq!(calls_to(&s.log, 0x0063_1ca0), vec![vec![0x6000, 0xa001]]);
        assert_eq!(calls_to(&s.log, 0x0063_1c50), vec![vec![0x6000, 0x55]]);
        let caster = &calls_to(&s.log, 0x0063_1d60)[0];
        assert_eq!(caster[0], 0x6000);
        assert_eq!(f32::from_bits(caster[3]), 100.0);
        assert_eq!(caster[5], s.manager.addr() + FUZZY_ACTIVATE_PICK);
        // The plain pick is limited to the nearer distance (4 < 100).
        let pick = &calls_to(&s.log, TES_PICK)[0];
        assert_eq!(f32::from_bits(pick[4]), 4.0);
        assert_eq!(s.e.get(s.manager, InterfaceManager::pActivateRef), 0x9200);
    }

    #[test]
    fn the_second_pick_keeps_the_caster_reference_when_it_finds_nothing() {
        let mut s = caster_scene(0x20);
        mock(&mut s.e, &s.log, TES_PICK, |_, _| ret(0));
        fn_0070bc20(&mut s.e, s.manager);
        assert_eq!(s.e.get(s.manager, InterfaceManager::pActivateRef), 0x9100);
        assert_eq!(calls_to(&s.log, NI_PICK_DESTRUCT).len(), 1);
    }

    #[test]
    fn some_form_types_skip_the_second_pick() {
        for form_type in [0x15u32, 0x1b, 0x1c, 0x2a, 0x2b] {
            let mut s = caster_scene(form_type);
            fn_0070bc20(&mut s.e, s.manager);
            assert!(calls_to(&s.log, TES_PICK).is_empty(), "type {form_type:#x}");
            assert!(calls_to(&s.log, CONTAINER_CAN_HOLD_TYPE).is_empty());
            assert_eq!(s.e.get(s.manager, InterfaceManager::pActivateRef), 0x9100);
        }
    }

    #[test]
    fn containers_that_hold_the_type_skip_the_second_pick() {
        let mut s = caster_scene(0x20);
        mock(&mut s.e, &s.log, CONTAINER_CAN_HOLD_TYPE, |_, _| ret(1));
        fn_0070bc20(&mut s.e, s.manager);
        assert_eq!(calls_to(&s.log, CONTAINER_CAN_HOLD_TYPE), vec![vec![0x20]]);
        assert!(calls_to(&s.log, TES_PICK).is_empty());
    }

    #[test]
    fn an_inactive_caster_with_a_controller_gets_the_player_handle() {
        let mut s = scene();
        mock(&mut s.e, &s.log, 0x0093_06d0, |_, _| ret(0xc000));
        mock(&mut s.e, &s.log, 0x004a_3a20, |_, _| ret(0x66));
        fn_0070bc20(&mut s.e, s.manager);
        let call = &calls_to(&s.log, 0x0063_1ab0)[0];
        assert_eq!(call[0], 0x6000);
        assert_eq!(f32::from_bits(call[1]), 7.5);
        assert_eq!(call[2], 0x66);
        // Not active: the later stages did not run.
        assert!(calls_to(&s.log, 0x0063_1d60).is_empty());
    }
}
