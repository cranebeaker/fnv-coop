//! `fallout shared/sky/atmosphere.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `Atmosphere` (a `SkyObject`, 0x1c bytes) owns the atmosphere mesh, the sky's fog
//! property and the sky quad. `Update` sets the fog from the sky's colours and fog values
//! each frame. The unit also holds four small `BSFogProperty` setters the compiler placed
//! here (`00633bf0`, `00633c20`, `00633c40`, `00633c60`).
//!
//! The code is compiled without optimisation: every `NiPointer` access and every sky field
//! read is a call, and the translations make those calls in the code's order.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_sky::skyobject::{
    pointer_assign, pointer_get, NI_ALLOCATE, NI_NODE_CONSTRUCT, NI_POINTER_DESTRUCT,
    NODE_SET_FLAG_004902F0,
};

/// `Atmosphere`'s vtable (`0104eb68`).
const ATMOSPHERE_VTABLE: u32 = 0x0104_eb68;
/// `SkyObject::~SkyObject` (`0063f8d0`) and `SkyObject::Initialize` (`0063f940`).
const SKY_OBJECT_DESTRUCT: u32 = 0x0063_f8d0;
const SKY_OBJECT_INITIALIZE: u32 = 0x0063_f940;
/// `operator delete` (`00401030`, `cdecl`, the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// A global byte (`011f941e`): while it is clear the atmosphere makes the sky quad and the
/// fog colour is scaled.
const FLAG_011F941E: u32 = 0x011f_941e;
/// `00b899a0` (unit `skyshader.cpp`, `cdecl`: the address of the `NiPointer` that receives
/// the sky quad).
const MAKE_SKY_QUAD: u32 = 0x00b8_99a0;
/// The camera holder (`0045c670`, no words), the camera's node (`00558310`, `thiscall`) and
/// `0043c490` (`thiscall` on that node: the word the quad's `00440460` takes).
const CAMERA_HOLDER_GET: u32 = 0x0045_c670;
const CAMERA_NODE: u32 = 0x0055_8310;
const NODE_VALUE_0043C490: u32 = 0x0043_c490;
const NODE_SET_00440460: u32 = 0x0044_0460;
/// `BSStream`'s constructor (`0043cfd0`) and destructor (`0043d090`) (`thiscall`, no stack
/// words), `BSStream::Load` (`00c3a8a0`, `thiscall`: path, 0; answers a byte); the local
/// stream is 0x5d4 bytes.
const STREAM_CONSTRUCT: u32 = 0x0043_cfd0;
const STREAM_DESTRUCT: u32 = 0x0043_d090;
const STREAM_LOAD: u32 = 0x00c3_a8a0;
const STREAM_SIZE: u32 = 0x5d4;
/// `Model::Model` (`0043aaf0`, `thiscall`: path, stream, 0, 0), `Model::~Model`
/// (`0043ab70`) and its root object (`0043b230`, `thiscall`); the local model is 0x1c
/// bytes.
const MODEL_CONSTRUCT: u32 = 0x0043_aaf0;
const MODEL_DESTRUCT: u32 = 0x0043_ab70;
const MODEL_ROOT: u32 = 0x0043_b230;
const MODEL_SIZE: u32 = 0x1c;
/// `0045bc00` (`thiscall`, one word: 0; the object a node answers) and the kind test
/// `0043b300` (`cdecl`: the RTTI `011f4a40`, the object).
const NODE_CHILD_0045BC00: u32 = 0x0045_bc00;
const IS_KIND_OF: u32 = 0x0043_b300;
const RTTI_011F4A40: u32 = 0x011f_4a40;
/// `"Meshes\\Sky\\Atmosphere.nif"` and `"Cannot load the atmosphere model."`, and the message
/// function `005b5e40` (`cdecl`, the text).
const ATMOSPHERE_PATH: u32 = 0x0104_eba0;
const CANNOT_LOAD_MESSAGE: u32 = 0x0104_eb7c;
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `BSShaderManager::GetFogProperty` (`00b55520`, `cdecl`: 0) and `::SetFogProperty`
/// (`00b55540`, `cdecl`: 0, the property).
const GET_FOG_PROPERTY: u32 = 0x00b5_5520;
const SET_FOG_PROPERTY: u32 = 0x00b5_5540;
/// The base update `008d0600` (`thiscall`: the sky, the time; its body is a bare `RET 8`,
/// which the engine map names after another folded function).
const BASE_UPDATE_008D0600: u32 = 0x008d_0600;
/// `Sky` accessors (`thiscall`, no stack words): the mode (`+0xf8`, `00476c90`), the
/// colours `SkyColor[1]` (`+0x48`, `00624700`), `[8]` (`+0x9c`, `0050d100`), `[7]`
/// (`+0x90`, `009c1a50`) and `[0]` (`+0x3c`, `005a8080`), the fog distances (`+0xd4`,
/// `00923000`), `fFogPower` (`+0xe8`, `004e45e0`, in `ST0`), the `float` at `+0xe4`
/// (`009a5480`, in `ST0`) and the water fog colour (`+0xb4`, `006ab360`).
const SKY_GET_MODE: u32 = 0x0047_6c90;
const SKY_COLOR_1: u32 = 0x0062_4700;
const SKY_COLOR_8: u32 = 0x0050_d100;
const SKY_COLOR_7: u32 = 0x009c_1a50;
const SKY_COLOR_0: u32 = 0x005a_8080;
const SKY_FOG_DISTANCES: u32 = 0x0092_3000;
const SKY_FOG_POWER: u32 = 0x004e_45e0;
const SKY_FLOAT_E4: u32 = 0x009a_5480;
const SKY_WATER_FOG_COLOR: u32 = 0x006a_b360;
/// The player (a pointer global), its parent cell (`008d6f30`, `+0x40`) and
/// `TESObjectREFR::GetRelevantWaterHeight` (`0057b0a0`, `float` in `ST0`); a node's world
/// position (`0045bb80`).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
const PLAYER_PARENT_CELL: u32 = 0x008d_6f30;
const RELEVANT_WATER_HEIGHT: u32 = 0x0057_b0a0;
const NODE_WORLD_POSITION: u32 = 0x0045_bb80;
/// `NiColorA::NiColorA(r, g, b, a)` (`00414430`, `thiscall`, four `float`s).
const COLOR_A_CONSTRUCT: u32 = 0x0041_4430;
/// The three `NiColorA` globals the update fills (`011ff8e0`, `011ff8f0`, `011ff900`).
const FOG_COLOR_A: u32 = 0x011f_f8e0;
const FOG_COLOR_B: u32 = 0x011f_f8f0;
const FOG_COLOR_C: u32 = 0x011f_f900;
/// The fog property calls: the flag-bit-0 setter (`0049ed90`, one word), the distances
/// (`005bf9a0`, two `float`s), the power (`005bfbd0`, one `float`).
const FOG_SET_FLAG: u32 = 0x0049_ed90;
const FOG_SET_DISTANCES: u32 = 0x005b_f9a0;
const FOG_SET_POWER: u32 = 0x005b_fbd0;
/// `NiColor * float` (`00633b40`, `thiscall` on the colour: the result, the factor).
const COLOR_SCALE: u32 = 0x0063_3b40;
/// The `float` and `double` the fog colour scale divides (`011f9190`, `0104ebc0`).
const COLOR_SCALE_NUMERATOR: u32 = 0x011f_9190;
const COLOR_SCALE_DENOMINATOR: u32 = 0x0104_ebc0;
/// The camera: its `NiPointer` at `+0xac` of the holder (`006629f0`), its frustum
/// (`0045bbe0`, `+0xdc`, seven words), the `float` setter `00507700` (`+0xfc`) and
/// `NiCamera::SetViewFrustum` (`00a6faf0`).
const CAMERA_GET: u32 = 0x0066_29f0;
const CAMERA_FRUSTUM: u32 = 0x0045_bbe0;
const CAMERA_SET_00507700: u32 = 0x0050_7700;
const CAMERA_SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;

