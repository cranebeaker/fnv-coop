//! `fallout/interface/forenderedmenuracesex.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `FORenderedMenuRaceSex` is the rendered 3D menu of the character creation
//! and the barber / plastic surgeon: it clones the player's 3D data into a
//! private scene (a node holding the player, its lights and a camera), shows it
//! on the menu and tears it down again. The class derives from
//! `FORenderedMenu`; on PC the fields of the derived class sit lower than in
//! the Xbox PDB because the PC build's matrices are 0x24 bytes (not 16-byte
//! aligned 0x30), so the layout below is the PC one, confirmed by the code.
//!
//! A few functions of this unit are methods of other classes the linker
//! happened to place here (`fn_007fd620` sets `FORenderedMenu::spCamera`;
//! `fn_007fd640` returns the pointer at +0x90 of the `Interface`;
//! `fn_007fdf10` sets `FORenderedMenu::spAccumulator`; `fn_007fe910` and
//! `fn_007fe930` are one-call wrappers); they are named by address.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`.
//!
//! The compiler's exception-unwinding frames and security cookie are not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPointer<T>::operator=(T *)`: `this` is the address of the pointer slot.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer<T>::operator T *()` (`*this`); also used for other one-member
/// wrappers.
const POINTER_VALUE: u32 = 0x0055_9450;
/// `NiPointer` destructor.
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `NiFixedString` constructor from a C string / destructor.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// Pointer to the `float` value of the `Setting` in ECX.
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
/// Pointer to the `int` value of the `Setting` in ECX.
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// Allocator (`operator new` of the engine's heap, cdecl).
const ALLOCATE: u32 = 0x00aa_13e0;
/// The `Interface` singleton getter.
const INTERFACE_GETTER: u32 = 0x004b_7210;
/// Sets the culled flag of a scene object (`this`, culled).
const SET_CULLED: u32 = 0x0045_0f90;
/// Gets the scene node `n` from the scene-graph holder (cdecl, 1 argument).
const SCENE_NODE_GETTER: u32 = 0x0045_0b80;
/// `NiPoint3` constructor (`this`, x, y, z) and its `NiColor` twin.
const POINT_CONSTRUCT: u32 = 0x0043_d410;
const COLOR_CONSTRUCT: u32 = 0x0041_6870;
/// `NiColorA` constructor (`this`, r, g, b, a).
const COLOR_A_CONSTRUCT: u32 = 0x0041_4430;
/// `NiMatrix3` default constructor.
const MATRIX_CONSTRUCT: u32 = 0x0068_15c0;
/// `NiAVObject::SetLocalTranslate(const NiPoint3 &)`-like setter (`this`,
/// point).
const SET_TRANSLATE_POINT: u32 = 0x00a5_9c60;
/// `NiAVObject::SetLocalRotate(const NiMatrix3 &)` (`this`, matrix).
const SET_ROTATE: u32 = 0x0043_fa80;
/// Sets a translation from three floats (`this`, x, y, z).
const SET_TRANSLATE: u32 = 0x004b_c1f0;
/// Vector setter taking a `const NiPoint3 &` (`this`, point); used for the
/// saved player translation and for the player light.
const SET_VECTOR: u32 = 0x0044_0460;
/// Reads the translation / rotation of a scene object (pointer into it).
const GET_TRANSLATE: u32 = 0x0043_c490;
const GET_ROTATE: u32 = 0x006a_9540;
/// `_ftol2_sse`: truncates the float in ST0 (passed as an `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// `tan`-like float function (cdecl, float in, ST0 out).
const FLOAT_TRIG: u32 = 0x0057_dd70;
/// Identity `NiMatrix3` in the exe's data.
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// `NiNode` name used to find the player's `Bip01` bone.
const BIP01_NAME: u32 = 0x0101_e460;
/// The `float` 1.0 factor `Initialize` / `OnMenuOpen` multiply scales by
/// (`double`).
const SCALE_DOUBLE: u32 = 0x0102_1928;
/// `double` multiplier of the field-of-view setting.
const FOV_DOUBLE: u32 = 0x0102_3128;
/// `float` constants of the frustum and lights.
const FRUSTUM_SCALE: u32 = 0x0101_6264;
const FRUSTUM_ANGLE: u32 = 0x0105_59cc;

layout! {
    /// `FORenderedMenuRaceSex` (Xbox PDB) on PC: the fields this unit uses,
    /// at their PC offsets. The base `FORenderedMenu` part ends at 0xD8;
    /// the PC object is 0x170 bytes (the Xbox one 0x180).
    pub struct FORenderedMenuRaceSex: 0x170 {
        /// `spCamera` (Xbox PDB, `FORenderedMenu`): `NiPointer<NiCamera>`.
        0x24 spCamera: Ptr,
        /// `spAccumulator` (Xbox PDB, `FORenderedMenu`).
        0xD4 spAccumulator: Ptr,
        /// `spButtons[0]` (Xbox PDB): four `NiPointer<NiAVObject>`.
        0xD8 spButtons0: Ptr,
        0xDC spButtons1: Ptr,
        0xE0 spButtons2: Ptr,
        0xE4 spButtons3: Ptr,
        /// `spBtnGlow[0]` (Xbox PDB): four `NiPointer<NiAVObject>`.
        0xE8 spBtnGlow0: Ptr,
        0xEC spBtnGlow1: Ptr,
        0xF0 spBtnGlow2: Ptr,
        0xF4 spBtnGlow3: Ptr,
        /// `spGeometry` (Xbox PDB): `NiPointer<BSFadeNode>`.
        0xF8 spGeometry: Ptr,
        /// `spPlayer3D` (Xbox PDB).
        0xFC spPlayer3D: Ptr,
        /// `spPlayerParent` (Xbox PDB).
        0x100 spPlayerParent: Ptr,
        /// `spClonedPlayerCamera` (Xbox PDB).
        0x104 spClonedPlayerCamera: Ptr,
        /// `spPlayerLights` (Xbox PDB).
        0x108 spPlayerLights: Ptr,
        /// `spPlayer3DParent` (Xbox PDB).
        0x10C spPlayer3DParent: Ptr,
        /// `ptSavedPlayerTranslation` (Xbox PDB): `NiPoint3`, 0xC bytes.
        0x110 ptSavedPlayerTranslation: f32,
        /// `mtxSavedPlayerRotation` (Xbox PDB): `NiMatrix3`, 0x24 bytes on PC.
        0x11C mtxSavedPlayerRotation: f32,
        /// `ptSavedBip01Translation` (Xbox PDB): `NiPoint3`.
        0x140 ptSavedBip01Translation: f32,
        /// `bPlayer3DCulled` (Xbox PDB).
        0x14C bPlayer3DCulled: u8,
        /// `fDistortStart` (Xbox PDB).
        0x150 fDistortStart: f32,
        /// `fDistortPos` (Xbox PDB).
        0x154 fDistortPos: f32,
        /// `bFakeDistortActive` (Xbox PDB).
        0x158 bFakeDistortActive: bool,
        /// `fFakeDistortDuration` (Xbox PDB).
        0x164 fFakeDistortDuration: f32,
        /// `fNextFakeDistortTime` (Xbox PDB).
        0x168 fNextFakeDistortTime: f32,
    }
}

fn pointer_value(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_VALUE, &args![slot]).u32()
}

fn assign_pointer(e: &mut Engine, slot: u32, value: u32) {
    e.call(NI_POINTER_ASSIGN, &args![slot, value]);
}

fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let pointer = e.call(SETTING_FLOAT_POINTER, &args![setting]).u32();
    e.mem.f32(pointer)
}

fn setting_int(e: &mut Engine, setting: u32) -> i32 {
    let pointer = e.call(SETTING_INT_POINTER, &args![setting]).u32();
    e.mem.i32(pointer)
}

/// A zero `NiPoint3` built in the caller's local `point`.
fn zero_point(e: &mut Engine, point: u32) -> u32 {
    e.call(POINT_CONSTRUCT, &args![point, 0.0f32, 0.0f32, 0.0f32]);
    point
}

/// Looks a child up by name: builds an `NiFixedString` in the caller's local `fixed`, calls
/// the virtual `GetObjectByName` (slot +0x9C) and destroys the string again.
fn find_child_by_name(e: &mut Engine, fixed: u32, parent: u32, name: u32) -> u32 {
    e.call(FIXED_STRING_CONSTRUCT, &args![fixed, name]);
    let child = e.vcall(parent, 0x9c, &args![fixed]).u32();
    e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
    child
}

/// The camera frustum of the player view: the four tangent-scaled edges.
/// `tan(fov)` is evaluated once per edge, as the game does.
fn fov_value(e: &mut Engine) -> f32 {
    let angle = setting_float(e, 0x0120_3150);
    let scaled = angle as f64 * e.global::<f64>(FOV_DOUBLE);
    let factor = setting_float(e, 0x011d_b5ec);
    (factor as f64 * scaled) as f32
}

fn trig(e: &mut Engine, value: f32) -> f32 {
    e.call(FLOAT_TRIG, &args![value]).f32()
}

/// Copies `words` dwords.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// The camera ratio `Initialize` (`007fd173`..`007fd1cb`) and `OnMenuOpen` (`007fdd7a`..`007fddd6`)
/// compute: the value of the renderer's virtual getter at +0x100 (`FSTP float`) divided by the
/// `Setting` at 0x01203144 (`FSTP float`), the quotient stored as a `float`. Both functions keep
/// the two `float`s in the stack words that are their `NiFrustum` local's `m_fNear` (+0x10, the
/// setting) and `m_fFar` (+0x14, the getter's value), which `NiFrustum::NiFrustum` (`00a71b70`)
/// leaves alone; so the frustum they hand to `NiCamera::SetViewFrustum` (`00a6faf0`) has that
/// near and far. Writes them into `frustum` and answers the ratio.
fn renderer_ratio(e: &mut Engine, frustum: u32) -> f32 {
    let renderer = e.call(0x0045_c670, &args![]).u32();
    let far = x87_store32(e.vcall(renderer, 0x100, &args![]).f64());
    e.mem.set_u32(frustum + 0x14, far);
    let near = x87_quiet_bits(setting_float(e, 0x0120_3144).to_bits());
    e.mem.set_u32(frustum + 0x10, near);
    let ratio = x87_store32(x87_div(x87_load32(far), x87_m32(near)));
    f32::from_bits(x87_quiet_bits(ratio))
}

