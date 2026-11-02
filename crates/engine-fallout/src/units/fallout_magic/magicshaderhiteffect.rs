//! `fallout/magic/magicshaderhiteffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006b3eb0(this, 0, 0)`: the array-shaped constructor (unit
/// `pathsmootherpov`).
const ARRAY_CONSTRUCTOR: u32 = 0x006b_3eb0;
/// The vtable `00822600` stores.
const VTABLE: u32 = 0x0107_b84c;

// Translated from 00822600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0107b84c` and runs
/// `006b3eb0(this, 0, 0)` (an empty array: no reserved size, no elements).
/// Returns `this`. The class is not confirmed from the exe.
pub fn fn_00822600(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(ARRAY_CONSTRUCTOR, &args![this, 0u32, 0u32]);
    this
}

/// `operator new` (`00401000`, `cdecl` (size)).
const OPERATOR_NEW: u32 = 0x0040_1000;

// Translated from 008225d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A map's bucket-array allocator (`thiscall`, `RET 4`; `this` unused):
/// `operator new` of `count * 4` bytes.
pub fn fn_008225d0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(OPERATOR_NEW, &args![count << 2]).u32()
}

/// `0081bf20` (named `MagicHitEffect::InitLoadGame_ov2` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const INIT_LOAD_GAME_TARGET_0081BF20: u32 = 0x0081_bf20;

// Translated from 0081d9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::InitLoadGame` overload 2 (engine map vtable name; vtable words
/// 0107b304 and 0107b7c4): forwards `this` and one pointer to the base body `0081bf20`
/// (`MagicHitEffect::InitLoadGame_ov2`, Xbox PDB
/// `?InitLoadGame@MagicHitEffect@@UAAXPAVBGSLoadGameBuffer@@@Z`, void). Returns nothing.
pub fn magic_shader_hit_effect_init_load_game_ov2(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(INIT_LOAD_GAME_TARGET_0081BF20, &args![this, buffer]);
}

// Translated from 0081f310 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011dc804 without reading the receiver. The map names this body `MagicShaderHitEffect::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0081f310(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_c804
}

layout! {
    /// `MagicShaderHitEffect` (Xbox PDB), 0x6c bytes; the PC constructor (`0081f350`) and
    /// destructor (`0081f770`) use the PDB's offsets.
    pub struct MagicShaderHitEffect: 0x6c {
        /// `MagicHitEffect::pTarget` (Xbox PDB): `TESObjectREFR *`.
        0x1c pTarget: u32,
        /// `MagicHitEffect::fAge` (Xbox PDB).
        0x20 fAge: u32,
        /// `MagicHitEffect::bFinished` (Xbox PDB).
        0x24 bFinished: u8,
        /// `bWeaponEnchantment` (Xbox PDB).
        0x28 bWeaponEnchantment: u8,
        /// `pWornObject` (Xbox PDB).
        0x2c pWornObject: u32,
        /// `pEffectData` (Xbox PDB): `TESEffectShader *`.
        0x30 pEffectData: u32,
        /// `fAlphaTimer` (Xbox PDB).
        0x34 fAlphaTimer: u32,
        /// `spParticleNode` (Xbox PDB): `NiPointer<NiNode>`, after the
        /// `ParticlePropertyArray` at +0x38.
        0x48 spParticleNode: u32,
        /// `b3rdPerson` (Xbox PDB).
        0x4c b3rdPerson: u8,
        /// `fAddonAlpha` (Xbox PDB), after the `AddOnObjects` array at +0x50.
        0x60 fAddonAlpha: u32,
        /// `fAddonScale` (Xbox PDB).
        0x64 fAddonScale: u32,
        /// `spTexEffectData` (Xbox PDB).
        0x68 spTexEffectData: u32,
    }
}

/// The member offsets of the two arrays (`BSSimpleArray<NiPointer<..>, 1024>`, 0x10 bytes)
/// and the two `NiPointer`s.
const PARTICLE_PROPERTY_ARRAY: u32 = 0x38;
const ADD_ON_OBJECTS: u32 = 0x50;
const PARTICLE_NODE: u32 = 0x48;
const TEX_EFFECT_DATA: u32 = 0x68;

/// The vtables: `MagicShaderHitEffect`'s (`0107b70c`) and the three arrays' (`0107b824`,
/// `0107b838`, `0107b84c`; their RTTI names `BSSimpleArray<NiPointer<ParticleShader..>>`,
/// `BSSimpleArray<NiPointer<NiAVObject>>` and `BSSimpleArray<char *, 1024>`).
const MAGIC_SHADER_HIT_EFFECT_VTABLE: u32 = 0x0107_b70c;
const PARTICLE_ARRAY_VTABLE: u32 = 0x0107_b824;
const OBJECT_ARRAY_VTABLE: u32 = 0x0107_b838;
const STRING_ARRAY_VTABLE: u32 = 0x0107_b84c;

/// The player (`011dea3c`), the object at `011dea10` (`TES`), the model loader (`011c3b3c`).
const PLAYER: u32 = 0x011d_ea3c;
const TES_011DEA10: u32 = 0x011d_ea10;
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;

/// Callees outside this unit, by address (`thiscall` unless noted; stack words in
/// brackets). Several are plain field getters: `0044ddc0` the word at `+8`, `00407820` at
/// `+0x78`, `00559450` at `+0`, `009611e0` at `+0x18`, `005a29b0` at `+0x10c`, `00726070` at
/// `+4`; `00409140` answers `this + 0x1c`, `006815c0` answers `this`, `0052aa80` whether the
/// word at `+0` differs from its argument, `008256d0` whether a list node is empty.
const ARRAY_DESTRUCT_ITEMS_00822820: u32 = 0x0082_2820; // [2]
const ARRAY_FREE_BUFFER_006A8500: u32 = 0x006a_8500;
const ARRAY_INIT_00822860: u32 = 0x0082_2860; // [2]
const STRING_ARRAY_CLEAR_008454F0: u32 = 0x0084_54f0; // [1]
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460; // cdecl [2]
const MAGIC_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_b700; // [2]
const MAGIC_HIT_EFFECT_DESTRUCTOR: u32 = 0x0081_b7b0;
const NI_POINTER_FROM_00633C90: u32 = 0x0063_3c90; // [1]
const NI_POINTER_ASSIGN_0066B0D0: u32 = 0x0066_b0d0; // [1]
const NI_POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
const WORD_AT_0_00559450: u32 = 0x0055_9450;
const WORD_DIFFERS_0052AA80: u32 = 0x0052_aa80; // [1]
const WORD_AT_8_0044DDC0: u32 = 0x0044_ddc0;
const ARRAY_AT_006A7AD0: u32 = 0x006a_7ad0; // [1]
const MAGIC_ITEM_GET_AV_EFFECT: u32 = 0x0040_a300; // [1]
const WORD_AT_78_00407820: u32 = 0x0040_7820;
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
const PLAYER_NODE_00950BB0: u32 = 0x0095_0bb0; // [1]
const PLAYER_GET_BIPED: u32 = 0x0095_0b00; // [1]
const QUERY_00456610: u32 = 0x0045_6610;
const WORD_AT_10C_005A29B0: u32 = 0x005a_29b0;
const LIST_HEAD_00409140: u32 = 0x0040_9140;
const NODE_EMPTY_008256D0: u32 = 0x0082_56d0;
const SELF_006815C0: u32 = 0x0068_15c0;
const WORD_AT_4_00726070: u32 = 0x0072_6070;
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080; // [6]
const MODEL_RELEASE_0045A5E0: u32 = 0x0045_a5e0; // [1]
const STOP_TEXTURE_SHADER: u32 = 0x0050_7c90; // [2]
const STOP_PARTICLE_SHADER: u32 = 0x0050_7810; // [2]
const WORD_AT_18_009611E0: u32 = 0x0096_11e0;
const ATTACH_PARENT_00A59D00: u32 = 0x00a5_9d00; // [1]
const BS_AUDIO_INSTANCE: u32 = 0x0045_3a70; // no this
const BS_AUDIO_STOP_MOVING_SOUNDS: u32 = 0x00ad_8570; // [3]
const SHADER_FILL_TEXTURE_00506390: u32 = 0x0050_6390;
const SHADER_TEXTURE_00506370: u32 = 0x0050_6370;
const NAME_STRING_00408DA0: u32 = 0x0040_8da0;
const STRING_LENGTH_0048CEE0: u32 = 0x0048_cee0;
const FORMAT_00406D00: u32 = 0x0040_6d00; // cdecl [5]
const TES_REMOVE_TEXTURE_IMAGE: u32 = 0x0045_6800; // [2]
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb; // cdecl [5]
const ANIMATION_00490F80: u32 = 0x0049_0f80;
const ACTIVE_HIT_EFFECT_00820BF0: u32 = 0x0082_0bf0;
const SAVED_ACQUIRE_OBJECT_008D8520: u32 = 0x008d_8520;
const LOCATION_006733E0: u32 = 0x0067_33e0;
const BIPED_SLOT_004AAF30: u32 = 0x004a_af30; // [1]
const FORM_007AF430: u32 = 0x007a_f430;
const WORLD_BOUND_0043D450: u32 = 0x0043_d450;
const BOUND_RADIUS_0084D030: u32 = 0x0084_d030;
const POINT_SET_0043D410: u32 = 0x0043_d410; // [3]
const SET_TRANSLATE_00A59C60: u32 = 0x00a5_9c60; // [1]
const CAMERA_POSITION_0045BB80: u32 = 0x0045_bb80;
const POINT_SUBTRACT_00439EF0: u32 = 0x0043_9ef0; // [2]
const POINT_LENGTH_00457990: u32 = 0x0045_7990;

/// `"%s\\%s"`-shaped format (`0101dcc4`) and the `"Textures"` folder name (`01017f88`).
const PATH_FORMAT: u32 = 0x0101_dcc4;
const TEXTURES: u32 = 0x0101_7f88;
/// The two effect-shader form ids whose removal also stops the target's moving sounds.
const FIRE_SHADER_852FE: u32 = 0x0008_52fe;
const FIRE_SHADER_84A51: u32 = 0x0008_4a51;
/// The `double` 0.0 at `01012060` that `fn_00820c10` compares the bound radius with.
const ZERO_01012060: u32 = 0x0101_2060;
/// The RTTI descriptors `fn_00820c10` casts with (`__RTDynamicCast`'s source and target
/// type words).
const RTTI_011841CC: u32 = 0x0118_41cc;
const RTTI_011846D4: u32 = 0x0118_46d4;
const RTTI_011A0D1C: u32 = 0x011a_0d1c;
const RTTI_011A10AC: u32 = 0x011a_10ac;
const RTTI_01183108: u32 = 0x0118_3108;
const RTTI_01183998: u32 = 0x0118_3998;
const RTTI_01186584: u32 = 0x0118_6584;

// Translated from 006c6200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed by the body (the engine map's `BSSimpleArray<..>::_DestructItems` is a call-graph
/// name): a `BSSimpleArray` clear (`thiscall`, one word: free the buffer). With a buffer
/// (`+4`), its items are destroyed (`00822820` with the buffer and the size at `+8`); with
/// the byte set the buffer is freed (`006a8500`) and the buffer and reserved size (`+0xc`)
/// cleared; the size is cleared.
pub fn fn_006c6200(e: &mut Engine, this: Ptr, free: u8) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(4)) == 0 {
        return;
    }
    let buffer = e.mem.u32(base.wrapping_add(4));
    let size = e.mem.u32(base.wrapping_add(8));
    e.call(ARRAY_DESTRUCT_ITEMS_00822820, &args![this, buffer, size]);
    if free != 0 {
        e.call(ARRAY_FREE_BUFFER_006A8500, &args![this]);
        e.mem.set_u32(base.wrapping_add(4), 0);
        e.mem.set_u32(base.wrapping_add(0xc), 0);
    }
    e.mem.set_u32(base.wrapping_add(8), 0);
}

// Translated from 0081f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::scalar deleting destructor` (engine map, from the vtable): the
/// destructor ([`magic_shader_hit_effect_destructor`]), then the 0x6c-byte object freed
/// (`00aa1460(this, 0x6c)`) when bit 0 of the flags is set. Answers `this`.
pub fn magic_shader_hit_effect_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    flags: u32,
) -> Ptr<MagicShaderHitEffect> {
    magic_shader_hit_effect_destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x6cu32]);
    }
    this
}

/// The add-on models of the effect shader (the word at `+0x10c` of `pEffectData`, its list
/// at `+0x1c`): until the walk ends or a node is empty (`008256d0`), `model` is called with
/// the model loader and the node's item's name (its first word's `+4`), then the next node
/// (`+4`).
fn walk_addon_models(e: &mut Engine, shader: u32, mut model: impl FnMut(&mut Engine, u32, u32)) {
    if shader == 0 || e.call(WORD_AT_10C_005A29B0, &args![shader]).u32() == 0 {
        return;
    }
    let source = e.call(WORD_AT_10C_005A29B0, &args![shader]).u32();
    let mut node = e.call(LIST_HEAD_00409140, &args![source]).u32();
    while node != 0 {
        if e.call(NODE_EMPTY_008256D0, &args![node]).bool() {
            break;
        }
        let entry = e.call(SELF_006815C0, &args![node]).u32();
        let name = e.mem.u32(e.mem.u32(entry).wrapping_add(4));
        let loader = e.mem.u32(MODEL_LOADER_011C3B3C);
        model(e, loader, name);
        node = e.call(WORD_AT_4_00726070, &args![node]).u32();
    }
}

// Translated from 0081f350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::MagicShaderHitEffect` (unnamed in the engine map; the body builds
/// the PDB's members): `thiscall`, two words passed to `MagicHitEffect::MagicHitEffect`
/// (`0081b700`; the second is the active effect). Sets the vtable `0107b70c`, constructs the
/// particle array ([`fn_00822530`]), `spParticleNode` and `spTexEffectData` (null
/// `NiPointer`s, `00633c90`) and the add-on array ([`fn_00822580`]); `pEffectData` is the
/// word at `+0x78` of `MagicItem::GetAVEffect(0)` on the active effect's word at `+8`;
/// `fAlphaTimer` 0, both pointers assigned null (`0066b0d0`), `bWeaponEnchantment` and
/// `pWornObject` cleared, `fAddonAlpha` and `fAddonScale` 1.0. `b3rdPerson` is set when the
/// target is the player, the player's node `00950bb0(0)` exists and `00456610` on it answers
/// 0. Then every add-on model of the shader is loaded (`ModelLoader::LoadFile` with the name,
/// 0, 1, 0, 0, 0). Answers `this`.
///
/// The compiler left a block behind `XOR ECX,ECX; JZ` (a folded false condition, a
/// `pWornObject` lookup through `__RTDynamicCast`); it never runs and is left out.
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0081f350 - the FS:[0] exception frame (unwinding the members on a throw) is not translated
pub fn magic_shader_hit_effect_magic_shader_hit_effect(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    first: u32,
    effect: u32,
) -> Ptr<MagicShaderHitEffect> {
    let base = this.addr();
    e.call(MAGIC_HIT_EFFECT_CONSTRUCTOR, &args![this, first, effect]);
    e.mem.set_u32(base, MAGIC_SHADER_HIT_EFFECT_VTABLE);
    fn_00822530(e, Ptr::new(base.wrapping_add(PARTICLE_PROPERTY_ARRAY)));
    e.call(
        NI_POINTER_FROM_00633C90,
        &args![base.wrapping_add(PARTICLE_NODE), 0u32],
    );
    fn_00822580(e, Ptr::new(base.wrapping_add(ADD_ON_OBJECTS)));
    e.call(
        NI_POINTER_FROM_00633C90,
        &args![base.wrapping_add(TEX_EFFECT_DATA), 0u32],
    );
    e.set(this, MagicShaderHitEffect::pEffectData, 0);
    let item = e.call(WORD_AT_8_0044DDC0, &args![effect]).u32();
    let setting = e.call(MAGIC_ITEM_GET_AV_EFFECT, &args![item, 0u32]).u32();
    let shader = e.call(WORD_AT_78_00407820, &args![setting]).u32();
    e.set(this, MagicShaderHitEffect::pEffectData, shader);
    e.set(this, MagicShaderHitEffect::fAlphaTimer, 0);
    e.call(
        NI_POINTER_ASSIGN_0066B0D0,
        &args![base.wrapping_add(TEX_EFFECT_DATA), 0u32],
    );
    e.call(
        NI_POINTER_ASSIGN_0066B0D0,
        &args![base.wrapping_add(PARTICLE_NODE), 0u32],
    );
    e.set(this, MagicShaderHitEffect::bWeaponEnchantment, 0);
    e.set(this, MagicShaderHitEffect::pWornObject, 0);
    e.set(this, MagicShaderHitEffect::fAddonAlpha, 1.0f32.to_bits());
    e.set(this, MagicShaderHitEffect::fAddonScale, 1.0f32.to_bits());
    let player = e.mem.u32(PLAYER);
    if e.get(this, MagicShaderHitEffect::pTarget) == player {
        let has_node = e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32() != 0;
        let third_person = has_node && {
            let player = e.mem.u32(PLAYER);
            let node = e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32();
            !e.call(QUERY_00456610, &args![node]).bool()
        };
        e.set(
            this,
            MagicShaderHitEffect::b3rdPerson,
            u8::from(third_person),
        );
    } else {
        e.set(this, MagicShaderHitEffect::b3rdPerson, 0);
    }
    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
    walk_addon_models(e, shader, |e, loader, name| {
        load_model(e, loader, name);
    });
    this
}

/// `ModelLoader::LoadFile` on the model loader with a model name and the words 0, 1, 0, 0, 0
/// (the order the six stack words take).
fn load_model(e: &mut Engine, loader: u32, name: u32) {
    e.call(
        MODEL_LOADER_LOAD_FILE,
        &args![loader, name, 0u32, 1u32, 0u32, 0u32, 0u32],
    );
}

/// Stops the texture shader of `pEffectData` on `target` with `spTexEffectData`
/// (`TESEffectShader::StopTextureShader`).
fn stop_texture_shader(e: &mut Engine, this: Ptr<MagicShaderHitEffect>, target: u32) {
    let data = e
        .call(
            WORD_AT_0_00559450,
            &args![this.addr().wrapping_add(TEX_EFFECT_DATA)],
        )
        .u32();
    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
    e.call(STOP_TEXTURE_SHADER, &args![shader, target, data]);
}

/// Formats `"Textures\<name>"` into the 0x104-byte buffer (`00406d00`) and removes that
/// texture image (`TES::RemoveTextureImage` on `011dea10` with 0).
fn remove_texture_image(e: &mut Engine, buffer: u32, name: u32) {
    e.call(
        FORMAT_00406D00,
        &args![buffer, 0x104u32, PATH_FORMAT, TEXTURES, name],
    );
    let tes = e.mem.u32(TES_011DEA10);
    e.call(TES_REMOVE_TEXTURE_IMAGE, &args![tes, buffer, 0u32]);
}

// Translated from 0081f770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081f770 - the FS:[0] exception frame and the /GS stack cookie check (00ec408c) are not translated
/// `MagicShaderHitEffect::~MagicShaderHitEffect` (unnamed in the engine map; the body
/// destroys the PDB's members). Sets the vtable `0107b70c`; clears `pTarget` when
/// `0084e3a0` answers 0 and the target is not the player. With texture effect data, the
/// texture shader is stopped: on each of the player's two nodes (`00950bb0(1)`,
/// `00950bb0(0)`) that exists for the player, else on the target's node (slot +0x1d0) when
/// it has one. With a particle node (`0052aa80(0)`), each particle property's particle
/// shader is stopped (`TESEffectShader::StopParticleShader` with the node and the property).
/// Each add-on object with a parent (the word at `+0x18`) is handed to the parent's slot
/// +0xe8. The particle array is cleared ([`fn_006c6200`] with 1); a particle node still set
/// is detached (`00a59d00(0)`); both pointers are assigned null. With `pEffectData`: the
/// target's moving sounds stop (`BSAudio::StopMovingSounds` with its node, 1.0, 0) for the
/// two fire shaders (form ids `852fe`, `84a51`); the shader's fill texture (`00506390`) and
/// texture (`00506370`, when its name is not empty, `0048cee0`) images are removed
/// (`"Textures\<name>"`); each add-on model is released (`0045a5e0` on the model loader).
/// Last the members' destructors run (`0045cec0`, the add-on array, `0045cec0`, the particle
/// array) and `MagicHitEffect::~MagicHitEffect` (`0081b7b0`).
pub fn magic_shader_hit_effect_destructor(e: &mut Engine, this: Ptr<MagicShaderHitEffect>) {
    let base = this.addr();
    let particles = base.wrapping_add(PARTICLE_PROPERTY_ARRAY);
    let objects = base.wrapping_add(ADD_ON_OBJECTS);
    let particle_node = base.wrapping_add(PARTICLE_NODE);
    let tex_effect_data = base.wrapping_add(TEX_EFFECT_DATA);
    e.mem.set_u32(base, MAGIC_SHADER_HIT_EFFECT_VTABLE);
    let id_is_zero = e.call(FORM_ID_0084E3A0, &args![this]).u32() == 0;
    let not_player = e.get(this, MagicShaderHitEffect::pTarget) != e.mem.u32(PLAYER);
    if id_is_zero && not_player {
        e.set(this, MagicShaderHitEffect::pTarget, 0);
    }
    if e.call(WORD_AT_0_00559450, &args![tex_effect_data]).u32() != 0 {
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        if target == e.mem.u32(PLAYER) {
            for third_person in [1u32, 0] {
                let player = e.mem.u32(PLAYER);
                let node = e
                    .call(PLAYER_NODE_00950BB0, &args![player, third_person])
                    .u32();
                if node != 0 {
                    stop_texture_shader(e, this, node);
                }
            }
        } else if target != 0 {
            let node = e.vcall(target, 0x1d0, &args![]).u32();
            if node != 0 {
                stop_texture_shader(e, this, node);
            }
        }
    }
    if e.call(WORD_DIFFERS_0052AA80, &args![particle_node, 0u32])
        .bool()
    {
        let mut index = 0u32;
        while index < e.call(WORD_AT_8_0044DDC0, &args![particles]).u32() {
            let slot = e.call(ARRAY_AT_006A7AD0, &args![particles, index]).u32();
            let property = e.call(WORD_AT_0_00559450, &args![slot]).u32();
            let node = e.call(WORD_AT_0_00559450, &args![particle_node]).u32();
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            e.call(STOP_PARTICLE_SHADER, &args![shader, node, property]);
            index += 1;
        }
    }
    if e.call(WORD_AT_8_0044DDC0, &args![objects]).u32() != 0 {
        let mut index = 0u32;
        while index < e.call(WORD_AT_8_0044DDC0, &args![objects]).u32() {
            let slot = e.call(ARRAY_AT_006A7AD0, &args![objects, index]).u32();
            let object = e.call(WORD_AT_0_00559450, &args![slot]).u32();
            if e.call(WORD_AT_18_009611E0, &args![object]).u32() != 0 {
                let parent = e.call(WORD_AT_18_009611E0, &args![object]).u32();
                e.vcall(parent, 0xe8, &args![object]);
            }
            index += 1;
        }
    }
    fn_006c6200(e, Ptr::new(particles), 1);
    if e.call(WORD_DIFFERS_0052AA80, &args![particle_node, 0u32])
        .bool()
    {
        let node = e.call(WORD_AT_0_00559450, &args![particle_node]).u32();
        e.call(ATTACH_PARENT_00A59D00, &args![node, 0u32]);
    }
    e.call(NI_POINTER_ASSIGN_0066B0D0, &args![particle_node, 0u32]);
    e.call(NI_POINTER_ASSIGN_0066B0D0, &args![tex_effect_data, 0u32]);
    if e.get(this, MagicShaderHitEffect::pEffectData) != 0 {
        if e.get(this, MagicShaderHitEffect::pTarget) != 0 {
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            let fire = e.call(FORM_ID_0084E3A0, &args![shader]).u32() == FIRE_SHADER_852FE || {
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                e.call(FORM_ID_0084E3A0, &args![shader]).u32() == FIRE_SHADER_84A51
            };
            if fire {
                let target = e.get(this, MagicShaderHitEffect::pTarget);
                let node = e.vcall(target, 0x1d0, &args![]).u32();
                let audio = e.call(BS_AUDIO_INSTANCE, &[]).u32();
                e.call(
                    BS_AUDIO_STOP_MOVING_SOUNDS,
                    &args![audio, node, 1.0f32.to_bits(), 0u32],
                );
            }
        }
        e.with_stack(0x104, |e, buffer| {
            let buffer = buffer.addr();
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            let fill = e.call(SHADER_FILL_TEXTURE_00506390, &args![shader]).u32();
            if e.call(NAME_STRING_00408DA0, &args![fill]).u32() != 0 {
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                let fill = e.call(SHADER_FILL_TEXTURE_00506390, &args![shader]).u32();
                let name = e.call(NAME_STRING_00408DA0, &args![fill]).u32();
                remove_texture_image(e, buffer, name);
            }
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            let texture = e.call(SHADER_TEXTURE_00506370, &args![shader]).u32();
            if e.call(STRING_LENGTH_0048CEE0, &args![texture]).u32() != 0 {
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                let texture = e.call(SHADER_TEXTURE_00506370, &args![shader]).u32();
                let name = e.call(NAME_STRING_00408DA0, &args![texture]).u32();
                remove_texture_image(e, buffer, name);
            }
        });
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        walk_addon_models(e, shader, |e, loader, name| {
            e.call(MODEL_RELEASE_0045A5E0, &args![loader, name]);
        });
    }
    e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![tex_effect_data]);
    bs_simple_array_ni_av_object_destructor(e, Ptr::new(objects));
    e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![particle_node]);
    bs_simple_array_particle_shader_property_destructor(e, Ptr::new(particles));
    e.call(MAGIC_HIT_EFFECT_DESTRUCTOR, &args![this]);
}

