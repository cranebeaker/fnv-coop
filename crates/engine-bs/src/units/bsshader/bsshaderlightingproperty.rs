//! `BSShader/bsshaderlightingproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The property keeps its lights in an `NiTPointerList` of `ShadowSceneLight *`
//! (`lLightList`, Xbox PDB, at `+0x60` on PC): head, tail, count; each node
//! is next (`+0`), previous (`+4`), the light (`+8`). A light counts as
//! active when its half-word at `+0x110` is not 0xFF and bit 0 of its
//! `NiLight`'s (`spLight`, `+0xF8`) byte `+0x30` is clear; a shadow light has
//! `bCastShadow` (`+0xEC`) set.

#[allow(unused_imports)]
use crate::prelude::*;

/// `ShadowSceneLight::GetLuminanceToBound` (`00b9dbe0`, `thiscall` on the
/// light, one word: the bound; result in ST0).
const LIGHT_LUMINANCE_TO_BOUND: u32 = 0x00b9_dbe0;
/// `ShadowSceneLight::RemoveTriGeom` (`00ba01a0`, `thiscall`, two words).
const LIGHT_REMOVE_TRI_GEOM: u32 = 0x00ba_01a0;
/// `00b702e0` (unit `run_00b70260`, `thiscall` on the list, two words: the
/// node, the node it goes before): moves a node.
const LIST_MOVE_BEFORE: u32 = 0x00b7_02e0;
/// `00b58570` (unnamed, `thiscall` on the list, one word: a pointer to the
/// light): adds the light.
const LIST_ADD: u32 = 0x00b5_8570;
/// `00e7db70` (unnamed, `thiscall` on the list, one word: a pointer to the
/// light): removes the light.
const LIST_REMOVE: u32 = 0x00e7_db70;
/// `00e7d5b0` (unnamed, `thiscall` on the list, no words): removes the tail.
const LIST_REMOVE_TAIL: u32 = 0x00e7_d5b0;
/// `00e7d7f0` (unnamed, `thiscall` on the list, one word: a pointer to the
/// node pointer): removes that node.
const LIST_REMOVE_NODE: u32 = 0x00e7_d7f0;
/// `NiTPointerListBase<..>::AddHead` (engine map; `0043a010`, `thiscall` on
/// the list's count word, no words): hands out a new node.
const LIST_NEW_NODE: u32 = 0x0043_a010;
/// `NiTPointerListBase<..>::RemoveAll` (`00b71300`, `thiscall` on the list).
const LIST_REMOVE_ALL: u32 = 0x00b7_1300;
/// `BSShaderProperty::BSShaderProperty` (`00ba8e50`, `thiscall`, no words).
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
/// The spin lock of the list nodes' pool (`011c5f80`: owner, then the count
/// at `011c5f84`), its enter function (`0040fbf0`, `thiscall`, one word),
/// and the pool's free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const SPIN_LOCK_ENTER: u32 = 0x0040_fbf0;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
/// Import slot of `DebugBreak`.
const DEBUG_BREAK: u32 = 0x00fd_f0c8;
/// The marker light the fence node holds (`011fa310`).
const FENCE_MARKER: u32 = 0x011f_a310;
/// The property vtable the constructor stores.
const LIGHTING_PROPERTY_VTABLE: u32 = 0x010a_e7b0;

layout! {
    /// `BSShaderLightingProperty` (Xbox PDB), the fields used here at their
    /// PC offsets (8 below the Xbox ones from the light list on).
    pub struct BSShaderLightingProperty: 0x7C {
        /// `iShaderPropertyType` (Xbox PDB).
        0x1C iShaderPropertyType: u32,
        /// `iLastRenderPassState` (Xbox PDB).
        0x38 iLastRenderPassState: u32,
        /// `lLightList` (Xbox PDB): head, tail and count.
        0x60 lLightList: u32,
        0x64 lLightList_tail: u32,
        0x68 lLightList_count: u32,
        /// `fForcedDarkness` (Xbox PDB), as bits.
        0x6C fForcedDarkness: u32,
        /// `uiReferenceID` (Xbox PDB).
        0x70 uiReferenceID: u32,
        /// `bLightListChanged` (Xbox PDB).
        0x74 bLightListChanged: u8,
        /// PC only: the fence node of the light list (`FenceAllLights`).
        0x78 fence: u32,
    }
}

type Lit = BSShaderLightingProperty;

/// Whether `light` counts as active (see the unit comment).
fn is_active(e: &Engine, light: u32) -> bool {
    e.mem.u16(light + 0x110) != 0xff && e.mem.u8(e.mem.u32(light + 0xf8) + 0x30) & 1 == 0
}

/// Active and not casting shadows.
fn is_active_non_shadow(e: &Engine, light: u32) -> bool {
    is_active(e, light) && e.mem.u8(light + 0xec) != 1
}

// Translated from 00b70340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words: the light, an unused
/// word, the result slot; `bool` in `AL`): clears the slot; with lights
/// (count `+0x68` at least 1) walks the list from the head (which it does
/// not check for null) and answers true with the node holding `light` in
/// the slot, false at the first null light or the end.
pub fn fn_00b70340(e: &mut Engine, this: Ptr<Lit>, light: u32, _unused_2: u32, node: Ptr) -> bool {
    e.mem.set_u32(node.addr(), 0);
    if e.get(this, Lit::lLightList_count) < 1 {
        return false;
    }
    let mut current = e.get(this, Lit::lLightList);
    let mut element = e.mem.u32(current + 8);
    let mut next = e.mem.u32(current);
    while element != 0 {
        if element == light {
            e.mem.set_u32(node.addr(), current);
            return true;
        }
        if next == 0 {
            return false;
        }
        element = e.mem.u32(next + 8);
        current = next;
        next = e.mem.u32(next);
    }
    false
}

/// The list move `00b70390` inlines: `node` goes just before `before`.
fn move_before(e: &mut Engine, this: Ptr<Lit>, node: u32, before: u32) {
    let list = this.addr() + 0x60;
    if e.mem.u32(list) == node {
        let next = e.mem.u32(node);
        e.mem.set_u32(list, next);
    }
    if e.mem.u32(list) == before {
        e.mem.set_u32(list, node);
    }
    if e.mem.u32(list + 4) == node {
        let previous = e.mem.u32(node + 4);
        e.mem.set_u32(list + 4, previous);
    }
    let next = e.mem.u32(node);
    if next != 0 {
        let previous = e.mem.u32(node + 4);
        e.mem.set_u32(next + 4, previous);
    }
    let previous = e.mem.u32(node + 4);
    if previous != 0 {
        let next = e.mem.u32(node);
        e.mem.set_u32(previous, next);
    }
    let previous = e.mem.u32(before + 4);
    e.mem.set_u32(node + 4, previous);
    e.mem.set_u32(node, before);
    if previous != 0 {
        e.mem.set_u32(previous, node);
    }
    e.mem.set_u32(before + 4, node);
}

