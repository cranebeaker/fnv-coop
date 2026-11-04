//! `fallout/interface/fopipboymanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007fa200 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `007f9840` (Ghidra name
/// `FOPipboyManager::InitLight`), which creates the menu light `Omni01` and sets its
/// colours. The engine map names this address `FOPipboyManager::ReInitialize` from the call
/// graph only (row `cg`), which this translation does not confirm, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `007f9840`'s disassembly ends with `CALL 007f99b0` (which stores a
/// float and ends `RET 4`) and no result is computed (Ghidra types it void).
pub fn fn_007fa200(e: &mut Engine, this: Ptr) {
    e.call(0x007f_9840, &args![this]);
}

/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;
/// The player's 3D root (`this` = player, flag).
const PLAYER_3D: u32 = 0x0095_0bb0;
/// `NiFixedString` constructor (`this`, text; answers the string) and
/// destructor.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// The checked cast to an RTTI type (cdecl: type record, object).
const NI_CAST: u32 = 0x0065_3270;
/// The RTTI records the code casts with (`NiNode` and `NiGeometry` by how
/// the results are used; not confirmed).
const RTTI_011F4428: u32 = 0x011f_4428;
const RTTI_011F4ACC: u32 = 0x011f_4acc;
/// A child of a node (`this`, index).
const CHILD_AT: u32 = 0x0045_bc00;
/// The object's parent (`this` only).
const PARENT_OF: u32 = 0x0096_11e0;
/// `NiPointer` assignment (`this` = the pointer, object).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The pointer's object (`this` = the pointer).
const POINTER_GET: u32 = 0x0055_9450;
/// `NiAVObject::SetAppCulled`-shaped call (`this`, flag).
const SET_CULLED: u32 = 0x0045_0f90;
/// The Pip-Boy state test (`this` only; a byte).
const PIPBOY_STATE: u32 = 0x0097_4d90;
/// A setting's bool address (`ECX` = the setting).
const SETTING_BOOL_ADDRESS: u32 = 0x0040_8d60;
/// A setting's value address (`ECX` = the setting).
const SETTING_VALUE_ADDRESS: u32 = 0x0040_3e20;
/// `FORenderedMenu::Initialize`, `PrepareGeometry` and `CreateMenuLight`.
const RENDERED_MENU_INITIALIZE: u32 = 0x007f_c150;
const RENDERED_MENU_PREPARE_GEOMETRY: u32 = 0x007f_c2d0;
const RENDERED_MENU_CREATE_LIGHT: u32 = 0x007f_c400;
/// The interface manager singleton getter and the object `007fa950`
/// answers for it.
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
const INTERFACE_OBJECT_007FA950: u32 = 0x007f_a950;
/// Stores an object at +0x14 of the rendered menu (`run_00705fc0`).
const SET_MEMBER_0X14: u32 = 0x0070_5fc0;
/// A call with the player's 3D root (`this`, root).
const USE_PLAYER_ROOT: u32 = 0x004d_c540;
/// `NiUpdateData` constructor (`this`, time, two flags).
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// The node's update (`this`, update data).
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `NiAVObject::UpdateProperties`.
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `NiColorA` constructor (`this`, four floats).
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
/// `SystemColorManager::GetInstance` (no arguments) and `GetColor`
/// (`this`, index, colour).
const SYSTEM_COLORS: u32 = 0x0071_8b60;
const SYSTEM_COLOR: u32 = 0x0071_9060;
/// A float setter of the menu (`this`, float).
const SETTER_00444020: u32 = 0x0044_4020;
/// A byte setter of the menu (`this`, flag).
const SET_FLAG_00810530: u32 = 0x0081_0530;
/// A geometry's property list entry (`this`, index).
const PROPERTY_AT: u32 = 0x0043_b4a0;
/// The shared body that answers `this` (here constructing two `NiMatrix3`
/// temporaries; it does not write them).
const ANSWER_THIS: u32 = 0x0068_15c0;
/// The local rotation of an object (`this` only; nine floats).
const LOCAL_ROTATION: u32 = 0x006a_9540;
/// `NiMatrix3` product (`this`, result, other).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// Sets an object's rotation (`this`, matrix).
const SET_ROTATION: u32 = 0x0043_fa80;
/// Sets an angle about an axis (`this`, angle, x, y, z).
const SET_AXIS_ANGLE: u32 = 0x0064_04b0;
/// A float of the interface (`007ddef0`, no arguments; ST0).
const INTERFACE_VALUE_007DDEF0: u32 = 0x007d_def0;
/// `NiPoint3` constructor (`this`, x, y, z).
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// The menu light (`this` only).
const MENU_LIGHT: u32 = 0x0068_38b0;
/// The light's setters.
const LIGHT_SET_AMBIENT: u32 = 0x004b_c2e0;
const LIGHT_SET_DIFFUSE: u32 = 0x004b_c280;
const LIGHT_SET_0050C630: u32 = 0x0050_c630;
const LIGHT_SET_0050C660: u32 = 0x0050_c660;
const LIGHT_SET_0050C690: u32 = 0x0050_c690;
const LIGHT_SET_DIMMER: u32 = 0x0050_dd20;
const LIGHT_SET_0050DD50: u32 = 0x0050_dd50;
/// The menu's screen light colour and intensity setters.
const SET_SCREEN_LIGHT_COLOR: u32 = 0x006b_0570;
const SET_SCREEN_LIGHT_BASE: u32 = 0x0096_5580;
/// Tests and property calls of `ShowPipboyLightEffect`.
const GEOMETRY_TEST_00453470: u32 = 0x0045_3470;
const PROPERTY_KIND_004AF350: u32 = 0x004a_f350;
const PROPERTY_KIND_00438220: u32 = 0x0043_8220;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_SET_0068C9F0: u32 = 0x0068_c9f0;
const SHADER_PROPERTY_SET_ALPHA: u32 = 0x00ba_8ab0;

/// The node names `Initialize`, `InitButtons`, `InitKnobs`, `InitLight`
/// and `ShowPipboyLightEffect` look up.
const PIPBOY_NAME_ONE: u32 = 0x0107_8140;
const PIPBOY_NAME_TWO: u32 = 0x0107_8130;
const LIGHT_NAME_ONE: u32 = 0x0107_778c;
const LIGHT_NAME_TWO: u32 = 0x0107_826c;
const MENU_LIGHT_NAME_ONE: u32 = 0x0107_8210;
const MENU_LIGHT_NAME_TWO: u32 = 0x0107_8204;
/// The settings `Initialize` reads.
const SETTING_ENABLED: u32 = 0x011d_b2cc;
const SETTING_BLUR: u32 = 0x011d_b2dc;
const SETTING_00444020: u32 = 0x011d_b328;
const SETTING_SCANLINES: u32 = 0x011d_b274;
const SETTING_LIGHT_RANGE: u32 = 0x011d_b514;
const SETTING_FIRST_TAB: u32 = 0x011d_b33c;
const SETTING_LAST_TAB: u32 = 0x011d_b34c;
/// `double` constants of `InitKnobs`.
const TAB_STEPS: u32 = 0x0101_db80;
const WHEEL_SCALE: u32 = 0x0101_ff40;
const WHEEL_OFFSET: u32 = 0x0106_4f50;

layout! {
    /// `FOPipboyManager` (Xbox PDB), 0x170 bytes; the fields this unit
    /// uses, at the PC offsets the code reads (the PDB's).
    pub struct FOPipboyManager: 0x170 {
        /// `ScreenLightColor`'s neighbour `fScreenLightFlickerIntensity`
        /// (Xbox PDB `FORenderedMenu`).
        0x3C fScreenLightFlickerIntensity: u32,
        /// `fBlurIntensity` (Xbox PDB `FORenderedMenu`).
        0x44 fBlurIntensity: u32,
        /// `fScanlineScale` (Xbox PDB `FORenderedMenu`).
        0x48 fScanlineScale: u32,
        /// `bDoScanlines` (Xbox PDB `FORenderedMenu`).
        0x4C bDoScanlines: u8,
        /// `b1stPersonLightOn` (Xbox PDB).
        0x160 b1stPersonLightOn: u8,
        /// `b3rdPersonLightOn` (Xbox PDB).
        0x161 b3rdPersonLightOn: u8,
    }
}

/// Offsets of `BackgroundColor`, `TintColor` (Xbox PDB `FORenderedMenu`),
/// `spPipboyRoot`, `sp3rdPersonPipboyRoot`, `sp3rdPersonScreen`,
/// `spButtons`, `spButtonGlow`, `spKnobs`, `fKnobPositions`,
/// `fKnobTargets`, `fKnobRates` and `fTabKnobPositions` (Xbox PDB; the
/// arrays hold three entries, five for the tab positions).
const BACKGROUND_COLOR: u32 = 0xa8;
const TINT_COLOR: u32 = 0xc0;
const PIPBOY_ROOT: u32 = 0xd8;
const THIRD_PERSON_ROOT: u32 = 0xdc;
const THIRD_PERSON_SCREEN: u32 = 0xe0;
const BUTTONS: u32 = 0xe8;
const BUTTON_GLOW: u32 = 0xf4;
const KNOBS: u32 = 0x100;
const KNOB_POSITIONS: u32 = 0x10c;
const KNOB_TARGETS: u32 = 0x118;
const KNOB_RATES: u32 = 0x124;
const TAB_KNOB_POSITIONS: u32 = 0x130;

/// The object named `name` under the player's 3D root (its slot `+0x9c`
/// with a temporary `NiFixedString`), handed to `f` while the string
/// lives, as the code orders its calls.
fn with_named<R>(e: &mut Engine, name: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    let player = e.global::<u32>(PLAYER);
    let root = e.call(PLAYER_3D, &args![player, 1u32]).u32();
    e.with_stack(4, |e, text| {
        let key = e.call(FIXED_STRING_CONSTRUCT, &args![text, name]).u32();
        let found = e.vcall(root, 0x9c, &args![key]).u32();
        let result = f(e, found);
        e.call(FIXED_STRING_DESTRUCT, &args![text]);
        result
    })
}

/// [`with_named`] answering the object.
fn named(e: &mut Engine, name: u32) -> u32 {
    with_named(e, name, |_, found| found)
}

/// [`with_named`] answering the object cast to the RTTI record.
fn named_cast(e: &mut Engine, name: u32, rtti: u32) -> u32 {
    with_named(e, name, |e, found| {
        e.call(NI_CAST, &args![rtti, found]).u32()
    })
}

/// [`with_named`] storing the object in the pointer at `slot`.
fn assign_named(e: &mut Engine, slot: u32, name: u32) {
    with_named(e, name, |e, found| {
        e.call(POINTER_ASSIGN, &args![slot, found]);
    });
}

/// The object of the pointer at `slot`.
fn pointer(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_GET, &args![slot]).u32()
}

/// A setting's float, as the bit pattern an `FLD`/`FSTP` copy gives.
fn setting_float_bits(e: &mut Engine, setting: u32) -> u32 {
    let address = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(address))
}

fn copy_color(e: &mut Engine, this: Ptr<FOPipboyManager>, offset: u32, color: u32) {
    for i in 0..4 {
        let word = e.mem.u32(color.wrapping_add(4 * i));
        e.mem
            .set_u32(this.addr().wrapping_add(offset + 4 * i), word);
    }
}

// Translated from 007f7fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four words of the colour at `color` into `TintColor` (+0xc0,
/// `FORenderedMenu`, Xbox PDB).
pub fn fn_007f7fb0(e: &mut Engine, this: Ptr<FOPipboyManager>, color: u32) {
    copy_color(e, this, TINT_COLOR, color);
}

// Translated from 007f7fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four words of the colour at `color` into `BackgroundColor`
/// (+0xa8, `FORenderedMenu`, Xbox PDB).
pub fn fn_007f7fe0(e: &mut Engine, this: Ptr<FOPipboyManager>, color: u32) {
    copy_color(e, this, BACKGROUND_COLOR, color);
}

// Translated from 007f86f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets knob `index`'s target (`fKnobTargets`, +0x118) and rate
/// (`fKnobRates`, +0x124) from the two floats (passed as bit patterns).
pub fn fn_007f86f0(e: &mut Engine, this: Ptr<FOPipboyManager>, index: u32, target: u32, rate: u32) {
    let at = this.addr().wrapping_add(index.wrapping_mul(4));
    e.mem
        .set_u32(at.wrapping_add(KNOB_TARGETS), x87_quiet_bits(target));
    e.mem
        .set_u32(at.wrapping_add(KNOB_RATES), x87_quiet_bits(rate));
}