/// More callees of [`magic_shader_hit_effect_init`] (`thiscall` unless noted; stack words in
/// brackets): the active effect's age (`006b9130`, in `ST0`), item (`007df1d0`) and flag
/// tests (`00403e60` [1], `00804b80` [1]); the effect shader's flag test (`00507040` [1]),
/// particle start (`TESEffectShader::StartParticleShader` [4]) and texture effect data
/// (`00507830` [3], `TESEffectShader::StartTextureShader` [2]), its fill-texture and
/// texture names (`00506390`, `00506370`, `005063b0`) and three floats (`008d01e0`,
/// `00644930`, `00567470`, in `ST0`); the `float` helper `00404010` (`cdecl` [2], `ST0`) and
/// `_ftol2_sse` (`00ec62c0`, the value in `ST0`); the particle count step `00ba52d0` (no
/// `this`); the sound handle calls; the node set-up (`00aa13e0` `cdecl` [1], the `NiNode`
/// constructor `00a5ecb0` [1], the fixed string `00438170` [1] / `004381b0`, the name setter
/// `00a5b950` [1], `0043fa80` [1]); the array append `006c61b0` [1]; the particle calls
/// (`00820b70` [1], `00820bd0` [1], `00820bb0`, `00820b90`,
/// `ParticleShaderProperty::CollectTargets` [1], `InitParticleData` [1],
/// `UpdateParticles` [3]); `TES::CreateTextureImage` [4]; the actor alpha calls; and the
/// add-on objects (`00820b50`, `00658930`, `00877a30` [1], the weighted random `00822440`,
/// `BSWeightedRandom<BGSBodyPart *>::AddWeightedResult` [2], `00822650`, `008224b0`,
/// `ModelLoader::FindModel` [2], `0043b230`, the cloning process `004ad050` [1] /
/// `004ad270`, `NiObject::Clone` [1], `008c7aa0` (no `this`), the task queue `004537b0` (no
/// `this`) and `TaskQueueInterface::QueueNodeAttach` [4], `00440490` [1],
/// `NiTimeController::StartAnimations` (`cdecl` [1]) and the pointer release `0040c110`).
const ACTIVE_EFFECT_AGE_006B9130: u32 = 0x006b_9130;
const ACTIVE_EFFECT_ITEM_007DF1D0: u32 = 0x007d_f1d0;
const ITEM_FLAG_00403E60: u32 = 0x0040_3e60;
const ACTIVE_EFFECT_FLAG_00804B80: u32 = 0x0080_4b80;
const SHADER_FLAG_00507040: u32 = 0x0050_7040;
const START_PARTICLE_SHADER: u32 = 0x0050_77c0;
const TEXTURE_EFFECT_DATA_00507830: u32 = 0x0050_7830;
const START_TEXTURE_SHADER: u32 = 0x0050_7b40;
const SHADER_TEXTURE_005063B0: u32 = 0x0050_63b0;
const SHADER_VALUE_008D01E0: u32 = 0x008d_01e0;
const SHADER_VALUE_00644930: u32 = 0x0064_4930;
const SHADER_VALUE_00567470: u32 = 0x0056_7470;
const FLOAT_HELPER_00404010: u32 = 0x0040_4010;
const FTOL: u32 = 0x00ec_62c0;
const PARTICLE_STEP_00BA52D0: u32 = 0x00ba_52d0;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550; // [3]
const SOUND_SET_POSITION: u32 = 0x00ad_8b60; // [3]
const SOUND_SET_VOLUME: u32 = 0x00ad_89e0; // [1]
const SOUND_PLAY: u32 = 0x00ad_8830; // [1]
const SOUND_SET_OBJECT_TO_FOLLOW: u32 = 0x00ad_8f20; // [1]
const SOUND_HANDLE_DESTRUCT_00483710: u32 = 0x0048_3710;
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const NI_NODE_CONSTRUCT_00A5ECB0: u32 = 0x00a5_ecb0;
const FIXED_STRING_FROM_00438170: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT_004381B0: u32 = 0x0043_81b0;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const NODE_SET_0043FA80: u32 = 0x0043_fa80;
const ARRAY_APPEND_006C61B0: u32 = 0x006c_61b0;
const PARTICLE_SET_RADIUS_00820B70: u32 = 0x0082_0b70;
const PARTICLE_SET_00820BD0: u32 = 0x0082_0bd0;
const PARTICLE_FIRST_PERSON_00820BB0: u32 = 0x0082_0bb0;
const PARTICLE_THIRD_PERSON_00820B90: u32 = 0x0082_0b90;
const COLLECT_TARGETS: u32 = 0x00ba_8560;
const INIT_PARTICLE_DATA: u32 = 0x00ba_6e00;
const UPDATE_PARTICLES: u32 = 0x00ba_7b60;
const TES_CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
const ACTOR_GET_ALPHA_MULT: u32 = 0x008c_4850;
const ACTOR_SET_ALPHA_MULT: u32 = 0x008c_4890;
const ACTOR_UPDATE_ALPHA: u32 = 0x008c_4640;
const PARTICLE_OBJECTS_00820B50: u32 = 0x0082_0b50;
const COUNT_00658930: u32 = 0x0065_8930;
const ELEMENT_00877A30: u32 = 0x0087_7a30;
const WEIGHTED_RANDOM_CONSTRUCT_00822440: u32 = 0x0082_2440;
const ADD_WEIGHTED_RESULT: u32 = 0x009b_7990;
const WEIGHTED_RANDOM_PICK_00822650: u32 = 0x0082_2650;
const WEIGHTED_RANDOM_DESTRUCT_008224B0: u32 = 0x0082_24b0;
const FIND_MODEL: u32 = 0x0044_72a0;
const MODEL_ROOT_0043B230: u32 = 0x0043_b230;
const CLONING_PROCESS_CONSTRUCT_004AD050: u32 = 0x004a_d050;
const CLONING_PROCESS_DESTRUCT_004AD270: u32 = 0x004a_d270;
const NI_OBJECT_CLONE: u32 = 0x00a5_d2c0;
const QUEUE_ATTACH_TEST_008C7AA0: u32 = 0x008c_7aa0;
const TASK_QUEUE_004537B0: u32 = 0x0045_37b0;
const QUEUE_NODE_ATTACH: u32 = 0x0087_ac00;
const SET_SCALE_00440490: u32 = 0x0044_0490;
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
const POINTER_RELEASE_0040C110: u32 = 0x0040_c110;

/// The data [`magic_shader_hit_effect_init`] reads: the alpha timer for an active effect with
/// flag 0x10 (`float` at `01016410`), the fire sound's name (`"AMBFireMediumLP"`,
/// `0107b810`), the particle node's name (`"ParticleShader Geometry"`, `0107b7f8`), the word
/// `0043fa80` gets (`011a9448`), the `double` subtracted for the first particle update
/// (`0107b7f0`), the alpha above which the target's alpha is lowered (`double` at
/// `0106d508`) and the lower alpha (`float` at `0106682c`), and the add-on scale's `float`
/// (`01013ea4`).
const ALPHA_TIMER_01016410: u32 = 0x0101_6410;
const FIRE_SOUND_NAME: u32 = 0x0107_b810;
const PARTICLE_NODE_NAME: u32 = 0x0107_b7f8;
const NODE_ARGUMENT_011A9448: u32 = 0x011a_9448;
const UPDATE_STEP_0107B7F0: u32 = 0x0107_b7f0;
const ALPHA_LIMIT_0106D508: u32 = 0x0106_d508;
const LOWER_ALPHA_0106682C: u32 = 0x0106_682c;
const ADDON_SCALE_01013EA4: u32 = 0x0101_3ea4;

/// The frame of [`magic_shader_hit_effect_init`]: the two flag bytes, the radius and the two
/// node words [`fn_00820c10`] fills, the sound handle, the fixed string, the `NiPointer`
/// locals, the weighted random, the cloning process and the three path buffers.
const FRAME_FIRST_FLAG: u32 = 0x00;
const FRAME_SECOND_FLAG: u32 = 0x01;
const FRAME_RADIUS: u32 = 0x04;
const FRAME_NODE: u32 = 0x08;
const FRAME_NODE_2: u32 = 0x0c;
const FRAME_SOUND: u32 = 0x10;
const FRAME_NAME: u32 = 0x20;
const FRAME_FILL: u32 = 0x24;
const FRAME_PARTICLE: u32 = 0x28;
const FRAME_TEXTURE_A: u32 = 0x2c;
const FRAME_TEXTURE_B: u32 = 0x30;
const FRAME_WEIGHTED: u32 = 0x40;
const FRAME_MODEL: u32 = 0x80;
const FRAME_CLONING: u32 = 0x84;
const FRAME_CLONE: u32 = 0xc4;
const FRAME_PATH_1: u32 = 0x100;
const FRAME_PATH_2: u32 = 0x210;
const FRAME_PATH_3: u32 = 0x320;
const FRAME_SIZE: u32 = 0x430;

/// `"Textures\<name>"` into the 0x104-byte buffer (`00406d00`) and `TES::CreateTextureImage`
/// of it into the `NiPointer` at `texture` (with 0, 0).
fn create_texture_image(e: &mut Engine, buffer: u32, name: u32, texture: u32) {
    e.call(
        FORMAT_00406D00,
        &args![buffer, 0x104u32, PATH_FORMAT, TEXTURES, name],
    );
    let tes = e.mem.u32(TES_011DEA10);
    e.call(
        TES_CREATE_TEXTURE_IMAGE,
        &args![tes, buffer, texture, 0u32, 0u32],
    );
}

// Translated from 0081fc70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081fc70 - the FS:[0] exception frame and the /GS stack cookie check (00ec408c) are not translated
/// `MagicShaderHitEffect::Init` (engine map, from the vtable; `thiscall`, answers a byte).
///
/// With an active effect (the word at `+0x18`): `fAge` is its age, `bFinished` is set when its
/// item lacks flag 0x400, and an effect with flag 0x10 sets `fAlphaTimer` from `01016410`.
/// For the player as target (with a node `00950bb0(0)`), `b3rdPerson` is updated and the
/// particles restart when it changed while `fAge` is above 0. The attach point
/// ([`fn_00820c10`]) must give both nodes and there must be an effect shader, else the answer
/// is 0. The two fire shaders (`852fe`, `84a51`) play the fire sound at the target, following
/// its node. Unless a weapon enchantment's target is an actor whose saved acquire object
/// says no (slot +0x1b0, or slot +0x454 failing while the player's active hit effect is
/// another one), and unless the shader has flag 8: a missing particle node is made (an
/// `NiNode` named `"ParticleShader Geometry"`); with no particles yet and a fill texture,
/// the texture is created and `ftol(00404010(...)) / 00ba52d0()` particle properties
/// started (`TESEffectShader::StartParticleShader`) and appended; each particle property is
/// then set up (the radius, the weapon enchantment, first or third person, its targets) and,
/// on a restart, its particles re-initialised and updated twice; then the vtable slot +0xcc.
/// Otherwise the particle shaders are stopped and the particle node dropped. Without shader
/// flag 1 the texture effect data is made when missing (two texture images, `00507830`; a
/// visible actor target above the alpha limit gets the lower alpha) and the texture shader
/// started. Without a restart, add-on objects and a particle property, the shader's add-on
/// models are loaded into a weighted random and a clone of a picked model attached to each
/// node of the first particle property's objects (queued when `008c7aa0` answers), with
/// `fAddonAlpha` 0 and `fAddonScale` from `00404010`. Answers whether there are particle
/// properties or texture effect data.
pub fn magic_shader_hit_effect_init(e: &mut Engine, this: Ptr<MagicShaderHitEffect>) -> bool {
    let base = this.addr();
    let particles = base.wrapping_add(PARTICLE_PROPERTY_ARRAY);
    let objects = base.wrapping_add(ADD_ON_OBJECTS);
    let particle_node = base.wrapping_add(PARTICLE_NODE);
    let tex_effect_data = base.wrapping_add(TEX_EFFECT_DATA);
    if e.call(WORD_AT_18_009611E0, &args![this]).u32() != 0 {
        let effect = e.call(WORD_AT_18_009611E0, &args![this]).u32();
        let age = e.call(ACTIVE_EFFECT_AGE_006B9130, &args![effect]).st0;
        e.set(this, MagicShaderHitEffect::fAge, x87_store32(age));
        let effect = e.call(WORD_AT_18_009611E0, &args![this]).u32();
        let item = e.call(ACTIVE_EFFECT_ITEM_007DF1D0, &args![effect]).u32();
        let flagged = e.call(ITEM_FLAG_00403E60, &args![item, 0x400u32]).bool();
        e.set(this, MagicShaderHitEffect::bFinished, u8::from(!flagged));
        let effect = e.call(WORD_AT_18_009611E0, &args![this]).u32();
        if e.call(ACTIVE_EFFECT_FLAG_00804B80, &args![effect, 0x10u32])
            .bool()
        {
            let timer = x87_quiet_bits(e.mem.u32(ALPHA_TIMER_01016410));
            e.set(this, MagicShaderHitEffect::fAlphaTimer, timer);
        }
    }
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let at = |offset: u32| frame + offset;
        e.mem.set_u8(at(FRAME_SECOND_FLAG), 0);
        e.mem.set_u8(at(FRAME_FIRST_FLAG), 0);
        e.mem.set_u32(at(FRAME_RADIUS), 0.0f32.to_bits());
        let mut restart = 0u8;
        let player = e.mem.u32(PLAYER);
        if e.get(this, MagicShaderHitEffect::pTarget) == player
            && e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32() != 0
        {
            let player = e.mem.u32(PLAYER);
            let node = e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32();
            let third_person = u8::from(!e.call(QUERY_00456610, &args![node]).bool());
            let age = x87_load32(e.get(this, MagicShaderHitEffect::fAge));
            let zero: f64 = e.global(ZERO_01012060);
            // FCOMP; TEST AH,0x41; JNZ: below, equal or unordered gives 0.
            let changed = third_person != e.get(this, MagicShaderHitEffect::b3rdPerson);
            restart = u8::from(age > zero && changed);
            e.set(this, MagicShaderHitEffect::b3rdPerson, third_person);
        }
        e.mem.set_u32(at(FRAME_NODE), 0);
        e.mem.set_u32(at(FRAME_NODE_2), 0);
        fn_00820c10(
            e,
            this,
            at(FRAME_FIRST_FLAG),
            at(FRAME_SECOND_FLAG),
            at(FRAME_RADIUS),
            at(FRAME_NODE),
            at(FRAME_NODE_2),
        );
        if e.mem.u32(at(FRAME_NODE)) == 0
            || e.mem.u32(at(FRAME_NODE_2)) == 0
            || e.get(this, MagicShaderHitEffect::pEffectData) == 0
        {
            return false;
        }
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        let fire = e.call(FORM_ID_0084E3A0, &args![shader]).u32() == FIRE_SHADER_852FE || {
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            e.call(FORM_ID_0084E3A0, &args![shader]).u32() == FIRE_SHADER_84A51
        };
        if fire {
            let sound = at(FRAME_SOUND);
            let audio = e.call(BS_AUDIO_INSTANCE, &[]).u32();
            e.call(
                GET_SOUND_HANDLE_BY_NAME,
                &args![audio, sound, FIRE_SOUND_NAME, 0x12u32],
            );
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            let position = e.vcall(target, 0x1f4, &args![]).u32();
            let x = x87_quiet_bits(e.mem.u32(position));
            let y = x87_quiet_bits(e.mem.u32(position.wrapping_add(4)));
            let z = x87_quiet_bits(e.mem.u32(position.wrapping_add(8)));
            e.call(SOUND_SET_POSITION, &args![sound, x, y, z]);
            e.call(SOUND_SET_VOLUME, &args![sound, 1.0f32.to_bits()]);
            e.call(SOUND_PLAY, &args![sound, 1u32]);
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            let node = e.vcall(target, 0x1d0, &args![]).u32();
            e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![sound, node]);
            e.call(SOUND_HANDLE_DESTRUCT_00483710, &args![sound]);
        }
        let mut attach = true;
        if e.get(this, MagicShaderHitEffect::bWeaponEnchantment) != 0 {
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            if e.vcall(target, 0x100, &args![]).bool()
                && e.call(SAVED_ACQUIRE_OBJECT_008D8520, &args![target]).u32() != 0
            {
                let target = e.get(this, MagicShaderHitEffect::pTarget);
                let acquired = e.call(SAVED_ACQUIRE_OBJECT_008D8520, &args![target]).u32();
                if e.vcall(acquired, 0x1b0, &args![]).bool() {
                    attach = false;
                }
                let target = e.get(this, MagicShaderHitEffect::pTarget);
                let acquired = e.call(SAVED_ACQUIRE_OBJECT_008D8520, &args![target]).u32();
                if !e.vcall(acquired, 0x454, &args![]).bool() {
                    let player = e.mem.u32(PLAYER);
                    if e.call(ACTIVE_HIT_EFFECT_00820BF0, &args![player]).u32() != base {
                        attach = false;
                    }
                }
            }
        }
        let start = attach && {
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            !e.call(SHADER_FLAG_00507040, &args![shader, 8u32]).bool()
        };
        if start {
            if fn_00822510(e, Ptr::new(particle_node), 0) != 0 {
                let block = e.call(ALLOCATE_00AA13E0, &args![0xacu32]).u32();
                let node = if block != 0 {
                    e.call(NI_NODE_CONSTRUCT_00A5ECB0, &args![block, 0u32])
                        .u32()
                } else {
                    0
                };
                e.call(NI_POINTER_ASSIGN_0066B0D0, &args![particle_node, node]);
                let name_slot = at(FRAME_NAME);
                let name = e
                    .call(
                        FIXED_STRING_FROM_00438170,
                        &args![name_slot, PARTICLE_NODE_NAME],
                    )
                    .u32();
                let node = e.call(WORD_AT_0_00559450, &args![particle_node]).u32();
                e.call(SET_NAME_00A5B950, &args![node, name]);
                e.call(FIXED_STRING_DESTRUCT_004381B0, &args![name_slot]);
                let node = e.call(WORD_AT_0_00559450, &args![particle_node]).u32();
                e.call(NODE_SET_0043FA80, &args![node, NODE_ARGUMENT_011A9448]);
            }
            if e.call(WORD_AT_8_0044DDC0, &args![particles]).u32() == 0 {
                let fill = at(FRAME_FILL);
                e.call(NI_POINTER_FROM_00633C90, &args![fill, 0u32]);
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                let texture = e.call(SHADER_FILL_TEXTURE_00506390, &args![shader]).u32();
                if e.call(STRING_LENGTH_0048CEE0, &args![texture]).u32() != 0 {
                    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                    let texture = e.call(SHADER_FILL_TEXTURE_00506390, &args![shader]).u32();
                    let name = e.call(NAME_STRING_00408DA0, &args![texture]).u32();
                    create_texture_image(e, at(FRAME_PATH_1), name, fill);
                    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                    let first = e.call(SHADER_VALUE_008D01E0, &args![shader]).st0;
                    let first = x87_store32(first);
                    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                    let second = e.call(SHADER_VALUE_00644930, &args![shader]).st0;
                    let second = x87_store32(second);
                    let value = e.call(FLOAT_HELPER_00404010, &args![second, first]).st0;
                    let total = e.call(FTOL, &args![value]).i32();
                    let step = e.call(PARTICLE_STEP_00BA52D0, &[]).i32();
                    let mut index = 0i32;
                    while index.wrapping_mul(step) < total {
                        let alpha = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAlphaTimer));
                        let texture = e.call(WORD_AT_0_00559450, &args![fill]).u32();
                        let node_2 = e.mem.u32(at(FRAME_NODE_2));
                        let node = e.call(WORD_AT_0_00559450, &args![particle_node]).u32();
                        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                        let property = e
                            .call(
                                START_PARTICLE_SHADER,
                                &args![shader, node, node_2, texture, alpha],
                            )
                            .u32();
                        let pointer = at(FRAME_PARTICLE);
                        e.call(NI_POINTER_FROM_00633C90, &args![pointer, property]);
                        e.call(ARRAY_APPEND_006C61B0, &args![particles, pointer]);
                        e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![pointer]);
                        index = index.wrapping_add(1);
                    }
                }
                e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![fill]);
            }
            let mut index = 0u32;
            while index < e.call(WORD_AT_8_0044DDC0, &args![particles]).u32() {
                let slot = e.call(ARRAY_AT_006A7AD0, &args![particles, index]).u32();
                let property = e.call(WORD_AT_0_00559450, &args![slot]).u32();
                if e.mem.u8(at(FRAME_SECOND_FLAG)) != 0 {
                    let radius = x87_quiet_bits(e.mem.u32(at(FRAME_RADIUS)));
                    e.call(PARTICLE_SET_RADIUS_00820B70, &args![property, radius]);
                }
                if e.get(this, MagicShaderHitEffect::bWeaponEnchantment) != 0 {
                    e.call(PARTICLE_SET_00820BD0, &args![property, 0u32]);
                }
                let target = e.get(this, MagicShaderHitEffect::pTarget);
                if target != 0 {
                    let form = e.call(FORM_007AF430, &args![target]).u32();
                    let cast = e
                        .call(
                            RT_DYNAMIC_CAST,
                            &args![form, 0u32, RTTI_01183108, RTTI_01186584, 0u32],
                        )
                        .u32();
                    if e.mem.u8(at(FRAME_FIRST_FLAG)) != 0 || cast != 0 {
                        if e.get(this, MagicShaderHitEffect::pTarget) == e.mem.u32(PLAYER)
                            && e.get(this, MagicShaderHitEffect::b3rdPerson) == 0
                        {
                            e.call(PARTICLE_FIRST_PERSON_00820BB0, &args![property]);
                        } else {
                            e.call(PARTICLE_THIRD_PERSON_00820B90, &args![property]);
                        }
                    }
                }
                let node_2 = e.mem.u32(at(FRAME_NODE_2));
                e.call(COLLECT_TARGETS, &args![property, node_2]);
                if restart != 0 {
                    e.call(INIT_PARTICLE_DATA, &args![property, 0.0f32.to_bits()]);
                    let step: f64 = e.global(UPDATE_STEP_0107B7F0);
                    let timer = x87_load32(e.get(this, MagicShaderHitEffect::fAlphaTimer));
                    let earlier = x87_store32(x87_sub(timer, step));
                    e.call(UPDATE_PARTICLES, &args![property, earlier, 0u32, 1u32]);
                    let timer = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAlphaTimer));
                    e.call(UPDATE_PARTICLES, &args![property, timer, 0u32, 1u32]);
                }
                index += 1;
            }
            e.vcall(base, 0xcc, &args![]);
        } else if e.call(WORD_AT_8_0044DDC0, &args![particles]).u32() != 0
            && e.call(WORD_AT_0_00559450, &args![particle_node]).u32() != 0
        {
            let mut index = 0u32;
            while index < e.call(WORD_AT_8_0044DDC0, &args![particles]).u32() {
                let slot = e.call(ARRAY_AT_006A7AD0, &args![particles, index]).u32();
                let property = e.call(WORD_AT_0_00559450, &args![slot]).u32();
                let node = e.call(WORD_AT_0_00559450, &args![particle_node]).u32();
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                e.call(STOP_PARTICLE_SHADER, &args![shader, node, property]);
                index += 1;
            }
            fn_006c6200(e, Ptr::new(particles), 1);
            if e.call(WORD_AT_0_00559450, &args![particle_node]).u32() != 0 {
                let node = e.call(WORD_AT_0_00559450, &args![particle_node]).u32();
                e.call(ATTACH_PARENT_00A59D00, &args![node, 0u32]);
            }
            e.call(NI_POINTER_ASSIGN_0066B0D0, &args![particle_node, 0u32]);
        }
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        if !e.call(SHADER_FLAG_00507040, &args![shader, 1u32]).bool() {
            if fn_00822510(e, Ptr::new(tex_effect_data), 0) != 0 {
                let texture_a = at(FRAME_TEXTURE_A);
                e.call(NI_POINTER_FROM_00633C90, &args![texture_a, 0u32]);
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                let texture = e.call(SHADER_TEXTURE_00506370, &args![shader]).u32();
                if e.call(STRING_LENGTH_0048CEE0, &args![texture]).u32() != 0 {
                    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                    let texture = e.call(SHADER_TEXTURE_00506370, &args![shader]).u32();
                    let name = e.call(NAME_STRING_00408DA0, &args![texture]).u32();
                    create_texture_image(e, at(FRAME_PATH_2), name, texture_a);
                }
                let texture_b = at(FRAME_TEXTURE_B);
                e.call(NI_POINTER_FROM_00633C90, &args![texture_b, 0u32]);
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                let texture = e.call(SHADER_TEXTURE_005063B0, &args![shader]).u32();
                if e.call(STRING_LENGTH_0048CEE0, &args![texture]).u32() != 0 {
                    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                    let texture = e.call(SHADER_TEXTURE_005063B0, &args![shader]).u32();
                    let name = e.call(NAME_STRING_00408DA0, &args![texture]).u32();
                    create_texture_image(e, at(FRAME_PATH_3), name, texture_b);
                }
                let second = e.call(WORD_AT_0_00559450, &args![texture_b]).u32();
                let first = e.call(WORD_AT_0_00559450, &args![texture_a]).u32();
                let node = e.mem.u32(at(FRAME_NODE));
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                let data = e
                    .call(
                        TEXTURE_EFFECT_DATA_00507830,
                        &args![shader, node, first, second],
                    )
                    .u32();
                e.call(NI_POINTER_ASSIGN_0066B0D0, &args![tex_effect_data, data]);
                if e.call(WORD_AT_0_00559450, &args![tex_effect_data]).u32() != 0 {
                    let data = e.call(WORD_AT_0_00559450, &args![tex_effect_data]).u32();
                    let target = e.get(this, MagicShaderHitEffect::pTarget);
                    if e.call(WORD_AT_0_00559450, &args![data.wrapping_add(0x6c)])
                        .u32()
                        != 0
                        && target != 0
                        && e.vcall(target, 0x100, &args![]).bool()
                    {
                        let target = e.get(this, MagicShaderHitEffect::pTarget);
                        let alpha = e.call(ACTOR_GET_ALPHA_MULT, &args![target]).st0;
                        let limit: f64 = e.global(ALPHA_LIMIT_0106D508);
                        // FCOMP; TEST AH,0x41; JNZ: only above the limit.
                        if alpha > limit {
                            let lower = x87_quiet_bits(e.mem.u32(LOWER_ALPHA_0106682C));
                            let target = e.get(this, MagicShaderHitEffect::pTarget);
                            e.call(ACTOR_SET_ALPHA_MULT, &args![target, lower]);
                            let target = e.get(this, MagicShaderHitEffect::pTarget);
                            e.call(ACTOR_UPDATE_ALPHA, &args![target]);
                        }
                    }
                }
                e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![texture_b]);
                e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![texture_a]);
            }
            let data = e.call(WORD_AT_0_00559450, &args![tex_effect_data]).u32();
            let node = e.mem.u32(at(FRAME_NODE));
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            e.call(START_TEXTURE_SHADER, &args![shader, node, data]);
        }
        let first_property = |e: &mut Engine| {
            let slot = e.call(ARRAY_AT_006A7AD0, &args![particles, 0u32]).u32();
            e.call(WORD_AT_0_00559450, &args![slot]).u32()
        };
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        if restart == 0
            && e.call(WORD_AT_8_0044DDC0, &args![objects]).u32() == 0
            && shader != 0
            && e.call(WORD_AT_10C_005A29B0, &args![shader]).u32() != 0
            && e.call(WORD_AT_8_0044DDC0, &args![particles]).u32() != 0
            && first_property(e) != 0
        {
            let property = first_property(e);
            let list = e.call(PARTICLE_OBJECTS_00820B50, &args![property]).u32();
            if e.call(COUNT_00658930, &args![list]).u32() != 0 {
                let weighted = at(FRAME_WEIGHTED);
                e.call(WEIGHTED_RANDOM_CONSTRUCT_00822440, &args![weighted]);
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                let source = e.call(WORD_AT_10C_005A29B0, &args![shader]).u32();
                let mut node = e.call(LIST_HEAD_00409140, &args![source]).u32();
                while node != 0 && !e.call(NODE_EMPTY_008256D0, &args![node]).bool() {
                    let entry = e.call(SELF_006815C0, &args![node]).u32();
                    let name = e.mem.u32(e.mem.u32(entry).wrapping_add(4));
                    let loader = e.mem.u32(MODEL_LOADER_011C3B3C);
                    load_model(e, loader, name);
                    let entry = e.call(SELF_006815C0, &args![node]).u32();
                    let kind = e.mem.u8(e.mem.u32(entry)) as i8 as i32;
                    let entry = e.call(SELF_006815C0, &args![node]).u32();
                    let name = e.mem.u32(e.mem.u32(entry).wrapping_add(4));
                    e.call(ADD_WEIGHTED_RESULT, &args![weighted, name, kind]);
                    node = e.call(WORD_AT_4_00726070, &args![node]).u32();
                }
                let property = first_property(e);
                let mut index = 0u32;
                loop {
                    let list = e.call(PARTICLE_OBJECTS_00820B50, &args![property]).u32();
                    if index >= e.call(COUNT_00658930, &args![list]).u32() {
                        break;
                    }
                    let list = e.call(PARTICLE_OBJECTS_00820B50, &args![property]).u32();
                    let element = e.call(ELEMENT_00877A30, &args![list, index]).u32();
                    let object = e.call(WORD_AT_0_00559450, &args![element]).u32();
                    if e.vcall(object, 0xc, &args![]).u32() != 0 {
                        let model = at(FRAME_MODEL);
                        e.call(NI_POINTER_FROM_00633C90, &args![model, 0u32]);
                        let pick = e
                            .call(WEIGHTED_RANDOM_PICK_00822650, &args![weighted])
                            .u32();
                        let loader = e.mem.u32(MODEL_LOADER_011C3B3C);
                        e.call(FIND_MODEL, &args![loader, pick, model]);
                        let found = e.call(WORD_AT_0_00559450, &args![model]).u32();
                        if found != 0 && e.call(MODEL_ROOT_0043B230, &args![found]).u32() != 0 {
                            let cloning = at(FRAME_CLONING);
                            e.call(
                                CLONING_PROCESS_CONSTRUCT_004AD050,
                                &args![cloning, 1.0f32.to_bits()],
                            );
                            let found = e.call(WORD_AT_0_00559450, &args![model]).u32();
                            let root = e.call(MODEL_ROOT_0043B230, &args![found]).u32();
                            let clone = e.call(NI_OBJECT_CLONE, &args![root, cloning]).u32();
                            if e.call(QUEUE_ATTACH_TEST_008C7AA0, &[]).bool() {
                                let parent = e.vcall(object, 0xc, &args![]).u32();
                                let queue = e.call(TASK_QUEUE_004537B0, &[]).u32();
                                e.call(QUEUE_NODE_ATTACH, &args![queue, clone, parent, 1u32, 0u32]);
                            } else {
                                let parent = e.vcall(object, 0xc, &args![]).u32();
                                e.vcall(parent, 0xdc, &args![clone, 1u32]);
                            }
                            e.set(this, MagicShaderHitEffect::fAddonAlpha, 0.0f32.to_bits());
                            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                            let value = e.call(SHADER_VALUE_00567470, &args![shader]).st0;
                            let value = x87_store32(value);
                            let factor = x87_quiet_bits(e.mem.u32(ADDON_SCALE_01013EA4));
                            let scale = e.call(FLOAT_HELPER_00404010, &args![factor, value]).st0;
                            e.set(this, MagicShaderHitEffect::fAddonScale, x87_store32(scale));
                            let scale =
                                x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAddonScale));
                            e.call(SET_SCALE_00440490, &args![clone, scale]);
                            e.call(START_ANIMATIONS, &args![clone]);
                            let pointer = at(FRAME_CLONE);
                            e.call(NI_POINTER_FROM_00633C90, &args![pointer, clone]);
                            e.call(ARRAY_APPEND_006C61B0, &args![objects, pointer]);
                            e.call(NI_POINTER_DESTRUCT_0045CEC0, &args![pointer]);
                            e.call(CLONING_PROCESS_DESTRUCT_004AD270, &args![cloning]);
                        }
                        e.call(POINTER_RELEASE_0040C110, &args![model]);
                    }
                    index += 1;
                }
                e.call(WEIGHTED_RANDOM_DESTRUCT_008224B0, &args![weighted]);
            }
        }
        e.call(WORD_AT_8_0044DDC0, &args![particles]).u32() != 0
            || e.call(WORD_AT_0_00559450, &args![tex_effect_data]).u32() != 0
    })
}