// Translated from 00b70390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::ResortLights` (Xbox PDB; one word: the bound):
/// for each node (its next taken first), its light's luminance to the bound
/// (stored as a float) is compared with each node from the head up to it;
/// the first one with a greater luminance (an ordered `>` on the x87 result
/// against the float widened to a double) gets the node moved before it and
/// ends that scan. When anything moved, `iLastRenderPassState` is cleared.
pub fn bs_shader_lighting_property_resort_lights(e: &mut Engine, this: Ptr<Lit>, bound: u32) {
    let mut changed = false;
    let mut node = e.get(this, Lit::lLightList);
    if node == 0 {
        return;
    }
    loop {
        let mut other = e.get(this, Lit::lLightList);
        let next = e.mem.u32(node);
        let light = e.mem.u32(node + 8);
        let luminance = x87_store32(e.call(LIGHT_LUMINANCE_TO_BOUND, &args![light, bound]).f64());
        while other != node && other != 0 {
            let reference = x87_load32(luminance);
            let candidate = other;
            other = e.mem.u32(other);
            let light = e.mem.u32(candidate + 8);
            let value = e.call(LIGHT_LUMINANCE_TO_BOUND, &args![light, bound]).f64();
            if value > reference {
                if node != candidate {
                    move_before(e, this, node, candidate);
                }
                other = 0;
                changed = true;
            }
        }
        if next == 0 {
            break;
        }
        node = next;
    }
    if changed {
        e.set(this, Lit::iLastRenderPassState, 0);
    }
}

// Translated from 00b70490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetFirstLight` (Xbox PDB; one word: the
/// iterator): the iterator gets the head; with one, it moves to the next
/// node and the head's light is answered; else 0 (the null head).
pub fn bs_shader_lighting_property_get_first_light(
    e: &mut Engine,
    this: Ptr<Lit>,
    iterator: Ptr,
) -> u32 {
    let head = e.get(this, Lit::lLightList);
    e.mem.set_u32(iterator.addr(), head);
    if head == 0 {
        return head;
    }
    let next = e.mem.u32(head);
    e.mem.set_u32(iterator.addr(), next);
    e.mem.u32(head + 8)
}

/// The shared body of `00b70590` and `00b70600`: the first light passing
/// `test`, the iterator left at the node after it.
fn first_matching(
    e: &mut Engine,
    this: Ptr<Lit>,
    iterator: Ptr,
    test: fn(&Engine, u32) -> bool,
) -> u32 {
    let head = e.get(this, Lit::lLightList);
    e.mem.set_u32(iterator.addr(), head);
    if head == 0 {
        return 0;
    }
    let mut next = e.mem.u32(head);
    e.mem.set_u32(iterator.addr(), next);
    let mut light = e.mem.u32(head + 8);
    while next != 0 {
        if light == 0 {
            return 0;
        }
        if test(e, light) {
            break;
        }
        let node = e.mem.u32(iterator.addr());
        next = e.mem.u32(node);
        e.mem.set_u32(iterator.addr(), next);
        light = e.mem.u32(node + 8);
    }
    if light == 0 || !test(e, light) {
        return 0;
    }
    light
}

// Translated from 00b70590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetFirstActiveLight` (Xbox PDB; one word: the
/// iterator): the first active light from the head (0 at a null light or
/// when the last one is not active), the iterator at the node after it.
pub fn bs_shader_lighting_property_get_first_active_light(
    e: &mut Engine,
    this: Ptr<Lit>,
    iterator: Ptr,
) -> u32 {
    first_matching(e, this, iterator, is_active)
}

// Translated from 00b70600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetFirstActiveNonShadowLight` (Xbox PDB): as
/// `GetFirstActiveLight` for active lights whose `bCastShadow` is not 1.
pub fn bs_shader_lighting_property_get_first_active_non_shadow_light(
    e: &mut Engine,
    this: Ptr<Lit>,
    iterator: Ptr,
) -> u32 {
    first_matching(e, this, iterator, is_active_non_shadow)
}

/// The shared body of `00b70680` and `00b70700`: from the iterator's node,
/// the first light passing `test`, the iterator moved past it.
fn next_matching(e: &mut Engine, iterator: Ptr, test: fn(&Engine, u32) -> bool) -> u32 {
    let node = e.mem.u32(iterator.addr());
    if node == 0 {
        return 0;
    }
    let mut next = e.mem.u32(node);
    e.mem.set_u32(iterator.addr(), next);
    let mut light = e.mem.u32(node + 8);
    if light == 0 {
        return 0;
    }
    while next != 0 {
        if test(e, light) {
            break;
        }
        let node = next;
        next = e.mem.u32(node);
        e.mem.set_u32(iterator.addr(), next);
        light = e.mem.u32(node + 8);
        if light == 0 {
            return 0;
        }
    }
    if light != 0 && test(e, light) {
        light
    } else {
        0
    }
}

// Translated from 00b70680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetNextActiveLight` (Xbox PDB; `thiscall`,
/// `this` unused, one word: the iterator): the next active light.
pub fn bs_shader_lighting_property_get_next_active_light(
    e: &mut Engine,
    _this: Ptr,
    iterator: Ptr,
) -> u32 {
    next_matching(e, iterator, is_active)
}

// Translated from 00b70700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetNextActiveNonShadowLight` (Xbox PDB;
/// `this` unused): the next active light that does not cast shadows.
pub fn bs_shader_lighting_property_get_next_active_non_shadow_light(
    e: &mut Engine,
    _this: Ptr,
    iterator: Ptr,
) -> u32 {
    next_matching(e, iterator, is_active_non_shadow)
}

// Translated from 00b70790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetNumberofActiveLights` (Xbox PDB): the
/// number of non-null active lights.
pub fn bs_shader_lighting_property_get_numberof_active_lights(
    e: &mut Engine,
    this: Ptr<Lit>,
) -> u32 {
    let mut node = e.get(this, Lit::lLightList);
    let mut count = 0u32;
    while node != 0 {
        let light = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if light != 0 && is_active(e, light) {
            count += 1;
        }
    }
    count
}

// Translated from 00b707d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetNumberofActiveNonShadowLights` (Xbox PDB):
/// the number of non-null active lights whose `bCastShadow` is 0.
pub fn bs_shader_lighting_property_get_numberof_active_non_shadow_lights(
    e: &mut Engine,
    this: Ptr<Lit>,
) -> u32 {
    let mut node = e.get(this, Lit::lLightList);
    let mut count = 0u32;
    while node != 0 {
        let light = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if light != 0 && is_active(e, light) && e.mem.u8(light + 0xec) == 0 {
            count += 1;
        }
    }
    count
}

