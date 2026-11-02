//! `fallout/magic/lighteffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `LightEffect` (Xbox PDB, 0x4C bytes) is an `ActiveEffect` with an
//! `NiPointer<NiPointLight> spPointLight` at +0x48. Of all light effects on a target only
//! the strongest (highest magnitude) keeps a light; the light is added to the shadow scene
//! node and attached to the actor's 3D, and a global counter (`011dc304`) limits how many
//! exist at once.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_magic::activeeffect::ActiveEffect;

/// `LightEffect`'s vtable (`0107a2cc`).
const LIGHT_EFFECT_VTABLE: u32 = 0x0107_a2cc;
/// The `ActiveEffect` constructor `00803d30` (`thiscall`, `RET 0xC`: caster, spell, effect
/// item).
const ACTIVE_EFFECT_CONSTRUCTOR: u32 = 0x0080_3d30;
/// The `ActiveEffect` destructor body `00803ed0` (`thiscall`).
const ACTIVE_EFFECT_DESTRUCTOR: u32 = 0x0080_3ed0;
/// `ActiveEffect::CopyData` (engine map, `00804090`, `thiscall`, `RET 4`).
const ACTIVE_EFFECT_COPY_DATA: u32 = 0x0080_4090;
/// `ActiveEffect::FinishInitLoadGame` (engine map, `00805760`, `thiscall`, `RET 4`).
const ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME: u32 = 0x0080_5760;
/// `ActiveEffect::FinishLoadGame` (engine map, `008067b0`, `thiscall`, `RET 4`).
const ACTIVE_EFFECT_FINISH_LOAD_GAME: u32 = 0x0080_67b0;
/// `004534f0` (`thiscall`, `RET 4`): the base `Revert` body.
const BASE_REVERT: u32 = 0x0045_34f0;
/// `NiPointer` helpers on the light slot (`ECX` = the slot): `00633c90` constructs it from
/// a pointer, `0066b0d0` assigns a pointer, `006e5cc0` assigns another `NiPointer`,
/// `0045cec0` destroys it (all `RET 4` but the destructor); `00559450` reads the pointer.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_ASSIGN_POINTER: u32 = 0x006e_5cc0;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
const POINTER_GET: u32 = 0x0055_9450;
/// `00401000` / `00401030`: the game's `operator new` and `operator delete` (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const DEALLOCATE: u32 = 0x0040_1030;
/// `0059bb30` (`thiscall`): the effect's magic target.
const GET_MAGIC_TARGET: u32 = 0x0059_bb30;
/// `MagicTarget::GetTargetAsActor` (engine map, `00822b40`, `thiscall`).
const GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `__RTDynamicCast` (`cdecl`: object, 0, source type, target type, 0) and the RTTI type
/// descriptors `.?AVActiveEffect@@` and `.?AVLightEffect@@`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_ACTIVE_EFFECT: u32 = 0x011a_0d1c;
const TYPE_LIGHT_EFFECT: u32 = 0x011a_116c;
/// `00450b80` (`cdecl`, one word): the shadow scene node (`ShadowSceneNode *`) for the index.
const SHADOW_SCENE_NODE: u32 = 0x0045_0b80;
/// `ShadowSceneNode::GetLight` (`00b5b4a0`, `RET 4`), `AddLight` (`00b5c940`, `RET 8`) and
/// `RemoveLight` (`00b5eed0`, `RET 4`) (engine map names).
const SHADOW_SCENE_NODE_GET_LIGHT: u32 = 0x00b5_b4a0;
const SHADOW_SCENE_NODE_ADD_LIGHT: u32 = 0x00b5_c940;
const SHADOW_SCENE_NODE_REMOVE_LIGHT: u32 = 0x00b5_eed0;
/// `009611e0` (`thiscall`): the word at +0x18 of the light (its parent node).
const PARENT_NODE: u32 = 0x0096_11e0;
/// `TESObjectREFR::KillLight` (engine map, `00572a50`, `thiscall`, `RET 4`).
const KILL_LIGHT: u32 = 0x0057_2a50;
/// `TESObjectREFR::GetSpellEffectLight` (engine map, `00572750`, `thiscall`) and the setter
/// `00572770` (`thiscall`, `RET 4`).
const GET_SPELL_EFFECT_LIGHT: u32 = 0x0057_2750;
const SET_SPELL_EFFECT_LIGHT: u32 = 0x0057_2770;
/// The number of light effects with a light (`011dc304`; the Xbox PDB has a static
/// `LightEffect::iLightCount`, not confirmed to be this word).
const LIGHT_COUNT: u32 = 0x011d_c304;
/// The setting whose integer value is the most light effects allowed (`011d1d18`, read
/// through `0043d4d0`, which answers the address of its value).
const MAX_LIGHTS_SETTING: u32 = 0x011d_1d18;
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
/// `00403e20` (`thiscall`): the address of a `float` setting's value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The `float` settings `AttachLight` reads (their names are not in the exe's data).
const RADIUS_ADD_SETTING: u32 = 0x011d_1980;
const RADIUS_SCALE_SETTING: u32 = 0x011d_15e0;
const OFFSET_X_SETTING: u32 = 0x011d_1a30;
const OFFSET_Y_SETTING: u32 = 0x011d_1a84;
const OFFSET_Z_SETTING: u32 = 0x011d_15ec;
/// `007df1d0` (`thiscall`): the active effect's effect setting.
const EFFECT_SETTING: u32 = 0x007d_f1d0;
/// `004fd380` (`thiscall`, `ECX` = the effect setting): its light form (the engine map's
/// `TESWorldSpace::GetParentWorldSimple` is a folded body).
const SETTING_LIGHT: u32 = 0x004f_d380;
/// `005b5e40` (`cdecl`, one word): prints an editor warning.
const EDITOR_WARNING: u32 = 0x005b_5e40;
/// `"EDITOR: Light Effect has no Light object associated with it. ..."` (`0107a328`).
const TEXT_NO_LIGHT: u32 = 0x0107_a328;
/// `00aa13e0` (`cdecl`, one word) and the `NiPointLight` constructor `00a7d670`
/// (`thiscall`).
const ALLOCATE_OBJECT: u32 = 0x00aa_13e0;
const POINT_LIGHT_CONSTRUCTOR: u32 = 0x00a7_d670;
/// `005e3fc0` (`thiscall`): the light form's colour (bytes red, green, blue).
const LIGHT_COLOR: u32 = 0x005e_3fc0;
/// `00416870` (`thiscall`, `RET 0xC`): sets a point's three `float`s.
const POINT_SET: u32 = 0x0041_6870;
/// `255.0` (`float`, `01023cd8`).
const COLOR_SCALE: u32 = 0x0102_3cd8;
/// Light setters (`thiscall`, `ECX` = the light): diffuse colour `004bc2e0` (`RET 4`, a
/// point's address), the radius point `0050dd50` (`RET 4`), a `float` `0050dd20` (`RET 4`),
/// the translation `004bc1f0` (`RET 0xC`, three `float`s).
const LIGHT_SET_DIFFUSE: u32 = 0x004b_c2e0;
const LIGHT_SET_RADIUS: u32 = 0x0050_dd50;
const LIGHT_SET_FLOAT: u32 = 0x0050_dd20;
const LIGHT_SET_TRANSLATE: u32 = 0x004b_c1f0;
/// `006d2c40` (`thiscall`, `ECX` = the light form): a `float` in `ST0`.
const LIGHT_FORM_FLOAT: u32 = 0x006d_2c40;
/// The active effect's magnitude (`00495460`: the `float` at +0x1C, in `ST0`).
const MAGNITUDE: u32 = 0x0049_5460;
/// `TESObjectREFR::GetScale` (engine map, `00567400`, `ST0`) and `Actor::GetHeight`
/// (`008853a0`, `ST0`).
const GET_SCALE: u32 = 0x0056_7400;
const GET_HEIGHT: u32 = 0x0088_53a0;
/// The player (`011dea3c`) and the player's node getter `00950bb0` (`thiscall`, `RET 4`).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_NODE: u32 = 0x0095_0bb0;
/// The effect-list walk: `008256d0` (a node is empty: item and next both null),
/// `006815c0` (the node's item slot), `00726070` (the next node).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// `00810fd0` (`thiscall`): the word at +0x98 of the effect setting (its archetype).
const SETTING_ARCHETYPE: u32 = 0x0081_0fd0;
/// `00705cf0` (`thiscall`): a `bool` of the active effect (the walk skips it when true).
const EFFECT_SKIPPED: u32 = 0x0070_5cf0;

