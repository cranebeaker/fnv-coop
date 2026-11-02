//! `fallout/interface/forenderedterminal.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007ff8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `007ff8e0`, which tests the flag `0x4000` of the
/// word at `+0x30` through `00456630`, and returns its result (a `bool`). Parameters: one
/// word, `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX`
/// on. The return type (`bool`) is from `00456630`'s disassembly (`AND EAX,[EBP+8]; NEG EAX;
/// SBB EAX,EAX; NEG EAX`: 1 when the masked word is non-zero, else 0), which `007ff8e0`
/// returns without change.
pub fn fn_007ff8c0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x007f_f8e0, &args![this]).bool()
}

/// `00456630` (`thiscall`, `RET 4`): whether the word at +0x30 has a bit of the mask.
const FLAG_TEST_00456630: u32 = 0x0045_6630;
/// `00450b80` (`cdecl`, one word, `ECX` unused): the word at `011f91c8 + 4 * index`.
const TES_00450B80: u32 = 0x0045_0b80;
/// The holder at `011db63c`: `00559450` (`thiscall`) returns its first word, `0066b0d0`
/// (`thiscall`, `RET 4`) is called on it with 0.
const HOLDER_011DB63C: u32 = 0x011d_b63c;
const HOLDER_GET_00559450: u32 = 0x0055_9450;
const HOLDER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `ModelLoader::ReleaseModel` (`00445300`, `thiscall` on the model loader at `011c3b3c`,
/// `RET 0xc`) and the model path pointer at `011a0bb0` (the terminal interface model).
const MODEL_LOADER_RELEASE_MODEL: u32 = 0x0044_5300;
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;
const TERMINAL_MODEL_PATH_011A0BB0: u32 = 0x011a_0bb0;

// Translated from 007ff8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`): `00456630(this, 0x4000)`, its `EAX` returned
/// unchanged (0 or 1).
pub fn fn_007ff8e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(FLAG_TEST_00456630, &args![this, 0x4000u32]).bool()
}

// Translated from 007ffad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`; `this` is not read): `00450b80(1)`, whose `EAX` it
/// leaves as its own return value.
pub fn fn_007ffad0(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(TES_00450B80, &args![1u32]).u32()
}

// Translated from 007ffe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::ReleaseStaticGeometry` (Xbox PDB name from the engine map; no
/// arguments): when the holder at `011db63c` holds something (`00559450`), releases the
/// terminal model (`ModelLoader::ReleaseModel(path, 1, 1)` with the path at `011a0bb0`),
/// then `0066b0d0(holder, 0)`.
pub fn fo_rendered_terminal_release_static_geometry(e: &mut Engine) {
    if e.call(HOLDER_GET_00559450, &args![HOLDER_011DB63C]).u32() != 0 {
        let path: u32 = e.global(TERMINAL_MODEL_PATH_011A0BB0);
        let loader: u32 = e.global(MODEL_LOADER_011C3B3C);
        e.call(MODEL_LOADER_RELEASE_MODEL, &args![loader, path, 1u32, 1u32]);
    }
    e.call(HOLDER_SET_0066B0D0, &args![HOLDER_011DB63C, 0u32]);
}

// ---------------------------------------------------------------------------
// `FORenderedTerminal` (batch n051)

layout! {
    /// `FORenderedTerminal` (Xbox PDB), 0xf0 bytes: `FORenderedMenu` (0xd8) and the
    /// fields below, at the offsets the PC code uses.
    pub struct FORenderedTerminal: 0xf0 {
        /// `OriginalPos` (Xbox PDB): `NiPoint3`.
        0xD8 OriginalPos: u32,
        /// `spExitButton` (Xbox PDB): `NiPointer<NiAVObject>`.
        0xE4 spExitButton: u32,
        /// `fLastFadePercent` (Xbox PDB).
        0xE8 fLastFadePercent: f32,
        /// `bFadingOut` (Xbox PDB).
        0xEC bFadingOut: u8,
    }
}

/// `FORenderedMenu`'s functions this unit calls (`forenderedmenu.cpp`): `OnIdle`
/// (`007faf10`), `OnMenuOpen` (`007fb040`), its close counterpart (`007fb0b0`), the
/// screen pick (`007fb790`, `RET 0xc`) and the render into the texture (`007fba00`,
/// `RET 0xc`).
const RENDERED_MENU_ON_IDLE: u32 = 0x007f_af10;
const RENDERED_MENU_ON_MENU_OPEN: u32 = 0x007f_b040;
const RENDERED_MENU_ON_MENU_CLOSE: u32 = 0x007f_b0b0;
const RENDERED_MENU_PICK: u32 = 0x007f_b790;
const RENDERED_MENU_RENDER: u32 = 0x007f_ba00;
/// The menu's `NiPointer` getters (`thiscall`): `spScreenParent` (+8, `005585e0`),
/// `spMenuRoot` (+0x14, `0063d040`) and `spLight` (+0x1c, `006838b0`).
const SCREEN_PARENT_005585E0: u32 = 0x0055_85e0;
const MENU_ROOT_0063D040: u32 = 0x0063_d040;
const MENU_LIGHT_006838B0: u32 = 0x0068_38b0;
/// `00450f90` (`thiscall`, `RET 4`): sets an object's app-culled flag.
const SET_APP_CULLED_00450F90: u32 = 0x0045_0f90;
/// The update data's initialisation (`0043d410`, `RET 0xc`: a `float` and two words)
/// and the object update (`00a59c60`, `RET 4`).
const UPDATE_DATA_INIT_0043D410: u32 = 0x0043_d410;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// `BSFadeNode::SetRange` (`00b4dfd0`, `RET 8`: two `float`s), `0054b800`
/// (`thiscall`) and `00476ae0` (`thiscall`, `RET 4`) on the terminal geometry.
const FADE_NODE_SET_RANGE: u32 = 0x00b4_dfd0;
const GEOMETRY_0054B800: u32 = 0x0054_b800;
const GEOMETRY_00476AE0: u32 = 0x0047_6ae0;
/// The terminal geometry's fade (`006d2c40`, `thiscall`, `ST0`).
const GEOMETRY_FADE_006D2C40: u32 = 0x006d_2c40;
/// The fade range of an opening terminal (`01017130`) and the menu root's x translation
/// (`01078bb0`).
const OPEN_FADE_RANGE: u32 = 0x0101_7130;
const ROOT_OFFSET_01078BB0: u32 = 0x0107_8bb0;
/// The object's translation from a point (`00440460`, `RET 4`) and from three `float`s
/// (`004bc1f0`, `RET 0xc`: the local translation at +0x58).
const SET_POSITION_00440460: u32 = 0x0044_0460;
const SET_TRANSLATE_004BC1F0: u32 = 0x004b_c1f0;
/// `InterfaceManager::EnterRenderedMenu` (`007181d0`, `RET 4`) on the interface manager
/// (`004b7210`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const ENTER_RENDERED_MENU: u32 = 0x0071_81d0;
/// The setting `011f9148`: its value address (`0043d4d0`) and its setter (`0045ce80`,
/// `RET 4`).
const SETTING_011F9148: u32 = 0x011f_9148;
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_SET_0045CE80: u32 = 0x0045_ce80;
/// `009611e0` (`thiscall`) and `00984f60` (`thiscall`, `RET 4`) on the menu.
const MENU_OBJECT_009611E0: u32 = 0x0096_11e0;
const MENU_00984F60: u32 = 0x0098_4f60;
/// `BSAudioManager::QInstance` (`00ad9060`), `BSAudioManager::GetSoundHandleByName`
/// (`00ae5680`, `RET 0x10`), `BSSoundHandle::Play` (`00ad8830`, `RET 4`) and the
/// handle's destructor (`00483710`); `"OBJComputerTerminalShutdown..."` (`01078bc8`).
const AUDIO_MANAGER_INSTANCE: u32 = 0x00ad_9060;
const AUDIO_MANAGER_SOUND_BY_NAME: u32 = 0x00ae_5680;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const SHUTDOWN_SOUND: u32 = 0x0107_8bc8;
/// The security cookie (`.data`).
const SECURITY_COOKIE: u32 = 0x011c_16bc;
/// The pick: the point initialisation (`006815c0`), the camera (`008b6200`, else
/// `006629f0` of `0045c670()`), `NiPick::NiPick` (`00e98f20`, `RET 8`), `00458b30`
/// (`RET 4`), the target (`00705fc0`, `RET 4`), `006ecd40` (`RET 4`), the cursor
/// (`00792760`, `009a9350`, `ST0`), `_ftol2` (`00ec62c0`), the camera's ray
/// (`00a71080`, `RET 0x14`), `NiPick::PickObjects` (`00e98e20`, `RET 0xc`), the results
/// (`00500940`), a record (`00968670`, `RET 4`), its object (`00458b50`) and
/// `NiPick::~NiPick` (`00e98fa0`).
const POINT_INIT_006815C0: u32 = 0x0068_15c0;
const CAMERA_008B6200: u32 = 0x008b_6200;
const GETTER_0045C670: u32 = 0x0045_c670;
const GETTER_006629F0: u32 = 0x0066_29f0;
const NI_PICK_CONSTRUCT: u32 = 0x00e9_8f20;
const PICK_00458B30: u32 = 0x0045_8b30;
const PICK_SET_TARGET_00705FC0: u32 = 0x0070_5fc0;
const PICK_006ECD40: u32 = 0x006e_cd40;
const CURSOR_00792760: u32 = 0x0079_2760;
const CURSOR_009A9350: u32 = 0x009a_9350;
const FTOL: u32 = 0x00ec_62c0;
const CAMERA_WINDOW_POINT_TO_RAY: u32 = 0x00a7_1080;
const NI_PICK_PICK_OBJECTS: u32 = 0x00e9_8e20;
const PICK_RESULTS_00500940: u32 = 0x0050_0940;
const PICK_RECORD_00968670: u32 = 0x0096_8670;
const RECORD_OBJECT_00458B50: u32 = 0x0045_8b50;
const NI_PICK_DESTROY: u32 = 0x00e9_8fa0;