// Translated from 00b713c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the light list, two words: the
/// node to insert before, a pointer to the light): a new node (`0043a010` on
/// the count word) holding the light goes before `before` (as the head when
/// it has no previous node); the count rises.
pub fn fn_00b713c0(e: &mut Engine, this: Ptr, before: u32, light: Ptr) {
    let list = this.addr();
    let count = list + 8;
    let node = e.call(LIST_NEW_NODE, &args![count]).u32();
    let value = e.mem.u32(light.addr());
    e.mem.set_u32(node + 8, value);
    e.mem.set_u32(node, before);
    let previous = e.mem.u32(before + 4);
    e.mem.set_u32(node + 4, previous);
    let previous = e.mem.u32(before + 4);
    if previous != 0 {
        e.mem.set_u32(previous, node);
    } else {
        e.mem.set_u32(list, node);
    }
    e.mem.set_u32(before + 4, node);
    let n = e.mem.u32(count);
    e.mem.set_u32(count, n.wrapping_add(1));
}

// Translated from 00b71560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the light, a word passed
/// on to `00b70340`): adds a light. With a fence node: a light already in the
/// list is moved before the fence (`00b702e0`; its node, or the tail when the
/// lookup left none), a new one is inserted before it (`00b713c0`) and the
/// list marked changed. Without one: a new light is added (`00b58570`), the
/// list is marked changed, and a shadow light clears `iLastRenderPassState`.
/// The lookup's node slot is the second argument's slot.
pub fn fn_00b71560(e: &mut Engine, this: Ptr<Lit>, light: u32, second: u32) {
    let fence = e.get(this, Lit::fence);
    let list = this.addr() + 0x60;
    e.with_stack(8, |e, slots| {
        let light_slot = slots;
        let node_slot: Ptr = Ptr::new(slots.addr() + 4);
        e.mem.set_u32(light_slot.addr(), light);
        e.mem.set_u32(node_slot.addr(), second);
        let found = fn_00b70340(e, this, light, second, node_slot);
        if fence != 0 {
            if found {
                let mut node = e.mem.u32(node_slot.addr());
                if node == 0 {
                    node = e.get(this, Lit::lLightList_tail);
                }
                e.call(LIST_MOVE_BEFORE, &args![list, node, fence]);
                return;
            }
            fn_00b713c0(e, Ptr::new(list), fence, light_slot);
            e.set(this, Lit::bLightListChanged, 1);
            return;
        }
        if !found {
            e.call(LIST_ADD, &args![list, light_slot]);
        }
        e.set(this, Lit::bLightListChanged, 1);
        if e.mem.u8(light + 0xec) != 0 {
            e.set(this, Lit::iLastRenderPassState, 0);
        }
    });
}

/// Returns a removed node to the node pool (`0040fbf0` enters the pool's
/// spin lock; the node joins the free list; the lock count drops and the
/// owner is cleared at zero), as `00b71600` does twice inline; the
/// element was cleared before the lock.
fn free_node(e: &mut Engine, node: u32) {
    e.call(SPIN_LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    e.mem.set_u32(node + 4, 0);
    let free = e.global::<u32>(NODE_POOL_FREE_LIST);
    e.mem.set_u32(node, free);
    let count = e.global::<u32>(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.set_global(NODE_POOL_LOCK_COUNT, count);
    e.set_global(NODE_POOL_FREE_LIST, node);
    if count == 0 {
        e.set_global(NODE_POOL_LOCK, 0u32);
    }
}

// Translated from 00b71600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::RemoveFencedLights` (Xbox PDB; one word: the
/// geometry passed to `RemoveTriGeom`): when the fence is not the tail, the
/// list is marked changed and every node after the fence is removed (the
/// head and middle ones unlinked and returned to the node pool inline, the
/// tail through `00e7d5b0`), each light losing the geometry
/// (`ShadowSceneLight::RemoveTriGeom(geometry, 0)`) and a shadow light
/// clearing `iLastRenderPassState`. Then the fence node itself is removed
/// (`00e7d7f0`) and the fence cleared.
pub fn bs_shader_lighting_property_remove_fenced_lights(
    e: &mut Engine,
    this: Ptr<Lit>,
    geometry: u32,
) {
    let fence = e.get(this, Lit::fence);
    let list = this.addr() + 0x60;
    if fence != e.get(this, Lit::lLightList_tail) {
        e.set(this, Lit::bLightListChanged, 1);
        let mut next = e.mem.u32(fence);
        while next != 0 {
            let head = e.mem.u32(list);
            let node = next;
            let light = e.mem.u32(node + 8);
            next = e.mem.u32(node);
            if node == head {
                let after = e.mem.u32(head);
                e.mem.set_u32(list, after);
                if after != 0 {
                    e.mem.set_u32(after + 4, 0);
                } else {
                    e.mem.set_u32(list + 4, 0);
                }
                e.mem.set_u32(head + 8, 0);
                free_node(e, head);
                let count = e.mem.u32(list + 8);
                e.mem.set_u32(list + 8, count.wrapping_sub(1));
            } else if node == e.mem.u32(list + 4) {
                e.call(LIST_REMOVE_TAIL, &args![list]);
            } else {
                let previous = e.mem.u32(node + 4);
                let after = e.mem.u32(node);
                if previous != 0 {
                    e.mem.set_u32(previous, after);
                }
                if after != 0 {
                    e.mem.set_u32(after + 4, previous);
                }
                e.mem.set_u32(node + 8, 0);
                free_node(e, node);
                let count = e.mem.u32(list + 8);
                e.mem.set_u32(list + 8, count.wrapping_sub(1));
            }
            e.call(LIGHT_REMOVE_TRI_GEOM, &args![light, geometry, 0u32]);
            if e.mem.u8(light + 0xec) != 0 {
                e.set(this, Lit::iLastRenderPassState, 0);
            }
        }
    }
    let fence_field = this.addr() + 0x78;
    e.call(LIST_REMOVE_NODE, &args![list, fence_field]);
    e.set(this, Lit::fence, 0);
}

// Translated from 00b71720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, no words): the
/// `BSShaderLightingProperty` constructor: the shader property part, the
/// vtable, an empty light list (then `RemoveAll`), reference ID 0, forced
/// darkness 1.0 (`FLD1`), not changed, no fence, shader property type 1.
pub fn fn_00b71720(e: &mut Engine, this: Ptr<Lit>) -> Ptr<Lit> {
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), LIGHTING_PROPERTY_VTABLE);
    e.set(this, Lit::lLightList_count, 0);
    e.set(this, Lit::lLightList, 0);
    e.set(this, Lit::lLightList_tail, 0);
    e.call(LIST_REMOVE_ALL, &args![this.addr() + 0x60]);
    e.set(this, Lit::uiReferenceID, 0);
    e.set(this, Lit::fForcedDarkness, 1.0f32.to_bits());
    e.set(this, Lit::bLightListChanged, 0);
    e.set(this, Lit::fence, 0);
    e.set(this, Lit::iShaderPropertyType, 1);
    this
}