// Translated from 007fc970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenuRaceSex::Initialize` (Xbox PDB): builds the menu's private
/// scene. Finds (or loads) the menu geometry, looks up the button and glow
/// nodes by name, creates the camera with its frustum and, when the
/// accumulator for the scene does not exist yet, a light; then calls
/// `007fe560` (`InitLight`) and `007fc150`.
pub fn forendered_menu_race_sex_initialize(e: &mut Engine, this: Ptr<FORenderedMenuRaceSex>) {
    type M = FORenderedMenuRaceSex;
    let me = this.addr();
    // The exe's frame (`SUB ESP,0x208` under the exception record, then `ESI` and the record
    // link): the colours at `EBP-0x78` and `EBP-0x34`, the body's name at `EBP-0x104`, the
    // button names from `EBP-0x108` down by 4, the rotations at `EBP-0x58` and `EBP-0xec`, their
    // product at `EBP-0x148`, the zero points at `EBP-0x24`, `EBP-0x18` and `EBP-0x100`, the
    // axes at `EBP-0x9c`, the frustum at `EBP-0xb8` and the light colours at `EBP-0x16c`,
    // `EBP-0x178` and `EBP-0x184`.
    e.with_stack(0x21c, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        e.set(this, M::bFakeDistortActive, false);
        // The first fake distortion happens a random time after the start.
        let ticks = e.call(0x0082_5c00, &args![0x011f_6394u32]).u32() as f64;
        let random = e.call(0x00ec_adb8, &args![]).i32();
        let modulus = setting_int(e, 0x011d_b4f8);
        let remainder = random.wrapping_rem(modulus);
        e.set(
            this,
            M::fNextFakeDistortTime,
            (remainder as f64 + ticks) as f32,
        );
        let duration = setting_float(e, 0x011d_b4d0);
        e.set(this, M::fFakeDistortDuration, duration);
        let color = local(0x1a4);
        e.call(
            COLOR_A_CONSTRUCT,
            &args![color, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let holder = e.call(0x0071_8b60, &args![]).u32();
        if e.call(0x0071_9060, &args![holder, 3u32, color]).bool() {
            e.call(0x007f_7fb0, &args![me, color]);
        }
        // The menu geometry.
        let geometry_slot = me + 0xf8;
        let node;
        if pointer_value(e, geometry_slot) == 0 {
            let owner = e.call(0x0096_11e0, &args![me]).u32();
            let menu = e.call(0x00a0_3c90, &args![owner]).u32();
            let name = e.call(0x007f_d640, &args![menu]).u32();
            let loader: u32 = e.global(0x011c_3b3c);
            node = e
                .call(
                    0x0044_7080,
                    &args![loader, name, 0u32, 1u32, 0u32, 0u32, 0u32],
                )
                .u32();
            assign_pointer(e, geometry_slot, node);
            e.call(0x004b_7660, &args![node]);
        } else {
            node = pointer_value(e, geometry_slot);
        }
        let scale = setting_float(e, 0x011d_b544);
        e.call(0x0044_0490, &args![node, scale]);
        e.call(0x004d_c540, &args![me, node]);
        let body = find_child_by_name(e, local(0x118), node, 0x0107_8a54);
        e.call(0x007f_ae70, &args![me, 0u32]);
        e.call(0x0081_0530, &args![me, 0u32]);
        assign_pointer(e, me + 0xfc, 0);
        assign_pointer(e, me + 0x104, 0);
        // The four buttons and their glows, by name.
        let names = [
            0x0107_8a48u32,
            0x0107_8a3c,
            0x0107_8a30,
            0x0107_8a24,
            0x0107_8a18,
            0x0107_8a0c,
            0x0107_8a00,
            0x0107_89f4,
        ];
        for (i, name) in names.iter().enumerate() {
            let fixed = local(0x114 - 4 * i as u32);
            e.call(FIXED_STRING_CONSTRUCT, &args![fixed, *name]);
            let child = e.vcall(node, 0x9c, &args![fixed]).u32();
            assign_pointer(e, me + 0xd8 + 4 * i as u32, child);
            e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
        }
        e.set(this, M::fDistortStart, 0.0);
        e.set(this, M::fDistortPos, 0.0);
        e.call(0x008d_7f00, &args![me, 0.0f32]);
        // Rotation of the geometry: the product of two rotations by the same
        // angle.
        let first = local(0x1c4);
        e.call(MATRIX_CONSTRUCT, &args![first]);
        let second = local(0x130);
        e.call(MATRIX_CONSTRUCT, &args![second]);
        let angle: f32 = e.global(FRUSTUM_ANGLE);
        e.call(0x0043_f850, &args![first, angle]);
        e.call(0x004a_0c90, &args![second, angle]);
        let product = local(0xd4);
        let rotation = e.call(0x0043_f8d0, &args![second, product, first]).u32();
        e.call(SET_ROTATE, &args![node, rotation]);
        // Position of the geometry; the depth depends on the menu's context.
        let context = e.call(0x009b_6600, &args![me]).i32();
        let mut depth: f32 = if context == 1 || e.call(0x009b_6600, &args![me]).i32() == 0 {
            e.global(0x0101_e33c)
        } else {
            setting_float(e, 0x011d_b570)
        };
        if !e.call(0x0070_7b50, &args![]).bool() && e.mem.u8(0x011f_9426) == 0 {
            depth = (depth as f64 * e.global::<f64>(0x0101_de30)) as f32;
        }
        let context = e.call(0x009b_6600, &args![me]).i32();
        if context == 1 || e.call(0x009b_6600, &args![me]).i32() == 0 {
            let z = setting_float(e, 0x011d_b5cc);
            e.call(SET_TRANSLATE, &args![node, depth, 0.0f32, z]);
        } else {
            let z = setting_float(e, 0x011d_b5cc);
            let y = setting_float(e, 0x011d_b594);
            e.call(SET_TRANSLATE, &args![node, depth, y, z]);
        }
        let point = zero_point(e, local(0x1f8));
        e.call(SET_TRANSLATE_POINT, &args![node, point]);
        // The menu camera.
        let memory = e.call(ALLOCATE, &args![0x114u32]).u32();
        let camera = if memory != 0 {
            e.call(0x00a7_12f0, &args![memory]).u32()
        } else {
            0
        };
        let background = local(0x1e8);
        e.call(
            COLOR_A_CONSTRUCT,
            &args![background, 0.0f32, 1.0f32, 1.0f32, 0.0f32],
        );
        e.call(0x0071_2e60, &args![camera, background]);
        let axes = local(0x180);
        e.call(MATRIX_CONSTRUCT, &args![axes]);
        // (The matrix built here is not used afterwards.)
        e.call(0x0071_33b0, &args![axes, 0u32, 0.0f32, 0.0f32, 1.0f32]);
        e.call(0x0071_33b0, &args![axes, 1u32, 1.0f32, 0.0f32, 0.0f32]);
        e.call(0x0071_33b0, &args![axes, 2u32, 0.0f32, 1.0f32, 0.0f32]);
        e.call(SET_ROTATE, &args![camera, IDENTITY_MATRIX]);
        e.call(0x0073_3fd0, &args![camera, 1.0f32]);
        let frustum = local(0x164);
        e.call(0x00a7_1b70, &args![frustum, 0u32]);
        let ratio = renderer_ratio(e, frustum);
        e.call(0x0050_7700, &args![camera, ratio]);
        let wide = e.mem.u8(0x011f_9426) != 0;
        let width = if wide {
            e.global::<u32>(0x011f_9434)
        } else {
            e.call(0x004d_c200, &args![]).u32()
        };
        let height = e.call(0x004d_c1f0, &args![]).u32();
        let aspect = (width as i32 as f64 / height as i32 as f64) as f32;
        let k: f32 = e.global(FRUSTUM_SCALE);
        let fov = fov_value(e);
        let t = trig(e, fov);
        e.mem.set_f32(frustum, (-(t as f64) * k as f64) as f32);
        let t = trig(e, fov);
        e.mem.set_f32(frustum + 4, (t as f64 * k as f64) as f32);
        let t = trig(e, fov);
        e.mem.set_f32(
            frustum + 0xc,
            (-(t as f64) * aspect as f64 * k as f64) as f32,
        );
        let t = trig(e, fov);
        e.mem
            .set_f32(frustum + 8, (t as f64 * aspect as f64 * k as f64) as f32);
        e.mem.set_u8(frustum + 0x18, 0);
        e.call(0x00a6_faf0, &args![camera, frustum]);
        let angle = setting_float(e, 0x011d_b5ec);
        e.call(0x0070_9d50, &args![camera, angle]);
        let point = zero_point(e, local(0x204));
        e.call(SET_TRANSLATE_POINT, &args![camera, point]);
        e.call(0x007f_d620, &args![me, camera]);
        // The shape found by name: its scene node (virtual +0x18), or the node
        // of its first child (virtual +0xC).
        if e.vcall(body, 0x18, &args![]).u32() != 0 {
            let found = e.vcall(body, 0x18, &args![]).u32();
            e.call(0x007f_9010, &args![me, found]);
        } else if e.vcall(body, 0xc, &args![]).u32() != 0 {
            let child = e.vcall(body, 0xc, &args![]).u32();
            let child = e.call(0x0043_b4a0, &args![child, 0u32]).u32();
            let found = e.vcall(child, 0x18, &args![]).u32();
            e.call(0x007f_9010, &args![me, found]);
        }
        e.call(0x004d_c540, &args![me, node]);
        let value = e.call(0x0040_df90, &args![]).u32();
        let holder = e.call(0x007f_a950, &args![me]).u32();
        e.call(0x00a5_b230, &args![holder, value]);
        let memory = e.call(ALLOCATE, &args![0x1cu32]).u32();
        let entry = if memory != 0 {
            e.call(0x0043_91c0, &args![memory]).u32()
        } else {
            0
        };
        let holder = e.call(0x007f_a950, &args![me]).u32();
        e.call(0x0043_9410, &args![holder, entry]);
        // The scene graph: the geometry's radius decides the node scale, and
        // when the graph has no entry for the scene yet it gets a light.
        let bound = e.call(0x0043_d450, &args![node]).u32();
        let radius = e.call(0x0084_d030, &args![bound]).f32();
        let radius = (radius as f64 * e.global::<f64>(SCALE_DOUBLE)) as f32;
        let graph = e.call(SCENE_NODE_GETTER, &args![1u32]).u32();
        let existing = e.call(0x00b5_ca70, &args![graph, node, radius]).u32();
        if existing == 0 {
            let memory = e.call(ALLOCATE, &args![0xfcu32]).u32();
            let light = if memory != 0 {
                e.call(0x00a7_d670, &args![memory]).u32()
            } else {
                0
            };
            e.vcall(node, 0xdc, &args![light, 1u32]);
            let ambient = local(0xb0);
            let green: f32 = e.global(0x0104_f7c4);
            let color = e
                .call(COLOR_CONSTRUCT, &args![ambient, 0.0f32, green, 0.0f32])
                .u32();
            e.call(SET_VECTOR, &args![light, color]);
            let k: f32 = e.global(FRUSTUM_SCALE);
            let diffuse = local(0xa4);
            let color = e.call(COLOR_CONSTRUCT, &args![diffuse, k, k, k]).u32();
            e.call(0x004b_c2e0, &args![light, color]);
            let specular = local(0x98);
            let red: f32 = e.global(0x0101_3974);
            let color = e
                .call(COLOR_CONSTRUCT, &args![specular, red, 0.0f32, 0.0f32])
                .u32();
            e.call(0x0050_dd50, &args![light, color]);
            e.call(0x0050_dd20, &args![light, 1.0f32]);
            let point = zero_point(e, local(0x11c));
            e.call(SET_TRANSLATE_POINT, &args![light, point]);
            let graph = e.call(SCENE_NODE_GETTER, &args![1u32]).u32();
            e.call(0x00b5_c940, &args![graph, light, 0u32]);
            forendered_menu_race_sex_init_light(e, this);
        }
        e.call(0x007f_c150, &args![me]);
    });
}

// Translated from 007fd620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `FORenderedMenu::spCamera` (the `NiPointer<NiCamera>` at +0x24).
pub fn fn_007fd620(e: &mut Engine, this: Ptr<FORenderedMenuRaceSex>, camera: u32) {
    assign_pointer(e, this.addr() + 0x24, camera);
}

// Translated from 007fd640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer held by the `NiPointer` at +0x90 of `this` (the `Interface`
/// when called from `Draw`).
pub fn fn_007fd640(e: &mut Engine, this: Ptr) -> u32 {
    pointer_value(e, this.addr() + 0x90)
}

// Translated from 007fd660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-scales the player's scene: the radius of the player 3D's bound times
/// the `double` at 0x01021928 and `scale` is passed with `spPlayerParent` to
/// the scene graph's `b5ca70`.
pub fn fn_007fd660(e: &mut Engine, this: Ptr<FORenderedMenuRaceSex>, scale: f32) {
    let me = this.addr();
    let player = pointer_value(e, me + 0xfc);
    let bound = e.call(0x0043_d450, &args![player]).u32();
    let radius = e.call(0x0084_d030, &args![bound]).f32();
    let value = ((radius as f64 * e.global::<f64>(SCALE_DOUBLE)) * scale as f64) as f32;
    let parent = pointer_value(e, me + 0x100);
    let graph = e.call(SCENE_NODE_GETTER, &args![3u32]).u32();
    e.call(0x00b5_ca70, &args![graph, parent, value]);
}

// Translated from 007fd6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenuRaceSex::OnMenuOpen` (Xbox PDB): when the menu is shown
/// (`00974d90`), saves the player's 3D translation, rotation and `Bip01`
/// translation, moves the player's scene into a private parent with its own
/// light node and cloned camera, sets up the menu node (`menu_node` is the
/// tile menu's object) and the buttons.
pub fn forendered_menu_race_sex_on_menu_open(e: &mut Engine, this: Ptr<FORenderedMenuRaceSex>) {
    let me = this.addr();
    // The exe's frame (`SUB ESP,0xcc` under the exception record, then `ESI`, `EDI` and the
    // record link): the zero points at `EBP-0x24`, `EBP-0x6c` and `EBP-0x18`, the bone name at
    // `EBP-0x7c`, the colour at `EBP-0x34`, the up and forward vectors at `EBP-0x40` and
    // `EBP-0x98`, the frustum at `EBP-0x60`.
    e.with_stack(0xe4, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        let global_a: u32 = e.global(0x011d_ea0c);
        e.call(0x0087_1dc0, &args![global_a]);
        e.mem.set_u8(0x011a_d884, 0);
        if !e.call(0x0097_4d90, &args![me]).bool() {
            return;
        }
        let cell_node = e.call(0x0055_85e0, &args![me]).u32();
        let point = zero_point(e, local(0xc0));
        e.call(SET_TRANSLATE_POINT, &args![cell_node, point]);
        let owner = e.call(0x0096_11e0, &args![me]).u32();
        let menu_node = e.call(0x00a0_3c90, &args![owner]).u32();
        if e.call(0x0068_38b0, &args![me]).u32() != 0 {
            let object = e.call(0x0068_38b0, &args![me]).u32();
            e.call(SET_CULLED, &args![object, 0u32]);
        }
        e.call(0x007f_b040, &args![me]);
        let graph = e.call(SCENE_NODE_GETTER, &args![1u32]).u32();
        let cell_node = e.call(0x0055_85e0, &args![me]).u32();
        e.vcall(graph, 0xdc, &args![cell_node, 1u32]);
        let controls = controls_get_instance(e);
        e.call(0x00a2_53d0, &args![controls]);
        // The world / player manager: prepares the player for the menu.
        let manager: u32 = e.global(0x011d_ea3c);
        let player_manager = e.call(0x008d_8520, &args![manager]).u32();
        e.vcall(player_manager, 0x458, &args![manager, 0u32]);
        e.call(0x0088_4f80, &args![manager]);
        e.call(0x008b_3a80, &args![manager, 0x400u32]);
        let object = e.call(0x0095_0a60, &args![manager, 0u32]).u32();
        e.call(0x0049_6080, &args![object, 0x14u32, 0.0f32]);
        let object = e.call(0x0095_0a60, &args![manager, 0u32]).u32();
        e.call(0x008e_eaa0, &args![object, 0x17u32]);
        let object = e.call(0x0095_0a60, &args![manager, 0u32]).u32();
        e.call(0x0049_4740, &args![object, 0u32, 1u32, -1i32, -1i32]);
        e.vcall(manager, 0x2f8, &args![0.0f32]);
        let player = e.call(0x0095_0bb0, &args![manager, 0u32]).u32();
        assign_pointer(e, me + 0xfc, player);
        // Save the player's placement.
        let player = pointer_value(e, me + 0xfc);
        let translation = e.call(GET_TRANSLATE, &args![player]).u32();
        copy_words(e, translation, me + 0x110, 3);
        let player = pointer_value(e, me + 0xfc);
        let rotation = e.call(GET_ROTATE, &args![player]).u32();
        copy_words(e, rotation, me + 0x11c, 9);
        let player = pointer_value(e, me + 0xfc);
        let fixed = local(0x68);
        e.call(FIXED_STRING_CONSTRUCT, &args![fixed, BIP01_NAME]);
        let bone = e.vcall(player, 0x9c, &args![fixed]).u32();
        let bone_translation = e.call(GET_TRANSLATE, &args![bone]).u32();
        copy_words(e, bone_translation, me + 0x140, 3);
        e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
        let player = pointer_value(e, me + 0xfc);
        let parent = e.call(0x0096_11e0, &args![player]).u32();
        assign_pointer(e, me + 0x10c, parent);
        let player = pointer_value(e, me + 0xfc);
        let culled = e.call(0x0045_6610, &args![player]).u8();
        e.mem.set_u8(me + 0x14c, culled);
        // A new parent node for the player.
        let memory = e.call(ALLOCATE, &args![0xacu32]).u32();
        let parent = if memory != 0 {
            e.call(0x00a5_ecb0, &args![memory, 0u32]).u32()
        } else {
            0
        };
        assign_pointer(e, me + 0x100, parent);
        let graph = e.call(SCENE_NODE_GETTER, &args![3u32]).u32();
        let parent = pointer_value(e, me + 0x100);
        e.vcall(graph, 0xdc, &args![parent, 1u32]);
        let loader: u32 = e.global(0x011c_3b3c);
        let lights = e
            .call(
                0x0044_7080,
                &args![loader, 0x0107_77a0u32, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assign_pointer(e, me + 0x108, lights);
        let parent = pointer_value(e, me + 0x100);
        let player = pointer_value(e, me + 0xfc);
        e.vcall(parent, 0xdc, &args![player, 1u32]);
        let parent = pointer_value(e, me + 0x100);
        let lights = pointer_value(e, me + 0x108);
        e.vcall(parent, 0xdc, &args![lights, 1u32]);
        let player = pointer_value(e, me + 0xfc);
        let bound = e.call(0x0043_d450, &args![player]).u32();
        let radius = e.call(0x0084_d030, &args![bound]).f32();
        let radius = (radius as f64 * e.global::<f64>(SCALE_DOUBLE)) as f32;
        let lights = pointer_value(e, me + 0x108);
        let light_node = e.vcall(lights, 0xc, &args![]).u32();
        let graph = e.call(SCENE_NODE_GETTER, &args![3u32]).u32();
        let found = e.call(0x00b5_ca70, &args![graph, light_node, radius]).u32();
        race_menu_light_check(e, me, lights, light_node, graph, found, radius);
        let parent = pointer_value(e, me + 0x100);
        let angle: f32 = e.global(0x0102_b3c8);
        e.call(0x0064_04b0, &args![parent, angle, 0.0f32, 0.0f32, 1.0f32]);
        let player = pointer_value(e, me + 0xfc);
        let height: f32 = e.global(0x0102_caf8);
        e.call(SET_TRANSLATE, &args![player, 0.0f32, height, 0.0f32]);
        let player = pointer_value(e, me + 0xfc);
        e.call(SET_ROTATE, &args![player, IDENTITY_MATRIX]);
        let point = zero_point(e, local(0x78));
        let parent = pointer_value(e, me + 0x100);
        e.call(SET_TRANSLATE_POINT, &args![parent, point]);
        let parent = pointer_value(e, me + 0x100);
        e.call(0x007f_df10, &args![menu_node, parent]);
        // The camera of the cloned scene.
        let memory = e.call(ALLOCATE, &args![0x114u32]).u32();
        let camera = if memory != 0 {
            e.call(0x00a7_12f0, &args![memory]).u32()
        } else {
            0
        };
        assign_pointer(e, me + 0x104, camera);
        let background = local(0xb0);
        e.call(
            COLOR_A_CONSTRUCT,
            &args![background, 0.0f32, 1.0f32, 1.0f32, 0.0f32],
        );
        let camera = pointer_value(e, me + 0x104);
        e.call(0x0071_2e60, &args![camera, background]);
        let camera = pointer_value(e, me + 0x104);
        e.call(0x0073_3fd0, &args![camera, 1.0f32]);
        let up = local(0xa4);
        e.call(COLOR_CONSTRUCT, &args![up, 0.0f32, 1.0f32, 0.0f32]);
        let forward = local(0x4c);
        let forward = e
            .call(COLOR_CONSTRUCT, &args![forward, 0.0f32, 0.0f32, 1.0f32])
            .u32();
        let camera = pointer_value(e, me + 0x104);
        e.call(0x00a7_01b0, &args![camera, up, forward]);
        let frustum = local(0x84);
        e.call(0x00a7_1b70, &args![frustum, 0u32]);
        let ratio = renderer_ratio(e, frustum);
        let camera = pointer_value(e, me + 0x104);
        e.call(0x0050_7700, &args![camera, ratio]);
        let fov = fov_value(e);
        let t = trig(e, fov);
        e.mem.set_f32(frustum, -t);
        let t = trig(e, fov);
        e.mem.set_f32(frustum + 4, t);
        let t = trig(e, fov);
        e.mem.set_f32(frustum + 0xc, -t);
        let t = trig(e, fov);
        e.mem.set_f32(frustum + 8, t);
        e.mem.set_u8(frustum + 0x18, 0);
        let camera = pointer_value(e, me + 0x104);
        e.call(0x00a6_faf0, &args![camera, frustum]);
        let point = zero_point(e, local(0xcc));
        let camera = pointer_value(e, me + 0x104);
        e.call(SET_TRANSLATE_POINT, &args![camera, point]);
        let player = pointer_value(e, me + 0xfc);
        e.call(SET_CULLED, &args![player, 0u32]);
        let k: f32 = e.global(0x0107_8a5c);
        e.call(0x007b_1970, &args![menu_node, k, 1u32]);
        e.call(0x007b_1b60, &args![menu_node, 0.0f32, 1u32]);
        let value = e.call(0x007d_cc00, &args![menu_node]).u32();
        forendered_menu_race_sex_light_button(e, this, value);
    });
}

/// Diagnostic only, not exe behaviour (`NV_LIGHT_CHECK`, see
/// `engine_bs::units::bsshader::shadowscenenode::light_check`): after `OnMenuOpen` added the
/// face lights to scene node 3, prints the loaded model and its node, how many lights
/// `AddAllLightsFromNode_Recurse` found, the radius it gave them (3 x the player's bound
/// radius), the player's bound, and each light in node 3's light list (+0xb4) and add queue (+0xe4).
fn race_menu_light_check(
    e: &Engine,
    me: u32,
    model: u32,
    light_node: u32,
    graph: u32,
    found: u32,
    radius: f32,
) {
    use crate::units::bsshader::shadowscenenode::light_check;
    if !light_check::on() {
        return;
    }
    let player = e.mem.u32(me + 0xfc);
    let bound = if player != 0 {
        e.mem.u32(player + 0x20)
    } else {
        0
    };
    let bound_text = if bound != 0 && e.mem.is_mapped(bound) {
        format!(
            "({}, {}, {}) r {}",
            e.mem.f32(bound),
            e.mem.f32(bound + 4),
            e.mem.f32(bound + 8),
            e.mem.f32(bound + 0xc)
        )
    } else {
        format!("{bound:08x}")
    };
    light_check::say(&format!(
        "race menu open: face-light model {model:08x} node {light_node:08x}, scene node 3 \
         {graph:08x}: {found} lights found, radius {radius}; player {player:08x} bound {bound_text}"
    ));
    if graph == 0 {
        return;
    }
    for (list, label) in [(0xb4u32, "light list"), (0xe4, "queued to add")] {
        let mut item = e.mem.u32(graph + list);
        while item != 0 && e.mem.is_mapped(item) {
            let light = e.mem.u32(item + 8);
            item = e.mem.u32(item);
            if light != 0 {
                light_check::say(&format!(
                    "  node 3 {label}: {}",
                    light_check::light_text(e, light)
                ));
            }
        }
    }
}

// Translated from 007fdf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `FORenderedMenu::spAccumulator` (the `NiPointer` at +0xD4).
pub fn fn_007fdf10(e: &mut Engine, this: Ptr<FORenderedMenuRaceSex>, accumulator: u32) {
    assign_pointer(e, this.addr() + 0xd4, accumulator);
}

// Translated from 007fdf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Controls::GetInstance` (Xbox PDB): the global pointer at 0x011f35cc.
pub fn controls_get_instance(e: &mut Engine) -> u32 {
    e.global(0x011f_35cc)
}

// Translated from 007fdf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tears down what `OnMenuOpen` built (the counterpart called when the menu
/// closes): when the menu was shown (`00974d90`), detaches the cloned scene,
/// puts the player back under its original parent with the saved
/// translation, rotation, `Bip01` position and culled flag, and releases
/// the pointers.
pub fn fn_007fdf40(e: &mut Engine, this: Ptr<FORenderedMenuRaceSex>) {
    let me = this.addr();
    // The exe's frame (`SUB ESP,0x34` under the exception record, then the record link): the
    // zero point at `EBP-0x18` (+0x2c), the bone name at `EBP-0x1c` (+0x28).
    e.with_stack(0x44, |e, frame| {
        e.mem.set_u8(0x011a_d884, 1);
        if !e.call(0x0097_4d90, &args![me]).bool() {
            return;
        }
        if e.call(0x0068_38b0, &args![me]).u32() != 0 {
            let object = e.call(0x0068_38b0, &args![me]).u32();
            e.call(SET_CULLED, &args![object, 1u32]);
            let point = zero_point(e, frame.addr() + 0x2c);
            let object = e.call(0x0068_38b0, &args![me]).u32();
            e.call(SET_TRANSLATE_POINT, &args![object, point]);
        }
        // Detach the lights, the player and the parent node.
        let parent = pointer_value(e, me + 0x100);
        let lights = pointer_value(e, me + 0x108);
        e.vcall(parent, 0xe8, &args![lights]);
        let parent = pointer_value(e, me + 0x100);
        let player = pointer_value(e, me + 0xfc);
        e.vcall(parent, 0xe8, &args![player]);
        let graph = e.call(SCENE_NODE_GETTER, &args![3u32]).u32();
        let parent = pointer_value(e, me + 0x100);
        e.vcall(graph, 0xe8, &args![parent]);
        assign_pointer(e, me + 0x100, 0);
        let graph = e.call(SCENE_NODE_GETTER, &args![1u32]).u32();
        let cell_node = e.call(0x0055_85e0, &args![me]).u32();
        e.vcall(graph, 0xe8, &args![cell_node]);
        // Put the player back with its saved placement.
        let original_parent = pointer_value(e, me + 0x10c);
        let player = pointer_value(e, me + 0xfc);
        e.vcall(original_parent, 0xdc, &args![player, 1u32]);
        let player = pointer_value(e, me + 0xfc);
        let fixed = frame.addr() + 0x28;
        e.call(FIXED_STRING_CONSTRUCT, &args![fixed, BIP01_NAME]);
        let bone = e.vcall(player, 0x9c, &args![fixed]).u32();
        e.call(SET_VECTOR, &args![bone, me + 0x140]);
        e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
        let player = pointer_value(e, me + 0xfc);
        e.call(SET_ROTATE, &args![player, me + 0x11c]);
        let player = pointer_value(e, me + 0xfc);
        e.call(SET_VECTOR, &args![player, me + 0x110]);
        let culled = e.mem.u8(me + 0x14c);
        let player = pointer_value(e, me + 0xfc);
        e.call(SET_CULLED, &args![player, culled as u32]);
        assign_pointer(e, me + 0xfc, 0);
        assign_pointer(e, me + 0x104, 0);
        assign_pointer(e, me + 0xf8, 0);
        let graph = e.call(SCENE_NODE_GETTER, &args![1u32]).u32();
        e.call(0x00b5_d180, &args![graph]);
        e.call(0x007f_b0b0, &args![me]);
        forendered_menu_race_sex_light_button(e, this, u32::MAX);
        assign_pointer(e, me + 0x108, 0);
    });
}

// Translated from 007fe1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenuRaceSex::Draw` (Xbox PDB): renders the cloned player scene
/// into the menu: builds a render target for the scene, clears it with the
/// background colour (fixed to `(0105559c, 01078a60, 01078a64, 1)` because
/// the flag byte the original tests was set to 1 on the previous line),
/// accumulates scene node 3 (which holds `spPlayerParent`) through
/// `spClonedPlayerCamera` with a culling process on the interface's
/// accumulator (`BSShaderUtil::AccumulateScene`, then `00b6c0d0`), hands the
/// result to `007fba00` and releases it again.
///
/// Only `argument` is used; the other two words of the 12-byte argument list
/// are never read.
pub fn forendered_menu_race_sex_draw(
    e: &mut Engine,
    this: Ptr<FORenderedMenuRaceSex>,
    argument: u32,
    _unused_1: u32,
    _unused_2: u32,
) {
    let me = this.addr();
    // The exe's frame (`SUB ESP,0x110` under the exception record, then the record link): the
    // render target at `EBP-0xe0`, the colours at `EBP-0x108`, `EBP-0xf8` and `EBP-0x118`, the
    // culling process at `EBP-0xdc`, the accumulator pointer at `EBP-0xe8`.
    e.with_stack(0x120, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        let root = e.call(0x0043_c4b0, &args![]).u32();
        let source = e.call(0x0043_c4b0, &args![]).u32();
        let object = e.call(0x004a_0ea0, &args![]).u32();
        let target = e
            .call(
                0x00b6_e110,
                &args![object, source, 0x30u32, 0u32, 0u32, 0u32],
            )
            .u32();
        let render_target = local(0x40);
        e.call(0x0063_3c90, &args![render_target, target]);
        // The background colour.
        let default_color = local(0x18);
        e.call(
            COLOR_A_CONSTRUCT,
            &args![default_color, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let background = local(0x28);
        e.call(
            COLOR_A_CONSTRUCT,
            &args![background, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let current = e.call(0x0050_3650, &args![me]).u32();
        copy_words(e, current, background, 4);
        e.mem.set_f32(background + 0xc, 1.0);
        // (The flag byte tested here was just set to 1, so this always runs.)
        let fixed = local(0x8);
        let (red, green, blue): (f32, f32, f32) = (
            e.global(0x0105_559c),
            e.global(0x0107_8a60),
            e.global(0x0107_8a64),
        );
        let color = e
            .call(COLOR_A_CONSTRUCT, &args![fixed, red, green, blue, 1.0f32])
            .u32();
        copy_words(e, color, background, 4);
        e.vcall(root, 0xb4, &args![default_color]);
        e.vcall(root, 0xac, &args![background]);
        let target = pointer_value(e, render_target);
        let texture = e.call(0x00b6_b260, &args![target]).u32();
        e.call(0x00b6_b8d0, &args![7u32, texture]);
        // The culling process (`004a0eb0`; destroyed by `004a0f60`) on the interface's
        // accumulator (`Interface` +0x90, `007fd640`).
        let culler = local(0x44);
        e.call(0x004a_0eb0, &args![culler, 0u32]);
        let interface = e.call(INTERFACE_GETTER, &args![]).u32();
        let accumulator = e.call(0x007f_d640, &args![interface]).u32();
        let accumulator_pointer = local(0x38);
        e.call(0x0063_3c90, &args![accumulator_pointer, accumulator]);
        let accumulator = pointer_value(e, accumulator_pointer);
        e.call(0x004a_0fd0, &args![culler, accumulator]);
        e.call(0x00c4_f270, &args![culler, 1u32]);
        // 007fe3d6-007fe3fa: the culler is pushed before `00450b80(3)`, whose result follows:
        // `BSShaderUtil::AccumulateScene(spClonedPlayerCamera, scene node 3, culler)`.
        let scene = e.call(SCENE_NODE_GETTER, &args![3u32]).u32();
        let camera = pointer_value(e, me + 0x104);
        e.call(0x00b6_bee0, &args![camera, scene, culler]);
        // 007fe402-007fe427: `00b6c0d0(camera, accumulator, 0)`, `cdecl`; it reads only the
        // first two words and the caller's `ADD ESP,0xc` drops all three.
        let accumulator = pointer_value(e, accumulator_pointer);
        let camera = pointer_value(e, me + 0x104);
        e.call(0x00b6_c0d0, &args![camera, accumulator, 0u32]);
        e.call(0x00c4_f2d0, &args![culler]);
        e.call(0x004a_0fd0, &args![culler, 0u32]);
        assign_pointer(e, accumulator_pointer, 0);
        e.call(0x00b6_b790, &args![]);
        let target = pointer_value(e, render_target);
        e.call(0x007f_ba00, &args![me, argument, 0u32, target]);
        let target = pointer_value(e, render_target);
        let object = e.call(0x004a_0ea0, &args![]).u32();
        e.call(0x00b6_da10, &args![object, target]);
        assign_pointer(e, render_target, 0);
        e.call(NI_POINTER_DESTRUCT, &args![accumulator_pointer]);
        e.call(0x004a_0f60, &args![culler]);
        e.call(NI_POINTER_DESTRUCT, &args![render_target]);
    });
}

// Translated from 007fe4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenuRaceSex::OnIdle` (Xbox PDB): without the menu being shown
/// (`00974d90`) it calls `007037e0` and returns 0; otherwise it refreshes the
/// player manager's state and returns the result of `007faf10`.
pub fn forendered_menu_race_sex_on_idle(e: &mut Engine, this: Ptr<FORenderedMenuRaceSex>) -> u8 {
    let me = this.addr();
    if !e.call(0x0097_4d90, &args![me]).bool() {
        e.call(0x0070_37e0, &args![]);
        return 0;
    }
    let manager: u32 = e.global(0x011d_ea3c);
    let object = e.call(0x0095_0a60, &args![manager, 0u32]).u32();
    e.call(0x008e_eaa0, &args![object, 0x17u32]);
    e.vcall(manager, 0x1e0, &args![]);
    let object = e.call(0x0095_0a60, &args![manager, 0u32]).u32();
    e.call(0x0088_85e0, &args![manager, object, 0.0f32]);
    e.call(0x007f_af10, &args![me]).u8()
}

// Translated from 007fe560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenuRaceSex::InitLight` (Xbox PDB): resets the menu light
/// (`6838b0`) to black / default colours and zero position after
/// `007fc400` accepted the scene; when it did not, reports it through
/// `005b5e40` and leaves.
pub fn forendered_menu_race_sex_init_light(e: &mut Engine, this: Ptr<FORenderedMenuRaceSex>) {
    let me = this.addr();
    // The exe's frame (`SUB ESP,0x44` under `EBP`): the colours at `EBP-0x28`, `EBP-0x34`,
    // `EBP-0x40` and `EBP-0x1c`, the zero point at `EBP-0x10`.
    e.with_stack(0x44, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        let graph = e.call(SCENE_NODE_GETTER, &args![1u32]).u32();
        if !e
            .call(0x007f_c400, &args![me, 0x0107_8210u32, graph])
            .bool()
        {
            e.call(0x005b_5e40, &args![0x0107_8a68u32]);
            return;
        }
        let light = e.call(0x0068_38b0, &args![me]).u32();
        if light == 0 {
            return;
        }
        let black = local(0x1c);
        let color = e
            .call(COLOR_CONSTRUCT, &args![black, 0.0f32, 0.0f32, 0.0f32])
            .u32();
        e.call(0x004b_c2e0, &args![light, color]);
        let black = local(0x10);
        let color = e
            .call(COLOR_CONSTRUCT, &args![black, 0.0f32, 0.0f32, 0.0f32])
            .u32();
        e.call(0x004b_c280, &args![light, color]);
        e.call(0x0050_c630, &args![light, 0.0f32]);
        e.call(0x0050_c660, &args![light, 0.0f32]);
        e.call(0x0050_c690, &args![light, 0.0f32]);
        e.call(0x0050_dd20, &args![light, 1.0f32]);
        let level: f32 = e.global(0x0101_7b78);
        let specular = local(0x4);
        let color = e
            .call(COLOR_CONSTRUCT, &args![specular, level, 0.0f32, 0.0f32])
            .u32();
        e.call(0x0050_dd50, &args![light, color]);
        let white = local(0x28);
        let color = e
            .call(COLOR_CONSTRUCT, &args![white, 1.0f32, 1.0f32, 1.0f32])
            .u32();
        e.call(0x006b_0570, &args![me, color]);
        e.call(0x0096_5580, &args![me, 1.0f32]);
        e.call(0x007f_99b0, &args![me, 1.0f32]);
        let point = zero_point(e, local(0x34));
        e.call(SET_TRANSLATE_POINT, &args![light, point]);
    });
}

// Translated from 007fe6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles a mouse click on the menu. With `flag` set and all four buttons
/// and glows present, it casts a ray from the cursor against each button
/// (`a71080` fills the ray, `e98e20` / `968670` test it) and, for the first
/// button hit, logs the click with `ad7550` / `ad8830`, tells the tile menu
/// (`007fe910`, through `a03c90(9611e0(this))`) which button it was and
/// returns 1. In every other case it returns the result of `007fb790`.
pub fn fn_007fe6c0(
    e: &mut Engine,
    this: Ptr<FORenderedMenuRaceSex>,
    flag: u8,
    second: u32,
    third: u32,
) -> u8 {
    let me = this.addr();
    let interface = e.call(INTERFACE_GETTER, &args![]).u32();
    if flag != 0 {
        let mut all_present = true;
        let mut i = 0u32;
        while all_present && i < 4 {
            if pointer_value(e, me + 0xd8 + 4 * i) == 0 || pointer_value(e, me + 0xe8 + 4 * i) == 0
            {
                all_present = false;
            }
            i += 1;
        }
        // The exe's frame (`SUB ESP,0x7c` under the exception record, then the record link): the
        // origin at `EBP-0x5c` (+0x30), the direction at `EBP-0x68` (+0x24), the pick at
        // `EBP-0x50` (+0x3c) and the sound handle at `EBP-0x78` (+0x14).
        if all_present
            && e.with_stack(0x8c, |e, frame| {
                let origin = frame.addr() + 0x30;
                e.call(MATRIX_CONSTRUCT, &args![origin]);
                let direction = frame.addr() + 0x24;
                e.call(MATRIX_CONSTRUCT, &args![direction]);
                let ray = frame.addr() + 0x3c;
                e.call(0x00e9_8f20, &args![ray, 0u32, 8u32]);
                e.call(0x0045_8b30, &args![ray, 1u32]);
                e.call(0x006e_cd40, &args![ray, 0u32]);
                e.call(0x0079_2780, &args![ray, 0u32]);
                let mut found = -1i32;
                let mut j = 0u32;
                while found < 0 && j < 4 {
                    let button = pointer_value(e, me + 0xd8 + 4 * j);
                    e.call(0x0070_5fc0, &args![ray, button]);
                    let x = e.call(0x0079_2760, &args![interface]).f32();
                    let x = e.call(FTOL, &args![x as f64]).i32();
                    let y = e.call(0x009a_9350, &args![interface]).f32();
                    let y = e.call(FTOL, &args![y as f64]).i32();
                    let target = e.call(0x008b_6200, &args![me]).u32();
                    e.call(0x00a7_1080, &args![target, y, x, origin, direction, 0u32]);
                    if e.call(0x00e9_8e20, &args![ray, origin, direction, 0u32])
                        .bool()
                    {
                        let hit = e.call(0x0050_0940, &args![ray]).u32();
                        if e.call(0x0096_8670, &args![hit, 0u32]).u32() != 0 {
                            found = j as i32;
                        }
                    }
                    j += 1;
                }
                if found >= 0 {
                    let scope = frame.addr() + 0x14;
                    let holder = e.call(0x0045_3a70, &args![]).u32();
                    let guard = e
                        .call(0x00ad_7550, &args![holder, scope, 0x0106_f060u32, 0x121u32])
                        .u32();
                    e.call(0x00ad_8830, &args![guard, 0u32]);
                    e.call(0x0048_3710, &args![scope]);
                    let owner = e.call(0x0096_11e0, &args![me]).u32();
                    let menu = e.call(0x00a0_3c90, &args![owner]).u32();
                    fn_007fe910(e, Ptr::new(menu), found as u32);
                    e.call(0x00e9_8fa0, &args![ray]);
                    return true;
                }
                e.call(0x00e9_8fa0, &args![ray]);
                false
            })
        {
            return 1;
        }
    }
    e.call(0x007f_b790, &args![me, flag as u32, second, third])
        .u8()
}

// Translated from 007fe910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `007af180(this, argument)`.
pub fn fn_007fe910(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x007a_f180, &args![this, argument]);
}

// Translated from 007fe930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `0063d040(this)`.
pub fn fn_007fe930(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0063_d040, &args![this]).u32()
}

// Translated from 007fe950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenuRaceSex::LightButton` (Xbox PDB): of the four glow nodes
/// (`spBtnGlow`, +0xE8), culls every present one except button `index`.
pub fn forendered_menu_race_sex_light_button(
    e: &mut Engine,
    this: Ptr<FORenderedMenuRaceSex>,
    index: u32,
) {
    let me = this.addr();
    for i in 0..4u32 {
        let glow = pointer_value(e, me + 0xe8 + 4 * i);
        if glow != 0 {
            let culled = (i != index) as u32;
            let glow = pointer_value(e, me + 0xe8 + 4 * i);
            e.call(SET_CULLED, &args![glow, culled]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x007fc970,
            forendered_menu_race_sex_initialize(Ptr<FORenderedMenuRaceSex>)
        ),
        entry!(0x007fd620, fn_007fd620(Ptr<FORenderedMenuRaceSex>, u32)),
        entry!(0x007fd640, fn_007fd640(Ptr) -> u32),
        entry!(0x007fd660, fn_007fd660(Ptr<FORenderedMenuRaceSex>, f32)),
        entry!(
            0x007fd6c0,
            forendered_menu_race_sex_on_menu_open(Ptr<FORenderedMenuRaceSex>)
        ),
        entry!(0x007fdf10, fn_007fdf10(Ptr<FORenderedMenuRaceSex>, u32)),
        entry!(0x007fdf30, controls_get_instance() -> u32),
        entry!(0x007fdf40, fn_007fdf40(Ptr<FORenderedMenuRaceSex>)),
        entry!(
            0x007fe1f0,
            forendered_menu_race_sex_draw(Ptr<FORenderedMenuRaceSex>, u32, u32, u32)
        ),
        entry!(
            0x007fe4e0,
            forendered_menu_race_sex_on_idle(Ptr<FORenderedMenuRaceSex>) -> u8
        ),
        entry!(
            0x007fe560,
            forendered_menu_race_sex_init_light(Ptr<FORenderedMenuRaceSex>)
        ),
        entry!(
            0x007fe6c0,
            fn_007fe6c0(Ptr<FORenderedMenuRaceSex>, u8, u32, u32) -> u8
        ),
        entry!(0x007fe910, fn_007fe910(Ptr, u32)),
        entry!(0x007fe930, fn_007fe930(Ptr) -> u32),
        entry!(
            0x007fe950,
            forendered_menu_race_sex_light_button(Ptr<FORenderedMenuRaceSex>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Registers do-nothing doubles (they still show up in `call_log`).
    fn stubs(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// The addresses of the calls in a log, in order.
    fn addresses(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().map(|(a, _)| *a).collect()
    }

    /// The words of the first call to `address`.
    fn first_call(log: &[(u32, Vec<u32>)], address: u32) -> Vec<u32> {
        log.iter()
            .find(|(a, _)| *a == address)
            .unwrap_or_else(|| panic!("no call to {address:08x}"))
            .1
            .clone()
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn put_float(e: &mut Engine, address: u32, value: f32) {
        e.mem.set_f32(address, value);
    }

    /// An engine with the pages of the globals the menu reads mapped, the
    /// pointer, setting and string helpers replaced by doubles, and the
    /// float constants and settings given distinct values.
    fn race_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_3000u32,
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0101_e000,
            0x0102_1000,
            0x0102_3000,
            0x0102_b000,
            0x0102_c000,
            0x0104_f000,
            0x0105_5000,
            0x0107_8000,
            0x011a_d000,
            0x011c_3000,
            0x011d_b000,
            0x011d_e000,
            0x011f_3000,
            0x011f_6000,
            0x011f_9000,
            0x0120_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(POINTER_VALUE, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(SETTING_FLOAT_POINTER, |_, a| ret(a[0]));
        e.register(SETTING_INT_POINTER, |_, a| ret(a[0]));
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        // Constructors store their floats and return their `this`.
        for address in [POINT_CONSTRUCT, COLOR_CONSTRUCT] {
            e.register(address, |e, a| {
                for i in 1..4 {
                    e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
                }
                ret(a[0])
            });
        }
        e.register(COLOR_A_CONSTRUCT, |e, a| {
            for i in 1..5 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            ret(a[0])
        });
        e.register(MATRIX_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(INTERFACE_GETTER, |e, _| ret(e.global::<u32>(0x011d_e100)));
        e
    }

    fn new_menu(e: &mut Engine) -> Ptr<FORenderedMenuRaceSex> {
        e.new_object()
    }

    #[test]
    fn small_setters_assign_the_pointer_fields() {
        let mut e = race_engine();
        let menu = new_menu(&mut e);
        e.call(0x007f_d620, &args![menu, 0x1111u32]);
        assert_eq!(e.mem.u32(menu.addr() + 0x24), 0x1111);
        e.call(0x007f_df10, &args![menu, 0x2222u32]);
        assert_eq!(e.mem.u32(menu.addr() + 0xd4), 0x2222);
        // The +0x90 pointer of another object.
        let other = e.mem.alloc(0x100);
        e.mem.set_u32(other + 0x90, 0x3333);
        assert_eq!(e.call(0x007f_d640, &args![other]).u32(), 0x3333);
        e.set_global(0x011f_35ccu32, 0x4444u32);
        assert_eq!(e.call(0x007f_df30, &args![]).u32(), 0x4444);
    }

    #[test]
    fn wrappers_pass_their_arguments_on() {
        let mut e = race_engine();
        e.register(0x007a_f180, |_, _| Ret::default());
        e.register(0x0063_d040, |_, a| ret(a[0] + 5));
        e.call_log = Some(vec![]);
        e.call(0x007f_e910, &args![0x10u32, 0x20u32]);
        assert_eq!(e.call(0x007f_e930, &args![0x100u32]).u32(), 0x105);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x007a_f180), [[0x10, 0x20]]);
    }

    #[test]
    fn light_button_culls_every_glow_but_the_chosen_one() {
        let mut e = race_engine();
        stubs(&mut e, &[SET_CULLED]);
        let menu = new_menu(&mut e);
        for (i, glow) in [0x100u32, 0x200, 0, 0x400].iter().enumerate() {
            e.mem.set_u32(menu.addr() + 0xe8 + 4 * i as u32, *glow);
        }
        e.call_log = Some(vec![]);
        e.call(0x007f_e950, &args![menu, 1u32]);
        let log = e.call_log.take().unwrap();
        // The missing glow (index 2) is skipped; index 1 stays visible.
        assert_eq!(
            calls_to(&log, SET_CULLED),
            [vec![0x100, 1], vec![0x200, 0], vec![0x400, 1]]
        );
        // An index beyond the glows culls all of them.
        e.call_log = Some(vec![]);
        e.call(0x007f_e950, &args![menu, u32::MAX]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_CULLED).len(), 3);
        assert!(calls_to(&log, SET_CULLED).iter().all(|w| w[1] == 1));
    }

    #[test]
    fn rescale_uses_the_radius_of_the_player_bound() {
        let mut e = race_engine();
        e.set_global(SCALE_DOUBLE, 0.5f64);
        let menu = new_menu(&mut e);
        e.mem.set_u32(menu.addr() + 0xfc, 0xa000);
        e.mem.set_u32(menu.addr() + 0x100, 0xb000);
        e.register(0x0043_d450, |_, a| ret(a[0] + 1));
        e.register(0x0084_d030, |_, _| 8.0f32.into_ret());
        e.register(SCENE_NODE_GETTER, |_, a| ret(0xc000 + a[0]));
        stubs(&mut e, &[0x00b5_ca70]);
        e.call_log = Some(vec![]);
        e.call(0x007f_d660, &args![menu, 3.0f32]);
        let log = e.call_log.take().unwrap();
        // 8.0 * 0.5 * 3.0 = 12.0, with the player parent, on scene node 3.
        assert_eq!(
            first_call(&log, 0x00b5_ca70),
            [0xc003, 0xb000, 12.0f32.to_bits()]
        );
        assert_eq!(first_call(&log, SCENE_NODE_GETTER), [3]);
    }

    #[test]
    fn on_idle_without_the_menu_shown_only_reports() {
        let mut e = race_engine();
        e.register(0x0097_4d90, |_, _| ret(0));
        stubs(&mut e, &[0x0070_37e0]);
        let menu = new_menu(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_e4e0, &args![menu]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(addresses(&log), [0x007f_e4e0, 0x0097_4d90, 0x0070_37e0]);
    }

    #[test]
    fn on_idle_refreshes_the_manager_and_returns_the_update_result() {
        let mut e = race_engine();
        e.register(0x0097_4d90, |_, _| ret(1));
        e.set_global(0x011d_ea3cu32, 0x0900_0000u32);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(vtable + 0x1e0, 0x0120_0010);
        let manager = e.mem.alloc(0x10);
        e.mem.set_u32(manager, vtable);
        e.set_global(0x011d_ea3cu32, manager);
        e.register(0x0120_0010, |_, _| Ret::default());
        e.register(0x0095_0a60, |_, a| ret(0x5000 + a[1]));
        stubs(&mut e, &[0x008e_eaa0, 0x0088_85e0]);
        e.register(0x007f_af10, |_, _| ret(1));
        let menu = new_menu(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_e4e0, &args![menu]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(first_call(&log, 0x008e_eaa0), [0x5000, 0x17]);
        assert_eq!(
            first_call(&log, 0x0088_85e0),
            [manager, 0x5000, 0.0f32.to_bits()]
        );
        assert_eq!(*addresses(&log).last().unwrap(), 0x007f_af10);
    }

    #[test]
    fn init_light_reports_a_scene_that_is_not_accepted() {
        let mut e = race_engine();
        e.register(SCENE_NODE_GETTER, |_, _| ret(0x77));
        e.register(0x007f_c400, |_, _| ret(0));
        stubs(&mut e, &[0x005b_5e40]);
        let menu = new_menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007f_e560, &args![menu]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            first_call(&log, 0x007f_c400),
            [menu.addr(), 0x0107_8210, 0x77]
        );
        assert_eq!(first_call(&log, 0x005b_5e40), [0x0107_8a68]);
        assert!(!addresses(&log).contains(&0x0068_38b0));
    }

    #[test]
    fn init_light_resets_the_light_when_the_scene_is_accepted() {
        let mut e = race_engine();
        e.register(SCENE_NODE_GETTER, |_, _| ret(0x77));
        e.register(0x007f_c400, |_, _| ret(1));
        put_float(&mut e, 0x0101_7b78, 0.75);
        let light = e.mem.alloc(0x40);
        e.register(0x0068_38b0, |e, _| ret(e.global::<u32>(0x011d_e104)));
        e.set_global(0x011d_e104u32, light);
        stubs(
            &mut e,
            &[
                0x004b_c2e0,
                0x004b_c280,
                0x0050_c630,
                0x0050_c660,
                0x0050_c690,
                0x0050_dd20,
                0x0050_dd50,
                0x006b_0570,
                0x0096_5580,
                0x007f_99b0,
                SET_TRANSLATE_POINT,
            ],
        );
        let menu = new_menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007f_e560, &args![menu]);
        let log = e.call_log.take().unwrap();
        // The third colour is (0.75, 0, 0), set on the light as specular.
        let colors = calls_to(&log, COLOR_CONSTRUCT);
        assert_eq!(colors.len(), 4);
        assert_eq!(colors[2][1..], [0.75f32.to_bits(), 0, 0]);
        assert_eq!(colors[3][1..], [1.0f32.to_bits(); 3]);
        assert_eq!(first_call(&log, 0x0050_dd20), [light, 1.0f32.to_bits()]);
        assert_eq!(
            first_call(&log, 0x0096_5580),
            [menu.addr(), 1.0f32.to_bits()]
        );
        assert_eq!(
            first_call(&log, 0x007f_99b0),
            [menu.addr(), 1.0f32.to_bits()]
        );
        assert_eq!(first_call(&log, 0x006b_0570)[0], menu.addr());
        // The light ends at the zero point.
        assert_eq!(calls_to(&log, SET_TRANSLATE_POINT)[0][0], light);
    }

    #[test]
    fn init_light_stops_without_a_light() {
        let mut e = race_engine();
        e.register(SCENE_NODE_GETTER, |_, _| ret(0x77));
        e.register(0x007f_c400, |_, _| ret(1));
        e.register(0x0068_38b0, |_, _| ret(0));
        let menu = new_menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007f_e560, &args![menu]);
        assert!(!addresses(&e.call_log.take().unwrap()).contains(&COLOR_CONSTRUCT));
    }

    #[test]
    fn on_click_without_the_flag_forwards_to_the_default_handler() {
        let mut e = race_engine();
        e.register(0x007f_b790, |_, _| ret(0));
        e.register(INTERFACE_GETTER, |_, _| ret(0x1234));
        let menu = new_menu(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_e6c0, &args![menu, 0u8, 5u32, 6u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(first_call(&log, 0x007f_b790), [menu.addr(), 0, 5, 6]);
    }

    #[test]
    fn on_click_needs_all_buttons_and_glows() {
        let mut e = race_engine();
        e.register(0x007f_b790, |_, _| ret(1));
        e.register(INTERFACE_GETTER, |_, _| ret(0x1234));
        let menu = new_menu(&mut e);
        for i in 0..4u32 {
            e.mem.set_u32(menu.addr() + 0xd8 + 4 * i, 0x100 + i);
            e.mem.set_u32(menu.addr() + 0xe8 + 4 * i, 0x200 + i);
        }
        // One glow missing: no ray cast, the default handler answers.
        e.mem.set_u32(menu.addr() + 0xf4, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_e6c0, &args![menu, 1u8, 7u32, 8u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&0x00e9_8f20));
        assert_eq!(first_call(&log, 0x007f_b790), [menu.addr(), 1, 7, 8]);
    }

    /// The click cast: the ray hits button `hit` (or none when it is 9).
    fn click_engine(hit: u32) -> (Engine, Ptr<FORenderedMenuRaceSex>) {
        let mut e = race_engine();
        e.register(INTERFACE_GETTER, |_, _| ret(0x1234));
        e.register(0x007f_b790, |_, _| ret(0));
        let menu = new_menu(&mut e);
        for i in 0..4u32 {
            e.mem.set_u32(menu.addr() + 0xd8 + 4 * i, 0x100 + i);
            e.mem.set_u32(menu.addr() + 0xe8 + 4 * i, 0x200 + i);
        }
        e.set_global(0x011d_e108u32, hit);
        stubs(
            &mut e,
            &[
                0x00e9_8f20,
                0x0045_8b30,
                0x006e_cd40,
                0x0079_2780,
                0x00a7_1080,
                0x00e9_8fa0,
                0x0048_3710,
                0x00ad_8830,
                0x007f_a950,
            ],
        );
        // The ray is tested against the buttons in order: the button the
        // ray was last given decides the answer.
        e.register(0x0070_5fc0, |e, a| {
            e.set_global(0x011d_e10cu32, a[1]);
            Ret::default()
        });
        e.register(0x0079_2760, |_, _| 10.7f32.into_ret());
        e.register(0x009a_9350, |_, _| 20.2f32.into_ret());
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(0x008b_6200, |_, _| ret(0x9000));
        e.register(0x00e9_8e20, |_, _| ret(1));
        e.register(0x0050_0940, |e, _| ret(e.global::<u32>(0x011d_e10c)));
        e.register(0x0096_8670, |e, a| {
            ret((a[0] == 0x100 + e.global::<u32>(0x011d_e108)) as u32)
        });
        e.register(0x0045_3a70, |_, _| ret(0x6000));
        e.register(0x00ad_7550, |_, _| ret(0x6100));
        e.register(0x0096_11e0, |_, _| ret(0x6200));
        e.register(0x00a0_3c90, |_, a| ret(a[0] + 1));
        stubs(&mut e, &[0x007a_f180]);
        (e, menu)
    }

    #[test]
    fn on_click_reports_the_button_the_ray_hits() {
        let (mut e, menu) = click_engine(2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_e6c0, &args![menu, 1u8, 7u32, 8u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        // The cast uses the interface's cursor position (10.7, 20.2) cut to
        // integers, the menu's `8b6200` object and the two direction points.
        let cast = first_call(&log, 0x00a7_1080);
        assert_eq!(cast[0], 0x9000);
        assert_eq!(cast[1..3], [20, 10]);
        assert_eq!(cast[5], 0);
        // The ray stopped at the third button, which is reported to the tile
        // menu, and the default handler was not asked.
        assert_eq!(calls_to(&log, 0x0070_5fc0).len(), 3);
        assert_eq!(calls_to(&log, 0x007a_f180), [vec![0x6201, 2]]);
        assert!(!addresses(&log).contains(&0x007f_b790));
        let logged = first_call(&log, 0x00ad_7550);
        assert_eq!(
            (logged[0], logged[2], logged[3]),
            (0x6000, 0x0106_f060, 0x121)
        );
        assert_eq!(first_call(&log, 0x00ad_8830), [0x6100, 0]);
    }

    #[test]
    fn on_click_without_a_hit_falls_back_to_the_default_handler() {
        let (mut e, menu) = click_engine(9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_e6c0, &args![menu, 1u8, 7u32, 8u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0070_5fc0).len(), 4);
        assert_eq!(first_call(&log, 0x007f_b790), [menu.addr(), 1, 7, 8]);
        assert_eq!(first_call(&log, 0x00e9_8fa0).len(), 1);
    }

    /// The doubles shared by the tests of `Initialize` / `OnMenuOpen`:
    /// the camera frustum maths and the scene graph holder.
    fn scene_engine() -> Engine {
        let mut e = race_engine();
        for (address, value) in [
            (0x0101_6264u32, 0.25f32),
            (0x0105_59cc, 0.5),
            (0x0101_e33c, 30.0),
            (0x0104_f7c4, 0.125),
            (0x0101_3974, 0.375),
            (0x0102_b3c8, 3.0),
            (0x0102_caf8, 4.0),
            (0x0107_8a5c, 5.0),
            (0x011d_b4d0, 3.5),
            (0x011d_b544, 2.0),
            (0x011d_b570, 55.0),
            (0x011d_b5cc, 9.0),
            (0x011d_b594, 8.0),
            (0x011d_b5ec, 3.0),
            (0x0120_3144, 0.5),
            (0x0120_3150, 2.0),
        ] {
            put_float(&mut e, address, value);
        }
        e.set_global(0x011d_b4f8u32, 100i32);
        e.set_global(FOV_DOUBLE, 0.5f64);
        e.set_global(SCALE_DOUBLE, 2.0f64);
        e.set_global(0x0101_de30u32, 0.5f64);
        e.set_global(0x011c_3b3cu32, 0x0c00_0000u32);
        e.register(0x0082_5c00, |_, _| ret(1000));
        e.register(0x00ec_adb8, |_, _| ret(12345));
        e.register(FLOAT_TRIG, |_, a| (f32::from_bits(a[0]) + 10.0).into_ret());
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(0x004d_c200, |_, _| ret(1600));
        e.register(0x004d_c1f0, |_, _| ret(800));
        // The renderer: its virtual +0x100 gives 1.5.
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(vtable + 0x100, 0x0120_0020);
        let renderer = e.mem.alloc(0x10);
        e.mem.set_u32(renderer, vtable);
        e.set_global(0x011d_e110u32, renderer);
        e.register(0x0045_c670, |e, _| ret(e.global::<u32>(0x011d_e110)));
        e.register(0x0120_0020, |_, _| 1.5f32.into_ret());
        e.register(SCENE_NODE_GETTER, |_, a| ret(0x0d00_0000 + a[0]));
        e.register(0x0043_d450, |_, a| ret(a[0] + 1));
        e.register(0x0084_d030, |_, _| 4.0f32.into_ret());
        e.register(0x0097_4d90, |_, _| ret(1));
        e
    }

    /// A scene object whose virtual +0x9C answers a name lookup with
    /// `0x7000_0000 + name pointer` (or the object stored in the global at
    /// 0x011d_e130 for the geometry's own name), whose +0xDC / +0xE8 slots
    /// are stubs and whose +0x18 gives 0x5555 and +0xC gives 0.
    fn node_with_lookup(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x500);
        for (slot, double) in [
            (0x9cu32, 0x0120_0030u32),
            (0xdc, 0x0120_0040),
            (0xe8, 0x0120_0040),
            (0x18, 0x0120_0050),
            (0xc, 0x0120_0060),
            (0xb4, 0x0120_0040),
            (0xac, 0x0120_0040),
            (0x458, 0x0120_0040),
            (0x2f8, 0x0120_0040),
        ] {
            e.mem.set_u32(vtable + slot, double);
        }
        e.register(0x0120_0030, |e, a| {
            let name = e.mem.u32(a[1]);
            if name == 0x0107_8a54 {
                ret(e.global::<u32>(0x011d_e130))
            } else {
                ret(0x7000_0000 + name)
            }
        });
        e.register(0x0120_0040, |_, _| Ret::default());
        e.register(0x0120_0050, |_, _| ret(0x5555));
        e.register(0x0120_0060, |_, _| ret(0));
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node, vtable);
        node
    }

    /// Doubles for every function `Initialize` calls that is not already
    /// replaced, as recording no-ops (or the simplest answer).
    fn initialize_stubs(e: &mut Engine) {
        stubs(
            e,
            &[
                0x0071_9060,
                0x007f_7fb0,
                0x0044_0490,
                0x004d_c540,
                0x007f_ae70,
                0x0081_0530,
                0x008d_7f00,
                0x0043_f850,
                0x004a_0c90,
                SET_ROTATE,
                SET_TRANSLATE,
                SET_TRANSLATE_POINT,
                0x0071_2e60,
                0x0071_33b0,
                0x0073_3fd0,
                0x00a7_1b70,
                0x0050_7700,
                0x00a6_faf0,
                0x0070_9d50,
                0x007f_d620,
                0x007f_9010,
                0x00a5_b230,
                0x0043_9410,
                0x004b_c2e0,
                0x0050_dd50,
                0x0050_dd20,
                0x00b5_c940,
                0x007f_c150,
                0x007f_c400,
                0x0068_38b0,
                0x004b_c280,
                0x0050_c630,
                0x0050_c660,
                0x0050_c690,
                0x006b_0570,
                0x0096_5580,
                0x007f_99b0,
                0x004b_7660,
                SET_VECTOR,
                0x005b_5e40,
            ],
        );
        e.register(0x0071_8b60, |_, _| ret(0x4000));
        e.register(0x0043_f8d0, |_, a| ret(a[1]));
        e.register(0x009b_6600, |_, _| ret(1));
        e.register(0x0070_7b50, |_, _| ret(0));
        e.register(0x00a7_12f0, |_, a| ret(a[0]));
        e.register(0x0043_91c0, |_, a| ret(a[0]));
        e.register(0x0040_df90, |_, _| ret(0x1234));
        e.register(0x007f_a950, |_, a| ret(a[0] + 0x10));
        e.register(0x0096_11e0, |_, _| ret(0x6200));
        e.register(0x00a0_3c90, |_, _| ret(0x6300));
        e.register(0x007f_d640, |_, _| ret(0x6400));
        e.register(0x0043_b4a0, |_, a| ret(a[0]));
        e.register(0x00a7_d670, |_, a| ret(a[0]));
    }

    #[test]
    fn initialize_builds_the_scene_with_a_new_light() {
        let mut e = scene_engine();
        initialize_stubs(&mut e);
        let node = node_with_lookup(&mut e);
        let body = node_with_lookup(&mut e);
        e.set_global(0x011d_e130u32, body);
        e.set_global(0x011d_e114u32, node);
        e.register(0x0044_7080, |e, _| ret(e.global::<u32>(0x011d_e114)));
        // No entry for the scene yet: a light is made.
        e.register(0x00b5_ca70, |_, _| ret(0));
        let frustum = Rc::new(RefCell::new(Vec::<f32>::new()));
        let seen = frustum.clone();
        e.register_double(0x00a6_faf0, move |e, a| {
            *seen.borrow_mut() = (0..7).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            Ret::default()
        });
        let menu = new_menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007f_c970, &args![menu]);
        let log = e.call_log.take().unwrap();
        // The fake distortion: (12345 % 100) + 1000 ticks, duration setting.
        assert_eq!(
            e.get(menu, FORenderedMenuRaceSex::fNextFakeDistortTime),
            1045.0
        );
        assert_eq!(
            e.get(menu, FORenderedMenuRaceSex::fFakeDistortDuration),
            3.5
        );
        assert!(!e.get(menu, FORenderedMenuRaceSex::bFakeDistortActive));
        assert_eq!(e.get(menu, FORenderedMenuRaceSex::fDistortStart), 0.0);
        // The geometry was created and stored, the buttons found by name.
        assert_eq!(e.mem.u32(menu.addr() + 0xf8), node);
        let names = [
            0x0107_8a48u32,
            0x0107_8a3c,
            0x0107_8a30,
            0x0107_8a24,
            0x0107_8a18,
            0x0107_8a0c,
            0x0107_8a00,
            0x0107_89f4,
        ];
        for (i, name) in names.iter().enumerate() {
            assert_eq!(
                e.mem.u32(menu.addr() + 0xd8 + 4 * i as u32),
                0x7000_0000 + name
            );
        }
        // The body found by name reports its node (0x5555) to 007f9010.
        assert_eq!(first_call(&log, 0x007f_9010), [menu.addr(), 0x5555]);
        // The frustum: tan(fov) = 3.0 * (2.0 * 0.5) + 10 = 13, scale 0.25,
        // aspect 1600 / 800; near and far are the setting at 01203144 (0.5) and
        // the renderer getter's value (1.5), which the exe keeps in the frustum's
        // own stack words (007fd194 / 007fd1a6).
        assert_eq!(*frustum.borrow(), [-3.25, 3.25, 6.5, -6.5, 0.5, 1.5, 0.0]);
        // The camera's ratio is 1.5 / 0.5.
        assert_eq!(calls_to(&log, 0x0050_7700)[0][1], 3.0f32.to_bits());
        // A light was attached and InitLight asked 007fc400; the last call
        // is 007fc150.
        assert_eq!(calls_to(&log, 0x007f_c400).len(), 1);
        assert_eq!(*addresses(&log).last().unwrap(), 0x007f_c150);
        // The geometry node got the depth 30 * 0.5 (the wide flag is 0 and
        // 00707b50 says no) and the z setting.
        let placed = calls_to(&log, SET_TRANSLATE);
        assert_eq!(
            placed[0][..],
            [node, 15.0f32.to_bits(), 0, 9.0f32.to_bits()]
        );
    }

    #[test]
    fn initialize_keeps_the_existing_geometry_and_skips_the_light_when_known() {
        let mut e = scene_engine();
        initialize_stubs(&mut e);
        let node = node_with_lookup(&mut e);
        let body = node_with_lookup(&mut e);
        e.set_global(0x011d_e130u32, body);
        e.register(0x0044_7080, |_, _| ret(0));
        // The scene graph already has an entry; the other context takes
        // the depth from the setting and the second translation form.
        e.register(0x00b5_ca70, |_, _| ret(1));
        e.register(0x009b_6600, |_, _| ret(2));
        e.register(0x0070_7b50, |_, _| ret(1));
        let menu = new_menu(&mut e);
        e.mem.set_u32(menu.addr() + 0xf8, node);
        e.call_log = Some(vec![]);
        e.call(0x007f_c970, &args![menu]);
        let log = e.call_log.take().unwrap();
        assert!(!addresses(&log).contains(&0x0044_7080));
        assert!(!addresses(&log).contains(&0x007f_c400));
        // Depth 55 (setting), no scaling, (x, y, z) = (55, 8, 9).
        let placed = calls_to(&log, SET_TRANSLATE);
        assert_eq!(
            placed[0][..],
            [node, 55.0f32.to_bits(), 8.0f32.to_bits(), 9.0f32.to_bits()]
        );
        assert_eq!(*addresses(&log).last().unwrap(), 0x007f_c150);
    }

    #[test]
    fn initialize_scales_the_depth_when_neither_flag_is_set() {
        let mut e = scene_engine();
        initialize_stubs(&mut e);
        let node = node_with_lookup(&mut e);
        let body = node_with_lookup(&mut e);
        e.set_global(0x011d_e130u32, body);
        e.register(0x0044_7080, |_, _| ret(0));
        e.register(0x00b5_ca70, |_, _| ret(1));
        e.register(0x009b_6600, |_, _| ret(2));
        // 00707b50 says no and the wide flag byte is 0: 55 * 0.5.
        let menu = new_menu(&mut e);
        e.mem.set_u32(menu.addr() + 0xf8, node);
        e.call_log = Some(vec![]);
        e.call(0x007f_c970, &args![menu]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_TRANSLATE)[0][1], 27.5f32.to_bits());
    }

    /// Doubles for `OnMenuOpen`: a player, a parent node, lights and a scene
    /// graph that are all `node_with_lookup` objects.
    fn menu_open_engine() -> (Engine, Ptr<FORenderedMenuRaceSex>, u32) {
        let mut e = scene_engine();
        stubs(
            &mut e,
            &[
                0x0087_1dc0,
                SET_TRANSLATE_POINT,
                SET_CULLED,
                0x007f_b040,
                0x00a2_53d0,
                0x0088_4f80,
                0x008b_3a80,
                0x0049_6080,
                0x008e_eaa0,
                0x0049_4740,
                SET_TRANSLATE,
                SET_ROTATE,
                0x0064_04b0,
                0x007f_df10,
                0x0071_2e60,
                0x0073_3fd0,
                0x00a7_01b0,
                0x00a7_1b70,
                0x0050_7700,
                0x00a6_faf0,
                0x007b_1970,
                0x007b_1b60,
                0x00b5_ca70,
            ],
        );
        let graph = node_with_lookup(&mut e);
        let player = node_with_lookup(&mut e);
        let parent = node_with_lookup(&mut e);
        let lights = node_with_lookup(&mut e);
        let manager = node_with_lookup(&mut e);
        for (slot, value) in [
            (0x011d_e124u32, graph),
            (0x011d_e118, player),
            (0x011d_e12c, parent),
            (0x011d_e134, lights),
            (0x011d_ea3c, manager),
        ] {
            e.set_global(slot, value);
        }
        // Where the player's translation and rotation live.
        let translation = e.mem.alloc(0x10);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(translation + 4 * i as u32, *v);
        }
        let rotation = e.mem.alloc(0x30);
        for i in 0..9u32 {
            e.mem.set_u32(rotation + 4 * i, 100 + i);
        }
        e.set_global(0x011d_e11cu32, translation);
        e.set_global(0x011d_e120u32, rotation);
        e.register(SCENE_NODE_GETTER, |e, _| ret(e.global::<u32>(0x011d_e124)));
        e.register(0x008d_8520, |e, _| ret(e.global::<u32>(0x011d_ea3c)));
        e.register(0x0055_85e0, |_, _| ret(0x8000));
        e.register(0x0096_11e0, |_, a| ret(a[0] + 0x40));
        e.register(0x00a0_3c90, |_, _| ret(0x8100));
        e.register(0x0068_38b0, |_, _| ret(0x8200));
        e.register(0x007f_a950, |_, a| ret(a[0] + 0x10));
        e.register(0x0095_0a60, |_, _| ret(0x8300));
        e.register(0x0095_0bb0, |e, _| ret(e.global::<u32>(0x011d_e118)));
        e.register(GET_TRANSLATE, |e, _| ret(e.global::<u32>(0x011d_e11c)));
        e.register(GET_ROTATE, |e, _| ret(e.global::<u32>(0x011d_e120)));
        e.register(0x0045_6610, |_, _| ret(1));
        e.register(0x00a5_ecb0, |e, _| ret(e.global::<u32>(0x011d_e12c)));
        e.register(0x00a7_12f0, |_, a| ret(a[0]));
        e.register(0x0044_7080, |e, _| ret(e.global::<u32>(0x011d_e134)));
        e.register(0x007d_cc00, |_, _| ret(2));
        let menu = new_menu(&mut e);
        (e, menu, player)
    }

    /// The p202 race-menu stop: the cloned scene's camera frustum takes its near and far from
    /// the stack words `OnMenuOpen` stores the ratio's two `float`s in (007fdd9b: the renderer
    /// getter's value at the frustum's +0x14; 007fddaa: the setting at 01203144 at +0x10). Left
    /// at 0 the culler's far plane went through the camera and culled every menu light.
    #[test]
    fn on_menu_open_gives_the_camera_frustum_the_ratio_near_and_far() {
        let (mut e, menu, _) = menu_open_engine();
        let frustum = Rc::new(RefCell::new(Vec::<f32>::new()));
        let seen = frustum.clone();
        e.register_double(0x00a6_faf0, move |e, a| {
            *seen.borrow_mut() = (0..6).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            Ret::default()
        });
        e.mem.set_u8(0x011a_d884, 1);
        e.call(0x007f_d6c0, &args![menu]);
        let frustum = frustum.borrow();
        assert_eq!(frustum.len(), 6);
        assert_eq!((frustum[4], frustum[5]), (0.5, 1.5));
    }

    #[test]
    fn on_menu_open_does_nothing_more_when_the_menu_is_not_shown() {
        let mut e = scene_engine();
        e.register(0x0097_4d90, |_, _| ret(0));
        stubs(&mut e, &[0x0087_1dc0]);
        e.set_global(0x011d_ea0cu32, 0x1000u32);
        let menu = new_menu(&mut e);
        e.mem.set_u8(0x011a_d884, 1);
        e.call_log = Some(vec![]);
        e.call(0x007f_d6c0, &args![menu]);
        let log = e.call_log.take().unwrap();
        assert_eq!(addresses(&log), [0x007f_d6c0, 0x0087_1dc0, 0x0097_4d90]);
        assert_eq!(first_call(&log, 0x0087_1dc0), [0x1000]);
        assert_eq!(e.mem.u8(0x011a_d884), 0);
    }

    #[test]
    fn on_menu_open_saves_the_player_and_builds_the_private_scene() {
        let (mut e, menu, player) = menu_open_engine();
        let parent = e.global::<u32>(0x011d_e12c);
        let lights = e.global::<u32>(0x011d_e134);
        e.mem.set_u8(0x011a_d884, 1);
        e.call_log = Some(vec![]);
        e.call(0x007f_d6c0, &args![menu]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(0x011a_d884), 0);
        // Saved placement: translation (3 dwords), rotation (9 dwords),
        // Bip01 translation, culled flag.
        assert_eq!(e.mem.f32(menu.addr() + 0x110 + 4), 2.0);
        for i in 0..9u32 {
            assert_eq!(e.mem.u32(menu.addr() + 0x11c + 4 * i), 100 + i);
        }
        assert_eq!(e.mem.f32(menu.addr() + 0x140 + 8), 3.0);
        assert_eq!(e.mem.u8(menu.addr() + 0x14c), 1);
        // The player, its saved parent, the new parent node, the lights and
        // the cloned camera were stored.
        assert_eq!(e.mem.u32(menu.addr() + 0xfc), player);
        assert_eq!(e.mem.u32(menu.addr() + 0x10c), player + 0x40);
        assert_eq!(e.mem.u32(menu.addr() + 0x100), parent);
        assert_eq!(e.mem.u32(menu.addr() + 0x108), lights);
        assert_ne!(e.mem.u32(menu.addr() + 0x104), 0);
        // The menu node is told about the new parent, and the glows are
        // updated with 007dcc00's answer last.
        assert_eq!(first_call(&log, 0x007f_df10), [0x8100, parent]);
        let last_update = log.iter().rposition(|(a, _)| *a == 0x007d_cc00).unwrap();
        // (Only the four glow lookups of `LightButton` follow it.)
        assert_eq!(last_update, log.len() - 5);
        // The clone's camera frustum: tan = 13 on all four sides, no scale.
        assert_eq!(first_call(&log, 0x0050_7700).len(), 2);
        // The name lookup for the bone used the shared Bip01 name.
        assert!(calls_to(&log, FIXED_STRING_CONSTRUCT)
            .iter()
            .any(|w| w[1] == BIP01_NAME));
    }

    #[test]
    fn on_menu_close_puts_the_player_back() {
        let mut e = scene_engine();
        stubs(
            &mut e,
            &[
                SET_CULLED,
                SET_TRANSLATE_POINT,
                SET_ROTATE,
                SET_VECTOR,
                0x00b5_d180,
                0x007f_b0b0,
            ],
        );
        let player = node_with_lookup(&mut e);
        let graph = node_with_lookup(&mut e);
        e.set_global(0x011d_e124u32, graph);
        e.register(0x0068_38b0, |_, _| ret(0x8200));
        e.register(0x0055_85e0, |_, _| ret(0x8000));
        e.register(SCENE_NODE_GETTER, |e, _| ret(e.global::<u32>(0x011d_e124)));
        let menu = new_menu(&mut e);
        e.mem.set_u32(menu.addr() + 0xfc, player);
        e.mem.set_u32(menu.addr() + 0x100, graph);
        e.mem.set_u32(menu.addr() + 0x104, 0x104);
        e.mem.set_u32(menu.addr() + 0x108, 0x108);
        e.mem.set_u32(menu.addr() + 0x10c, graph);
        e.mem.set_u32(menu.addr() + 0xf8, 0xf8);
        e.mem.set_u8(menu.addr() + 0x14c, 1);
        // The frame of 0x44 bytes: the zero point at +0x2c, the bone name at +0x28.
        let frame = next_frame(&mut e, 0x44);
        e.call(0x007f_df40, &args![menu]);
        let log = e.call_log.take().unwrap();
        assert_eq!(firsts(&log, POINT_CONSTRUCT), at(frame, &[0x2c]));
        assert_eq!(firsts(&log, FIXED_STRING_CONSTRUCT), at(frame, &[0x28]));
        assert_eq!(e.mem.u8(0x011a_d884), 1);
        // Every pointer the open built is released again.
        for offset in [0xfcu32, 0x100, 0x104, 0xf8, 0x108] {
            assert_eq!(e.mem.u32(menu.addr() + offset), 0, "offset {offset:x}");
        }
        // The saved translation, rotation and culled flag are restored.
        assert!(calls_to(&log, SET_VECTOR)
            .iter()
            .any(|w| w[1] == menu.addr() + 0x110));
        assert!(calls_to(&log, SET_VECTOR)
            .iter()
            .any(|w| w[1] == menu.addr() + 0x140));
        assert_eq!(calls_to(&log, SET_ROTATE)[0][1], menu.addr() + 0x11c);
        assert!(calls_to(&log, SET_CULLED)
            .iter()
            .any(|w| w[0] == player && w[1] == 1));
    }

    #[test]
    fn on_menu_close_does_nothing_when_the_menu_is_not_shown() {
        let mut e = scene_engine();
        e.register(0x0097_4d90, |_, _| ret(0));
        let menu = new_menu(&mut e);
        e.mem.set_u32(menu.addr() + 0xfc, 0x1234);
        e.call(0x007f_df40, &args![menu]);
        assert_eq!(e.mem.u8(0x011a_d884), 1);
        assert_eq!(e.mem.u32(menu.addr() + 0xfc), 0x1234);
    }

    #[test]
    fn draw_renders_into_a_target_and_releases_it() {
        let mut e = scene_engine();
        for (address, value) in [
            (0x0105_559cu32, 0.2f32),
            (0x0107_8a60, 0.4),
            (0x0107_8a64, 0.6),
        ] {
            put_float(&mut e, address, value);
        }
        let root = node_with_lookup(&mut e);
        e.set_global(0x011d_e124u32, root);
        e.register(0x0043_c4b0, |e, _| ret(e.global::<u32>(0x011d_e124)));
        e.register(0x004a_0ea0, |_, _| ret(0x9100));
        e.register(0x00b6_e110, |_, _| ret(0x9200));
        e.register(0x0063_3c90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let current = e.mem.alloc(0x10);
        for (i, v) in [0.5f32, 0.6, 0.7, 0.8].iter().enumerate() {
            e.mem.set_f32(current + 4 * i as u32, *v);
        }
        e.set_global(0x011d_e128u32, current);
        e.register(0x0050_3650, |e, _| ret(e.global::<u32>(0x011d_e128)));
        e.register(0x007f_d640, |_, _| ret(0x9300));
        stubs(
            &mut e,
            &[
                0x00b6_b8d0,
                0x004a_0eb0,
                0x004a_0fd0,
                0x00c4_f270,
                0x00b6_bee0,
                0x00b6_c0d0,
                0x00c4_f2d0,
                0x00b6_b790,
                0x007f_ba00,
                0x00b6_da10,
                0x0045_cec0,
                0x004a_0f60,
            ],
        );
        e.register(0x00b6_b260, |_, a| ret(a[0] + 1));
        // The two virtual calls on the root record the colours they got.
        let colors = Rc::new(RefCell::new(Vec::<Vec<f32>>::new()));
        let seen = colors.clone();
        let vtable = e.mem.u32(root);
        e.mem.set_u32(vtable + 0xb4, 0x0120_0070);
        e.mem.set_u32(vtable + 0xac, 0x0120_0070);
        e.register_double(0x0120_0070, move |e, a| {
            seen.borrow_mut()
                .push((0..4).map(|i| e.mem.f32(a[1] + 4 * i)).collect());
            Ret::default()
        });
        let menu = new_menu(&mut e);
        e.mem.set_u32(menu.addr() + 0x104, 0x1040);
        // The frame of 0x120 bytes: the render target at +0x40, the colours at +0x18, +0x28 and
        // +0x8, the accumulator at +0x44, the shader pointer at +0x38.
        let frame = next_frame(&mut e, 0x120);
        e.call(0x007f_e1f0, &args![menu, 0x55u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            firsts(&log, COLOR_A_CONSTRUCT),
            at(frame, &[0x18, 0x28, 0x8])
        );
        assert_eq!(firsts(&log, 0x0063_3c90), at(frame, &[0x40, 0x38]));
        assert_eq!(firsts(&log, 0x004a_0eb0), at(frame, &[0x44]));
        // The first colour stays zero; the second ends as the fixed colour
        // (constants at 0105559c, 01078a60, 01078a64 and 1.0).
        assert_eq!(colors.borrow()[0], [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(colors.borrow()[1], [0.2, 0.4, 0.6, 1.0]);
        // The scene is drawn with the argument and the target, which is then
        // released (both `NiPointer`s are destroyed).
        let draw = first_call(&log, 0x007f_ba00);
        assert_eq!(draw[..], [menu.addr(), 0x55, 0, 0x9200]);
        assert_eq!(calls_to(&log, 0x0045_cec0).len(), 2);
        assert_eq!(calls_to(&log, 0x004a_0f60).len(), 1);
        // The accumulator is bound to the cloned camera.
        assert_eq!(first_call(&log, 0x00b6_bee0)[0], 0x1040);
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on this thread takes (asked of the engine), with
    /// the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.call_log = Some(vec![]);
        e.next_stack_local(size)
    }

    fn firsts(log: &[(u32, Vec<u32>)], address: u32) -> Vec<u32> {
        calls_to(log, address).iter().map(|w| w[0]).collect()
    }

    fn at(frame: u32, offsets: &[u32]) -> Vec<u32> {
        offsets.iter().map(|o| frame + o).collect()
    }

    #[test]
    fn initialize_keeps_its_temporaries_at_the_exe_offsets() {
        let mut e = scene_engine();
        initialize_stubs(&mut e);
        let node = node_with_lookup(&mut e);
        let body = node_with_lookup(&mut e);
        e.set_global(0x011d_e130u32, body);
        e.set_global(0x011d_e114u32, node);
        e.register(0x0044_7080, |e, _| ret(e.global::<u32>(0x011d_e114)));
        e.register(0x00b5_ca70, |_, _| ret(0));
        let menu = new_menu(&mut e);
        let frame = next_frame(&mut e, 0x21c);
        e.call(0x007f_c970, &args![menu]);
        let log = e.call_log.take().unwrap();
        let names: Vec<u32> = (0..9).map(|i| 0x118 - 4 * i).collect();
        assert_eq!(firsts(&log, FIXED_STRING_CONSTRUCT), at(frame, &names));
        assert_eq!(firsts(&log, COLOR_A_CONSTRUCT), at(frame, &[0x1a4, 0x1e8]));
        assert_eq!(
            firsts(&log, MATRIX_CONSTRUCT),
            at(frame, &[0x1c4, 0x130, 0x180])
        );
        assert_eq!(
            calls_to(&log, 0x0043_f8d0),
            vec![at(frame, &[0x130, 0xd4, 0x1c4])]
        );
        assert_eq!(
            firsts(&log, POINT_CONSTRUCT),
            at(frame, &[0x1f8, 0x204, 0x11c])
        );
        assert_eq!(firsts(&log, 0x00a7_1b70), at(frame, &[0x164]));
        assert_eq!(
            firsts(&log, COLOR_CONSTRUCT),
            at(frame, &[0xb0, 0xa4, 0x98])
        );
    }

    #[test]
    fn on_menu_open_keeps_its_temporaries_at_the_exe_offsets() {
        let (mut e, menu, _) = menu_open_engine();
        let frame = next_frame(&mut e, 0xe4);
        e.call(0x007f_d6c0, &args![menu]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            firsts(&log, POINT_CONSTRUCT),
            at(frame, &[0xc0, 0x78, 0xcc])
        );
        assert_eq!(firsts(&log, FIXED_STRING_CONSTRUCT), at(frame, &[0x68]));
        assert_eq!(firsts(&log, COLOR_A_CONSTRUCT), at(frame, &[0xb0]));
        assert_eq!(firsts(&log, COLOR_CONSTRUCT), at(frame, &[0xa4, 0x4c]));
        assert_eq!(firsts(&log, 0x00a7_1b70), at(frame, &[0x84]));
    }

    #[test]
    fn init_light_keeps_its_colours_at_the_exe_offsets() {
        let mut e = race_engine();
        e.register(SCENE_NODE_GETTER, |_, _| ret(0x77));
        e.register(0x007f_c400, |_, _| ret(1));
        let light = e.mem.alloc(0x40);
        e.set_global(0x011d_e104u32, light);
        e.register(0x0068_38b0, |e, _| ret(e.global::<u32>(0x011d_e104)));
        stubs(
            &mut e,
            &[
                0x004b_c2e0,
                0x004b_c280,
                0x0050_c630,
                0x0050_c660,
                0x0050_c690,
                0x0050_dd20,
                0x0050_dd50,
                0x006b_0570,
                0x0096_5580,
                0x007f_99b0,
                SET_TRANSLATE_POINT,
            ],
        );
        let menu = new_menu(&mut e);
        let frame = next_frame(&mut e, 0x44);
        e.call(0x007f_e560, &args![menu]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            firsts(&log, COLOR_CONSTRUCT),
            at(frame, &[0x1c, 0x10, 0x4, 0x28])
        );
        assert_eq!(firsts(&log, POINT_CONSTRUCT), at(frame, &[0x34]));
    }

    #[test]
    fn on_click_keeps_the_pick_and_the_sound_handle_in_the_frame() {
        let (mut e, menu) = click_engine(2);
        let frame = next_frame(&mut e, 0x8c);
        e.call(0x007f_e6c0, &args![menu, 1u8, 7u32, 8u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(firsts(&log, MATRIX_CONSTRUCT), at(frame, &[0x30, 0x24]));
        assert_eq!(firsts(&log, 0x00e9_8f20), at(frame, &[0x3c]));
        assert_eq!(
            calls_to(&log, 0x00e9_8e20)[0][..3],
            at(frame, &[0x3c, 0x30, 0x24])
        );
        assert_eq!(calls_to(&log, 0x00ad_7550)[0][1], frame + 0x14);
        assert_eq!(firsts(&log, 0x0048_3710), at(frame, &[0x14]));
    }
}