// Translated from 00820c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the hit effect's attach point (`thiscall`, five words: the
/// addresses of two flag bytes, a float and two words). Two locals the code sets to 0 and
/// never changes select branches (the compiler kept them). The target cast to an actor
/// (`__RTDynamicCast` with `011841cc`/`011846d4`) gives the biped: for the player
/// `PlayerCharacter::GetBiped` (with 1 when `b3rdPerson` is 0) unless `00490f80`
/// answers and the player's active hit effect (`00820bf0`) is this one; for another actor its
/// slot +0x1e8. A weapon enchantment (or the first constant local) with a target that answers
/// slot +0x100 hands the target's saved acquire object (`008d8520`) to its slot +0x148. When
/// the second constant local is set (never), `pWornObject` comes from the active effect
/// (the word at `+0x18`, cast with `011a0d1c`/`011a10ac`, `006733e0`, its word at `+8`) and
/// the node is the biped's slot for it (`004aaf30`, `+8`). Otherwise the node is the
/// target's (slot +0x1d0); a target answering slot +0x100 sets the first flag (and for the
/// player takes the player's node `00950bb0`); any other target that casts (`007af430`,
/// `01183108`/`01183998`) with a node sets the second flag, moves the node to the origin
/// when its bound's radius (`0043d450`, `0084d030`) is not above the `double` at `01012060`
/// (`0043d410` with 0.0, 0, 0; `00a59c60`), and stores in the float the radius plus the
/// length (`00457990`) of the bound's offset from the camera (`0045bb80`, `00439ef0`). Both
/// words get the node.
pub fn fn_00820c10(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    first_flag: u32,
    second_flag: u32,
    radius: u32,
    node_out: u32,
    node_out_2: u32,
) {
    let base = this.addr();
    let first_constant = 0u8;
    let second_constant = 0u8;
    let mut node = 0u32;
    let target = e.get(this, MagicShaderHitEffect::pTarget);
    let actor = e
        .call(
            RT_DYNAMIC_CAST,
            &args![target, 0u32, RTTI_011841CC, RTTI_011846D4, 0u32],
        )
        .u32();
    let mut biped = 0u32;
    if actor != 0 {
        if actor != e.mem.u32(PLAYER) {
            biped = e.vcall(actor, 0x1e8, &args![]).u32();
        } else {
            let player = e.mem.u32(PLAYER);
            let uses_this = e.call(ANIMATION_00490F80, &args![player]).u32() != 0 && {
                let player = e.mem.u32(PLAYER);
                e.call(ACTIVE_HIT_EFFECT_00820BF0, &args![player]).u32() == base
            };
            // When the player's active hit effect is this one, the exe only sets a local
            // flag that nothing reads.
            if !uses_this {
                let not_third_person =
                    u32::from(e.get(this, MagicShaderHitEffect::b3rdPerson) == 0);
                let player = e.mem.u32(PLAYER);
                biped = e
                    .call(PLAYER_GET_BIPED, &args![player, not_third_person])
                    .u32();
            }
        }
    }
    if e.get(this, MagicShaderHitEffect::bWeaponEnchantment) != 0 || first_constant != 0 {
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        if target != 0 && e.vcall(target, 0x100, &args![]).bool() {
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            let acquired = e.call(SAVED_ACQUIRE_OBJECT_008D8520, &args![target]).u32();
            if acquired != 0 {
                e.vcall(acquired, 0x148, &args![]);
            }
        }
    } else if second_constant != 0 {
        let effect = e.call(WORD_AT_18_009611E0, &args![this]).u32();
        let process = e
            .call(
                RT_DYNAMIC_CAST,
                &args![effect, 0u32, RTTI_011A0D1C, RTTI_011A10AC, 0u32],
            )
            .u32();
        if process != 0 {
            let location = e.call(LOCATION_006733E0, &args![process]).u32();
            if location != 0 {
                let worn = e.call(WORD_AT_8_0044DDC0, &args![location]).u32();
                e.set(this, MagicShaderHitEffect::pWornObject, worn);
            }
        }
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        if target != 0
            && e.vcall(target, 0x100, &args![]).bool()
            && e.get(this, MagicShaderHitEffect::pWornObject) != 0
            && biped != 0
        {
            let worn = e.get(this, MagicShaderHitEffect::pWornObject);
            let slot = e.call(BIPED_SLOT_004AAF30, &args![biped, worn]).u32();
            if slot != 0 {
                node = e.mem.u32(slot.wrapping_add(8));
            }
        }
    } else {
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        if target != 0 {
            node = e.vcall(target, 0x1d0, &args![]).u32();
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            if e.vcall(target, 0x100, &args![]).bool() {
                if e.get(this, MagicShaderHitEffect::pTarget) == e.mem.u32(PLAYER) {
                    let not_third_person =
                        u32::from(e.get(this, MagicShaderHitEffect::b3rdPerson) == 0);
                    let player = e.mem.u32(PLAYER);
                    node = e
                        .call(PLAYER_NODE_00950BB0, &args![player, not_third_person])
                        .u32();
                }
                e.mem.set_u8(first_flag, 1);
            } else {
                let target = e.get(this, MagicShaderHitEffect::pTarget);
                let form = e.call(FORM_007AF430, &args![target]).u32();
                let cast = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, RTTI_01183108, RTTI_01183998, 0u32],
                    )
                    .u32();
                if cast != 0 && node != 0 {
                    e.mem.set_u8(second_flag, 1);
                    let bound = e.call(WORLD_BOUND_0043D450, &args![node]).u32();
                    let size = e.call(BOUND_RADIUS_0084D030, &args![bound]).st0;
                    let zero: f64 = e.global(ZERO_01012060);
                    // FCOMP; TEST AH,0x41; JP: skipped when above (or unordered).
                    if size <= zero {
                        e.with_stack(0xc, |e, origin| {
                            e.call(
                                POINT_SET_0043D410,
                                &args![origin, 0.0f32.to_bits(), 0u32, 0u32],
                            );
                            e.call(SET_TRANSLATE_00A59C60, &args![node, origin]);
                        });
                    }
                    let camera = e.call(CAMERA_POSITION_0045BB80, &args![node]).u32();
                    let distance = e.with_stack(0xc, |e, offset| {
                        let bound = e.call(WORLD_BOUND_0043D450, &args![node]).u32();
                        let centre = e.call(SELF_006815C0, &args![bound]).u32();
                        e.call(POINT_SUBTRACT_00439EF0, &args![centre, offset, camera]);
                        e.call(POINT_LENGTH_00457990, &args![offset]).st0
                    });
                    let bound = e.call(WORLD_BOUND_0043D450, &args![node]).u32();
                    let size = e.call(BOUND_RADIUS_0084D030, &args![bound]).st0;
                    e.mem.set_u32(radius, x87_store32(x87_add(size, distance)));
                }
            }
        }
    }
    e.mem.set_u32(node_out, node);
    e.mem.set_u32(node_out_2, node);
}

// Translated from 00821600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the `float` argument at `+0x78` (`FLD`/`FSTP`: a
/// signalling NaN is stored quiet).
pub fn fn_00821600(e: &mut Engine, this: Ptr, value: u32) {
    e.mem
        .set_u32(this.addr().wrapping_add(0x78), x87_quiet_bits(value));
}

// Translated from 00821640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `float` at `+0x134`, in `ST0`.
pub fn fn_00821640(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x134)))
}

// Translated from 00821660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `float` at `+0x138`, in `ST0`.
pub fn fn_00821660(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x138)))
}

// Translated from 00821680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `float` at `+0x140`, in `ST0`.
pub fn fn_00821680(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x140)))
}

// Translated from 00822510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the word at `this` equals the argument (1 or 0).
pub fn fn_00822510(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    u32::from(e.mem.u32(this.addr()) == value)
}

// Translated from 00822530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the particle array's constructor: the vtable `0107b824`, then
/// `00822860(0, 0)`. Answers `this`.
pub fn fn_00822530(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), PARTICLE_ARRAY_VTABLE);
    e.call(ARRAY_INIT_00822860, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00822560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<ParticleShaderProperty>,1024>::~BSSimpleArray` (engine map; the
/// body matches): the vtable `0107b824`, then the clear with the buffer freed
/// ([`fn_006c6200`] with 1).
pub fn bs_simple_array_particle_shader_property_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), PARTICLE_ARRAY_VTABLE);
    fn_006c6200(e, this, 1);
}

// Translated from 00822580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the add-on array's constructor: the vtable `0107b838`, then
/// `00822860(0, 0)`. Answers `this`.
pub fn fn_00822580(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), OBJECT_ARRAY_VTABLE);
    e.call(ARRAY_INIT_00822860, &args![this, 0u32, 0u32]);
    this
}

// Translated from 008225b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<NiAVObject>,1024>::~BSSimpleArray` (engine map; the body
/// matches): the vtable `0107b838`, then the clear with the buffer freed ([`fn_006c6200`],
/// the identical code the compiler shares, with 1).
pub fn bs_simple_array_ni_av_object_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), OBJECT_ARRAY_VTABLE);
    fn_006c6200(e, this, 1);
}

// Translated from 00822630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor of the array whose vtable `0107b84c`
/// [`fn_00822600`] installs (`BSSimpleArray<char *, 1024>` by its RTTI name): the vtable,
/// then `008454f0(1)`.
pub fn fn_00822630(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), STRING_ARRAY_VTABLE);
    e.call(STRING_ARRAY_CLEAR_008454F0, &args![this, 1u32]);
}

/// `MagicHitEffect`'s methods this class calls: `InitLoadGame` (`0081be50`,
/// two words), `Update` (`0081b860`, one `float`), `GetSaveSize`,
/// `SaveGame`, `LoadGame` (`0081bd30`, `0081bd80`, `0081bde0`, two words),
/// `SaveGame_ov2`, `LoadGame_ov2` (`0081be80`, `0081bed0`, one word) and
/// `0081bc70` (one word); the base's `FinishInitLoadGame` (`008d05e0`, three
/// words) and `FinishLoadGame` (`009dace0`, one word) as the class calls
/// them.
const HIT_EFFECT_INIT_LOAD_GAME: u32 = 0x0081_be50;
const HIT_EFFECT_UPDATE: u32 = 0x0081_b860;
const HIT_EFFECT_GET_SAVE_SIZE: u32 = 0x0081_bd30;
const HIT_EFFECT_SAVE_GAME: u32 = 0x0081_bd80;
const HIT_EFFECT_LOAD_GAME: u32 = 0x0081_bde0;
const HIT_EFFECT_SAVE_GAME_OV2: u32 = 0x0081_be80;
const HIT_EFFECT_LOAD_GAME_OV2: u32 = 0x0081_bed0;
const HIT_EFFECT_0081BC70: u32 = 0x0081_bc70;
const HIT_EFFECT_FINISH_INIT_LOAD_GAME: u32 = 0x008d_05e0;
const HIT_EFFECT_FINISH_LOAD_GAME: u32 = 0x009d_ace0;
/// The largest finite `float` (`01016970`) the second constructor stores for
/// a negative lifetime.
const LONGEST_LIFETIME: u32 = 0x0101_6970;
/// `0080d0d0` (no arguments): the effect shader the alpha code skips.
const SKIPPED_SHADER_0080D0D0: u32 = 0x0080_d0d0;
/// `0043b300` (`cdecl`, two words) with `011f9140`, and `006d2c40`
/// (`thiscall`, `ST0`): the 3D's value the alpha scale takes; otherwise
/// the target's slot +0x420.
const NODE_TEST_0043B300: u32 = 0x0043_b300;
const NODE_TEST_ARGUMENT_011F9140: u32 = 0x011f_9140;
const NODE_VALUE_006D2C40: u32 = 0x006d_2c40;
/// `006a78f0` (`thiscall`, `ST0`: a particle property's rate), `00507da0`
/// (`thiscall` on the shader, four words), `0040ebd0` (`cdecl`, two
/// floats: the smaller), `004fd380` (`thiscall`: the property's live
/// count; the engine map's `TESWorldSpace::GetParentWorldSimple` is a
/// folded body), `008133b0` / `00567470` / `00821680` and `00821640` /
/// `00821660` / `008216a0` (the shader's fade times, `ST0`), `00507e30`
/// (`thiscall` on the shader, five words: the texture effect's update),
/// `BSShaderUtil::RecursiveSetPropertyFadeAlpha` (`00b6bb30`, `cdecl`, two
/// words) and the `double` -1.0 (`0101a6b0`).
const PARTICLE_RATE_006A78F0: u32 = 0x006a_78f0;
const SHADER_PARTICLES_00507DA0: u32 = 0x0050_7da0;
const MIN_0040EBD0: u32 = 0x0040_ebd0;
const PARTICLE_LIVE_004FD380: u32 = 0x004f_d380;
const SHADER_FADE_IN_008133B0: u32 = 0x0081_33b0;
const SHADER_TEXTURE_UPDATE_00507E30: u32 = 0x0050_7e30;
const RECURSIVE_SET_PROPERTY_FADE_ALPHA: u32 = 0x00b6_bb30;
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `009a7c40` and `00759450` (`thiscall` on the shader, `ST0`) and
/// `00ba6cf0` (`thiscall` on a particle property, one `float`).
const SHADER_TIME_009A7C40: u32 = 0x009a_7c40;
const SHADER_TIME_00759450: u32 = 0x0075_9450;
const PARTICLE_SET_TIME_00BA6CF0: u32 = 0x00ba_6cf0;
/// `PathingLocation::GetWorldspace` (the engine map's name for `00441110`:
/// the hit effect's target word), `00461130` (a node's rotation),
/// `NiMatrix3::Inverse` (`004b45b0`, one word), `NiMatrix3::operatorP`
/// (`0043f8d0`, two words), `004b4500` (two words: a matrix times a point)
/// and `00653270` (`cdecl`) with `011f4428`.
const TARGET_00441110: u32 = 0x0044_1110;
const NODE_ROTATION_00461130: u32 = 0x0046_1130;
const MATRIX_INVERSE: u32 = 0x004b_45b0;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const MATRIX_TIMES_POINT_004B4500: u32 = 0x004b_4500;
const NODE_LOOKUP_00653270: u32 = 0x0065_3270;
const NODE_LOOKUP_ARGUMENT_011F4428: u32 = 0x011f_4428;
/// `00440460` (`thiscall`, one word: set a translation).
const SET_TRANSLATE_00440460: u32 = 0x0044_0460;
/// The save/load game object (`011de45c`), its version (`008df040`), write
/// and read (`008579b0`, `008579e0`) and `TESSaveLoadGame::SaveNumericID` /
/// `LoadNumericID` (`00857a10`, `00857aa0`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;
const SAVE_WRITE: u32 = 0x0085_79b0;
const LOAD_READ: u32 = 0x0085_79e0;
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// `004839c0` (`cdecl`: the form of an id) and the type descriptors
/// `01183028` (`TESForm`), `01183918` (`TESEffectShader`).
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_EFFECT_SHADER: u32 = 0x0118_3918;
/// `008d6f30` (`thiscall`: the reference's cell) and `00450ff0`
/// (`thiscall` on the cell, a byte).
const REFERENCE_CELL_008D6F30: u32 = 0x008d_6f30;
const CELL_TEST_00450FF0: u32 = 0x0045_0ff0;
/// The save buffer's `00865e50` (three words: write) and
/// `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`, two words); the load
/// buffer's `00864980` (two words: read) and
/// `BGSLoadGameBuffer::LoadFormID` (`008648a0`).
const BUFFER_WRITE_00865E50: u32 = 0x0086_5e50;
const BUFFER_SAVE_FORM_ID: u32 = 0x0086_5df0;
const BUFFER_READ_00864980: u32 = 0x0086_4980;
const BUFFER_LOAD_FORM_ID: u32 = 0x0086_48a0;
/// The weighted random's members: `00438660` / `00438690` (`thiscall`:
/// construct / destroy the array at +0x10) and `005a00a0` (`cdecl`, the
/// word +0x20: the roll).
const WEIGHTS_CONSTRUCT_00438660: u32 = 0x0043_8660;
const WEIGHTS_DESTRUCT_00438690: u32 = 0x0043_8690;
const ROLL_005A00A0: u32 = 0x005a_00a0;
/// The free `00401030` (`cdecl`, one word).
const FREE_00401030: u32 = 0x0040_1030;