fn dynamic_cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

fn target_actor(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(GET_MAGIC_TARGET, &args![this]).u32() == 0 {
        return 0;
    }
    let target = e.call(GET_MAGIC_TARGET, &args![this]).u32();
    e.call(GET_TARGET_AS_ACTOR, &args![target]).u32()
}

fn light(e: &mut Engine, this: Ptr) -> u32 {
    e.call(POINTER_GET, &args![this.addr().wrapping_add(0x48)])
        .u32()
}

/// The address of a `float` setting's value.
fn setting_value(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_VALUE, &args![setting]).u32()
}

// Translated from 0080e610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080e610 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// The `LightEffect` constructor (`thiscall`, `RET 0xC`): the `ActiveEffect` constructor
/// (`00803d30`) with the caster, spell and effect item, its vtable (`0107a2cc`), then the
/// light pointer at +0x48 constructed null (`00633c90`) and assigned null (`0066b0d0`).
/// Returns `this`.
pub fn fn_0080e610(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        ACTIVE_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), LIGHT_EFFECT_VTABLE);
    let slot = this.addr().wrapping_add(0x48);
    e.call(POINTER_CONSTRUCT, &args![slot, 0u32]);
    e.call(POINTER_ASSIGN, &args![slot, 0u32]);
    this
}

// Translated from 0080e6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect`'s scalar deleting destructor (engine map; `thiscall`, `RET 4`): the
/// destructor ([`fn_0080e6d0`]), then `operator delete` (`00401030`) when bit 0
/// of the flags is set. Returns `this`.
pub fn light_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0080e6d0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 0080e6d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080e6d0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// The `LightEffect` destructor (`thiscall`): destroys the light pointer at +0x48
/// (`0045cec0`), then runs the `ActiveEffect` destructor body (`00803ed0`). It does not put
/// a vtable back.
pub fn fn_0080e6d0(e: &mut Engine, this: Ptr) {
    e.call(POINTER_DESTRUCT, &args![this.addr().wrapping_add(0x48)]);
    e.call(ACTIVE_EFFECT_DESTRUCTOR, &args![this]);
}

