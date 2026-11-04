//! `fallout shared/sky/clouds.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `Clouds` (a `SkyObject`, 0x5c bytes) keeps up to four cloud layer geometries from
//! `Meshes\Sky\Clouds.nif`, the layers' transition textures and colours. `Update` moves
//! the layers with the wind and blends their textures between the last and the current
//! weather. The unit also holds small accessors of other classes the compiler placed here
//! (a weather's cloud textures and speeds, the sky's flags, a `SkyShaderProperty`'s base
//! map and file name); each says whose it is.
//!
//! The code is compiled without optimisation: every `NiPointer` access is a call and the
//! translations make those calls in the code's order.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_sky::skyobject::{pointer_assign, pointer_get};

/// `Clouds`' vtable (`0104ec08`).
const CLOUDS_VTABLE: u32 = 0x0104_ec08;
/// `SkyObject::SkyObject` (`0063f830`), `::~SkyObject` (`0063f8d0`) and `::Initialize`
/// (`0063f940`).
const SKY_OBJECT_CONSTRUCT: u32 = 0x0063_f830;
const SKY_OBJECT_DESTRUCT: u32 = 0x0063_f8d0;
const SKY_OBJECT_INITIALIZE: u32 = 0x0063_f940;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The compiler's array helpers: `eh vector constructor iterator` (`00ec782f`, `stdcall`:
/// array, size, count, constructor, destructor), `vector constructor iterator` (`00401050`,
/// `stdcall`: array, size, count, constructor) and `eh vector destructor iterator`
/// (`00ec5fce`, `stdcall`: array, size, count, destructor).
const EH_VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const EH_VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The `NiPointer` element constructor and destructor the arrays use (`006694e0`,
/// `0045cec0`).
const NI_POINTER_CONSTRUCT_006694E0: u32 = 0x0066_94e0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `NiColor::NiColor(r, g, b)` (`00416870`, `thiscall`, three `float`s).
const COLOR_CONSTRUCT: u32 = 0x0041_6870;
/// The `NiColorA` constructor (`00414430`, `thiscall`, four `float`s).
const COLOR_A_CONSTRUCT: u32 = 0x0041_4430;
/// `BSStream` (constructor `0043cfd0`, destructor `0043d090`, `BSStream::Load` `00c3a8a0`;
/// 0x5d4 bytes) and `Model` (`Model::Model` `0043aaf0`, `::~Model` `0043ab70`, the root
/// object `0043b230`: its `NiPointer` at +0xc; 0x1c bytes).
const STREAM_CONSTRUCT: u32 = 0x0043_cfd0;
const STREAM_DESTRUCT: u32 = 0x0043_d090;
const STREAM_LOAD: u32 = 0x00c3_a8a0;
const STREAM_SIZE: u32 = 0x5d4;
const MODEL_CONSTRUCT: u32 = 0x0043_aaf0;
const MODEL_DESTRUCT: u32 = 0x0043_ab70;
const MODEL_ROOT: u32 = 0x0043_b230;
const MODEL_SIZE: u32 = 0x1c;
/// `"Meshes\\Sky\\Clouds.nif"`.
const CLOUDS_PATH: u32 = 0x0104_ec18;
/// A node's child count (`00453470`) and child (`0043b4a0`, one word: the index)
/// (`thiscall`).
const NODE_CHILD_COUNT: u32 = 0x0045_3470;
const NODE_CHILD: u32 = 0x0043_b4a0;
/// The base update `008d0600` (`thiscall`: the sky, the time; a bare `RET 8`).
const BASE_UPDATE_008D0600: u32 = 0x008d_0600;
/// `Sky` accessors (`thiscall`, no stack words): `pCurrentWeather` (`0044edb0`, `+0x10`),
/// `pLastWeather` (`00825c00`, `+0x14`), the mode (`00476c90`, `+0xf8`),
/// `fCurrentWeatherPct` (`006447d0`, `+0xf4`, in `ST0`) and `fWindSpeed` (`004536e0`,
/// `+0xcc`, in `ST0`).
const SKY_CURRENT_WEATHER: u32 = 0x0044_edb0;
const SKY_LAST_WEATHER: u32 = 0x0082_5c00;
const SKY_GET_MODE: u32 = 0x0047_6c90;
const SKY_WEATHER_PERCENT: u32 = 0x0064_47d0;
const SKY_WIND_SPEED: u32 = 0x0045_36e0;
/// A geometry's property state address (`0050d100`, `+0x9c`); on it `0043b230` answers the
/// `NiPointer` at +0xc (the shade property); a property's type (`00441110`, `+0x1c`).
const GEOMETRY_PROPERTY_STATE: u32 = 0x0050_d100;
const PROPERTY_TYPE: u32 = 0x0044_1110;
/// `NiAVObject::GetProperty` (`00a59d30`, `thiscall`, one word) and the cast `00653270`
/// (`cdecl`: the RTTI, the object; 0 for a null object).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const DYNAMIC_CAST: u32 = 0x0065_3270;
/// `SkyShaderProperty::ms_RTTI` (`012005b8`).
const SKY_SHADER_PROPERTY_RTTI: u32 = 0x0120_05b8;
/// `SkyShaderProperty::SetBlendTexture` (`00bae930`, one word) and the blend value setter
/// (`0051b8c0`, `+0x84`, one `float`); its base map (`009c8680`, the `NiPointer` at +0x70).
const SET_BLEND_TEXTURE: u32 = 0x00ba_e930;
const SET_BLEND_VALUE: u32 = 0x0051_b8c0;
const SKY_PROPERTY_BASE_MAP: u32 = 0x009c_8680;
/// The node visibility setter (`00450f90`, one byte: hidden).
const SET_HIDDEN: u32 = 0x0045_0f90;
/// The setting object (`011cca6c`) whose `float` value accessor (`00403e20`, `thiscall`,
/// a pointer to the value) gives the cloud speed scale.
const CLOUD_SPEED_SETTING: u32 = 0x011c_ca6c;
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// The four cloud layer offsets `006348b0` adds to (`011ff8b4`, `float`s).
const LAYER_OFFSETS: u32 = 0x011f_f8b4;
/// `1.0 / 255.0` (`double`, `010231e8`).
const ONE_OVER_255: u32 = 0x0102_31e8;
/// `BSStringT<char>`: constructor (`004037b0`), destructor (`004037d0`), set (`00438390`,
/// one word: the text), append (`00404820`, one word) (`thiscall`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const STRING_SET: u32 = 0x0043_8390;
const STRING_APPEND: u32 = 0x0040_4820;
/// A `TESTexture`'s name test (`0048cee0`), its vtable slot +0x18 (a text) and `00408da0`
/// (another text) (`thiscall`).
const TEXTURE_HAS_NAME: u32 = 0x0048_cee0;
const TEXTURE_PATH_00408DA0: u32 = 0x0040_8da0;
/// `0063fb70` (`cdecl`: the `NiPointer` to fill, the texture path).
const LOAD_TEXTURE_0063FB70: u32 = 0x0063_fb70;
/// `BSShaderManager::GetTexture` (`00b55840`, `cdecl`, five words) and the `NiPointer`
/// constructor from a raw pointer (`00633c90`).
const SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;
const NI_POINTER_FROM_RAW: u32 = 0x0063_3c90;
/// The `NiFixedString` constructor from text (`00438170`, one word) and destructor
/// (`004381b0`); its text (`0043b1b0`) (`thiscall`).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const FIXED_STRING_TEXT: u32 = 0x0043_b1b0;
/// The empty `NiFixedString` (`011f49e0`) `RemoveTextures` names the properties with.
const EMPTY_FIXED_STRING_OBJECT: u32 = 0x011f_49e0;

layout! {
    /// `Clouds` (Xbox PDB), 0x5c bytes (`Sky::Initialize` allocates 0x5c).
    pub struct Clouds: 0x5C {
        /// `SkyObject::spRoot` (Xbox PDB).
        0x04 spRoot: u32,
        /// `spClouds` (Xbox PDB): four `NiPointer<NiGeometry>`.
        0x08 spClouds: u32,
        /// `spTransTexture` (Xbox PDB): four `NiPointer<NiTexture>`.
        0x18 spTransTexture: u32,
        /// `pColors` (Xbox PDB): four `NiColor`s.
        0x28 pColors: f32,
        /// `usNumLayers` (Xbox PDB).
        0x58 usNumLayers: u16,
        /// `bForceUpdate` (Xbox PDB).
        0x5A bForceUpdate: bool,
    }
}

/// The address of `spClouds[layer]`.
fn cloud_slot(this: u32, layer: i32) -> u32 {
    this.wrapping_add((layer as u32).wrapping_mul(4))
        .wrapping_add(8)
}

/// The address of `spTransTexture[layer]`.
fn trans_slot(this: u32, layer: i32) -> u32 {
    this.wrapping_add((layer as u32).wrapping_mul(4))
        .wrapping_add(0x18)
}

/// `layer < GetNumLayers()` (the loops' signed test, the count read again each time).
fn below_layers(e: &mut Engine, this: u32, layer: i32) -> bool {
    layer < clouds_get_num_layers(e, Ptr::new(this)) as i32
}

// Translated from 00633cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds::Clouds` (engine map; `thiscall`, no stack words): `SkyObject::SkyObject`, the
/// vtable, the two `NiPointer` arrays constructed, the four colours constructed (`00633e00`),
/// no layers, `ZeroGeo`, `ClearTransTextures`, `bForceUpdate` clear. Answers `this`.
// Translation incomplete: 00633cc0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn clouds_clouds(e: &mut Engine, this: Ptr<Clouds>) -> Ptr<Clouds> {
    let base = this.addr();
    e.call(SKY_OBJECT_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, CLOUDS_VTABLE);
    for offset in [8u32, 0x18] {
        e.call(
            EH_VECTOR_CONSTRUCT,
            &args![
                base + offset,
                4u32,
                4u32,
                NI_POINTER_CONSTRUCT_006694E0,
                NI_POINTER_DESTRUCT
            ],
        );
    }
    e.call(
        VECTOR_CONSTRUCT,
        &args![base + 0x28, 0xcu32, 4u32, 0x0063_3e00u32],
    );
    e.mem.set_u16(base + 0x58, 0);
    clouds_zero_geo(e, this);
    clouds_clear_trans_textures(e, this);
    e.mem.set_u8(base + 0x5a, 0);
    this
}