/// Stores `value` at `address` as a `float` (`FSTP`).
fn store_float(e: &mut Engine, address: u32, value: f64) {
    e.mem.set_u32(address, x87_store32(value));
}

/// The item `index` of the array at `this + array` held through its
/// `NiPointer` (`006a7ad0`, `00559450`).
fn array_item(e: &mut Engine, this: u32, array: u32, index: u32) -> u32 {
    let slot = e
        .call(ARRAY_AT_006A7AD0, &args![this.wrapping_add(array), index])
        .u32();
    e.call(WORD_AT_0_00559450, &args![slot]).u32()
}

/// The count of the array at `this + array` (`0044ddc0`).
fn array_count(e: &mut Engine, this: u32, array: u32) -> u32 {
    e.call(WORD_AT_8_0044DDC0, &args![this.wrapping_add(array)])
        .u32()
}

/// The object held by the `NiPointer` at `this + offset` (`00559450`).
fn held(e: &mut Engine, this: u32, offset: u32) -> u32 {
    e.call(WORD_AT_0_00559450, &args![this.wrapping_add(offset)])
        .u32()
}

// Translated from 0081d390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::InitLoadGame` (engine map; `thiscall`, `RET 8`):
/// `MagicHitEffect::InitLoadGame` with the two words.
pub fn magic_shader_hit_effect_init_load_game(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(HIT_EFFECT_INIT_LOAD_GAME, &args![this, first, second]);
}

// Translated from 0081f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second `MagicShaderHitEffect` constructor (`thiscall`, `RET 0xC`: the
/// target, the effect shader, the lifetime): as `0081f350` but with the
/// shader given (`pEffectData`), `MagicHitEffect`'s constructor with the
/// target and 0, and the lifetime stored (a negative one becomes the
/// largest `float`, `01016970`). Returns `this`.
// Translation incomplete: 0081f580 - the FS:[0] exception frame (unwinding the members on a throw) is not translated
pub fn fn_0081f580(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    target: u32,
    shader: u32,
    lifetime: f32,
) -> Ptr<MagicShaderHitEffect> {
    let base = this.addr();
    e.call(MAGIC_HIT_EFFECT_CONSTRUCTOR, &args![this, target, 0u32]);
    e.mem.set_u32(base, MAGIC_SHADER_HIT_EFFECT_VTABLE);
    fn_00822530(e, Ptr::new(base.wrapping_add(PARTICLE_PROPERTY_ARRAY)));
    e.call(
        NI_POINTER_FROM_00633C90,
        &args![base.wrapping_add(PARTICLE_NODE), 0u32],
    );
    fn_00822580(e, Ptr::new(base.wrapping_add(ADD_ON_OBJECTS)));
    e.call(
        NI_POINTER_FROM_00633C90,
        &args![base.wrapping_add(TEX_EFFECT_DATA), 0u32],
    );
    e.set(this, MagicShaderHitEffect::pEffectData, shader);
    e.set(this, MagicShaderHitEffect::fAlphaTimer, 0);
    e.call(
        NI_POINTER_ASSIGN_0066B0D0,
        &args![base.wrapping_add(TEX_EFFECT_DATA), 0u32],
    );
    e.call(
        NI_POINTER_ASSIGN_0066B0D0,
        &args![base.wrapping_add(PARTICLE_NODE), 0u32],
    );
    e.set(this, MagicShaderHitEffect::bWeaponEnchantment, 0);
    e.set(this, MagicShaderHitEffect::pWornObject, 0);
    e.set(this, MagicShaderHitEffect::fAddonAlpha, 1.0f32.to_bits());
    e.set(this, MagicShaderHitEffect::fAddonScale, 1.0f32.to_bits());
    let player = e.mem.u32(PLAYER);
    if e.get(this, MagicShaderHitEffect::pTarget) == player {
        let has_node = e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32() != 0;
        let third_person = has_node && {
            let player = e.mem.u32(PLAYER);
            let node = e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32();
            !e.call(QUERY_00456610, &args![node]).bool()
        };
        e.set(
            this,
            MagicShaderHitEffect::b3rdPerson,
            u8::from(third_person),
        );
    } else {
        e.set(this, MagicShaderHitEffect::b3rdPerson, 0);
    }
    // FCOMP with 0.0, TEST AH,5, JP: the largest float only below 0.0.
    let stored = if x87_load32(lifetime.to_bits()) < e.global::<f64>(ZERO_01012060) {
        x87_quiet_bits(e.global::<u32>(LONGEST_LIFETIME))
    } else {
        x87_quiet_bits(lifetime.to_bits())
    };
    e.mem.set_u32(base.wrapping_add(8), stored);
    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
    walk_addon_models(e, shader, |e, loader, name| {
        load_model(e, loader, name);
    });
    this
}

// Translated from 00820b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): `this + 0x11C`.
pub fn fn_00820b50(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x11c)
}

// Translated from 00820b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a `float`): the word +0x64 becomes 2 and
/// the `float` +0x68 the value (through `FLD`/`FSTP`).
pub fn fn_00820b70(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(this.addr().wrapping_add(0x64), 2);
    e.mem.set_u32(
        this.addr().wrapping_add(0x68),
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 00820b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the word +0x64 becomes 1.
pub fn fn_00820b90(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr().wrapping_add(0x64), 1);
}

// Translated from 00820bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the word +0x64 becomes 0.
pub fn fn_00820bb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr().wrapping_add(0x64), 0);
}

// Translated from 00820bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a byte): stores the byte at +0x6C.
pub fn fn_00820bd0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x6c), value);
}

// Translated from 00820bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the word at +0x6A4 (the code reads it from the
/// player as the active hit effect).
pub fn fn_00820bf0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x6a4))
}

// Translated from 00820f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::Update` (engine map; `thiscall`, `RET 4`: the
/// elapsed seconds; answers a byte). When `MagicHitEffect::Update` answers
/// false or the target has no 3D, slot +0xC8 runs and the answer is false.
/// Otherwise `fAlphaTimer` advances, slot +0xCC runs, and an alpha scale is
/// found (1.0; for an actor target whose shader is not `0080d0d0`'s: the
/// target's alpha above 0.99 is lowered once a texture effect has its +0x6C
/// object, and the scale is the 3D's `006d2c40` value or the target's slot
/// +0x420, divided by the target's alpha when that is above 0). With
/// particle properties and a particle node, the particles are stepped
/// (`00507da0` with their total rate, then each property's share,
/// `00821600`, `00821620`, `UpdateParticles`); with add-ons the add-on
/// alpha and scale fade (`0040ebd0`, `00404010`) and are applied
/// (`RecursiveSetPropertyFadeAlpha`, `00440490`). With a texture effect it
/// is updated (`00507e30`) and its +0x18 and +0x28 scaled. Any particle
/// still alive, positive fade or texture alpha keeps the effect; a finished
/// effect with nothing left releases an actor target's process (slot
/// +0x580 with 1, 1, 0), runs slot +0xC8 and answers false.
pub fn magic_shader_hit_effect_update(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    seconds: f32,
) -> u8 {
    let base = this.addr();
    let seconds = seconds.to_bits();
    let target = e.get(this, MagicShaderHitEffect::pTarget);
    let alive = e
        .call(HIT_EFFECT_UPDATE, &args![base, x87_quiet_bits(seconds)])
        .bool()
        && target != 0
        && e.vcall(target, 0x1d0, &[]).u32() != 0;
    if !alive {
        e.vcall(base, 0xc8, &[]);
        return 0;
    }
    let timer = x87_add(
        x87_load32(e.get(this, MagicShaderHitEffect::fAlphaTimer)),
        x87_m32(seconds),
    );
    e.set(this, MagicShaderHitEffect::fAlphaTimer, x87_store32(timer));
    let mut done = e.get(this, MagicShaderHitEffect::bFinished);
    e.vcall(base, 0xcc, &[]);
    let mut scale = 1.0f32.to_bits();
    let zero = e.global::<f64>(ZERO_01012060);
    let target = e.get(this, MagicShaderHitEffect::pTarget);
    if e.vcall(target, 0x100, &[]).bool()
        && e.get(this, MagicShaderHitEffect::pEffectData)
            != e.call(SKIPPED_SHADER_0080D0D0, &[]).u32()
    {
        if held(e, base, TEX_EFFECT_DATA) != 0 {
            let texture = held(e, base, TEX_EFFECT_DATA);
            let lowered = held(e, texture, 0x6c) != 0
                && {
                    let target = e.get(this, MagicShaderHitEffect::pTarget);
                    e.vcall(target, 0x100, &[]).bool()
                }
                && {
                    let target = e.get(this, MagicShaderHitEffect::pTarget);
                    let alpha = e.call(ACTOR_GET_ALPHA_MULT, &args![target]).st0;
                    alpha > e.global::<f64>(ALPHA_LIMIT_0106D508)
                };
            if lowered {
                let lower = x87_quiet_bits(e.global::<u32>(LOWER_ALPHA_0106682C));
                let target = e.get(this, MagicShaderHitEffect::pTarget);
                e.call(ACTOR_SET_ALPHA_MULT, &args![target, lower]);
                let target = e.get(this, MagicShaderHitEffect::pTarget);
                e.call(ACTOR_UPDATE_ALPHA, &args![target]);
            }
        }
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        let node = e.vcall(target, 0x1d0, &[]).u32();
        scale = if e
            .call(
                NODE_TEST_0043B300,
                &args![NODE_TEST_ARGUMENT_011F9140, node],
            )
            .bool()
        {
            x87_store32(e.call(NODE_VALUE_006D2C40, &args![node]).st0)
        } else {
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            x87_store32(e.vcall(target, 0x420, &[]).st0)
        };
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        let alpha = x87_store32(e.call(ACTOR_GET_ALPHA_MULT, &args![target]).st0);
        if x87_load32(alpha) > zero {
            scale = x87_store32(x87_div(x87_load32(scale), x87_m32(alpha)));
        }
    }
    if array_count(e, base, PARTICLE_PROPERTY_ARRAY) != 0
        && e.call(
            WORD_DIFFERS_0052AA80,
            &args![base.wrapping_add(PARTICLE_NODE), 0u32],
        )
        .bool()
    {
        let mut total = 0u32;
        let mut index = 0;
        while index < array_count(e, base, PARTICLE_PROPERTY_ARRAY) {
            let property = array_item(e, base, PARTICLE_PROPERTY_ARRAY, index);
            let rate = e.call(PARTICLE_RATE_006A78F0, &args![property]).st0;
            let step = e.call(PARTICLE_STEP_00BA52D0, &[]).u32();
            total = x87_store32(x87_add(x87_mul(f64::from(step), rate), x87_m32(total)));
            index += 1;
        }
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        let timer = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAlphaTimer));
        let finished = u32::from(e.get(this, MagicShaderHitEffect::bFinished));
        let mut remaining = x87_store32(
            e.call(
                SHADER_PARTICLES_00507DA0,
                &args![
                    shader,
                    x87_quiet_bits(total),
                    x87_quiet_bits(seconds),
                    timer,
                    finished
                ],
            )
            .st0,
        );
        if x87_load32(remaining) > zero {
            done = 0;
        }
        let mut index = 0;
        while index < array_count(e, base, PARTICLE_PROPERTY_ARRAY) {
            let step = e.call(PARTICLE_STEP_00BA52D0, &[]).u32();
            let step = x87_store32(f64::from(step));
            let part = x87_store32(
                e.call(MIN_0040EBD0, &args![x87_quiet_bits(remaining), step])
                    .st0,
            );
            remaining = x87_store32(x87_sub(x87_load32(remaining), x87_m32(part)));
            let property = array_item(e, base, PARTICLE_PROPERTY_ARRAY, index);
            let step = e.call(PARTICLE_STEP_00BA52D0, &[]).u32();
            let share = x87_store32(x87_div(x87_m32(part), f64::from(step)));
            fn_00821600(e, Ptr::new(property), x87_quiet_bits(share));
            fn_00821620(e, Ptr::new(property), f32::from_bits(x87_quiet_bits(scale)));
            let timer = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAlphaTimer));
            e.call(UPDATE_PARTICLES, &args![property, timer, 0u32, 1u32]);
            if e.call(PARTICLE_LIVE_004FD380, &args![property]).i32() > 0 {
                done = 0;
            }
            index += 1;
        }
        if array_count(e, base, ADD_ON_OBJECTS) != 0 {
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            let finished = e.get(this, MagicShaderHitEffect::bFinished) != 0;
            let alpha_time = if !finished {
                f64::from(fn_00821640(e, Ptr::new(shader)))
            } else {
                x87_mul(
                    f64::from(fn_00821660(e, Ptr::new(shader))),
                    e.global::<f64>(MINUS_ONE_DOUBLE),
                )
            };
            let step = x87_store32(x87_div(x87_load32(seconds), alpha_time));
            let alpha = x87_store32(x87_add(
                x87_load32(e.get(this, MagicShaderHitEffect::fAddonAlpha)),
                x87_m32(step),
            ));
            let alpha = fade_clamp(e, alpha);
            e.set(this, MagicShaderHitEffect::fAddonAlpha, alpha);
            if x87_load32(e.get(this, MagicShaderHitEffect::fAddonAlpha)) > zero {
                done = 0;
            }
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            let scale_time = if e.get(this, MagicShaderHitEffect::bFinished) == 0 {
                e.call(SHADER_FADE_IN_008133B0, &args![shader]).st0
            } else {
                x87_mul(
                    f64::from(fn_008216a0(e, Ptr::new(shader))),
                    e.global::<f64>(MINUS_ONE_DOUBLE),
                )
            };
            let step = x87_store32(x87_div(x87_load32(seconds), scale_time));
            let addon_scale = x87_store32(x87_add(
                x87_load32(e.get(this, MagicShaderHitEffect::fAddonScale)),
                x87_m32(step),
            ));
            let addon_scale = fade_clamp(e, addon_scale);
            e.set(this, MagicShaderHitEffect::fAddonScale, addon_scale);
            let one = 1.0f32.to_bits();
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            let full = if e.get(this, MagicShaderHitEffect::bFinished) == 0 {
                x87_store32(e.call(SHADER_VALUE_00567470, &args![shader]).st0)
            } else {
                x87_store32(f64::from(fn_00821680(e, Ptr::new(shader))))
            };
            let smallest = x87_quiet_bits(e.global::<u32>(ADDON_SCALE_01013EA4));
            let low = x87_store32(e.call(FLOAT_HELPER_00404010, &args![smallest, full]).st0);
            let low = x87_quiet_bits(low);
            let size = x87_store32(x87_add(
                x87_mul(
                    x87_sub(x87_load32(one), x87_m32(low)),
                    x87_m32(e.get(this, MagicShaderHitEffect::fAddonScale)),
                ),
                x87_m32(low),
            ));
            let mut index = 0;
            while index < array_count(e, base, ADD_ON_OBJECTS) {
                let object = array_item(e, base, ADD_ON_OBJECTS, index);
                let alpha = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAddonAlpha));
                let node = e.vcall(object, 0xc, &[]).u32();
                e.call(RECURSIVE_SET_PROPERTY_FADE_ALPHA, &args![node, alpha]);
                let object = array_item(e, base, ADD_ON_OBJECTS, index);
                e.call(SET_SCALE_00440490, &args![object, x87_quiet_bits(size)]);
                index += 1;
            }
        }
    }
    if held(e, base, TEX_EFFECT_DATA) != 0 {
        let finished = u32::from(e.get(this, MagicShaderHitEffect::bFinished));
        let age = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAge));
        let timer = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAlphaTimer));
        let texture = held(e, base, TEX_EFFECT_DATA);
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        e.call(
            SHADER_TEXTURE_UPDATE_00507E30,
            &args![
                shader,
                texture,
                x87_quiet_bits(seconds),
                timer,
                age,
                finished
            ],
        );
        for offset in [0x18u32, 0x28] {
            let field = held(e, base, TEX_EFFECT_DATA).wrapping_add(offset);
            let value = x87_mul(x87_load32(e.mem.u32(field)), x87_m32(scale));
            store_float(e, field, value);
        }
        let texture = held(e, base, TEX_EFFECT_DATA);
        let fading = x87_load32(e.mem.u32(texture.wrapping_add(0x48))) > zero || {
            let texture = held(e, base, TEX_EFFECT_DATA);
            x87_load32(e.mem.u32(texture.wrapping_add(0x38))) > zero
        };
        if fading {
            done = 0;
        }
    }
    if e.get(this, MagicShaderHitEffect::bFinished) != 0 && done != 0 {
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        if e.vcall(target, 0x100, &[]).bool()
            && e.call(SAVED_ACQUIRE_OBJECT_008D8520, &args![target]).u32() != 0
        {
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            let process = e.call(SAVED_ACQUIRE_OBJECT_008D8520, &args![target]).u32();
            e.vcall(process, 0x580, &args![1u32, 1u32, 0u32]);
        }
        e.vcall(base, 0xc8, &[]);
        return 0;
    }
    1
}

/// `max(0, min(1, value))` as `00820f30` computes the add-on fades: the
/// value (through `FLD`/`FSTP`) and 1.0 to `0040ebd0`, its answer stored
/// as a `float` with 0.0 to `00404010`, that answer stored as a `float`.
fn fade_clamp(e: &mut Engine, value: u32) -> u32 {
    let smaller = e
        .call(
            MIN_0040EBD0,
            &args![1.0f32.to_bits(), x87_quiet_bits(value)],
        )
        .st0;
    let smaller = x87_store32(smaller);
    x87_store32(e.call(FLOAT_HELPER_00404010, &args![0u32, smaller]).st0)
}

// Translated from 00821620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a `float`): stores the value at +0x130
/// (through `FLD`/`FSTP`).
pub fn fn_00821620(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0x130),
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 008216a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the `float` at +0x148, in `ST0`.
pub fn fn_008216a0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x148)))
}

// Translated from 008216c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::ResetAlphaTimer` (engine map; `thiscall`): the
/// texture effect's larger alpha (`00404010` of its +0x38 and +0x48, 0.0
/// without one; a weapon enchantment clears both and uses 0.0) sets
/// `fAlphaTimer`: the shader's `00759450` time scaled by that alpha over
/// the shader's `009a7c40` alpha when it is below it, otherwise the
/// `00759450` time itself; each particle property gets the timer
/// (`00ba6cf0`).
pub fn magic_shader_hit_effect_reset_alpha_timer(e: &mut Engine, this: Ptr<MagicShaderHitEffect>) {
    let base = this.addr();
    let mut alpha = 0u32;
    if held(e, base, TEX_EFFECT_DATA) != 0 {
        let texture = held(e, base, TEX_EFFECT_DATA);
        let second = x87_quiet_bits(e.mem.u32(texture.wrapping_add(0x48)));
        let texture = held(e, base, TEX_EFFECT_DATA);
        let first = x87_quiet_bits(e.mem.u32(texture.wrapping_add(0x38)));
        alpha = x87_store32(e.call(FLOAT_HELPER_00404010, &args![first, second]).st0);
    }
    if e.get(this, MagicShaderHitEffect::bWeaponEnchantment) != 0 {
        alpha = 0;
        let texture = held(e, base, TEX_EFFECT_DATA);
        e.mem.set_u32(texture.wrapping_add(0x38), 0);
        let texture = held(e, base, TEX_EFFECT_DATA);
        e.mem.set_u32(texture.wrapping_add(0x48), 0);
    }
    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
    let full = e.call(SHADER_TIME_009A7C40, &args![shader]).st0;
    // FCOMPP; TEST AH,5; JP: the scaled time only when alpha < full.
    let timer = if x87_load32(alpha) < full {
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        let time = e.call(SHADER_TIME_00759450, &args![shader]).st0;
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        let full = e.call(SHADER_TIME_009A7C40, &args![shader]).st0;
        x87_store32(x87_mul(x87_div(x87_m32(alpha), full), time))
    } else {
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        x87_store32(e.call(SHADER_TIME_00759450, &args![shader]).st0)
    };
    e.set(this, MagicShaderHitEffect::fAlphaTimer, timer);
    let mut index = 0;
    while index < array_count(e, base, PARTICLE_PROPERTY_ARRAY) {
        let timer = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAlphaTimer));
        let property = array_item(e, base, PARTICLE_PROPERTY_ARRAY, index);
        e.call(PARTICLE_SET_TIME_00BA6CF0, &args![property, timer]);
        index += 1;
    }
}