// Translated from 00b717a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::RemoveLight` (Xbox PDB; one word: the light):
/// only without a fence: the light is removed (`00e7db70`, through the
/// argument slot), the list marked changed, a shadow light clears
/// `iLastRenderPassState`; then (still without a fence) every node still
/// holding the light calls `DebugBreak`.
pub fn bs_shader_lighting_property_remove_light(e: &mut Engine, this: Ptr<Lit>, light: u32) {
    if e.get(this, Lit::fence) != 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), light);
        e.call(LIST_REMOVE, &args![this.addr() + 0x60, slot]);
    });
    e.set(this, Lit::bLightListChanged, 1);
    if e.mem.u8(light + 0xec) != 0 {
        e.set(this, Lit::iLastRenderPassState, 0);
    }
    if e.get(this, Lit::fence) != 0 {
        return;
    }
    let mut node = e.get(this, Lit::lLightList);
    while node != 0 {
        let element = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if element == light {
            e.call(DEBUG_BREAK, &[]);
        }
    }
}

// Translated from 00b718b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::FenceAllLights` (Xbox PDB): removes the marker
/// light `011fa310` (`00e7db70`, through a local), then puts a new node
/// (`0043a010`) holding the marker at the head (also the tail of an empty
/// list), counted, and makes it the fence.
pub fn bs_shader_lighting_property_fence_all_lights(e: &mut Engine, this: Ptr<Lit>) {
    let list = this.addr() + 0x60;
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), FENCE_MARKER);
        e.call(LIST_REMOVE, &args![list, local]);
    });
    let count = list + 8;
    let node = e.call(LIST_NEW_NODE, &args![count]).u32();
    e.mem.set_u32(node + 8, FENCE_MARKER);
    e.mem.set_u32(node + 4, 0);
    let head = e.mem.u32(list);
    e.mem.set_u32(node, head);
    let head = e.mem.u32(list);
    if head != 0 {
        e.mem.set_u32(head + 4, node);
    } else {
        e.mem.set_u32(list + 4, node);
    }
    let n = e.mem.u32(count);
    e.mem.set_u32(count, n.wrapping_add(1));
    e.mem.set_u32(list, node);
    e.set(this, Lit::fence, node);
}

use crate::units::bsshader::bsshaderproperty::{
    viewer_strings_add, VIEWER_STRING, VIEWER_STRING_UINT,
};

/// The once-only guard of `00b704c0`'s four `float`s (bit 0 of `011fa578`), the floats
/// (`011fa568` to `011fa574`) and the `float` the first starts from (`01017d00`).
const GUARD_011FA578: u32 = 0x011f_a578;
const DEFAULTS_011FA568: u32 = 0x011f_a568;
const SOURCE_01017D00: u32 = 0x0101_7d00;
/// The word table `00b704c0` clears from an index up (eight words at `011fa070`), the
/// 16-byte records it fills (`011fa0d0` up to `011fa150`, with a second set of four words
/// `0x120` after each) and the four words the records take (`011a9bd0` to `011a9bdc`).
const WORDS_011FA070: u32 = 0x011f_a070;
const RECORDS_011FA0D0: u32 = 0x011f_a0d0;
const RECORDS_END_011FA150: u32 = 0x011f_a150;
const VALUES_011A9BD0: u32 = 0x011a_9bd0;
/// `BSShaderProperty::GetViewerStrings` (`00ba9890`, `thiscall`, one word), the class-name
/// record (`011fa560`), the 16-bit viewer-string maker (`00a7d490`) and the labels
/// (`"number of lights"`, `"active lights"`, `"Reference ID"`).
const SHADER_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00ba_9890;
const CLASS_RTTI_011FA560: u32 = 0x011f_a560;
const VIEWER_STRING_SHORT: u32 = 0x00a7_d490;
const LABEL_LIGHT_COUNT: u32 = 0x010a_e89c;
const LABEL_ACTIVE_LIGHTS: u32 = 0x010a_e88c;
const LABEL_REFERENCE_ID: u32 = 0x010a_e87c;
/// The base destructor (`00baa0e0`, `thiscall`), the sized delete (`00aa1460`, `cdecl`:
/// block, size), the allocation (`00aa13e0`, `cdecl`, size) and the copy (`00ba8b10`,
/// `thiscall`: destination, clone process).
const BASE_DESTRUCT_00BAA0E0: u32 = 0x00ba_a0e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const BASE_COPY_MEMBERS_00BA8B10: u32 = 0x00ba_8b10;

// Translated from 00b704b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` without words: answers the low 16 bits of the light
/// list's count (`+0x68`).
pub fn fn_00b704b0(e: &mut Engine, this: Ptr<Lit>) -> u16 {
    e.get(this, Lit::lLightList_count) as u16
}

// Translated from 00b704c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `cdecl` with one word (an index). On the first call the four
/// `float`s at `011fa568` become the `float` at `01017d00` (through `FLD`/`FSTP`) and three
/// zeros. For an index below 8 (signed): the words of the table at `011fa070` from that index
/// up to the eighth are cleared, and each 16-byte record at `011fa0d0` from that index on
/// (up to `011fa150`) gets the four words at `011a9bd0`, the words `0x120` after it the four
/// `float`s.
pub fn fn_00b704c0(e: &mut Engine, index: i32) {
    if e.mem.u8(GUARD_011FA578) & 1 == 0 {
        let guard = e.mem.u32(GUARD_011FA578);
        e.mem.set_u32(GUARD_011FA578, guard | 1);
        let first = x87_quiet_bits(e.mem.u32(SOURCE_01017D00));
        e.mem.set_u32(DEFAULTS_011FA568, first);
        for k in 1..4u32 {
            e.mem.set_u32(DEFAULTS_011FA568 + 4 * k, 0);
        }
    }
    if index >= 8 {
        return;
    }
    let fourth_value = e.mem.u32(VALUES_011A9BD0 + 0xc);
    let second_default = e.mem.u32(DEFAULTS_011FA568 + 4);
    let start = index as u32;
    // `REP STOSD`: 8 - index words from the index on.
    let mut word = WORDS_011FA070.wrapping_add(start.wrapping_mul(4));
    for _ in 0..8u32.wrapping_sub(start) {
        e.mem.set_u32(word, 0);
        word = word.wrapping_add(4);
    }
    let fourth_default = e.mem.u32(DEFAULTS_011FA568 + 0xc);
    let third_default = e.mem.u32(DEFAULTS_011FA568 + 8);
    let first_default = e.mem.u32(DEFAULTS_011FA568);
    let mut record = RECORDS_011FA0D0.wrapping_add(start.wrapping_mul(16));
    loop {
        let v = e.mem.u32(VALUES_011A9BD0);
        e.mem.set_u32(record, v);
        let v = e.mem.u32(VALUES_011A9BD0 + 4);
        e.mem.set_u32(record + 4, v);
        let v = e.mem.u32(VALUES_011A9BD0 + 8);
        e.mem.set_u32(record + 0x120, first_default);
        e.mem.set_u32(record + 0x124, second_default);
        e.mem.set_u32(record + 0x128, third_default);
        e.mem.set_u32(record + 8, v);
        e.mem.set_u32(record + 0x12c, fourth_default);
        e.mem.set_u32(record + 0xc, fourth_value);
        record = record.wrapping_add(0x10);
        if (record as i32) >= (RECORDS_END_011FA150 as i32) {
            break;
        }
    }
}