// Translated from 0080e730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080e730 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `LightEffect::Copy` (Xbox PDB): a new `LightEffect` (0x4C bytes, `00401000`) with this
/// effect's caster (+0x28), spell (+8) and effect item (+0xC), or null when the allocation
/// failed; this effect's slot +0x44 (`CopyData`) copies itself into it. Returns it.
pub fn light_effect_copy(e: &mut Engine, this: Ptr<ActiveEffect>) -> u32 {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    let copy = if block != 0 {
        let caster = e.get(this, ActiveEffect::pCaster);
        let spell = e.get(this, ActiveEffect::pSpell);
        let effect = e.get(this, ActiveEffect::pEffect);
        fn_0080e610(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080e7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect::CopyData` (Xbox PDB; `thiscall`, `RET 4`): `ActiveEffect::CopyData` into
/// the target, then, when the target is a `LightEffect` (`__RTDynamicCast`), its light
/// pointer (+0x48) is assigned this effect's (`006e5cc0`).
pub fn light_effect_copy_data(e: &mut Engine, this: Ptr, target: u32) {
    e.call(ACTIVE_EFFECT_COPY_DATA, &args![this, target]);
    let light_effect = dynamic_cast(e, target, TYPE_ACTIVE_EFFECT, TYPE_LIGHT_EFFECT);
    if light_effect != 0 {
        e.call(
            POINTER_ASSIGN_POINTER,
            &args![
                light_effect.wrapping_add(0x48),
                this.addr().wrapping_add(0x48)
            ],
        );
    }
}

// Translated from 0080e830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect::Start` (Xbox PDB): for a target actor, runs
/// [`light_effect_attach_best_light_effect`] on the actor's magic target (`actor + 0x94`).
pub fn light_effect_start(e: &mut Engine, this: Ptr) {
    let actor = target_actor(e, this);
    if actor != 0 {
        // The exe tests the actor again for the base-class adjustment.
        let target = if actor != 0 {
            actor.wrapping_add(0x94)
        } else {
            0
        };
        light_effect_attach_best_light_effect(e, target);
    }
}

// Translated from 0080e8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect::Update` (Xbox PDB; `thiscall`, `RET 4`, the elapsed time unread): with a
/// light and a shadow scene node (`00450b80(0)`), adds the light to the node
/// (`ShadowSceneNode::AddLight(light, 1)`) when `GetLight` does not find it there.
pub fn light_effect_update(e: &mut Engine, this: Ptr, _unused_1: f32) {
    if light(e, this) == 0 {
        return;
    }
    let node = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
    if node == 0 {
        return;
    }
    let current = light(e, this);
    if e.call(SHADOW_SCENE_NODE_GET_LIGHT, &args![node, current])
        .u32()
        == 0
    {
        let current = light(e, this);
        e.call(SHADOW_SCENE_NODE_ADD_LIGHT, &args![node, current, 1u32]);
    }
}

// Translated from 0080e900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect::Finish` (Xbox PDB): detaches the light ([`light_effect_detach_light`]),
/// then lets the best remaining light effect of the target actor take the light
/// ([`light_effect_attach_best_light_effect`] on `actor + 0x94`).
pub fn light_effect_finish(e: &mut Engine, this: Ptr) {
    light_effect_detach_light(e, this);
    let actor = target_actor(e, this);
    if actor != 0 {
        let target = if actor != 0 {
            actor.wrapping_add(0x94)
        } else {
            0
        };
        light_effect_attach_best_light_effect(e, target);
    }
}

// Translated from 0080e970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080e970 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `LightEffect::AttachLight` (Xbox PDB). Only when the effect has no light yet, its target
/// actor has no spell-effect light (`00572750`) and the light count (`011dc304`) is not
/// above the setting at `011d1d18`:
///
/// - without a light form on the effect setting (`004fd380`), prints the editor warning;
/// - otherwise makes an `NiPointLight` (0xFC bytes, `00aa13e0`, `00a7d670`) into the light
///   pointer; its diffuse colour is the form's colour bytes divided by `255.0`; its radius
///   point is `((magnitude + setting 011d1980) * setting 011d15e0, 0, 0)`; its `0050dd20`
///   value is the form's `006d2c40`; it is added to the shadow scene node (`00450b80(0)`,
///   `AddLight(light, 1)`); its translation is `(scale * setting 011d1a30, (slot +0x1DC's
///   point.y + setting 011d1a84) * scale, height + scale * setting 011d15ec)`;
/// - attaches it under the actor's node (the player's `00950bb0(0)`, others' slot +0x1D0,
///   held in a local `NiPointer`) through the node's slot +0xDC (light, 1), records it on
///   the actor (`00572770`) and counts it.
pub fn light_effect_attach_light(e: &mut Engine, this: Ptr) {
    let actor = target_actor(e, this);
    if light(e, this) != 0 || actor == 0 {
        return;
    }
    if e.call(GET_SPELL_EFFECT_LIGHT, &args![actor]).u32() != 0 {
        return;
    }
    let max_lights = e.call(SETTING_INT_VALUE, &args![MAX_LIGHTS_SETTING]).u32();
    if e.mem.i32(LIGHT_COUNT) > e.mem.i32(max_lights) {
        return;
    }
    let setting = e.call(EFFECT_SETTING, &args![this]).u32();
    let light_form = e.call(SETTING_LIGHT, &args![setting]).u32();
    if light_form == 0 {
        e.call(EDITOR_WARNING, &args![TEXT_NO_LIGHT]);
        return;
    }
    let block = e.call(ALLOCATE_OBJECT, &args![0xfcu32]).u32();
    let new_light = if block != 0 {
        e.call(POINT_LIGHT_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    };
    let slot = this.addr().wrapping_add(0x48);
    e.call(POINTER_ASSIGN, &args![slot, new_light]);
    let color = e.call(LIGHT_COLOR, &args![light_form]).u32();
    let red = x87_store32(f64::from(color & 0xff));
    let green = x87_store32(f64::from((color & 0xffff) >> 8 & 0xff));
    let blue = x87_store32(f64::from(color >> 16 & 0xff));
    e.with_stack(0x0c, |e, diffuse| {
        e.call(POINT_SET, &args![diffuse, red, green, blue]);
        let scale = x87_quiet_bits(e.mem.u32(COLOR_SCALE));
        fn_0080ed20(e, diffuse, f32::from_bits(scale));
        let current = light(e, this);
        e.call(LIGHT_SET_DIFFUSE, &args![current, diffuse]);
    });
    let add = setting_value(e, RADIUS_ADD_SETTING);
    let magnitude = e.call(MAGNITUDE, &args![this]).f64();
    let sum = x87_add(magnitude, x87_m32(e.mem.u32(add)));
    let scale_value = setting_value(e, RADIUS_SCALE_SETTING);
    let radius = x87_store32(x87_mul(x87_load32(e.mem.u32(scale_value)), sum));
    e.with_stack(0x0c, |e, point| {
        e.call(POINT_SET, &args![point, radius, 0.0f32, 0.0f32]);
        let current = light(e, this);
        e.call(LIGHT_SET_RADIUS, &args![current, point]);
    });
    let value = x87_store32(e.call(LIGHT_FORM_FLOAT, &args![light_form]).f64());
    let current = light(e, this);
    e.call(LIGHT_SET_FLOAT, &args![current, value]);
    let node = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
    if node != 0 {
        let current = light(e, this);
        e.call(SHADOW_SCENE_NODE_ADD_LIGHT, &args![node, current, 1u32]);
    }
    let x_value = setting_value(e, OFFSET_X_SETTING);
    let scale = e.call(GET_SCALE, &args![actor]).f64();
    let x = x87_store32(x87_mul(scale, x87_m32(e.mem.u32(x_value))));
    let y = e.with_stack(0x0c, |e, out| {
        let point = e.vcall(actor, 0x1dc, &args![out]).u32();
        let y_value = setting_value(e, OFFSET_Y_SETTING);
        let sum = x87_add(
            x87_load32(e.mem.u32(point.wrapping_add(4))),
            x87_m32(e.mem.u32(y_value)),
        );
        let scale = e.call(GET_SCALE, &args![actor]).f64();
        x87_store32(x87_mul(scale, sum))
    });
    let height = e.call(GET_HEIGHT, &args![actor]).f64();
    let z_value = setting_value(e, OFFSET_Z_SETTING);
    let scale = e.call(GET_SCALE, &args![actor]).f64();
    let z = x87_store32(x87_add(x87_mul(scale, x87_m32(e.mem.u32(z_value))), height));
    let current = light(e, this);
    e.call(LIGHT_SET_TRANSLATE, &args![current, x, y, z]);
    let parent = if actor == e.mem.u32(PLAYER) {
        let player = e.mem.u32(PLAYER);
        e.call(PLAYER_NODE, &args![player, 0u32]).u32()
    } else {
        e.vcall(actor, 0x1d0, &[]).u32()
    };
    e.with_stack(4, |e, holder| {
        e.call(POINTER_CONSTRUCT, &args![holder, parent]);
        let node = e.call(POINTER_GET, &args![holder]).u32();
        let current = light(e, this);
        e.vcall(node, 0xdc, &args![current, 1u32]);
        let current = light(e, this);
        e.call(SET_SPELL_EFFECT_LIGHT, &args![actor, current]);
        let count = e.mem.u32(LIGHT_COUNT).wrapping_add(1);
        e.mem.set_u32(LIGHT_COUNT, count);
        e.call(POINTER_DESTRUCT, &args![holder]);
    });
}

// Translated from 0080ed20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): divides the three `float`s at `this` by
/// the argument in place (x87 `FDIV` each, stored as `float`). Returns `this`.
pub fn fn_0080ed20(e: &mut Engine, this: Ptr, divisor: f32) -> Ptr {
    for offset in [0u32, 4, 8] {
        let address = this.addr().wrapping_add(offset);
        let value = x87_div(x87_load32(e.mem.u32(address)), x87_m32(divisor.to_bits()));
        e.mem.set_u32(address, x87_store32(value));
    }
    this
}

// Translated from 0080ed60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect::DetachLight` (Xbox PDB). With a light: removes it from the shadow scene
/// node (`00450b80(0)`, `RemoveLight`), from its parent node (the parent's slot +0xE8),
/// clears the target actor's light (`TESObjectREFR::KillLight(1)`), assigns the light
/// pointer null and counts the light down (`011dc304`, not below 0).
pub fn light_effect_detach_light(e: &mut Engine, this: Ptr) {
    let actor = target_actor(e, this);
    if light(e, this) == 0 {
        return;
    }
    let node = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
    if node != 0 {
        let current = light(e, this);
        e.call(SHADOW_SCENE_NODE_REMOVE_LIGHT, &args![node, current]);
    }
    let current = light(e, this);
    let parent = e.call(PARENT_NODE, &args![current]).u32();
    if parent != 0 {
        let current = light(e, this);
        e.vcall(parent, 0xe8, &args![current]);
    }
    if actor != 0 {
        e.call(KILL_LIGHT, &args![actor, 1u32]);
    }
    e.call(POINTER_ASSIGN, &args![this.addr().wrapping_add(0x48), 0u32]);
    let count = e.mem.i32(LIGHT_COUNT).wrapping_sub(1);
    e.mem.set_u32(LIGHT_COUNT, count as u32);
    if count < 0 {
        e.mem.set_u32(LIGHT_COUNT, 0);
    }
}

// Translated from 0080ee50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect::AttachBestLightEffect` (Xbox PDB; `cdecl`, one word: the magic target).
/// Walks the target's active effects (its slot +8 answers the list) and, of the light
/// effects among them (effect setting archetype 0xD, `00810fd0`; `00705cf0` false; cast to
/// `LightEffect`), keeps the one with the greatest magnitude (`00495460`, compared as the
/// exe compares: a later one wins only when strictly greater), detaching the light of each
/// other one ([`light_effect_detach_light`]); the winner gets
/// [`light_effect_attach_light`].
pub fn light_effect_attach_best_light_effect(e: &mut Engine, target: u32) {
    if target == 0 {
        return;
    }
    let mut best = 0u32;
    let mut node = e.vcall(target, 8, &[]).u32();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let effect = e.mem.u32(item);
        if effect != 0 {
            let setting = e.call(EFFECT_SETTING, &args![effect]).u32();
            if e.call(SETTING_ARCHETYPE, &args![setting]).u32() == 0xd
                && !e.call(EFFECT_SKIPPED, &args![effect]).bool()
            {
                let candidate = dynamic_cast(e, effect, TYPE_ACTIVE_EFFECT, TYPE_LIGHT_EFFECT);
                let replaces = if best == 0 {
                    true
                } else {
                    let best_magnitude = e.call(MAGNITUDE, &args![best]).f64();
                    let candidate_magnitude = e.call(MAGNITUDE, &args![candidate]).f64();
                    // FCOMP / TEST AH,0x41 / JNZ: replaced only when greater.
                    candidate_magnitude > best_magnitude
                };
                if replaces {
                    if best != 0 {
                        light_effect_detach_light(e, Ptr::new(best));
                    }
                    best = candidate;
                } else {
                    light_effect_detach_light(e, Ptr::new(candidate));
                }
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if best != 0 {
        light_effect_attach_light(e, Ptr::new(best));
    }
}

// Translated from 0080ef50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080ef50 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`cdecl`, three words): a new `LightEffect` (0x4C bytes,
/// `00401000`) with the caster, spell and effect item, or null when the allocation failed.
pub fn fn_0080ef50(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    if block != 0 {
        fn_0080e610(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    }
}

// Translated from 0080efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect::FinishInitLoadGame` (Xbox PDB; `thiscall`, `RET 4`):
/// `ActiveEffect::FinishInitLoadGame` with the argument, then this effect's slot +0x50.
pub fn light_effect_finish_init_load_game(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME, &args![this, buffer]);
    e.vcall(this.addr(), 0x50, &[]);
}

// Translated from 0080f000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect::FinishLoadGame` (Xbox PDB; `thiscall`, `RET 4`): `ActiveEffect::
/// FinishLoadGame` with the argument, then this effect's slot +0x50.
pub fn light_effect_finish_load_game(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(ACTIVE_EFFECT_FINISH_LOAD_GAME, &args![this, buffer]);
    e.vcall(this.addr(), 0x50, &[]);
}

// Translated from 0080f030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LightEffect::Revert` (Xbox PDB; `thiscall`, `RET 4`): the base body `004534f0` with the
/// argument, then [`light_effect_detach_light`].
pub fn light_effect_revert(e: &mut Engine, this: Ptr, actor: u32) {
    e.call(BASE_REVERT, &args![this, actor]);
    light_effect_detach_light(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_e610, fn_0080e610(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0080_e6a0, light_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0080_e6d0, fn_0080e6d0(Ptr)),
        entry!(0x0080_e730, light_effect_copy(Ptr<ActiveEffect>) -> u32),
        entry!(0x0080_e7e0, light_effect_copy_data(Ptr, u32)),
        entry!(0x0080_e830, light_effect_start(Ptr)),
        entry!(0x0080_e8a0, light_effect_update(Ptr, f32)),
        entry!(0x0080_e900, light_effect_finish(Ptr)),
        entry!(0x0080_e970, light_effect_attach_light(Ptr)),
        entry!(0x0080_ed20, fn_0080ed20(Ptr, f32) -> Ptr),
        entry!(0x0080_ed60, light_effect_detach_light(Ptr)),
        entry!(0x0080_ee50, light_effect_attach_best_light_effect(u32)),
        entry!(0x0080_ef50, fn_0080ef50(u32, u32, u32) -> u32),
        entry!(0x0080_efd0, light_effect_finish_init_load_game(Ptr, u32)),
        entry!(0x0080_f000, light_effect_finish_load_game(Ptr, u32)),
        entry!(0x0080_f030, light_effect_revert(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
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

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &addr in addresses {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn constructor_destructor_and_deleting_destructor() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                ACTIVE_EFFECT_CONSTRUCTOR,
                POINTER_CONSTRUCT,
                POINTER_ASSIGN,
                POINTER_DESTRUCT,
                ACTIVE_EFFECT_DESTRUCTOR,
                DEALLOCATE,
            ],
        );
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x4c);
        assert_eq!(
            e.call(0x0080_e610, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), LIGHT_EFFECT_VTABLE);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCTOR),
            vec![vec![this, 1, 2, 3]]
        );
        assert_eq!(calls(&e, POINTER_CONSTRUCT), vec![vec![this + 0x48, 0]]);
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![this + 0x48, 0]]);
        assert_eq!(e.call(0x0080_e6a0, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, DEALLOCATE).is_empty());
        e.call(0x0080_e6a0, &args![this, 1u32]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![this]]);
        assert_eq!(calls(&e, POINTER_DESTRUCT), vec![vec![this + 0x48]; 2]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DESTRUCTOR), vec![vec![this]; 2]);
    }

    #[test]
    fn copy_create_and_copy_data() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                ACTIVE_EFFECT_CONSTRUCTOR,
                POINTER_CONSTRUCT,
                POINTER_ASSIGN,
                ACTIVE_EFFECT_COPY_DATA,
                POINTER_ASSIGN_POINTER,
                0x0310_0000,
            ],
        );
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let mut slots = vec![0u32; 0x48 / 4];
        slots[0x44 / 4] = 0x0310_0000;
        e.put_vtable(0x0300_0000, &slots);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, 0x0300_0000);
        e.mem.set_u32(this + 8, 0x22);
        e.mem.set_u32(this + 0xc, 0x33);
        e.mem.set_u32(this + 0x28, 0x11);
        let copy = e.call(0x0080_e730, &args![this]).u32();
        assert_eq!(e.mem.u32(copy), LIGHT_EFFECT_VTABLE);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCTOR),
            vec![vec![copy, 0x11, 0x22, 0x33]]
        );
        assert_eq!(calls(&e, 0x0310_0000), vec![vec![this, copy]]);
        let made = e.call(0x0080_ef50, &args![4u32, 5u32, 6u32]).u32();
        assert_eq!(e.mem.u32(made), LIGHT_EFFECT_VTABLE);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0080_e730, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0080_ef50, &args![4u32, 5u32, 6u32]).u32(), 0);
        // CopyData: only a LightEffect target gets the light pointer.
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call(0x0080_e7e0, &args![this, 0x7000u32]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_COPY_DATA), vec![vec![this, 0x7000]]);
        assert_eq!(
            calls(&e, POINTER_ASSIGN_POINTER),
            vec![vec![0x7048, this + 0x48]]
        );
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![0x7000, 0, TYPE_ACTIVE_EFFECT, TYPE_LIGHT_EFFECT, 0]]
        );
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call(0x0080_e7e0, &args![this, 0x7000u32]);
        assert_eq!(calls(&e, POINTER_ASSIGN_POINTER).len(), 1);
    }

    #[test]
    fn point_divide_divides_each_float() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x10);
        e.mem.set_f32(p, 255.0);
        e.mem.set_f32(p + 4, 51.0);
        e.mem.set_f32(p + 8, 0.0);
        e.mem.set_f32(p + 12, 9.0);
        assert_eq!(e.call(0x0080_ed20, &args![p, 255.0f32]).u32(), p);
        assert_eq!(e.mem.f32(p), 1.0);
        assert_eq!(e.mem.f32(p + 4), 0.2);
        assert_eq!(e.mem.f32(p + 8), 0.0);
        assert_eq!(e.mem.f32(p + 12), 9.0);
    }

    #[test]
    fn update_adds_the_light_only_when_missing() {
        let mut e = Engine::new();
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SHADOW_SCENE_NODE, |_, _| 0x9000u32.into_ret());
        e.register(SHADOW_SCENE_NODE_GET_LIGHT, |_, _| 0u32.into_ret());
        quiet(&mut e, &[SHADOW_SCENE_NODE_ADD_LIGHT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x4c);
        e.call(0x0080_e8a0, &args![this, 1.0f32]);
        assert!(calls(&e, SHADOW_SCENE_NODE).is_empty());
        e.mem.set_u32(this + 0x48, 0x5555);
        e.call(0x0080_e8a0, &args![this, 1.0f32]);
        assert_eq!(
            calls(&e, SHADOW_SCENE_NODE_GET_LIGHT),
            vec![vec![0x9000, 0x5555]]
        );
        assert_eq!(
            calls(&e, SHADOW_SCENE_NODE_ADD_LIGHT),
            vec![vec![0x9000, 0x5555, 1]]
        );
        e.register(SHADOW_SCENE_NODE_GET_LIGHT, |_, _| 1u32.into_ret());
        e.call(0x0080_e8a0, &args![this, 1.0f32]);
        assert_eq!(calls(&e, SHADOW_SCENE_NODE_ADD_LIGHT).len(), 1);
        e.register(SHADOW_SCENE_NODE, |_, _| 0u32.into_ret());
        e.call(0x0080_e8a0, &args![this, 1.0f32]);
        assert_eq!(calls(&e, SHADOW_SCENE_NODE_GET_LIGHT).len(), 2);
    }

    /// A detach rig: effect `this` with the light 0x5555 whose parent is a node with a
    /// slot +0xE8.
    fn detach_rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011d_c000, 0x1000);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GET_MAGIC_TARGET, |_, _| 0x6000u32.into_ret());
        e.register(GET_TARGET_AS_ACTOR, |_, _| 0x7000u32.into_ret());
        e.register(SHADOW_SCENE_NODE, |_, _| 0x9000u32.into_ret());
        e.register(PARENT_NODE, |e, _| {
            let parent = e.mem.alloc(4);
            e.mem.set_u32(parent, 0x0300_0000);
            parent.into_ret()
        });
        let mut slots = vec![0u32; 0xec / 4];
        slots[0xe8 / 4] = 0x0310_0000;
        e.put_vtable(0x0300_0000, &slots);
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                SHADOW_SCENE_NODE_REMOVE_LIGHT,
                KILL_LIGHT,
                0x0310_0000,
                BASE_REVERT,
            ],
        );
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 0x48, 0x5555);
        (e, this)
    }

    #[test]
    fn detach_light_removes_it_everywhere_and_counts_down() {
        let (mut e, this) = detach_rig();
        e.mem.set_u32(LIGHT_COUNT, 2);
        e.call(0x0080_ed60, &args![this]);
        assert_eq!(
            calls(&e, SHADOW_SCENE_NODE_REMOVE_LIGHT),
            vec![vec![0x9000, 0x5555]]
        );
        assert_eq!(calls(&e, 0x0310_0000)[0][1], 0x5555);
        assert_eq!(calls(&e, KILL_LIGHT), vec![vec![0x7000, 1]]);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(LIGHT_COUNT), 1);
        // Without a light nothing more happens; the count never goes below 0.
        e.call(0x0080_ed60, &args![this]);
        assert_eq!(calls(&e, KILL_LIGHT).len(), 1);
        e.mem.set_u32(this + 0x48, 0x5555);
        e.mem.set_u32(LIGHT_COUNT, 0);
        e.register(SHADOW_SCENE_NODE, |_, _| 0u32.into_ret());
        e.register(PARENT_NODE, |_, _| 0u32.into_ret());
        e.register(GET_MAGIC_TARGET, |_, _| 0u32.into_ret());
        e.call(0x0080_f030, &args![this, 0x7000u32]);
        assert_eq!(calls(&e, BASE_REVERT), vec![vec![this, 0x7000]]);
        assert_eq!(calls(&e, SHADOW_SCENE_NODE_REMOVE_LIGHT).len(), 1);
        assert_eq!(calls(&e, KILL_LIGHT).len(), 1);
        assert_eq!(e.mem.u32(LIGHT_COUNT), 0);
    }

    #[test]
    fn load_game_finishers_run_the_base_then_slot_0x50() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME,
                ACTIVE_EFFECT_FINISH_LOAD_GAME,
                0x0310_0000,
            ],
        );
        let mut slots = vec![0u32; 0x54 / 4];
        slots[0x50 / 4] = 0x0310_0000;
        e.put_vtable(0x0300_0000, &slots);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, 0x0300_0000);
        e.call(0x0080_efd0, &args![this, 0x99u32]);
        e.call(0x0080_f000, &args![this, 0x98u32]);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME),
            vec![vec![this, 0x99]]
        );
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_FINISH_LOAD_GAME),
            vec![vec![this, 0x98]]
        );
        assert_eq!(calls(&e, 0x0310_0000), vec![vec![this], vec![this]]);
    }

    /// Effects for the best-light walk: each is an object whose magnitude is at +0x1C.
    fn walk_rig(magnitudes: &[f32]) -> (Engine, Vec<u32>, u32) {
        let mut e = Engine::new();
        e.map(0x011d_c000, 0x1000);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, 0x0300_0000);
        e.put_vtable(0x0300_0000, &[0, 0, 0x0310_0000]);
        let mut effects = vec![];
        let mut nodes = vec![];
        for &m in magnitudes {
            let effect = e.mem.alloc(0x4c);
            e.mem.set_f32(effect + 0x1c, m);
            effects.push(effect);
            nodes.push(e.mem.alloc(8));
        }
        for (i, &node) in nodes.iter().enumerate() {
            e.mem.set_u32(node, effects[i]);
            e.mem
                .set_u32(node + 4, nodes.get(i + 1).copied().unwrap_or(0));
        }
        let head = nodes[0];
        e.register_double(0x0310_0000, move |_, _| head.into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EFFECT_SETTING, |_, _| 0x4000u32.into_ret());
        e.register(SETTING_ARCHETYPE, |_, _| 0xdu32.into_ret());
        e.register(EFFECT_SKIPPED, |_, _| 0u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(MAGNITUDE, |e, a| st0(f64::from(e.mem.f32(a[0] + 0x1c))));
        // Detach and attach find no light and no target.
        e.register(GET_MAGIC_TARGET, |_, _| 0u32.into_ret());
        e.register(POINTER_GET, |_, _| 0x0bad_u32.into_ret());
        e.register(GET_SPELL_EFFECT_LIGHT, |_, _| 1u32.into_ret());
        quiet(&mut e, &[SHADOW_SCENE_NODE, PARENT_NODE, POINTER_ASSIGN]);
        e.call_log = Some(vec![]);
        (e, effects, list)
    }

    #[test]
    fn attach_best_keeps_the_strictly_greatest_magnitude() {
        let (mut e, effects, list) = walk_rig(&[2.0, 5.0, 5.0, 1.0]);
        e.call(0x0080_ee50, &args![list]);
        // Detach order: the first (beaten by the second), the third (equal), the fourth.
        let detached: Vec<u32> = calls(&e, POINTER_ASSIGN)
            .iter()
            .map(|c| c[0] - 0x48)
            .collect();
        assert_eq!(detached, vec![effects[0], effects[2], effects[3]]);
        // The winner's AttachLight asks its light (then stops: it has one).
        assert_eq!(calls(&e, GET_MAGIC_TARGET).last(), Some(&vec![effects[1]]));
        // A null target does nothing.
        let n = e.call_log.as_ref().unwrap().len();
        e.call(0x0080_ee50, &args![0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), n + 1);
    }

    #[test]
    fn attach_best_skips_effects_that_are_not_light_effects() {
        let (mut e, effects, list) = walk_rig(&[2.0, 5.0]);
        e.register_double(SETTING_ARCHETYPE, |_, _| 0xcu32.into_ret());
        e.call(0x0080_ee50, &args![list]);
        assert!(calls(&e, RT_DYNAMIC_CAST).is_empty());
        assert!(calls(&e, GET_MAGIC_TARGET).is_empty());
        e.register(SETTING_ARCHETYPE, |_, _| 0xdu32.into_ret());
        e.register(EFFECT_SKIPPED, |_, _| 1u32.into_ret());
        e.call(0x0080_ee50, &args![list]);
        assert!(calls(&e, RT_DYNAMIC_CAST).is_empty());
        assert_eq!(calls(&e, EFFECT_SETTING).len(), 4);
        let _ = effects;
    }

    #[test]
    fn start_and_finish_hand_the_actors_magic_target_to_the_walk() {
        let (mut e, this) = detach_rig();
        // The walk sees an empty list.
        let actor = e.mem.alloc(0x100);
        e.register_double(GET_TARGET_AS_ACTOR, move |_, _| actor.into_ret());
        let target_vtable = 0x0300_1000;
        e.mem.set_u32(actor + 0x94, target_vtable);
        e.put_vtable(target_vtable, &[0, 0, 0x0310_1000]);
        e.register(0x0310_1000, |_, _| 0u32.into_ret());
        e.call(0x0080_e830, &args![this]);
        assert_eq!(calls(&e, 0x0310_1000), vec![vec![actor + 0x94]]);
        e.call(0x0080_e900, &args![this]);
        assert_eq!(calls(&e, 0x0310_1000).len(), 2);
        assert_eq!(calls(&e, KILL_LIGHT), vec![vec![actor, 1]]);
        e.register(GET_MAGIC_TARGET, |_, _| 0u32.into_ret());
        e.call(0x0080_e830, &args![this]);
        assert_eq!(calls(&e, 0x0310_1000).len(), 2);
    }

    /// An attach rig: the effect 0x... with no light, its actor, and doubles for the light's
    /// making.
    fn attach_rig(is_player: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011d_c000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.mem.set_f32(COLOR_SCALE, 255.0);
        e.mem.set_u32(LIGHT_COUNT, 3);
        let this = e.mem.alloc(0x4c);
        let actor = e.mem.alloc(0x100);
        e.mem.set_u32(actor, 0x0300_0000);
        e.mem.set_u32(PLAYER, if is_player { actor } else { 1 });
        let mut slots = vec![0u32; 0x1e0 / 4];
        slots[0x1d0 / 4] = 0x0310_0000;
        slots[0x1dc / 4] = 0x0310_0010;
        e.put_vtable(0x0300_0000, &slots);
        let node = e.mem.alloc(4);
        e.mem.set_u32(node, 0x0300_1000);
        let mut node_slots = vec![0u32; 0xe0 / 4];
        node_slots[0xdc / 4] = 0x0310_0020;
        e.put_vtable(0x0300_1000, &node_slots);
        e.register_double(0x0310_0000, move |_, _| node.into_ret());
        e.register_double(PLAYER_NODE, move |_, _| node.into_ret());
        e.register(0x0310_0010, |e, a| {
            e.mem.set_f32(a[1] + 4, 3.0);
            a[1].into_ret()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(GET_MAGIC_TARGET, |_, _| 0x6000u32.into_ret());
        e.register_double(GET_TARGET_AS_ACTOR, move |_, _| actor.into_ret());
        e.register(GET_SPELL_EFFECT_LIGHT, |_, _| 0u32.into_ret());
        e.register(SETTING_INT_VALUE, |e, _| {
            e.mem.set_u32(0x0120_0000, 4);
            0x0120_0000u32.into_ret()
        });
        e.register(EFFECT_SETTING, |_, _| 0x4000u32.into_ret());
        e.register(SETTING_LIGHT, |_, _| 0x4400u32.into_ret());
        e.register(ALLOCATE_OBJECT, |_, _| 0x0aa0u32.into_ret());
        e.register(POINT_LIGHT_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
        e.register(LIGHT_COLOR, |_, _| 0x00ff_3300u32.into_ret());
        e.register(POINT_SET, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        e.register(LIGHT_SET_DIFFUSE, |e, a| {
            assert_eq!(e.mem.f32(a[1]), 0.0);
            assert_eq!(e.mem.f32(a[1] + 4), 0.2);
            assert_eq!(e.mem.f32(a[1] + 8), 1.0);
            Ret::default()
        });
        e.register(LIGHT_SET_RADIUS, |e, a| {
            assert_eq!(e.mem.f32(a[1]), 12.0);
            assert_eq!(e.mem.f32(a[1] + 4), 0.0);
            Ret::default()
        });
        // Settings: radius add 1.0, radius scale 2.0, x 4.0, y 5.0, z 6.0.
        e.register(SETTING_VALUE, |e, a| {
            let value = match a[0] {
                RADIUS_ADD_SETTING => 1.0f32,
                RADIUS_SCALE_SETTING => 2.0,
                OFFSET_X_SETTING => 4.0,
                OFFSET_Y_SETTING => 5.0,
                _ => 6.0,
            };
            let p = e.mem.alloc(4);
            e.mem.set_f32(p, value);
            p.into_ret()
        });
        e.register(MAGNITUDE, |_, _| st0(5.0));
        e.register(LIGHT_FORM_FLOAT, |_, _| st0(0.75));
        e.register(GET_SCALE, |_, _| st0(2.0));
        e.register(GET_HEIGHT, |_, _| st0(100.0));
        e.register(SHADOW_SCENE_NODE, |_, _| 0x9000u32.into_ret());
        quiet(
            &mut e,
            &[
                LIGHT_SET_FLOAT,
                SHADOW_SCENE_NODE_ADD_LIGHT,
                LIGHT_SET_TRANSLATE,
                0x0310_0020,
                SET_SPELL_EFFECT_LIGHT,
                POINTER_DESTRUCT,
                EDITOR_WARNING,
            ],
        );
        e.call_log = Some(vec![]);
        (e, this, actor)
    }

    #[test]
    fn attach_light_builds_and_places_the_light() {
        let (mut e, this, actor) = attach_rig(false);
        e.call(0x0080_e970, &args![this]);
        assert_eq!(e.mem.u32(this + 0x48), 0x0aa1);
        assert_eq!(
            calls(&e, LIGHT_SET_FLOAT),
            vec![vec![0x0aa1, 0.75f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, SHADOW_SCENE_NODE_ADD_LIGHT),
            vec![vec![0x9000, 0x0aa1, 1]]
        );
        assert_eq!(
            calls(&e, LIGHT_SET_TRANSLATE),
            vec![vec![
                0x0aa1,
                8.0f32.to_bits(),
                16.0f32.to_bits(),
                112.0f32.to_bits()
            ]]
        );
        assert_eq!(calls(&e, 0x0310_0020)[0][1..], [0x0aa1, 1]);
        assert_eq!(calls(&e, SET_SPELL_EFFECT_LIGHT), vec![vec![actor, 0x0aa1]]);
        assert_eq!(e.mem.u32(LIGHT_COUNT), 4);
        assert!(calls(&e, PLAYER_NODE).is_empty());
        assert_eq!(calls(&e, POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn attach_light_uses_the_player_node_and_stops_on_its_guards() {
        let (mut e, this, actor) = attach_rig(true);
        e.call(0x0080_e970, &args![this]);
        let player = calls(&e, PLAYER_NODE);
        assert_eq!(player, vec![vec![actor, 0]]);
        assert!(calls(&e, 0x0310_0000).is_empty());
        // Already lit: nothing.
        let (mut e, this, _) = attach_rig(false);
        e.mem.set_u32(this + 0x48, 0x77);
        e.call(0x0080_e970, &args![this]);
        assert!(calls(&e, GET_SPELL_EFFECT_LIGHT).is_empty());
        // Too many lights.
        let (mut e, this, _) = attach_rig(false);
        e.mem.set_u32(LIGHT_COUNT, 5);
        e.call(0x0080_e970, &args![this]);
        assert!(calls(&e, EFFECT_SETTING).is_empty());
        // No light form: the editor warning.
        let (mut e, this, _) = attach_rig(false);
        e.register(SETTING_LIGHT, |_, _| 0u32.into_ret());
        e.call(0x0080_e970, &args![this]);
        assert_eq!(calls(&e, EDITOR_WARNING), vec![vec![TEXT_NO_LIGHT]]);
        assert!(calls(&e, ALLOCATE_OBJECT).is_empty());
        // The actor already has a spell-effect light.
        let (mut e, this, _) = attach_rig(false);
        e.register(GET_SPELL_EFFECT_LIGHT, |_, _| 1u32.into_ret());
        e.call(0x0080_e970, &args![this]);
        assert!(calls(&e, SETTING_INT_VALUE).is_empty());
    }
}