/// Inits a local update data (`0043d410` with 0.0, 0, 0) and updates `object` with it
/// (`00a59c60`).
fn update_object(e: &mut Engine, object: u32) {
    e.with_stack(0xc, |e, data| {
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![data, x87_store32(0.0), 0u32, 0u32],
        );
        e.call(UPDATE_00A59C60, &args![object, data]);
    });
}

/// The terminal geometry the static holder at `011db63c` holds (`00559450`).
fn terminal_geometry(e: &mut Engine) -> u32 {
    e.call(HOLDER_GET_00559450, &args![HOLDER_011DB63C]).u32()
}

// Translated from 007ff650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::OnMenuOpen` (Xbox PDB): attaches the screen parent to the
/// shadow scene node (`00450b80(1)`, its slot +0xdc with it and 1) and updates it, shows
/// the light (`00450f90` with 0) when there is one, runs
/// `FORenderedMenu::OnMenuOpen`, clears `bFadingOut` and `fLastFadePercent`, and with a
/// terminal geometry sets its fade range to the value at `01017130` twice
/// (`BSFadeNode::SetRange`), then `0054b800` and `00476ae0(0)` on it.
pub fn fo_rendered_terminal_on_menu_open(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    let root = e.call(SCREEN_PARENT_005585E0, &args![this]).u32();
    let scene = e.call(TES_00450B80, &args![1u32]).u32();
    let again = e.call(SCREEN_PARENT_005585E0, &args![this]).u32();
    e.vcall(scene, 0xdc, &args![again, 1u32]);
    update_object(e, root);
    if e.call(MENU_LIGHT_006838B0, &args![this]).u32() != 0 {
        let node = e.call(MENU_LIGHT_006838B0, &args![this]).u32();
        e.call(SET_APP_CULLED_00450F90, &args![node, 0u32]);
    }
    e.call(RENDERED_MENU_ON_MENU_OPEN, &args![this]);
    e.set(this, FORenderedTerminal::bFadingOut, 0);
    e.mem.set_u32(this.addr() + 0xe8, x87_store32(0.0));
    if terminal_geometry(e) != 0 {
        let range = x87_quiet_bits(e.global::<u32>(OPEN_FADE_RANGE));
        let geometry = terminal_geometry(e);
        e.call(FADE_NODE_SET_RANGE, &args![geometry, range, range]);
        let geometry = terminal_geometry(e);
        e.call(GEOMETRY_0054B800, &args![geometry]);
        let geometry = terminal_geometry(e);
        e.call(GEOMETRY_00476AE0, &args![geometry, 0u32]);
    }
}

// Translated from 007ff740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the terminal's menu close): hides the light (`00450f90` with 1) and
/// updates it when there is one, detaches the screen parent from the shadow scene node
/// (`00450b80(1)`, its slot +0xe8) and runs `FORenderedMenu`'s close (`007fb0b0`).
pub fn fn_007ff740(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    if e.call(MENU_LIGHT_006838B0, &args![this]).u32() != 0 {
        let node = e.call(MENU_LIGHT_006838B0, &args![this]).u32();
        e.call(SET_APP_CULLED_00450F90, &args![node, 1u32]);
        e.with_stack(0xc, |e, data| {
            e.call(
                UPDATE_DATA_INIT_0043D410,
                &args![data, x87_store32(0.0), 0u32, 0u32],
            );
            let node = e.call(MENU_LIGHT_006838B0, &args![this]).u32();
            e.call(UPDATE_00A59C60, &args![node, data]);
        });
    }
    let scene = e.call(TES_00450B80, &args![1u32]).u32();
    let root = e.call(SCREEN_PARENT_005585E0, &args![this]).u32();
    e.vcall(scene, 0xe8, &args![root]);
    e.call(RENDERED_MENU_ON_MENU_CLOSE, &args![this]);
}

// Translated from 007ff7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0xc`; only the first word is read): puts the menu root back at
/// `OriginalPos` and updates it (when there is one), then renders the menu into the
/// texture (`007fba00` with the word, 1 and 0).
pub fn fn_007ff7c0(
    e: &mut Engine,
    this: Ptr<FORenderedTerminal>,
    texture: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
    let root = e.call(MENU_ROOT_0063D040, &args![this]).u32();
    if root != 0 {
        e.call(SET_POSITION_00440460, &args![root, this.addr() + 0xd8]);
        update_object(e, root);
    }
    e.call(RENDERED_MENU_RENDER, &args![this, texture, 1u32, 0u32]);
}

// Translated from 007ff820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::OnIdle` (Xbox PDB): while fading out, enters the rendered menu
/// (`InterfaceManager::EnterRenderedMenu(0)`) and answers true once the terminal geometry
/// is gone, no longer fading (`007ff8c0`) or its fade (`006d2c40`) equals the last one
/// (`FUCOMPP`); otherwise stores the new fade in `fLastFadePercent`. Then (and when not
/// fading out) answers `FORenderedMenu::OnIdle`.
pub fn fo_rendered_terminal_on_idle(e: &mut Engine, this: Ptr<FORenderedTerminal>) -> bool {
    if e.get(this, FORenderedTerminal::bFadingOut) != 0 {
        let done = terminal_geometry(e) == 0 || {
            let geometry = terminal_geometry(e);
            !fn_007ff8c0(e, Ptr::new(geometry)) || {
                let geometry = terminal_geometry(e);
                let fade = e.call(GEOMETRY_FADE_006D2C40, &args![geometry]).f64();
                // FUCOMPP; TEST AH,0x44; JP: the last fade unchanged (equal, not NaN).
                x87_load32(e.mem.u32(this.addr() + 0xe8)) == fade
            }
        };
        if done {
            let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
            e.call(ENTER_RENDERED_MENU, &args![manager, 0u32]);
            return true;
        }
        let geometry = terminal_geometry(e);
        let fade = e.call(GEOMETRY_FADE_006D2C40, &args![geometry]).f64();
        e.mem.set_u32(this.addr() + 0xe8, x87_store32(fade));
    }
    e.call(RENDERED_MENU_ON_IDLE, &args![this]).bool()
}

// Translated from 007ff900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): with a menu root, moves it to (the `float` at `01078bb0`, 0.0,
/// 0.0) (`004bc1f0`) and updates it.
pub fn fn_007ff900(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    let root = e.call(MENU_ROOT_0063D040, &args![this]).u32();
    if root == 0 {
        return;
    }
    let offset = x87_quiet_bits(e.global::<u32>(ROOT_OFFSET_01078BB0));
    e.call(
        SET_TRANSLATE_004BC1F0,
        &args![root, offset, x87_store32(0.0), x87_store32(0.0)],
    );
    update_object(e, root);
}

/// `FORenderedMenu::CreateMenuLight` (`007fc400`, `RET 8`), `"Omni01"` (`01078210`)
/// and its failure message (`01078a68`, printed by `005b5e40`).
const CREATE_MENU_LIGHT: u32 = 0x007f_c400;
const LIGHT_NODE_NAME: u32 = 0x0107_8210;
const NO_LIGHT_TEXT: u32 = 0x0107_8a68;
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// The point/color constructor from three `float`s (`00416870`, `RET 0xc`, returns
/// `this`) and the light's setters: `004bc280` (`RET 4`, a point), `0050c630`,
/// `0050c660`, `0050c690`, `0050dd20` (`RET 4`, a `float` each) and `0050dd50` (`RET 4`,
/// a point).
const POINT_FROM_FLOATS_00416870: u32 = 0x0041_6870;
const LIGHT_004BC280: u32 = 0x004b_c280;
const LIGHT_0050C630: u32 = 0x0050_c630;
const LIGHT_0050C660: u32 = 0x0050_c660;
const LIGHT_0050C690: u32 = 0x0050_c690;
const LIGHT_0050DD20: u32 = 0x0050_dd20;
const LIGHT_0050DD50: u32 = 0x0050_dd50;
/// A setting's value address (`00403e20`, `thiscall` on the setting) and the screen
/// light settings read here (`011db644`, `011db5f8`, `011db668`, `011db608`, `011db650`).
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// The menu's light color (`006b0570`, `RET 4`), `00965580` (`RET 4`, a `float`) and
/// `007f99b0` (`RET 4`, a `float`).
const MENU_LIGHT_COLOR_006B0570: u32 = 0x006b_0570;
const MENU_00965580: u32 = 0x0096_5580;
const MENU_007F99B0: u32 = 0x007f_99b0;