// Translated from 00b71450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetViewerStrings` (engine map; the body matches): `thiscall`,
/// one word (the strings). The `BSShaderProperty` strings (`00ba9890`), the class name, the
/// light count (`+0x68`, as a 16-bit value), the number of active lights
/// ([`bs_shader_lighting_property_get_numberof_active_lights`], its low 16 bits) and the
/// reference ID (`+0x70`), each added inline.
pub fn bs_shader_lighting_property_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<Lit>,
    strings: u32,
) {
    e.call(SHADER_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI_011FA560);
    let text = e.call(VIEWER_STRING, &args![class_name]).u32();
    viewer_strings_add(e, strings, text);
    let count = e.get(this, Lit::lLightList_count);
    let text = e
        .call(VIEWER_STRING_SHORT, &args![LABEL_LIGHT_COUNT, count])
        .u32();
    viewer_strings_add(e, strings, text);
    let active = u32::from(bs_shader_lighting_property_get_numberof_active_lights(e, this) as u16);
    let text = e
        .call(VIEWER_STRING_SHORT, &args![LABEL_ACTIVE_LIGHTS, active])
        .u32();
    viewer_strings_add(e, strings, text);
    let reference = e.get(this, Lit::uiReferenceID);
    let text = e
        .call(VIEWER_STRING_UINT, &args![LABEL_REFERENCE_ID, reference])
        .u32();
    viewer_strings_add(e, strings, text);
}

// Translated from 00b71760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the deleting destructor of `BSShaderLightingProperty`: the
/// vtable `010ae7b0`, the light list emptied twice (`00b71300`, the list's own destructor
/// inlined after the class's), the base destructor (`00baa0e0`), then the sized delete
/// (`00aa1460` with 0x7c) when bit 0 of the flags is set. Answers `this`.
pub fn fn_00b71760(e: &mut Engine, this: Ptr<Lit>, flags: u32) -> Ptr<Lit> {
    let list = this.addr() + 0x60;
    e.mem.set_u32(this.addr(), LIGHTING_PROPERTY_VTABLE);
    e.call(LIST_REMOVE_ALL, &args![list]);
    e.call(LIST_REMOVE_ALL, &args![list]);
    e.call(BASE_DESTRUCT_00BAA0E0, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, 0x7cu32]);
    }
    this
}