// Translated from 00633d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds::ZeroGeo` (engine map; `thiscall`, no stack words): the four `spClouds`
/// assigned 0.
pub fn clouds_zero_geo(e: &mut Engine, this: Ptr<Clouds>) {
    for layer in 0..4 {
        pointer_assign(e, cloud_slot(this.addr(), layer), 0);
    }
}

// Translated from 00633dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds`' scalar deleting destructor (engine map): the destructor, then `operator delete`
/// when `flags & 1`. Answers `this`.
pub fn clouds_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Clouds>,
    flags: u32,
) -> Ptr<Clouds> {
    clouds_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00633e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the colour array's element constructor (`thiscall` on an
/// `NiColor`, no stack words): `NiColor::NiColor(0, 0, 0)`; answers what it answers.
pub fn fn_00633e00(e: &mut Engine, this: Ptr) -> u32 {
    let zero = 0.0f32.to_bits();
    e.call(COLOR_CONSTRUCT, &args![this, zero, zero, zero])
        .u32()
}

// Translated from 00633e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds::~Clouds` (engine map; `thiscall`, no stack words): the vtable, `ZeroGeo`,
/// `ClearTransTextures`, the transition textures' and then the geometries' `NiPointer`
/// arrays destroyed, then `SkyObject::~SkyObject`.
// Translation incomplete: 00633e30 - the C++ exception frame (FS:[0] chain, unwind states 2 to 0 for the arrays and the base) is not translated.
pub fn clouds_destructor(e: &mut Engine, this: Ptr<Clouds>) {
    let base = this.addr();
    e.mem.set_u32(base, CLOUDS_VTABLE);
    clouds_zero_geo(e, this);
    clouds_clear_trans_textures(e, this);
    for offset in [0x18u32, 8] {
        e.call(
            EH_VECTOR_DESTRUCT,
            &args![base + offset, 4u32, 4u32, NI_POINTER_DESTRUCT],
        );
    }
    e.call(SKY_OBJECT_DESTRUCT, &args![base]);
}

// Translated from 00633ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds::Initialize` (engine map; `thiscall`, one word: the parent node).
/// `SkyObject::Initialize`, then loads `Meshes\Sky\Clouds.nif` through a local `BSStream`;
/// when it loads: the model's root, the clouds' root attached to the parent again (vtable
/// slot +0xdc with 1), and every child of the model's root whose vtable slot +0x18 answers
/// non-zero is stored in `spClouds[child index]` and attached to the clouds' root; the
/// number of such children becomes the layer count (`SetNumLayers`, at most 4).
// Translation incomplete: 00633ed0 - the C++ exception frame (FS:[0] chain, unwind states for the stream and the model) and the /GS cookie check (00ec408c) are not translated.
pub fn clouds_initialize(e: &mut Engine, this: Ptr<Clouds>, parent: u32) {
    let base = this.addr();
    e.call(SKY_OBJECT_INITIALIZE, &args![base, parent]);
    let stream = e.mem.alloc(STREAM_SIZE);
    e.call(STREAM_CONSTRUCT, &args![stream]);
    let loaded = e.call(STREAM_LOAD, &args![stream, CLOUDS_PATH, 0u32]).u8();
    if loaded != 0 {
        let model = e.mem.alloc(MODEL_SIZE);
        e.call(
            MODEL_CONSTRUCT,
            &args![model, CLOUDS_PATH, stream, 0u32, 0u32],
        );
        let node = e.call(MODEL_ROOT, &args![model]).u32();
        let root = pointer_get(e, base + 4);
        e.vcall(parent, 0xdc, &args![root, 1u32]);
        let mut count = 0u32;
        let children = e.call(NODE_CHILD_COUNT, &args![node]).i32();
        let mut index = 0i32;
        while index < children {
            let child = e.call(NODE_CHILD, &args![node, index]).u32();
            if e.vcall(child, 0x18, &args![]).u32() != 0 {
                let child = e.call(NODE_CHILD, &args![node, index]).u32();
                pointer_assign(e, cloud_slot(base, index), child);
                let root = pointer_get(e, base + 4);
                let geometry = pointer_get(e, cloud_slot(base, index));
                e.vcall(root, 0xdc, &args![geometry, 1u32]);
                count = count.wrapping_add(1);
            }
            index += 1;
        }
        fn_006340e0(e, this, count as i32);
        e.call(MODEL_DESTRUCT, &args![model]);
        e.mem.free(model);
    }
    e.call(STREAM_DESTRUCT, &args![stream]);
    e.mem.free(stream);
}

// Translated from 006340e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the layer count setter by its body):
/// `usNumLayers` takes the count, at most 4 (a signed test).
pub fn fn_006340e0(e: &mut Engine, this: Ptr<Clouds>, count: i32) {
    let count = if count > 4 { 4 } else { count };
    e.set(this, Clouds::usNumLayers, count as u16);
}

/// The texture path a weather's cloud texture names: its vtable slot +0x18 text set into
/// the local string, `00408da0`'s text appended.
fn texture_path(e: &mut Engine, string: u32, texture: u32) {
    let text = e.vcall(texture, 0x18, &args![]).u32();
    e.call(STRING_SET, &args![string, text]);
    let more = e.call(TEXTURE_PATH_00408DA0, &args![texture]).u32();
    e.call(STRING_APPEND, &args![string, more]);
}

// Translated from 00634110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds::Update` (engine map; `thiscall`, two words: the sky, the time). The base update
/// `008d0600`; nothing without a current weather or while a layer has no geometry. In sky
/// modes 3 and 2, for each layer: a sky shader property (shade property type 0xd) takes the
/// layer's colour (alpha 1) as its vertex colour; the layer's speed is the current weather's
/// (`006347c0` with the cloud speed setting and 0), blended with the last weather's by
/// `fCurrentWeatherPct` when there is one, and `006348b0` adds wind speed * time * speed to
/// the layer's offset. When the sky's flags 1 or 2 are set or `bForceUpdate` (which is
/// cleared): with a last weather each layer's transition texture is loaded (`0063fb70`) from
/// the current weather's cloud texture path, or cleared without one; without a last weather,
/// in modes 3 and 2, each layer's property gets the texture (`BSShaderManager::GetTexture`)
/// as its base map, the path as its file name and blend 0. Then for each layer with a
/// transition texture: the sky shader property blends to it by the weather percentage, and
/// without a last weather it becomes the base map (the blend cleared); the layer is hidden
/// unless it has a sky shader property with a base map.
// Translation incomplete: 00634110 - the C++ exception frame (FS:[0] chain, unwind states for the local strings and pointers) is not translated.
pub fn clouds_update(e: &mut Engine, this: Ptr<Clouds>, sky: u32, time: f32) {
    let base = this.addr();
    e.call(
        BASE_UPDATE_008D0600,
        &args![base, sky, x87_quiet_bits(time.to_bits())],
    );
    let current = e.call(SKY_CURRENT_WEATHER, &args![sky]).u32();
    let last = e.call(SKY_LAST_WEATHER, &args![sky]).u32();
    if current == 0 {
        return;
    }
    let mut layer = 0i32;
    while below_layers(e, base, layer) {
        if pointer_get(e, cloud_slot(base, layer)) == 0 {
            return;
        }
        layer += 1;
    }
    let in_mode = |e: &mut Engine| {
        e.call(SKY_GET_MODE, &args![sky]).u32() == 3 || e.call(SKY_GET_MODE, &args![sky]).u32() == 2
    };
    if in_mode(e) {
        let mut layer = 0i32;
        while below_layers(e, base, layer) {
            update_layer_motion(e, base, sky, time, current, last, layer);
            layer += 1;
        }
    }
    if fn_00634870(e, sky) != 0 || e.mem.u8(base + 0x5a) != 0 {
        e.mem.set_u8(base + 0x5a, 0);
        if last != 0 {
            let mut layer = 0i32;
            while below_layers(e, base, layer) {
                let string = e.mem.alloc(8);
                e.call(STRING_CONSTRUCT, &args![string]);
                let geometry = pointer_get(e, cloud_slot(base, layer));
                e.call(GET_PROPERTY, &args![geometry, 3u32]);
                let texture = fn_006347a0(e, current, layer);
                if texture != 0 && e.call(TEXTURE_HAS_NAME, &args![texture]).u32() != 0 {
                    texture_path(e, string, texture);
                    fn_00634890(e, sky);
                    let path = pointer_get(e, string);
                    e.call(LOAD_TEXTURE_0063FB70, &args![trans_slot(base, layer), path]);
                } else {
                    pointer_assign(e, trans_slot(base, layer), 0);
                }
                e.call(STRING_DESTRUCT, &args![string]);
                e.mem.free(string);
                layer += 1;
            }
        } else if in_mode(e) {
            let mut layer = 0i32;
            while below_layers(e, base, layer) {
                set_layer_base_map(e, base, current, layer);
                layer += 1;
            }
        }
    }
    let mut layer = 0i32;
    while below_layers(e, base, layer) {
        let geometry = pointer_get(e, cloud_slot(base, layer));
        let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
        let sky_property = e
            .call(DYNAMIC_CAST, &args![SKY_SHADER_PROPERTY_RTTI, property])
            .u32();
        if pointer_get(e, trans_slot(base, layer)) != 0 {
            if sky_property != 0 {
                let texture = pointer_get(e, trans_slot(base, layer));
                e.call(SET_BLEND_TEXTURE, &args![sky_property, texture]);
                let percent = e.call(SKY_WEATHER_PERCENT, &args![sky]).f64();
                e.call(SET_BLEND_VALUE, &args![sky_property, x87_store32(percent)]);
            }
            if last == 0 {
                let texture = pointer_get(e, trans_slot(base, layer));
                fn_006348e0(e, Ptr::new(sky_property), texture);
                pointer_assign(e, trans_slot(base, layer), 0);
                e.call(SET_BLEND_TEXTURE, &args![sky_property, 0u32]);
                e.call(SET_BLEND_VALUE, &args![sky_property, 0.0f32.to_bits()]);
            }
        }
        let shown =
            sky_property != 0 && e.call(SKY_PROPERTY_BASE_MAP, &args![sky_property]).u32() != 0;
        let geometry = pointer_get(e, cloud_slot(base, layer));
        e.call(SET_HIDDEN, &args![geometry, (!shown) as u32]);
        layer += 1;
    }
}

/// One layer of `Update`'s colour and motion loop (modes 3 and 2).
fn update_layer_motion(
    e: &mut Engine,
    base: u32,
    sky: u32,
    time: f32,
    current: u32,
    last: u32,
    layer: i32,
) {
    let shade = |e: &mut Engine| {
        let geometry = pointer_get(e, cloud_slot(base, layer));
        let state = e.call(GEOMETRY_PROPERTY_STATE, &args![geometry]).u32();
        e.call(MODEL_ROOT, &args![state]).u32()
    };
    let is_sky = if shade(e) == 0 {
        false
    } else {
        let property = shade(e);
        e.call(PROPERTY_TYPE, &args![property]).u32() == 0xd
    };
    let property = if is_sky { shade(e) } else { 0 };
    if property != 0 {
        let color = clouds_get_color(e, Ptr::new(base), layer);
        e.with_stack(0x1c, |e, local| {
            let local = local.addr();
            for word in 0..3 {
                let value = e.mem.u32(color + 4 * word);
                e.mem.set_u32(local + 4 * word, value);
            }
            let blue = x87_quiet_bits(e.mem.u32(local + 8));
            let green = x87_quiet_bits(e.mem.u32(local + 4));
            let red = x87_quiet_bits(e.mem.u32(local));
            let made = e
                .call(
                    COLOR_A_CONSTRUCT,
                    &args![local + 0xc, red, green, blue, 1.0f32.to_bits()],
                )
                .u32();
            for word in 0..4 {
                let value = e.mem.u32(made + 4 * word);
                e.mem.set_u32(property + 0x60 + 4 * word, value);
            }
        });
    }
    let scale = e
        .call(SETTING_FLOAT_VALUE, &args![CLOUD_SPEED_SETTING])
        .u32();
    let scale_bits = x87_quiet_bits(e.mem.u32(scale));
    let mut speed =
        fn_006347c0(e, Ptr::new(current), layer, f32::from_bits(scale_bits), 0.0).to_bits();
    if last != 0 {
        let percent = e.call(SKY_WEATHER_PERCENT, &args![sky]).f64();
        speed = x87_store32(x87_mul(percent, x87_m32(speed)));
        let percent = e.call(SKY_WEATHER_PERCENT, &args![sky]).f64();
        let rest = x87_sub(1.0, percent);
        // FSTP double: the rest is kept as a double.
        let scale = e
            .call(SETTING_FLOAT_VALUE, &args![CLOUD_SPEED_SETTING])
            .u32();
        let scale_bits = x87_quiet_bits(e.mem.u32(scale));
        let last_speed = fn_006347c0(e, Ptr::new(last), layer, f32::from_bits(scale_bits), 0.0);
        speed = x87_store32(x87_add(
            x87_mul(x87_load32(last_speed.to_bits()), rest),
            x87_m32(speed),
        ));
    }
    let wind = e.call(SKY_WIND_SPEED, &args![sky]).f64();
    let offset = x87_store32(x87_mul(
        x87_mul(wind, x87_m32(time.to_bits())),
        x87_m32(speed),
    ));
    fn_006348b0(e, layer as u16, f32::from_bits(x87_quiet_bits(offset)));
}

/// One layer of `Update`'s base map loop (no last weather, modes 3 and 2).
fn set_layer_base_map(e: &mut Engine, base: u32, current: u32, layer: i32) {
    let string = e.mem.alloc(8);
    e.call(STRING_CONSTRUCT, &args![string]);
    let geometry = pointer_get(e, cloud_slot(base, layer));
    let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
    let texture = fn_006347a0(e, current, layer);
    if texture != 0 && e.call(TEXTURE_HAS_NAME, &args![texture]).u32() != 0 {
        texture_path(e, string, texture);
        let pointer = e.mem.alloc(4);
        e.call(NI_POINTER_FROM_RAW, &args![pointer, 0u32]);
        let path = pointer_get(e, string);
        e.call(
            SHADER_MANAGER_GET_TEXTURE,
            &args![path, 1u32, pointer, 1u32, 0u32],
        );
        let loaded = pointer_get(e, pointer);
        if loaded != 0 {
            fn_006348e0(e, Ptr::new(property), loaded);
            let fixed = e.mem.alloc(4);
            let path = pointer_get(e, string);
            let name = e.call(FIXED_STRING_CONSTRUCT, &args![fixed, path]).u32();
            fn_00634900(e, Ptr::new(property), name);
            e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
            e.mem.free(fixed);
            e.call(SET_BLEND_VALUE, &args![property, 0.0f32.to_bits()]);
        }
        e.call(NI_POINTER_DESTRUCT, &args![pointer]);
        e.mem.free(pointer);
    }
    e.call(STRING_DESTRUCT, &args![string]);
    e.mem.free(string);
}

// Translated from 006347a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `TESWeather` by its use in `Clouds::Update`,
/// one word): the address of cloud texture `layer` (`+0x34`, 12 bytes each).
pub fn fn_006347a0(_e: &mut Engine, this: u32, layer: i32) -> u32 {
    this.wrapping_add((layer as u32).wrapping_mul(0xc))
        .wrapping_add(0x34)
}

// Translated from 006347c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `TESWeather` by its use, three words: the
/// layer, `high`, `low`): the layer's speed byte (`+0x64 + layer`, layer 0 when it is not
/// below the count at `+0x368`; 0x33 when that count is not positive) times `1/255` scales
/// `high - low` above `low`; the result is rounded to a `float`.
pub fn fn_006347c0(e: &mut Engine, this: Ptr, layer: i32, high: f32, low: f32) -> f32 {
    let weather = this.addr();
    let mut speed = 0x33u8;
    let count = e.mem.i32(weather + 0x368);
    if count > 0 {
        let layer = if layer >= count { 0 } else { layer };
        speed = e
            .mem
            .u8(weather.wrapping_add(layer as u32).wrapping_add(0x64));
    }
    let factor = x87_mul(f64::from(speed), e.global::<f64>(ONE_OVER_255));
    let range = x87_sub(x87_m32(high.to_bits()), x87_m32(low.to_bits()));
    let value = x87_add(x87_mul(factor, range), x87_m32(low.to_bits()));
    f32::from_bits(x87_store32(value))
}

// Translated from 00634830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; `CLOUDS_LAYER_COUNT` in
/// `sky.rs`): `usNumLayers`.
pub fn clouds_get_num_layers(e: &mut Engine, this: Ptr<Clouds>) -> u32 {
    e.get(this, Clouds::usNumLayers) as u32
}

// Translated from 00634850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): the address of `pColors[layer]`.
pub fn clouds_get_color(_e: &mut Engine, this: Ptr<Clouds>, layer: i32) -> u32 {
    this.addr()
        .wrapping_add((layer as u32).wrapping_mul(0xc))
        .wrapping_add(0x28)
}

// Translated from 00634870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `Sky`, no stack words): 1 when flag 1 or 2
/// of `uiFlags` (`+0x118`) is set, else 0.
pub fn fn_00634870(e: &mut Engine, sky: u32) -> u32 {
    (e.mem.u32(sky + 0x118) & 3 != 0) as u32
}

// Translated from 00634890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `Sky`, no stack words): the address of its
/// `+0x114`.
pub fn fn_00634890(_e: &mut Engine, sky: u32) -> u32 {
    sky.wrapping_add(0x114)
}

// Translated from 006348b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: the layer (`u16`), a `float`): for a layer
/// below 4, adds the value to its offset (`011ff8b4 + 4 * layer`).
pub fn fn_006348b0(e: &mut Engine, layer: u16, delta: f32) {
    if (layer as u32) < 4 {
        let address = LAYER_OFFSETS + 4 * layer as u32;
        let sum = x87_add(x87_m32(e.global::<u32>(address)), x87_m32(delta.to_bits()));
        e.set_global(address, x87_store32(sum));
    }
}

// Translated from 006348e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `SkyShaderProperty` by its use, one word):
/// `spBaseMap` (`+0x70`) assigned the texture.
pub fn fn_006348e0(e: &mut Engine, this: Ptr, texture: u32) {
    pointer_assign(e, this.addr().wrapping_add(0x70), texture);
}

// Translated from 00634900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `SkyShaderProperty` by its use, one word: an
/// `NiFixedString`): `TextureFilename` (`+0x74`) set from the fixed string's text.
pub fn fn_00634900(e: &mut Engine, this: Ptr, name: u32) {
    let text = e.call(FIXED_STRING_TEXT, &args![name]).u32();
    e.call(STRING_SET, &args![this.addr().wrapping_add(0x74), text]);
}

// Translated from 00634930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds::ClearTransTextures` (engine map; `thiscall`, no stack words): for each layer,
/// the transition texture assigned 0 and, with a geometry whose shade property (property 3)
/// is a `SkyShaderProperty`, its blend texture cleared and blend value 0.
pub fn clouds_clear_trans_textures(e: &mut Engine, this: Ptr<Clouds>) {
    let base = this.addr();
    let mut layer = 0i32;
    while below_layers(e, base, layer) {
        pointer_assign(e, trans_slot(base, layer), 0);
        if pointer_get(e, cloud_slot(base, layer)) != 0 {
            let geometry = pointer_get(e, cloud_slot(base, layer));
            let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
            let sky_property = e
                .call(DYNAMIC_CAST, &args![SKY_SHADER_PROPERTY_RTTI, property])
                .u32();
            if sky_property != 0 {
                e.call(SET_BLEND_TEXTURE, &args![sky_property, 0u32]);
                e.call(SET_BLEND_VALUE, &args![sky_property, 0.0f32.to_bits()]);
            }
        }
        layer += 1;
    }
}

// Translated from 006349e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Clouds::RemoveTextures` (engine map; `thiscall`, no stack words): for each layer with a
/// geometry whose shade property is a `SkyShaderProperty`, its base map assigned 0 and its
/// file name set from the empty fixed string (`011f49e0`).
pub fn clouds_remove_textures(e: &mut Engine, this: Ptr<Clouds>) {
    let base = this.addr();
    let mut layer = 0i32;
    while below_layers(e, base, layer) {
        if pointer_get(e, cloud_slot(base, layer)) != 0 {
            let geometry = pointer_get(e, cloud_slot(base, layer));
            let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
            let sky_property = e
                .call(DYNAMIC_CAST, &args![SKY_SHADER_PROPERTY_RTTI, property])
                .u32();
            if sky_property != 0 {
                fn_006348e0(e, Ptr::new(sky_property), 0);
                fn_00634900(e, Ptr::new(sky_property), EMPTY_FIXED_STRING_OBJECT);
            }
        }
        layer += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_3cc0, clouds_clouds(Ptr<Clouds>) -> Ptr<Clouds>),
        entry!(0x0063_3d90, clouds_zero_geo(Ptr<Clouds>)),
        entry!(
            0x0063_3dd0,
            clouds_scalar_deleting_destructor(Ptr<Clouds>, u32) -> Ptr<Clouds>
        ),
        entry!(0x0063_3e00, fn_00633e00(Ptr) -> u32),
        entry!(0x0063_3e30, clouds_destructor(Ptr<Clouds>)),
        entry!(0x0063_3ed0, clouds_initialize(Ptr<Clouds>, u32)),
        entry!(0x0063_40e0, fn_006340e0(Ptr<Clouds>, i32)),
        entry!(0x0063_4110, clouds_update(Ptr<Clouds>, u32, f32)),
        entry!(0x0063_47a0, fn_006347a0(u32, i32) -> u32),
        entry!(0x0063_47c0, fn_006347c0(Ptr, i32, f32, f32) -> f32),
        entry!(0x0063_4830, clouds_get_num_layers(Ptr<Clouds>) -> u32),
        entry!(0x0063_4850, clouds_get_color(Ptr<Clouds>, i32) -> u32),
        entry!(0x0063_4870, fn_00634870(u32) -> u32),
        entry!(0x0063_4890, fn_00634890(u32) -> u32),
        entry!(0x0063_48b0, fn_006348b0(u16, f32)),
        entry!(0x0063_48e0, fn_006348e0(Ptr, u32)),
        entry!(0x0063_4900, fn_00634900(Ptr, u32)),
        entry!(0x0063_4930, clouds_clear_trans_textures(Ptr<Clouds>)),
        entry!(0x0063_49e0, clouds_remove_textures(Ptr<Clouds>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::fallout_shared_sky::skyobject::test_support::*;
    use crate::units::fallout_shared_sky::skyobject::NI_POINTER_ASSIGN;

    /// A geometry whose property 3 and property state shade property (`+0x9c + 0xc`) is
    /// `property`.
    fn geometry(e: &mut Engine, property: u32) -> u32 {
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xa8, property);
        geometry
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        e.register(GET_PROPERTY, |e, a| answer(e.mem.u32(a[0] + 0xa8)));
        // Every property with type word 0xd is a sky shader property.
        e.register(DYNAMIC_CAST, |e, a| {
            if a[1] != 0 && e.mem.u32(a[1] + 0x1c) == 0xd {
                answer(a[1])
            } else {
                answer(0)
            }
        });
        quiet(
            &mut e,
            &[SET_BLEND_TEXTURE, SET_BLEND_VALUE, SET_HIDDEN, STRING_SET],
        );
        e
    }

    fn sky_property(e: &mut Engine) -> u32 {
        let property = e.mem.alloc(0x90);
        e.mem.set_u32(property + 0x1c, 0xd);
        property
    }

    #[test]
    fn constructor_builds_the_arrays_and_clears_everything() {
        let mut e = rig();
        quiet(
            &mut e,
            &[SKY_OBJECT_CONSTRUCT, EH_VECTOR_CONSTRUCT, VECTOR_CONSTRUCT],
        );
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 0x58, 0x0101_0003);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_3cc0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CLOUDS_VTABLE);
        assert_eq!(
            calls(&e, EH_VECTOR_CONSTRUCT),
            vec![
                vec![
                    this + 8,
                    4,
                    4,
                    NI_POINTER_CONSTRUCT_006694E0,
                    NI_POINTER_DESTRUCT
                ],
                vec![
                    this + 0x18,
                    4,
                    4,
                    NI_POINTER_CONSTRUCT_006694E0,
                    NI_POINTER_DESTRUCT
                ]
            ]
        );
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![this + 0x28, 0xc, 4, 0x0063_3e00]]
        );
        assert_eq!(e.mem.u16(this + 0x58), 0);
        assert_eq!(e.mem.u8(this + 0x5a), 0);
        // ZeroGeo clears the four geometries; no layers, so nothing else.
        assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 4);
    }

    #[test]
    fn destructor_and_scalar_deleting_destructor() {
        let mut e = rig();
        quiet(
            &mut e,
            &[EH_VECTOR_DESTRUCT, SKY_OBJECT_DESTRUCT, OPERATOR_DELETE],
        );
        let this = e.mem.alloc(0x5c);
        e.mem.set_u16(this + 0x58, 1);
        e.mem.set_u32(this + 0x18, 0x0123);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_3dd0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), CLOUDS_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(
            calls(&e, EH_VECTOR_DESTRUCT),
            vec![
                vec![this + 0x18, 4, 4, NI_POINTER_DESTRUCT],
                vec![this + 8, 4, 4, NI_POINTER_DESTRUCT]
            ]
        );
        assert_eq!(calls(&e, SKY_OBJECT_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.call(0x0063_3dd0, &args![this, 0u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn color_element_constructor_zeroes_the_colour() {
        let mut e = Engine::new();
        e.register(COLOR_CONSTRUCT, |e, a| {
            for word in 0..3u32 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            answer(a[0])
        });
        let color = e.mem.alloc(12);
        e.mem.set_u32(color, 0xffff_ffff);
        assert_eq!(e.call(0x0063_3e00, &args![color]).u32(), color);
        assert_eq!(e.mem.u32(color), 0);
    }

    #[test]
    fn initialize_takes_the_geometry_children_as_layers() {
        let mut e = rig();
        quiet(
            &mut e,
            &[
                SKY_OBJECT_INITIALIZE,
                STREAM_CONSTRUCT,
                STREAM_DESTRUCT,
                MODEL_CONSTRUCT,
                MODEL_DESTRUCT,
                0x0000_7770,
            ],
        );
        e.register(STREAM_LOAD, |_, _| answer(1));
        e.register(MODEL_ROOT, |_, _| answer(0x0000_0b00));
        // Six children: 0, 2, 3, 4 and 5 are geometries.
        e.register(NODE_CHILD_COUNT, |_, _| answer(6));
        let mut children = vec![];
        for index in 0..6u32 {
            let is_geometry = if index == 1 { 0x0000_7790 } else { 0x0000_7798 };
            children.push(object_with(&mut e, 8, &[(0x18, is_geometry)]));
        }
        e.register(0x0000_7790, |_, _| answer(0));
        e.register(0x0000_7798, |_, _| answer(1));
        let list = children.clone();
        e.register_double(NODE_CHILD, move |_, a| answer(list[a[1] as usize]));
        let parent = object_with(&mut e, 4, &[(0xdc, 0x0000_7770)]);
        let root = object_with(&mut e, 4, &[(0xdc, 0x0000_7770)]);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 4, root);
        e.call_log = Some(vec![]);
        e.call(0x0063_3ed0, &args![this, parent]);
        assert_eq!(calls(&e, SKY_OBJECT_INITIALIZE), vec![vec![this, parent]]);
        let attached = calls(&e, 0x0000_7770);
        assert_eq!(attached[0], vec![parent, root, 1]);
        assert_eq!(attached.len(), 6);
        assert_eq!(e.mem.u32(this + 8), children[0]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), children[2]);
        // Index 5 lands past the four geometry slots, in the second transition texture.
        assert_eq!(e.mem.u32(this + 0x1c), children[5]);
        // Five geometries, at most four layers.
        assert_eq!(e.mem.u16(this + 0x58), 4);
        // Not loaded: no model.
        e.register(STREAM_LOAD, |_, _| answer(0));
        e.call_log = Some(vec![]);
        e.call(0x0063_3ed0, &args![this, parent]);
        assert!(calls(&e, MODEL_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, STREAM_DESTRUCT).len(), 1);
    }

    #[test]
    fn layer_count_and_address_accessors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x5c);
        e.call(0x0063_40e0, &args![this, 3i32]);
        assert_eq!(e.call(0x0063_4830, &args![this]).u32(), 3);
        e.call(0x0063_40e0, &args![this, 9i32]);
        assert_eq!(e.call(0x0063_4830, &args![this]).u32(), 4);
        e.call(0x0063_40e0, &args![this, -1i32]);
        assert_eq!(e.call(0x0063_4830, &args![this]).u32(), 0xffff);
        assert_eq!(
            e.call(0x0063_4850, &args![this, 2i32]).u32(),
            this + 0x28 + 0x18
        );
        assert_eq!(
            e.call(0x0063_47a0, &args![0x1000u32, 3i32]).u32(),
            0x1000 + 0x34 + 0x24
        );
        assert_eq!(e.call(0x0063_4890, &args![0x2000u32]).u32(), 0x2114);
    }

    #[test]
    fn sky_flag_test() {
        let mut e = Engine::new();
        let sky = e.mem.alloc(0x138);
        for (flags, expected) in [(0u32, 0u32), (1, 1), (2, 1), (4, 0), (7, 1)] {
            e.mem.set_u32(sky + 0x118, flags);
            assert_eq!(e.call(0x0063_4870, &args![sky]).u32(), expected);
        }
    }

    #[test]
    fn weather_cloud_speed_scales_the_range() {
        let mut e = Engine::new();
        e.map(ONE_OVER_255, 8);
        e.set_global(ONE_OVER_255, 1.0f64 / 255.0);
        let weather = e.mem.alloc(0x370);
        // No count: the default byte 0x33.
        let value = e
            .call(0x0063_47c0, &args![weather, 0i32, 10.0f32, 0.0f32])
            .f32();
        assert_eq!(value, (51.0f64 / 255.0 * 10.0) as f32);
        e.mem.set_i32(weather + 0x368, 2);
        e.mem.set_u8(weather + 0x64, 255);
        e.mem.set_u8(weather + 0x65, 0);
        assert_eq!(
            e.call(0x0063_47c0, &args![weather, 0i32, 10.0f32, 2.0f32])
                .f32(),
            10.0
        );
        assert_eq!(
            e.call(0x0063_47c0, &args![weather, 1i32, 10.0f32, 2.0f32])
                .f32(),
            2.0
        );
        // A layer past the count reads layer 0.
        assert_eq!(
            e.call(0x0063_47c0, &args![weather, 5i32, 10.0f32, 2.0f32])
                .f32(),
            10.0
        );
    }

    #[test]
    fn layer_offsets_add_below_four() {
        let mut e = Engine::new();
        e.map(LAYER_OFFSETS, 16);
        e.set_global(LAYER_OFFSETS + 4, 1.5f32);
        e.call(0x0063_48b0, &args![1u16, 2.0f32]);
        assert_eq!(e.global::<f32>(LAYER_OFFSETS + 4), 3.5);
        e.call(0x0063_48b0, &args![4u16, 2.0f32]);
        assert_eq!(e.global::<f32>(LAYER_OFFSETS + 4), 3.5);
    }

    #[test]
    fn sky_property_base_map_and_name_setters() {
        let mut e = rig();
        e.register(FIXED_STRING_TEXT, |e, a| answer(e.mem.u32(a[0])));
        let property = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        e.call(0x0063_48e0, &args![property, 0x4444u32]);
        assert_eq!(e.mem.u32(property + 0x70), 0x4444);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x5555);
        e.call(0x0063_4900, &args![property, name]);
        assert_eq!(calls(&e, STRING_SET), vec![vec![property + 0x74, 0x5555]]);
    }

    #[test]
    fn clear_and_remove_textures_touch_only_sky_properties() {
        let mut e = rig();
        e.register(FIXED_STRING_TEXT, |_, _| answer(0x6666));
        let sky = sky_property(&mut e);
        let other = e.mem.alloc(0x90);
        let this = e.mem.alloc(0x5c);
        e.mem.set_u16(this + 0x58, 3);
        let first = geometry(&mut e, sky);
        let second = geometry(&mut e, other);
        e.mem.set_u32(this + 8, first);
        e.mem.set_u32(this + 0xc, second);
        for layer in 0..3u32 {
            e.mem.set_u32(this + 0x18 + 4 * layer, 0x100 + layer);
        }
        e.mem.set_u32(sky + 0x70, 0x777);
        e.call_log = Some(vec![]);
        e.call(0x0063_4930, &args![this]);
        for layer in 0..3u32 {
            assert_eq!(e.mem.u32(this + 0x18 + 4 * layer), 0);
        }
        assert_eq!(calls(&e, SET_BLEND_TEXTURE), vec![vec![sky, 0]]);
        assert_eq!(calls(&e, SET_BLEND_VALUE), vec![vec![sky, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x0063_49e0, &args![this]);
        assert_eq!(e.mem.u32(sky + 0x70), 0);
        assert_eq!(
            calls(&e, FIXED_STRING_TEXT),
            vec![vec![EMPTY_FIXED_STRING_OBJECT]]
        );
        assert_eq!(calls(&e, STRING_SET), vec![vec![sky + 0x74, 0x6666]]);
    }

    struct Update {
        e: Engine,
        this: u32,
        sky: u32,
        property: u32,
        geometry: u32,
    }

    /// One layer whose geometry has a sky shader property; the sky in `mode`.
    fn update_rig(mode: u32, last: bool) -> Update {
        let mut e = rig();
        quiet(&mut e, &[BASE_UPDATE_008D0600]);
        let sky = e.mem.alloc(0x138);
        let current = e.mem.alloc(0x370);
        let previous = e.mem.alloc(0x370);
        e.mem.set_u32(sky + 0x10, current);
        if last {
            e.mem.set_u32(sky + 0x14, previous);
        }
        e.mem.set_u32(sky + 0xf8, mode);
        e.mem.set_f32(sky + 0xf4, 0.25);
        e.mem.set_f32(sky + 0xcc, 2.0);
        for (address, offset) in [
            (SKY_CURRENT_WEATHER, 0x10u32),
            (SKY_LAST_WEATHER, 0x14),
            (SKY_GET_MODE, 0xf8),
        ] {
            e.register_double(address, move |e, a| answer(e.mem.u32(a[0] + offset)));
        }
        e.register(SKY_WEATHER_PERCENT, |e, a| {
            answer_float(e.mem.f32(a[0] + 0xf4))
        });
        e.register(SKY_WIND_SPEED, |e, a| answer_float(e.mem.f32(a[0] + 0xcc)));
        e.register(GEOMETRY_PROPERTY_STATE, |_, a| answer(a[0] + 0x9c));
        e.register(MODEL_ROOT, |e, a| answer(e.mem.u32(a[0] + 0xc)));
        e.register(PROPERTY_TYPE, |e, a| answer(e.mem.u32(a[0] + 0x1c)));
        e.register(COLOR_A_CONSTRUCT, |e, a| {
            for word in 0..4u32 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            answer(a[0])
        });
        let setting = e.mem.alloc(8);
        e.mem.set_f32(setting + 4, 10.0);
        e.register_double(SETTING_FLOAT_VALUE, move |_, _| answer(setting + 4));
        e.map(ONE_OVER_255, 8);
        e.set_global(ONE_OVER_255, 1.0f64 / 255.0);
        e.map(LAYER_OFFSETS, 16);
        e.register(SKY_PROPERTY_BASE_MAP, |e, a| answer(e.mem.u32(a[0] + 0x70)));
        let property = sky_property(&mut e);
        let geometry = geometry(&mut e, property);
        let this = e.mem.alloc(0x5c);
        e.mem.set_u16(this + 0x58, 1);
        e.mem.set_u32(this + 8, geometry);
        e.mem.set_f32(this + 0x28, 0.5);
        e.mem.set_f32(this + 0x2c, 0.25);
        e.mem.set_f32(this + 0x30, 0.125);
        Update {
            e,
            this,
            sky,
            property,
            geometry,
        }
    }

    /// A texture in the current weather's first cloud texture slot whose vtable slot +0x18
    /// answers 0x0a00 and whose `00408da0` answers 0x0a10.
    fn named_texture(u: &mut Update) {
        quiet(
            &mut u.e,
            &[STRING_CONSTRUCT, STRING_DESTRUCT, STRING_APPEND],
        );
        u.e.register(TEXTURE_HAS_NAME, |_, _| answer(1));
        u.e.register(TEXTURE_PATH_00408DA0, |_, _| answer(0x0000_0a10));
        u.e.register(0x0000_7a18, |_, _| answer(0x0000_0a00));
        let current = u.e.mem.u32(u.sky + 0x10);
        let texture_vtable = u.e.mem.alloc(0x20);
        u.e.mem.set_u32(texture_vtable + 0x18, 0x0000_7a18);
        u.e.mem.set_u32(current + 0x34, texture_vtable);
    }

    #[test]
    fn update_moves_and_colours_the_layers_in_modes_3_and_2() {
        let mut u = update_rig(3, false);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_4110, &args![u.this, u.sky, 0.5f32]);
        let e = &u.e;
        assert_eq!(e.mem.f32(u.property + 0x60), 0.5);
        assert_eq!(e.mem.f32(u.property + 0x68), 0.125);
        assert_eq!(e.mem.f32(u.property + 0x6c), 1.0);
        // Speed 0x33/255 * 10; the offset grows by wind 2 * time 0.5 * speed.
        let speed = (51.0f64 / 255.0 * 10.0) as f32;
        assert_eq!(e.global::<f32>(LAYER_OFFSETS), speed);
        // No base map: the layer is hidden.
        assert_eq!(calls(e, SET_HIDDEN), vec![vec![u.geometry, 1]]);

        // With a last weather the speeds blend by the weather percentage.
        let mut u = update_rig(2, true);
        let current = u.e.mem.u32(u.sky + 0x10);
        u.e.mem.set_i32(current + 0x368, 1);
        u.e.mem.set_u8(current + 0x64, 255);
        let last = u.e.mem.u32(u.sky + 0x14);
        u.e.mem.set_i32(last + 0x368, 1);
        u.e.mem.set_u8(last + 0x64, 51);
        u.e.mem.set_u32(u.property + 0x70, 1);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_4110, &args![u.this, u.sky, 0.5f32]);
        // 0.25 * 10 + 0.75 * 2 = 4; the offset grows by 2 * 0.5 * 4.
        assert_eq!(u.e.global::<f32>(LAYER_OFFSETS), 4.0);
        assert_eq!(calls(&u.e, SET_HIDDEN), vec![vec![u.geometry, 0]]);
    }

    #[test]
    fn update_stops_without_a_weather_or_a_geometry() {
        let mut u = update_rig(3, false);
        u.e.mem.set_u32(u.sky + 0x10, 0);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_4110, &args![u.this, u.sky, 0.5f32]);
        assert!(calls(&u.e, SET_HIDDEN).is_empty());
        let mut u = update_rig(3, false);
        u.e.mem.set_u32(u.this + 8, 0);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_4110, &args![u.this, u.sky, 0.5f32]);
        assert!(calls(&u.e, SKY_GET_MODE).is_empty());
    }

    #[test]
    fn forced_update_with_a_last_weather_loads_the_transition_texture() {
        let mut u = update_rig(1, true);
        named_texture(&mut u);
        u.e.register(LOAD_TEXTURE_0063FB70, |e, a| {
            e.mem.set_u32(a[0], 0x0000_0e00);
            Ret::default()
        });
        u.e.mem.set_u8(u.this + 0x5a, 1);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_4110, &args![u.this, u.sky, 0.5f32]);
        let e = &u.e;
        assert_eq!(e.mem.u8(u.this + 0x5a), 0);
        let string = calls(e, STRING_CONSTRUCT)[0][0];
        assert_eq!(calls(e, STRING_SET), vec![vec![string, 0x0a00]]);
        assert_eq!(calls(e, STRING_APPEND), vec![vec![string, 0x0a10]]);
        let load = calls(e, LOAD_TEXTURE_0063FB70);
        assert_eq!(load[0][0], u.this + 0x18);
        // The transition texture blends in by the weather percentage.
        assert_eq!(calls(e, SET_BLEND_TEXTURE), vec![vec![u.property, 0x0e00]]);
        assert_eq!(
            calls(e, SET_BLEND_VALUE),
            vec![vec![u.property, 0.25f32.to_bits()]]
        );
    }

    #[test]
    fn update_without_a_last_weather_sets_the_base_map() {
        let mut u = update_rig(3, false);
        named_texture(&mut u);
        quiet(&mut u.e, &[NI_POINTER_DESTRUCT, FIXED_STRING_DESTRUCT]);
        u.e.register(SHADER_MANAGER_GET_TEXTURE, |e, a| {
            e.mem.set_u32(a[2], 0x0000_0f00);
            Ret::default()
        });
        u.e.register(FIXED_STRING_CONSTRUCT, |_, a| answer(a[0]));
        u.e.register(FIXED_STRING_TEXT, |_, _| answer(0x0000_0b10));
        u.e.mem.set_u32(u.sky + 0x118, 1);
        u.e.call_log = Some(vec![]);
        u.e.call(0x0063_4110, &args![u.this, u.sky, 0.5f32]);
        let e = &u.e;
        let get = calls(e, SHADER_MANAGER_GET_TEXTURE);
        assert_eq!(get.len(), 1);
        assert_eq!(get[0][1], 1);
        assert_eq!(get[0][3..], [1, 0]);
        assert_eq!(e.mem.u32(u.property + 0x70), 0x0f00);
        assert!(calls(e, STRING_SET).contains(&vec![u.property + 0x74, 0x0b10]));
        assert_eq!(calls(e, SET_BLEND_VALUE), vec![vec![u.property, 0]]);
        // With a base map now, the layer shows.
        assert_eq!(calls(e, SET_HIDDEN), vec![vec![u.geometry, 0]]);
    }
}