// Translated from 007ff960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::InitLight` (Xbox PDB): creates the menu light at `"Omni01"` in the
/// shadow scene node (`CreateMenuLight`; prints a failure message and stops when it
/// fails); with the light: a zero point (`004bc280`), 0.0 for `0050c630`, `0050c660` and
/// `0050c690`, 1.0 for `0050dd20`, the point (setting `011db644`, 0, 0) for `0050dd50`;
/// the color (settings `011db608`, `011db668`, `011db5f8`) for the menu (`006b0570`),
/// the setting `011db650` for `00965580`, 1.0 for `007f99b0`, then updates the light.
pub fn fo_rendered_terminal_init_light(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    let scene = e.call(TES_00450B80, &args![1u32]).u32();
    if !e
        .call(CREATE_MENU_LIGHT, &args![this, LIGHT_NODE_NAME, scene])
        .bool()
    {
        e.call(PRINT_MESSAGE, &args![NO_LIGHT_TEXT]);
        return;
    }
    let light = e.call(MENU_LIGHT_006838B0, &args![this]).u32();
    if light == 0 {
        return;
    }
    let zero = x87_store32(0.0);
    let setting = |e: &mut Engine, address: u32| {
        let value = e.call(SETTING_FLOAT_VALUE, &args![address]).u32();
        x87_quiet_bits(e.mem.u32(value))
    };
    // The frame below `EBP`: points at `EBP - 0x28`, `- 0x34` and `- 0x1c`, the update
    // data at `EBP - 0x10`.
    e.with_stack(0x30, |e, frame| {
        let first = frame.addr();
        let second = frame.addr() + 0xc;
        let color = frame.addr() + 0x18;
        let data = frame.addr() + 0x24;
        let point = e
            .call(POINT_FROM_FLOATS_00416870, &args![first, zero, zero, zero])
            .u32();
        e.call(LIGHT_004BC280, &args![light, point]);
        e.call(LIGHT_0050C630, &args![light, zero]);
        e.call(LIGHT_0050C660, &args![light, zero]);
        e.call(LIGHT_0050C690, &args![light, zero]);
        e.call(LIGHT_0050DD20, &args![light, x87_store32(1.0)]);
        let x = setting(e, 0x011d_b644);
        let point = e
            .call(POINT_FROM_FLOATS_00416870, &args![second, x, zero, zero])
            .u32();
        e.call(LIGHT_0050DD50, &args![light, point]);
        let blue = setting(e, 0x011d_b5f8);
        let green = setting(e, 0x011d_b668);
        let red = setting(e, 0x011d_b608);
        e.call(POINT_FROM_FLOATS_00416870, &args![color, red, green, blue]);
        e.call(MENU_LIGHT_COLOR_006B0570, &args![this, color]);
        let value = setting(e, 0x011d_b650);
        e.call(MENU_00965580, &args![this, value]);
        e.call(MENU_007F99B0, &args![this, x87_store32(1.0)]);
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![data, x87_store32(0.0), 0u32, 0u32],
        );
        e.call(UPDATE_00A59C60, &args![light, data]);
    });
}

// Translated from 007ffaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::FadeOut` (Xbox PDB): with a terminal geometry, sets its fade range
/// to 0.0 and 0.0 with the setting at `011f9148` cleared around it (`007ffb60(0)`, then
/// the old value back); sets `bFadingOut` and `fLastFadePercent` to 1.0. The 0 pushed for
/// `007ffb60` is left on the stack and becomes the range's first `float`.
pub fn fo_rendered_terminal_fade_out(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    if terminal_geometry(e) != 0 {
        let old = fn_007ffb80(e);
        fn_007ffb60(e, 0);
        let geometry = terminal_geometry(e);
        e.call(
            FADE_NODE_SET_RANGE,
            &args![geometry, x87_store32(0.0), x87_store32(0.0)],
        );
        fn_007ffb60(e, old);
    }
    e.set(this, FORenderedTerminal::bFadingOut, 1);
    e.mem.set_u32(this.addr() + 0xe8, x87_store32(1.0));
}

// Translated from 007ffb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): sets the setting at `011f9148` (`0045ce80`).
pub fn fn_007ffb60(e: &mut Engine, value: u32) {
    e.call(SETTING_SET_0045CE80, &args![SETTING_011F9148, value]);
}

// Translated from 007ffb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the word value of the setting at `011f9148` (`0043d4d0`).
pub fn fn_007ffb80(e: &mut Engine) -> u32 {
    let value = e
        .call(SETTING_VALUE_ADDRESS, &args![SETTING_011F9148])
        .u32();
    e.mem.u32(value)
}

// Translated from 007ffba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007ffba0 - the C++ exception-unwinding frame (FS:[0] around the local NiPick) is not translated.
/// Unnamed (`thiscall`, `RET 0xc`: a `bool` and two words; the terminal's click): with
/// an exit button and the flag, casts a pick (`NiPick(0, 8)`, `00458b30(0)`, the exit
/// button as its target, `006ecd40(1)`) along the camera's ray through the cursor
/// (`00a71080` with the truncated `009a9350` and `00792760` of the interface manager);
/// when the first record's object is the exit button, closes the terminal (`007ffd50`)
/// and answers false. Otherwise answers `FORenderedMenu`'s screen pick (`007fb790`)
/// with the three words.
pub fn fn_007ffba0(
    e: &mut Engine,
    this: Ptr<FORenderedTerminal>,
    flag: u8,
    first: u32,
    second: u32,
) -> bool {
    let base = this.addr();
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let has_button = e.call(HOLDER_GET_00559450, &args![base + 0xe4]).u32() != 0;
    if has_button && flag != 0 {
        // The frame below `EBP`: the points at `EBP - 0x58` and `- 0x64`, the pick at
        // `EBP - 0x4c` (to `EBP - 0x18`).
        let hit = e.with_stack(0x58, |e, frame| {
            let local = |offset: u32| frame.addr() + 0x68 - offset;
            let origin = local(0x58);
            let direction = local(0x64);
            let pick = local(0x4c);
            e.call(POINT_INIT_006815C0, &args![origin]);
            e.call(POINT_INIT_006815C0, &args![direction]);
            let mut camera = e.call(CAMERA_008B6200, &args![this]).u32();
            if camera == 0 {
                let object = e.call(GETTER_0045C670, &args![]).u32();
                camera = e.call(GETTER_006629F0, &args![object]).u32();
            }
            e.call(NI_PICK_CONSTRUCT, &args![pick, 0u32, 8u32]);
            e.call(PICK_00458B30, &args![pick, 0u32]);
            let button = e.call(HOLDER_GET_00559450, &args![base + 0xe4]).u32();
            e.call(PICK_SET_TARGET_00705FC0, &args![pick, button]);
            e.call(PICK_006ECD40, &args![pick, 1u32]);
            let x = e.call(CURSOR_00792760, &args![interface]).f64();
            let x = e.call(FTOL, &args![x]).u32();
            let y = e.call(CURSOR_009A9350, &args![interface]).f64();
            let y = e.call(FTOL, &args![y]).u32();
            e.call(
                CAMERA_WINDOW_POINT_TO_RAY,
                &args![camera, y, x, origin, direction, 0u32],
            );
            let mut hit = false;
            if e.call(NI_PICK_PICK_OBJECTS, &args![pick, origin, direction, 0u32])
                .bool()
            {
                let results = e.call(PICK_RESULTS_00500940, &args![pick]).u32();
                let record = e.call(PICK_RECORD_00968670, &args![results, 0u32]).u32();
                if record != 0 {
                    let object = e.call(RECORD_OBJECT_00458B50, &args![record]).u32();
                    let button = e.call(HOLDER_GET_00559450, &args![base + 0xe4]).u32();
                    if object == button {
                        fn_007ffd50(e, this);
                        hit = true;
                    }
                }
            }
            e.call(NI_PICK_DESTROY, &args![pick]);
            hit
        });
        if hit {
            return false;
        }
    }
    e.call(
        RENDERED_MENU_PICK,
        &args![this, u32::from(flag), first, second],
    )
    .bool()
}

// Translated from 007ffd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the terminal's close): plays the shutdown sound (`PlayShutdownSound`),
/// deletes the menu's object (`009611e0`, slot 0 with 1) when there is one, then
/// `00984f60(0)` and `InterfaceManager::EnterRenderedMenu(0)`.
pub fn fn_007ffd50(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    fo_rendered_terminal_play_shutdown_sound(e);
    let object = e.call(MENU_OBJECT_009611E0, &args![this]).u32();
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.call(MENU_00984F60, &args![this, 0u32]);
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    e.call(ENTER_RENDERED_MENU, &args![manager, 0u32]);
}

// Translated from 007ffdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: with a menu root, puts it at `OriginalPos` and updates it.
pub fn fn_007ffdb0(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    let root = e.call(MENU_ROOT_0063D040, &args![this]).u32();
    if root == 0 {
        return;
    }
    e.call(SET_POSITION_00440460, &args![root, this.addr() + 0xd8]);
    update_object(e, root);
}