layout! {
    /// `Atmosphere` (Xbox PDB), 0x1c bytes (`Sky::Initialize` allocates 0x1c).
    pub struct Atmosphere: 0x1C {
        /// `SkyObject::spRoot` (Xbox PDB).
        0x04 spRoot: u32,
        /// `spAtmosphere` (Xbox PDB): `NiPointer<NiTriShape>`.
        0x08 spAtmosphere: u32,
        /// `spFog` (Xbox PDB): `NiPointer<BSFogProperty>`.
        0x0C spFog: u32,
        /// `spSkyQuadNode` (Xbox PDB): `NiPointer<NiNode>`.
        0x10 spSkyQuadNode: u32,
        /// `spSkyQuad` (Xbox PDB): `NiPointer<NiTriShape>`.
        0x14 spSkyQuad: u32,
        /// `bUpdateFogDistance` (Xbox PDB).
        0x18 bUpdateFogDistance: bool,
    }
}

/// A `float` read with `FLD dword` and pushed with `FSTP dword` (the bit pattern, quieted).
fn pushed_float(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// Builds an `NiColorA` from the colour at `color` (alpha 1.0) in a local and copies it into
/// the global at `to`.
fn store_color(e: &mut Engine, color_of: impl Fn(&mut Engine) -> u32, to: u32) {
    let alpha = 1.0f32.to_bits();
    let color = color_of(e);
    let blue = pushed_float(e, color + 8);
    let color = color_of(e);
    let green = pushed_float(e, color + 4);
    let color = color_of(e);
    let red = pushed_float(e, color);
    e.with_stack(16, |e, local| {
        let local = local.addr();
        let made = e
            .call(COLOR_A_CONSTRUCT, &args![local, red, green, blue, alpha])
            .u32();
        for word in 0..4 {
            let value = e.mem.u32(made + 4 * word);
            e.mem.set_u32(to + 4 * word, value);
        }
    });
}

// Translated from 00633160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Atmosphere`'s scalar deleting destructor (engine map): the destructor, then `operator
/// delete` when `flags & 1`. Answers `this`.
pub fn atmosphere_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Atmosphere>,
    flags: u32,
) -> Ptr<Atmosphere> {
    atmosphere_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00633190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Atmosphere::~Atmosphere` (engine map; `thiscall`, no stack words): the vtable,
/// `spFog`, `spAtmosphere`, `spSkyQuad` and `spSkyQuadNode` assigned 0, their `NiPointer`
/// destructors (sky quad, quad node, fog, atmosphere), then `SkyObject::~SkyObject`.
// Translation incomplete: 00633190 - the C++ exception frame (FS:[0] chain, unwind states 4 to 0 for the members) is not translated.
pub fn atmosphere_destructor(e: &mut Engine, this: Ptr<Atmosphere>) {
    let base = this.addr();
    e.mem.set_u32(base, ATMOSPHERE_VTABLE);
    for offset in [0xcu32, 0x8, 0x14, 0x10] {
        pointer_assign(e, base + offset, 0);
    }
    for offset in [0x14u32, 0x10, 0xc, 0x8] {
        e.call(NI_POINTER_DESTRUCT, &args![base + offset]);
    }
    e.call(SKY_OBJECT_DESTRUCT, &args![base]);
}

// Translated from 00633260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Atmosphere::Initialize` (engine map; `thiscall`, two words: the parent node, the fog
/// property). `SkyObject::Initialize`; while `011f941e` is clear: the sky quad (`00b899a0`
/// into `spSkyQuad`, flag 1 through `004902f0`, `00440460` with the camera node's
/// `0043c490` word), a new quad node (`NiNode`, 0xac bytes) as `spSkyQuadNode` with flag 1,
/// the quad attached to it and it to the root (vtable slot +0xdc, `AttachChild`, with 1).
/// Then loads `Meshes\Sky\Atmosphere.nif` through a local `BSStream`; when it loads, the
/// model's root (a node's `0045bc00(0)`, or the root itself when `0043b300` says it is of
/// kind `011f4a40`) becomes `spAtmosphere` with flag 1 and is attached to the root; without
/// such an object it prints "Cannot load the atmosphere model." and returns before the fog.
/// Finally `spFog` takes the fog property and `BSShaderManager::SetFogProperty(0, fog)`.
// Translation incomplete: 00633260 - the C++ exception frame (FS:[0] chain, unwind states for the allocation, stream and model) and the /GS cookie check (00ec408c) are not translated.
pub fn atmosphere_initialize(e: &mut Engine, this: Ptr<Atmosphere>, parent: u32, fog: u32) {
    let base = this.addr();
    e.call(SKY_OBJECT_INITIALIZE, &args![base, parent]);
    if e.global::<u8>(FLAG_011F941E) == 0 {
        e.call(MAKE_SKY_QUAD, &args![base + 0x14]);
        let quad = pointer_get(e, base + 0x14);
        e.call(NODE_SET_FLAG_004902F0, &args![quad, 1u32]);
        let holder = e.call(CAMERA_HOLDER_GET, &args![]).u32();
        let node = e.call(CAMERA_NODE, &args![holder]).u32();
        let word = e.call(NODE_VALUE_0043C490, &args![node]).u32();
        let quad = pointer_get(e, base + 0x14);
        e.call(NODE_SET_00440460, &args![quad, word]);
        let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
        let quad_node = if block != 0 {
            e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
        } else {
            0
        };
        pointer_assign(e, base + 0x10, quad_node);
        let quad_node = pointer_get(e, base + 0x10);
        e.call(NODE_SET_FLAG_004902F0, &args![quad_node, 1u32]);
        let quad_node = pointer_get(e, base + 0x10);
        let quad = pointer_get(e, base + 0x14);
        e.vcall(quad_node, 0xdc, &args![quad, 1u32]);
        let root = pointer_get(e, base + 4);
        let quad_node = pointer_get(e, base + 0x10);
        e.vcall(root, 0xdc, &args![quad_node, 1u32]);
    }
    let stream = e.mem.alloc(STREAM_SIZE);
    e.call(STREAM_CONSTRUCT, &args![stream]);
    let loaded = e
        .call(STREAM_LOAD, &args![stream, ATMOSPHERE_PATH, 0u32])
        .u8();
    if loaded != 0 {
        let model = e.mem.alloc(MODEL_SIZE);
        e.call(
            MODEL_CONSTRUCT,
            &args![model, ATMOSPHERE_PATH, stream, 0u32, 0u32],
        );
        e.call(MODEL_ROOT, &args![model]);
        let mut found = 0u32;
        let object = e.call(MODEL_ROOT, &args![model]).u32();
        if object != 0 {
            if e.vcall(object, 0xc, &args![]).u32() != 0 {
                found = e.call(NODE_CHILD_0045BC00, &args![object, 0u32]).u32();
            } else if e.call(IS_KIND_OF, &args![RTTI_011F4A40, object]).u8() != 0 {
                found = object;
            }
        }
        if found == 0 {
            e.call(PRINT_MESSAGE, &args![CANNOT_LOAD_MESSAGE]);
            e.call(MODEL_DESTRUCT, &args![model]);
            e.mem.free(model);
            e.call(STREAM_DESTRUCT, &args![stream]);
            e.mem.free(stream);
            return;
        }
        pointer_assign(e, base + 8, found);
        let mesh = pointer_get(e, base + 8);
        e.call(NODE_SET_FLAG_004902F0, &args![mesh, 1u32]);
        let root = pointer_get(e, base + 4);
        let mesh = pointer_get(e, base + 8);
        e.vcall(root, 0xdc, &args![mesh, 1u32]);
        e.call(MODEL_DESTRUCT, &args![model]);
        e.mem.free(model);
    }
    pointer_assign(e, base + 0xc, fog);
    e.call(SET_FOG_PROPERTY, &args![0u32, fog]);
    e.call(STREAM_DESTRUCT, &args![stream]);
    e.mem.free(stream);
}

// Translated from 006335d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Atmosphere::Update` (engine map; `thiscall`, two words: the sky, the time). Nothing
/// without a sky. Makes `spFog` the shader manager's fog property when it is not, runs the
/// base update `008d0600`. In sky modes 3 and 2, with an atmosphere mesh (else it returns):
/// fills the three fog `NiColorA` globals (alpha 1) from `SkyColor[1]` when the player is in
/// a cell and the camera node's height (`+8` of its world position) is below the player's
/// relevant water height, else from `SkyColor[8]`, `[7]` and `[0]`. With a fog property:
/// the fog's flag bit 0 says near < far; with `bUpdateFogDistance` the distances and the
/// power go to the fog; its colour is `SkyColor[1]` scaled by `011f9190 / 0104ebc0` while
/// `011f941e` is clear (`SkyColor[1]` itself when set), its height the sky's `+0xe4`, its
/// water distances the third and fourth fog distances and its water colour the sky's water
/// fog colour. Then, with `bUpdateFogDistance` and a camera whose frustum's far plane (word
/// 5) is not equal to the camera holder's vtable slot +0x100 `float`: the far plane takes
/// it, the camera's `00507700` takes far / frustum word 4, and the frustum is set again.
pub fn atmosphere_update(e: &mut Engine, this: Ptr<Atmosphere>, sky: u32, time: f32) {
    let base = this.addr();
    if sky == 0 {
        return;
    }
    let current = e.call(GET_FOG_PROPERTY, &args![0u32]).u32();
    if current != pointer_get(e, base + 0xc) {
        let fog = pointer_get(e, base + 0xc);
        e.call(SET_FOG_PROPERTY, &args![0u32, fog]);
    }
    e.call(
        BASE_UPDATE_008D0600,
        &args![base, sky, x87_quiet_bits(time.to_bits())],
    );
    let mode_applies = e.call(SKY_GET_MODE, &args![sky]).u32() == 3
        || e.call(SKY_GET_MODE, &args![sky]).u32() == 2;
    if mode_applies {
        if pointer_get(e, base + 8) == 0 {
            return;
        }
        let mut under_water = false;
        let player = e.global::<u32>(PLAYER_CHARACTER);
        if player != 0 && e.call(PLAYER_PARENT_CELL, &args![player]).u32() != 0 {
            let holder = e.call(CAMERA_HOLDER_GET, &args![]).u32();
            let node = e.call(CAMERA_NODE, &args![holder]).u32();
            let position = e.call(NODE_WORLD_POSITION, &args![node]).u32();
            let player = e.global::<u32>(PLAYER_CHARACTER);
            let water = e.call(RELEVANT_WATER_HEIGHT, &args![player]).f64();
            // FCOMPP of the height against the water height; JP leaves unless below.
            under_water = x87_m32(e.mem.u32(position + 8)) < water;
        }
        if under_water {
            for to in [FOG_COLOR_A, FOG_COLOR_B, FOG_COLOR_C] {
                store_color(e, |e| e.call(SKY_COLOR_1, &args![sky]).u32(), to);
            }
        } else {
            store_color(e, |e| e.call(SKY_COLOR_8, &args![sky]).u32(), FOG_COLOR_A);
            store_color(e, |e| e.call(SKY_COLOR_7, &args![sky]).u32(), FOG_COLOR_B);
            store_color(e, |e| e.call(SKY_COLOR_0, &args![sky]).u32(), FOG_COLOR_C);
        }
    }
    if pointer_get(e, base + 0xc) != 0 {
        let distances = e.call(SKY_FOG_DISTANCES, &args![sky]).u32();
        let near = x87_m32(e.mem.u32(distances));
        let far = x87_m32(e.mem.u32(distances + 4));
        // FCOMPP of near against far; JP clears the flag unless near is below.
        let enabled = near < far;
        let fog = pointer_get(e, base + 0xc);
        e.call(FOG_SET_FLAG, &args![fog, enabled as u32]);
        if e.mem.u8(base + 0x18) != 0 {
            let near = pushed_float(e, distances);
            let far = pushed_float(e, distances + 4);
            let fog = pointer_get(e, base + 0xc);
            e.call(FOG_SET_DISTANCES, &args![fog, near, far]);
            let power = e.call(SKY_FOG_POWER, &args![sky]).f64();
            let fog = pointer_get(e, base + 0xc);
            e.call(FOG_SET_POWER, &args![fog, x87_store32(power)]);
        }
        if e.global::<u8>(FLAG_011F941E) == 0 {
            let numerator = x87_m32(e.global::<u32>(COLOR_SCALE_NUMERATOR));
            let denominator = e.global::<f64>(COLOR_SCALE_DENOMINATOR);
            let scale = x87_store32(x87_div(numerator, denominator));
            e.with_stack(12, |e, result| {
                let color = e.call(SKY_COLOR_1, &args![sky]).u32();
                let scaled = e
                    .call(
                        COLOR_SCALE,
                        &args![color, result.addr(), x87_quiet_bits(scale)],
                    )
                    .u32();
                let fog = pointer_get(e, base + 0xc);
                fn_00633bf0(e, Ptr::new(fog), scaled);
            });
        } else {
            let color = e.call(SKY_COLOR_1, &args![sky]).u32();
            let fog = pointer_get(e, base + 0xc);
            fn_00633bf0(e, Ptr::new(fog), color);
        }
        // The first read is stored in a local nobody reads.
        e.call(SKY_FLOAT_E4, &args![sky]);
        let height = x87_store32(e.call(SKY_FLOAT_E4, &args![sky]).f64());
        let fog = pointer_get(e, base + 0xc);
        fn_00633c40(e, Ptr::new(fog), f32::from_bits(height));
        let start = pushed_float(e, distances + 8);
        let end = pushed_float(e, distances + 0xc);
        let fog = pointer_get(e, base + 0xc);
        fn_00633c20(e, Ptr::new(fog), f32::from_bits(start), f32::from_bits(end));
        let water_color = e.call(SKY_WATER_FOG_COLOR, &args![sky]).u32();
        let fog = pointer_get(e, base + 0xc);
        fn_00633c60(e, Ptr::new(fog), water_color);
    }
    if e.mem.u8(base + 0x18) == 0 {
        return;
    }
    let holder = e.call(CAMERA_HOLDER_GET, &args![]).u32();
    let far = x87_store32(e.vcall(holder, 0x100, &args![]).f64());
    let holder = e.call(CAMERA_HOLDER_GET, &args![]).u32();
    let camera = e.call(CAMERA_GET, &args![holder]).u32();
    if camera == 0 {
        return;
    }
    let frustum = e.call(CAMERA_FRUSTUM, &args![camera]).u32();
    e.with_stack(0x1c, |e, copy| {
        let copy = copy.addr();
        for word in 0..7 {
            let value = e.mem.u32(frustum + 4 * word);
            e.mem.set_u32(copy + 4 * word, value);
        }
        // FUCOMPP of the frustum's far plane against the new far; JNP leaves when equal.
        let current = x87_m32(e.mem.u32(copy + 0x14));
        let new_far = x87_m32(far);
        if current == new_far {
            return;
        }
        e.mem.set_u32(copy + 0x14, x87_quiet_bits(far));
        let ratio = x87_store32(x87_div(new_far, x87_m32(e.mem.u32(copy + 0x10))));
        e.call(CAMERA_SET_00507700, &args![camera, x87_quiet_bits(ratio)]);
        e.call(CAMERA_SET_VIEW_FRUSTUM, &args![camera, copy]);
    });
}

// Translated from 00633bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the sky's `BSFogProperty`, one word: an
/// `NiColor`): copies the colour into `m_kColor` (`+0x20`).
pub fn fn_00633bf0(e: &mut Engine, this: Ptr, color: u32) {
    let fog = this.addr();
    for word in 0..3 {
        let value = e.mem.u32(color + 4 * word);
        e.mem.set_u32(fog + 0x20 + 4 * word, value);
    }
}

// Translated from 00633c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `BSFogProperty`, two `float`s): sets
/// `fStartWaterDistance` (`+0x34`) and `fEndWaterDistance` (`+0x38`).
pub fn fn_00633c20(e: &mut Engine, this: Ptr, start: f32, end: f32) {
    let fog = this.addr();
    e.mem.set_f32(fog + 0x34, x87_quiet(start));
    e.mem.set_f32(fog + 0x38, x87_quiet(end));
}

// Translated from 00633c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `BSFogProperty`, one `float`): sets `fHeight`
/// (`+0x50`).
pub fn fn_00633c40(e: &mut Engine, this: Ptr, height: f32) {
    e.mem.set_f32(this.addr() + 0x50, x87_quiet(height));
}

// Translated from 00633c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `BSFogProperty`, one word: an `NiColor`):
/// copies the colour into `kWaterColor` (`+0x54`).
pub fn fn_00633c60(e: &mut Engine, this: Ptr, color: u32) {
    let fog = this.addr();
    for word in 0..3 {
        let value = e.mem.u32(color + 4 * word);
        e.mem.set_u32(fog + 0x54 + 4 * word, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0063_3160,
            atmosphere_scalar_deleting_destructor(Ptr<Atmosphere>, u32) -> Ptr<Atmosphere>
        ),
        entry!(0x0063_3190, atmosphere_destructor(Ptr<Atmosphere>)),
        entry!(
            0x0063_3260,
            atmosphere_initialize(Ptr<Atmosphere>, u32, u32)
        ),
        entry!(0x0063_35d0, atmosphere_update(Ptr<Atmosphere>, u32, f32)),
        entry!(0x0063_3bf0, fn_00633bf0(Ptr, u32)),
        entry!(0x0063_3c20, fn_00633c20(Ptr, f32, f32)),
        entry!(0x0063_3c40, fn_00633c40(Ptr, f32)),
        entry!(0x0063_3c60, fn_00633c60(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::fallout_shared_sky::skyobject::test_support::*;

    #[test]
    fn scalar_deleting_destructor_clears_the_members() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[NI_POINTER_DESTRUCT, SKY_OBJECT_DESTRUCT, OPERATOR_DELETE],
        );
        let this = e.mem.alloc(0x1c);
        for offset in [8u32, 0xc, 0x10, 0x14] {
            e.mem.set_u32(this + offset, 0x100 + offset);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_3160, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ATMOSPHERE_VTABLE);
        for offset in [8u32, 0xc, 0x10, 0x14] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(
            calls(&e, NI_POINTER_DESTRUCT),
            vec![
                vec![this + 0x14],
                vec![this + 0x10],
                vec![this + 0xc],
                vec![this + 8]
            ]
        );
        assert_eq!(calls(&e, SKY_OBJECT_DESTRUCT), vec![vec![this]]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0063_3160, &args![this, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    struct Init {
        e: Engine,
        this: u32,
        root_attach: u32,
    }

    /// `loaded`: what `BSStream::Load` answers; `is_node`: the model root's slot +0xc;
    /// `kind`: what the kind test answers.
    fn init_rig(flag: u8, loaded: u32, root: u32, is_node: u32, kind: u32) -> Init {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                SKY_OBJECT_INITIALIZE,
                MAKE_SKY_QUAD,
                NODE_SET_FLAG_004902F0,
                NODE_SET_00440460,
                STREAM_CONSTRUCT,
                STREAM_DESTRUCT,
                MODEL_CONSTRUCT,
                MODEL_DESTRUCT,
                PRINT_MESSAGE,
                SET_FOG_PROPERTY,
                0x0000_7770,
                0x0000_7780,
            ],
        );
        e.map(FLAG_011F941E, 1);
        e.set_global(FLAG_011F941E, flag);
        e.register(CAMERA_HOLDER_GET, |_, _| answer(0x0000_0c00));
        e.register(CAMERA_NODE, |_, _| answer(0x0000_0c04));
        e.register(NODE_VALUE_0043C490, |_, _| answer(0x0000_0c08));
        e.register(NI_ALLOCATE, |_, _| answer(0x0060_0000));
        e.register(NI_NODE_CONSTRUCT, |_, a| answer(a[0]));
        e.register_double(STREAM_LOAD, move |_, _| answer(loaded));
        let model_root = if root != 0 {
            object_with(&mut e, 8, &[(0xc, 0x0000_7790)])
        } else {
            0
        };
        e.register_double(MODEL_ROOT, move |_, _| answer(model_root));
        e.register_double(0x0000_7790, move |_, _| answer(is_node));
        e.register(NODE_CHILD_0045BC00, |_, _| answer(0x0000_0d00));
        e.register_double(IS_KIND_OF, move |_, _| answer(kind));
        let this = e.mem.alloc(0x1c);
        let root_node = object_with(&mut e, 4, &[(0xdc, 0x0000_7770)]);
        e.mem.set_u32(this + 4, root_node);
        // The quad node made from the allocation attaches through 0x7780.
        e.map(0x0060_0000, 0x10);
        let quad_vtable = e.mem.alloc(0xe0);
        e.mem.set_u32(quad_vtable + 0xdc, 0x0000_7780);
        e.mem.set_u32(0x0060_0000, quad_vtable);
        Init {
            e,
            this,
            root_attach: root_node,
        }
    }

    #[test]
    fn initialize_makes_the_quad_loads_the_mesh_and_sets_the_fog() {
        let mut r = init_rig(0, 1, 1, 1, 0);
        let (this, root) = (r.this, r.root_attach);
        r.e.mem.set_u32(this + 0x14, 0x0000_0e00);
        r.e.call_log = Some(vec![]);
        r.e.call(0x0063_3260, &args![this, 0x55u32, 0x66u32]);
        let e = &r.e;
        assert_eq!(calls(e, SKY_OBJECT_INITIALIZE), vec![vec![this, 0x55]]);
        assert_eq!(calls(e, MAKE_SKY_QUAD), vec![vec![this + 0x14]]);
        assert_eq!(calls(e, NODE_SET_00440460), vec![vec![0x0e00, 0x0c08]]);
        assert_eq!(e.mem.u32(this + 0x10), 0x0060_0000);
        assert_eq!(calls(e, 0x0000_7780), vec![vec![0x0060_0000, 0x0e00, 1]]);
        assert_eq!(
            calls(e, 0x0000_7770),
            vec![vec![root, 0x0060_0000, 1], vec![root, 0x0d00, 1]]
        );
        assert_eq!(e.mem.u32(this + 8), 0x0d00);
        assert_eq!(
            calls(e, NODE_SET_FLAG_004902F0),
            vec![vec![0x0e00, 1], vec![0x0060_0000, 1], vec![0x0d00, 1]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0x66);
        assert_eq!(calls(e, SET_FOG_PROPERTY), vec![vec![0, 0x66]]);
        assert_eq!(calls(e, MODEL_DESTRUCT).len(), 1);
        assert_eq!(calls(e, STREAM_DESTRUCT).len(), 1);
        assert_eq!(calls(e, STREAM_LOAD)[0][1..], [ATMOSPHERE_PATH, 0]);
    }

    #[test]
    fn initialize_with_the_flag_set_skips_the_quad_and_takes_a_shape_root() {
        let mut r = init_rig(1, 1, 1, 0, 1);
        let this = r.this;
        r.e.call_log = Some(vec![]);
        r.e.call(0x0063_3260, &args![this, 0x55u32, 0x66u32]);
        assert!(calls(&r.e, MAKE_SKY_QUAD).is_empty());
        let mesh = r.e.mem.u32(this + 8);
        assert_ne!(mesh, 0);
        assert!(calls(&r.e, NODE_CHILD_0045BC00).is_empty());
        assert_eq!(calls(&r.e, IS_KIND_OF), vec![vec![RTTI_011F4A40, mesh]]);
        assert_eq!(r.e.mem.u32(this + 0xc), 0x66);
    }

    #[test]
    fn initialize_without_a_mesh_prints_and_skips_the_fog() {
        for (root, kind) in [(1u32, 0u32), (0, 0)] {
            let mut r = init_rig(1, 1, root, 0, kind);
            let this = r.this;
            r.e.call_log = Some(vec![]);
            r.e.call(0x0063_3260, &args![this, 0x55u32, 0x66u32]);
            assert_eq!(calls(&r.e, PRINT_MESSAGE), vec![vec![CANNOT_LOAD_MESSAGE]]);
            assert!(calls(&r.e, SET_FOG_PROPERTY).is_empty());
            assert_eq!(r.e.mem.u32(this + 0xc), 0);
            assert_eq!(calls(&r.e, MODEL_DESTRUCT).len(), 1);
            assert_eq!(calls(&r.e, STREAM_DESTRUCT).len(), 1);
        }
        // A stream that does not load still sets the fog.
        let mut r = init_rig(1, 0, 1, 0, 0);
        let this = r.this;
        r.e.call_log = Some(vec![]);
        r.e.call(0x0063_3260, &args![this, 0x55u32, 0x66u32]);
        assert!(calls(&r.e, MODEL_CONSTRUCT).is_empty());
        assert_eq!(calls(&r.e, SET_FOG_PROPERTY), vec![vec![0, 0x66]]);
    }

    struct Update {
        e: Engine,
        this: u32,
        sky: u32,
        fog: u32,
    }

    fn update_rig(mode: u32, flag: u8) -> Update {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                SET_FOG_PROPERTY,
                BASE_UPDATE_008D0600,
                FOG_SET_FLAG,
                FOG_SET_DISTANCES,
                FOG_SET_POWER,
                CAMERA_SET_00507700,
                CAMERA_SET_VIEW_FRUSTUM,
            ],
        );
        e.register(GET_FOG_PROPERTY, |_, _| answer(0x0000_0f00));
        let sky = e.mem.alloc(0x138);
        e.mem.set_u32(sky + 0xf8, mode);
        for (i, offset) in [0x3cu32, 0x48, 0x90, 0x9c].into_iter().enumerate() {
            for word in 0..3u32 {
                e.mem
                    .set_f32(sky + offset + 4 * word, (i * 10) as f32 + word as f32);
            }
        }
        for (i, value) in [100.0f32, 200.0, 300.0, 400.0].into_iter().enumerate() {
            e.mem.set_f32(sky + 0xd4 + 4 * i as u32, value);
        }
        e.mem.set_f32(sky + 0xe4, 7.5);
        e.mem.set_f32(sky + 0xe8, 2.0);
        e.mem.set_u32(sky + 0xb4, 0x3f80_0000);
        e.register(SKY_GET_MODE, |e, a| answer(e.mem.u32(a[0] + 0xf8)));
        for (address, offset) in [
            (SKY_COLOR_1, 0x48u32),
            (SKY_COLOR_8, 0x9c),
            (SKY_COLOR_7, 0x90),
            (SKY_COLOR_0, 0x3c),
            (SKY_FOG_DISTANCES, 0xd4),
            (SKY_WATER_FOG_COLOR, 0xb4),
        ] {
            e.register_double(address, move |_, a| answer(a[0] + offset));
        }
        e.register(SKY_FOG_POWER, |e, a| answer_float(e.mem.f32(a[0] + 0xe8)));
        e.register(SKY_FLOAT_E4, |e, a| answer_float(e.mem.f32(a[0] + 0xe4)));
        e.register(COLOR_A_CONSTRUCT, |e, a| {
            for word in 0..4u32 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            answer(a[0])
        });
        e.register(COLOR_SCALE, |e, a| {
            let factor = f32::from_bits(a[2]);
            for word in 0..3u32 {
                let value = e.mem.f32(a[0] + 4 * word) * factor;
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            answer(a[1])
        });
        for global in [FOG_COLOR_A, FOG_COLOR_B, FOG_COLOR_C] {
            e.map(global, 16);
        }
        e.map(FLAG_011F941E, 1);
        e.set_global(FLAG_011F941E, flag);
        e.map(COLOR_SCALE_NUMERATOR, 4);
        e.set_global(COLOR_SCALE_NUMERATOR, 3.0f32);
        e.map(COLOR_SCALE_DENOMINATOR, 8);
        e.set_global(COLOR_SCALE_DENOMINATOR, 2.0f64);
        e.map(PLAYER_CHARACTER, 4);
        let this = e.mem.alloc(0x1c);
        let fog = e.mem.alloc(0x64);
        e.mem.set_u32(this + 8, 0x0000_0a00);
        e.mem.set_u32(this + 0xc, fog);
        Update { e, this, sky, fog }
    }

    #[test]
    fn update_without_a_sky_does_nothing() {
        let mut u = update_rig(3, 0);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_35d0, &args![u.this, 0u32, 1.0f32]);
        assert_eq!(u.e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn update_in_mode_3_fills_the_fog_colours_from_the_sky() {
        let mut u = update_rig(3, 0);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_35d0, &args![u.this, u.sky, 0.25f32]);
        let e = &u.e;
        assert_eq!(calls(e, SET_FOG_PROPERTY), vec![vec![0, u.fog]]);
        assert_eq!(
            calls(e, BASE_UPDATE_008D0600),
            vec![vec![u.this, u.sky, 0.25f32.to_bits()]]
        );
        // Not under water (no player): colours 8, 7 and 0.
        assert_eq!(e.mem.f32(FOG_COLOR_A), 30.0);
        assert_eq!(e.mem.f32(FOG_COLOR_A + 12), 1.0);
        assert_eq!(e.mem.f32(FOG_COLOR_B + 4), 21.0);
        assert_eq!(e.mem.f32(FOG_COLOR_C + 8), 2.0);
        // The fog: enabled (100 < 200), colour 1 scaled by 1.5, height, water distances
        // and colour.
        assert_eq!(calls(e, FOG_SET_FLAG), vec![vec![u.fog, 1]]);
        assert!(calls(e, FOG_SET_DISTANCES).is_empty());
        assert_eq!(e.mem.f32(u.fog + 0x20), 15.0);
        assert_eq!(e.mem.f32(u.fog + 0x28), 18.0);
        assert_eq!(e.mem.f32(u.fog + 0x50), 7.5);
        assert_eq!(e.mem.f32(u.fog + 0x34), 300.0);
        assert_eq!(e.mem.f32(u.fog + 0x38), 400.0);
        assert_eq!(e.mem.u32(u.fog + 0x54), 0x3f80_0000);
    }

    #[test]
    fn update_under_water_uses_colour_1_for_all_three() {
        let mut u = update_rig(2, 1);
        let player = u.e.mem.alloc(0x50);
        u.e.set_global(PLAYER_CHARACTER, player);
        u.e.register(PLAYER_PARENT_CELL, |_, _| answer(1));
        u.e.register(CAMERA_HOLDER_GET, |_, _| answer(0x0c00));
        u.e.register(CAMERA_NODE, |_, _| answer(0x0c04));
        let position = u.e.mem.alloc(12);
        u.e.mem.set_f32(position + 8, -5.0);
        u.e.register_double(NODE_WORLD_POSITION, move |_, _| answer(position));
        u.e.register(RELEVANT_WATER_HEIGHT, |_, _| answer_float(0.0));
        u.e.call(0x0063_35d0, &args![u.this, u.sky, 0.25f32]);
        for global in [FOG_COLOR_A, FOG_COLOR_B, FOG_COLOR_C] {
            assert_eq!(u.e.mem.f32(global), 10.0);
        }
        // The flag set: colour 1 unscaled.
        assert_eq!(u.e.mem.f32(u.fog + 0x20), 10.0);
    }

    #[test]
    fn update_outside_the_modes_and_without_a_mesh() {
        let mut u = update_rig(1, 0);
        u.e.mem.set_f32(u.sky + 0xd4, 500.0);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_35d0, &args![u.this, u.sky, 0.25f32]);
        assert!(calls(&u.e, SKY_COLOR_8).is_empty());
        assert_eq!(calls(&u.e, FOG_SET_FLAG), vec![vec![u.fog, 0]]);
        // Mode 3 without the atmosphere mesh returns before the fog.
        let mut u = update_rig(3, 0);
        u.e.mem.set_u32(u.this + 8, 0);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_35d0, &args![u.this, u.sky, 0.25f32]);
        assert!(calls(&u.e, FOG_SET_FLAG).is_empty());
    }

    #[test]
    fn update_with_fog_distances_sets_them_and_the_camera_far_plane() {
        let mut u = update_rig(1, 1);
        u.e.mem.set_u8(u.this + 0x18, 1);
        let holder = object_with(&mut u.e, 0xb0, &[(0x100, 0x0000_7a00)]);
        u.e.register(0x0000_7a00, |_, _| answer_float(8000.0));
        u.e.register_double(CAMERA_HOLDER_GET, move |_, _| answer(holder));
        let camera = u.e.mem.alloc(0x100);
        u.e.mem.set_u32(holder + 0xac, camera);
        u.e.register(CAMERA_GET, |e, a| answer(e.mem.u32(a[0] + 0xac)));
        u.e.register(CAMERA_FRUSTUM, |_, a| answer(a[0] + 0xdc));
        for word in 0..7u32 {
            u.e.mem
                .set_f32(camera + 0xdc + 4 * word, word as f32 * 1000.0);
        }
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_35d0, &args![u.this, u.sky, 0.25f32]);
        let e = &u.e;
        assert_eq!(
            calls(e, FOG_SET_DISTANCES),
            vec![vec![u.fog, 100.0f32.to_bits(), 200.0f32.to_bits()]]
        );
        assert_eq!(calls(e, FOG_SET_POWER), vec![vec![u.fog, 2.0f32.to_bits()]]);
        assert_eq!(
            calls(e, CAMERA_SET_00507700),
            vec![vec![camera, 2.0f32.to_bits()]]
        );
        let set = calls(e, CAMERA_SET_VIEW_FRUSTUM);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][0], camera);
        // The far plane already equal: nothing is set.
        u.e.mem.set_f32(camera + 0xdc + 0x14, 8000.0);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_35d0, &args![u.this, u.sky, 0.25f32]);
        assert!(calls(&u.e, CAMERA_SET_VIEW_FRUSTUM).is_empty());
        // No camera: nothing either.
        u.e.mem.set_u32(holder + 0xac, 0);
        u.e.mem.set_f32(camera + 0xdc + 0x14, 1.0);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_35d0, &args![u.this, u.sky, 0.25f32]);
        assert!(calls(&u.e, CAMERA_FRUSTUM).is_empty());
    }

    #[test]
    fn fog_setters_store_their_fields() {
        let mut e = Engine::new();
        let fog = e.mem.alloc(0x64);
        let color = e.mem.alloc(12);
        for word in 0..3u32 {
            e.mem.set_u32(color + 4 * word, 0x10 + word);
        }
        e.call(0x0063_3bf0, &args![fog, color]);
        e.call(0x0063_3c60, &args![fog, color]);
        e.call(0x0063_3c20, &args![fog, 1.5f32, 2.5f32]);
        e.call(0x0063_3c40, &args![fog, 3.5f32]);
        for word in 0..3u32 {
            assert_eq!(e.mem.u32(fog + 0x20 + 4 * word), 0x10 + word);
            assert_eq!(e.mem.u32(fog + 0x54 + 4 * word), 0x10 + word);
        }
        assert_eq!(e.mem.f32(fog + 0x34), 1.5);
        assert_eq!(e.mem.f32(fog + 0x38), 2.5);
        assert_eq!(e.mem.f32(fog + 0x50), 3.5);
        // A signalling NaN is stored quiet.
        e.call(0x0063_3c40, &args![fog, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(fog + 0x50), 0x7fc0_0001);
    }
}