// Translated from 00b71800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b71800 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `thiscall` with one word (a clone process): a new 0x7c-byte
/// `BSShaderLightingProperty` (`00aa13e0`; null when the allocation fails), constructed
/// inline as [`fn_00b71720`] does it (the shader property part `00ba8e50`, the vtable, an
/// emptied light list, forced darkness 1.0, reference ID 0, not changed, type 1, no fence);
/// then the copy `00ba8b10` into it and this property's slot +0xc8 with it. Answers it.
pub fn fn_00b71800(e: &mut Engine, this: Ptr<Lit>, process: u32) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x7cu32]).u32();
    let copy = if block != 0 {
        let lit = Ptr::<Lit>::new(block);
        e.call(SHADER_PROPERTY_CONSTRUCT, &args![lit]);
        e.mem.set_u32(block, LIGHTING_PROPERTY_VTABLE);
        e.set(lit, Lit::lLightList_count, 0);
        e.set(lit, Lit::lLightList, 0);
        e.set(lit, Lit::lLightList_tail, 0);
        e.call(LIST_REMOVE_ALL, &args![block + 0x60]);
        e.set(lit, Lit::fForcedDarkness, x87_store32(1.0));
        e.set(lit, Lit::uiReferenceID, 0);
        e.set(lit, Lit::bLightListChanged, 0);
        e.set(lit, Lit::iShaderPropertyType, 1);
        e.set(lit, Lit::fence, 0);
        block
    } else {
        0
    };
    e.call(BASE_COPY_MEMBERS_00BA8B10, &args![this, copy, process]);
    e.vcall(this.addr(), 0xc8, &args![copy]);
    copy
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_0340, fn_00b70340(Ptr<Lit>, u32, u32, Ptr) -> bool),
        entry!(
            0x00b7_0390,
            bs_shader_lighting_property_resort_lights(Ptr<Lit>, u32)
        ),
        entry!(
            0x00b7_0490,
            bs_shader_lighting_property_get_first_light(Ptr<Lit>, Ptr) -> u32
        ),
        entry!(
            0x00b7_0590,
            bs_shader_lighting_property_get_first_active_light(Ptr<Lit>, Ptr) -> u32
        ),
        entry!(
            0x00b7_0600,
            bs_shader_lighting_property_get_first_active_non_shadow_light(Ptr<Lit>, Ptr) -> u32
        ),
        entry!(
            0x00b7_0680,
            bs_shader_lighting_property_get_next_active_light(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00b7_0700,
            bs_shader_lighting_property_get_next_active_non_shadow_light(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00b7_0790,
            bs_shader_lighting_property_get_numberof_active_lights(Ptr<Lit>) -> u32
        ),
        entry!(
            0x00b7_07d0,
            bs_shader_lighting_property_get_numberof_active_non_shadow_lights(Ptr<Lit>) -> u32
        ),
        entry!(0x00b7_13c0, fn_00b713c0(Ptr, u32, Ptr)),
        entry!(0x00b7_1560, fn_00b71560(Ptr<Lit>, u32, u32)),
        entry!(
            0x00b7_1600,
            bs_shader_lighting_property_remove_fenced_lights(Ptr<Lit>, u32)
        ),
        entry!(0x00b7_1720, fn_00b71720(Ptr<Lit>) -> Ptr<Lit>),
        entry!(
            0x00b7_17a0,
            bs_shader_lighting_property_remove_light(Ptr<Lit>, u32)
        ),
        entry!(
            0x00b7_18b0,
            bs_shader_lighting_property_fence_all_lights(Ptr<Lit>)
        ),
        entry!(0x00b7_04b0, fn_00b704b0(Ptr<Lit>) -> u16),
        entry!(0x00b7_04c0, fn_00b704c0(i32)),
        entry!(
            0x00b7_1450,
            bs_shader_lighting_property_get_viewer_strings(Ptr<Lit>, u32)
        ),
        entry!(0x00b7_1760, fn_00b71760(Ptr<Lit>, u32) -> Ptr<Lit>),
        entry!(0x00b7_1800, fn_00b71800(Ptr<Lit>, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
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

    /// A light: active unless `index` is 0xFF or `hidden`, casting shadows
    /// when `shadow`.
    fn light(e: &mut Engine, index: u16, hidden: bool, shadow: bool) -> u32 {
        let ni_light = e.mem.alloc(0x40);
        e.mem.set_u8(ni_light + 0x30, u8::from(hidden));
        let light = e.mem.alloc(0x120);
        e.mem.set_u32(light + 0xf8, ni_light);
        e.mem.set_u16(light + 0x110, index);
        e.mem.set_u8(light + 0xec, u8::from(shadow));
        light
    }

    /// A property whose light list holds `lights` in order; answers it and
    /// the nodes.
    fn property(e: &mut Engine, lights: &[u32]) -> (Ptr<Lit>, Vec<u32>) {
        let this: Ptr<Lit> = e.new_object();
        let nodes: Vec<u32> = lights.iter().map(|_| e.mem.alloc(0xc)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(node + 8, lights[i]);
            if i + 1 < nodes.len() {
                e.mem.set_u32(*node, nodes[i + 1]);
            }
            if i > 0 {
                e.mem.set_u32(node + 4, nodes[i - 1]);
            }
        }
        e.set(this, Lit::lLightList, nodes.first().copied().unwrap_or(0));
        e.set(
            this,
            Lit::lLightList_tail,
            nodes.last().copied().unwrap_or(0),
        );
        e.set(this, Lit::lLightList_count, nodes.len() as u32);
        (this, nodes)
    }

    fn order(e: &Engine, this: Ptr<Lit>) -> Vec<u32> {
        let mut out = vec![];
        let mut node = e.get(this, Lit::lLightList);
        while node != 0 {
            out.push(e.mem.u32(node + 8));
            node = e.mem.u32(node);
        }
        out
    }

    #[test]
    fn fn_00b70340_finds_the_node_of_a_light() {
        let mut e = Engine::new();
        let (a, b) = (
            light(&mut e, 0, false, false),
            light(&mut e, 0, false, false),
        );
        let (this, nodes) = property(&mut e, &[a, b]);
        let slot = e.mem.alloc(4);
        assert!(e.call(0x00b7_0340, &args![this, b, 0u32, slot]).bool());
        assert_eq!(e.mem.u32(slot), nodes[1]);
        assert!(!e
            .call(0x00b7_0340, &args![this, 0x77u32, 0u32, slot])
            .bool());
        assert_eq!(e.mem.u32(slot), 0);
        e.set(this, Lit::lLightList_count, 0);
        assert!(!e.call(0x00b7_0340, &args![this, a, 0u32, slot]).bool());
    }

    #[test]
    fn resort_lights_moves_each_light_before_a_brighter_one() {
        let mut e = Engine::new();
        let (a, b, c) = (
            light(&mut e, 1, false, false),
            light(&mut e, 2, false, false),
            light(&mut e, 3, false, false),
        );
        // Luminance: the light's index.
        e.register(LIGHT_LUMINANCE_TO_BOUND, |e, a| {
            f64::from(e.mem.u16(a[0] + 0x110)).into_ret()
        });
        let (this, _) = property(&mut e, &[c, a, b]);
        e.set(this, Lit::iLastRenderPassState, 9);
        e.call(0x00b7_0390, &args![this, 0x5000u32]);
        // Each node moves before the first brighter one ahead of it: a
        // before c, then b before c.
        assert_eq!(order(&e, this), vec![a, b, c]);
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 0);
        e.set(this, Lit::iLastRenderPassState, 9);
        // Reversed luminance: b before a, then c before b.
        e.register(LIGHT_LUMINANCE_TO_BOUND, |e, a| {
            (10.0 - f64::from(e.mem.u16(a[0] + 0x110))).into_ret()
        });
        e.call(0x00b7_0390, &args![this, 0x5000u32]);
        assert_eq!(order(&e, this), vec![c, b, a]);
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 0);
        // NaN luminance never moves anything.
        e.register(LIGHT_LUMINANCE_TO_BOUND, |_, _| f64::NAN.into_ret());
        e.set(this, Lit::iLastRenderPassState, 9);
        e.call(0x00b7_0390, &args![this, 0x5000u32]);
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 9);
    }

    #[test]
    fn get_first_light_answers_the_head() {
        let mut e = Engine::new();
        let a = light(&mut e, 1, false, false);
        let (this, _) = property(&mut e, &[a]);
        let iterator = e.mem.alloc(4);
        assert_eq!(e.call(0x00b7_0490, &args![this, iterator]).u32(), a);
        assert_eq!(e.mem.u32(iterator), 0);
        let (empty, _) = property(&mut e, &[]);
        assert_eq!(e.call(0x00b7_0490, &args![empty, iterator]).u32(), 0);
    }

    #[test]
    fn first_and_next_active_lights_skip_inactive_ones() {
        let mut e = Engine::new();
        let off = light(&mut e, 0xff, false, false);
        let hidden = light(&mut e, 1, true, false);
        let on = light(&mut e, 2, false, false);
        let shadow = light(&mut e, 3, false, true);
        let last = light(&mut e, 4, false, false);
        let (this, nodes) = property(&mut e, &[off, hidden, on, shadow, last]);
        let iterator = e.mem.alloc(4);
        assert_eq!(e.call(0x00b7_0590, &args![this, iterator]).u32(), on);
        assert_eq!(e.mem.u32(iterator), nodes[3]);
        assert_eq!(e.call(0x00b7_0680, &args![this, iterator]).u32(), shadow);
        assert_eq!(e.call(0x00b7_0680, &args![this, iterator]).u32(), last);
        assert_eq!(e.call(0x00b7_0680, &args![this, iterator]).u32(), 0);
        assert_eq!(e.call(0x00b7_0600, &args![this, iterator]).u32(), on);
        assert_eq!(e.call(0x00b7_0700, &args![this, iterator]).u32(), last);
        // A last inactive light gives 0.
        let (tail_off, _) = property(&mut e, &[off]);
        assert_eq!(e.call(0x00b7_0590, &args![tail_off, iterator]).u32(), 0);
        assert_eq!(e.call(0x00b7_0600, &args![tail_off, iterator]).u32(), 0);
    }

    #[test]
    fn counts_active_and_non_shadow_lights() {
        let mut e = Engine::new();
        let lights = [
            light(&mut e, 0xff, false, false),
            light(&mut e, 1, true, false),
            light(&mut e, 2, false, false),
            light(&mut e, 3, false, true),
            0,
        ];
        let (this, _) = property(&mut e, &lights);
        assert_eq!(e.call(0x00b7_0790, &args![this]).u32(), 2);
        assert_eq!(e.call(0x00b7_07d0, &args![this]).u32(), 1);
    }

    #[test]
    fn fn_00b713c0_inserts_before_a_node() {
        let mut e = Engine::new();
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let (a, b) = (
            light(&mut e, 1, false, false),
            light(&mut e, 2, false, false),
        );
        let (this, nodes) = property(&mut e, &[a, b]);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x77);
        let list: Ptr = Ptr::new(this.addr() + 0x60);
        e.call(0x00b7_13c0, &args![list, nodes[1], value]);
        assert_eq!(order(&e, this), vec![a, 0x77, b]);
        e.call(0x00b7_13c0, &args![list, nodes[0], value]);
        assert_eq!(order(&e, this), vec![0x77, a, 0x77, b]);
        assert_eq!(e.get(this, Lit::lLightList_count), 4);
    }

    #[test]
    fn fn_00b71560_adds_or_moves_a_light() {
        let mut e = Engine::new();
        e.register(LIST_ADD, |_, _| Ret::default());
        e.register(LIST_MOVE_BEFORE, |_, _| Ret::default());
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let a = light(&mut e, 1, false, true);
        let b = light(&mut e, 2, false, false);
        let (this, nodes) = property(&mut e, &[a]);
        e.set(this, Lit::iLastRenderPassState, 9);
        e.call_log = Some(vec![]);
        // No fence, already there: nothing added; a shadow light clears the state.
        e.call(0x00b7_1560, &args![this, a, 5u32]);
        assert!(calls_to(&e, LIST_ADD).is_empty());
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 0);
        assert_eq!(e.get(this, Lit::bLightListChanged), 1);
        e.call(0x00b7_1560, &args![this, b, 5u32]);
        assert_eq!(calls_to(&e, LIST_ADD).len(), 1);
        assert_eq!(calls_to(&e, LIST_ADD)[0][0], this.addr() + 0x60);
        // With a fence: a known light is moved before it.
        e.set(this, Lit::fence, nodes[0]);
        e.call(0x00b7_1560, &args![this, a, 5u32]);
        assert_eq!(
            calls_to(&e, LIST_MOVE_BEFORE),
            vec![vec![this.addr() + 0x60, nodes[0], nodes[0]]]
        );
        // A new one is inserted before it.
        e.call(0x00b7_1560, &args![this, b, 5u32]);
        assert_eq!(order(&e, this), vec![b, a]);
    }

    #[test]
    fn remove_fenced_lights_removes_everything_after_the_fence() {
        let mut e = Engine::new();
        e.register(SPIN_LOCK_ENTER, |e, _| {
            let n = e.global::<u32>(NODE_POOL_LOCK_COUNT) + 1;
            e.set_global(NODE_POOL_LOCK_COUNT, n);
            Ret::default()
        });
        e.register(LIGHT_REMOVE_TRI_GEOM, |_, _| Ret::default());
        e.register(LIST_REMOVE_NODE, |_, _| Ret::default());
        e.register(LIST_REMOVE_TAIL, |e, a| {
            let tail = e.mem.u32(a[0] + 4);
            let previous = e.mem.u32(tail + 4);
            e.mem.set_u32(a[0] + 4, previous);
            e.mem.set_u32(previous, 0);
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count - 1);
            Ret::default()
        });
        e.map(NODE_POOL_FREE_LIST, 0x40);
        e.map(NODE_POOL_LOCK, 8);
        let fence_light = light(&mut e, 1, false, false);
        let middle = light(&mut e, 2, false, true);
        let tail = light(&mut e, 3, false, false);
        let (this, nodes) = property(&mut e, &[fence_light, middle, tail]);
        e.set(this, Lit::fence, nodes[0]);
        e.set(this, Lit::iLastRenderPassState, 9);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1600, &args![this, 0x6000u32]);
        assert_eq!(order(&e, this), vec![fence_light]);
        assert_eq!(e.get(this, Lit::lLightList_count), 1);
        assert_eq!(e.global::<u32>(NODE_POOL_FREE_LIST), nodes[1]);
        assert_eq!(
            calls_to(&e, LIGHT_REMOVE_TRI_GEOM),
            vec![vec![middle, 0x6000, 0], vec![tail, 0x6000, 0]]
        );
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 0);
        assert_eq!(
            calls_to(&e, LIST_REMOVE_NODE),
            vec![vec![this.addr() + 0x60, this.addr() + 0x78]]
        );
        assert_eq!(e.get(this, Lit::fence), 0);
        // A fence outside the list whose next is the head: the head branch,
        // twice.
        let (other, other_nodes) = property(&mut e, &[fence_light, middle]);
        let outside = e.mem.alloc(0xc);
        e.mem.set_u32(outside, other_nodes[0]);
        e.set(other, Lit::fence, outside);
        e.call(0x00b7_1600, &args![other, 0x6000u32]);
        assert_eq!(e.get(other, Lit::lLightList), 0);
        assert_eq!(e.get(other, Lit::lLightList_tail), 0);
        assert_eq!(e.get(other, Lit::lLightList_count), 0);
        assert_eq!(e.global::<u32>(NODE_POOL_FREE_LIST), other_nodes[1]);
    }

    #[test]
    fn constructor_sets_up_an_empty_property() {
        let mut e = Engine::new();
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        let this: Ptr<Lit> = e.new_object();
        e.set(this, Lit::fence, 5);
        assert_eq!(e.call(0x00b7_1720, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), LIGHTING_PROPERTY_VTABLE);
        assert_eq!(e.get(this, Lit::fForcedDarkness), 1.0f32.to_bits());
        assert_eq!(e.get(this, Lit::iShaderPropertyType), 1);
        assert_eq!(e.get(this, Lit::fence), 0);
    }

    #[test]
    fn remove_light_breaks_when_the_light_is_still_listed() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE, |_, _| Ret::default());
        e.register(DEBUG_BREAK, |_, _| Ret::default());
        let a = light(&mut e, 1, false, true);
        let (this, _) = property(&mut e, &[a]);
        e.call_log = Some(vec![]);
        e.call(0x00b7_17a0, &args![this, a]);
        assert_eq!(calls_to(&e, DEBUG_BREAK).len(), 1);
        assert_eq!(e.get(this, Lit::bLightListChanged), 1);
        assert_eq!(calls_to(&e, LIST_REMOVE)[0][0], this.addr() + 0x60);
        // With a fence nothing happens.
        e.set(this, Lit::fence, 1);
        e.call(0x00b7_17a0, &args![this, a]);
        assert_eq!(calls_to(&e, LIST_REMOVE).len(), 1);
    }

    #[test]
    fn fence_all_lights_puts_the_marker_at_the_head() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE, |e, a| {
            assert_eq!(e.mem.u32(a[1]), FENCE_MARKER);
            Ret::default()
        });
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let (empty, _) = property(&mut e, &[]);
        e.call(0x00b7_18b0, &args![empty]);
        let fence = e.get(empty, Lit::fence);
        assert_eq!(e.get(empty, Lit::lLightList), fence);
        assert_eq!(e.get(empty, Lit::lLightList_tail), fence);
        assert_eq!(e.get(empty, Lit::lLightList_count), 1);
        let a = light(&mut e, 1, false, false);
        let (this, nodes) = property(&mut e, &[a]);
        e.call(0x00b7_18b0, &args![this]);
        assert_eq!(order(&e, this), vec![FENCE_MARKER, a]);
        assert_eq!(e.mem.u32(nodes[0] + 4), e.get(this, Lit::fence));
        assert_eq!(e.get(this, Lit::lLightList_count), 2);
    }
}