// Translated from 007ffe40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007ffe40 - the C++ exception-unwinding frame (FS:[0] around the sound handle) is not translated.
/// `FORenderedTerminal::PlayShutdownSound` (Xbox PDB, no arguments): a sound handle by
/// name (`BSAudioManager::GetSoundHandleByName(QInstance(), handle,
/// "OBJComputerTerminalShutdown...", 0x121)`), played and destroyed. Four words are pushed
/// for `GetSoundHandleByName`, which takes three (`RET 0xc`): the last, 0, is the word of
/// `BSSoundHandle::Play` (`RET 4`). The frame keeps the saved security cookie
/// (`011c16bc ^ EBP`, at `EBP - 0x24`).
pub fn fo_rendered_terminal_play_shutdown_sound(e: &mut Engine) {
    // The frame below `EBP`: the handle at `EBP - 0x18`, the saved cookie at `EBP - 0x24`.
    e.with_stack(0x24, |e, frame| {
        let ebp = frame.addr() + 0x24;
        let cookie = e.global::<u32>(SECURITY_COOKIE) ^ ebp;
        e.mem.set_u32(ebp - 0x24, cookie);
        let handle = ebp - 0x18;
        let manager = e.call(AUDIO_MANAGER_INSTANCE, &args![]).u32();
        // 007ffe7c: `00ae5680` takes three words (RET 0xc); the pushed 0 is the word of
        // `00ad8830` (RET 4).
        let sound = e
            .call(
                AUDIO_MANAGER_SOUND_BY_NAME,
                &args![manager, handle, SHUTDOWN_SOUND, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

// ---------------------------------------------------------------------------
// Batch n059x: `FORenderedTerminal::Initialize`.

/// `007f9050` (`RET 8`: a word and a `float`; the setting `011db674`), the model loader
/// (`ModelLoader::LoadFile`, `00447080`, `RET 0x18`) and `SwapPlatformLanguageTextures`
/// (`004b7660`, cdecl).
const MENU_007F9050: u32 = 0x007f_9050;
const SETTING_011DB674: u32 = 0x011d_b674;
const MODEL_LOAD_FILE: u32 = 0x0044_7080;
const SWAP_LANGUAGE_TEXTURES: u32 = 0x004b_7660;
/// The fixed-string constructor (`00438170`, `RET 4`) and destructor (`004381b0`),
/// `"screen"` (`01078bc0`) and `"PowerButton"` (`01078bb4`); the child node getter
/// (`0043b4a0`, `RET 4`).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const SCREEN_NAME: u32 = 0x0107_8bc0;
const POWER_BUTTON_NAME: u32 = 0x0107_8bb4;
const CHILD_NODE_0043B4A0: u32 = 0x0043_b4a0;
/// The model's rotation: `NiMatrix3::MakeYRotation` (`0043f850`, `RET 4`), `004a0c90`
/// (`RET 4`), `NiMatrix3::operator*` (`0043f8d0`, `RET 8`) and the rotation setter
/// (`0043fa80`, `RET 4`) with 1.57 (`010559cc`).
const MAKE_Y_ROTATION: u32 = 0x0043_f850;
const MATRIX_004A0C90: u32 = 0x004a_0c90;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const SET_ROTATE_0043FA80: u32 = 0x0043_fa80;
const TERMINAL_ANGLE_010559CC: u32 = 0x0105_59cc;
/// The model's position settings (`011db65c`, `011db614`, `011db62c`), `00707b50` and
/// the byte `011f9426`, and 0.75 (`0101de30`, `double`).
const SETTING_011DB65C: u32 = 0x011d_b65c;
const SETTING_011DB614: u32 = 0x011d_b614;
const SETTING_011DB62C: u32 = 0x011d_b62c;
const INTERFACE_00707B50: u32 = 0x0070_7b50;
const FLAG_011F9426: u32 = 0x011f_9426;
const THREE_QUARTERS_DOUBLE: u32 = 0x0101_de30;
/// The camera: the allocator (`00aa13e0`, cdecl), `NiCamera`'s constructor (`00a712f0`),
/// the four-float set (`00414430`, `RET 0x10`), `00712e60` (`RET 4`), the matrix row
/// set (`007133b0`, `RET 0x10`), the rotation `011a9448`, `00733fd0` (`RET 4`),
/// `NiFrustum::NiFrustum` (`00a71b70`, `RET 4`), `0045c670` and its slot +0x100
/// (`ST0`), the setting `01203144`, `00507700` (`RET 4`), the screen width (the word
/// `011f9434` with the flag, else `004dc200`) and height (`004dc1f0`), 0.75
/// (`01016264`), the settings `01203150` and `011db5ec`, pi/180 (`01023128`,
/// `double`), `0057dd70` (cdecl, a `float`, `ST0`) and `NiCamera::SetViewFrustum`
/// (`00a6faf0`, `RET 4`).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const CAMERA_CONSTRUCT_00A712F0: u32 = 0x00a7_12f0;
const FOUR_FLOATS_00414430: u32 = 0x0041_4430;
const CAMERA_00712E60: u32 = 0x0071_2e60;
const MATRIX_ROW_007133B0: u32 = 0x0071_33b0;
const CAMERA_ROTATION_011A9448: u32 = 0x011a_9448;
const CAMERA_00733FD0: u32 = 0x0073_3fd0;
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
const SETTING_01203144: u32 = 0x0120_3144;
const CAMERA_00507700: u32 = 0x0050_7700;
const SCREEN_WIDTH_011F9434: u32 = 0x011f_9434;
const SCREEN_WIDTH_004DC200: u32 = 0x004d_c200;
const SCREEN_HEIGHT_004DC1F0: u32 = 0x004d_c1f0;
const FRUSTUM_SCALE_01016264: u32 = 0x0101_6264;
const SETTING_01203150: u32 = 0x0120_3150;
const SETTING_011DB5EC: u32 = 0x011d_b5ec;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
const ANGLE_0057DD70: u32 = 0x0057_dd70;
const SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
/// `FORenderedMenu::PrepareGeometry` (`007fc2d0`, `RET 4`), `007f9010` (`RET 4`),
/// `004dc540` (`RET 4`), `0040df90`, `NiAVObject::RemoveProperty` (`00a5b230`, `RET
/// 4`), the property's constructor (`004391c0`, 0x1c bytes),
/// `NiAVObject::AttachProperty` (`00439410`, `RET 4`), `007fd620` (`RET 4`),
/// `FORenderedMenu::Initialize` (`007fc150`), the root's translation (`0043c490`).
const PREPARE_GEOMETRY: u32 = 0x007f_c2d0;
const MENU_007F9010: u32 = 0x007f_9010;
const MENU_004DC540: u32 = 0x004d_c540;
const PROPERTY_0040DF90: u32 = 0x0040_df90;
const REMOVE_PROPERTY: u32 = 0x00a5_b230;
const PROPERTY_CONSTRUCT_004391C0: u32 = 0x0043_91c0;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const MENU_007FD620: u32 = 0x007f_d620;
const RENDERED_MENU_INITIALIZE: u32 = 0x007f_c150;
const TRANSLATE_0043C490: u32 = 0x0043_c490;
/// `NiAVObject::GetWorldBound` (`0043d450`), the bound's radius (`0084d030`, `ST0`), 3.0
/// (`01021928`, `double`), `ShadowSceneNode::AddAllLightsFromNode_Recurse`
/// (`00b5ca70`, `RET 8`).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
const BOUND_RADIUS_0084D030: u32 = 0x0084_d030;
const THREE_DOUBLE: u32 = 0x0102_1928;
const ADD_ALL_LIGHTS: u32 = 0x00b5_ca70;
/// `FORenderedMenu::SetScreenTexture` (`007fae70`, `RET 4`), `00810530` (`RET 4`),
/// `BSFadeNode::SetLODMultType` (`00b4dec0`, `RET 4`), `SystemColorManager::GetInstance`
/// (`00718b60`), `SystemColorManager::GetColor` (`00719060`, `RET 8`) and `007f7fb0`
/// (`RET 4`).
const SET_SCREEN_TEXTURE: u32 = 0x007f_ae70;
const MENU_00810530: u32 = 0x0081_0530;
const SET_LOD_MULT_TYPE: u32 = 0x00b4_dec0;
const SYSTEM_COLOR_MANAGER: u32 = 0x0071_8b60;
const SYSTEM_COLOR_GET: u32 = 0x0071_9060;
const MENU_007F7FB0: u32 = 0x007f_7fb0;

/// A setting's `float` (`00403e20`, then `FLD`).
fn setting_float(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_FLOAT_VALUE, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(value))
}

/// A named object of `model`: its slot +0x9c with a fixed string built (and destroyed
/// after) in the local at `local`.
fn named_object(e: &mut Engine, model: u32, name: u32, local: u32) -> u32 {
    let fixed = e.call(FIXED_STRING_CONSTRUCT, &args![local, name]).u32();
    let object = e.vcall(model, 0x9c, &args![fixed]).u32();
    e.call(FIXED_STRING_DESTRUCT, &args![local]);
    object
}

// Translated from 007feeb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007feeb0 - the C++ exception-unwinding frame (FS:[0] around the fixed strings and the allocations) is not translated.
/// `FORenderedTerminal::Initialize` (Xbox PDB, `thiscall`): `007f9050(1, setting
/// 011db674)`; the terminal model (the static holder `011db63c`, loaded and its textures
/// swapped when empty); its `"screen"` geometry (or the first child's) and its
/// `"PowerButton"` (or its first child) as the exit button (+0xe4); the model rotated
/// (1.57 about y), placed at the settings `011db65c` (times 0.75 unless `00707b50` or
/// the byte `011f9426`), `011db614` and `011db62c`, and updated; a new camera (0x114
/// bytes) set up with its frustum (near the setting `01203144`, far the slot +0x100 of
/// `0045c670()`, the sides from `0057dd70` of the settings `01203150` times `011db5ec`
/// in degrees, scaled by 0.75 and the screen's aspect); the geometry prepared and given
/// a new property (0x1c bytes, `004391c0`) for the one `0040df90` names; the rendered
/// menu initialised, the root's translation kept (+0xd8) and its x set (`01078bb0`); the
/// model's lights added to the shadow scene node (within three times its bound's
/// radius; without any, `InitLight`); the screen texture cleared, the fade node's LOD
/// type 8, the system color 3 applied when set (`007f7fb0`); the fade 1.0 (+0xe8).
pub fn fo_rendered_terminal_initialize(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    let base = this.addr();
    let value = setting_float(e, SETTING_011DB674);
    e.call(MENU_007F9050, &args![this, 1u32, value]);
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x180, |e, frame| {
        let ebp = frame.addr() + 0x180;
        let local = |offset: u32| ebp - offset;
        let model = if terminal_geometry(e) == 0 {
            let path = e.global::<u32>(TERMINAL_MODEL_PATH_011A0BB0);
            let loader = e.global::<u32>(MODEL_LOADER_011C3B3C);
            let model = e
                .call(
                    MODEL_LOAD_FILE,
                    &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
                )
                .u32();
            e.call(HOLDER_SET_0066B0D0, &args![HOLDER_011DB63C, model]);
            e.call(SWAP_LANGUAGE_TEXTURES, &args![model]);
            model
        } else {
            terminal_geometry(e)
        };
        let screen = named_object(e, model, SCREEN_NAME, local(0x100));
        let mut geometry = e.vcall(screen, 0x18, &args![]).u32();
        if geometry == 0 {
            let node = e.vcall(screen, 0xc, &args![]).u32();
            let child = e.call(CHILD_NODE_0043B4A0, &args![node, 0u32]).u32();
            geometry = e.vcall(child, 0x18, &args![]).u32();
        }
        let button = named_object(e, model, POWER_BUTTON_NAME, local(0x104));
        if e.vcall(button, 0x18, &args![]).u32() != 0 {
            e.call(HOLDER_SET_0066B0D0, &args![base + 0xe4, button]);
        } else {
            let node = e.vcall(button, 0xc, &args![]).u32();
            let child = e.call(CHILD_NODE_0043B4A0, &args![node, 0u32]).u32();
            e.call(HOLDER_SET_0066B0D0, &args![base + 0xe4, child]);
        }
        let (turn, tilt, product) = (local(0x58), local(0xf8), local(0x128));
        e.call(POINT_INIT_006815C0, &args![turn]);
        e.call(POINT_INIT_006815C0, &args![tilt]);
        let angle = x87_quiet_bits(e.global::<u32>(TERMINAL_ANGLE_010559CC));
        e.call(MAKE_Y_ROTATION, &args![turn, angle]);
        e.call(MATRIX_004A0C90, &args![tilt, angle]);
        let rotation = e.call(MATRIX_MULTIPLY, &args![tilt, product, turn]).u32();
        e.call(SET_ROTATE_0043FA80, &args![model, rotation]);
        let mut x = setting_float(e, SETTING_011DB65C);
        if !e.call(INTERFACE_00707B50, &args![]).bool() && e.mem.u8(FLAG_011F9426) == 0 {
            let scale: f64 = e.global(THREE_QUARTERS_DOUBLE);
            x = x87_store32(x87_mul(x87_load32(x), scale));
        }
        let z = setting_float(e, SETTING_011DB62C);
        let y = setting_float(e, SETTING_011DB614);
        e.call(
            SET_TRANSLATE_004BC1F0,
            &args![model, x87_quiet_bits(x), y, z],
        );
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![local(0x24), x87_store32(0.0), 0u32, 0u32],
        );
        e.call(UPDATE_00A59C60, &args![model, local(0x24)]);
        let block = e.call(ALLOCATE_00AA13E0, &args![0x114u32]).u32();
        let camera = if block != 0 {
            e.call(CAMERA_CONSTRUCT_00A712F0, &args![block]).u32()
        } else {
            0
        };
        let (zero, one) = (x87_store32(0.0), x87_store32(1.0));
        let viewport = local(0x34);
        e.call(FOUR_FLOATS_00414430, &args![viewport, zero, one, one, zero]);
        e.call(CAMERA_00712E60, &args![camera, viewport]);
        let rows = local(0xa4);
        e.call(POINT_INIT_006815C0, &args![rows]);
        e.call(MATRIX_ROW_007133B0, &args![rows, 0u32, zero, zero, one]);
        e.call(MATRIX_ROW_007133B0, &args![rows, 1u32, one, zero, zero]);
        e.call(MATRIX_ROW_007133B0, &args![rows, 2u32, zero, one, zero]);
        e.call(
            SET_ROTATE_0043FA80,
            &args![camera, CAMERA_ROTATION_011A9448],
        );
        e.call(CAMERA_00733FD0, &args![camera, one]);
        // The frustum at `EBP - 0xc4`: left, right, top, bottom, near (-0xb4), far
        // (-0xb0) and the orthographic byte (-0xac).
        let frustum = local(0xc4);
        e.call(FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
        let object = e.call(GETTER_0045C670, &args![]).u32();
        let far = e.vcall(object, 0x100, &args![]).f64();
        e.mem.set_u32(local(0xb0), x87_store32(far));
        let near = setting_float(e, SETTING_01203144);
        e.mem.set_u32(local(0xb4), near);
        let ratio = x87_store32(x87_div(
            x87_load32(e.mem.u32(local(0xb0))),
            x87_m32(e.mem.u32(local(0xb4))),
        ));
        e.call(CAMERA_00507700, &args![camera, x87_quiet_bits(ratio)]);
        let width = if e.mem.u8(FLAG_011F9426) != 0 {
            e.global::<i32>(SCREEN_WIDTH_011F9434)
        } else {
            e.call(SCREEN_WIDTH_004DC200, &args![]).i32()
        };
        let height = e.call(SCREEN_HEIGHT_004DC1F0, &args![]).i32();
        let aspect = x87_store32(x87_div(f64::from(width), f64::from(height)));
        let scale = x87_quiet_bits(e.global::<u32>(FRUSTUM_SCALE_01016264));
        let degrees = setting_float(e, SETTING_01203150);
        let radians = x87_mul(x87_load32(degrees), e.global::<f64>(DEGREES_TO_RADIANS));
        let factor = setting_float(e, SETTING_011DB5EC);
        let angle = x87_store32(x87_mul(x87_load32(factor), radians));
        let side = |e: &mut Engine| e.call(ANGLE_0057DD70, &args![x87_quiet_bits(angle)]).f64();
        let left = -side(e);
        e.mem
            .set_u32(frustum, x87_store32(x87_mul(left, x87_m32(scale))));
        let right = side(e);
        e.mem
            .set_u32(frustum + 4, x87_store32(x87_mul(right, x87_m32(scale))));
        let bottom = -side(e);
        let bottom = x87_mul(x87_mul(bottom, x87_m32(aspect)), x87_m32(scale));
        e.mem.set_u32(frustum + 0xc, x87_store32(bottom));
        let top = side(e);
        let top = x87_mul(x87_mul(top, x87_m32(aspect)), x87_m32(scale));
        e.mem.set_u32(frustum + 8, x87_store32(top));
        e.mem.set_u8(local(0xac), 0);
        e.call(SET_VIEW_FRUSTUM, &args![camera, frustum]);
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![local(0x18), x87_store32(0.0), 0u32, 0u32],
        );
        e.call(UPDATE_00A59C60, &args![camera, local(0x18)]);
        e.call(PREPARE_GEOMETRY, &args![this, geometry]);
        e.call(MENU_007F9010, &args![this, geometry]);
        e.call(MENU_004DC540, &args![this, model]);
        let removed = e.call(PROPERTY_0040DF90, &args![]).u32();
        e.call(REMOVE_PROPERTY, &args![geometry, removed]);
        let block = e.call(ALLOCATE_00AA13E0, &args![0x1cu32]).u32();
        let property = if block != 0 {
            e.call(PROPERTY_CONSTRUCT_004391C0, &args![block]).u32()
        } else {
            0
        };
        e.call(ATTACH_PROPERTY, &args![geometry, property]);
        e.call(MENU_007FD620, &args![this, camera]);
        e.call(RENDERED_MENU_INITIALIZE, &args![this]);
        let root = e.call(MENU_ROOT_0063D040, &args![this]).u32();
        let position = e.call(TRANSLATE_0043C490, &args![root]).u32();
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(base + 0xd8 + 4 * i, word);
        }
        let offset = x87_quiet_bits(e.global::<u32>(ROOT_OFFSET_01078BB0));
        let root = e.call(MENU_ROOT_0063D040, &args![this]).u32();
        e.call(SET_TRANSLATE_004BC1F0, &args![root, offset, zero, zero]);
        let bound = e.call(GET_WORLD_BOUND, &args![model]).u32();
        let radius = e.call(BOUND_RADIUS_0084D030, &args![bound]).f64();
        let reach = x87_store32(x87_mul(radius, e.global::<f64>(THREE_DOUBLE)));
        let scene = e.call(TES_00450B80, &args![1u32]).u32();
        let lights = e
            .call(ADD_ALL_LIGHTS, &args![scene, model, x87_quiet_bits(reach)])
            .u32();
        if lights == 0 {
            fo_rendered_terminal_init_light(e, this);
        }
        e.call(SET_SCREEN_TEXTURE, &args![this, 0u32]);
        e.call(MENU_00810530, &args![this, 0u32]);
        let fade = terminal_geometry(e);
        e.call(SET_LOD_MULT_TYPE, &args![fade, 8u32]);
        let color = local(0x80);
        e.call(FOUR_FLOATS_00414430, &args![color, zero, zero, zero, zero]);
        let manager = e.call(SYSTEM_COLOR_MANAGER, &args![]).u32();
        if e.call(SYSTEM_COLOR_GET, &args![manager, 3u32, color])
            .bool()
        {
            e.call(MENU_007F7FB0, &args![this, color]);
        }
    });
    e.set(this, FORenderedTerminal::fLastFadePercent, 1.0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007f_f8c0, fn_007ff8c0(Ptr) -> bool),
        entry!(0x007f_f8e0, fn_007ff8e0(Ptr) -> bool),
        entry!(0x007f_fad0, fn_007ffad0(Ptr) -> u32),
        entry!(0x007f_fe00, fo_rendered_terminal_release_static_geometry()),
        entry!(
            0x007f_f650,
            fo_rendered_terminal_on_menu_open(Ptr<FORenderedTerminal>)
        ),
        entry!(0x007f_f740, fn_007ff740(Ptr<FORenderedTerminal>)),
        entry!(
            0x007f_f7c0,
            fn_007ff7c0(Ptr<FORenderedTerminal>, u32, u32, u32)
        ),
        entry!(
            0x007f_f820,
            fo_rendered_terminal_on_idle(Ptr<FORenderedTerminal>) -> bool
        ),
        entry!(0x007f_f900, fn_007ff900(Ptr<FORenderedTerminal>)),
        entry!(
            0x007f_f960,
            fo_rendered_terminal_init_light(Ptr<FORenderedTerminal>)
        ),
        entry!(
            0x007f_faf0,
            fo_rendered_terminal_fade_out(Ptr<FORenderedTerminal>)
        ),
        entry!(0x007f_fb60, fn_007ffb60(u32)),
        entry!(0x007f_fb80, fn_007ffb80() -> u32),
        entry!(
            0x007f_fba0,
            fn_007ffba0(Ptr<FORenderedTerminal>, u8, u32, u32) -> bool
        ),
        entry!(0x007f_fd50, fn_007ffd50(Ptr<FORenderedTerminal>)),
        entry!(0x007f_fdb0, fn_007ffdb0(Ptr<FORenderedTerminal>)),
        entry!(0x007f_fe40, fo_rendered_terminal_play_shutdown_sound()),
        entry!(
            0x007f_eeb0,
            fo_rendered_terminal_initialize(Ptr<FORenderedTerminal>)
        ),
    ]
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_007ff8c0_returns_the_callees_al() {
        let mut e = Engine::new();
        e.register(0x007f_f8e0, |_, _| Ret {
            eax: 0x1234_5601,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f8c0, &args![0x0000_5000u32]).bool());
        assert_eq!(calls_to(&e, 0x007f_f8e0), vec![vec![0x5000]]);
        // Only AL counts: a non-zero upper part with AL clear is false.
        e.register(0x007f_f8e0, |_, _| Ret {
            eax: 0x1234_5600,
            ..Ret::default()
        });
        assert!(!e.call(0x007f_f8c0, &args![0x0000_5000u32]).bool());
    }
}