// Translated from 008217e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::UpdatePosition` (engine map; `thiscall`). For the
/// player's effect (the target `00441110` is the player) it notes whether
/// the player's active hit effect (`00820bf0`) is this one and whether the
/// player is in first person (the player's 3D `00950bb0(0)` passes
/// `00456610`). The 3D is the player's (`00950bb0(0)`), the target's (slot
/// +0x1D0), or for a weapon enchantment the attach point (`00820c10`).
/// With a particle node and 3D (and not the player's active effect): the
/// rotation (the 3D's for a weapon enchantment, else `011a9448`'s nine
/// words) and position are given to the particle node; in first person
/// with a first-person 3D (`00950bb0(1)`), the node is attached under
/// `00653270`'s node there (its slot +0xDC with 1) when it is not its parent
/// yet, and placed relative to its parent (inverse rotation, the offset of
/// the 3D from the parent) and updated at time 0. Finally `0081bc70` gets
/// the particle node, or 0 in first person or for the player's active
/// effect.
///
/// The stack frame is modelled as a block of the frame's size, the locals
/// at their `EBP` offsets.
pub fn magic_shader_hit_effect_update_position(e: &mut Engine, this: Ptr<MagicShaderHitEffect>) {
    let base = this.addr();
    e.with_stack(0xe8, |e, frame| {
        let ebp = frame.addr().wrapping_add(0xe8);
        let local = |offset: u32| ebp.wrapping_sub(offset);
        let target = e.call(TARGET_00441110, &args![base]).u32();
        let is_player = target == e.mem.u32(PLAYER);
        let own = is_player && e.mem.u32(PLAYER) != 0 && {
            let player = e.mem.u32(PLAYER);
            fn_00820bf0(e, Ptr::new(player)) == base
        };
        let first_person = is_player
            && e.mem.u32(PLAYER) != 0
            && {
                let player = e.mem.u32(PLAYER);
                e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32() != 0
            }
            && {
                let player = e.mem.u32(PLAYER);
                let node = e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32();
                e.call(QUERY_00456610, &args![node]).bool()
            };
        let node = if e.get(this, MagicShaderHitEffect::bWeaponEnchantment) == 0 {
            if is_player {
                let player = e.mem.u32(PLAYER);
                e.call(PLAYER_NODE_00950BB0, &args![player, 0u32]).u32()
            } else {
                let target = e.get(this, MagicShaderHitEffect::pTarget);
                e.vcall(target, 0x1d0, &[]).u32()
            }
        } else {
            fn_00820c10(
                e,
                this,
                local(0x11),
                local(0x11),
                local(0x10),
                local(0x18),
                local(0x1c),
            );
            e.mem.u32(local(0x1c))
        };
        let particle_node = base.wrapping_add(PARTICLE_NODE);
        if e.call(WORD_DIFFERS_0052AA80, &args![particle_node, 0u32])
            .bool()
            && node != 0
        {
            let source = if e.get(this, MagicShaderHitEffect::bWeaponEnchantment) != 0 {
                e.call(NODE_ROTATION_00461130, &args![node]).u32()
            } else {
                NODE_ARGUMENT_011A9448
            };
            let rotation = local(0x40);
            for i in 0..9 {
                let word = e.mem.u32(source.wrapping_add(4 * i));
                e.mem.set_u32(rotation.wrapping_add(4 * i), word);
            }
            if !own {
                let first_view = first_person && {
                    let player = e.mem.u32(PLAYER);
                    e.call(PLAYER_NODE_00950BB0, &args![player, 1u32]).u32() != 0
                };
                if !first_view {
                    let position = e.call(CAMERA_POSITION_0045BB80, &args![node]).u32();
                    let object = held(e, base, PARTICLE_NODE);
                    e.call(SET_TRANSLATE_00440460, &args![object, position]);
                    let object = held(e, base, PARTICLE_NODE);
                    e.call(NODE_SET_0043FA80, &args![object, rotation]);
                } else {
                    let player = e.mem.u32(PLAYER);
                    let view = e.call(PLAYER_NODE_00950BB0, &args![player, 1u32]).u32();
                    let attach = e
                        .call(
                            NODE_LOOKUP_00653270,
                            &args![NODE_LOOKUP_ARGUMENT_011F4428, view],
                        )
                        .u32();
                    if attach != 0 {
                        let object = held(e, base, PARTICLE_NODE);
                        let parent = e.call(WORD_AT_18_009611E0, &args![object]).u32();
                        if attach != parent {
                            let object = held(e, base, PARTICLE_NODE);
                            e.vcall(attach, 0xdc, &args![object, 1u32]);
                        }
                    }
                    if first_person {
                        let object = held(e, base, PARTICLE_NODE);
                        let parent = e.call(WORD_AT_18_009611E0, &args![object]).u32();
                        let parent_rotation = e.call(NODE_ROTATION_00461130, &args![parent]).u32();
                        let inverse = e
                            .call(MATRIX_INVERSE, &args![parent_rotation, local(0x78)])
                            .u32();
                        let product = e
                            .call(MATRIX_MULTIPLY, &args![inverse, local(0x9c), rotation])
                            .u32();
                        let object = held(e, base, PARTICLE_NODE);
                        e.call(NODE_SET_0043FA80, &args![object, product]);
                        let object = held(e, base, PARTICLE_NODE);
                        let parent = e.call(WORD_AT_18_009611E0, &args![object]).u32();
                        let parent_position =
                            e.call(CAMERA_POSITION_0045BB80, &args![parent]).u32();
                        let position = e.call(CAMERA_POSITION_0045BB80, &args![node]).u32();
                        let offset = e
                            .call(
                                POINT_SUBTRACT_00439EF0,
                                &args![position, local(0xcc), parent_position],
                            )
                            .u32();
                        let object = held(e, base, PARTICLE_NODE);
                        let parent = e.call(WORD_AT_18_009611E0, &args![object]).u32();
                        let parent_rotation = e.call(NODE_ROTATION_00461130, &args![parent]).u32();
                        let inverse = e
                            .call(MATRIX_INVERSE, &args![parent_rotation, local(0xc0)])
                            .u32();
                        let local_position = e
                            .call(
                                MATRIX_TIMES_POINT_004B4500,
                                &args![inverse, local(0xd8), offset],
                            )
                            .u32();
                        let object = held(e, base, PARTICLE_NODE);
                        e.call(SET_TRANSLATE_00440460, &args![object, local_position]);
                        let block = local(0x50);
                        e.call(POINT_SET_0043D410, &args![block, 0u32, 0u32, 0u32]);
                        let object = held(e, base, PARTICLE_NODE);
                        e.call(SET_TRANSLATE_00A59C60, &args![object, block]);
                    } else {
                        // Not reached: this branch needs the first-person flag
                        // the enclosing test already required.
                        let object = held(e, base, PARTICLE_NODE);
                        e.call(NODE_SET_0043FA80, &args![object, rotation]);
                        let position = e.call(CAMERA_POSITION_0045BB80, &args![node]).u32();
                        let object = held(e, base, PARTICLE_NODE);
                        e.call(SET_TRANSLATE_00440460, &args![object, position]);
                    }
                }
            }
        }
        if first_person || own {
            e.call(HIT_EFFECT_0081BC70, &args![base, 0u32]);
        } else {
            let object = held(e, base, PARTICLE_NODE);
            e.call(HIT_EFFECT_0081BC70, &args![base, object]);
        }
    });
}

// Translated from 00821bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::ClearTarget` (engine map; `thiscall`): for the
/// two fire shaders (form ids 0x852FE and 0x84A51) the target's moving
/// sounds stop (`BSAudio::StopMovingSounds(3D, 1.0, 0)`); with a texture
/// effect its shader stops on the player's two views or on the target's
/// 3D; with a particle node every particle shader stops on it; the
/// particle array is cleared (its buffer freed), the particle node
/// detached (`AttachParent(0)`), both pointers cleared and the target
/// forgotten.
pub fn magic_shader_hit_effect_clear_target(e: &mut Engine, this: Ptr<MagicShaderHitEffect>) {
    let base = this.addr();
    let fire = e.get(this, MagicShaderHitEffect::pTarget) != 0
        && e.get(this, MagicShaderHitEffect::pEffectData) != 0
        && {
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            e.call(FORM_ID_0084E3A0, &args![shader]).u32() == FIRE_SHADER_852FE || {
                let shader = e.get(this, MagicShaderHitEffect::pEffectData);
                e.call(FORM_ID_0084E3A0, &args![shader]).u32() == FIRE_SHADER_84A51
            }
        };
    if fire {
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        let node = e.vcall(target, 0x1d0, &[]).u32();
        let audio = e.call(BS_AUDIO_INSTANCE, &[]).u32();
        e.call(
            BS_AUDIO_STOP_MOVING_SOUNDS,
            &args![audio, node, 1.0f32.to_bits(), 0u32],
        );
    }
    if held(e, base, TEX_EFFECT_DATA) != 0 {
        let player = e.mem.u32(PLAYER);
        if e.get(this, MagicShaderHitEffect::pTarget) == player {
            for view in [1u32, 0] {
                let player = e.mem.u32(PLAYER);
                let node = e.call(PLAYER_NODE_00950BB0, &args![player, view]).u32();
                if node != 0 {
                    stop_texture_shader(e, this, node);
                }
            }
        } else if e.get(this, MagicShaderHitEffect::pTarget) != 0 {
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            let node = e.vcall(target, 0x1d0, &[]).u32();
            if node != 0 {
                stop_texture_shader(e, this, node);
            }
        }
    }
    let particle_node = base.wrapping_add(PARTICLE_NODE);
    if e.call(WORD_DIFFERS_0052AA80, &args![particle_node, 0u32])
        .bool()
    {
        let mut index = 0;
        while index < array_count(e, base, PARTICLE_PROPERTY_ARRAY) {
            let property = array_item(e, base, PARTICLE_PROPERTY_ARRAY, index);
            let node = held(e, base, PARTICLE_NODE);
            let shader = e.get(this, MagicShaderHitEffect::pEffectData);
            e.call(STOP_PARTICLE_SHADER, &args![shader, node, property]);
            index += 1;
        }
    }
    fn_006c6200(e, Ptr::new(base.wrapping_add(PARTICLE_PROPERTY_ARRAY)), 1);
    if e.call(WORD_DIFFERS_0052AA80, &args![particle_node, 0u32])
        .bool()
    {
        let node = held(e, base, PARTICLE_NODE);
        e.call(ATTACH_PARENT_00A59D00, &args![node, 0u32]);
    }
    e.call(NI_POINTER_ASSIGN_0066B0D0, &args![particle_node, 0u32]);
    e.call(
        NI_POINTER_ASSIGN_0066B0D0,
        &args![base.wrapping_add(TEX_EFFECT_DATA), 0u32],
    );
    e.set(this, MagicShaderHitEffect::pTarget, 0);
}

// Translated from 00821dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::GetSaveSize` (engine map; `thiscall`, `RET 8`):
/// the base's size plus 4, plus 1 and 4 from save version 0x37 (16-bit
/// sums).
pub fn magic_shader_hit_effect_get_save_size(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
) -> u16 {
    let mut size = e
        .call(HIT_EFFECT_GET_SAVE_SIZE, &args![this, first, second])
        .u16();
    size = size.wrapping_add(4);
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x37 {
        size = size.wrapping_add(1).wrapping_add(4);
    }
    size
}

// Translated from 00821e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::SaveGame` (engine map; `thiscall`, `RET 8`): the
/// base's save and `fAlphaTimer`; from save version 0x37 also
/// `bWeaponEnchantment` and the worn object's form id (0 without one).
pub fn magic_shader_hit_effect_save_game(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    first: u32,
    second: u32,
) {
    let base = this.addr();
    e.call(HIT_EFFECT_SAVE_GAME, &args![base, first, second]);
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    e.call(SAVE_WRITE, &args![save_load, base.wrapping_add(0x34), 4u32]);
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![save_load]).u8() < 0x37 {
        return;
    }
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    e.call(SAVE_WRITE, &args![save_load, base.wrapping_add(0x28), 1u32]);
    e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), 0);
        if e.get(this, MagicShaderHitEffect::pWornObject) != 0 {
            let worn = e.get(this, MagicShaderHitEffect::pWornObject);
            let value = e.call(FORM_ID_0084E3A0, &args![worn]).u32();
            e.mem.set_u32(id.addr(), value);
        }
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_NUMERIC_ID, &args![save_load, id, 4u32]);
    });
}

// Translated from 00821ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::LoadGame` (engine map; `thiscall`, `RET 8`): the
/// base's load and `fAlphaTimer`; from save version 0x37 also
/// `bWeaponEnchantment` and the worn object (an id's form cast to
/// `TESBoundObject`, kept only when the id is not 0).
pub fn magic_shader_hit_effect_load_game(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    first: u32,
    second: u32,
) {
    let base = this.addr();
    e.call(HIT_EFFECT_LOAD_GAME, &args![base, first, second]);
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    e.call(LOAD_READ, &args![save_load, base.wrapping_add(0x34), 4u32]);
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![save_load]).u8() < 0x37 {
        return;
    }
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    e.call(LOAD_READ, &args![save_load, base.wrapping_add(0x28), 1u32]);
    e.with_stack(4, |e, id| {
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(LOAD_NUMERIC_ID, &args![save_load, id, 4u32]);
        let value = e.mem.u32(id.addr());
        if value != 0 {
            let form = e.call(LOOKUP_FORM_004839C0, &args![value]).u32();
            let worn = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, RTTI_01183108, 0u32],
                )
                .u32();
            e.set(this, MagicShaderHitEffect::pWornObject, worn);
        }
    });
}

// Translated from 00821f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::FinishInitLoadGame` (engine map; `thiscall`,
/// `RET 0xC`: the active effect, the reference, a shader): the base's
/// (`008d05e0` with the two words and 0); `pEffectData` becomes the effect
/// setting's shader (`MagicItem::GetAVEffect(0)` of the active effect's
/// item, its `00407820`) or the third word; slot +0xC4 runs with `fAge` and
/// `bFinished` kept, then slot +0x94 with 0.0.
pub fn magic_shader_hit_effect_finish_init_load_game(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    effect: u32,
    reference: u32,
    shader: u32,
) {
    let base = this.addr();
    e.call(
        HIT_EFFECT_FINISH_INIT_LOAD_GAME,
        &args![base, effect, reference, 0u32],
    );
    if effect != 0 {
        let item = e.call(WORD_AT_8_0044DDC0, &args![effect]).u32();
        let setting = e.call(MAGIC_ITEM_GET_AV_EFFECT, &args![item, 0u32]).u32();
        let found = e.call(WORD_AT_78_00407820, &args![setting]).u32();
        e.set(this, MagicShaderHitEffect::pEffectData, found);
    } else {
        e.set(this, MagicShaderHitEffect::pEffectData, shader);
    }
    let age = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAge));
    let finished = e.get(this, MagicShaderHitEffect::bFinished);
    e.vcall(base, 0xc4, &[]);
    e.set(this, MagicShaderHitEffect::fAge, x87_quiet_bits(age));
    e.set(this, MagicShaderHitEffect::bFinished, finished);
    e.vcall(base, 0x94, &args![0u32]);
}

// Translated from 00822010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::CheckShouldSave` (engine map; `thiscall`): true
/// only without an active effect, with a target and a shader, not a weapon
/// enchantment, and when the shader is not `0080d0d0`'s.
pub fn magic_shader_hit_effect_check_should_save(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
) -> u8 {
    if e.mem.u32(this.addr().wrapping_add(0x18)) != 0 {
        return 0;
    }
    if e.get(this, MagicShaderHitEffect::pTarget) == 0
        || e.get(this, MagicShaderHitEffect::pEffectData) == 0
        || e.get(this, MagicShaderHitEffect::bWeaponEnchantment) != 0
    {
        return 0;
    }
    let skipped = e.call(SKIPPED_SHADER_0080D0D0, &[]).u32();
    u8::from(e.get(this, MagicShaderHitEffect::pEffectData) != skipped)
}

// Translated from 00822060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::GetSaveSize_ov2` (engine map; `thiscall`): 8 plus
/// this effect's slot +0xD0 with (0, target) (16-bit sums).
pub fn magic_shader_hit_effect_get_save_size_ov2(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
) -> u16 {
    let size: u16 = 4u16.wrapping_add(4);
    let target = e.get(this, MagicShaderHitEffect::pTarget);
    let more = e.vcall(this.addr(), 0xd0, &args![0u32, target]).u16();
    size.wrapping_add(more)
}

// Translated from 008220c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::SaveGame_ov2` (engine map; `thiscall`): the
/// target's and the shader's ids (`0084e3a0`) through
/// `TESSaveLoadGame::SaveNumericID`, then slot +0xD4 with (0, target).
pub fn magic_shader_hit_effect_save_game_ov2(e: &mut Engine, this: Ptr<MagicShaderHitEffect>) {
    e.with_stack(8, |e, frame| {
        // `[EBP-4]` and `[EBP-8]`.
        let target_id = frame.addr().wrapping_add(4);
        let shader_id = frame.addr();
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        let id = e.call(FORM_ID_0084E3A0, &args![target]).u32();
        e.mem.set_u32(target_id, id);
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_NUMERIC_ID, &args![save_load, target_id, 4u32]);
        let shader = e.get(this, MagicShaderHitEffect::pEffectData);
        let id = e.call(FORM_ID_0084E3A0, &args![shader]).u32();
        e.mem.set_u32(shader_id, id);
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_NUMERIC_ID, &args![save_load, shader_id, 4u32]);
    });
    let target = e.get(this, MagicShaderHitEffect::pTarget);
    e.vcall(this.addr(), 0xd4, &args![0u32, target]);
}

// Translated from 00822130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::LoadGame_ov2` (engine map; `thiscall`; answers a
/// byte): reads the target's id (a `TESObjectREFR`) and the shader's id (a
/// `TESEffectShader`); the answer is true when the target has a cell that
/// passes `00450ff0` and the shader was found. Slot +0xD8 runs with (0,
/// target), and when the answer is true slots +0xDC with (0, target) and
/// +0xE0 with (0, target, shader).
pub fn magic_shader_hit_effect_load_game_ov2(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
) -> u8 {
    let base = this.addr();
    e.with_stack(0x10, |e, frame| {
        // `[EBP-8]` (the target's id) and `[EBP-0x10]` (the shader's id).
        let target_id = frame.addr().wrapping_add(8);
        let shader_id = frame.addr();
        let mut loaded = 1u8;
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(LOAD_NUMERIC_ID, &args![save_load, target_id, 4u32]);
        let id = e.mem.u32(target_id);
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        let target = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, RTTI_011841CC, 0u32],
            )
            .u32();
        e.set(this, MagicShaderHitEffect::pTarget, target);
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        let good = target != 0 && e.call(REFERENCE_CELL_008D6F30, &args![target]).u32() != 0 && {
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            let cell = e.call(REFERENCE_CELL_008D6F30, &args![target]).u32();
            e.call(CELL_TEST_00450FF0, &args![cell]).bool()
        };
        if !good {
            loaded = 0;
        }
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(LOAD_NUMERIC_ID, &args![save_load, shader_id, 4u32]);
        let id = e.mem.u32(shader_id);
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        let shader = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_EFFECT_SHADER, 0u32],
            )
            .u32();
        if shader == 0 {
            loaded = 0;
        }
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        e.vcall(base, 0xd8, &args![0u32, target]);
        if loaded != 0 {
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            e.vcall(base, 0xdc, &args![0u32, target]);
            let target = e.get(this, MagicShaderHitEffect::pTarget);
            e.vcall(base, 0xe0, &args![0u32, target, shader]);
        }
        loaded
    })
}

// Translated from 00822250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::SaveGame_ov3` (engine map; `thiscall`, `RET 4`:
/// the save buffer): the base's save, then `bWeaponEnchantment`,
/// `fAlphaTimer`, `fAddonAlpha`, `fAddonScale`, `b3rdPerson` (`00865e50`
/// with their sizes and 0) and the worn object's and the shader's form ids
/// (`SaveFormID_ov2` with 0).
pub fn magic_shader_hit_effect_save_game_ov3(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    buffer: u32,
) {
    let base = this.addr();
    e.call(HIT_EFFECT_SAVE_GAME_OV2, &args![base, buffer]);
    for (offset, size) in [(0x28u32, 1u32), (0x34, 4), (0x60, 4), (0x64, 4), (0x4c, 1)] {
        e.call(
            BUFFER_WRITE_00865E50,
            &args![buffer, base.wrapping_add(offset), size, 0u32],
        );
    }
    let worn = e.get(this, MagicShaderHitEffect::pWornObject);
    e.call(BUFFER_SAVE_FORM_ID, &args![buffer, worn, 0u32]);
    let shader = e.get(this, MagicShaderHitEffect::pEffectData);
    e.call(BUFFER_SAVE_FORM_ID, &args![buffer, shader, 0u32]);
}

// Translated from 008222f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::LoadGame_ov3` (engine map; `thiscall`, `RET 4`:
/// the load buffer): the base's load, `bWeaponEnchantment`,
/// `fAlphaTimer`, from buffer version 8 (its slot 0) `fAddonAlpha` and
/// `fAddonScale`, `b3rdPerson`, then the worn object (a `TESBoundObject`)
/// and the shader (a `TESEffectShader`) from their form ids.
pub fn magic_shader_hit_effect_load_game_ov3(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    buffer: u32,
) {
    let base = this.addr();
    e.call(HIT_EFFECT_LOAD_GAME_OV2, &args![base, buffer]);
    e.call(
        BUFFER_READ_00864980,
        &args![buffer, base.wrapping_add(0x28), 1u32],
    );
    e.call(
        BUFFER_READ_00864980,
        &args![buffer, base.wrapping_add(0x34), 4u32],
    );
    if e.vcall(buffer, 0, &[]).u8() >= 8 {
        e.call(
            BUFFER_READ_00864980,
            &args![buffer, base.wrapping_add(0x60), 4u32],
        );
        e.call(
            BUFFER_READ_00864980,
            &args![buffer, base.wrapping_add(0x64), 4u32],
        );
    }
    e.call(
        BUFFER_READ_00864980,
        &args![buffer, base.wrapping_add(0x4c), 1u32],
    );
    let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
    let worn = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, RTTI_01183108, 0u32],
        )
        .u32();
    e.set(this, MagicShaderHitEffect::pWornObject, worn);
    let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
    let shader = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_EFFECT_SHADER, 0u32],
        )
        .u32();
    e.set(this, MagicShaderHitEffect::pEffectData, shader);
}

// Translated from 008223d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicShaderHitEffect::FinishLoadGame` (engine map; `thiscall`, `RET 4`:
/// the load buffer): the base's (`009dace0`), slot +0xC4 with `fAge` and
/// `bFinished` kept, then slot +0x94 with 0.0.
pub fn magic_shader_hit_effect_finish_load_game(
    e: &mut Engine,
    this: Ptr<MagicShaderHitEffect>,
    buffer: u32,
) {
    let base = this.addr();
    e.call(HIT_EFFECT_FINISH_LOAD_GAME, &args![base, buffer]);
    let age = x87_quiet_bits(e.get(this, MagicShaderHitEffect::fAge));
    let finished = e.get(this, MagicShaderHitEffect::bFinished);
    e.vcall(base, 0xc4, &[]);
    e.set(this, MagicShaderHitEffect::fAge, x87_quiet_bits(age));
    e.set(this, MagicShaderHitEffect::bFinished, finished);
    e.vcall(base, 0x94, &args![0u32]);
}

// Translated from 00822440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weighted random's constructor (`thiscall`; its use in `0081fc70`):
/// the string array base ([`fn_00822600`]), the weights at +0x10
/// (`00438660`) and the word +0x20 cleared. Returns `this`.
// Translation incomplete: 00822440 - the FS:[0] exception frame (unwinding the base on a throw) is not translated
pub fn fn_00822440(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00822600(e, this);
    e.call(
        WEIGHTS_CONSTRUCT_00438660,
        &args![this.addr().wrapping_add(0x10)],
    );
    e.mem.set_u32(this.addr().wrapping_add(0x20), 0);
    this
}

// Translated from 008224b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weighted random's destructor (`thiscall`): the weights at +0x10
/// (`00438690`), then the string array base ([`fn_00822630`]).
// Translation incomplete: 008224b0 - the FS:[0] exception frame (unwinding the base on a throw) is not translated
pub fn fn_008224b0(e: &mut Engine, this: Ptr) {
    e.call(
        WEIGHTS_DESTRUCT_00438690,
        &args![this.addr().wrapping_add(0x10)],
    );
    fn_00822630(e, this);
}

// Translated from 00822650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weighted random's pick (`thiscall`): a roll (`005a00a0` with the
/// word +0x20) walks the weights (+0x10's items, `006a7ad0`) taking each
/// away until one is above what is left; that index's string (`006a7ad0`
/// on `this`) is the answer, 0 when none is.
pub fn fn_00822650(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let mut result = 0;
    let mut found = false;
    let total = e.mem.u32(base.wrapping_add(0x20));
    let mut left = e.call(ROLL_005A00A0, &args![total]).u32();
    let mut index = 0u32;
    loop {
        let count = e.call(WORD_AT_8_0044DDC0, &args![base]).u32();
        if index >= count || found {
            break;
        }
        let weight = e
            .call(ARRAY_AT_006A7AD0, &args![base.wrapping_add(0x10), index])
            .u32();
        if left < e.mem.u32(weight) {
            let entry = e.call(ARRAY_AT_006A7AD0, &args![base, index]).u32();
            result = e.mem.u32(entry);
            found = true;
        }
        let weight = e
            .call(ARRAY_AT_006A7AD0, &args![base.wrapping_add(0x10), index])
            .u32();
        left = left.wrapping_sub(e.mem.u32(weight));
        index = index.wrapping_add(1);
    }
    result
}

/// A scalar deleting destructor: `destroy`, then the free (`00401030`) of
/// `this` when bit 0 of the flags is set.
fn scalar_deleting(e: &mut Engine, this: Ptr, flags: u32, destroy: fn(&mut Engine, Ptr)) -> Ptr {
    destroy(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00401030, &args![this]);
    }
    this
}

// Translated from 008226f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<ParticleShaderProperty>,1024>`'s scalar
/// deleting destructor (engine map; `thiscall`, `RET 4`): the destructor
/// ([`bs_simple_array_particle_shader_property_destructor`]) and the free
/// with bit 0. Returns `this`.
pub fn bs_simple_array_particle_shader_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(
        e,
        this,
        flags,
        bs_simple_array_particle_shader_property_destructor,
    )
}

// Translated from 00822720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<NiAVObject>,1024>`'s scalar deleting destructor
/// (engine map; `thiscall`, `RET 4`): the destructor
/// ([`bs_simple_array_ni_av_object_destructor`]) and the free with bit 0.
/// Returns `this`.
pub fn bs_simple_array_ni_av_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, bs_simple_array_ni_av_object_destructor)
}