// Translated from 007f8ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::Initialize` (Xbox PDB). With the setting `011db2cc`
/// set, the Pip-Boy state test false and a player with a 3D root: finds the
/// Pip-Boy node (`01078140`, else `01078130`, as `011f4428`), initializes
/// the rendered menu, stores the interface's object, takes the Pip-Boy
/// root and screen (from the node's first child, or the `011f4acc` object
/// named `01078130`), the third-person root and screen, prepares both
/// screens' geometry, initializes the buttons, knobs and light, updates
/// both roots (time 0.0), takes the background and tint colours from
/// system colour 4 when there is one, reads three settings into the blur,
/// `00444020` and scanline values, calls `00810530(1)` and turns both
/// Pip-Boy lights off without storing the state.
// Translation incomplete: 007f8ba0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary strings is not translated
pub fn fo_pipboy_manager_initialize(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    let enabled = e.call(SETTING_BOOL_ADDRESS, &args![SETTING_ENABLED]).u32();
    if e.mem.u8(enabled) == 0 {
        return;
    }
    if e.call(PIPBOY_STATE, &args![this]).u8() != 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    if player == 0 || e.call(PLAYER_3D, &args![player, 1u32]).u32() == 0 {
        return;
    }
    let mut third_screen = 0u32;
    let mut pipboy = named_cast(e, PIPBOY_NAME_ONE, RTTI_011F4428);
    if pipboy == 0 {
        pipboy = named_cast(e, PIPBOY_NAME_TWO, RTTI_011F4428);
    }
    e.call(RENDERED_MENU_INITIALIZE, &args![this]);
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let object = e.call(INTERFACE_OBJECT_007FA950, &args![interface]).u32();
    e.call(SET_MEMBER_0X14, &args![this, object]);
    let screen = if pipboy != 0 {
        let screen = e.call(CHILD_AT, &args![pipboy, 0u32]).u32();
        let parent = e.call(PARENT_OF, &args![pipboy]).u32();
        e.call(POINTER_ASSIGN, &args![this.addr() + PIPBOY_ROOT, parent]);
        screen
    } else {
        let screen = named_cast(e, PIPBOY_NAME_TWO, RTTI_011F4ACC);
        if screen != 0 {
            let parent = e.call(PARENT_OF, &args![screen]).u32();
            e.call(POINTER_ASSIGN, &args![this.addr() + PIPBOY_ROOT, parent]);
        }
        screen
    };
    let player = e.global::<u32>(PLAYER);
    let root = e.call(PLAYER_3D, &args![player, 1u32]).u32();
    e.call(USE_PLAYER_ROOT, &args![this, root]);
    if pipboy != 0 {
        let child = e.call(CHILD_AT, &args![pipboy, 0u32]).u32();
        third_screen = e.call(NI_CAST, &args![RTTI_011F4ACC, child]).u32();
        let parent = e.call(PARENT_OF, &args![pipboy]).u32();
        e.call(
            POINTER_ASSIGN,
            &args![this.addr() + THIRD_PERSON_ROOT, parent],
        );
        e.call(
            POINTER_ASSIGN,
            &args![this.addr() + THIRD_PERSON_SCREEN, third_screen],
        );
    } else if screen != 0 {
        third_screen = screen;
        let parent = e.call(PARENT_OF, &args![screen]).u32();
        e.call(
            POINTER_ASSIGN,
            &args![this.addr() + THIRD_PERSON_ROOT, parent],
        );
        e.call(
            POINTER_ASSIGN,
            &args![this.addr() + THIRD_PERSON_SCREEN, third_screen],
        );
    }
    if screen != 0 && third_screen != 0 {
        e.call(RENDERED_MENU_PREPARE_GEOMETRY, &args![this, screen]);
        e.call(RENDERED_MENU_PREPARE_GEOMETRY, &args![this, third_screen]);
        fn_007f9010(e, this, screen);
    }
    fo_pipboy_manager_init_buttons(e, this);
    fo_pipboy_manager_init_knobs(e, this);
    fo_pipboy_manager_init_light(e, this);
    for slot in [PIPBOY_ROOT, THIRD_PERSON_ROOT] {
        if pointer(e, this.addr() + slot) != 0 {
            e.with_stack(0xc, |e, update| {
                e.call(UPDATE_DATA_CONSTRUCT, &args![update, 0.0f32, 0u32, 0u32]);
                let node = pointer(e, this.addr() + slot);
                e.call(NODE_UPDATE, &args![node, update]);
                let node = pointer(e, this.addr() + slot);
                e.call(UPDATE_PROPERTIES, &args![node]);
            });
        }
    }
    e.with_stack(0x10, |e, color| {
        e.call(
            COLOR_CONSTRUCT,
            &args![color, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let colors = e.call(SYSTEM_COLORS, &args![]).u32();
        if e.call(SYSTEM_COLOR, &args![colors, 4u32, color]).bool() {
            fn_007f7fe0(e, this, color.addr());
            fn_007f7fb0(e, this, color.addr());
        }
    });
    let blur = setting_float_bits(e, SETTING_BLUR);
    fn_007f9030(e, this, blur);
    let value = setting_float_bits(e, SETTING_00444020);
    e.call(SETTER_00444020, &args![this, value]);
    let scale = setting_float_bits(e, SETTING_SCANLINES);
    fn_007f9050(e, this, 1, scale);
    e.call(SET_FLAG_00810530, &args![this, 1u32]);
    fo_pipboy_manager_show_pipboy_light_effect(e, this, 0, 0, 1);
    fo_pipboy_manager_show_pipboy_light_effect(e, this, 1, 0, 1);
}

// Translated from 007f9010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `object` in the pointer at +4 (`spScreenTriShape`, Xbox PDB
/// `FORenderedMenu`) through the `NiPointer` assignment `0066b0d0`.
pub fn fn_007f9010(e: &mut Engine, this: Ptr<FOPipboyManager>, object: u32) {
    e.call(POINTER_ASSIGN, &args![this.byte_add(4), object]);
}

// Translated from 007f9030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fBlurIntensity` (+0x44, Xbox PDB `FORenderedMenu`) from the float
/// (passed as its bit pattern).
pub fn fn_007f9030(e: &mut Engine, this: Ptr<FOPipboyManager>, value: u32) {
    e.set(this, FOPipboyManager::fBlurIntensity, x87_quiet_bits(value));
}

// Translated from 007f9050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bDoScanlines` (+0x4c) and `fScanlineScale` (+0x48, Xbox PDB
/// `FORenderedMenu`; the float passed as its bit pattern).
pub fn fn_007f9050(e: &mut Engine, this: Ptr<FOPipboyManager>, on: u8, scale: u32) {
    e.set(this, FOPipboyManager::bDoScanlines, on);
    e.set(this, FOPipboyManager::fScanlineScale, x87_quiet_bits(scale));
}

// Translated from 007f9070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::InitButtons` (Xbox PDB): with the buttons node
/// (`010781f4`, else `010781e0`) present, stores the first child of each
/// of the three button nodes (each name tried, then its fallback, cast as
/// `011f4428`) in `spButtons`; with the glow node (`0107818c`, else
/// `01078180`) present, stores the three glow objects in `spButtonGlow`
/// (each with a fallback name) and hides them (`007fa010(0)`).
// Translation incomplete: 007f9070 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary strings is not translated
pub fn fo_pipboy_manager_init_buttons(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    let mut found = named(e, 0x0107_81f4);
    if found == 0 {
        found = named(e, 0x0107_81e0);
    }
    if found != 0 {
        for (index, (name, fallback)) in [
            (0x0107_81f4u32, 0x0107_81e0u32),
            (0x0107_81d0, 0x0107_81bc),
            (0x0107_81ac, 0x0107_8198),
        ]
        .into_iter()
        .enumerate()
        {
            let mut node = named_cast(e, name, RTTI_011F4428);
            if node == 0 {
                node = named_cast(e, fallback, RTTI_011F4428);
            }
            let child = e.call(CHILD_AT, &args![node, 0u32]).u32();
            let slot = this.addr() + BUTTONS + 4 * index as u32;
            e.call(POINTER_ASSIGN, &args![slot, child]);
        }
    }
    let mut glow = named(e, 0x0107_818c);
    if glow == 0 {
        glow = named(e, 0x0107_8180);
    }
    if glow == 0 {
        return;
    }
    for (index, (name, fallback)) in [
        (0x0107_818cu32, 0x0107_8180u32),
        (0x0107_8174, 0x0107_8168),
        (0x0107_815c, 0x0107_8150),
    ]
    .into_iter()
    .enumerate()
    {
        let slot = this.addr() + BUTTON_GLOW + 4 * index as u32;
        assign_named(e, slot, name);
        if pointer(e, slot) == 0 {
            assign_named(e, slot, fallback);
        }
    }
    fn_007fa010(e, this, 0);
}

// Translated from 007f9840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::InitLight` (Xbox PDB): creates the menu light
/// (`01078210`, else `01078204`) and, when there is one, zeroes its two
/// colours and three values, sets its dimmer to 1.0 and its point
/// (`0050dd50`) to (setting `011db514`, 0, 0), sets the screen light colour
/// from system colour 4 and both screen light intensities to 1.0.
pub fn fo_pipboy_manager_init_light(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    if !e
        .call(
            RENDERED_MENU_CREATE_LIGHT,
            &args![this, MENU_LIGHT_NAME_ONE, 0u32],
        )
        .bool()
    {
        e.call(
            RENDERED_MENU_CREATE_LIGHT,
            &args![this, MENU_LIGHT_NAME_TWO, 0u32],
        );
    }
    let light = e.call(MENU_LIGHT, &args![this]).u32();
    if light == 0 {
        return;
    }
    e.with_stack(0x40, |e, frame| {
        let first = frame.addr();
        let second = frame.addr() + 0xc;
        let third = frame.addr() + 0x18;
        let colour = frame.addr() + 0x24;
        let point = frame.addr() + 0x34;
        let zero = e
            .call(POINT_CONSTRUCT, &args![first, 0.0f32, 0.0f32, 0.0f32])
            .u32();
        e.call(LIGHT_SET_AMBIENT, &args![light, zero]);
        let zero = e
            .call(POINT_CONSTRUCT, &args![second, 0.0f32, 0.0f32, 0.0f32])
            .u32();
        e.call(LIGHT_SET_DIFFUSE, &args![light, zero]);
        e.call(LIGHT_SET_0050C630, &args![light, 0.0f32]);
        e.call(LIGHT_SET_0050C660, &args![light, 0.0f32]);
        e.call(LIGHT_SET_0050C690, &args![light, 0.0f32]);
        e.call(LIGHT_SET_DIMMER, &args![light, 1.0f32]);
        let range = setting_float_bits(e, SETTING_LIGHT_RANGE);
        let at = e
            .call(POINT_CONSTRUCT, &args![third, range, 0.0f32, 0.0f32])
            .u32();
        e.call(LIGHT_SET_0050DD50, &args![light, at]);
        e.call(
            COLOR_CONSTRUCT,
            &args![colour, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let colors = e.call(SYSTEM_COLORS, &args![]).u32();
        e.call(SYSTEM_COLOR, &args![colors, 4u32, colour]);
        let red = x87_quiet_bits(e.mem.u32(colour));
        let green = x87_quiet_bits(e.mem.u32(colour + 4));
        let blue = x87_quiet_bits(e.mem.u32(colour + 8));
        e.call(POINT_CONSTRUCT, &args![point, red, green, blue]);
        e.call(SET_SCREEN_LIGHT_COLOR, &args![this, point]);
    });
    e.call(SET_SCREEN_LIGHT_BASE, &args![this, 1.0f32]);
    fn_007f99b0(e, this, 1.0f32.to_bits());
}

// Translated from 007f99b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fScreenLightFlickerIntensity` (+0x3c, Xbox PDB `FORenderedMenu`)
/// from the float (passed as its bit pattern).
pub fn fn_007f99b0(e: &mut Engine, this: Ptr<FOPipboyManager>, value: u32) {
    e.set(
        this,
        FOPipboyManager::fScreenLightFlickerIntensity,
        x87_quiet_bits(value),
    );
}

// Translated from 007f99d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::InitKnobs` (Xbox PDB): takes the tab knob (the first
/// property of the geometry of `01078254`, else `01078244`) and the two
/// other knobs (`01078238`/`0107822c`, `01078224`/`01078218`); spreads the
/// five tab knob positions evenly from the setting `011db33c` to the
/// setting `011db34c` (x87), starts the third knob at the first position,
/// the second at a value from the player's actor value `0x36` and
/// `007ddef0` (0.0 without a player), clears two rates and turns each knob
/// to its position about its axis (the tab knob's rotation first composed
/// with its parent's).
// Translation incomplete: 007f99d0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary strings is not translated
pub fn fo_pipboy_manager_init_knobs(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    let me_addr = this.addr();
    let mut tab_knob = named(e, 0x0107_8254);
    if tab_knob == 0 {
        tab_knob = named(e, 0x0107_8244);
    }
    if tab_knob != 0 && e.vcall(tab_knob, 0xc, &args![]).u32() != 0 {
        let geometry = e.vcall(tab_knob, 0xc, &args![]).u32();
        let property = e.call(PROPERTY_AT, &args![geometry, 0u32]).u32();
        e.call(POINTER_ASSIGN, &args![me_addr + KNOBS, property]);
    }
    for (index, (name, fallback)) in [(0x0107_8238u32, 0x0107_822cu32), (0x0107_8224, 0x0107_8218)]
        .into_iter()
        .enumerate()
    {
        let slot = me_addr + KNOBS + 4 * (index as u32 + 1);
        assign_named(e, slot, name);
        if pointer(e, slot) == 0 {
            assign_named(e, slot, fallback);
        }
    }
    let first = setting_float_bits(e, SETTING_FIRST_TAB);
    e.mem.set_u32(me_addr + TAB_KNOB_POSITIONS, first);
    let last = setting_float_bits(e, SETTING_LAST_TAB);
    e.mem.set_u32(me_addr + TAB_KNOB_POSITIONS + 0x10, last);
    let span = x87_sub(
        x87_m32(e.mem.u32(me_addr + TAB_KNOB_POSITIONS + 0x10)),
        x87_m32(e.mem.u32(me_addr + TAB_KNOB_POSITIONS)),
    );
    let step = x87_store32(x87_div(span, e.global::<f64>(TAB_STEPS)));
    for index in 1..4u32 {
        let previous = e.mem.u32(me_addr + TAB_KNOB_POSITIONS + 4 * (index - 1));
        let value = x87_store32(x87_add(x87_m32(previous), x87_m32(step)));
        e.mem
            .set_u32(me_addr + TAB_KNOB_POSITIONS + 4 * index, value);
    }
    let first = x87_quiet_bits(e.mem.u32(me_addr + TAB_KNOB_POSITIONS));
    e.mem.set_u32(me_addr + KNOB_POSITIONS + 8, first);
    let first = x87_quiet_bits(e.mem.u32(me_addr + TAB_KNOB_POSITIONS));
    e.mem.set_u32(me_addr + KNOB_TARGETS + 8, first);
    e.mem.set_u32(me_addr + KNOB_RATES + 8, 0);
    let player = e.global::<u32>(PLAYER);
    let wheel = if player != 0 {
        let player = e.global::<u32>(PLAYER);
        // The player's object at +0xa4, slot +0xc, asked for actor value 0x36.
        let value = e
            .vcall(player.wrapping_add(0xa4), 0xc, &args![0x36u32])
            .f64();
        let divisor = e.call(INTERFACE_VALUE_007DDEF0, &args![]).f64();
        let ratio = x87_div(value, divisor);
        let scaled = x87_mul(ratio, e.global::<f64>(WHEEL_SCALE));
        x87_sub(e.global::<f64>(WHEEL_OFFSET), scaled)
    } else {
        0.0
    };
    e.mem
        .set_u32(me_addr + KNOB_TARGETS + 4, x87_store32(wheel));
    let target = x87_quiet_bits(e.mem.u32(me_addr + KNOB_TARGETS + 4));
    e.mem.set_u32(me_addr + KNOB_POSITIONS + 4, target);
    e.mem.set_u32(me_addr + KNOB_RATES + 4, 0);
    e.with_stack(0x6c, |e, frame| {
        // [EBP-0x54] and [EBP-0x30]: two matrices; [EBP-0x9c]: the product.
        let own = frame.addr();
        let parent = frame.addr() + 0x24;
        let product = frame.addr() + 0x48;
        e.call(ANSWER_THIS, &args![own]);
        e.call(ANSWER_THIS, &args![parent]);
        if pointer(e, me_addr + KNOBS) != 0 {
            let knob = pointer(e, me_addr + KNOBS);
            let rotation = e.call(LOCAL_ROTATION, &args![knob]).u32();
            let words = e.mem.bytes(rotation, 0x24);
            e.mem.write(own, &words);
            let knob = pointer(e, me_addr + KNOBS);
            let up = e.call(PARENT_OF, &args![knob]).u32();
            let rotation = e.call(LOCAL_ROTATION, &args![up]).u32();
            let words = e.mem.bytes(rotation, 0x24);
            e.mem.write(parent, &words);
            let result = e.call(MATRIX_MULTIPLY, &args![parent, product, own]).u32();
            let knob = pointer(e, me_addr + KNOBS);
            let up = e.call(PARENT_OF, &args![knob]).u32();
            e.call(SET_ROTATION, &args![up, result]);
            let angle = x87_quiet_bits(e.mem.u32(me_addr + KNOB_POSITIONS));
            let knob = pointer(e, me_addr + KNOBS);
            e.call(SET_AXIS_ANGLE, &args![knob, angle, 1.0f32, 0.0f32, 0.0f32]);
        }
    });
    for (index, offset) in [(1u32, 4u32), (2, 8)] {
        let slot = me_addr + KNOBS + 4 * index;
        if pointer(e, slot) != 0 {
            let angle = x87_quiet_bits(e.mem.u32(me_addr + KNOB_POSITIONS + offset));
            let knob = pointer(e, slot);
            e.call(SET_AXIS_ANGLE, &args![knob, angle, 0.0f32, 0.0f32, 1.0f32]);
        }
    }
}

// Translated from 007fa010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the glow of button `button` (1 to 3; 0 shows none): unless the
/// Pip-Boy state test fails, hides the three button glows that exist and
/// shows glow `button - 1` (`spButtonGlow`, +0xf4; the index is not
/// checked against the array).
pub fn fn_007fa010(e: &mut Engine, this: Ptr<FOPipboyManager>, button: u32) {
    if e.call(PIPBOY_STATE, &args![this]).u8() == 0 {
        return;
    }
    for index in 0..3 {
        let slot = this.addr() + BUTTON_GLOW + 4 * index;
        if pointer(e, slot) != 0 {
            let glow = pointer(e, slot);
            e.call(SET_CULLED, &args![glow, 1u32]);
        }
    }
    if button != 0 {
        let slot = this
            .addr()
            .wrapping_add(button.wrapping_mul(4))
            .wrapping_add(0xf0);
        if pointer(e, slot) != 0 {
            let glow = pointer(e, slot);
            e.call(SET_CULLED, &args![glow, 0u32]);
        }
    }
}

// Translated from 007fa310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::ShowPipboyLightEffect` (Xbox PDB name from the call
/// graph): finds the light node (`0107778c`, else `0107826c`) under the
/// first- or third-person Pip-Boy root and shows it when `on` (hides it
/// otherwise). With `store`, the state goes to `b1stPersonLightOn` when
/// `third_person` is set (as the code does) and to `b3rdPersonLightOn`
/// otherwise; in the latter case, when the node has geometry passing
/// `00453470` with a property list, the property of kind `004af350` (or the
/// shader property of kind `00438220`) gets `on` as a float and the
/// properties are updated. Without the root, only the state is stored.
// Translation incomplete: 007fa310 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary strings is not translated
pub fn fo_pipboy_manager_show_pipboy_light_effect(
    e: &mut Engine,
    this: Ptr<FOPipboyManager>,
    third_person: u8,
    on: u8,
    store: u8,
) {
    let slot = if third_person != 0 {
        this.addr() + THIRD_PERSON_ROOT
    } else {
        this.addr() + PIPBOY_ROOT
    };
    let root = pointer(e, slot);
    if root == 0 {
        if store != 0 {
            store_light_state(e, this, third_person, on);
        }
        return;
    }
    let mut node = find_under(e, root, LIGHT_NAME_ONE);
    if node == 0 {
        node = find_under(e, root, LIGHT_NAME_TWO);
    }
    if node == 0 {
        return;
    }
    e.call(SET_CULLED, &args![node, u32::from(on == 0)]);
    if store == 0 {
        return;
    }
    if third_person != 0 {
        e.set(this, FOPipboyManager::b1stPersonLightOn, on);
        return;
    }
    e.set(this, FOPipboyManager::b3rdPersonLightOn, on);
    let geometry = e.vcall(node, 0xc, &args![]).u32();
    if geometry == 0 || e.call(GEOMETRY_TEST_00453470, &args![geometry]).u32() == 0 {
        return;
    }
    let entry = e.call(PROPERTY_AT, &args![geometry, 0u32]).u32();
    let object = e.vcall(entry, 0x18, &args![]).u32();
    if object == 0 {
        return;
    }
    let value = x87_store32(f64::from(on));
    let kind = e.call(PROPERTY_KIND_004AF350, &args![]).u32();
    let property = e.call(GET_PROPERTY, &args![object, kind]).u32();
    if property != 0 {
        e.call(PROPERTY_SET_0068C9F0, &args![property, value]);
    } else {
        let kind = e.call(PROPERTY_KIND_00438220, &args![]).u32();
        let shader = e.call(GET_PROPERTY, &args![object, kind]).u32();
        e.call(SHADER_PROPERTY_SET_ALPHA, &args![shader, value]);
    }
    e.call(UPDATE_PROPERTIES, &args![object]);
}

/// The object named `name` under `root` (its slot `+0x9c` with a temporary
/// `NiFixedString`).
fn find_under(e: &mut Engine, root: u32, name: u32) -> u32 {
    e.with_stack(4, |e, text| {
        let key = e.call(FIXED_STRING_CONSTRUCT, &args![text, name]).u32();
        let found = e.vcall(root, 0x9c, &args![key]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![text]);
        found
    })
}

/// The stored light state of [`fo_pipboy_manager_show_pipboy_light_effect`]
/// without a root.
fn store_light_state(e: &mut Engine, this: Ptr<FOPipboyManager>, third_person: u8, on: u8) {
    if third_person != 0 {
        e.set(this, FOPipboyManager::b1stPersonLightOn, on);
    } else {
        e.set(this, FOPipboyManager::b3rdPersonLightOn, on);
    }
}

// Translated from 007f7f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::ResetColors` (engine map; `thiscall`, no stack words): builds a zero
/// `NiColorA` on the stack (`00414430` with four `0.0`), and when
/// `SystemColorManager::GetColor(4, &color)` answers true, copies it with `007f7fe0` and
/// `007f7fb0` and runs `InitLight`.
pub fn fo_pipboy_manager_reset_colors(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    e.with_stack(0x10, |e, color| {
        e.call(
            COLOR_CONSTRUCT,
            &args![color, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let colors = e.call(SYSTEM_COLORS, &args![]).u32();
        if e.call(SYSTEM_COLOR, &args![colors, 4u32, color]).bool() {
            fn_007f7fe0(e, this, color.addr());
            fn_007f7fb0(e, this, color.addr());
            fo_pipboy_manager_init_light(e, this);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x007f_7f40,
            fo_pipboy_manager_reset_colors(Ptr<FOPipboyManager>)
        ),
        entry!(0x007f_a200, fn_007fa200(Ptr)),
        entry!(0x007f_7fb0, fn_007f7fb0(Ptr<FOPipboyManager>, u32)),
        entry!(0x007f_7fe0, fn_007f7fe0(Ptr<FOPipboyManager>, u32)),
        entry!(
            0x007f_86f0,
            fn_007f86f0(Ptr<FOPipboyManager>, u32, u32, u32)
        ),
        entry!(
            0x007f_8ba0,
            fo_pipboy_manager_initialize(Ptr<FOPipboyManager>)
        ),
        entry!(0x007f_9010, fn_007f9010(Ptr<FOPipboyManager>, u32)),
        entry!(0x007f_9030, fn_007f9030(Ptr<FOPipboyManager>, u32)),
        entry!(0x007f_9050, fn_007f9050(Ptr<FOPipboyManager>, u8, u32)),
        entry!(
            0x007f_9070,
            fo_pipboy_manager_init_buttons(Ptr<FOPipboyManager>)
        ),
        entry!(
            0x007f_9840,
            fo_pipboy_manager_init_light(Ptr<FOPipboyManager>)
        ),
        entry!(0x007f_99b0, fn_007f99b0(Ptr<FOPipboyManager>, u32)),
        entry!(
            0x007f_99d0,
            fo_pipboy_manager_init_knobs(Ptr<FOPipboyManager>)
        ),
        entry!(0x007f_a010, fn_007fa010(Ptr<FOPipboyManager>, u32)),
        entry!(
            0x007f_a310,
            fo_pipboy_manager_show_pipboy_light_effect(Ptr<FOPipboyManager>, u8, u8, u8)
        ),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
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
    fn fwd_a03_007fa200_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x007f_9840, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_a200, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x007f_9840), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b070_tests {
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

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// An object whose vtable slots are `base + slot`.
    fn object_with_vtable(e: &mut Engine, size: u32, base: u32) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
        }
        object
    }

    /// A player whose 3D root answers names through its slot `+0x9c`
    /// (double at `00c1_009c`): the fixed string's word is the name.
    fn player_with_root(e: &mut Engine) -> u32 {
        e.map(PLAYER & !0xfff, 0x1000);
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER, player);
        let root = object_with_vtable(e, 0x10, 0x00c1_0000);
        answer(e, PLAYER_3D, root);
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        stub(e, &[FIXED_STRING_DESTRUCT]);
        // The pointer of a slot answers the word in it.
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        player
    }

    #[test]
    fn colour_copies_and_float_setters() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        let color = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(color + 4 * i, 0x10 + i);
        }
        e.call(0x007f_7fb0, &args![this, color]);
        e.call(0x007f_7fe0, &args![this, color]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this.addr() + 0xc0 + 4 * i), 0x10 + i);
            assert_eq!(e.mem.u32(this.addr() + 0xa8 + 4 * i), 0x10 + i);
        }
        e.call(0x007f_86f0, &args![this, 2u32, 1.5f32, 2.5f32]);
        assert_eq!(e.mem.u32(this.addr() + 0x118 + 8), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(this.addr() + 0x124 + 8), 2.5f32.to_bits());
        e.call(0x007f_9030, &args![this, 0.75f32]);
        assert_eq!(
            e.get(this, FOPipboyManager::fBlurIntensity),
            0.75f32.to_bits()
        );
        e.call(0x007f_9050, &args![this, 1u32, 3.0f32]);
        assert_eq!(e.get(this, FOPipboyManager::bDoScanlines), 1);
        assert_eq!(
            e.get(this, FOPipboyManager::fScanlineScale),
            3.0f32.to_bits()
        );
        e.call(0x007f_99b0, &args![this, 0x7f80_0001u32]);
        assert_eq!(
            e.get(this, FOPipboyManager::fScreenLightFlickerIntensity),
            0x7fc0_0001,
            "an FLD/FSTP copy quiets a signalling NaN"
        );
    }

    #[test]
    fn screen_pointer_store() {
        let mut e = Engine::new();
        stub(&mut e, &[POINTER_ASSIGN]);
        let this = e.new_object::<FOPipboyManager>();
        e.call_log = Some(vec![]);
        e.call(0x007f_9010, &args![this, 0x900u32]);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN),
            vec![vec![this.addr() + 4, 0x900]]
        );
    }

    #[test]
    fn button_glows_hide_all_and_show_one() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        e.mem.set_u32(this.addr() + 0xf4, 0x901);
        e.mem.set_u32(this.addr() + 0xfc, 0x903);
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        answer(&mut e, PIPBOY_STATE, 1);
        stub(&mut e, &[SET_CULLED]);
        e.call_log = Some(vec![]);
        e.call(0x007f_a010, &args![this, 3u32]);
        assert_eq!(
            calls_to(&e, SET_CULLED),
            vec![vec![0x901, 1], vec![0x903, 1], vec![0x903, 0]]
        );
        answer(&mut e, PIPBOY_STATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x007f_a010, &args![this, 3u32]);
        assert!(calls_to(&e, SET_CULLED).is_empty());
    }

    #[test]
    fn light_effect_with_a_root_stores_the_state() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        let root = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        e.mem.set_u32(this.addr() + 0xd8, root);
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let node = object_with_vtable(&mut e, 0x10, 0x00c2_0000);
        e.register_double(0x00c1_009c, move |e, a| Ret {
            eax: if e.mem.u32(a[1]) == LIGHT_NAME_TWO {
                node
            } else {
                0
            },
            ..Ret::default()
        });
        answer(&mut e, 0x00c2_000c, 0);
        stub(&mut e, &[FIXED_STRING_DESTRUCT, SET_CULLED]);
        e.call_log = Some(vec![]);
        e.call(0x007f_a310, &args![this, 0u32, 1u32, 1u32]);
        assert_eq!(calls_to(&e, 0x00c1_009c).len(), 2);
        assert_eq!(calls_to(&e, SET_CULLED), vec![vec![node, 0]]);
        assert_eq!(e.get(this, FOPipboyManager::b3rdPersonLightOn), 1);
        // Without the third-person root only the state is stored.
        e.call(0x007f_a310, &args![this, 1u32, 1u32, 1u32]);
        assert_eq!(e.get(this, FOPipboyManager::b1stPersonLightOn), 1);
    }

    #[test]
    fn light_effect_sets_the_property_value() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        let root = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        e.mem.set_u32(this.addr() + 0xd8, root);
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let node = object_with_vtable(&mut e, 0x10, 0x00c2_0000);
        let geometry = e.mem.alloc(4);
        let entry = object_with_vtable(&mut e, 0x10, 0x00c3_0000);
        answer(&mut e, 0x00c1_009c, node);
        answer(&mut e, 0x00c2_000c, geometry);
        answer(&mut e, GEOMETRY_TEST_00453470, 1);
        answer(&mut e, PROPERTY_AT, entry);
        answer(&mut e, 0x00c3_0018, 0x950);
        answer(&mut e, PROPERTY_KIND_004AF350, 7);
        answer(&mut e, GET_PROPERTY, 0);
        answer(&mut e, PROPERTY_KIND_00438220, 8);
        stub(
            &mut e,
            &[
                FIXED_STRING_CONSTRUCT,
                FIXED_STRING_DESTRUCT,
                SET_CULLED,
                SHADER_PROPERTY_SET_ALPHA,
                UPDATE_PROPERTIES,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x007f_a310, &args![this, 0u32, 1u32, 1u32]);
        assert_eq!(
            calls_to(&e, GET_PROPERTY),
            vec![vec![0x950, 7], vec![0x950, 8]]
        );
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_SET_ALPHA),
            vec![vec![0, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, UPDATE_PROPERTIES), vec![vec![0x950]]);
    }

    #[test]
    fn init_buttons_takes_buttons_and_glows() {
        let mut e = Engine::new();
        player_with_root(&mut e);
        let this = e.new_object::<FOPipboyManager>();
        // Every name is found except the glow names 0107818c (fallback
        // 01078180 is used); objects are named by their name's low word.
        e.register(0x00c1_009c, |e, a| {
            let name = e.mem.u32(a[1]);
            Ret {
                eax: if name == 0x0107_818c {
                    0
                } else {
                    name & 0xffff
                },
                ..Ret::default()
            }
        });
        e.register(NI_CAST, |_, a| Ret {
            eax: a[1] + 0x1_0000,
            ..Ret::default()
        });
        e.register(CHILD_AT, |_, a| Ret {
            eax: a[0] + 0x2_0000,
            ..Ret::default()
        });
        answer(&mut e, PIPBOY_STATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x007f_9070, &args![this]);
        let me_addr = this.addr();
        assert_eq!(e.mem.u32(me_addr + 0xe8), 0x3_81f4);
        assert_eq!(e.mem.u32(me_addr + 0xec), 0x3_81d0);
        assert_eq!(e.mem.u32(me_addr + 0xf0), 0x3_81ac);
        assert_eq!(e.mem.u32(me_addr + 0xf4), 0x8180);
        assert_eq!(e.mem.u32(me_addr + 0xf8), 0x8174);
        assert_eq!(e.mem.u32(me_addr + 0xfc), 0x815c);
        assert_eq!(calls_to(&e, PIPBOY_STATE), vec![vec![me_addr]]);
    }

    #[test]
    fn init_light_sets_up_the_menu_light() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        answer(&mut e, RENDERED_MENU_CREATE_LIGHT, 0);
        answer(&mut e, MENU_LIGHT, 0x700);
        e.register(POINT_CONSTRUCT, |e, a| {
            e.mem
                .write(a[0], &[a[1], a[2], a[3]].map(u32::to_le_bytes).concat());
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let range = e.mem.alloc(4);
        e.mem.set_f32(range, 96.0);
        answer(&mut e, SETTING_VALUE_ADDRESS, range);
        stub(
            &mut e,
            &[
                LIGHT_SET_AMBIENT,
                LIGHT_SET_DIFFUSE,
                LIGHT_SET_0050C630,
                LIGHT_SET_0050C660,
                LIGHT_SET_0050C690,
                LIGHT_SET_DIMMER,
                LIGHT_SET_0050DD50,
                COLOR_CONSTRUCT,
                SYSTEM_COLORS,
                SET_SCREEN_LIGHT_COLOR,
                SET_SCREEN_LIGHT_BASE,
            ],
        );
        e.register(SYSTEM_COLOR, |e, a| {
            e.mem.set_f32(a[2], 0.25);
            e.mem.set_f32(a[2] + 4, 0.5);
            e.mem.set_f32(a[2] + 8, 0.75);
            Ret::default()
        });
        e.register(LIGHT_SET_0050DD50, |e, a| {
            assert_eq!(e.mem.f32(a[1]), 96.0);
            Ret::default()
        });
        e.register(SET_SCREEN_LIGHT_COLOR, |e, a| {
            assert_eq!(e.mem.f32(a[1] + 8), 0.75);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007f_9840, &args![this]);
        assert_eq!(
            calls_to(&e, RENDERED_MENU_CREATE_LIGHT),
            vec![
                vec![this.addr(), MENU_LIGHT_NAME_ONE, 0],
                vec![this.addr(), MENU_LIGHT_NAME_TWO, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, LIGHT_SET_DIMMER),
            vec![vec![0x700, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, SET_SCREEN_LIGHT_COLOR).len(), 1);
        assert_eq!(
            e.get(this, FOPipboyManager::fScreenLightFlickerIntensity),
            1.0f32.to_bits()
        );
    }

    #[test]
    fn init_knobs_spreads_the_tab_positions() {
        let mut e = Engine::new();
        let player = player_with_root(&mut e);
        e.map(0x0101_d000, 0x1000);
        e.map(0x0101_f000, 0x1000);
        e.map(0x0106_4000, 0x1000);
        e.set_global(TAB_STEPS, 4.0f64);
        e.set_global(WHEEL_SCALE, 2.0f64);
        e.set_global(WHEEL_OFFSET, 10.0f64);
        let this = e.new_object::<FOPipboyManager>();
        // No knob objects: every name answers 0.
        answer(&mut e, 0x00c1_009c, 0);
        let first = e.mem.alloc(4);
        e.mem.set_f32(first, 10.0);
        let last = e.mem.alloc(4);
        e.mem.set_f32(last, 30.0);
        e.register_double(SETTING_VALUE_ADDRESS, move |_, a| Ret {
            eax: if a[0] == SETTING_FIRST_TAB {
                first
            } else {
                last
            },
            ..Ret::default()
        });
        let owner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        e.mem.set_u32(owner_vtable + 0xc, 0x00c5_000c);
        e.register(0x00c5_000c, |_, _| Ret {
            st0: 6.0,
            ..Ret::default()
        });
        e.register(INTERFACE_VALUE_007DDEF0, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        stub(&mut e, &[ANSWER_THIS]);
        e.call(0x007f_99d0, &args![this]);
        let me_addr = this.addr();
        let positions: Vec<f32> = (0..5).map(|i| e.mem.f32(me_addr + 0x130 + 4 * i)).collect();
        assert_eq!(positions, vec![10.0, 15.0, 20.0, 25.0, 30.0]);
        assert_eq!(e.mem.f32(me_addr + 0x114), 10.0);
        assert_eq!(e.mem.f32(me_addr + 0x120), 10.0);
        // 10 - (6 / 3) * 2 = 6.
        assert_eq!(e.mem.f32(me_addr + 0x11c), 6.0);
        assert_eq!(e.mem.f32(me_addr + 0x110), 6.0);
    }

    #[test]
    fn init_knobs_turns_the_knobs() {
        let mut e = Engine::new();
        player_with_root(&mut e);
        e.map(0x0101_d000, 0x1000);
        e.set_global(TAB_STEPS, 4.0f64);
        e.set_global(PLAYER, 0u32);
        let this = e.new_object::<FOPipboyManager>();
        let tab = object_with_vtable(&mut e, 0x10, 0x00c6_0000);
        e.register_double(0x00c1_009c, move |e, a| Ret {
            eax: match e.mem.u32(a[1]) {
                0x0107_8254 => tab,
                0x0107_8238 => 0x904,
                0x0107_8224 => 0x908,
                _ => 0,
            },
            ..Ret::default()
        });
        answer(&mut e, 0x00c6_000c, 0x600);
        answer(&mut e, PROPERTY_AT, 0x900);
        let zero = e.mem.alloc(4);
        answer(&mut e, SETTING_VALUE_ADDRESS, zero);
        let rotation = e.mem.alloc(0x24);
        e.mem.set_f32(rotation, 1.0);
        answer(&mut e, LOCAL_ROTATION, rotation);
        answer(&mut e, PARENT_OF, 0x980);
        e.register(MATRIX_MULTIPLY, |e, a| {
            assert_eq!(e.mem.f32(a[0]), 1.0);
            assert_eq!(e.mem.f32(a[2]), 1.0);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        stub(&mut e, &[ANSWER_THIS, SET_ROTATION, SET_AXIS_ANGLE]);
        e.call_log = Some(vec![]);
        e.call(0x007f_99d0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(e.mem.u32(me_addr + 0x100), 0x900);
        assert_eq!(e.mem.u32(me_addr + 0x104), 0x904);
        assert_eq!(e.mem.u32(me_addr + 0x108), 0x908);
        assert_eq!(calls_to(&e, SET_ROTATION).len(), 1);
        assert_eq!(calls_to(&e, SET_ROTATION)[0][0], 0x980);
        assert_eq!(
            calls_to(&e, SET_AXIS_ANGLE),
            vec![
                vec![0x900, 0, 1.0f32.to_bits(), 0, 0],
                vec![0x904, 0, 0, 0, 1.0f32.to_bits()],
                vec![0x908, 0, 0, 0, 1.0f32.to_bits()]
            ]
        );
    }

    #[test]
    fn initialize_stops_when_disabled() {
        let mut e = Engine::new();
        let flag = e.mem.alloc(4);
        answer(&mut e, SETTING_BOOL_ADDRESS, flag);
        let this = e.new_object::<FOPipboyManager>();
        e.call_log = Some(vec![]);
        e.call(0x007f_8ba0, &args![this]);
        assert_eq!(
            calls_to(&e, SETTING_BOOL_ADDRESS),
            vec![vec![SETTING_ENABLED]]
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn initialize_builds_the_pipboy() {
        let mut e = Engine::new();
        let player = player_with_root(&mut e);
        e.map(0x0101_d000, 0x1000);
        e.map(0x0101_f000, 0x1000);
        e.map(0x0106_4000, 0x1000);
        e.set_global(TAB_STEPS, 4.0f64);
        let owner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        e.mem.set_u32(owner_vtable + 0xc, 0x00c5_000c);
        stub(&mut e, &[0x00c5_000c, INTERFACE_VALUE_007DDEF0]);
        // The roots answer names through the same slot double.
        let root = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        answer(&mut e, SETTING_BOOL_ADDRESS, flag);
        answer(&mut e, PIPBOY_STATE, 0);
        let this = e.new_object::<FOPipboyManager>();
        // The Pip-Boy node is found under its first name.
        e.register(0x00c1_009c, |e, a| Ret {
            eax: if e.mem.u32(a[1]) == PIPBOY_NAME_ONE {
                0x700
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(NI_CAST, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        answer(&mut e, CHILD_AT, 0x710);
        answer(&mut e, PARENT_OF, root);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 0.5);
        answer(&mut e, SETTING_VALUE_ADDRESS, setting);
        answer(&mut e, SYSTEM_COLOR, 0);
        answer(&mut e, RENDERED_MENU_CREATE_LIGHT, 1);
        answer(&mut e, MENU_LIGHT, 0);
        stub(
            &mut e,
            &[
                RENDERED_MENU_INITIALIZE,
                INTERFACE_SINGLETON,
                INTERFACE_OBJECT_007FA950,
                SET_MEMBER_0X14,
                USE_PLAYER_ROOT,
                RENDERED_MENU_PREPARE_GEOMETRY,
                ANSWER_THIS,
                UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
                UPDATE_PROPERTIES,
                COLOR_CONSTRUCT,
                SYSTEM_COLORS,
                SETTER_00444020,
                SET_FLAG_00810530,
                SET_SCREEN_LIGHT_BASE,
                SET_CULLED,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x007f_8ba0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(e.mem.u32(me_addr + 0xd8), root);
        assert_eq!(e.mem.u32(me_addr + 0xdc), root);
        assert_eq!(e.mem.u32(me_addr + 0xe0), 0x710);
        assert_eq!(
            e.mem.u32(me_addr + 4),
            0x710,
            "the screen stored by 007f9010"
        );
        assert_eq!(
            calls_to(&e, RENDERED_MENU_PREPARE_GEOMETRY),
            vec![vec![me_addr, 0x710], vec![me_addr, 0x710]]
        );
        assert_eq!(calls_to(&e, NODE_UPDATE).len(), 2);
        assert_eq!(
            e.get(this, FOPipboyManager::fBlurIntensity),
            0.5f32.to_bits()
        );
        assert_eq!(e.get(this, FOPipboyManager::bDoScanlines), 1);
        assert_eq!(calls_to(&e, SET_FLAG_00810530), vec![vec![me_addr, 1]]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn reset_engine(found: u32) -> Engine {
        let mut e = Engine::new();
        e.register(COLOR_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        e.register(SYSTEM_COLORS, |_, _| 0x0c00_0000u32.into_ret());
        e.register_double(SYSTEM_COLOR, move |e, a| {
            e.mem.set_f32(a[2], 0.25);
            e.mem.set_f32(a[2] + 4, 0.5);
            e.mem.set_f32(a[2] + 8, 0.75);
            e.mem.set_f32(a[2] + 12, 1.0);
            found.into_ret()
        });
        // `InitLight` stops at once without a light.
        e.register(RENDERED_MENU_CREATE_LIGHT, |_, _| Ret::default());
        e.register(MENU_LIGHT, |_, _| Ret::default());
        e
    }

    #[test]
    fn reset_colors_copies_system_colour_4_and_inits_the_light() {
        let mut e = reset_engine(0x1234_5601);
        let this = e.new_object::<FOPipboyManager>();
        e.call_log = Some(vec![]);
        e.call(0x007f_7f40, &args![this]);
        let log = e.call_log.take().unwrap();
        let construct = log.iter().find(|(a, _)| *a == COLOR_CONSTRUCT).unwrap();
        assert_eq!(&construct.1[1..], &[0, 0, 0, 0]);
        let query = log.iter().find(|(a, _)| *a == SYSTEM_COLOR).unwrap();
        assert_eq!(query.1[..2], [0x0c00_0000, 4]);
        assert_eq!(query.1[2], construct.1[0]);
        for base in [0xa8u32, 0xc0] {
            let got: Vec<f32> = (0..4)
                .map(|i| e.mem.f32(this.addr() + base + 4 * i))
                .collect();
            assert_eq!(got, vec![0.25, 0.5, 0.75, 1.0]);
        }
        assert_eq!(log.iter().filter(|(a, _)| *a == MENU_LIGHT).count(), 1);
    }

    #[test]
    fn reset_colors_without_the_colour_does_nothing_more() {
        let mut e = reset_engine(0x1234_5600);
        let this = e.new_object::<FOPipboyManager>();
        e.call_log = Some(vec![]);
        e.call(0x007f_7f40, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(this.addr() + 0xa8), 0.0);
        assert_eq!(e.mem.f32(this.addr() + 0xc0), 0.0);
        assert!(log.iter().all(|(a, _)| *a != MENU_LIGHT));
    }
}

/// `007fa8d0` (`thiscall`, one word) runs first on menu open (with 0) and
/// close (with the player's `Actor::IsRefractive`, `008c51c0`).
const MENU_STATE_007FA8D0: u32 = 0x007f_a8d0;
const ACTOR_IS_REFRACTIVE: u32 = 0x008c_51c0;
/// The bytes menu open sets (`011d890a`, `011db378`; the second is
/// cleared on close and tested by `SetTabKnobPosition`).
const MENU_OPEN_BYTE_011D890A: u32 = 0x011d_890a;
const PIPBOY_OPEN_011DB378: u32 = 0x011d_b378;
/// `Interface::IsolateMenuElements` (`00702c80`, `cdecl`, two words) and
/// `Interface::RestoreMenuElements` (`00702dd0`, `cdecl`).
const ISOLATE_MENU_ELEMENTS: u32 = 0x0070_2c80;
const RESTORE_MENU_ELEMENTS: u32 = 0x0070_2dd0;
/// The Pip-Boy's hum sound handle (`+0x154`).
const HUM_SOUND: u32 = 0x154;
/// `BSSoundHandle` calls (engine map): `Stop` (`00ad88f0`), `Release`
/// (`00ad8d10`), `Play` (`00ad8830`, one word), the constructor
/// (`0041a250`), assignment (`00418900`, one word) and the destructor
/// (`00483710`); `BSAudio::QInstance` (`00453a70`) and
/// `BSAudio::GetSoundHandleByName` (`00ad7550`: out, name, flags).
const SOUND_STOP: u32 = 0x00ad_88f0;
const SOUND_RELEASE: u32 = 0x00ad_8d10;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_ASSIGN: u32 = 0x0041_8900;
const SOUND_DESTRUCT: u32 = 0x0048_3710;
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_BY_NAME: u32 = 0x00ad_7550;
/// The sound names: `"UIPipBoyHumLP"` (`01078100`), `"UIPipBoyAccessDown"`
/// (`01078110`), `"UIPipBoyScroll"` (`01071e30`), `"UIPipBoyTab"`
/// (`01078260`).
const SOUND_HUM: u32 = 0x0107_8100;
const SOUND_ACCESS_DOWN: u32 = 0x0107_8110;
const SOUND_SCROLL: u32 = 0x0107_1e30;
const SOUND_TAB: u32 = 0x0107_8260;
/// `FORenderedMenu::TriggerBurst` (`007fb150`, three floats),
/// `FORenderedMenu::OnMenuOpen` (`007fb040`), `007fb0b0` (on close), and
/// the two other burst-like calls of `007f8a80` (`007fb250`, two floats;
/// `007fb2b0`, three floats) on the interface's rendered menu
/// (`00704370` of `004b7210`).
const TRIGGER_BURST: u32 = 0x007f_b150;
const RENDERED_MENU_ON_MENU_OPEN: u32 = 0x007f_b040;
const RENDERED_MENU_007FB0B0: u32 = 0x007f_b0b0;
const RENDERED_MENU_007FB250: u32 = 0x007f_b250;
const RENDERED_MENU_007FB2B0: u32 = 0x007f_b2b0;
const INTERFACE_RENDERED_MENU: u32 = 0x0070_4370;
/// The burst's last float (`01016248`) and its settings (`011db480`,
/// `011db43c`), and the settings `007f8a80` reads.
const BURST_FLOAT_01016248: u32 = 0x0101_6248;
const SETTING_011DB480: u32 = 0x011d_b480;
const SETTING_011DB43C: u32 = 0x011d_b43c;
const SETTING_011DB4B4: u32 = 0x011d_b4b4;
const SETTING_011DB4E0: u32 = 0x011d_b4e0;
const SETTING_011DB4EC: u32 = 0x011d_b4ec;
const SETTING_011DB3FC: u32 = 0x011d_b3fc;
const SETTING_011DB414: u32 = 0x011d_b414;
const SETTING_011DB3BC: u32 = 0x011d_b3bc;
const SETTING_011DB3B0: u32 = 0x011d_b3b0;
/// `TESImageSpaceModifier::GetGetHit` (`005d2860`) and
/// `ImageSpaceModifierInstanceForm::Stop` (`00529c90`, `cdecl`), and
/// `004de8c0` (`cdecl`, a float).
const GET_HIT_MODIFIER: u32 = 0x005d_2860;
const MODIFIER_STOP: u32 = 0x0052_9c90;
const AFTER_OPEN_004DE8C0: u32 = 0x004d_e8c0;
/// The byte `007f8160` stores (`0118abb1`).
const OPEN_STATE_0118ABB1: u32 = 0x0118_abb1;
/// The player's animation calls (engine map): `GetAnimation`
/// (`00950a60`, one word), `Animation::SpecialIdleDonePlaying`
/// (`004985f0`), `Animation::ClearGroup` (`00496080`: group, float),
/// `Actor::GetAnimGroup` (`00897910`, four words) and `Animation::PlayGroup`
/// (`00494740`, four words).
const GET_ANIMATION: u32 = 0x0095_0a60;
const SPECIAL_IDLE_DONE_PLAYING: u32 = 0x0049_85f0;
const CLEAR_GROUP: u32 = 0x0049_6080;
const GET_ANIM_GROUP: u32 = 0x0089_7910;
const PLAY_GROUP: u32 = 0x0049_4740;
/// The time of the last knob update (`011db370`).
const LAST_KNOB_TIME: u32 = 0x011d_b370;
/// The setting `SetTabKnobPosition` copies into the third knob's rate
/// (`011db288`).
const SETTING_TAB_KNOB_RATE: u32 = 0x011d_b288;
/// The timer object (`011f6394`) with its time (`00825c00`, a word) and its
/// float (`0084d030`, in `ST0`).
const TIMER_011F6394: u32 = 0x011f_6394;
const TIMER_TIME: u32 = 0x0082_5c00;
const TIMER_FLOAT: u32 = 0x0084_d030;
/// The fade's duration settings (`011db308` for `FadeLight`, `011db2bc`
/// for `UpdateLightEffect`), the clamps it goes through (`0040ebd0` and
/// `00404010`, `cdecl`, two floats, the result in `ST0`), the double `0.0`
/// it compares the fade direction with (`01012060`), and the player's light
/// (`00572750`, an `NiPointer` slot).
const SETTING_FADE_LIGHT: u32 = 0x011d_b308;
const SETTING_LIGHT_EFFECT: u32 = 0x011d_b2bc;
const CLAMP_0040EBD0: u32 = 0x0040_ebd0;
const CLAMP_00404010: u32 = 0x0040_4010;
const ZERO_DOUBLE_01012060: u32 = 0x0101_2060;
const PLAYER_LIGHT_00572750: u32 = 0x0057_2750;
/// The float `00491180` gets last (`01012054`).
const TIME_ARGUMENT_01012054: u32 = 0x0101_2054;
/// The checks and calls of `007fa220`: `00703d50` (a byte), the interface's
/// word `+0x4bc`, `00424940` on the interface (a byte), `00491040` on the
/// animation (one word), `0048f7f0`, `005f2420` (a word), `00494710` (one
/// word, a byte), `008eeaa0` (one word), `00491180` (three words),
/// `00493900` (one word).
const CHECK_00703D50: u32 = 0x0070_3d50;
const INTERFACE_BYTE_00424940: u32 = 0x0042_4940;
const ANIMATION_00491040: u32 = 0x0049_1040;
const SEQUENCE_0048F7F0: u32 = 0x0048_f7f0;
const SEQUENCE_WORD_005F2420: u32 = 0x005f_2420;
const ANIMATION_TEST_00494710: u32 = 0x0049_4710;
const ANIMATION_008EEAA0: u32 = 0x008e_eaa0;
const ANIMATION_00491180: u32 = 0x0049_1180;
const ANIMATION_00493900: u32 = 0x0049_3900;
/// `005c5420` (`cdecl`, a float in `ST0`): the value `007f8a80` compares.
const VALUE_005C5420: u32 = 0x005c_5420;
/// The pick calls of `007f8720`: `007178a0` and `00717920` (bytes), the
/// point constructor (`006815c0`), the `NiPick` constructor (`00e98f20`,
/// two words), its setters (`00458b30`, `006ecd40`, `00792780`, one word;
/// the target `00705fc0`), pick (`00e98e20`: origin, direction, 0; a byte),
/// results (`00500940`), result at (`00968670`, one word), destructor
/// (`00e98fa0`); the camera (`006629f0` of `0045c670`) and its ray
/// (`00a71080`, five words); the cursor values of the interface
/// (`00792760`, `009a9350`, in `ST0`, through `_ftol2` `00ec62c0`); a
/// result's object (`00458b50`) and texture coordinates (`00891170`); the
/// scale `00706e20` (in `ST0`) and the double `01078128`.
const PICK_CHECK_007178A0: u32 = 0x0071_78a0;
const PICK_CHECK_00717920: u32 = 0x0071_7920;
const POINT_CONSTRUCT_006815C0: u32 = 0x0068_15c0;
const PICK_CONSTRUCT: u32 = 0x00e9_8f20;
const PICK_SET_00458B30: u32 = 0x0045_8b30;
const PICK_SET_006ECD40: u32 = 0x006e_cd40;
const PICK_SET_00792780: u32 = 0x0079_2780;
const PICK_OBJECTS: u32 = 0x00e9_8e20;
const PICK_RESULTS: u32 = 0x0050_0940;
const PICK_RESULT_AT: u32 = 0x0096_8670;
const PICK_DESTRUCT: u32 = 0x00e9_8fa0;
const CAMERA_OWNER_0045C670: u32 = 0x0045_c670;
const CAMERA_006629F0: u32 = 0x0066_29f0;
const CAMERA_RAY: u32 = 0x00a7_1080;
const CURSOR_X_00792760: u32 = 0x0079_2760;
const CURSOR_Y_009A9350: u32 = 0x009a_9350;
const FTOL2: u32 = 0x00ec_62c0;
const RESULT_OBJECT_00458B50: u32 = 0x0045_8b50;
const RESULT_UV_00891170: u32 = 0x0089_1170;
const SCALE_00706E20: u32 = 0x0070_6e20;
const SCALE_DOUBLE_01078128: u32 = 0x0107_8128;
/// The button pressed on the last pick (`011a0ba0`, -1 for none).
const PRESSED_BUTTON_011A0BA0: u32 = 0x011a_0ba0;
/// The light effect's fields: `bLightEffectOn` (`+0x162`), its start time
/// (`+0x164`) and the fade's end value (`+0x168`) (Xbox PDB names
/// unconfirmed; the offsets the code uses).
const LIGHT_EFFECT_ON: u32 = 0x162;
const LIGHT_EFFECT_START: u32 = 0x164;
const LIGHT_EFFECT_END: u32 = 0x168;
/// `fTabKnobPosition`'s copy in the third knob target (`+0x120`) and rate
/// (`+0x12c`).
const THIRD_KNOB_TARGET: u32 = KNOB_TARGETS + 8;
const THIRD_KNOB_RATE: u32 = KNOB_RATES + 8;

/// The size of a sound handle (`BSSoundHandle`): `0041a250` sets +0, +4 (a byte) and +8.
const SOUND_HANDLE_SIZE: u32 = 0xc;

/// A sound named `name` played once through a stack handle: constructed
/// (`0041a250`), given the audio's handle for the name (a temporary,
/// assigned and destroyed), played (`Play` with 0) and destroyed. A sound
/// handle is 0xC bytes (the id, a byte at +4, a word at +8: `0041a250`); the
/// exe keeps the handle at `[EBP-0x18]` and the temporary at `[EBP-0x24]`
/// (`007fa179`, `007fa192`).
fn play_named_sound(e: &mut Engine, name: u32) {
    e.with_stack(2 * SOUND_HANDLE_SIZE, |e, handles| {
        let handle = handles.addr();
        let temporary = handles.addr() + SOUND_HANDLE_SIZE;
        e.call(SOUND_CONSTRUCT, &args![handle]);
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        let found = e
            .call(
                AUDIO_GET_SOUND_BY_NAME,
                &args![audio, temporary, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_ASSIGN, &args![handle, found]);
        e.call(SOUND_DESTRUCT, &args![temporary]);
        e.call(SOUND_PLAY, &args![handle, 0u32]);
        e.call(SOUND_DESTRUCT, &args![handle]);
    });
}

/// The interface's rendered menu (`00704370` of `004b7210`).
fn rendered_menu(e: &mut Engine) -> u32 {
    let interface = e.call(INTERFACE_SINGLETON, &[]).u32();
    e.call(INTERFACE_RENDERED_MENU, &args![interface]).u32()
}

/// A float argument copied through `FLD dword`/`FSTP dword`.
fn float_copy(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

// Translated from 007f8010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007f8010 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `FOPipboyManager::OnMenuOpen` (engine map; `thiscall`, plain `RET`):
/// nothing unless the Pip-Boy state test (`00974d90`) passes. `007fa8d0`
/// with 0, `ResetColors`, the bytes `011d890a` and `011db378` set, the menu
/// elements isolated (1, 1), the hum handle (`+0x154`) stopped, released,
/// given `"UIPipBoyHumLP"` (flags `0x31`) and played; a burst with the
/// settings `011db480`, `011db43c` and the float `01016248`, the rendered
/// menu's `OnMenuOpen`, the get-hit image space modifier stopped, the byte
/// `0118abb1` cleared (`007f8160`) and `004de8c0` with 0.0.
pub fn fo_pipboy_manager_on_menu_open(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    if e.call(PIPBOY_STATE, &args![this]).u32() & 0xff == 0 {
        return;
    }
    e.call(MENU_STATE_007FA8D0, &args![this, 0u32]);
    fo_pipboy_manager_reset_colors(e, this);
    e.set_global(MENU_OPEN_BYTE_011D890A, 1u8);
    e.set_global(PIPBOY_OPEN_011DB378, 1u8);
    e.call(ISOLATE_MENU_ELEMENTS, &args![1u32, 1u32]);
    let hum = this.addr() + HUM_SOUND;
    e.call(SOUND_STOP, &args![hum]);
    e.call(SOUND_RELEASE, &args![hum]);
    e.with_stack(SOUND_HANDLE_SIZE, |e, temporary| {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        let found = e
            .call(
                AUDIO_GET_SOUND_BY_NAME,
                &args![audio, temporary, SOUND_HUM, 0x31u32],
            )
            .u32();
        e.call(SOUND_ASSIGN, &args![hum, found]);
        e.call(SOUND_DESTRUCT, &args![temporary]);
    });
    e.call(SOUND_PLAY, &args![hum, 0u32]);
    let last = float_copy(e, BURST_FLOAT_01016248);
    let second = setting_float_bits(e, SETTING_011DB43C);
    let first = setting_float_bits(e, SETTING_011DB480);
    e.call(TRIGGER_BURST, &args![this, first, second, last]);
    e.call(RENDERED_MENU_ON_MENU_OPEN, &args![this]);
    let modifier = e.call(GET_HIT_MODIFIER, &[]).u32();
    e.call(MODIFIER_STOP, &args![modifier]);
    fn_007f8160(e, 0);
    e.call(AFTER_OPEN_004DE8C0, &args![0.0f32.to_bits()]);
}

// Translated from 007f8160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): its low byte stored at
/// `0118abb1`.
pub fn fn_007f8160(e: &mut Engine, value: u32) {
    e.set_global(OPEN_STATE_0118ABB1, value as u8);
}

// Translated from 007f8170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007f8170 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `FOPipboyManager::OnMenuClose` (engine map; `thiscall`, plain `RET`):
/// `007fa8d0` with the player's `IsRefractive`; with the player's
/// first-person animation, its group 7 cleared (0.0) unless its special
/// idle is done playing, the player's current group (`GetAnimGroup` with 0,
/// 0, 0, 0) played on it and on the third-person animation (1, -1, -1),
/// and group 0 (1, -1, 0) when the player's slot `+0x214` answers non-zero.
/// Then, when the Pip-Boy state test passes, `"UIPipBoyAccessDown"` played,
/// the hum stopped and released, the button glows hidden (`007fa010` with
/// 0), the menu elements restored, `007fb0b0`, the byte `011db378` cleared
/// and `007f8160` with 1.
pub fn fo_pipboy_manager_on_menu_close(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    let player = e.global::<u32>(PLAYER);
    let refractive = e.call(ACTOR_IS_REFRACTIVE, &args![player]).u32() & 0xff;
    e.call(MENU_STATE_007FA8D0, &args![this, refractive]);
    let player = e.global::<u32>(PLAYER);
    let animation = e.call(GET_ANIMATION, &args![player, 1u32]).u32();
    if animation != 0 {
        if e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).u32() & 0xff == 0 {
            e.call(CLEAR_GROUP, &args![animation, 7u32, 0.0f32.to_bits()]);
        }
        let player = e.global::<u32>(PLAYER);
        let group = e
            .call(GET_ANIM_GROUP, &args![player, 0u32, 0u32, 0u32, 0u32])
            .u32()
            & 0xffff;
        e.call(
            PLAY_GROUP,
            &args![animation, group, 1u32, u32::MAX, u32::MAX],
        );
        let player = e.global::<u32>(PLAYER);
        let group = e
            .call(GET_ANIM_GROUP, &args![player, 0u32, 0u32, 0u32, 0u32])
            .u32()
            & 0xffff;
        let player = e.global::<u32>(PLAYER);
        let third = e.call(GET_ANIMATION, &args![player, 0u32]).u32();
        e.call(PLAY_GROUP, &args![third, group, 1u32, u32::MAX, u32::MAX]);
        let player = e.global::<u32>(PLAYER);
        if e.vcall(player, 0x214, &[]).u32() != 0 {
            e.call(PLAY_GROUP, &args![animation, 0u32, 1u32, u32::MAX, 0u32]);
        }
    }
    if e.call(PIPBOY_STATE, &args![this]).u32() & 0xff == 0 {
        return;
    }
    e.with_stack(SOUND_HANDLE_SIZE, |e, temporary| {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        let found = e
            .call(
                AUDIO_GET_SOUND_BY_NAME,
                &args![audio, temporary, SOUND_ACCESS_DOWN, 0x121u32],
            )
            .u32();
        e.call(SOUND_PLAY, &args![found, 0u32]);
        e.call(SOUND_DESTRUCT, &args![temporary]);
    });
    let hum = this.addr() + HUM_SOUND;
    e.call(SOUND_STOP, &args![hum]);
    e.call(SOUND_RELEASE, &args![hum]);
    fn_007fa010(e, this, 0);
    e.call(RESTORE_MENU_ELEMENTS, &[]);
    e.call(RENDERED_MENU_007FB0B0, &args![this]);
    e.set_global(PIPBOY_OPEN_011DB378, 0u8);
    fn_007f8160(e, 1);
}

/// One knob's step toward its target for `007f8320`: `rate * dt` added
/// (below the target) or taken off (above it), not past the target.
/// Answers whether it moved.
fn step_knob(e: &mut Engine, this: Ptr<FOPipboyManager>, index: u32, dt: u32) -> bool {
    let at = |offset: u32| this.addr() + offset + index * 4;
    let mut moved = false;
    // FLD target; FLD position; FCOMPP: position < target.
    if fld(e, at(KNOB_POSITIONS)) < fld(e, at(KNOB_TARGETS)) {
        let step = x87_mul(fld(e, at(KNOB_RATES)), x87_m32(dt));
        let position = x87_add(step, fmem(e, at(KNOB_POSITIONS)));
        e.mem.set_u32(at(KNOB_POSITIONS), x87_store32(position));
        if fld(e, at(KNOB_TARGETS)) < fld(e, at(KNOB_POSITIONS)) {
            let target = float_copy(e, at(KNOB_TARGETS));
            e.mem.set_u32(at(KNOB_POSITIONS), target);
        }
        moved = true;
    }
    if fld(e, at(KNOB_POSITIONS)) > fld(e, at(KNOB_TARGETS)) {
        let step = x87_mul(fld(e, at(KNOB_RATES)), x87_m32(dt));
        // FSUBR: position - step.
        let position = x87_sub(fmem(e, at(KNOB_POSITIONS)), step);
        e.mem.set_u32(at(KNOB_POSITIONS), x87_store32(position));
        if fld(e, at(KNOB_TARGETS)) > fld(e, at(KNOB_POSITIONS)) {
            let target = float_copy(e, at(KNOB_TARGETS));
            e.mem.set_u32(at(KNOB_POSITIONS), target);
        }
        moved = true;
    }
    moved
}

/// `FLD dword` of the float at `addr`.
fn fld(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// A `float` memory operand at `addr`.
fn fmem(e: &Engine, addr: u32) -> f64 {
    x87_m32(e.mem.u32(addr))
}

// Translated from 007f8320 (decompiled, FalloutNV.exe 1.4.0.525)
/// The knob update (unnamed in the engine map; `thiscall`, `RET 4`: the
/// time): each of the three knobs (`fKnobPositions` toward `fKnobTargets`
/// at `fKnobRates` per unit of the time since `011db370`) steps, and each
/// knob that moved and exists (`spKnobs`) is turned to its position about
/// its axis (`006404b0`: the first about (1, 0, 0), the others about
/// (0, 0, 1)) and updated (`00a59c60` with a stack update data, `0043d410`
/// with 0.0, 0, 0). The time is kept at `011db370`.
pub fn fn_007f8320(e: &mut Engine, this: Ptr<FOPipboyManager>, now: f32) {
    let dt = x87_store32(x87_sub(x87_load32(now.to_bits()), fmem(e, LAST_KNOB_TIME)));
    let mut moved = [false; 3];
    for index in 0..3u32 {
        moved[index as usize] = step_knob(e, this, index, dt);
    }
    let (one, zero) = (1.0f32.to_bits(), 0.0f32.to_bits());
    for (index, moved) in (0u32..).zip(moved) {
        let slot = this.addr() + KNOBS + index * 4;
        if pointer(e, slot) == 0 || !moved {
            continue;
        }
        let angle = float_copy(e, this.addr() + KNOB_POSITIONS + index * 4);
        let knob = pointer(e, slot);
        if index == 0 {
            e.call(SET_AXIS_ANGLE, &args![knob, angle, one, zero, zero]);
        } else {
            e.call(SET_AXIS_ANGLE, &args![knob, angle, zero, zero, one]);
        }
        e.with_stack(0xc, |e, update| {
            e.call(UPDATE_DATA_CONSTRUCT, &args![update, zero, 0u32, 0u32]);
            let knob = pointer(e, slot);
            e.call(NODE_UPDATE, &args![knob, update]);
        });
    }
    e.set_global(LAST_KNOB_TIME, x87_quiet_bits(now.to_bits()));
}

// Translated from 007f8610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007f8610 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `FOPipboyManager::MoveKnob` (engine map; `thiscall`, `RET 0x10`): with
/// `sound` and knob 0, `"UIPipBoyScroll"` plays; then the knob's target
/// (`fKnobTargets`) moves by `delta` and its rate (`fKnobRates`) becomes
/// `rate`.
pub fn fo_pipboy_manager_move_knob(
    e: &mut Engine,
    this: Ptr<FOPipboyManager>,
    index: u32,
    delta: f32,
    rate: f32,
    sound: u32,
) {
    if sound & 0xff != 0 && index == 0 {
        play_named_sound(e, SOUND_SCROLL);
    }
    let at = this.addr().wrapping_add(index.wrapping_mul(4));
    let target = x87_add(
        fld(e, at.wrapping_add(KNOB_TARGETS)),
        x87_m32(delta.to_bits()),
    );
    e.mem
        .set_u32(at.wrapping_add(KNOB_TARGETS), x87_store32(target));
    e.mem
        .set_u32(at.wrapping_add(KNOB_RATES), x87_quiet_bits(rate.to_bits()));
}

/// The ray from the cursor through the interface's camera for `007f8720`:
/// the cursor values (`00792760`, `009a9350`, each through `_ftol2`) and
/// `00a71080` on the camera (`006629f0` of `0045c670`, fetched first when
/// `camera_first`) into the origin and direction.
fn cursor_ray(e: &mut Engine, interface: u32, origin: u32, direction: u32, camera_first: bool) {
    let mut camera = 0;
    if camera_first {
        let owner = e.call(CAMERA_OWNER_0045C670, &[]).u32();
        camera = e.call(CAMERA_006629F0, &args![owner]).u32();
    }
    let x = e.call(CURSOR_X_00792760, &args![interface]).f64();
    let x = e.call(FTOL2, &args![x]).u32();
    let y = e.call(CURSOR_Y_009A9350, &args![interface]).f64();
    let y = e.call(FTOL2, &args![y]).u32();
    if !camera_first {
        let owner = e.call(CAMERA_OWNER_0045C670, &[]).u32();
        camera = e.call(CAMERA_006629F0, &args![owner]).u32();
    }
    e.call(CAMERA_RAY, &args![camera, y, x, origin, direction, 0u32]);
}

/// The first pick result's object (`00500940`, `00968670` with 0), when
/// the pick (`00e98e20`) hit; null otherwise.
fn pick_result(e: &mut Engine, pick: u32, origin: u32, direction: u32) -> u32 {
    if e.call(PICK_OBJECTS, &args![pick, origin, direction, 0u32])
        .u32()
        & 0xff
        == 0
    {
        return 0;
    }
    let results = e.call(PICK_RESULTS, &args![pick]).u32();
    e.call(PICK_RESULT_AT, &args![results, 0u32]).u32()
}

// Translated from 007f8720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007f8720 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The Pip-Boy screen pick (unnamed in the engine map; `thiscall`,
/// `RET 0x14`: press and release bytes, the texture coordinate outputs and
/// the button output): false unless the screen object (`007fa950`) and the
/// three buttons (`spButtons`) exist; false (nothing picked) unless
/// `007178a0` and `00717920` pass and the Pip-Boy root is set. A stack
/// `NiPick` (`00e98f20` with 0, 8) on the screen object picks along the
/// cursor's ray; when its first result's object is the screen object, its
/// texture coordinates (`00891170`) times the scale (`00706e20`, times the
/// double `01078128` for u) go to the outputs and the answer is true. With
/// a press or release, each button is picked: a hit with a press stores
/// its index at `011a0ba0`, a hit without one keeps it; a release on the
/// pressed button answers it plus 1 in the button output and clears
/// `011a0ba0`.
pub fn fn_007f8720(
    e: &mut Engine,
    this: Ptr<FOPipboyManager>,
    press: u32,
    release: u32,
    out_u: u32,
    out_v: u32,
    out_button: u32,
) -> u8 {
    let mut hit = 0u8;
    let interface = e.call(INTERFACE_SINGLETON, &[]).u32();
    if e.call(INTERFACE_OBJECT_007FA950, &args![this]).u32() == 0
        || pointer(e, this.addr() + BUTTONS) == 0
        || pointer(e, this.addr() + BUTTONS + 4) == 0
        || pointer(e, this.addr() + BUTTONS + 8) == 0
    {
        return 0;
    }
    if e.call(PICK_CHECK_007178A0, &[]).u32() & 0xff == 0
        || e.call(PICK_CHECK_00717920, &[]).u32() & 0xff == 0
        || pointer(e, this.addr() + PIPBOY_ROOT) == 0
    {
        return hit;
    }
    let (press, release) = (press & 0xff, release & 0xff);
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let direction = f + 0x1c;
        let origin = f + 0x28;
        let pick = f + 0x34;
        e.call(POINT_CONSTRUCT_006815C0, &args![origin]);
        e.call(POINT_CONSTRUCT_006815C0, &args![direction]);
        e.call(PICK_CONSTRUCT, &args![pick, 0u32, 8u32]);
        e.call(PICK_SET_00458B30, &args![pick, 1u32]);
        let screen = e.call(INTERFACE_OBJECT_007FA950, &args![this]).u32();
        e.call(SET_MEMBER_0X14, &args![pick, screen]);
        e.call(PICK_SET_006ECD40, &args![pick, 0u32]);
        e.call(PICK_SET_00792780, &args![pick, 1u32]);
        cursor_ray(e, interface, origin, direction, true);
        let result = pick_result(e, pick, origin, direction);
        if result != 0 {
            let object = e.call(RESULT_OBJECT_00458B50, &args![result]).u32();
            if object == e.call(INTERFACE_OBJECT_007FA950, &args![this]).u32() {
                let uv = e.call(RESULT_UV_00891170, &args![result]).u32();
                let (u, v) = (e.mem.u32(uv), e.mem.u32(uv.wrapping_add(4)));
                let scale = x87_store32(e.call(SCALE_00706E20, &[]).f64());
                let factor = f64::from_bits(e.mem.u64(SCALE_DOUBLE_01078128));
                let scale_u = x87_store32(x87_mul(x87_load32(scale), factor));
                let x = x87_mul(x87_load32(u), x87_m32(scale_u));
                e.mem.set_u32(out_u, x87_store32(x));
                let y = x87_mul(x87_load32(v), x87_m32(scale));
                e.mem.set_u32(out_v, x87_store32(y));
                hit = 1;
            }
        }
        if press != 0 || release != 0 {
            let mut released = u32::MAX;
            e.call(PICK_SET_006ECD40, &args![pick, 0u32]);
            e.call(PICK_SET_00792780, &args![pick, 0u32]);
            for button in 0..3u32 {
                let target = pointer(e, this.addr() + BUTTONS + button * 4);
                e.call(SET_MEMBER_0X14, &args![pick, target]);
                cursor_ray(e, interface, origin, direction, false);
                if pick_result(e, pick, origin, direction) != 0 {
                    if press != 0 {
                        e.set_global(PRESSED_BUTTON_011A0BA0, button);
                    } else {
                        released = button;
                    }
                }
            }
            if release != 0 && released == e.global::<u32>(PRESSED_BUTTON_011A0BA0) {
                e.mem.set_u32(out_button, released.wrapping_add(1));
                e.set_global(PRESSED_BUTTON_011A0BA0, u32::MAX);
            }
        }
        e.call(PICK_DESTRUCT, &args![pick]);
    });
    hit
}

// Translated from 007f8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): with the value of
/// `005c5420` below the setting `011db4b4`, the rendered menu's `007fb250`
/// with the settings `011db4ec` and `011db3fc`; with it from that setting
/// up to (not including) that plus `011db4e0`, its `007fb2b0` with
/// `011db414`, `011db3bc` and `011db3b0`; then its burst with `011db480`,
/// `011db43c` and the float `01016248`.
pub fn fn_007f8a80(e: &mut Engine, _this: Ptr) {
    let value = x87_store32(e.call(VALUE_005C5420, &[]).f64());
    let value = x87_load32(value);
    let address = e
        .call(SETTING_VALUE_ADDRESS, &args![SETTING_011DB4B4])
        .u32();
    // FLD value; FLD setting; FCOMPP: setting above value.
    if fld(e, address) > value {
        let second = setting_float_bits(e, SETTING_011DB3FC);
        let first = setting_float_bits(e, SETTING_011DB4EC);
        let menu = rendered_menu(e);
        e.call(RENDERED_MENU_007FB250, &args![menu, first, second]);
    }
    let address = e
        .call(SETTING_VALUE_ADDRESS, &args![SETTING_011DB4B4])
        .u32();
    if fld(e, address) <= value {
        let start = e
            .call(SETTING_VALUE_ADDRESS, &args![SETTING_011DB4B4])
            .u32();
        let length = e
            .call(SETTING_VALUE_ADDRESS, &args![SETTING_011DB4E0])
            .u32();
        let end = x87_add(fld(e, start), fmem(e, length));
        if value < end {
            let third = setting_float_bits(e, SETTING_011DB3B0);
            let second = setting_float_bits(e, SETTING_011DB3BC);
            let first = setting_float_bits(e, SETTING_011DB414);
            let menu = rendered_menu(e);
            e.call(RENDERED_MENU_007FB2B0, &args![menu, first, second, third]);
        }
    }
    let last = float_copy(e, BURST_FLOAT_01016248);
    let second = setting_float_bits(e, SETTING_011DB43C);
    let first = setting_float_bits(e, SETTING_011DB480);
    let menu = rendered_menu(e);
    e.call(TRIGGER_BURST, &args![menu, first, second, last]);
}

// Translated from 007fa0f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007fa0f0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `FOPipboyManager::SetTabKnobPosition` (engine map; `thiscall`, `RET 8`):
/// when tab position `index` (`fTabKnobPositions`) differs from the third
/// knob's target (or either is a NaN), it becomes that target and the
/// setting `011db288` that knob's rate; with the Pip-Boy open (`011db378`)
/// and `sound`, `"UIPipBoyTab"` plays.
pub fn fo_pipboy_manager_set_tab_knob_position(
    e: &mut Engine,
    this: Ptr<FOPipboyManager>,
    index: u32,
    sound: u32,
) {
    let position = this
        .addr()
        .wrapping_add(TAB_KNOB_POSITIONS)
        .wrapping_add(index.wrapping_mul(4));
    // FUCOMPP; TEST AH,0x44; JNP: equal skips.
    if fld(e, position) == fld(e, this.addr() + THIRD_KNOB_TARGET) {
        return;
    }
    let target = float_copy(e, position);
    e.mem.set_u32(this.addr() + THIRD_KNOB_TARGET, target);
    let rate = setting_float_bits(e, SETTING_TAB_KNOB_RATE);
    e.mem.set_u32(this.addr() + THIRD_KNOB_RATE, rate);
    if e.global::<u8>(PIPBOY_OPEN_011DB378) != 0 && sound & 0xff != 0 {
        play_named_sound(e, SOUND_TAB);
    }
}

// Translated from 007fa220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): unless `00703d50`
/// is set, the interface's word `+0x4bc` is not 3 or its `00424940` byte is
/// 4: on the player's first-person animation, group 1 is played (1, -1,
/// -1) when its `00491040` sequence's `005f2420` word is not 1 and its
/// `00494710` test with 1 passes; then `008eeaa0` with `0x17`, `00491180`
/// with the player, the timer's float (`0084d030`) and the float
/// `01012054`, and `00493900` with the player.
pub fn fn_007fa220(e: &mut Engine, _this: Ptr) {
    if e.call(CHECK_00703D50, &[]).u32() & 0xff != 0 {
        return;
    }
    let interface = e.call(INTERFACE_SINGLETON, &[]).u32();
    if e.mem.u32(interface.wrapping_add(0x4bc)) != 3 {
        return;
    }
    let interface = e.call(INTERFACE_SINGLETON, &[]).u32();
    if e.call(INTERFACE_BYTE_00424940, &args![interface]).u32() as i8 == 4 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let animation = e.call(GET_ANIMATION, &args![player, 1u32]).u32();
    let sequence = e.call(ANIMATION_00491040, &args![animation, 0u32]).u32();
    if sequence != 0 {
        let inner = e.call(SEQUENCE_0048F7F0, &args![sequence]).u32();
        if e.call(SEQUENCE_WORD_005F2420, &args![inner]).u32() != 1
            && e.call(ANIMATION_TEST_00494710, &args![animation, 1u32])
                .u32()
                & 0xff
                != 0
        {
            e.call(
                PLAY_GROUP,
                &args![animation, 1u32, 1u32, u32::MAX, u32::MAX],
            );
        }
    }
    e.call(ANIMATION_008EEAA0, &args![animation, 0x17u32]);
    let last = float_copy(e, TIME_ARGUMENT_01012054);
    let time = x87_store32(e.call(TIMER_FLOAT, &args![TIMER_011F6394]).f64());
    let player = e.global::<u32>(PLAYER);
    e.call(ANIMATION_00491180, &args![animation, player, time, last]);
    let player = e.global::<u32>(PLAYER);
    e.call(ANIMATION_00493900, &args![animation, player]);
}

// Translated from 007fa540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007fa540 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `FOPipboyManager::UpdateLightEffect` (engine map; `thiscall`, `RET 4`:
/// the time): while the light effect runs (`+0x162`): `FadeLight`, the
/// light shown (`ShowPipboyLightEffect` with 0, 1, 0); the fade value (the
/// time since `+0x164` over the setting `011db2bc`, clamped by `0040ebd0`
/// with 1.0 and `00404010` with 0.0, reversed when the end value `+0x168`
/// is 0) goes to the light node's (`0107778c`, else `0107826c`, under the
/// Pip-Boy root) geometry property of kind `004af350`, or its shader
/// property of kind `00438220`, and its properties are updated. When the
/// value reaches the end value the effect stops, and for an end value of 0
/// the light is hidden (`ShowPipboyLightEffect` with 0, 0, 0).
pub fn fo_pipboy_manager_update_light_effect(e: &mut Engine, this: Ptr<FOPipboyManager>, now: f32) {
    let base = this.addr();
    if e.mem.u8(base + LIGHT_EFFECT_ON) == 0 {
        return;
    }
    fo_pipboy_manager_fade_light(e, this, f32::from_bits(x87_quiet_bits(now.to_bits())));
    fo_pipboy_manager_show_pipboy_light_effect(e, this, 0, 1, 0);
    let root = pointer(e, base + PIPBOY_ROOT);
    let value = fade_value(e, this, now, SETTING_LIGHT_EFFECT);
    let mut light = find_under(e, root, LIGHT_NAME_ONE);
    if light == 0 {
        light = find_under(e, root, LIGHT_NAME_TWO);
    }
    let geometry = if light != 0 {
        e.vcall(light, 0xc, &[]).u32()
    } else {
        0
    };
    if geometry != 0 && e.call(GEOMETRY_TEST_00453470, &args![geometry]).u32() != 0 {
        let entry = e.call(PROPERTY_AT, &args![geometry, 0u32]).u32();
        let object = e.vcall(entry, 0x18, &[]).u32();
        if object != 0 {
            let kind = e.call(PROPERTY_KIND_004AF350, &[]).u32();
            let property = e.call(GET_PROPERTY, &args![object, kind]).u32();
            if property != 0 {
                e.call(PROPERTY_SET_0068C9F0, &args![property, float_bits(value)]);
            } else {
                let kind = e.call(PROPERTY_KIND_00438220, &[]).u32();
                let shader = e.call(GET_PROPERTY, &args![object, kind]).u32();
                e.call(SHADER_PROPERTY_SET_ALPHA, &args![shader, float_bits(value)]);
            }
            e.call(UPDATE_PROPERTIES, &args![object]);
        }
    }
    // FLD value; FLD end; FUCOMPP: equal stops the effect.
    if fld(e, base + LIGHT_EFFECT_END) == x87_load32(value) {
        e.mem.set_u8(base + LIGHT_EFFECT_ON, 0);
        if fld(e, base + LIGHT_EFFECT_END) == zero_double(e) {
            fo_pipboy_manager_show_pipboy_light_effect(e, this, 0, 0, 0);
        }
    }
}

/// The double at `01012060` (`FCOMP qword`).
fn zero_double(e: &Engine) -> f64 {
    f64::from_bits(e.mem.u64(ZERO_DOUBLE_01012060))
}

/// A stored float passed through `FLD`/`FSTP` as an argument.
fn float_bits(value: u32) -> u32 {
    x87_quiet_bits(value)
}

/// The fade value of the light effect at `now`, as a stored float: the
/// time since the start over the setting, `0040ebd0` with 1.0 and
/// `00404010` with 0.0 (results in `ST0`, stored), then `1 - value` when
/// the end value equals the double `01012060`.
fn fade_value(e: &mut Engine, this: Ptr<FOPipboyManager>, now: f32, setting: u32) -> u32 {
    let base = this.addr();
    let elapsed = x87_store32(x87_sub(
        x87_load32(now.to_bits()),
        fmem(e, base + LIGHT_EFFECT_START),
    ));
    let duration = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
    let ratio = x87_store32(x87_div(x87_load32(elapsed), fmem(e, duration)));
    let one = 1.0f32.to_bits();
    let clamped = e
        .call(CLAMP_0040EBD0, &args![x87_quiet_bits(ratio), one])
        .f64();
    let value = x87_store32(clamped);
    let clamped = e
        .call(
            CLAMP_00404010,
            &args![0.0f32.to_bits(), x87_quiet_bits(value)],
        )
        .f64();
    let mut value = x87_store32(clamped);
    if fld(e, base + LIGHT_EFFECT_END) == zero_double(e) {
        // FLD value; FLD1; FSUBRP: 1 - value.
        value = x87_store32(x87_sub(1.0, x87_load32(value)));
    }
    value
}

// Translated from 007fa7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::FadeLightEffect` (engine map; `thiscall`, `RET 4`):
/// with the first-person light on (`b1stPersonLightOn`), the effect starts
/// now (the timer's time as a float at `+0x164`, `+0x162` set) toward 0.0
/// for `fade_out`, else 1.0 (`+0x168`).
pub fn fo_pipboy_manager_fade_light_effect(
    e: &mut Engine,
    this: Ptr<FOPipboyManager>,
    fade_out: u32,
) {
    if e.get(this, FOPipboyManager::b1stPersonLightOn) == 0 {
        return;
    }
    let base = this.addr();
    let time = e.call(TIMER_TIME, &args![TIMER_011F6394]).u32();
    // FILD qword of the zero-extended time.
    e.mem
        .set_u32(base + LIGHT_EFFECT_START, x87_store32(f64::from(time)));
    e.mem.set_u8(base + LIGHT_EFFECT_ON, 1);
    let end = if fade_out & 0xff != 0 { 0.0f32 } else { 1.0 };
    e.mem.set_u32(base + LIGHT_EFFECT_END, end.to_bits());
}

// Translated from 007fa810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::FadeLight` (engine map; `thiscall`, `RET 4`: the
/// time): the fade value (as in `UpdateLightEffect`, over the setting
/// `011db308`) given to the player's light (`00572750`, when it and its
/// object are set) as its dimmer (`0050dd20`).
pub fn fo_pipboy_manager_fade_light(e: &mut Engine, this: Ptr<FOPipboyManager>, now: f32) {
    let value = fade_value(e, this, now, SETTING_FADE_LIGHT);
    let player = e.global::<u32>(PLAYER);
    let light = e.call(PLAYER_LIGHT_00572750, &args![player]).u32();
    if light != 0 && pointer(e, light) != 0 {
        let object = pointer(e, light);
        e.call(LIGHT_SET_DIMMER, &args![object, float_bits(value)]);
    }
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x007f_8010,
            fo_pipboy_manager_on_menu_open(Ptr<FOPipboyManager>)
        ),
        entry!(0x007f_8160, fn_007f8160(u32)),
        entry!(
            0x007f_8170,
            fo_pipboy_manager_on_menu_close(Ptr<FOPipboyManager>)
        ),
        entry!(0x007f_8320, fn_007f8320(Ptr<FOPipboyManager>, f32)),
        entry!(
            0x007f_8610,
            fo_pipboy_manager_move_knob(Ptr<FOPipboyManager>, u32, f32, f32, u32)
        ),
        entry!(
            0x007f_8720,
            fn_007f8720(Ptr<FOPipboyManager>, u32, u32, u32, u32, u32) -> u8
        ),
        entry!(0x007f_8a80, fn_007f8a80(Ptr)),
        entry!(
            0x007f_a0f0,
            fo_pipboy_manager_set_tab_knob_position(Ptr<FOPipboyManager>, u32, u32)
        ),
        entry!(0x007f_a220, fn_007fa220(Ptr)),
        entry!(
            0x007f_a540,
            fo_pipboy_manager_update_light_effect(Ptr<FOPipboyManager>, f32)
        ),
        entry!(
            0x007f_a7a0,
            fo_pipboy_manager_fade_light_effect(Ptr<FOPipboyManager>, u32)
        ),
        entry!(
            0x007f_a810,
            fo_pipboy_manager_fade_light(Ptr<FOPipboyManager>, f32)
        ),
    ]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    fn st0(value: f32) -> Ret {
        Ret {
            st0: f64::from(value),
            ..Ret::default()
        }
    }

    /// The pages of the globals and settings these functions read; each
    /// setting's value is at the setting itself (`00403e20` answers its
    /// argument).
    fn world(e: &mut Engine) -> u32 {
        for page in [
            0x011d_8000u32,
            0x011d_b000,
            0x0118_a000,
            0x011a_0000,
            0x0101_6000,
            0x0107_8000,
            0x0101_2000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(SETTING_VALUE_ADDRESS, |_, a| a[0].into_ret());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        let player = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x300);
        e.mem.set_u32(player, vtable);
        e.mem.set_u32(vtable + 0x214, 0x7500_0214);
        e.set_global(PLAYER, player);
        player
    }

    fn manager(e: &mut Engine) -> u32 {
        e.mem.alloc(0x170)
    }

    fn sounds(e: &mut Engine) {
        quiet(
            e,
            &[
                SOUND_STOP,
                SOUND_RELEASE,
                SOUND_PLAY,
                SOUND_CONSTRUCT,
                SOUND_ASSIGN,
                SOUND_DESTRUCT,
            ],
        );
        e.register(AUDIO_INSTANCE, |_, _| 0xa0d10u32.into_ret());
        e.register(AUDIO_GET_SOUND_BY_NAME, |_, a| {
            (a[2] | 0x8000_0000).into_ret()
        });
    }

    #[test]
    fn on_menu_open_starts_the_hum_and_the_burst() {
        let mut e = Engine::new();
        world(&mut e);
        sounds(&mut e);
        quiet(
            &mut e,
            &[
                MENU_STATE_007FA8D0,
                COLOR_CONSTRUCT,
                ISOLATE_MENU_ELEMENTS,
                TRIGGER_BURST,
                RENDERED_MENU_ON_MENU_OPEN,
                MODIFIER_STOP,
                AFTER_OPEN_004DE8C0,
            ],
        );
        e.register(SYSTEM_COLORS, |_, _| 0u32.into_ret());
        e.register(SYSTEM_COLOR, |_, _| 0u32.into_ret());
        e.register(GET_HIT_MODIFIER, |_, _| 0x3131u32.into_ret());
        e.register(PIPBOY_STATE, |_, _| 0xff00u32.into_ret());
        let this = manager(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007f_8010, &args![this]);
        assert!(calls(&e, MENU_STATE_007FA8D0).is_empty());
        e.register(PIPBOY_STATE, |_, _| 1u32.into_ret());
        e.mem.set_f32(SETTING_011DB480, 1.5);
        e.mem.set_f32(SETTING_011DB43C, 2.5);
        e.mem.set_f32(BURST_FLOAT_01016248, 3.5);
        e.set_global(OPEN_STATE_0118ABB1, 9u8);
        e.call_log = Some(vec![]);
        e.call(0x007f_8010, &args![this]);
        assert_eq!(calls(&e, MENU_STATE_007FA8D0), vec![vec![this, 0]]);
        assert_eq!(e.global::<u8>(MENU_OPEN_BYTE_011D890A), 1);
        assert_eq!(e.global::<u8>(PIPBOY_OPEN_011DB378), 1);
        assert_eq!(calls(&e, ISOLATE_MENU_ELEMENTS), vec![vec![1, 1]]);
        let hum = this + HUM_SOUND;
        assert_eq!(calls(&e, SOUND_STOP), vec![vec![hum]]);
        assert_eq!(
            calls(&e, SOUND_ASSIGN),
            vec![vec![hum, SOUND_HUM | 0x8000_0000]]
        );
        assert_eq!(
            calls(&e, AUDIO_GET_SOUND_BY_NAME)[0][2..],
            [SOUND_HUM, 0x31]
        );
        assert_eq!(calls(&e, SOUND_PLAY), vec![vec![hum, 0]]);
        assert_eq!(
            calls(&e, TRIGGER_BURST),
            vec![vec![
                this,
                1.5f32.to_bits(),
                2.5f32.to_bits(),
                3.5f32.to_bits()
            ]]
        );
        assert_eq!(calls(&e, MODIFIER_STOP), vec![vec![0x3131]]);
        assert_eq!(e.global::<u8>(OPEN_STATE_0118ABB1), 0);
        assert_eq!(calls(&e, AFTER_OPEN_004DE8C0), vec![vec![0]]);
    }

    #[test]
    fn on_menu_close_restores_the_animations() {
        let mut e = Engine::new();
        let player = world(&mut e);
        sounds(&mut e);
        quiet(
            &mut e,
            &[
                MENU_STATE_007FA8D0,
                CLEAR_GROUP,
                PLAY_GROUP,
                RESTORE_MENU_ELEMENTS,
                RENDERED_MENU_007FB0B0,
            ],
        );
        e.register(ACTOR_IS_REFRACTIVE, |_, _| 0x1201u32.into_ret());
        e.register(GET_ANIMATION, |_, a| (0x6000 + a[1]).into_ret());
        e.register(SPECIAL_IDLE_DONE_PLAYING, |_, _| 0x100u32.into_ret());
        e.register(GET_ANIM_GROUP, |_, _| 0xabcd_0012u32.into_ret());
        e.register(0x7500_0214, |_, _| 1u32.into_ret());
        e.register(PIPBOY_STATE, |_, _| 1u32.into_ret());
        let this = manager(&mut e);
        e.set_global(PIPBOY_OPEN_011DB378, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x007f_8170, &args![this]);
        assert_eq!(calls(&e, MENU_STATE_007FA8D0), vec![vec![this, 1]]);
        assert_eq!(calls(&e, CLEAR_GROUP), vec![vec![0x6001, 7, 0]]);
        assert_eq!(
            calls(&e, PLAY_GROUP),
            vec![
                vec![0x6001, 0x12, 1, u32::MAX, u32::MAX],
                vec![0x6000, 0x12, 1, u32::MAX, u32::MAX],
                vec![0x6001, 0, 1, u32::MAX, 0]
            ]
        );
        assert_eq!(calls(&e, GET_ANIM_GROUP)[0], vec![player, 0, 0, 0, 0]);
        assert_eq!(
            calls(&e, AUDIO_GET_SOUND_BY_NAME)[0][2..],
            [SOUND_ACCESS_DOWN, 0x121]
        );
        assert_eq!(
            calls(&e, SOUND_PLAY),
            vec![vec![SOUND_ACCESS_DOWN | 0x8000_0000, 0]]
        );
        assert_eq!(calls(&e, SOUND_STOP), vec![vec![this + HUM_SOUND]]);
        assert_eq!(calls(&e, RENDERED_MENU_007FB0B0), vec![vec![this]]);
        assert_eq!(e.global::<u8>(PIPBOY_OPEN_011DB378), 0);
        assert_eq!(e.global::<u8>(OPEN_STATE_0118ABB1), 1);

        // No animation, and the state test fails: only 007fa8d0.
        e.register(GET_ANIMATION, |_, _| 0u32.into_ret());
        e.register(PIPBOY_STATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007f_8170, &args![this]);
        assert!(calls(&e, PLAY_GROUP).is_empty());
        assert!(calls(&e, SOUND_STOP).is_empty());
    }

    #[test]
    fn knob_update_steps_toward_the_targets() {
        let mut e = Engine::new();
        world(&mut e);
        quiet(
            &mut e,
            &[SET_AXIS_ANGLE, UPDATE_DATA_CONSTRUCT, NODE_UPDATE],
        );
        let this = manager(&mut e);
        let knob = |i: u32| this + i * 4;
        for (i, (position, target, rate)) in
            (0u32..).zip([(0.0f32, 1.0f32, 2.0f32), (3.0, 1.0, 4.0), (1.0, 1.0, 1.0)])
        {
            e.mem.set_f32(knob(i) + KNOB_POSITIONS, position);
            e.mem.set_f32(knob(i) + KNOB_TARGETS, target);
            e.mem.set_f32(knob(i) + KNOB_RATES, rate);
            e.mem.set_u32(knob(i) + KNOBS, 0x900 + i);
        }
        e.mem.set_f32(LAST_KNOB_TIME, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x007f_8320, &args![this, 1.25f32.to_bits()]);
        assert_eq!(e.mem.f32(knob(0) + KNOB_POSITIONS), 0.5);
        assert_eq!(e.mem.f32(knob(1) + KNOB_POSITIONS), 2.0);
        assert_eq!(e.mem.f32(knob(2) + KNOB_POSITIONS), 1.0);
        let (one, half, two) = (1.0f32.to_bits(), 0.5f32.to_bits(), 2.0f32.to_bits());
        assert_eq!(
            calls(&e, SET_AXIS_ANGLE),
            vec![vec![0x900, half, one, 0, 0], vec![0x901, two, 0, 0, one]]
        );
        assert_eq!(calls(&e, NODE_UPDATE).len(), 2);
        assert_eq!(e.mem.f32(LAST_KNOB_TIME), 1.25);
        // Overshooting stops at the target.
        e.mem.set_f32(knob(0) + KNOB_RATES, 100.0);
        e.call(0x007f_8320, &args![this, 2.0f32.to_bits()]);
        assert_eq!(e.mem.f32(knob(0) + KNOB_POSITIONS), 1.0);
        assert_eq!(e.mem.f32(knob(1) + KNOB_POSITIONS), 1.0);
    }

    #[test]
    fn move_knob_moves_the_target() {
        let mut e = Engine::new();
        world(&mut e);
        sounds(&mut e);
        let this = manager(&mut e);
        e.mem.set_f32(this + KNOB_TARGETS + 4, 1.0);
        e.call_log = Some(vec![]);
        e.call(
            0x007f_8610,
            &args![this, 1u32, 0.5f32.to_bits(), 3.0f32.to_bits(), 1u32],
        );
        assert_eq!(e.mem.f32(this + KNOB_TARGETS + 4), 1.5);
        assert_eq!(e.mem.f32(this + KNOB_RATES + 4), 3.0);
        assert!(calls(&e, SOUND_PLAY).is_empty());
        e.call(
            0x007f_8610,
            &args![this, 0u32, 0.5f32.to_bits(), 3.0f32.to_bits(), 1u32],
        );
        assert_eq!(e.mem.f32(this + KNOB_TARGETS), 0.5);
        assert_eq!(
            calls(&e, AUDIO_GET_SOUND_BY_NAME)[0][2..],
            [SOUND_SCROLL, 0x121]
        );
        assert_eq!(calls(&e, SOUND_PLAY).len(), 1);
        assert_eq!(calls(&e, SOUND_DESTRUCT).len(), 2);
    }

    #[test]
    fn screen_pick_answers_coordinates_and_buttons() {
        let mut e = Engine::new();
        world(&mut e);
        quiet(
            &mut e,
            &[
                POINT_CONSTRUCT_006815C0,
                PICK_CONSTRUCT,
                PICK_SET_00458B30,
                SET_MEMBER_0X14,
                PICK_SET_006ECD40,
                PICK_SET_00792780,
                CAMERA_RAY,
                PICK_DESTRUCT,
            ],
        );
        e.register(INTERFACE_SINGLETON, |_, _| 0x1111u32.into_ret());
        e.register(INTERFACE_OBJECT_007FA950, |_, _| 0x5c5cu32.into_ret());
        e.register(PICK_CHECK_007178A0, |_, _| 1u32.into_ret());
        e.register(PICK_CHECK_00717920, |_, _| 1u32.into_ret());
        e.register(CAMERA_OWNER_0045C670, |_, _| 0x2222u32.into_ret());
        e.register(CAMERA_006629F0, |_, _| 0x3333u32.into_ret());
        e.register(CURSOR_X_00792760, |_, _| st0(10.0));
        e.register(CURSOR_Y_009A9350, |_, _| st0(20.0));
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)) as u32).into_ret()
        });
        e.register(PICK_OBJECTS, |_, _| 1u32.into_ret());
        e.register(PICK_RESULTS, |_, _| 0x4444u32.into_ret());
        e.register(PICK_RESULT_AT, |_, _| 0x4545u32.into_ret());
        e.register(RESULT_OBJECT_00458B50, |_, _| 0x5c5cu32.into_ret());
        let uv = e.mem.alloc(8);
        e.mem.set_f32(uv, 0.5);
        e.mem.set_f32(uv + 4, 0.25);
        e.register_double(RESULT_UV_00891170, move |_, _| uv.into_ret());
        e.register(SCALE_00706E20, |_, _| st0(2.0));
        e.mem.set_u64(SCALE_DOUBLE_01078128, 1.5f64.to_bits());
        let this = manager(&mut e);
        for i in 0..3 {
            e.mem.set_u32(this + BUTTONS + i * 4, 0x700 + i);
        }
        e.mem.set_u32(this + PIPBOY_ROOT, 0x800);
        let outs = e.mem.alloc(12);
        e.set_global(PRESSED_BUTTON_011A0BA0, u32::MAX);
        e.call_log = Some(vec![]);
        let hit = e.call(
            0x007f_8720,
            &args![this, 1u32, 0u32, outs, outs + 4, outs + 8],
        );
        assert_eq!(hit.u32() & 0xff, 1);
        assert_eq!(e.mem.f32(outs), 1.5);
        assert_eq!(e.mem.f32(outs + 4), 0.5);
        let rays = calls(&e, CAMERA_RAY);
        assert_eq!(rays.len(), 4);
        assert_eq!(rays[0][..3], [0x3333, 20, 10]);
        assert_eq!(e.global::<u32>(PRESSED_BUTTON_011A0BA0), 2);
        let pick = calls(&e, PICK_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, SET_MEMBER_0X14)[1..],
            [vec![pick, 0x700], vec![pick, 0x701], vec![pick, 0x702]]
        );
        // A release on the pressed button answers it plus 1.
        e.call(
            0x007f_8720,
            &args![this, 0u32, 1u32, outs, outs + 4, outs + 8],
        );
        assert_eq!(e.mem.u32(outs + 8), 3);
        assert_eq!(e.global::<u32>(PRESSED_BUTTON_011A0BA0), u32::MAX);
        // Without the third button: false, nothing picked.
        e.mem.set_u32(this + BUTTONS + 8, 0);
        e.call_log = Some(vec![]);
        let hit = e.call(
            0x007f_8720,
            &args![this, 1u32, 0u32, outs, outs + 4, outs + 8],
        );
        assert_eq!(hit.u32() & 0xff, 0);
        assert!(calls(&e, PICK_CONSTRUCT).is_empty());
    }

    #[test]
    fn value_bursts_by_the_settings() {
        let mut e = Engine::new();
        world(&mut e);
        quiet(
            &mut e,
            &[
                RENDERED_MENU_007FB250,
                RENDERED_MENU_007FB2B0,
                TRIGGER_BURST,
            ],
        );
        e.register(INTERFACE_SINGLETON, |_, _| 0x1111u32.into_ret());
        e.register(INTERFACE_RENDERED_MENU, |_, _| 0x2222u32.into_ret());
        e.register(VALUE_005C5420, |_, _| st0(1.0));
        for (setting, value) in [
            (SETTING_011DB4B4, 2.0f32),
            (SETTING_011DB4E0, 5.0),
            (SETTING_011DB4EC, 0.25),
            (SETTING_011DB3FC, 0.5),
            (SETTING_011DB414, 0.75),
        ] {
            e.mem.set_f32(setting, value);
        }
        e.call_log = Some(vec![]);
        e.call(0x007f_8a80, &args![0u32]);
        assert_eq!(
            calls(&e, RENDERED_MENU_007FB250),
            vec![vec![0x2222, 0.25f32.to_bits(), 0.5f32.to_bits()]]
        );
        assert!(calls(&e, RENDERED_MENU_007FB2B0).is_empty());
        assert_eq!(calls(&e, TRIGGER_BURST).len(), 1);
        e.register(VALUE_005C5420, |_, _| st0(3.0));
        e.call_log = Some(vec![]);
        e.call(0x007f_8a80, &args![0u32]);
        assert!(calls(&e, RENDERED_MENU_007FB250).is_empty());
        assert_eq!(calls(&e, RENDERED_MENU_007FB2B0)[0][1], 0.75f32.to_bits());
        // At the window's end: no second call.
        e.register(VALUE_005C5420, |_, _| st0(7.0));
        e.call_log = Some(vec![]);
        e.call(0x007f_8a80, &args![0u32]);
        assert!(calls(&e, RENDERED_MENU_007FB2B0).is_empty());
    }

    #[test]
    fn tab_knob_position_copies_and_ticks() {
        let mut e = Engine::new();
        world(&mut e);
        sounds(&mut e);
        let this = manager(&mut e);
        e.mem.set_f32(this + TAB_KNOB_POSITIONS + 8, 0.75);
        e.mem.set_f32(SETTING_TAB_KNOB_RATE, 4.0);
        e.set_global(PIPBOY_OPEN_011DB378, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x007f_a0f0, &args![this, 2u32, 1u32]);
        assert_eq!(e.mem.f32(this + THIRD_KNOB_TARGET), 0.75);
        assert_eq!(e.mem.f32(this + THIRD_KNOB_RATE), 4.0);
        assert_eq!(
            calls(&e, AUDIO_GET_SOUND_BY_NAME)[0][2..],
            [SOUND_TAB, 0x121]
        );
        // The same position again: nothing.
        e.mem.set_f32(SETTING_TAB_KNOB_RATE, 5.0);
        e.call_log = Some(vec![]);
        e.call(0x007f_a0f0, &args![this, 2u32, 1u32]);
        assert_eq!(e.mem.f32(this + THIRD_KNOB_RATE), 4.0);
        assert!(calls(&e, SOUND_PLAY).is_empty());
    }

    #[test]
    fn fn_007fa220_plays_the_idle_groups() {
        let mut e = Engine::new();
        let player = world(&mut e);
        quiet(
            &mut e,
            &[
                PLAY_GROUP,
                ANIMATION_008EEAA0,
                ANIMATION_00491180,
                ANIMATION_00493900,
            ],
        );
        let interface = e.mem.alloc(0x500);
        e.mem.set_u32(interface + 0x4bc, 3);
        e.register_double(INTERFACE_SINGLETON, move |_, _| interface.into_ret());
        e.register(CHECK_00703D50, |_, _| 0x100u32.into_ret());
        e.register(INTERFACE_BYTE_00424940, |_, _| 0x105u32.into_ret());
        e.register(GET_ANIMATION, |_, _| 0x6000u32.into_ret());
        e.register(ANIMATION_00491040, |_, _| 0x6100u32.into_ret());
        e.register(SEQUENCE_0048F7F0, |_, _| 0x6200u32.into_ret());
        e.register(SEQUENCE_WORD_005F2420, |_, _| 2u32.into_ret());
        e.register(ANIMATION_TEST_00494710, |_, _| 1u32.into_ret());
        e.register(TIMER_FLOAT, |_, _| st0(0.5));
        e.mem.set_f32(TIME_ARGUMENT_01012054, -1.0);
        e.call_log = Some(vec![]);
        e.call(0x007f_a220, &args![0u32]);
        assert_eq!(
            calls(&e, PLAY_GROUP),
            vec![vec![0x6000, 1, 1, u32::MAX, u32::MAX]]
        );
        assert_eq!(calls(&e, ANIMATION_008EEAA0), vec![vec![0x6000, 0x17]]);
        assert_eq!(
            calls(&e, ANIMATION_00491180),
            vec![vec![0x6000, player, 0.5f32.to_bits(), (-1.0f32).to_bits()]]
        );
        assert_eq!(calls(&e, ANIMATION_00493900), vec![vec![0x6000, player]]);
        // The interface byte 4 stops it.
        e.register(INTERFACE_BYTE_00424940, |_, _| 0x104u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007f_a220, &args![0u32]);
        assert!(calls(&e, GET_ANIMATION).is_empty());
    }

    #[test]
    fn light_effect_fades_and_stops() {
        let mut e = Engine::new();
        let player = world(&mut e);
        quiet(
            &mut e,
            &[
                LIGHT_SET_DIMMER,
                SET_CULLED,
                FIXED_STRING_DESTRUCT,
                PROPERTY_SET_0068C9F0,
                UPDATE_PROPERTIES,
            ],
        );
        // The clamps: min with the second, max with the first.
        e.register(CLAMP_0040EBD0, |_, a| {
            st0(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
        });
        e.register(CLAMP_00404010, |_, a| {
            st0(f32::from_bits(a[0]).max(f32::from_bits(a[1])))
        });
        e.register(TIMER_TIME, |_, _| 100u32.into_ret());
        let light = e.mem.alloc(4);
        e.mem.set_u32(light, 0x0e0e);
        e.register_double(PLAYER_LIGHT_00572750, move |_, _| light.into_ret());
        let this = manager(&mut e);
        e.mem.set_f32(SETTING_FADE_LIGHT, 4.0);
        e.mem.set_f32(SETTING_LIGHT_EFFECT, 4.0);
        // FadeLightEffect: only with the first-person light on.
        e.call(0x007f_a7a0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + LIGHT_EFFECT_ON), 0);
        e.set(
            Ptr::<FOPipboyManager>::new(this),
            FOPipboyManager::b1stPersonLightOn,
            1,
        );
        e.call(0x007f_a7a0, &args![this, 0u32]);
        assert_eq!(e.mem.f32(this + LIGHT_EFFECT_START), 100.0);
        assert_eq!(e.mem.u8(this + LIGHT_EFFECT_ON), 1);
        assert_eq!(e.mem.f32(this + LIGHT_EFFECT_END), 1.0);
        // FadeLight at 101: a quarter.
        e.call_log = Some(vec![]);
        e.call(0x007f_a810, &args![this, 101.0f32.to_bits()]);
        assert_eq!(calls(&e, PLAYER_LIGHT_00572750), vec![vec![player]]);
        assert_eq!(
            calls(&e, LIGHT_SET_DIMMER),
            vec![vec![0x0e0e, 0.25f32.to_bits()]]
        );
        // Fading out: the value is reversed.
        e.call(0x007f_a7a0, &args![this, 1u32]);
        e.call_log = Some(vec![]);
        e.call(0x007f_a810, &args![this, 101.0f32.to_bits()]);
        assert_eq!(calls(&e, LIGHT_SET_DIMMER)[0][1], 0.75f32.to_bits());

        // UpdateLightEffect at the end (value 0 = end 0): stops and hides.
        let root = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(root, vtable);
        e.mem.set_u32(vtable + 0x9c, 0x7600_009c);
        e.register(0x7600_009c, |_, _| 0u32.into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[1].into_ret());
        e.mem.set_u32(this + PIPBOY_ROOT, root);
        e.call_log = Some(vec![]);
        e.call(0x007f_a540, &args![this, 200.0f32.to_bits()]);
        assert_eq!(e.mem.u8(this + LIGHT_EFFECT_ON), 0);
        // Two lookups for the effect's own light, two for each show.
        assert_eq!(calls(&e, 0x7600_009c).len(), 6);
        assert_eq!(calls(&e, LIGHT_SET_DIMMER).len(), 1);
        // Not running: nothing.
        e.call_log = Some(vec![]);
        e.call(0x007f_a540, &args![this, 200.0f32.to_bits()]);
        assert!(calls(&e, LIGHT_SET_DIMMER).is_empty());
    }
}