#[cfg(test)]
mod b069_tests {
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
    fn fn_007ff8e0_tests_the_flag_0x4000() {
        let mut e = Engine::new();
        e.register(FLAG_TEST_00456630, |_, a| {
            u32::from(a[1] == 0x4000).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f8e0, &args![0x5000u32]).bool());
        assert_eq!(calls_to(&e, FLAG_TEST_00456630), vec![vec![0x5000, 0x4000]]);
    }

    #[test]
    fn fn_007ffad0_returns_the_entry_at_index_1() {
        let mut e = Engine::new();
        e.register(TES_00450B80, |_, a| (0x100 + a[0]).into_ret());
        assert_eq!(e.call(0x007f_fad0, &args![0x5000u32]).u32(), 0x101);
    }

    #[test]
    fn release_static_geometry_releases_the_model_only_when_held() {
        let mut e = Engine::new();
        e.map(MODEL_LOADER_011C3B3C & !0xfff, 0x1000);
        e.map(TERMINAL_MODEL_PATH_011A0BB0 & !0xfff, 0x1000);
        e.set_global(MODEL_LOADER_011C3B3C, 0x6000u32);
        e.set_global(TERMINAL_MODEL_PATH_011A0BB0, 0x0107_8b80u32);
        e.register(HOLDER_GET_00559450, |_, _| 1u32.into_ret());
        e.register(HOLDER_SET_0066B0D0, |_, _| Ret::default());
        e.register(MODEL_LOADER_RELEASE_MODEL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_fe00, &args![]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_RELEASE_MODEL),
            vec![vec![0x6000, 0x0107_8b80, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, HOLDER_SET_0066B0D0),
            vec![vec![HOLDER_011DB63C, 0]]
        );
        e.register(HOLDER_GET_00559450, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_fe00, &args![]);
        assert!(calls_to(&e, MODEL_LOADER_RELEASE_MODEL).is_empty());
        assert_eq!(calls_to(&e, HOLDER_SET_0066B0D0).len(), 1);
    }
}