#[cfg(test)]
mod n012_tests {
    use super::*;
    use crate::units::bsshader::bsshaderproperty::{STRINGS_SET_AT, STRINGS_SET_SIZE};

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00b704b0_answers_the_low_count() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x7c);
        e.mem.set_u32(this + 0x68, 0x0003_0002);
        assert_eq!(e.call(0x00b7_04b0, &args![this]).u32(), 2);
    }

    #[test]
    fn fn_00b704c0_resets_the_tables_from_the_index() {
        let mut e = Engine::new();
        e.map(0x011f_a000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.mem.set_u32(SOURCE_01017D00, 0x7f80_0001);
        for k in 0..4u32 {
            e.mem.set_u32(VALUES_011A9BD0 + 4 * k, 0x10 + k);
        }
        for k in 0..8u32 {
            e.mem.set_u32(WORDS_011FA070 + 4 * k, 0xee);
        }
        e.call(0x00b7_04c0, &args![6u32]);
        assert_eq!(e.mem.u32(DEFAULTS_011FA568), 0x7fc0_0001);
        assert_eq!(e.mem.u8(GUARD_011FA578) & 1, 1);
        assert_eq!(e.mem.u32(WORDS_011FA070 + 4 * 5), 0xee);
        assert_eq!(e.mem.u32(WORDS_011FA070 + 4 * 6), 0);
        assert_eq!(e.mem.u32(WORDS_011FA070 + 4 * 7), 0);
        assert_eq!(e.mem.u32(RECORDS_011FA0D0 + 0x50), 0);
        for record in [RECORDS_011FA0D0 + 0x60, RECORDS_011FA0D0 + 0x70] {
            assert_eq!(e.mem.u32(record), 0x10);
            assert_eq!(e.mem.u32(record + 0xc), 0x13);
            assert_eq!(e.mem.u32(record + 0x120), 0x7fc0_0001);
            assert_eq!(e.mem.u32(record + 0x12c), 0);
        }
        // The guard is set: the defaults are kept; an index of 8 changes nothing.
        e.mem.set_u32(DEFAULTS_011FA568 + 4, 0x3f80_0000);
        e.call(0x00b7_04c0, &args![7u32]);
        assert_eq!(e.mem.u32(RECORDS_011FA0D0 + 0x70 + 0x124), 0x3f80_0000);
        assert_eq!(e.mem.u32(RECORDS_011FA0D0 + 0x60 + 0x124), 0);
        e.mem.set_u32(WORDS_011FA070, 0xee);
        e.call(0x00b7_04c0, &args![8u32]);
        assert_eq!(e.mem.u32(WORDS_011FA070), 0xee);
    }

    #[test]
    fn get_viewer_strings_adds_light_counts_and_reference() {
        let mut e = Engine::new();
        e.map(0x011f_a000, 0x1000);
        e.mem.set_u32(CLASS_RTTI_011FA560, 0x0011_0000);
        e.register(SHADER_PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, a| a[0].into_ret());
        e.register(VIEWER_STRING_SHORT, |_, a| (a[0] << 8 | a[1]).into_ret());
        e.register(VIEWER_STRING_UINT, |_, a| (a[1] + 0x900).into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let v = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], v);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x7c);
        e.mem.set_u32(this + 0x68, 0);
        e.mem.set_u32(this + 0x70, 0x42);
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x20);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1450, &args![this, strings]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_SHORT),
            vec![vec![LABEL_LIGHT_COUNT, 0], vec![LABEL_ACTIVE_LIGHTS, 0]]
        );
        assert_eq!(e.mem.u32(base), 0x0011_0000);
        assert_eq!(e.mem.u32(base + 0xc), 0x942);
        assert_eq!(e.mem.u16(strings + 0xa), 4);
    }

    #[test]
    fn deleting_destructor_empties_the_list_and_frees_with_bit_0() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(BASE_DESTRUCT_00BAA0E0, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(0x7c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b7_1760, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), LIGHTING_PROPERTY_VTABLE);
        assert_eq!(calls(&e, LIST_REMOVE_ALL), vec![vec![this + 0x60]; 2]);
        assert_eq!(calls(&e, BASE_DESTRUCT_00BAA0E0), vec![vec![this]]);
        assert_eq!(calls(&e, NI_FREE_SIZED), vec![vec![this, 0x7c]]);
        e.call(0x00b7_1760, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_SIZED).len(), 1);
    }

    #[test]
    fn fn_00b71800_clones_into_a_new_property() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS_00BA8B10, |_, _| Ret::default());
        e.register(0x0f00_00c8, |_, _| Ret::default());
        let mut slots = [0u32; 0x33];
        slots[0xc8 / 4] = 0x0f00_00c8;
        e.put_vtable(0x0f00_5000, &slots);
        let this = e.mem.alloc(0x7c);
        e.mem.set_u32(this, 0x0f00_5000);
        e.call_log = Some(vec![]);
        let copy = e.call(0x00b7_1800, &args![this, 3u32]).u32();
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x7c]]);
        assert_eq!(e.mem.u32(copy), LIGHTING_PROPERTY_VTABLE);
        assert_eq!(e.mem.f32(copy + 0x6c), 1.0);
        assert_eq!(e.mem.u32(copy + 0x1c), 1);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS_00BA8B10),
            vec![vec![this, copy, 3]]
        );
        assert_eq!(calls(&e, 0x0f00_00c8), vec![vec![this, copy]]);
    }
}