// Translated from 00822750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<char *,1024>`'s scalar deleting destructor (engine map;
/// `thiscall`, `RET 4`): the destructor ([`fn_00822630`]) and the free with
/// bit 0. Returns `this`.
pub fn bs_simple_array_char_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_00822630)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c_6200, fn_006c6200(Ptr, u8)),
        entry!(
            0x0081_f320,
            magic_shader_hit_effect_scalar_deleting_destructor(
                Ptr<MagicShaderHitEffect>,
                u32,
            )
                -> Ptr<MagicShaderHitEffect>
        ),
        entry!(
            0x0081_f350,
            magic_shader_hit_effect_magic_shader_hit_effect(
                Ptr<MagicShaderHitEffect>,
                u32,
                u32,
            )
                -> Ptr<MagicShaderHitEffect>
        ),
        entry!(
            0x0081_f770,
            magic_shader_hit_effect_destructor(Ptr<MagicShaderHitEffect>)
        ),
        entry!(
            0x0081_fc70,
            magic_shader_hit_effect_init(Ptr<MagicShaderHitEffect>) -> bool
        ),
        entry!(
            0x0082_0c10,
            fn_00820c10(Ptr<MagicShaderHitEffect>, u32, u32, u32, u32, u32)
        ),
        entry!(0x0082_1600, fn_00821600(Ptr, u32)),
        entry!(0x0082_1640, fn_00821640(Ptr) -> f32),
        entry!(0x0082_1660, fn_00821660(Ptr) -> f32),
        entry!(0x0082_1680, fn_00821680(Ptr) -> f32),
        entry!(0x0082_2510, fn_00822510(Ptr, u32) -> u32),
        entry!(0x0082_2530, fn_00822530(Ptr) -> Ptr),
        entry!(
            0x0082_2560,
            bs_simple_array_particle_shader_property_destructor(Ptr)
        ),
        entry!(0x0082_2580, fn_00822580(Ptr) -> Ptr),
        entry!(0x0082_25b0, bs_simple_array_ni_av_object_destructor(Ptr)),
        entry!(0x0082_2630, fn_00822630(Ptr)),
        entry!(0x0081_f310, fn_0081f310(Ptr) -> u32),
        entry!(
            0x0081_d9d0,
            magic_shader_hit_effect_init_load_game_ov2(Ptr, Ptr)
        ),
        entry!(0x00822600, fn_00822600(Ptr) -> Ptr),
        entry!(0x008225d0, fn_008225d0(Ptr, u32) -> u32),
        entry!(
            0x0081_d390,
            magic_shader_hit_effect_init_load_game(Ptr, u32, u32)
        ),
        entry!(
            0x0081_f580,
            fn_0081f580(Ptr<MagicShaderHitEffect>, u32, u32, f32) -> Ptr<MagicShaderHitEffect>
        ),
        entry!(0x0082_0b50, fn_00820b50(Ptr) -> u32),
        entry!(0x0082_0b70, fn_00820b70(Ptr, f32)),
        entry!(0x0082_0b90, fn_00820b90(Ptr)),
        entry!(0x0082_0bb0, fn_00820bb0(Ptr)),
        entry!(0x0082_0bd0, fn_00820bd0(Ptr, u8)),
        entry!(0x0082_0bf0, fn_00820bf0(Ptr) -> u32),
        entry!(
            0x0082_0f30,
            magic_shader_hit_effect_update(Ptr<MagicShaderHitEffect>, f32) -> u8
        ),
        entry!(0x0082_1620, fn_00821620(Ptr, f32)),
        entry!(0x0082_16a0, fn_008216a0(Ptr) -> f32),
        entry!(
            0x0082_16c0,
            magic_shader_hit_effect_reset_alpha_timer(Ptr<MagicShaderHitEffect>)
        ),
        entry!(
            0x0082_17e0,
            magic_shader_hit_effect_update_position(Ptr<MagicShaderHitEffect>)
        ),
        entry!(
            0x0082_1bd0,
            magic_shader_hit_effect_clear_target(Ptr<MagicShaderHitEffect>)
        ),
        entry!(
            0x0082_1dd0,
            magic_shader_hit_effect_get_save_size(Ptr, u32, u32) -> u16
        ),
        entry!(
            0x0082_1e40,
            magic_shader_hit_effect_save_game(Ptr<MagicShaderHitEffect>, u32, u32)
        ),
        entry!(
            0x0082_1ed0,
            magic_shader_hit_effect_load_game(Ptr<MagicShaderHitEffect>, u32, u32)
        ),
        entry!(
            0x0082_1f70,
            magic_shader_hit_effect_finish_init_load_game(Ptr<MagicShaderHitEffect>, u32, u32, u32)
        ),
        entry!(
            0x0082_2010,
            magic_shader_hit_effect_check_should_save(Ptr<MagicShaderHitEffect>) -> u8
        ),
        entry!(
            0x0082_2060,
            magic_shader_hit_effect_get_save_size_ov2(Ptr<MagicShaderHitEffect>) -> u16
        ),
        entry!(
            0x0082_20c0,
            magic_shader_hit_effect_save_game_ov2(Ptr<MagicShaderHitEffect>)
        ),
        entry!(
            0x0082_2130,
            magic_shader_hit_effect_load_game_ov2(Ptr<MagicShaderHitEffect>) -> u8
        ),
        entry!(
            0x0082_2250,
            magic_shader_hit_effect_save_game_ov3(Ptr<MagicShaderHitEffect>, u32)
        ),
        entry!(
            0x0082_22f0,
            magic_shader_hit_effect_load_game_ov3(Ptr<MagicShaderHitEffect>, u32)
        ),
        entry!(
            0x0082_23d0,
            magic_shader_hit_effect_finish_load_game(Ptr<MagicShaderHitEffect>, u32)
        ),
        entry!(0x0082_2440, fn_00822440(Ptr) -> Ptr),
        entry!(0x0082_24b0, fn_008224b0(Ptr)),
        entry!(0x0082_2650, fn_00822650(Ptr) -> u32),
        entry!(
            0x0082_26f0,
            bs_simple_array_particle_shader_property_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0082_2720,
            bs_simple_array_ni_av_object_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0082_2750,
            bs_simple_array_char_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bucket_array_is_four_bytes_a_bucket() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (0x5000 + a[0]).into_ret());
        assert_eq!(e.call(0x008225d0, &[0, 37]).u32(), 0x5000 + 37 * 4);
    }

    #[test]
    fn stores_the_vtable_then_builds_an_empty_array() {
        let mut e = Engine::new();
        e.register(ARRAY_CONSTRUCTOR, |e, a| {
            // The vtable is already in place when the constructor runs.
            assert_eq!(e.mem.u32(a[0]), VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(16);
        e.call_log = Some(vec![]);
        let r = e.call(0x00822600, &args![this]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ARRAY_CONSTRUCTOR, vec![this, 0, 0]));
    }

    #[test]
    fn forwarder_magic_shader_hit_effect_init_load_game_ov2_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(INIT_LOAD_GAME_TARGET_0081BF20, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0081_d9d0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INIT_LOAD_GAME_TARGET_0081BF20)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0081f310_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0081_f310, &args![0x1000u32]).u32(), 0x011d_c804);
        assert_eq!(e.call(0x0081_f310, &args![0u32]).u32(), 0x011d_c804);
    }
}

#[cfg(test)]
mod hit_effect_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The vtable every test object uses (0x130 slots); slot `n` calls `0x0f20_0000 + n`.
    const VTABLE: u32 = 0x0f10_0000;
    const SLOT_BASE: u32 = 0x0f20_0000;

    /// Field getters as their bodies, the rest recorded and answering 0; the test objects'
    /// virtual slots answer the words at fixed offsets of the object: +0x100 the word at
    /// +0x40, +0x1d0 at +0x44, +0x1e8 at +0x48, +0x1f4 the address +0x50, +0xc at +0x5c,
    /// +0x1b0 at +0x60, +0x454 at +0x64.
    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_e000,
            0x011c_3000,
            0x0101_2000,
            0x0101_3000,
            0x0101_6000,
            0x0106_6000,
            0x0106_d000,
            0x0107_b000,
            0x011a_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ALPHA_TIMER_01016410, 100.0f32);
        e.set_global(UPDATE_STEP_0107B7F0, 0.166f64);
        e.set_global(ALPHA_LIMIT_0106D508, 0.99f64);
        e.set_global(LOWER_ALPHA_0106682C, 0.99f32);
        e.set_global(ADDON_SCALE_01013EA4, 0.01f32);
        let slots: Vec<u32> = (0..0x130u32).map(|i| SLOT_BASE + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for addr in slots {
            e.register(addr, |_, _| Ret::default());
        }
        let field = |offset: u32| -> AbiFn {
            match offset {
                0x40 => |e, a| e.mem.u32(a[0] + 0x40).into_ret(),
                0x44 => |e, a| e.mem.u32(a[0] + 0x44).into_ret(),
                0x48 => |e, a| e.mem.u32(a[0] + 0x48).into_ret(),
                0x50 => |_, a| (a[0] + 0x50).into_ret(),
                0x5c => |e, a| e.mem.u32(a[0] + 0x5c).into_ret(),
                0x60 => |e, a| e.mem.u32(a[0] + 0x60).into_ret(),
                _ => |e, a| e.mem.u32(a[0] + 0x64).into_ret(),
            }
        };
        for (slot, offset) in [
            (0x100u32, 0x40u32),
            (0x1d0, 0x44),
            (0x1e8, 0x48),
            (0x1f4, 0x50),
            (0xc, 0x5c),
            (0x1b0, 0x60),
            (0x454, 0x64),
        ] {
            e.register(SLOT_BASE + slot, field(offset));
        }
        for addr in [
            ARRAY_DESTRUCT_ITEMS_00822820,
            ARRAY_FREE_BUFFER_006A8500,
            ARRAY_INIT_00822860,
            STRING_ARRAY_CLEAR_008454F0,
            FREE_SIZED_00AA1460,
            MAGIC_HIT_EFFECT_CONSTRUCTOR,
            MAGIC_HIT_EFFECT_DESTRUCTOR,
            NI_POINTER_DESTRUCT_0045CEC0,
            MAGIC_ITEM_GET_AV_EFFECT,
            FORM_ID_0084E3A0,
            PLAYER_NODE_00950BB0,
            PLAYER_GET_BIPED,
            QUERY_00456610,
            MODEL_LOADER_LOAD_FILE,
            MODEL_RELEASE_0045A5E0,
            STOP_TEXTURE_SHADER,
            STOP_PARTICLE_SHADER,
            ATTACH_PARENT_00A59D00,
            BS_AUDIO_INSTANCE,
            BS_AUDIO_STOP_MOVING_SOUNDS,
            SHADER_FILL_TEXTURE_00506390,
            SHADER_TEXTURE_00506370,
            SHADER_TEXTURE_005063B0,
            NAME_STRING_00408DA0,
            STRING_LENGTH_0048CEE0,
            FORMAT_00406D00,
            TES_REMOVE_TEXTURE_IMAGE,
            TES_CREATE_TEXTURE_IMAGE,
            RT_DYNAMIC_CAST,
            ANIMATION_00490F80,
            ACTIVE_HIT_EFFECT_00820BF0,
            SAVED_ACQUIRE_OBJECT_008D8520,
            LOCATION_006733E0,
            BIPED_SLOT_004AAF30,
            FORM_007AF430,
            WORLD_BOUND_0043D450,
            BOUND_RADIUS_0084D030,
            POINT_SET_0043D410,
            SET_TRANSLATE_00A59C60,
            CAMERA_POSITION_0045BB80,
            POINT_SUBTRACT_00439EF0,
            POINT_LENGTH_00457990,
            ACTIVE_EFFECT_AGE_006B9130,
            ACTIVE_EFFECT_ITEM_007DF1D0,
            ITEM_FLAG_00403E60,
            ACTIVE_EFFECT_FLAG_00804B80,
            SHADER_FLAG_00507040,
            START_PARTICLE_SHADER,
            TEXTURE_EFFECT_DATA_00507830,
            START_TEXTURE_SHADER,
            SHADER_VALUE_008D01E0,
            SHADER_VALUE_00644930,
            SHADER_VALUE_00567470,
            FLOAT_HELPER_00404010,
            FTOL,
            PARTICLE_STEP_00BA52D0,
            GET_SOUND_HANDLE_BY_NAME,
            SOUND_SET_POSITION,
            SOUND_SET_VOLUME,
            SOUND_PLAY,
            SOUND_SET_OBJECT_TO_FOLLOW,
            SOUND_HANDLE_DESTRUCT_00483710,
            ALLOCATE_00AA13E0,
            NI_NODE_CONSTRUCT_00A5ECB0,
            FIXED_STRING_FROM_00438170,
            FIXED_STRING_DESTRUCT_004381B0,
            SET_NAME_00A5B950,
            NODE_SET_0043FA80,
            PARTICLE_SET_RADIUS_00820B70,
            PARTICLE_SET_00820BD0,
            PARTICLE_FIRST_PERSON_00820BB0,
            PARTICLE_THIRD_PERSON_00820B90,
            COLLECT_TARGETS,
            INIT_PARTICLE_DATA,
            UPDATE_PARTICLES,
            ACTOR_GET_ALPHA_MULT,
            ACTOR_SET_ALPHA_MULT,
            ACTOR_UPDATE_ALPHA,
            PARTICLE_OBJECTS_00820B50,
            COUNT_00658930,
            ELEMENT_00877A30,
            WEIGHTED_RANDOM_CONSTRUCT_00822440,
            ADD_WEIGHTED_RESULT,
            WEIGHTED_RANDOM_PICK_00822650,
            WEIGHTED_RANDOM_DESTRUCT_008224B0,
            FIND_MODEL,
            MODEL_ROOT_0043B230,
            CLONING_PROCESS_CONSTRUCT_004AD050,
            CLONING_PROCESS_DESTRUCT_004AD270,
            NI_OBJECT_CLONE,
            QUEUE_ATTACH_TEST_008C7AA0,
            TASK_QUEUE_004537B0,
            QUEUE_NODE_ATTACH,
            SET_SCALE_00440490,
            START_ANIMATIONS,
            POINTER_RELEASE_0040C110,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        // NiPointer helpers and plain field getters, as their bodies.
        e.register(NI_POINTER_FROM_00633C90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NI_POINTER_ASSIGN_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(WORD_AT_0_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WORD_AT_8_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(WORD_AT_18_009611E0, |e, a| {
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        e.register(WORD_AT_78_00407820, |e, a| {
            e.mem.u32(a[0] + 0x78).into_ret()
        });
        e.register(WORD_AT_10C_005A29B0, |e, a| {
            e.mem.u32(a[0] + 0x10c).into_ret()
        });
        e.register(WORD_AT_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_HEAD_00409140, |_, a| (a[0] + 0x1c).into_ret());
        e.register(SELF_006815C0, |_, a| a[0].into_ret());
        e.register(WORD_DIFFERS_0052AA80, |e, a| {
            u32::from(e.mem.u32(a[0]) != a[1]).into_ret()
        });
        e.register(NODE_EMPTY_008256D0, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(ARRAY_AT_006A7AD0, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        // Appending to an array: the pointer word goes to the end of its buffer.
        e.register(ARRAY_APPEND_006C61B0, |e, a| {
            let mut buffer = e.mem.u32(a[0] + 4);
            if buffer == 0 {
                buffer = e.mem.alloc(0x40);
                e.mem.set_u32(a[0] + 4, buffer);
            }
            let size = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A test object with the shared vtable.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let o = e.mem.alloc(size.max(0x70));
        e.mem.set_u32(o, VTABLE);
        o
    }

    /// An array (`BSSimpleArray`) of the given words.
    fn array(e: &mut Engine, at: u32, words: &[u32]) {
        let buffer = e.mem.alloc(0x40);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *w);
        }
        e.mem.set_u32(at + 4, buffer);
        e.mem.set_u32(at + 8, words.len() as u32);
    }

    #[test]
    fn clear_destroys_items_and_frees_with_the_byte() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.call(0x006c_6200, &args![this, 1u32]);
        assert!(calls(&e, ARRAY_DESTRUCT_ITEMS_00822820).is_empty());
        e.mem.set_u32(this + 4, 0x9000);
        e.mem.set_u32(this + 8, 3);
        e.mem.set_u32(this + 0xc, 5);
        e.call(0x006c_6200, &args![this, 0x100u32]);
        assert_eq!(
            calls(&e, ARRAY_DESTRUCT_ITEMS_00822820),
            vec![vec![this, 0x9000, 3]]
        );
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0x9000, 0, 5)
        );
        e.call(0x006c_6200, &args![this, 1u32]);
        assert_eq!(calls(&e, ARRAY_FREE_BUFFER_006A8500), vec![vec![this]]);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (0, 0));
    }

    #[test]
    fn array_constructors_destructors_and_small_helpers() {
        let mut e = rig();
        let a = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0082_2530, &args![a]).u32(), a);
        assert_eq!(e.mem.u32(a), PARTICLE_ARRAY_VTABLE);
        assert_eq!(e.call(0x0082_2580, &args![a]).u32(), a);
        assert_eq!(e.mem.u32(a), OBJECT_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_INIT_00822860), vec![vec![a, 0, 0]; 2]);
        e.mem.set_u32(a + 4, 0x9000);
        e.call(0x0082_2560, &args![a]);
        assert_eq!(e.mem.u32(a), PARTICLE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(a + 4), 0);
        e.mem.set_u32(a + 4, 0x9000);
        e.call(0x0082_25b0, &args![a]);
        assert_eq!(e.mem.u32(a), OBJECT_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_FREE_BUFFER_006A8500).len(), 2);
        e.call(0x0082_2630, &args![a]);
        assert_eq!(e.mem.u32(a), STRING_ARRAY_VTABLE);
        assert_eq!(calls(&e, STRING_ARRAY_CLEAR_008454F0), vec![vec![a, 1]]);
        e.mem.set_u32(a, 7);
        assert_eq!(e.call(0x0082_2510, &args![a, 7u32]).u32(), 1);
        assert_eq!(e.call(0x0082_2510, &args![a, 8u32]).u32(), 0);
    }

    #[test]
    fn float_accessors() {
        let mut e = rig();
        let this = e.mem.alloc(0x150);
        e.call(0x0082_1600, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x78), 0x7fc0_0001);
        e.mem.set_f32(this + 0x134, 1.5);
        e.mem.set_f32(this + 0x138, -2.5);
        e.mem.set_u32(this + 0x140, 0x7f80_0001);
        assert_eq!(e.call(0x0082_1640, &args![this]).st0, 1.5);
        assert_eq!(e.call(0x0082_1660, &args![this]).st0, -2.5);
        assert!(e.call(0x0082_1680, &args![this]).st0.is_nan());
    }

    #[test]
    fn constructor_builds_the_members_and_loads_the_add_ons() {
        let mut e = rig();
        let player = object(&mut e, 0x70);
        e.set_global(PLAYER, player);
        e.set_global(MODEL_LOADER_011C3B3C, 0x4400u32);
        e.register(PLAYER_NODE_00950BB0, |_, _| 0x5500u32.into_ret());
        e.register(QUERY_00456610, |_, _| 0x100u32.into_ret());
        e.register(MAGIC_ITEM_GET_AV_EFFECT, |_, a| (a[0] + 1).into_ret());
        // The shader's add-on list: one node naming model 0x7700, then an empty one.
        let shader = e.mem.alloc(0x120);
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(shader + 0x10c, source);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data + 4, 0x7700);
        e.mem.set_u32(source + 0x1c, data);
        let setting = e.mem.alloc(0x80);
        e.mem.set_u32(setting + 0x78, shader);
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| setting.into_ret());
        let effect = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + 0x1c, player);
        e.call(0x0081_f350, &args![this, 0x11u32, effect]);
        assert_eq!(
            calls(&e, MAGIC_HIT_EFFECT_CONSTRUCTOR),
            vec![vec![this, 0x11, effect]]
        );
        assert_eq!(e.mem.u32(this), MAGIC_SHADER_HIT_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x38), PARTICLE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x50), OBJECT_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), shader);
        assert_eq!(e.mem.f32(this + 0x60), 1.0);
        assert_eq!(e.mem.f32(this + 0x64), 1.0);
        // The node query answers a zero low byte: third person.
        assert_eq!(e.mem.u8(this + 0x4c), 1);
        assert_eq!(
            calls(&e, MODEL_LOADER_LOAD_FILE),
            vec![vec![0x4400, 0x7700, 0, 1, 0, 0, 0]]
        );
    }

    #[test]
    fn destructor_stops_the_shaders_and_releases_the_members() {
        let mut e = rig();
        let player = object(&mut e, 0x70);
        e.set_global(PLAYER, player);
        e.register(FORM_ID_0084E3A0, |_, _| FIRE_SHADER_84A51.into_ret());
        e.register(PLAYER_NODE_00950BB0, |_, a| (0x5500 + a[1]).into_ret());
        e.register(NAME_STRING_00408DA0, |_, _| 0x6600u32.into_ret());
        e.register(STRING_LENGTH_0048CEE0, |_, _| 4u32.into_ret());
        e.register(BS_AUDIO_INSTANCE, |_, _| 0x4300u32.into_ret());
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + 0x1c, player);
        e.mem.set_u32(player + 0x44, 0x5600);
        let shader = e.mem.alloc(0x120);
        e.mem.set_u32(this + 0x30, shader);
        e.mem.set_u32(this + 0x68, 0x7100);
        e.mem.set_u32(this + 0x48, 0x7200);
        array(&mut e, this + 0x38, &[0x7300]);
        let parent = object(&mut e, 0x70);
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(child + 0x18, parent);
        array(&mut e, this + 0x50, &[child]);
        e.call(0x0081_f770, &args![this]);
        assert_eq!(e.mem.u32(this), MAGIC_SHADER_HIT_EFFECT_VTABLE);
        assert_eq!(
            calls(&e, STOP_TEXTURE_SHADER),
            vec![vec![shader, 0x5501, 0x7100], vec![shader, 0x5500, 0x7100]]
        );
        assert_eq!(
            calls(&e, STOP_PARTICLE_SHADER),
            vec![vec![shader, 0x7200, 0x7300]]
        );
        assert_eq!(calls(&e, SLOT_BASE + 0xe8), vec![vec![parent, child]]);
        assert_eq!(calls(&e, ATTACH_PARENT_00A59D00), vec![vec![0x7200, 0]]);
        assert_eq!((e.mem.u32(this + 0x48), e.mem.u32(this + 0x68)), (0, 0));
        assert_eq!(
            calls(&e, BS_AUDIO_STOP_MOVING_SOUNDS),
            vec![vec![0x4300, 0x5600, 1.0f32.to_bits(), 0]]
        );
        assert_eq!(calls(&e, TES_REMOVE_TEXTURE_IMAGE).len(), 2);
        assert_eq!(calls(&e, MAGIC_HIT_EFFECT_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = rig();
        let this = e.mem.alloc(0x6c);
        assert_eq!(e.call(0x0081_f320, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        e.call(0x0081_f320, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE_SIZED_00AA1460), vec![vec![this, 0x6c]]);
        assert_eq!(calls(&e, MAGIC_HIT_EFFECT_DESTRUCTOR).len(), 2);
    }

    fn outputs(e: &mut Engine) -> u32 {
        e.mem.alloc(0x20)
    }

    #[test]
    fn attach_point_of_the_player_sets_the_first_flag() {
        let mut e = rig();
        let player = object(&mut e, 0x70);
        e.set_global(PLAYER, player);
        e.mem.set_u32(player + 0x40, 1);
        e.mem.set_u32(player + 0x44, 0x5600);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(PLAYER_NODE_00950BB0, |_, a| (0x5500 + a[1]).into_ret());
        e.register(PLAYER_GET_BIPED, |_, _| 0x5800u32.into_ret());
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + 0x1c, player);
        let out = outputs(&mut e);
        e.call(
            0x0082_0c10,
            &args![this, out, out + 1, out + 4, out + 8, out + 0xc],
        );
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.u8(out + 1), 0);
        // b3rdPerson is 0: the player's node with 1.
        assert_eq!((e.mem.u32(out + 8), e.mem.u32(out + 0xc)), (0x5501, 0x5501));
        assert_eq!(calls(&e, PLAYER_GET_BIPED), vec![vec![player, 1]]);
    }

    #[test]
    fn attach_point_of_another_object_measures_its_bound() {
        let mut e = rig();
        let target = object(&mut e, 0x70);
        e.mem.set_u32(target + 0x44, 0x5600);
        e.register(FORM_007AF430, |_, _| 0x5900u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| {
            u32::from(a[2] == RTTI_01183108)
                .wrapping_mul(a[0])
                .into_ret()
        });
        e.register(BOUND_RADIUS_0084D030, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(POINT_LENGTH_00457990, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + 0x1c, target);
        let out = outputs(&mut e);
        e.call(
            0x0082_0c10,
            &args![this, out, out + 1, out + 4, out + 8, out + 0xc],
        );
        assert_eq!((e.mem.u8(out), e.mem.u8(out + 1)), (0, 1));
        assert_eq!(e.mem.f32(out + 4), 2.5);
        assert_eq!(e.mem.u32(out + 8), 0x5600);
        // The radius is above 0: the node is not moved.
        assert!(calls(&e, SET_TRANSLATE_00A59C60).is_empty());
        e.register(BOUND_RADIUS_0084D030, |_, _| Ret::default());
        e.call(
            0x0082_0c10,
            &args![this, out, out + 1, out + 4, out + 8, out + 0xc],
        );
        assert_eq!(calls(&e, SET_TRANSLATE_00A59C60).len(), 1);
        assert_eq!(e.mem.f32(out + 4), 0.5);
    }

    #[test]
    fn attach_point_of_a_weapon_enchantment_hands_over_the_acquire_object() {
        let mut e = rig();
        let target = object(&mut e, 0x70);
        e.mem.set_u32(target + 0x40, 1);
        let acquired = object(&mut e, 0x70);
        e.register_double(SAVED_ACQUIRE_OBJECT_008D8520, move |_, _| {
            acquired.into_ret()
        });
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + 0x1c, target);
        e.mem.set_u8(this + 0x28, 1);
        let out = outputs(&mut e);
        e.mem.set_u32(out + 8, 0xdead);
        e.call(
            0x0082_0c10,
            &args![this, out, out + 1, out + 4, out + 8, out + 0xc],
        );
        assert_eq!(calls(&e, SLOT_BASE + 0x148), vec![vec![acquired]]);
        assert_eq!(e.mem.u32(out + 8), 0);
    }

    /// An effect for `Init` whose target is a plain object with a node, an effect shader
    /// with a fill texture and both attach nodes.
    fn init_rig() -> (Engine, u32, u32) {
        let mut e = rig();
        let target = object(&mut e, 0x70);
        e.mem.set_u32(target + 0x44, 0x5600);
        e.register(FORM_007AF430, |_, _| 0x5900u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| {
            u32::from(a[2] == RTTI_01183108)
                .wrapping_mul(a[0])
                .into_ret()
        });
        let this = object(&mut e, 0x6c);
        e.mem.set_u32(this + 0x1c, target);
        let shader = e.mem.alloc(0x120);
        e.mem.set_u32(this + 0x30, shader);
        (e, this, shader)
    }

    #[test]
    fn init_without_nodes_answers_false() {
        let (mut e, this, _) = init_rig();
        let target = e.mem.u32(this + 0x1c);
        e.mem.set_u32(target + 0x44, 0);
        assert!(!e.call(0x0081_fc70, &args![this]).bool());
        assert!(calls(&e, SHADER_FLAG_00507040).is_empty());
    }

    #[test]
    fn init_starts_particles_and_the_texture_shader() {
        let (mut e, this, shader) = init_rig();
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_NODE_CONSTRUCT_00A5ECB0, |_, a| a[0].into_ret());
        e.register(STRING_LENGTH_0048CEE0, |_, _| 3u32.into_ret());
        e.register(FLOAT_HELPER_00404010, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        e.register(PARTICLE_STEP_00BA52D0, |_, _| 1u32.into_ret());
        e.register(START_PARTICLE_SHADER, |_, _| 0x7300u32.into_ret());
        let data = e.mem.alloc(0x70);
        e.register_double(TEXTURE_EFFECT_DATA_00507830, move |_, _| data.into_ret());
        assert!(e.call(0x0081_fc70, &args![this]).bool());
        let node = e.mem.u32(this + 0x48);
        assert_ne!(node, 0);
        assert_eq!(
            calls(&e, SET_NAME_00A5B950)[0][0],
            node,
            "the new node is named"
        );
        // ftol(2.0) / 1: two particle properties.
        assert_eq!(e.mem.u32(this + 0x38 + 8), 2);
        assert_eq!(calls(&e, START_PARTICLE_SHADER).len(), 2);
        assert_eq!(
            calls(&e, START_PARTICLE_SHADER)[0][..3],
            [shader, node, 0x5600]
        );
        assert_eq!(calls(&e, COLLECT_TARGETS), vec![vec![0x7300, 0x5600]; 2]);
        assert_eq!(calls(&e, PARTICLE_THIRD_PERSON_00820B90).len(), 2);
        assert!(calls(&e, UPDATE_PARTICLES).is_empty());
        assert_eq!(calls(&e, SLOT_BASE + 0xcc), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0x68), data);
        assert_eq!(
            calls(&e, START_TEXTURE_SHADER),
            vec![vec![shader, 0x5600, data]]
        );
        assert_eq!(calls(&e, TES_CREATE_TEXTURE_IMAGE).len(), 3);
    }

    #[test]
    fn init_stops_particles_when_the_shader_has_flag_8() {
        let (mut e, this, shader) = init_rig();
        e.register(SHADER_FLAG_00507040, |_, _| 1u32.into_ret());
        e.mem.set_u32(this + 0x48, 0x7200);
        array(&mut e, this + 0x38, &[0x7300]);
        assert!(!e.call(0x0081_fc70, &args![this]).bool());
        assert_eq!(
            calls(&e, STOP_PARTICLE_SHADER),
            vec![vec![shader, 0x7200, 0x7300]]
        );
        assert_eq!(e.mem.u32(this + 0x48), 0);
        // Flag 1 is also set: no texture shader.
        assert!(calls(&e, START_TEXTURE_SHADER).is_empty());
    }

    #[test]
    fn init_restarts_the_players_particles_and_attaches_add_ons() {
        let (mut e, this, shader) = init_rig();
        let player = e.mem.u32(this + 0x1c);
        e.set_global(PLAYER, player);
        e.mem.set_u32(player + 0x40, 1);
        e.mem.set_f32(this + 0x20, 1.0);
        e.mem.set_f32(this + 0x34, 1.0);
        // An active effect (the word at +0x18) with flag 0x10.
        let effect = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x18, effect);
        e.register(ACTIVE_EFFECT_AGE_006B9130, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register(ACTIVE_EFFECT_FLAG_00804B80, |_, _| 1u32.into_ret());
        e.register(ITEM_FLAG_00403E60, |_, _| 1u32.into_ret());
        e.register(PLAYER_NODE_00950BB0, |_, _| 0x5600u32.into_ret());
        e.register(SHADER_FLAG_00507040, |_, a| u32::from(a[1] == 1).into_ret());
        e.mem.set_u32(this + 0x48, 0x7200);
        let property = e.mem.alloc(0x10);
        array(&mut e, this + 0x38, &[property]);
        // The add-on list and the property's objects.
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(shader + 0x10c, source);
        let data = e.mem.alloc(8);
        e.mem.set_u8(data, 3);
        e.mem.set_u32(data + 4, 0x7700);
        e.mem.set_u32(source + 0x1c, data);
        e.register(COUNT_00658930, |_, _| 1u32.into_ret());
        let holder = object(&mut e, 0x70);
        let parent = object(&mut e, 0x70);
        e.mem.set_u32(holder + 0x5c, parent);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, holder);
        e.register_double(ELEMENT_00877A30, move |_, _| element.into_ret());
        e.register(FIND_MODEL, |e, a| {
            e.mem.set_u32(a[2], 0x7900);
            Ret::default()
        });
        e.register(MODEL_ROOT_0043B230, |_, _| 0x7a00u32.into_ret());
        e.register(NI_OBJECT_CLONE, |_, _| 0x7b00u32.into_ret());
        e.register(FLOAT_HELPER_00404010, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        assert!(e.call(0x0081_fc70, &args![this]).bool());
        assert_eq!(e.mem.f32(this + 0x20), 1.0);
        assert_eq!(e.mem.u8(this + 0x24), 0);
        assert_eq!(e.mem.f32(this + 0x34), 100.0);
        // b3rdPerson goes from 0 to 1 while fAge is above 0: a restart, so no add-ons.
        assert_eq!(e.mem.u8(this + 0x4c), 1);
        assert_eq!(calls(&e, INIT_PARTICLE_DATA), vec![vec![property, 0]]);
        let updates = calls(&e, UPDATE_PARTICLES);
        assert_eq!(updates.len(), 2);
        assert_eq!(updates[0][1], ((100.0f32 as f64 - 0.166) as f32).to_bits());
        assert_eq!(updates[1][1], 100.0f32.to_bits());
        assert!(calls(&e, WEIGHTED_RANDOM_CONSTRUCT_00822440).is_empty());
        // A second run without a change attaches the add-on clone.
        e.call(0x0081_fc70, &args![this]);
        assert_eq!(calls(&e, ADD_WEIGHTED_RESULT)[0][1..], [0x7700, 3]);
        assert_eq!(calls(&e, SLOT_BASE + 0xdc), vec![vec![parent, 0x7b00, 1]]);
        assert_eq!(e.mem.f32(this + 0x64), 0.25);
        assert_eq!(e.mem.f32(this + 0x60), 0.0);
        assert_eq!(e.mem.u32(this + 0x50 + 8), 1);
        assert_eq!(calls(&e, START_ANIMATIONS), vec![vec![0x7b00]]);
    }
}