#[cfg(test)]
mod n051_tests {
    use super::*;

    const STUBBED: &[u32] = &[
        RENDERED_MENU_ON_IDLE,
        RENDERED_MENU_ON_MENU_OPEN,
        RENDERED_MENU_ON_MENU_CLOSE,
        RENDERED_MENU_PICK,
        RENDERED_MENU_RENDER,
        SCREEN_PARENT_005585E0,
        MENU_ROOT_0063D040,
        MENU_LIGHT_006838B0,
        SET_APP_CULLED_00450F90,
        UPDATE_DATA_INIT_0043D410,
        UPDATE_00A59C60,
        FADE_NODE_SET_RANGE,
        GEOMETRY_0054B800,
        GEOMETRY_00476AE0,
        GEOMETRY_FADE_006D2C40,
        SET_POSITION_00440460,
        SET_TRANSLATE_004BC1F0,
        INTERFACE_MANAGER,
        ENTER_RENDERED_MENU,
        SETTING_VALUE_ADDRESS,
        SETTING_SET_0045CE80,
        MENU_OBJECT_009611E0,
        MENU_00984F60,
        AUDIO_MANAGER_INSTANCE,
        AUDIO_MANAGER_SOUND_BY_NAME,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_DESTRUCT,
        POINT_INIT_006815C0,
        CAMERA_008B6200,
        GETTER_0045C670,
        GETTER_006629F0,
        NI_PICK_CONSTRUCT,
        PICK_00458B30,
        PICK_SET_TARGET_00705FC0,
        PICK_006ECD40,
        CURSOR_00792760,
        CURSOR_009A9350,
        FTOL,
        CAMERA_WINDOW_POINT_TO_RAY,
        NI_PICK_PICK_OBJECTS,
        PICK_RESULTS_00500940,
        PICK_RECORD_00968670,
        RECORD_OBJECT_00458B50,
        NI_PICK_DESTROY,
        TES_00450B80,
        HOLDER_GET_00559450,
        FLAG_TEST_00456630,
        CREATE_MENU_LIGHT,
        PRINT_MESSAGE,
        POINT_FROM_FLOATS_00416870,
        LIGHT_004BC280,
        LIGHT_0050C630,
        LIGHT_0050C660,
        LIGHT_0050C690,
        LIGHT_0050DD20,
        LIGHT_0050DD50,
        SETTING_FLOAT_VALUE,
        MENU_LIGHT_COLOR_006B0570,
        MENU_00965580,
        MENU_007F99B0,
    ];

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x0107_8000, 0x1000);
        e.map(0x011c_1000, 0x1000);
        e.set_global(OPEN_FADE_RANGE, 100_000.0f32);
        e.set_global(ROOT_OFFSET_01078BB0, 30_490.0f32);
        for &addr in STUBBED {
            e.register(addr, |_, _| Ret::default());
        }
        let slots: Vec<u32> = (0..0x80).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    fn terminal(e: &mut Engine) -> Ptr<FORenderedTerminal> {
        let this = Ptr::new(e.mem.alloc(0xf0));
        e.mem.set_u32(this.addr(), VTABLE);
        this
    }

    #[test]
    fn opening_attaches_the_root_and_resets_the_fade() {
        let mut e = engine();
        let this = terminal(&mut e);
        let scene = e.mem.alloc(4);
        e.mem.set_u32(scene, VTABLE);
        answer(&mut e, TES_00450B80, scene);
        answer(&mut e, SCREEN_PARENT_005585E0, 0x5000);
        answer(&mut e, MENU_LIGHT_006838B0, 0x5100);
        answer(&mut e, HOLDER_GET_00559450, 0x5200);
        e.set(this, FORenderedTerminal::bFadingOut, 1);
        e.call_log = Some(vec![]);
        e.call(0x007f_f650, &args![this]);
        assert_eq!(calls_to(&e, slot(0xdc)), vec![vec![scene, 0x5000, 1]]);
        assert_eq!(calls_to(&e, UPDATE_00A59C60)[0][0], 0x5000);
        assert_eq!(calls_to(&e, SET_APP_CULLED_00450F90), vec![vec![0x5100, 0]]);
        assert_eq!(
            calls_to(&e, RENDERED_MENU_ON_MENU_OPEN),
            vec![vec![this.addr()]]
        );
        assert_eq!(e.get(this, FORenderedTerminal::bFadingOut), 0);
        let range = 100_000.0f32.to_bits();
        assert_eq!(
            calls_to(&e, FADE_NODE_SET_RANGE),
            vec![vec![0x5200, range, range]]
        );
        assert_eq!(calls_to(&e, GEOMETRY_00476AE0), vec![vec![0x5200, 0]]);
    }

    #[test]
    fn closing_hides_the_node_and_detaches_the_root() {
        let mut e = engine();
        let this = terminal(&mut e);
        let scene = e.mem.alloc(4);
        e.mem.set_u32(scene, VTABLE);
        answer(&mut e, TES_00450B80, scene);
        answer(&mut e, SCREEN_PARENT_005585E0, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x007f_f740, &args![this]);
        assert!(calls_to(&e, SET_APP_CULLED_00450F90).is_empty());
        answer(&mut e, MENU_LIGHT_006838B0, 0x5100);
        e.call_log = Some(vec![]);
        e.call(0x007f_f740, &args![this]);
        assert_eq!(calls_to(&e, SET_APP_CULLED_00450F90), vec![vec![0x5100, 1]]);
        assert_eq!(calls_to(&e, UPDATE_00A59C60)[0][0], 0x5100);
        assert_eq!(calls_to(&e, slot(0xe8)), vec![vec![scene, 0x5000]]);
        assert_eq!(
            calls_to(&e, RENDERED_MENU_ON_MENU_CLOSE),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn the_light_goes_back_and_the_menu_renders() {
        let mut e = engine();
        let this = terminal(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007f_f7c0, &args![this, 0x7000u32, 9u32, 9u32]);
        assert!(calls_to(&e, SET_POSITION_00440460).is_empty());
        assert_eq!(
            calls_to(&e, RENDERED_MENU_RENDER),
            vec![vec![this.addr(), 0x7000, 1, 0]]
        );
        answer(&mut e, MENU_ROOT_0063D040, 0x5300);
        e.call_log = Some(vec![]);
        e.call(0x007f_f7c0, &args![this, 0x7000u32, 9u32, 9u32]);
        assert_eq!(
            calls_to(&e, SET_POSITION_00440460),
            vec![vec![0x5300, this.addr() + 0xd8]]
        );
        e.call_log = Some(vec![]);
        e.call(0x007f_fdb0, &args![this]);
        assert_eq!(calls_to(&e, SET_POSITION_00440460).len(), 1);
        assert_eq!(calls_to(&e, UPDATE_00A59C60)[0][0], 0x5300);
        e.call_log = Some(vec![]);
        e.call(0x007f_f900, &args![this]);
        assert_eq!(
            calls_to(&e, SET_TRANSLATE_004BC1F0),
            vec![vec![0x5300, 30_490.0f32.to_bits(), 0, 0]]
        );
    }

    #[test]
    fn idle_waits_for_the_fade_to_settle() {
        let mut e = engine();
        let this = terminal(&mut e);
        answer(&mut e, RENDERED_MENU_ON_IDLE, 1);
        assert!(e.call(0x007f_f820, &args![this]).bool());
        e.set(this, FORenderedTerminal::bFadingOut, 1);
        // No geometry: done.
        answer(&mut e, RENDERED_MENU_ON_IDLE, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f820, &args![this]).bool());
        assert_eq!(calls_to(&e, ENTER_RENDERED_MENU).len(), 1);
        // Still fading and changing: the fade is kept and the menu idles.
        answer(&mut e, HOLDER_GET_00559450, 0x5200);
        answer(&mut e, FLAG_TEST_00456630, 1);
        e.register(GEOMETRY_FADE_006D2C40, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007f_f820, &args![this]).bool());
        assert_eq!(e.get(this, FORenderedTerminal::fLastFadePercent), 0.5);
        assert!(calls_to(&e, ENTER_RENDERED_MENU).is_empty());
        // The same fade again: done.
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f820, &args![this]).bool());
        assert_eq!(calls_to(&e, ENTER_RENDERED_MENU).len(), 1);
    }

    #[test]
    fn fading_out_clears_the_setting_around_the_range() {
        let mut e = engine();
        let this = terminal(&mut e);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x33);
        answer(&mut e, SETTING_VALUE_ADDRESS, value);
        e.call_log = Some(vec![]);
        e.call(0x007f_faf0, &args![this]);
        assert!(calls_to(&e, FADE_NODE_SET_RANGE).is_empty());
        assert_eq!(e.get(this, FORenderedTerminal::bFadingOut), 1);
        assert_eq!(e.get(this, FORenderedTerminal::fLastFadePercent), 1.0);
        answer(&mut e, HOLDER_GET_00559450, 0x5200);
        e.call_log = Some(vec![]);
        e.call(0x007f_faf0, &args![this]);
        assert_eq!(
            calls_to(&e, SETTING_SET_0045CE80),
            vec![vec![SETTING_011F9148, 0], vec![SETTING_011F9148, 0x33]]
        );
        assert_eq!(calls_to(&e, FADE_NODE_SET_RANGE), vec![vec![0x5200, 0, 0]]);
        assert_eq!(e.call(0x007f_fb80, &args![]).u32(), 0x33);
    }

    #[test]
    fn a_click_on_the_exit_button_closes() {
        let mut e = engine();
        let this = terminal(&mut e);
        answer(&mut e, RENDERED_MENU_PICK, 1);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x007f_fba0, &args![this, 1u32, 0x10u32, 0x20u32])
            .bool());
        assert_eq!(
            calls_to(&e, RENDERED_MENU_PICK),
            vec![vec![this.addr(), 1, 0x10, 0x20]]
        );
        answer(&mut e, HOLDER_GET_00559450, 0x5400);
        answer(&mut e, NI_PICK_PICK_OBJECTS, 1);
        answer(&mut e, PICK_RECORD_00968670, 0x5500);
        answer(&mut e, RECORD_OBJECT_00458B50, 0x5400);
        answer(&mut e, CAMERA_008B6200, 0x5600);
        e.register(FTOL, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e.register(CURSOR_00792760, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        e.register(CURSOR_009A9350, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x007f_fba0, &args![this, 1u32, 0x10u32, 0x20u32])
            .bool());
        let ray = calls_to(&e, CAMERA_WINDOW_POINT_TO_RAY);
        assert_eq!(ray[0][..3], [0x5600, 4, 3]);
        assert_eq!(calls_to(&e, PICK_SET_TARGET_00705FC0)[0][1], 0x5400);
        assert!(calls_to(&e, RENDERED_MENU_PICK).is_empty());
        assert_eq!(calls_to(&e, MENU_00984F60), vec![vec![this.addr(), 0]]);
        assert_eq!(calls_to(&e, NI_PICK_DESTROY).len(), 1);
        // Without the flag the menu's pick answers.
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x007f_fba0, &args![this, 0u32, 0x10u32, 0x20u32])
            .bool());
        assert!(calls_to(&e, NI_PICK_CONSTRUCT).is_empty());
    }

    #[test]
    fn the_light_is_created_and_set_up() {
        let mut e = engine();
        let this = terminal(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007f_f960, &args![this]);
        assert_eq!(calls_to(&e, PRINT_MESSAGE), vec![vec![NO_LIGHT_TEXT]]);
        assert!(calls_to(&e, LIGHT_004BC280).is_empty());
        answer(&mut e, CREATE_MENU_LIGHT, 1);
        answer(&mut e, TES_00450B80, 0x5600);
        answer(&mut e, MENU_LIGHT_006838B0, 0x5700);
        e.register(POINT_FROM_FLOATS_00416870, |_, a| a[0].into_ret());
        // Each setting answers its own address as the value.
        e.register(SETTING_FLOAT_VALUE, |e, a| {
            let value = e.mem.alloc(4);
            e.mem.set_u32(value, a[0] & 0xffff);
            value.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x007f_f960, &args![this]);
        assert_eq!(
            calls_to(&e, CREATE_MENU_LIGHT),
            vec![vec![this.addr(), LIGHT_NODE_NAME, 0x5600]]
        );
        assert_eq!(
            calls_to(&e, LIGHT_0050DD20),
            vec![vec![0x5700, 1.0f32.to_bits()]]
        );
        let points = calls_to(&e, POINT_FROM_FLOATS_00416870);
        assert_eq!(points[1][1..], [0xb644, 0, 0]);
        assert_eq!(points[2][1..], [0xb608, 0xb668, 0xb5f8]);
        assert_eq!(
            calls_to(&e, MENU_LIGHT_COLOR_006B0570),
            vec![vec![this.addr(), points[2][0]]]
        );
        assert_eq!(calls_to(&e, MENU_00965580), vec![vec![this.addr(), 0xb650]]);
        assert_eq!(calls_to(&e, UPDATE_00A59C60)[0][0], 0x5700);
    }

    #[test]
    fn closing_plays_the_shutdown_sound_with_the_stack_word() {
        let mut e = engine();
        let this = terminal(&mut e);
        e.set_global(SECURITY_COOKIE, 0x1234_5678u32);
        answer(&mut e, AUDIO_MANAGER_SOUND_BY_NAME, 0x7700);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, VTABLE);
        answer(&mut e, MENU_OBJECT_009611E0, object);
        e.call_log = Some(vec![]);
        e.call(0x007f_fd50, &args![this]);
        let sound = calls_to(&e, AUDIO_MANAGER_SOUND_BY_NAME);
        assert_eq!(sound[0][2..], [SHUTDOWN_SOUND, 0x121]);
        let played = calls_to(&e, SOUND_HANDLE_PLAY);
        assert_eq!(played, vec![vec![0x7700, 0]]);
        assert_eq!(calls_to(&e, slot(0)), vec![vec![object, 1]]);
        assert_eq!(calls_to(&e, ENTER_RENDERED_MENU)[0][1], 0);
        e.call_log = Some(vec![]);
        e.call(0x007f_fb60, &args![5u32]);
        assert_eq!(
            calls_to(&e, SETTING_SET_0045CE80),
            vec![vec![SETTING_011F9148, 5]]
        );
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    const STUBBED: &[u32] = &[
        MENU_007F9050,
        MODEL_LOAD_FILE,
        HOLDER_SET_0066B0D0,
        SWAP_LANGUAGE_TEXTURES,
        FIXED_STRING_CONSTRUCT,
        FIXED_STRING_DESTRUCT,
        CHILD_NODE_0043B4A0,
        POINT_INIT_006815C0,
        MAKE_Y_ROTATION,
        MATRIX_004A0C90,
        MATRIX_MULTIPLY,
        SET_ROTATE_0043FA80,
        INTERFACE_00707B50,
        SET_TRANSLATE_004BC1F0,
        UPDATE_DATA_INIT_0043D410,
        UPDATE_00A59C60,
        ALLOCATE_00AA13E0,
        CAMERA_CONSTRUCT_00A712F0,
        FOUR_FLOATS_00414430,
        CAMERA_00712E60,
        MATRIX_ROW_007133B0,
        CAMERA_00733FD0,
        FRUSTUM_CONSTRUCT,
        GETTER_0045C670,
        CAMERA_00507700,
        SCREEN_WIDTH_004DC200,
        SCREEN_HEIGHT_004DC1F0,
        ANGLE_0057DD70,
        SET_VIEW_FRUSTUM,
        PREPARE_GEOMETRY,
        MENU_007F9010,
        MENU_004DC540,
        PROPERTY_0040DF90,
        REMOVE_PROPERTY,
        PROPERTY_CONSTRUCT_004391C0,
        ATTACH_PROPERTY,
        MENU_007FD620,
        RENDERED_MENU_INITIALIZE,
        MENU_ROOT_0063D040,
        TRANSLATE_0043C490,
        GET_WORLD_BOUND,
        BOUND_RADIUS_0084D030,
        TES_00450B80,
        ADD_ALL_LIGHTS,
        SET_SCREEN_TEXTURE,
        MENU_00810530,
        SET_LOD_MULT_TYPE,
        SYSTEM_COLOR_MANAGER,
        SYSTEM_COLOR_GET,
        MENU_007F7FB0,
        CREATE_MENU_LIGHT,
        PRINT_MESSAGE,
    ];

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    #[test]
    fn initialize_loads_the_model_and_sets_up_the_camera() {
        let mut e = Engine::new();
        for page in [
            0x011d_b000,
            0x011a_0000,
            0x011c_3000,
            0x011f_9000,
            0x0120_3000,
            0x0101_6000,
            0x0101_d000,
            0x0102_1000,
            0x0102_3000,
            0x0105_5000,
            0x0107_8000,
        ] {
            e.map(page, 0x1000);
        }
        for &addr in STUBBED {
            e.register(addr, |_, _| Ret::default());
        }
        let slots: Vec<u32> = (0..0x48).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e.register(HOLDER_GET_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(HOLDER_SET_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // Every setting's value is its own address plus 0x800.
        e.register(SETTING_FLOAT_VALUE, |_, a| (a[0] + 0x800).into_ret());
        for (setting, value) in [
            (SETTING_011DB674, 2.0f32),
            (SETTING_011DB65C, 8.0),
            (SETTING_011DB614, 3.0),
            (SETTING_011DB62C, 4.0),
            (SETTING_01203144, 5.0),
            (SETTING_01203150, 90.0),
            (SETTING_011DB5EC, 0.5),
        ] {
            e.set_global(setting + 0x800, value);
        }
        e.set_global(TERMINAL_ANGLE_010559CC, 1.57f32);
        e.set_global(THREE_QUARTERS_DOUBLE, 0.75f64);
        e.set_global(FRUSTUM_SCALE_01016264, 0.75f32);
        e.set_global(DEGREES_TO_RADIANS, std::f64::consts::PI / 180.0);
        e.set_global(THREE_DOUBLE, 3.0f64);
        e.set_global(ROOT_OFFSET_01078BB0, 30_490.0f32);
        let model = e.mem.alloc(0x10);
        e.mem.set_u32(model, VTABLE);
        answer(&mut e, MODEL_LOAD_FILE, model);
        // The model's named objects are geometry (slot +0x18 answers non-zero).
        let named = e.mem.alloc(0x10);
        e.mem.set_u32(named, VTABLE);
        answer(&mut e, slot(0x9c), named);
        answer(&mut e, slot(0x18), 0x6600);
        let camera = e.mem.alloc(0x114);
        answer(&mut e, ALLOCATE_00AA13E0, camera);
        e.register(CAMERA_CONSTRUCT_00A712F0, |_, a| a[0].into_ret());
        let getter = e.mem.alloc(0x10);
        e.mem.set_u32(getter, VTABLE);
        answer(&mut e, GETTER_0045C670, getter);
        st0(&mut e, slot(0x100), 1000.0);
        answer(&mut e, SCREEN_WIDTH_004DC200, 1600);
        answer(&mut e, SCREEN_HEIGHT_004DC1F0, 800);
        st0(&mut e, ANGLE_0057DD70, 2.0);
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position, 7.0);
        answer(&mut e, TRANSLATE_0043C490, position);
        answer(&mut e, MENU_ROOT_0063D040, 0x5500);
        st0(&mut e, BOUND_RADIUS_0084D030, 10.0);
        answer(&mut e, ADD_ALL_LIGHTS, 1);
        let this = Ptr::new(e.mem.alloc(0xf0));
        e.mem.set_u32(this.addr(), VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x007f_eeb0, &args![this]);
        assert_eq!(
            calls_to(&e, MENU_007F9050),
            vec![vec![this.addr(), 1, 2.0f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(HOLDER_011DB63C), model);
        assert_eq!(calls_to(&e, SWAP_LANGUAGE_TEXTURES), vec![vec![model]]);
        assert_eq!(e.mem.u32(this.addr() + 0xe4), named);
        // x scaled by 0.75 (no `00707b50`, no flag), y and z from the settings.
        assert_eq!(
            calls_to(&e, SET_TRANSLATE_004BC1F0)[0],
            vec![model, 6.0f32.to_bits(), 3.0f32.to_bits(), 4.0f32.to_bits()]
        );
        assert_eq!(
            calls_to(&e, CAMERA_00507700),
            vec![vec![camera, 200.0f32.to_bits()]]
        );
        // The frustum: the sides from 2.0, scaled by 0.75 and the 2.0 aspect.
        let frustum = calls_to(&e, SET_VIEW_FRUSTUM)[0][1];
        let side = |offset: u32| e.mem.f32(frustum + offset);
        assert_eq!(
            [side(0), side(4), side(8), side(0xc)],
            [-1.5, 1.5, 3.0, -3.0]
        );
        assert_eq!([side(0x10), side(0x14)], [5.0, 1000.0]);
        assert_eq!(e.mem.f32(this.addr() + 0xd8), 7.0);
        assert_eq!(
            calls_to(&e, SET_TRANSLATE_004BC1F0)[1],
            vec![0x5500, 30_490.0f32.to_bits(), 0, 0]
        );
        assert_eq!(calls_to(&e, ADD_ALL_LIGHTS)[0][2], 30.0f32.to_bits());
        assert!(calls_to(&e, CREATE_MENU_LIGHT).is_empty());
        assert_eq!(calls_to(&e, SET_LOD_MULT_TYPE), vec![vec![model, 8]]);
        assert!(calls_to(&e, MENU_007F7FB0).is_empty());
        assert_eq!(e.get(this, FORenderedTerminal::fLastFadePercent), 1.0);
    }
}