#[cfg(test)]
mod tests_n048 {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn zero(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// An object whose vtable sends each listed slot to `code + slot`, a
    /// double answering the paired word.
    fn object(e: &mut Engine, code: u32, slots: &[(u32, u32)]) -> u32 {
        let size = slots.iter().map(|s| s.0).max().unwrap_or(0) + 4;
        let vtable = e.mem.alloc(size);
        for &(slot, answer) in slots {
            e.mem.set_u32(vtable + slot, code + slot);
            e.register_double(code + slot, move |_, _| ret(answer));
        }
        let object = e.mem.alloc(0x700);
        e.mem.set_u32(object, vtable);
        object
    }

    const EFFECT: u32 = 0x0300_0000;
    const TARGET: u32 = 0x0310_0000;
    const PROCESS: u32 = 0x0320_0000;
    const ADDON: u32 = 0x0330_0000;
    const ATTACH: u32 = 0x0340_0000;
    const BUFFER: u32 = 0x0350_0000;

    /// `NiPointer`s holding their object in their first word, the arrays
    /// as `[count at +8, item i at items + 4 i]` (`0044ddc0`, `006a7ad0`),
    /// the globals mapped.
    fn rig(e: &mut Engine) {
        e.register(WORD_AT_0_00559450, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_ASSIGN_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(WORD_AT_8_0044DDC0, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(
            ARRAY_AT_006A7AD0,
            |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]),
        );
        e.register(WORD_DIFFERS_0052AA80, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) != a[1]))
        });
        for addr in [PLAYER, SAVE_LOAD_GAME] {
            e.map(addr, 4);
        }
        e.mem.set_u32(SAVE_LOAD_GAME, 0x5a1d);
        for (addr, value) in [
            (ZERO_01012060, 0.0f64),
            (ALPHA_LIMIT_0106D508, 0.99),
            (MINUS_ONE_DOUBLE, -1.0),
        ] {
            e.map(addr, 8);
            e.set_global::<f64>(addr, value);
        }
        for (addr, bits) in [
            (LONGEST_LIFETIME, f32::MAX.to_bits()),
            (LOWER_ALPHA_0106682C, 0.99f32.to_bits()),
            (ADDON_SCALE_01013EA4, 0.01f32.to_bits()),
        ] {
            e.map(addr, 4);
            e.set_global::<u32>(addr, bits);
        }
    }

    /// Fills the array at `this + array` with the words (each slot an
    /// `NiPointer` holding its item).
    fn word_array(e: &mut Engine, this: u32, array: u32, words: &[u32]) {
        let buffer = e.mem.alloc(4 * words.len() as u32 + 4);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *word);
        }
        e.mem.set_u32(this + array + 4, buffer);
        e.mem.set_u32(this + array + 8, words.len() as u32);
    }

    fn effect(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<MagicShaderHitEffect> {
        Ptr::new(object(e, EFFECT, slots))
    }

    #[test]
    fn init_load_game_forwards() {
        let mut e = Engine::new();
        zero(&mut e, &[HIT_EFFECT_INIT_LOAD_GAME]);
        let (_, log) = run(&mut e, 0x0081_d390, &args![0x10u32, 0x20u32, 0x30u32]);
        assert_eq!(
            calls(&log, HIT_EFFECT_INIT_LOAD_GAME),
            vec![vec![0x10, 0x20, 0x30]]
        );
    }

    #[test]
    fn second_constructor_takes_the_shader_and_lifetime() {
        let mut e = Engine::new();
        rig(&mut e);
        zero(
            &mut e,
            &[
                MAGIC_HIT_EFFECT_CONSTRUCTOR,
                ARRAY_INIT_00822860,
                NI_POINTER_FROM_00633C90,
                MODEL_LOADER_LOAD_FILE,
            ],
        );
        e.register(WORD_AT_10C_005A29B0, |_, _| ret(0));
        let this = e.new_object::<MagicShaderHitEffect>();
        e.mem.write(this.addr(), &[0xffu8; 0x6c]);
        let (r, log) = run(
            &mut e,
            0x0081_f580,
            &args![this, 0x7a7au32, 0x5aedu32, (-2.0f32).to_bits()],
        );
        assert_eq!(r.u32(), this.addr());
        assert_eq!(
            calls(&log, MAGIC_HIT_EFFECT_CONSTRUCTOR),
            vec![vec![this.addr(), 0x7a7a, 0]]
        );
        assert_eq!(e.mem.u32(this.addr()), MAGIC_SHADER_HIT_EFFECT_VTABLE);
        assert_eq!(e.get(this, MagicShaderHitEffect::pEffectData), 0x5aed);
        assert_eq!(e.get(this, MagicShaderHitEffect::fAlphaTimer), 0);
        assert_eq!(e.get(this, MagicShaderHitEffect::spParticleNode), 0);
        assert_eq!(e.get(this, MagicShaderHitEffect::spTexEffectData), 0);
        assert_eq!(e.get(this, MagicShaderHitEffect::bWeaponEnchantment), 0);
        assert_eq!(e.get(this, MagicShaderHitEffect::pWornObject), 0);
        assert_eq!(
            e.get(this, MagicShaderHitEffect::fAddonAlpha),
            1.0f32.to_bits()
        );
        assert_eq!(e.get(this, MagicShaderHitEffect::b3rdPerson), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), f32::MAX.to_bits());
        assert_eq!(e.mem.u32(this.addr() + 0x38), PARTICLE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x50), OBJECT_ARRAY_VTABLE);
        // The player's effect in third person; a positive lifetime kept.
        e.mem.write(this.addr(), &[0u8; 0x6c]);
        e.mem.set_u32(this.addr() + 0x1c, 0x7a7a);
        e.mem.set_u32(PLAYER, 0x7a7a);
        e.register(MAGIC_HIT_EFFECT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x1c, a[1]);
            Ret::default()
        });
        e.register(PLAYER_NODE_00950BB0, |_, _| ret(0x0de));
        e.register(QUERY_00456610, |_, _| ret(0));
        e.call(
            0x0081_f580,
            &args![this, 0x7a7au32, 0x5aedu32, 3.0f32.to_bits()],
        );
        assert_eq!(e.get(this, MagicShaderHitEffect::b3rdPerson), 1);
        assert_eq!(e.mem.u32(this.addr() + 8), 3.0f32.to_bits());
    }

    #[test]
    fn small_accessors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x6a8);
        assert_eq!(e.call(0x0082_0b50, &args![this]).u32(), this + 0x11c);
        e.call(0x0082_0b70, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x64), 2);
        assert_eq!(e.mem.u32(this + 0x68), 0x7fc0_0001);
        e.call(0x0082_0b90, &args![this]);
        assert_eq!(e.mem.u32(this + 0x64), 1);
        e.call(0x0082_0bb0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        e.call(0x0082_0bd0, &args![this, 0x1ffu32]);
        assert_eq!(e.mem.u8(this + 0x6c), 0xff);
        e.mem.set_u32(this + 0x6a4, 0xace);
        assert_eq!(e.call(0x0082_0bf0, &args![this]).u32(), 0xace);
        e.call(0x0082_1620, &args![this, 2.5f32.to_bits()]);
        assert_eq!(e.mem.u32(this + 0x130), 2.5f32.to_bits());
        e.mem.set_u32(this + 0x148, 4.0f32.to_bits());
        assert_eq!(e.call(0x0082_16a0, &args![this]).f64(), 4.0);
    }

    /// An effect with a live target (`TARGET` slots), a shader and
    /// `bFinished` set as given.
    fn update_world(e: &mut Engine, actor: u32) -> (Ptr<MagicShaderHitEffect>, u32) {
        rig(e);
        let target = object(e, TARGET, &[(0x1d0, 0x0d0d), (0x100, actor), (0x420, 0)]);
        e.register(TARGET + 0x420, |_, _| st0(2.0));
        let this = effect(e, &[(0xc8, 0), (0xcc, 0)]);
        e.set(this, MagicShaderHitEffect::pTarget, target);
        e.set(this, MagicShaderHitEffect::pEffectData, 0x5aed);
        e.register(HIT_EFFECT_UPDATE, |_, _| ret(1));
        e.register(SKIPPED_SHADER_0080D0D0, |_, _| ret(0x5aed));
        (this, target)
    }

    #[test]
    fn update_answers_false_when_the_base_or_target_ends() {
        let mut e = Engine::new();
        let (this, _) = update_world(&mut e, 0);
        e.register(HIT_EFFECT_UPDATE, |_, _| ret(0x100));
        let (r, log) = run(&mut e, 0x0082_0f30, &args![this, 0.5f32.to_bits()]);
        assert_eq!(r.u32(), 0);
        assert_eq!(calls(&log, EFFECT + 0xc8), vec![vec![this.addr()]]);
        assert!(calls(&log, EFFECT + 0xcc).is_empty());
        e.register(HIT_EFFECT_UPDATE, |_, _| ret(1));
        e.register(TARGET + 0x1d0, |_, _| ret(0));
        let (r, _) = run(&mut e, 0x0082_0f30, &args![this, 0.5f32.to_bits()]);
        assert_eq!(r.u32(), 0);
    }

    #[test]
    fn update_steps_particles_addons_and_texture() {
        let mut e = Engine::new();
        let (this, target) = update_world(&mut e, 1);
        let base = this.addr();
        e.register(SKIPPED_SHADER_0080D0D0, |_, _| ret(0));
        e.set(this, MagicShaderHitEffect::fAlphaTimer, 1.0f32.to_bits());
        e.set(this, MagicShaderHitEffect::fAddonAlpha, 0.25f32.to_bits());
        e.set(this, MagicShaderHitEffect::fAddonScale, 0.5f32.to_bits());
        // A texture effect whose +0x6C object is set, a target at alpha 1.
        let texture = e.mem.alloc(0x70);
        e.mem.set_u32(texture + 0x6c, 0x0b0b);
        e.mem.set_u32(texture + 0x18, 2.0f32.to_bits());
        e.mem.set_u32(texture + 0x28, 3.0f32.to_bits());
        e.set(this, MagicShaderHitEffect::spTexEffectData, texture);
        e.register(ACTOR_GET_ALPHA_MULT, |_, _| st0(1.0));
        zero(
            &mut e,
            &[
                ACTOR_SET_ALPHA_MULT,
                ACTOR_UPDATE_ALPHA,
                SHADER_TEXTURE_UPDATE_00507E30,
                UPDATE_PARTICLES,
                RECURSIVE_SET_PROPERTY_FADE_ALPHA,
                SET_SCALE_00440490,
            ],
        );
        e.register(NODE_TEST_0043B300, |_, _| ret(0));
        // Particles: one property with rate 2, step 3.
        e.set(this, MagicShaderHitEffect::spParticleNode, 0x9a9e);
        let property = e.mem.alloc(0x150);
        word_array(&mut e, base, PARTICLE_PROPERTY_ARRAY, &[property]);
        e.register(PARTICLE_RATE_006A78F0, |_, _| st0(2.0));
        e.register(PARTICLE_STEP_00BA52D0, |_, _| ret(3));
        e.register(SHADER_PARTICLES_00507DA0, |_, _| st0(1.5));
        e.register(MIN_0040EBD0, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            st0(f64::from(if x < y { x } else { y }))
        });
        e.register(FLOAT_HELPER_00404010, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            st0(f64::from(if x > y { x } else { y }))
        });
        e.register(PARTICLE_LIVE_004FD380, |_, _| ret(0));
        // Add-ons: one object, fade times 4 (in) and 8.
        let addon = object(&mut e, ADDON, &[(0xc, 0x4e0d)]);
        word_array(&mut e, base, ADD_ON_OBJECTS, &[addon]);
        let shader = e.mem.alloc(0x150);
        e.mem.set_u32(shader + 0x134, 4.0f32.to_bits());
        e.set(this, MagicShaderHitEffect::pEffectData, shader);
        e.register(SHADER_FADE_IN_008133B0, |_, _| st0(8.0));
        e.register(SHADER_VALUE_00567470, |_, _| st0(0.5));
        let (r, log) = run(&mut e, 0x0082_0f30, &args![this, 0.5f32.to_bits()]);
        assert_eq!(r.u32(), 1);
        assert_eq!(
            e.get(this, MagicShaderHitEffect::fAlphaTimer),
            1.5f32.to_bits()
        );
        // Alpha 1.0 above 0.99: lowered.
        assert_eq!(
            calls(&log, ACTOR_SET_ALPHA_MULT),
            vec![vec![target, 0.99f32.to_bits()]]
        );
        // Total rate 3 * 2 = 6.
        let stepped = calls(&log, SHADER_PARTICLES_00507DA0);
        assert_eq!(
            stepped,
            vec![vec![
                shader,
                6.0f32.to_bits(),
                0.5f32.to_bits(),
                1.5f32.to_bits(),
                0
            ]]
        );
        // min(1.5, 3) = 1.5 shared by 3: 0.5.
        assert_eq!(e.mem.u32(property + 0x78), 0.5f32.to_bits());
        // The scale: slot +0x420's 2.0 over alpha 1.0.
        assert_eq!(e.mem.u32(property + 0x130), 2.0f32.to_bits());
        assert_eq!(
            calls(&log, UPDATE_PARTICLES),
            vec![vec![property, 1.5f32.to_bits(), 0, 1]]
        );
        // Add-on alpha 0.25 + 0.5 / 4 = 0.375, scale 0.5 + 0.5 / 8 = 0.5625.
        assert_eq!(
            e.get(this, MagicShaderHitEffect::fAddonAlpha),
            0.375f32.to_bits()
        );
        assert_eq!(
            e.get(this, MagicShaderHitEffect::fAddonScale),
            0.5625f32.to_bits()
        );
        assert_eq!(
            calls(&log, RECURSIVE_SET_PROPERTY_FADE_ALPHA),
            vec![vec![0x4e0d, 0.375f32.to_bits()]]
        );
        // Size (1 - 0.5) * 0.5625 + 0.5.
        assert_eq!(
            calls(&log, SET_SCALE_00440490),
            vec![vec![addon, 0.78125f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, SHADER_TEXTURE_UPDATE_00507E30),
            vec![vec![
                shader,
                texture,
                0.5f32.to_bits(),
                1.5f32.to_bits(),
                0,
                0
            ]]
        );
        assert_eq!(e.mem.u32(texture + 0x18), 4.0f32.to_bits());
        assert_eq!(e.mem.u32(texture + 0x28), 6.0f32.to_bits());
    }

    #[test]
    fn update_releases_a_finished_effect() {
        let mut e = Engine::new();
        let (this, target) = update_world(&mut e, 1);
        e.set(this, MagicShaderHitEffect::bFinished, 1);
        let process = object(&mut e, PROCESS, &[(0x580, 0)]);
        e.register_double(SAVED_ACQUIRE_OBJECT_008D8520, move |_, _| ret(process));
        let (r, log) = run(&mut e, 0x0082_0f30, &args![this, 0.5f32.to_bits()]);
        assert_eq!(r.u32(), 0);
        assert_eq!(calls(&log, PROCESS + 0x580), vec![vec![process, 1, 1, 0]]);
        assert_eq!(calls(&log, EFFECT + 0xc8), vec![vec![this.addr()]]);
        assert_eq!(calls(&log, SAVED_ACQUIRE_OBJECT_008D8520)[0], vec![target]);
        // A texture alpha still above 0 keeps it.
        let texture = e.mem.alloc(0x70);
        e.mem.set_u32(texture + 0x38, 0.5f32.to_bits());
        e.set(this, MagicShaderHitEffect::spTexEffectData, texture);
        zero(&mut e, &[SHADER_TEXTURE_UPDATE_00507E30]);
        let (r, log) = run(&mut e, 0x0082_0f30, &args![this, 0.5f32.to_bits()]);
        assert_eq!(r.u32(), 1);
        assert!(calls(&log, PROCESS + 0x580).is_empty());
    }

    #[test]
    fn reset_alpha_timer_scales_the_shader_time() {
        let mut e = Engine::new();
        rig(&mut e);
        let this = effect(&mut e, &[]);
        let base = this.addr();
        let texture = e.mem.alloc(0x50);
        e.mem.set_u32(texture + 0x38, 0.25f32.to_bits());
        e.mem.set_u32(texture + 0x48, 0.5f32.to_bits());
        e.set(this, MagicShaderHitEffect::spTexEffectData, texture);
        e.register(FLOAT_HELPER_00404010, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            st0(f64::from(if x > y { x } else { y }))
        });
        e.register(SHADER_TIME_009A7C40, |_, _| st0(1.0));
        e.register(SHADER_TIME_00759450, |_, _| st0(8.0));
        let property = e.mem.alloc(4);
        word_array(&mut e, base, PARTICLE_PROPERTY_ARRAY, &[property]);
        zero(&mut e, &[PARTICLE_SET_TIME_00BA6CF0]);
        let (_, log) = run(&mut e, 0x0082_16c0, &args![this]);
        assert_eq!(
            calls(&log, FLOAT_HELPER_00404010),
            vec![vec![0.25f32.to_bits(), 0.5f32.to_bits()]]
        );
        assert_eq!(
            e.get(this, MagicShaderHitEffect::fAlphaTimer),
            4.0f32.to_bits()
        );
        assert_eq!(
            calls(&log, PARTICLE_SET_TIME_00BA6CF0),
            vec![vec![property, 4.0f32.to_bits()]]
        );
        // Alpha not below the full alpha: the time itself.
        e.register(SHADER_TIME_009A7C40, |_, _| st0(0.5));
        e.call(0x0082_16c0, &args![this]);
        assert_eq!(
            e.get(this, MagicShaderHitEffect::fAlphaTimer),
            8.0f32.to_bits()
        );
        // A weapon enchantment clears the texture's alphas.
        e.set(this, MagicShaderHitEffect::bWeaponEnchantment, 1);
        e.register(SHADER_TIME_009A7C40, |_, _| st0(1.0));
        e.call(0x0082_16c0, &args![this]);
        assert_eq!(e.mem.u32(texture + 0x38), 0);
        assert_eq!(e.mem.u32(texture + 0x48), 0);
        assert_eq!(e.get(this, MagicShaderHitEffect::fAlphaTimer), 0);
    }

    fn position_world(e: &mut Engine) -> (Ptr<MagicShaderHitEffect>, u32) {
        rig(e);
        let node = e.mem.alloc(0x40);
        let target = object(e, TARGET, &[(0x1d0, node)]);
        let this = effect(e, &[]);
        e.set(this, MagicShaderHitEffect::pTarget, target);
        e.set(this, MagicShaderHitEffect::spParticleNode, 0x9a9e);
        e.register_double(TARGET_00441110, move |_, _| ret(target));
        e.register(CAMERA_POSITION_0045BB80, |_, a| ret(a[0] + 0x30));
        zero(
            e,
            &[
                SET_TRANSLATE_00440460,
                NODE_SET_0043FA80,
                HIT_EFFECT_0081BC70,
                POINT_SET_0043D410,
                SET_TRANSLATE_00A59C60,
            ],
        );
        e.map(NODE_ARGUMENT_011A9448, 0x24);
        for i in 0..9 {
            e.mem.set_u32(NODE_ARGUMENT_011A9448 + 4 * i, 0x100 + i);
        }
        (this, node)
    }

    #[test]
    fn update_position_places_the_particle_node() {
        let mut e = Engine::new();
        let (this, node) = position_world(&mut e);
        let (_, log) = run(&mut e, 0x0082_17e0, &args![this]);
        assert_eq!(
            calls(&log, SET_TRANSLATE_00440460),
            vec![vec![0x9a9e, node + 0x30]]
        );
        let rotation = calls(&log, NODE_SET_0043FA80)[0][1];
        for i in 0..9 {
            assert_eq!(e.mem.u32(rotation + 4 * i), 0x100 + i);
        }
        assert_eq!(
            calls(&log, HIT_EFFECT_0081BC70),
            vec![vec![this.addr(), 0x9a9e]]
        );
        // No particle node: only the hand-over.
        e.set(this, MagicShaderHitEffect::spParticleNode, 0);
        let (_, log) = run(&mut e, 0x0082_17e0, &args![this]);
        assert!(calls(&log, SET_TRANSLATE_00440460).is_empty());
        assert_eq!(calls(&log, HIT_EFFECT_0081BC70), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn update_position_in_first_person() {
        let mut e = Engine::new();
        let (this, _) = position_world(&mut e);
        let target = e.get(this, MagicShaderHitEffect::pTarget);
        e.mem.set_u32(PLAYER, target);
        let player_node = e.mem.alloc(0x40);
        e.register_double(PLAYER_NODE_00950BB0, move |_, _| ret(player_node));
        e.register(QUERY_00456610, |_, _| ret(1));
        let attach = object(&mut e, ATTACH, &[(0xdc, 0)]);
        e.register_double(NODE_LOOKUP_00653270, move |_, _| ret(attach));
        e.register(WORD_AT_18_009611E0, |_, _| ret(0x9a7e));
        e.register(NODE_ROTATION_00461130, |_, a| ret(a[0] + 0x10));
        e.register(MATRIX_INVERSE, |_, a| ret(a[1]));
        e.register(MATRIX_MULTIPLY, |_, a| ret(a[1]));
        e.register(POINT_SUBTRACT_00439EF0, |_, a| ret(a[1]));
        e.register(MATRIX_TIMES_POINT_004B4500, |_, a| ret(a[1]));
        let (_, log) = run(&mut e, 0x0082_17e0, &args![this]);
        assert_eq!(
            calls(&log, NODE_LOOKUP_00653270),
            vec![vec![NODE_LOOKUP_ARGUMENT_011F4428, player_node]]
        );
        assert_eq!(calls(&log, ATTACH + 0xdc), vec![vec![attach, 0x9a9e, 1]]);
        assert_eq!(calls(&log, MATRIX_INVERSE)[0][0], 0x9a8e);
        let product = calls(&log, MATRIX_MULTIPLY)[0].clone();
        assert_eq!(
            calls(&log, NODE_SET_0043FA80),
            vec![vec![0x9a9e, product[1]]]
        );
        let offset = calls(&log, POINT_SUBTRACT_00439EF0)[0].clone();
        assert_eq!(offset[0], player_node + 0x30);
        assert_eq!(offset[2], 0x9a7e + 0x30);
        let placed = calls(&log, MATRIX_TIMES_POINT_004B4500)[0].clone();
        assert_eq!(placed[2], offset[1]);
        assert_eq!(
            calls(&log, SET_TRANSLATE_00440460),
            vec![vec![0x9a9e, placed[1]]]
        );
        assert_eq!(calls(&log, SET_TRANSLATE_00A59C60).len(), 1);
        assert_eq!(calls(&log, HIT_EFFECT_0081BC70), vec![vec![this.addr(), 0]]);
        // The player's active effect: nothing placed.
        e.mem.set_u32(target + 0x6a4, this.addr());
        e.register(QUERY_00456610, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0082_17e0, &args![this]);
        assert!(calls(&log, SET_TRANSLATE_00440460).is_empty());
        assert_eq!(calls(&log, HIT_EFFECT_0081BC70), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn clear_target_stops_everything() {
        let mut e = Engine::new();
        rig(&mut e);
        let node = e.mem.alloc(4);
        let target = object(&mut e, TARGET, &[(0x1d0, node)]);
        let this = effect(&mut e, &[]);
        let base = this.addr();
        e.set(this, MagicShaderHitEffect::pTarget, target);
        e.set(this, MagicShaderHitEffect::pEffectData, 0x5aed);
        e.set(this, MagicShaderHitEffect::spTexEffectData, 0x7e7e);
        e.set(this, MagicShaderHitEffect::spParticleNode, 0x9a9e);
        let property = e.mem.alloc(4);
        word_array(&mut e, base, PARTICLE_PROPERTY_ARRAY, &[property]);
        e.register(FORM_ID_0084E3A0, |_, _| ret(FIRE_SHADER_84A51));
        e.register(BS_AUDIO_INSTANCE, |_, _| ret(0xa0d1));
        zero(
            &mut e,
            &[
                BS_AUDIO_STOP_MOVING_SOUNDS,
                STOP_TEXTURE_SHADER,
                STOP_PARTICLE_SHADER,
                ARRAY_DESTRUCT_ITEMS_00822820,
                ARRAY_FREE_BUFFER_006A8500,
                ATTACH_PARENT_00A59D00,
            ],
        );
        let (_, log) = run(&mut e, 0x0082_1bd0, &args![this]);
        assert_eq!(
            calls(&log, BS_AUDIO_STOP_MOVING_SOUNDS),
            vec![vec![0xa0d1, node, 1.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls(&log, STOP_TEXTURE_SHADER),
            vec![vec![0x5aed, node, 0x7e7e]]
        );
        assert_eq!(
            calls(&log, STOP_PARTICLE_SHADER),
            vec![vec![0x5aed, 0x9a9e, property]]
        );
        assert_eq!(calls(&log, ATTACH_PARENT_00A59D00), vec![vec![0x9a9e, 0]]);
        assert_eq!(calls(&log, ARRAY_FREE_BUFFER_006A8500).len(), 1);
        assert_eq!(e.get(this, MagicShaderHitEffect::spParticleNode), 0);
        assert_eq!(e.get(this, MagicShaderHitEffect::spTexEffectData), 0);
        assert_eq!(e.get(this, MagicShaderHitEffect::pTarget), 0);
        // The player's effect stops on both views; another shader is silent.
        e.set(this, MagicShaderHitEffect::pTarget, target);
        e.set(this, MagicShaderHitEffect::spTexEffectData, 0x7e7e);
        e.mem.set_u32(PLAYER, target);
        e.register(PLAYER_NODE_00950BB0, |_, a| ret(0x100 + a[1]));
        e.register(FORM_ID_0084E3A0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0082_1bd0, &args![this]);
        assert!(calls(&log, BS_AUDIO_STOP_MOVING_SOUNDS).is_empty());
        assert_eq!(
            calls(&log, STOP_TEXTURE_SHADER),
            vec![vec![0x5aed, 0x101, 0x7e7e], vec![0x5aed, 0x100, 0x7e7e]]
        );
    }

    #[test]
    fn save_size_save_and_load() {
        let mut e = Engine::new();
        rig(&mut e);
        e.register(HIT_EFFECT_GET_SAVE_SIZE, |_, _| ret(0x10));
        e.register(SAVE_VERSION, |_, _| ret(0x36));
        assert_eq!(
            e.call(0x0082_1dd0, &args![0u32, 1u32, 2u32]).u32() & 0xffff,
            0x14
        );
        e.register(SAVE_VERSION, |_, _| ret(0x37));
        assert_eq!(
            e.call(0x0082_1dd0, &args![0u32, 1u32, 2u32]).u32() & 0xffff,
            0x19
        );
        let this = e.new_object::<MagicShaderHitEffect>();
        let base = this.addr();
        zero(
            &mut e,
            &[
                HIT_EFFECT_SAVE_GAME,
                SAVE_WRITE,
                HIT_EFFECT_LOAD_GAME,
                LOAD_READ,
            ],
        );
        e.register(SAVE_NUMERIC_ID, |e, a| ret(e.mem.u32(a[1])));
        e.register(FORM_ID_0084E3A0, |_, _| ret(0x00ab_cdef));
        e.set(this, MagicShaderHitEffect::pWornObject, 0x3030);
        let (_, log) = run(&mut e, 0x0082_1e40, &args![this, 1u32, 2u32]);
        assert_eq!(calls(&log, HIT_EFFECT_SAVE_GAME), vec![vec![base, 1, 2]]);
        assert_eq!(
            calls(&log, SAVE_WRITE),
            vec![vec![0x5a1d, base + 0x34, 4], vec![0x5a1d, base + 0x28, 1]]
        );
        let saved: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == SAVE_NUMERIC_ID)
            .map(|(_, w)| w[2])
            .collect();
        assert_eq!(saved, vec![4]);
        e.register(SAVE_VERSION, |_, _| ret(0x36));
        let (_, log) = run(&mut e, 0x0082_1e40, &args![this, 1u32, 2u32]);
        assert_eq!(calls(&log, SAVE_WRITE).len(), 1);
        // Load with a worn object id.
        e.register(SAVE_VERSION, |_, _| ret(0x37));
        e.register(LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        e.register(LOOKUP_FORM_004839C0, |_, a| ret(a[0] + 1));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        let (_, log) = run(&mut e, 0x0082_1ed0, &args![this, 1u32, 2u32]);
        assert_eq!(calls(&log, LOAD_READ)[1], vec![0x5a1d, base + 0x28, 1]);
        assert_eq!(
            calls(&log, RT_DYNAMIC_CAST),
            vec![vec![0x78, 0, TYPE_TES_FORM, RTTI_01183108, 0]]
        );
        assert_eq!(e.get(this, MagicShaderHitEffect::pWornObject), 0x79);
        e.register(LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        e.set(this, MagicShaderHitEffect::pWornObject, 5);
        e.call(0x0082_1ed0, &args![this, 1u32, 2u32]);
        assert_eq!(e.get(this, MagicShaderHitEffect::pWornObject), 5);
    }

    #[test]
    fn finish_init_load_game_and_finish_load_game() {
        let mut e = Engine::new();
        rig(&mut e);
        let this = effect(&mut e, &[(0xc4, 0), (0x94, 0)]);
        let base = this.addr();
        e.register(EFFECT + 0xc4, |e, a| {
            e.mem.set_u32(a[0] + 0x20, 0);
            e.mem.set_u8(a[0] + 0x24, 0);
            Ret::default()
        });
        e.set(this, MagicShaderHitEffect::fAge, 1.5f32.to_bits());
        e.set(this, MagicShaderHitEffect::bFinished, 1);
        zero(
            &mut e,
            &[
                HIT_EFFECT_FINISH_INIT_LOAD_GAME,
                HIT_EFFECT_FINISH_LOAD_GAME,
            ],
        );
        let (_, log) = run(&mut e, 0x0082_1f70, &args![this, 0u32, 2u32, 0x5aedu32]);
        assert_eq!(
            calls(&log, HIT_EFFECT_FINISH_INIT_LOAD_GAME),
            vec![vec![base, 0, 2, 0]]
        );
        assert_eq!(e.get(this, MagicShaderHitEffect::pEffectData), 0x5aed);
        assert_eq!(e.get(this, MagicShaderHitEffect::fAge), 1.5f32.to_bits());
        assert_eq!(e.get(this, MagicShaderHitEffect::bFinished), 1);
        assert_eq!(calls(&log, EFFECT + 0x94), vec![vec![base, 0]]);
        e.register(WORD_AT_8_0044DDC0, |_, a| ret(a[0] + 8));
        e.register(MAGIC_ITEM_GET_AV_EFFECT, |_, a| ret(a[0] + 1));
        e.register(WORD_AT_78_00407820, |_, a| ret(a[0] + 0x78));
        e.call(0x0082_1f70, &args![this, 0x1000u32, 2u32, 0x5aedu32]);
        assert_eq!(e.get(this, MagicShaderHitEffect::pEffectData), 0x1081);
        let (_, log) = run(&mut e, 0x0082_23d0, &args![this, 0xb0u32]);
        assert_eq!(
            calls(&log, HIT_EFFECT_FINISH_LOAD_GAME),
            vec![vec![base, 0xb0]]
        );
        assert_eq!(e.get(this, MagicShaderHitEffect::fAge), 1.5f32.to_bits());
        assert_eq!(calls(&log, EFFECT + 0x94), vec![vec![base, 0]]);
    }

    #[test]
    fn check_should_save() {
        let mut e = Engine::new();
        e.register(SKIPPED_SHADER_0080D0D0, |_, _| ret(0x5aed));
        let this = e.new_object::<MagicShaderHitEffect>();
        assert_eq!(e.call(0x0082_2010, &args![this]).u32(), 0);
        e.set(this, MagicShaderHitEffect::pTarget, 1);
        e.set(this, MagicShaderHitEffect::pEffectData, 2);
        assert_eq!(e.call(0x0082_2010, &args![this]).u32(), 1);
        e.set(this, MagicShaderHitEffect::pEffectData, 0x5aed);
        assert_eq!(e.call(0x0082_2010, &args![this]).u32(), 0);
        e.set(this, MagicShaderHitEffect::pEffectData, 2);
        e.set(this, MagicShaderHitEffect::bWeaponEnchantment, 1);
        assert_eq!(e.call(0x0082_2010, &args![this]).u32(), 0);
        e.set(this, MagicShaderHitEffect::bWeaponEnchantment, 0);
        e.mem.set_u32(this.addr() + 0x18, 3);
        assert_eq!(e.call(0x0082_2010, &args![this]).u32(), 0);
    }

    #[test]
    fn second_save_size_save_and_load() {
        let mut e = Engine::new();
        rig(&mut e);
        let this = effect(
            &mut e,
            &[(0xd0, 0x10), (0xd4, 0), (0xd8, 0), (0xdc, 0), (0xe0, 0)],
        );
        let base = this.addr();
        e.set(this, MagicShaderHitEffect::pTarget, 0x7a7a);
        e.set(this, MagicShaderHitEffect::pEffectData, 0x5aed);
        let (r, log) = run(&mut e, 0x0082_2060, &args![this]);
        assert_eq!(r.u32() & 0xffff, 0x18);
        assert_eq!(calls(&log, EFFECT + 0xd0), vec![vec![base, 0, 0x7a7a]]);
        e.register(FORM_ID_0084E3A0, |_, a| ret(a[0] + 1));
        e.register(SAVE_NUMERIC_ID, |e, a| ret(e.mem.u32(a[1])));
        let (_, log) = run(&mut e, 0x0082_20c0, &args![this]);
        let ids: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == SAVE_NUMERIC_ID)
            .map(|(_, w)| w[2])
            .collect();
        assert_eq!(ids, vec![4, 4]);
        assert_eq!(calls(&log, EFFECT + 0xd4), vec![vec![base, 0, 0x7a7a]]);
        // Load: a target in a cell that passes, a shader.
        let mut next = 0x100;
        e.register_double(LOAD_NUMERIC_ID, move |e, a| {
            next += 0x100;
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        e.register(LOOKUP_FORM_004839C0, |_, a| ret(a[0] + 1));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        e.register(REFERENCE_CELL_008D6F30, |_, _| ret(0xce11));
        e.register(CELL_TEST_00450FF0, |_, _| ret(1));
        let (r, log) = run(&mut e, 0x0082_2130, &args![this]);
        assert_eq!(r.u32(), 1);
        assert_eq!(e.get(this, MagicShaderHitEffect::pTarget), 0x202);
        assert_eq!(
            calls(&log, RT_DYNAMIC_CAST)[1],
            vec![0x301, 0, TYPE_TES_FORM, TYPE_TES_EFFECT_SHADER, 0]
        );
        assert_eq!(
            calls(&log, EFFECT + 0xe0),
            vec![vec![base, 0, 0x202, 0x302]]
        );
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[3] == TYPE_TES_EFFECT_SHADER {
                0
            } else {
                a[0]
            })
        });
        let (r, log) = run(&mut e, 0x0082_2130, &args![this]);
        assert_eq!(r.u32(), 0);
        assert_eq!(calls(&log, EFFECT + 0xd8).len(), 1);
        assert!(calls(&log, EFFECT + 0xdc).is_empty());
    }

    #[test]
    fn third_save_and_load() {
        let mut e = Engine::new();
        rig(&mut e);
        let this = e.new_object::<MagicShaderHitEffect>();
        let base = this.addr();
        e.set(this, MagicShaderHitEffect::pWornObject, 0x3030);
        e.set(this, MagicShaderHitEffect::pEffectData, 0x5aed);
        zero(
            &mut e,
            &[
                HIT_EFFECT_SAVE_GAME_OV2,
                BUFFER_WRITE_00865E50,
                BUFFER_SAVE_FORM_ID,
            ],
        );
        let (_, log) = run(&mut e, 0x0082_2250, &args![this, 0xb0u32]);
        assert_eq!(
            calls(&log, BUFFER_WRITE_00865E50),
            vec![
                vec![0xb0, base + 0x28, 1, 0],
                vec![0xb0, base + 0x34, 4, 0],
                vec![0xb0, base + 0x60, 4, 0],
                vec![0xb0, base + 0x64, 4, 0],
                vec![0xb0, base + 0x4c, 1, 0]
            ]
        );
        assert_eq!(
            calls(&log, BUFFER_SAVE_FORM_ID),
            vec![vec![0xb0, 0x3030, 0], vec![0xb0, 0x5aed, 0]]
        );
        let buffer = object(&mut e, BUFFER, &[(0, 7)]);
        zero(&mut e, &[HIT_EFFECT_LOAD_GAME_OV2, BUFFER_READ_00864980]);
        e.register(BUFFER_LOAD_FORM_ID, |_, _| ret(0x40));
        e.register(LOOKUP_FORM_004839C0, |_, a| ret(a[0] + 1));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0] + a[3] - 0x0118_0000));
        let (_, log) = run(&mut e, 0x0082_22f0, &args![this, buffer]);
        assert_eq!(calls(&log, BUFFER_READ_00864980).len(), 3);
        assert_eq!(
            e.get(this, MagicShaderHitEffect::pWornObject),
            0x41 + RTTI_01183108 - 0x0118_0000
        );
        assert_eq!(
            e.get(this, MagicShaderHitEffect::pEffectData),
            0x41 + TYPE_TES_EFFECT_SHADER - 0x0118_0000
        );
        e.register(BUFFER, |_, _| ret(8));
        let (_, log) = run(&mut e, 0x0082_22f0, &args![this, buffer]);
        assert_eq!(
            calls(&log, BUFFER_READ_00864980)[2],
            vec![buffer, base + 0x60, 4]
        );
        assert_eq!(calls(&log, BUFFER_READ_00864980).len(), 5);
    }

    #[test]
    fn weighted_random() {
        let mut e = Engine::new();
        rig(&mut e);
        zero(
            &mut e,
            &[
                ARRAY_CONSTRUCTOR,
                WEIGHTS_CONSTRUCT_00438660,
                WEIGHTS_DESTRUCT_00438690,
                STRING_ARRAY_CLEAR_008454F0,
            ],
        );
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x20, 9);
        let (r, log) = run(&mut e, 0x0082_2440, &args![this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(
            calls(&log, WEIGHTS_CONSTRUCT_00438660),
            vec![vec![this + 0x10]]
        );
        assert_eq!(e.mem.u32(this + 0x20), 0);
        // Weights 3, 4, 5 for names 0xa, 0xb, 0xc; a roll of 5 picks 0xb.
        word_array(&mut e, this, 0, &[0xa, 0xb, 0xc]);
        word_array(&mut e, this, 0x10, &[3, 4, 5]);
        e.register(ROLL_005A00A0, |_, _| ret(5));
        assert_eq!(e.call(0x0082_2650, &args![this]).u32(), 0xb);
        e.register(ROLL_005A00A0, |_, _| ret(12));
        assert_eq!(e.call(0x0082_2650, &args![this]).u32(), 0);
        let (_, log) = run(&mut e, 0x0082_24b0, &args![this]);
        assert_eq!(
            calls(&log, WEIGHTS_DESTRUCT_00438690),
            vec![vec![this + 0x10]]
        );
        assert_eq!(
            calls(&log, STRING_ARRAY_CLEAR_008454F0),
            vec![vec![this, 1]]
        );
        assert_eq!(e.mem.u32(this), STRING_ARRAY_VTABLE);
    }

    #[test]
    fn array_scalar_deleting_destructors() {
        let mut e = Engine::new();
        zero(&mut e, &[FREE_00401030, STRING_ARRAY_CLEAR_008454F0]);
        for (address, vtable) in [
            (0x0082_26f0u32, PARTICLE_ARRAY_VTABLE),
            (0x0082_2720, OBJECT_ARRAY_VTABLE),
            (0x0082_2750, STRING_ARRAY_VTABLE),
        ] {
            let this = e.mem.alloc(0x10);
            let (r, log) = run(&mut e, address, &args![this, 1u32]);
            assert_eq!(r.u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls(&log, FREE_00401030), vec![vec![this]]);
            let (_, log) = run(&mut e, address, &args![this, 2u32]);
            assert!(calls(&log, FREE_00401030).is_empty());
        }
    }
}
