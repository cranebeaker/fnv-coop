//! `fallout shared/tesobjectrefr.cpp` (Xbox PDB source unit), part 3: its functions from `0056a860` up to
//! (not including) `0056ea40` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesobjectrefr`]; anything public there may be used here.
//!
//! # Where this file stops
//!
//! This holds the first three batches of 40 functions of the range:
//! `0056a860` to `0056d230` (the enable-parent and package-start-location
//! helpers, the small `ExtraDataList` accessors the linker placed here,
//! `Load3D` (`0056b2d0`), the setters of the Havok glue and
//! `InitHavokForPlaceableWater` (`0056c8f0`)), and `0056d250` to `0056e090`
//! (the Havok constructors, destructors and `GetRTTI` methods of
//! `bhkWorldObject`, `bhkEntity`, `bhkRigidBody` and `bhkCollisionObject`,
//! the Gamebryo-to-Havok transform conversion `NI2HK` and
//! `InitHavokForPrimitiveTrigger` (`0056d7e0`)), and `0056e0b0` to `0056ea00`
//! (the phantom creation-info helpers and the constructors, `GetRTTI`
//! methods and destructors of the `bhk*Phantom` and `bhk*Shape` classes).
//! The range is complete: the next function after `0056ea00` is
//! `0056ea40`, the start of the following part.
//!
//! Several functions here are not `TESObjectREFR` methods but small helpers
//! of other classes (a `BSFadeNode` field setter, a `LOADED_REF_DATA` flag
//! setter, ...); their `this` is documented per function. Callees outside
//! this file, including the unit's own other parts, are called by address.
//!
//! # What is not translated
//!
//! The compiler's exception-unwinding frames (`__CxxFrameHandler` states)
//! and the stack-protector cookie check of `Load3D` and
//! `InitHavokForPlaceableWater` are left out. The stack locals the game
//! passes by address (a `NiPointer` temporary, vectors, a Havok
//! `bhkRigidBodyCinfo`) are blocks of the engine's memory, freed on every
//! exit.

#[allow(unused_imports)]
use super::tesobjectrefr::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `this + 0x44`: the reference's embedded `ExtraDataList` (`005d43c0`).
const GET_EXTRA_LIST: u32 = 0x005d_43c0;
/// The reference's base object, `this + 0x20` (`007af430`; the engine map
/// names it `BGSSaveFormBuffer::GetForm` because the linker folded the
/// identical code).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// `TESForm::GetFormType` (`00401170`, `MOVZX EAX,[ECX+4]`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `TESObjectREFR::GetRefPersists` (`005653d0`).
const GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// `TESForm::iFormID` getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `this + 0x40`, the reference's parent cell (`008d6f30`).
const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// The identity function on `this` (`006815c0`): the item slot of a list
/// node, which is the node's first field.
const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
/// The next node of a list node (`00726070`).
const LIST_NEXT: u32 = 0x0072_6070;
/// `NiPointer<T>::operator T*`: the first dword of the pointer (`00559450`).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<T>::NiPointer(T*)` (`00633c90`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer<T>::operator=(T*)` (`0066b0d0`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer<T>::~NiPointer` (`0045cec0`).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `__RTDynamicCast(object, 0, from, to, 0)` (cdecl).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors the casts use (`.?AVTESForm@@` and
/// `.?AVTESObjectREFR@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `TESObjectREFR` vtable slots used here: `IsActor`, the slot that returns
/// the reference's 3D root, the one that returns a pointer to its position,
/// the changed-flags setters and the base-object test the save helpers use.
const SLOT_IS_ACTOR: u32 = 0x100;
const SLOT_GET_3D: u32 = 0x1d0;
const SLOT_GET_POSITION: u32 = 0x1f4;
const SLOT_SET_CHANGED: u32 = 0x48;
const SLOT_FORCE_CHANGED: u32 = 0x4c;
const SLOT_BASE_FORM_TEST: u32 = 0xf8;
/// The `NiObject` vtable slot `IsNode` (Xbox PDB `+0x00c`) and `IsFadeNode`
/// (`+0x010`).
const SLOT_IS_NODE: u32 = 0xc;
const SLOT_IS_FADE_NODE: u32 = 0x10;

fn extra_list(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_EXTRA_LIST, &args![refr]).u32()
}

fn base_form(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_BASE_FORM, &args![refr]).u32()
}

fn form_type(e: &mut Engine, form: u32) -> u32 {
    e.call(FORM_TYPE, &args![form]).u32()
}

/// The first dword of a `NiPointer` slot.
fn ni_pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

/// `__RTDynamicCast(object, 0, from, to, 0)`.
fn dynamic_cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    e.call(DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

/// `pObjectReference` (`this + 0x20`), read directly the way the game does
/// where it does not go through `007af430`.
fn base_object_field(e: &Engine, this: Ptr<TESObjectREFR>) -> u32 {
    e.get(this.at(TESObjectREFR::data), OBJ_REFR::pObjectReference)
        .addr()
}

/// A set of stack locals the game passes by address, freed together.
#[derive(Default)]
struct Locals(Vec<u32>);

impl Locals {
    fn alloc(&mut self, e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size);
        self.0.push(block);
        block
    }

    fn release(self, e: &mut Engine) {
        for block in self.0 {
            e.mem.free(block);
        }
    }
}

// Translated from 0056a860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the pointer at `this + 0x80` is set. Called by `0056a4a0` and
/// `0058e730` on a record that carries such a pointer; the class is not
/// named in the Xbox PDB for this body.
pub fn fn_0056a860(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x80) != 0
}

// Translated from 0056a880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer at `this + 0x80`, or [`fn_0056a8c0`]'s default object when it
/// is null.
pub fn fn_0056a880(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.mem.u32(this.addr() + 0x80);
    if value == 0 {
        fn_0056a8c0(e)
    } else {
        value
    }
}

// Translated from 0056a8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the default record (`011ca8b4`, in `.data`) that
/// [`fn_0056a880`] falls back to.
pub fn fn_0056a8c0(_e: &mut Engine) -> u32 {
    0x011c_a8b4
}

// Translated from 0056a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the children of the reference's 3D root (virtual `+0x1d0` of the
/// reference, then `IsNode`, virtual `+0xc`, of the root; nothing happens
/// without a node). For every child for which
/// `0045bad0(011c7d34, child)` holds, calls `0050f5a0` on the child and
/// clears its slot in the node (virtual `+0xf8` with the child index and 0).
/// The child count (`0043b480`) is read again on every iteration, as in the
/// game.
pub fn fn_0056a8d0(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let root = e.vcall(this.addr(), SLOT_GET_3D, &args![]).u32();
    let node = if root == 0 {
        0
    } else {
        e.vcall(root, SLOT_IS_NODE, &args![]).u32()
    };
    if node == 0 {
        return;
    }
    let mut index = 0u32;
    while index < e.call(0x0043_b480, &args![node]).u32() {
        let child = e.call(0x0043_b4a0, &args![node, index]).u32();
        if child != 0 && e.call(0x0045_bad0, &args![0x011c_7d34u32, child]).bool() {
            e.call(0x0050_f5a0, &args![child]);
            e.vcall(node, 0xf8, &args![index, 0u32]);
        }
        index += 1;
    }
}

// Translated from 0056a990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetMerchantContainer` (`00421400`) of the reference's
/// extra data list.
pub fn fn_0056a990(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(0x0042_1400, &args![list]).u32()
}

// Translated from 0056a9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes `other` to `00421430` of the reference's extra data list, unless
/// `other` is a non-null reference that is not persistent
/// (`TESObjectREFR::GetRefPersists`).
pub fn fn_0056a9b0(e: &mut Engine, this: Ptr<TESObjectREFR>, other: Ptr<TESObjectREFR>) {
    if other.addr() != 0 && !e.call(GET_REF_PERSISTS, &args![other]).bool() {
        return;
    }
    let list = extra_list(e, this.addr());
    e.call(0x0042_1430, &args![list, other]);
}

// Translated from 0056a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The enable-state parent: `0041da10` of the reference's extra data list
/// (the pointer in its enable-state-parent record, 0 without one).
pub fn fn_0056a9f0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(0x0041_da10, &args![list]).u32()
}

// Translated from 0056aa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-parents the enable state: tells the current enable-state parent's
/// extra data list about this reference (`0041dda0(parent list, this)`),
/// then gives `other` to this reference's own list (`0041da40`), unless
/// `other` is a non-null reference that is not persistent.
pub fn fn_0056aa10(e: &mut Engine, this: Ptr<TESObjectREFR>, other: Ptr<TESObjectREFR>) {
    let parent = fn_0056a9f0(e, this);
    if parent != 0 {
        let list = extra_list(e, parent);
        e.call(0x0041_dda0, &args![list, this]);
    }
    if other.addr() != 0 && !e.call(GET_REF_PERSISTS, &args![other]).bool() {
        return;
    }
    let list = extra_list(e, this.addr());
    e.call(0x0041_da40, &args![list, other]);
}

// Translated from 0056aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0041db00` of the reference's extra data list: a flag of the
/// enable-state-parent record, read by
/// `UpdateEnableStateChildrenFlagsRecursive` to invert the disabled state.
pub fn fn_0056aa70(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let list = extra_list(e, this.addr());
    e.call(0x0041_db00, &args![list]).bool()
}

// Translated from 0056aa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0041db50(list, flag)` on the reference's extra data list, the setter
/// that goes with [`fn_0056aa70`].
pub fn fn_0056aa90(e: &mut Engine, this: Ptr<TESObjectREFR>, flag: u8) {
    let list = extra_list(e, this.addr());
    e.call(0x0041_db50, &args![list, flag]);
}

// Translated from 0056aac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::CheckEnableParentLoop` (Xbox PDB): walks the enable-state
/// parent chain upwards from this reference. Returns false when the chain
/// leads back to the reference itself and true when it ends, or when
/// `0057c850` reports the visited set closed the chain.
///
/// The walk keeps a visited set in a 16-byte local that `0057c470`
/// constructs (with `0x25`) and `0057c8d0` destroys; every link is recorded
/// with `0084d310(set, link, 1)`. A link that `004013e0` accepts is followed
/// to its own enable-state parent; any other link's parent is resolved
/// through the save-game reference lookup (`004839c0`) and cast to
/// `TESObjectREFR`. The compiler's exception-unwinding frame is not
/// translated.
pub fn tes_object_refr_check_enable_parent_loop(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let set = e.mem.alloc(0x10);
    let seen_twice = e.mem.alloc(4);
    let mut acyclic = true;
    e.call(0x0057_c470, &args![set, 0x25u32]);

    let mut current = fn_0056a9f0(e, this);
    while current != 0 && acyclic {
        if current == this.addr() {
            acyclic = false;
            continue;
        }
        e.call(0x0084_d310, &args![set, current, 1u32]);
        if e.call(0x0040_13e0, &args![current]).bool() {
            current = fn_0056a9f0(e, Ptr::new(current));
        } else {
            let parent = fn_0056a9f0(e, Ptr::new(current));
            current = if parent == 0 {
                0
            } else {
                let resolved = e.call(0x0048_39c0, &args![parent]).u32();
                dynamic_cast(e, resolved, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR)
            };
        }
        e.mem.set_u8(seen_twice, 0);
        if current != 0
            && e.call(0x0057_c850, &args![set, current, seen_twice]).bool()
            && e.mem.u8(seen_twice) != 0
        {
            current = 0;
        }
    }
    e.call(0x0057_c8d0, &args![set]);
    e.mem.free(set);
    e.mem.free(seen_twice);
    acyclic
}

// Translated from 0056ac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::UpdateEnableStateChildrenFlagsRecursive` (Xbox PDB): for
/// every child reference in the enable-state children list
/// ([`fn_0056ac90`]), works out the disabled state the child should have
/// (this reference's, inverted when the child's flag [`fn_0056aa70`] is set)
/// and, when the child differs, applies it with `TESForm::SetDisabled`
/// (`00484af0`) and recurses into the child.
pub fn tes_object_refr_update_enable_state_children_flags_recursive(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) {
    let mut node = fn_0056ac90(e, this);
    while node != 0 {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let child = e.mem.u32(slot);
        if child != 0 {
            let mut disabled = e.call(0x0044_0da0, &args![this]).u8();
            if fn_0056aa70(e, Ptr::new(child)) {
                disabled = u8::from(disabled == 0);
            }
            if e.call(0x0044_0da0, &args![child]).u8() != disabled {
                e.call(0x0048_4af0, &args![child, disabled]);
                tes_object_refr_update_enable_state_children_flags_recursive(e, Ptr::new(child));
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

// Translated from 0056ac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0041dca0` of the reference's extra data list: the head of the
/// enable-state children list (0 without one).
pub fn fn_0056ac90(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    e.call(0x0041_dca0, &args![list]).u32()
}

// Translated from 0056acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(holder, item)`. Does nothing unless both are non-null, `holder`
/// is not the global at `011dea3c`, `item` is a non-persistent reference
/// (`GetRefPersists` false) and virtual `+0x160` of `item` is false. Then
/// it passes `item` to `0041e000` of `holder`'s extra data list, records
/// `holder` in `item`'s list (`ExtraDataList::AddDroppedItem`, `0041de40`)
/// and calls virtual `+0x48` of `item` with `0x80000000`.
pub fn fn_0056acb0(e: &mut Engine, holder: Ptr<TESObjectREFR>, item: Ptr<TESObjectREFR>) {
    if item.addr() == 0 || holder.addr() == 0 || holder.addr() == e.global::<u32>(0x011d_ea3c) {
        return;
    }
    if e.call(GET_REF_PERSISTS, &args![item]).bool() {
        return;
    }
    if e.vcall(item.addr(), 0x160, &args![]).bool() {
        return;
    }
    let holder_list = extra_list(e, holder.addr());
    e.call(0x0041_e000, &args![holder_list, item]);
    let item_list = extra_list(e, item.addr());
    e.call(0x0041_de40, &args![item_list, holder]);
    e.vcall(item.addr(), SLOT_SET_CHANGED, &args![0x8000_0000u32]);
}

// Translated from 0056ad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetPackageStartLocationWorld` (Xbox PDB): the form of the
/// extra data list's package start location when its form type is `0x41`,
/// else 0.
pub fn tes_object_refr_get_package_start_location_world(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) -> u32 {
    package_start_form_of_type(e, this, 0x41)
}

// Translated from 0056ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetPackageStartLocationInteriorCell` (Xbox PDB): the form
/// of the extra data list's package start location when its form type is
/// `0x39`, else 0.
pub fn tes_object_refr_get_package_start_location_interior_cell(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) -> u32 {
    package_start_form_of_type(e, this, 0x39)
}

/// The form of the package start location record
/// (`ExtraDataList::GetPackageStartLocation`, `00418b70`; its first word is
/// the form) when that form has the type `kind`, else 0.
fn package_start_form_of_type(e: &mut Engine, this: Ptr<TESObjectREFR>, kind: u32) -> u32 {
    let list = extra_list(e, this.addr());
    let record = e.call(0x0041_8b70, &args![list]).u32();
    if record == 0 {
        return 0;
    }
    let form = e.mem.u32(record);
    if form != 0 && form_type(e, form) == kind {
        form
    } else {
        0
    }
}

// Translated from 0056add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetPackageStartLocationCoord` (Xbox PDB): the position
/// part of the package start location record (record + 4), or without a
/// record the reference's own position (virtual `+0x1f4`).
pub fn tes_object_refr_get_package_start_location_coord(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) -> u32 {
    let list = extra_list(e, this.addr());
    let record = e.call(0x0041_8b70, &args![list]).u32();
    if record != 0 {
        record + 4
    } else {
        e.vcall(this.addr(), SLOT_GET_POSITION, &args![]).u32()
    }
}

// Translated from 0056ae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetPackageStartLocation` (Xbox PDB): stores the package
/// start location in the extra data list
/// (`ExtraDataList::SetPackageStartLocation`, `0041ad00`: the form
/// `location`, the words `second` and `third` copied unchanged, and the
/// float `fourth`), then calls virtual `+0x48` with `0x800`.
pub fn tes_object_refr_set_package_start_location(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    location: u32,
    second: u32,
    third: u32,
    fourth: f32,
) {
    let list = extra_list(e, this.addr());
    e.call(0x0041_ad00, &args![list, location, second, third, fourth]);
    e.vcall(this.addr(), SLOT_SET_CHANGED, &args![0x800u32]);
}

// Translated from 0056ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A property of the reference's base object that depends on the form type:
/// false when [`fn_0056af40`] holds. Otherwise, for form type `0x1b`, bit 2
/// of the byte at base `+0x98` ([`fn_0056af20`]), and for form types `0x2a`
/// and `0x2b`, [`fn_0056af00`] on `base + 0x30`; false for any other type.
pub fn fn_0056ae60(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let mut result = false;
    if fn_0056af40(e, this) || base_form(e, this.addr()) == 0 {
        return result;
    }
    let base = base_form(e, this.addr());
    let kind = form_type(e, base) as i32;
    if kind == 0x1b {
        let base = base_form(e, this.addr());
        if base != 0 {
            result = fn_0056af20(e, Ptr::new(base));
        }
    } else if kind > 0x29 && kind <= 0x2b {
        let base = base_form(e, this.addr());
        if base != 0 {
            result = fn_0056af00(e, Ptr::new(base + 0x30));
        }
    }
    result
}

// Translated from 0056af00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00461580(this, 8)`: whether the dword at `this + 4` has the bit `8` set
/// (here `this` is the base object's `+0x30` sub-object).
pub fn fn_0056af00(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0046_1580, &args![this, 8u32]).bool()
}

// Translated from 0056af20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the (sign-extended) byte at `this + 0x98` is set.
pub fn fn_0056af20(e: &mut Engine, this: Ptr) -> bool {
    e.mem.i8(this.addr() + 0x98) as i32 & 2 != 0
}

// Translated from 0056af40 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the reference has a base object whose virtual `+0xf8` holds
/// and either the extra data list's flag `004216d0` is set or
/// `0047cdb0(base + 0x30)` holds.
pub fn fn_0056af40(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if base_form(e, this.addr()) == 0 {
        return false;
    }
    let base = base_form(e, this.addr());
    if !e.vcall(base, SLOT_BASE_FORM_TEST, &args![]).bool() {
        return false;
    }
    let list = extra_list(e, this.addr());
    if e.call(0x0042_16d0, &args![list]).bool() {
        return true;
    }
    let base = base_form(e, this.addr());
    e.call(0x0047_cdb0, &args![base + 0x30]).bool()
}

// Translated from 0056afc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the reference has a base object whose virtual `+0xf8` holds and
/// the extra data list's flag `004216d0` is set.
pub fn fn_0056afc0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if base_form(e, this.addr()) == 0 {
        return false;
    }
    let base = base_form(e, this.addr());
    if !e.vcall(base, SLOT_BASE_FORM_TEST, &args![]).bool() {
        return false;
    }
    let list = extra_list(e, this.addr());
    e.call(0x0042_16d0, &args![list]).bool()
}

// Translated from 0056b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the reference's base object exists and its virtual `+0xf8` holds:
/// marks the reference changed with `0x40000` (`TESForm::ForceChange`,
/// `00484b90`, when `first` and `second` are both non-zero, else virtual
/// `+0x4c`), then passes both words to `00421750` of the extra data list.
pub fn fn_0056b020(e: &mut Engine, this: Ptr<TESObjectREFR>, first: u32, second: u32) {
    if base_form(e, this.addr()) == 0 {
        return;
    }
    let base = base_form(e, this.addr());
    if !e.vcall(base, SLOT_BASE_FORM_TEST, &args![]).bool() {
        return;
    }
    if first != 0 && second != 0 {
        e.call(0x0048_4b90, &args![this, 0x40000u32]);
    } else {
        e.vcall(this.addr(), SLOT_FORCE_CHANGED, &args![0x40000u32]);
    }
    let list = extra_list(e, this.addr());
    e.call(0x0042_1750, &args![list, first, second]);
}

// Translated from 0056b0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RestoreLeveledCreatureOriginalBase` (Xbox PDB): when the
/// reference has a base object whose form ID `00469860` (on the global
/// `011c3f2c`) accepts and [`fn_0056af40`] holds, replaces
/// `pObjectReference` with the extra data list's leveled-creature original
/// base (`ExtraDataList::GetLevCreaOriginalBase`, `004216f0`). Before that
/// the game calls virtual `+0x10` of the old base object with 1 and ignores
/// the result.
pub fn tes_object_refr_restore_leveled_creature_original_base(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) {
    let base = base_object_field(e, this);
    if base == 0 {
        return;
    }
    let id = e.call(FORM_ID, &args![base]).u32();
    let table = e.global::<u32>(0x011c_3f2c);
    if !e.call(0x0046_9860, &args![table, id]).bool() || !fn_0056af40(e, this) {
        return;
    }
    let old = base_object_field(e, this);
    if old != 0 {
        e.vcall(old, 0x10, &args![1u32]);
    }
    let list = extra_list(e, this.addr());
    let original = e.call(0x0042_16f0, &args![list]).u32();
    e.set(
        this.at(TESObjectREFR::data),
        OBJ_REFR::pObjectReference,
        Ptr::<()>::new(original),
    );
}

// Translated from 0056b140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The record `004182e0` returns for the reference's extra data list (a
/// pointer into the extra at +0xc): its word at +0xc when the word at +4 is
/// 0, else 0.
pub fn fn_0056b140(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    let record = e.call(0x0041_82e0, &args![list]).u32();
    if record != 0 && e.mem.u32(record + 4) == 0 {
        e.mem.u32(record + 0xc)
    } else {
        0
    }
}

// Translated from 0056b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at +0 of the record `004182e0` returns, when its word at +4 is
/// 0, else 0.0.
pub fn fn_0056b190(e: &mut Engine, this: Ptr<TESObjectREFR>) -> f32 {
    let list = extra_list(e, this.addr());
    let record = e.call(0x0041_82e0, &args![list]).u32();
    if record != 0 && e.mem.u32(record + 4) == 0 {
        e.mem.f32(record)
    } else {
        0.0
    }
}

// Translated from 0056b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at +8 of the record `004182e0` returns, when its word at +4 is
/// 0, else 0.0.
pub fn fn_0056b1d0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> f32 {
    let list = extra_list(e, this.addr());
    let record = e.call(0x0041_82e0, &args![list]).u32();
    if record != 0 && e.mem.u32(record + 4) == 0 {
        e.mem.f32(record + 8)
    } else {
        0.0
    }
}

// Translated from 0056b210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +4 of the record `004182e0` returns, 0 without a record.
pub fn fn_0056b210(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let list = extra_list(e, this.addr());
    let record = e.call(0x0041_82e0, &args![list]).u32();
    if record != 0 {
        e.mem.u32(record + 4)
    } else {
        0
    }
}

// Translated from 0056b250 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `00564e60` holds; otherwise false for an actor (virtual
/// `+0x100`), false when `00444ed0` holds, and else true when the base
/// object's bound size (`0050ebf0`, a float) is greater than 700.0
/// (`010300a8`).
pub fn fn_0056b250(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if e.call(0x0056_4e60, &args![this]).bool() {
        return true;
    }
    if e.vcall(this.addr(), SLOT_IS_ACTOR, &args![]).bool() {
        return false;
    }
    if e.call(0x0044_4ed0, &args![this]).bool() {
        return false;
    }
    if base_form(e, this.addr()) == 0 {
        return false;
    }
    let base = base_form(e, this.addr());
    let size = e.call(0x0050_ebf0, &args![base]).f32() as f64;
    size > e.global::<f64>(0x0103_00a8)
}

// Translated from 0056c760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x8000000` of the dword at `this + 8` (`TESForm::iFormFlags`)
/// is set.
pub fn fn_0056c760(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    // TESForm::iFormFlags (Xbox PDB) +0x08
    e.mem.u32(this.addr() + 8) & 0x0800_0000 != 0
}

// Translated from 0056c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit `0x8000000` of the dword at
/// `this + 8` (`TESForm::iFormFlags`).
pub fn fn_0056c780(e: &mut Engine, this: Ptr<TESObjectREFR>, flag: u8) {
    // TESForm::iFormFlags (Xbox PDB) +0x08
    let flags = e.mem.u32(this.addr() + 8);
    let flags = if flag != 0 {
        flags | 0x0800_0000
    } else {
        flags & 0xf7ff_ffff
    };
    e.mem.set_u32(this.addr() + 8, flags);
}

// Translated from 0056c7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the dword at `this + 0xcc`. `Load3D` calls it on a
/// reference's 3D root that `IsFadeNode` accepted, with the reference
/// itself as the value.
pub fn fn_0056c7d0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xcc, value);
}

// Translated from 0056c7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first dword of the `NiPointer` embedded at `this + 0x2c`
/// (`00559450`).
pub fn fn_0056c7f0(e: &mut Engine, this: Ptr) -> u32 {
    ni_pointer_get(e, this.addr() + 0x2c)
}

// Translated from 0056c810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundRoom::AddJoinedMultiBound` (Xbox PDB): appends a
/// `NiPointer` to `multi_bound` to the list embedded at `this + 0xd0` (the
/// PC offset of `kJoinedMultiBoundList`; `0057c590` is the list's
/// `AddTail`). The temporary `NiPointer` is a 4-byte local, built with
/// `00633c90` and destroyed with `0045cec0`. The compiler's
/// exception-unwinding frame is not translated.
pub fn bs_multi_bound_room_add_joined_multi_bound(e: &mut Engine, this: Ptr, multi_bound: u32) {
    let pointer = e.mem.alloc(4);
    e.call(NI_POINTER_CONSTRUCT, &args![pointer, multi_bound]);
    e.call(0x0057_c590, &args![this.addr() + 0xd0, pointer]);
    e.call(NI_POINTER_DESTRUCT, &args![pointer]);
    e.mem.free(pointer);
}

// Translated from 0056c880 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the reference's `pLoadedData` exists, sets (`set` non-zero) or
/// clears the bits `mask` of its `iFlags` ([`fn_0056c8b0`]).
pub fn fn_0056c880(e: &mut Engine, this: Ptr<TESObjectREFR>, mask: u32, set: u8) {
    let loaded = e.get(this, TESObjectREFR::pLoadedData);
    if loaded.addr() != 0 {
        fn_0056c8b0(e, loaded.cast(), mask, set);
    }
}

// Translated from 0056c8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LOADED_REF_DATA` flag setter: sets (`set` non-zero) or clears the bits
/// `mask` of `iFlags` (+0x10).
pub fn fn_0056c8b0(e: &mut Engine, this: Ptr<LOADED_REF_DATA>, mask: u32, set: u8) {
    // LOADED_REF_DATA::iFlags (Xbox PDB) +0x10
    let flags = e.mem.u32(this.addr() + 0x10);
    let flags = if set != 0 {
        flags | mask
    } else {
        !mask & flags
    };
    e.mem.set_u32(this.addr() + 0x10, flags);
}

// Translated from 0056d230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::FromRotation` (`00a6df40`) of the 3x3 rotation at
/// `this + 0x34` (`NiAVObject::m_kLocal.m_Rotate` on the PC) into `out`.
pub fn fn_0056d230(e: &mut Engine, this: Ptr, out: Ptr) {
    e.call(0x00a6_df40, &args![out, this.addr() + 0x34]);
}

/// `Load3D`: the global `011dea10` that several of its callees run on, and
/// the one it passes to `0042ce10` (`011ddf38`).
const GLOBAL_TES: u32 = 0x011d_ea10;
const GLOBAL_LOADING_STATE: u32 = 0x011d_df38;
/// `Load3D` compares the base object with these globals (`011ca240` before
/// the loaded data is built, `011ca220` and `011ca250` afterwards).
const GLOBAL_BASE_FORM_C: u32 = 0x011c_a240;
const GLOBAL_BASE_FORM_A: u32 = 0x011c_a220;
const GLOBAL_BASE_FORM_B: u32 = 0x011c_a250;
/// RTTI descriptors `Load3D` casts the reference's model with
/// (`.?AVTESModel@@`, `.?AVTESModelTextureSwap@@`).
const TYPE_TES_MODEL: u32 = 0x0118_31e8;
const TYPE_TES_MODEL_TEXTURE_SWAP: u32 = 0x0118_31c4;
/// `BSShaderProperty::SetFlagRecurse` (`00ba91a0`).
const SET_FLAG_RECURSE: u32 = 0x00ba_91a0;
/// `ShadowSceneNode` finder (`00450b80`, cdecl, one argument) and
/// `ShadowSceneNode::RemoveLight` (`00b5eed0`).
const FIND_SHADOW_SCENE_NODE: u32 = 0x0045_0b80;
const SHADOW_SCENE_REMOVE_LIGHT: u32 = 0x00b5_eed0;
/// `"WATER: Trying to load water object '%s' (%08X) without multibound
/// data. Please have an artist re-export this object ASAP!"` and the log
/// call's first argument (`01019f08`).
const WATER_WITHOUT_MULTIBOUND_FORMAT: u32 = 0x0103_00b0;
const WATER_LOG: u32 = 0x0101_9f08;

/// The tail of `Load3D`'s two loops over lit-water and water-light
/// references, which take a light off the shadow scene: looks the
/// reference's light record up (`0070ec90` on the global, `004e8030`), and
/// when its owner (`004030b0`) reports kind `0xd` (`008d8520`), registers the
/// 3D root held by the `NiPointer` at `light_slot` in the record's
/// property (`NiAVObject::GetProperty(3)`, with a set at `+0x128` and a flag
/// byte at `+0x83`) and removes the light from the shadow scene.
fn unlink_shadow_light(e: &mut Engine, light_slot: u32, reference: u32) {
    let table = e.global::<u32>(GLOBAL_TES);
    let finder = e.call(0x0070_ec90, &args![table]).u32();
    let record = e.call(0x004e_8030, &args![finder, reference]).u32();
    let owner = e.call(0x0040_30b0, &args![record]).u32();
    if owner == 0 || e.call(0x008d_8520, &args![owner]).u32() != 0xd {
        return;
    }
    let property = e.call(0x00a5_9d30, &args![record, 3u32]).u32();
    let set = property + 0x128;
    let key = e.mem.alloc(4);
    let node = ni_pointer_get(e, light_slot);
    e.mem.set_u32(key, node);
    if e.call(0x0049_c680, &args![set, key, 0u32]).u32() == 0 {
        let node = ni_pointer_get(e, light_slot);
        e.mem.set_u32(key, node);
        e.call(0x004e_d8c0, &args![set, key]);
        e.mem.set_u8(property + 0x83, 1);
    }
    e.mem.free(key);
    let node = ni_pointer_get(e, light_slot);
    let scene = e.call(FIND_SHADOW_SCENE_NODE, &args![0u32]).u32();
    e.call(SHADOW_SCENE_REMOVE_LIGHT, &args![scene, node]);
}

// Translated from 0056b2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Load3D` (Xbox PDB): loads (or re-attaches) the
/// reference's 3D and returns the 3D root (0 when the reference is
/// disabled or `00440d80` holds). `flag` is the byte the game passes to
/// `00570f70` or `00441780`/`004417c0` and does not otherwise interpret.
///
/// In order: the early exits; the water-texture touch for form type `0x23`;
/// `CreateLoadedData` (`0057c180`); the model (`0043fcd0`) into a
/// `NiPointer` local; the body (only when the base object exists and either
/// the 3D is missing or the extra data list has a primitive): the primitive
/// or the base object's virtual `+0x178` loads the 3D, the loaded-data
/// flags, destructible, texture-swap, addon-node, orientation, bound and
/// Havok setup, the actor animation and ragdoll steps; then, for every
/// reference, the nav mesh obstacle, shader flags, lit-water and
/// water-light loops, the multibound room join and the missing-multibound
/// log message for water, the Securitron face and `0055d6d0`.
///
/// The compiler's exception-unwinding frame and the stack-protector cookie
/// are not translated. A few results the game stores in locals it never
/// reads again are still computed (their callees are getters).
pub fn tes_object_refr_load_3d(e: &mut Engine, this: Ptr<TESObjectREFR>, flag: u8) -> Ptr {
    let me = this.addr();
    let disabled = e.call(0x0044_0da0, &args![me]).bool();
    let state = e.global::<u32>(GLOBAL_LOADING_STATE);
    let loading = e.call(0x0042_ce10, &args![state]).bool();
    if e.call(0x0044_0d80, &args![me]).bool() || disabled {
        return Ptr::NULL;
    }

    if base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x23 {
            let tes = e.global::<u32>(GLOBAL_TES);
            e.call(0x0045_ce40, &args![tes]);
            if e.call(GET_PARENT_CELL, &args![me]).u32() != 0 {
                let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
                let holder = e.call(0x0045_43c0, &args![cell]).u32();
                if e.call(0x0059_bb30, &args![holder]).u32() != 0 {
                    let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
                    let holder = e.call(0x0045_43c0, &args![cell]).u32();
                    let inner = e.call(0x0059_bb30, &args![holder]).u32();
                    e.call(0x0093_7090, &args![inner, 1u32]);
                }
            }
        }
    }

    e.call(0x0057_c180, &args![me]);
    let model = e.call(0x0043_fcd0, &args![me]).u32();
    let mut locals = Locals::default();
    // the `NiPointer` the 3D root lives in
    let slot = locals.alloc(e, 4);
    e.call(NI_POINTER_CONSTRUCT, &args![slot, model]);
    let room_check = e.call(0x0082_2510, &args![slot, 0u32]).u8();

    let node = ni_pointer_get(e, slot);
    let run_body = if node == 0 {
        true
    } else {
        let list = extra_list(e, me);
        e.call(0x0041_fbe0, &args![list]).u32() != 0
    };
    let entry_base = base_object_field(e, this);
    if run_body && entry_base != 0 {
        let list = extra_list(e, me);
        let primitive = e.call(0x0041_fbe0, &args![list]).u32();
        if primitive != 0 {
            e.vcall(primitive, 4, &args![]);
            let held = fn_0056c7f0(e, Ptr::new(primitive));
            e.call(NI_POINTER_ASSIGN, &args![slot, held]);
            if e.call(0x004b_9930, &args![0x011c_a2dcu32]).bool() {
                // the primitive's class ID (`00413f90` copies 16 bytes), whose
                // last dword the game clears before the virtual `+8` call
                let class_id = locals.alloc(e, 16);
                e.call(0x0041_3f90, &args![primitive, class_id]);
                e.mem.set_f32(class_id + 12, 0.0);
                e.vcall(primitive, 8, &args![class_id]);
            }
            if base_form(e, me) != e.global::<u32>(GLOBAL_BASE_FORM_C)
                && ni_pointer_get(e, slot) != 0
            {
                let node = ni_pointer_get(e, slot);
                e.call(0x0045_0f90, &args![node, 1u32]);
            }
            fn_0056c880(e, this, 4, 1);
        } else {
            let base = base_object_field(e, this);
            let loaded = e.vcall(base, 0x178, &args![me]).u32();
            e.call(NI_POINTER_ASSIGN, &args![slot, loaded]);
        }

        if flag == 0 {
            let node = ni_pointer_get(e, slot);
            e.call(0x0057_0f70, &args![me, node]);
        } else {
            let node = ni_pointer_get(e, slot);
            e.call(0x0044_1780, &args![me, node]);
            if ni_pointer_get(e, slot) != 0 {
                let node = ni_pointer_get(e, slot);
                e.call(0x0040_f6e0, &args![node]);
            }
        }

        let mut addon = 0u8;
        if ni_pointer_get(e, slot) != 0 {
            let node = ni_pointer_get(e, slot);
            let as_node = e.vcall(node, SLOT_IS_NODE, &args![]).u32();
            addon = e.call(0x0057_84b0, &args![as_node]).u8();
            fn_0056c880(e, this, 2, addon);
        }
        if fn_0056c760(e, this) {
            let node = ni_pointer_get(e, slot);
            e.call(0x0047_6ab0, &args![node]);
            fn_0056c780(e, this, 0);
        }

        if e.call(0x0045_2370, &args![me]).bool() && ni_pointer_get(e, slot) != 0 {
            let list = extra_list(e, me);
            let health = e.call(0x0041_b6b0, &args![list]).f32() as f64;
            if health != e.global::<f64>(0x0101_2060) {
                let first = base_form(e, me);
                let second = base_form(e, me);
                let destruction = e.call(0x0047_5400, &args![second]).u32();
                e.call(0x0047_7780, &args![destruction, first]);
            }
            if e.call(0x0050_8d70, &args![me]).bool() {
                let node = ni_pointer_get(e, slot);
                if e.call(0x0047_7560, &args![node]).bool() {
                    let list = extra_list(e, me);
                    e.call(0x0042_2c20, &args![list]);
                }
            }
            let base = base_form(e, me);
            let destruction = e.call(0x0047_5400, &args![base]).u32();
            if e.call(0x0047_6f50, &args![destruction, me, 0u32]).bool() {
                e.call(0x0057_a3c0, &args![me, 0u32]);
            }
        }

        let model = e.call(0x0057_1630, &args![me]).u32();
        let swap = dynamic_cast(e, model, TYPE_TES_MODEL, TYPE_TES_MODEL_TEXTURE_SWAP);
        if swap != 0 && ni_pointer_get(e, slot) != 0 {
            let node = ni_pointer_get(e, slot);
            let as_node = e.vcall(node, SLOT_IS_NODE, &args![]).u32();
            e.call(0x0048_afe0, &args![swap, as_node]);
        }
        if base_form(e, me) != 0 {
            let base = base_form(e, me);
            if e.vcall(base, 0xac, &args![]).bool() {
                let node = ni_pointer_get(e, slot);
                e.call(0x004b_7660, &args![node]);
            }
        }

        if ni_pointer_get(e, slot) != 0 {
            // the reference as a creature (form type 0x2b), 0 otherwise
            let mut creature = 0u32;
            if e.vcall(me, SLOT_IS_ACTOR, &args![]).bool() {
                let base = base_object_field(e, this);
                if form_type(e, base) == 0x2b {
                    creature = me;
                    let node = ni_pointer_get(e, slot);
                    let flag = e.call(0x008d_4ab0, &args![creature, node]).u8();
                    e.vcall(creature, 0x388, &args![flag]);
                }
            }
            if addon != 0 {
                if !e.call(0x0044_8a20, &args![me]).bool() {
                    let node = ni_pointer_get(e, slot);
                    e.call(0x0057_7e20, &args![node]);
                } else {
                    let node = ni_pointer_get(e, slot);
                    e.call(0x0057_8300, &args![node]);
                }
            }
            if e.vcall(me, 0x1d4, &args![]).bool() {
                let value = e.call(0x0048_7f50, &args![]).u32();
                let scaled = ((value % 1000) as f64 / e.global::<f64>(0x0101_7a40)) as f32;
                let node = ni_pointer_get(e, slot);
                let as_node = e.vcall(node, SLOT_IS_NODE, &args![]).u32();
                e.call(0x0057_84f0, &args![as_node, scaled]);
            }
            if form_type(e, entry_base) == 0x28 {
                let node = ni_pointer_get(e, slot);
                e.call(0x004b_5b20, &args![node]);
            }
            let motion = e.vcall(me, 0x150, &args![]).u8();
            let node = ni_pointer_get(e, slot);
            e.call(0x00ba_9120, &args![node, motion]);
            let node = ni_pointer_get(e, slot);
            e.call(0x0044_0460, &args![node, me + 0x30]);
            let orientation = locals.alloc(e, 0x24);
            let rotation = e.call(0x0056_fa00, &args![me, orientation]).u32();
            let node = ni_pointer_get(e, slot);
            e.call(0x0043_fa80, &args![node, rotation]);

            if e.call(0x0056_4cd0, &args![me]).bool() {
                let node = ni_pointer_get(e, slot);
                e.call(0x00c6_a040, &args![node, 0u32, 1u32, 1u32]);
                let origin = locals.alloc(e, 12);
                e.call(0x0043_d410, &args![origin, 0.0f32, 0u32, 0u32]);
                let node = ni_pointer_get(e, slot);
                e.call(0x00a5_9c60, &args![node, origin]);
                let origin = locals.alloc(e, 12);
                e.call(0x0043_d410, &args![origin, 0.0f32, 0u32, 0u32]);
                let node = ni_pointer_get(e, slot);
                e.call(0x00a5_9c60, &args![node, origin]);
                let node = ni_pointer_get(e, slot);
                e.call(0x00c6_a040, &args![node, 1u32, 1u32, 1u32]);
                let node = ni_pointer_get(e, slot);
                let bound = e.call(0x0043_d450, &args![node]).u32();
                let center = e.call(LIST_ITEM_SLOT, &args![bound]).u32();
                let bounds = locals.alloc(e, 12);
                e.call(0x0043_9ef0, &args![me + 0x30, bounds, center]);
                e.call(0x0063_c8a0, &args![me + 0x30, bounds]);
                let node = ni_pointer_get(e, slot);
                e.call(0x0044_0460, &args![node, me + 0x30]);
            }
            let node = ni_pointer_get(e, slot);
            e.call(0x00c6_bd00, &args![node, 1u32]);
            let node = ni_pointer_get(e, slot);
            let as_node = e.vcall(node, SLOT_IS_NODE, &args![]).u32();
            if as_node != 0 && e.vcall(as_node, SLOT_IS_FADE_NODE, &args![]).u32() != 0 {
                let node = ni_pointer_get(e, slot);
                fn_0056c7d0(e, Ptr::new(node), me);
            }
            let node = ni_pointer_get(e, slot);
            e.call(0x00b5_5b80, &args![node]);
            let base = base_object_field(e, this);
            if base == e.global::<u32>(GLOBAL_BASE_FORM_A)
                || base_object_field(e, this) == e.global::<u32>(GLOBAL_BASE_FORM_B)
            {
                let node = ni_pointer_get(e, slot);
                e.call(0x0045_0f90, &args![node, 1u32]);
            }
            // The game keeps the base object of a type 0x2a form in a local
            // it never reads again.
            if base_form(e, me) != 0 {
                let base = base_form(e, me);
                if form_type(e, base) == 0x2a {
                    base_form(e, me);
                }
            }
            if e.call(0x0056_8e50, &args![me]).u32() != 0 {
                let door = e.call(0x0056_8e50, &args![me]).u32();
                if e.call(0x0043_a2b0, &args![door]).u32() != 0 {
                    let door = e.call(0x0056_8e50, &args![me]).u32();
                    let cell = e.call(0x0043_a2b0, &args![door]).u32();
                    e.call(0x0055_1480, &args![cell, 1u32]);
                }
            }
            let scale = e.call(0x0056_7400, &args![me]).f32();
            let node = ni_pointer_get(e, slot);
            e.call(0x0044_0490, &args![node, scale]);
            e.call(0x0056_59f0, &args![me]);

            if e.vcall(me, SLOT_IS_ACTOR, &args![]).bool() {
                let base = base_object_field(e, this);
                if form_type(e, base) == 0x2a {
                    let package = e.vcall(me, 0x1b8, &args![0u32]).u32();
                    if package != 0 {
                        // 0056bbc2: slot 0x22c takes one word (0); the pushed 1
                        // is the second word of the call through +0xd8.
                        let flag = e.vcall(me, 0x22c, &args![0u32]).u8();
                        e.vcall(package, 0xd8, &args![flag, 1u32]);
                    }
                    let base = base_object_field(e, this);
                    e.call(0x0060_56f0, &args![base, me]);
                }
            }
            let node = ni_pointer_get(e, slot);
            e.call(0x00a5_a040, &args![node]);

            if e.vcall(me, SLOT_IS_ACTOR, &args![]).bool() {
                let actor = me;
                let mut acquired = 0u32;
                let node = ni_pointer_get(e, slot);
                e.call(0x008b_0bd0, &args![actor, node]);
                if e.call(0x008d_8520, &args![actor]).u32() != 0 {
                    let object = e.call(0x008d_8520, &args![actor]).u32();
                    if e.call(0x0045_cd60, &args![object]).u32() == 0 {
                        acquired = e.call(0x008d_8520, &args![actor]).u32();
                    }
                }
                if acquired != 0 {
                    e.call(0x008d_a1a0, &args![acquired, actor]);
                    e.vcall(acquired, 0x58, &args![]);
                }
                if creature != 0 {
                    e.call(0x008d_4b60, &args![creature]);
                }
                let tes = e.global::<u32>(GLOBAL_TES);
                if !e.call(0x0045_1530, &args![tes]).bool()
                    && e.vcall(me, 0x22c, &args![1u32]).bool()
                    && e.vcall(me, 0x1e4, &args![]).u32() != 0
                {
                    let animation = e.vcall(me, 0x1e4, &args![]).u32();
                    if e.call(0x0049_4710, &args![animation, 0xe0u32]).bool() && !loading {
                        let animation = e.vcall(me, 0x1e4, &args![]).u32();
                        e.call(
                            0x0049_55c0,
                            &args![
                                animation,
                                0x14u32,
                                0xe0u32,
                                0xffff_ffffu32,
                                0.0f32,
                                0xffff_ffffu32
                            ],
                        );
                        let section = e.call(0x0049_1040, &args![animation, 1u32]).u32();
                        if section != 0 {
                            if !e.vcall(actor, 0x38c, &args![]).bool() {
                                let time = e.call(0x0050_8100, &args![section]).f32();
                                e.call(0x0098_adb0, &args![section, time]);
                                let time = e.call(0x0050_8100, &args![section]).f32();
                                e.call(0x0049_1180, &args![animation, me, 0.0f32, time]);
                                e.call(0x0049_3900, &args![animation, me]);
                            } else if !e.call(0x0057_22c0, &args![me, 0u32]).bool() {
                                let group = e.call(0x0048_f7f0, &args![section]).u32();
                                let time = e.call(0x005f_3780, &args![group, 1u32]).f32();
                                e.call(0x0049_1180, &args![animation, me, 0.0f32, time]);
                                e.call(0x0049_3900, &args![animation, me]);
                                // the knock-down direction: a rotation about the
                                // reference's heading applied to (0, 1, 0)
                                let rotation = locals.alloc(e, 0x24);
                                e.call(LIST_ITEM_SLOT, &args![rotation]);
                                let heading = e.call(0x0043_0830, &args![me]).u32();
                                let angle = e.mem.f32(heading + 8);
                                e.call(0x004a_0c90, &args![rotation, angle]);
                                let direction = locals.alloc(e, 12);
                                e.call(0x0041_6870, &args![direction, 0.0f32, 1.0f32, 0.0f32]);
                                let rotated = locals.alloc(e, 12);
                                let result = e
                                    .call(0x004b_4500, &args![rotation, rotated, direction])
                                    .u32();
                                for word in 0..3 {
                                    let value = e.mem.u32(result + 4 * word);
                                    e.mem.set_u32(direction + 4 * word, value);
                                }
                                let ragdoll = e.mem.u32(actor + 0xac);
                                if ragdoll != 0 {
                                    e.call(0x00c7_c150, &args![ragdoll, 1u32]);
                                }
                                let node = ni_pointer_get(e, slot);
                                e.call(0x00c9_b670, &args![node, direction, 1u32, 0.0f32, 1u32]);
                                e.vcall(me, SLOT_SET_CHANGED, &args![4u32]);
                            }
                        }
                    }
                }
            }

            let node = ni_pointer_get(e, slot);
            if e.call(0x0062_c3c0, &args![me, node]).bool() {
                e.call(0x0057_7330, &args![me, 0u32]);
            }
            let list = extra_list(e, me);
            e.call(0x0042_2a40, &args![list, me]);
            e.call(0x0056_a4a0, &args![me, 0u32]);
            let node = ni_pointer_get(e, slot);
            e.call(0x0057_6d50, &args![me, node]);
        }
    }

    if e.vcall(me, 0x15c, &args![]).bool() {
        let manager = e.call(0x006c_0720, &args![]).u32();
        e.call(0x006c_0c30, &args![manager, me]);
    }
    if e.vcall(me, 0xb8, &args![]).bool() {
        let node = ni_pointer_get(e, slot);
        e.call(SET_FLAG_RECURSE, &args![node, 0x32u32, 1u32]);
    }
    if e.call(0x0056_5050, &args![me]).bool() {
        let node = ni_pointer_get(e, slot);
        e.call(SET_FLAG_RECURSE, &args![node, 0x37u32, 1u32]);
    }
    if e.call(0x0056_4f00, &args![me]).bool() {
        let node = ni_pointer_get(e, slot);
        e.call(SET_FLAG_RECURSE, &args![node, 0x22u32, 1u32]);
        if !e.call(0x004f_2640, &args![me]).bool() {
            e.call(0x004f_24e0, &args![me, 0u32, 1u32]);
        } else {
            e.call(0x004f_24e0, &args![me, 1u32, 0u32]);
        }
    }

    if base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x1e {
            let sound = locals.alloc(e, 12);
            e.call(0x0041_a250, &args![sound]);
            let list = extra_list(e, me);
            e.call(0x0041_8890, &args![list, sound]);
            if e.call(0x00ad_8ce0, &args![sound]).bool() {
                let node = ni_pointer_get(e, slot);
                e.call(0x00ad_8f20, &args![sound, node]);
            }
            let list = extra_list(e, me);
            let mut lit = e.call(0x0041_f810, &args![list]).u32();
            let light_slot = e.call(0x0057_25f0, &args![me]).u32();
            if light_slot != 0
                && ni_pointer_get(e, light_slot) != 0
                && !e.call(0x0040_77c0, &args![me]).bool()
            {
                while lit != 0 {
                    let item = e.call(LIST_ITEM_SLOT, &args![lit]).u32();
                    if e.mem.u32(item) == 0 {
                        break;
                    }
                    let item = e.call(LIST_ITEM_SLOT, &args![lit]).u32();
                    let light_ref = e.mem.u32(item);
                    if e.vcall(light_ref, SLOT_GET_3D, &args![]).u32() != 0 {
                        // the game fetches the 3D root a second time into a
                        // local it never reads
                        e.vcall(light_ref, SLOT_GET_3D, &args![]);
                        unlink_shadow_light(e, light_slot, light_ref);
                    }
                    lit = e.call(LIST_NEXT, &args![lit]).u32();
                }
            }
            e.call(0x0048_3710, &args![sound]);
        }
    }

    if base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x23 {
            let list = extra_list(e, me);
            let mut lit = e.call(0x0041_f7e0, &args![list]).u32();
            while lit != 0 {
                let item = e.call(LIST_ITEM_SLOT, &args![lit]).u32();
                if e.mem.u32(item) == 0 {
                    break;
                }
                let item = e.call(LIST_ITEM_SLOT, &args![lit]).u32();
                let light_ref = e.mem.u32(item);
                let light_slot = e.call(0x0057_25f0, &args![light_ref]).u32();
                if light_slot != 0
                    && ni_pointer_get(e, light_slot) != 0
                    && !e.call(0x0040_77c0, &args![me]).bool()
                    && e.vcall(me, SLOT_GET_3D, &args![]).u32() != 0
                {
                    unlink_shadow_light(e, light_slot, me);
                }
                lit = e.call(LIST_NEXT, &args![lit]).u32();
            }
        }
    }

    if room_check != 0 {
        let list = extra_list(e, me);
        if !e.call(0x0042_0810, &args![list]).bool() {
            let list = extra_list(e, me);
            let master = e.call(0x0042_08b0, &args![list]).u32();
            if master != 0 {
                let multi_bound = e.call(0x0056_9920, &args![me]).u32();
                let pointer = e.mem.alloc(4);
                e.call(NI_POINTER_CONSTRUCT, &args![pointer, multi_bound]);
                let room = e.call(0x0056_99b0, &args![master]).u32();
                let room = e.call(0x0045_c650, &args![room]).u32();
                let joined = e.call(0x0057_c6a0, &args![room, pointer, 0u32]).u32() != 0;
                e.call(NI_POINTER_DESTRUCT, &args![pointer]);
                e.mem.free(pointer);
                if !joined {
                    let multi_bound = e.call(0x0056_9920, &args![me]).u32();
                    let room = e.call(0x0056_99b0, &args![master]).u32();
                    bs_multi_bound_room_add_joined_multi_bound(e, Ptr::new(room), multi_bound);
                }
            }
        }
    }

    if flag != 0 {
        e.call(0x0044_17c0, &args![]);
    }

    if room_check != 0 && base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x23 {
            let base = base_form(e, me);
            if !e.call(0x0045_2440, &args![base]).bool() {
                let node = ni_pointer_get(e, slot);
                let found = e.call(0x0057_bd80, &args![me, node]).u8();
                let node = ni_pointer_get(e, slot);
                e.vcall(node, 0xbc, &args![]);
                if found == 0 {
                    let id = e.call(FORM_ID, &args![me]).u32();
                    let name = e.vcall(me, 0x130, &args![]).u32();
                    let text = locals.alloc(e, 0x100);
                    e.call(
                        0x0040_6d00,
                        &args![text, 0x100u32, WATER_WITHOUT_MULTIBOUND_FORMAT, name, id],
                    );
                    e.call(0x005b_5e40, &args![WATER_LOG, text]);
                }
            }
        }
    }

    let face = e.call(0x0052_7080, &args![me, 0x8fu32]).u32();
    if face != 0 {
        let node = ni_pointer_get(e, slot);
        e.call(0x0043_7f90, &args![face, node]);
    }
    let node = ni_pointer_get(e, slot);
    e.call(0x0055_d6d0, &args![me, node]);
    let root = ni_pointer_get(e, slot);
    e.call(NI_POINTER_DESTRUCT, &args![slot]);
    locals.release(e);
    Ptr::new(root)
}

layout! {
    /// `bhkRigidBodyCinfo` (Xbox PDB), 0xe0 bytes, only the fields used so
    /// far: `hkWorldObjectCinfo::m_collisionFilterInfo` (+0) and `m_shape`
    /// (+4).
    pub struct BhkRigidBodyCinfo: 0xe0 {
        /// `m_collisionFilterInfo` (Xbox PDB).
        0x00 m_collisionFilterInfo: u32,
        /// `m_shape` (Xbox PDB): `const hkpShape*`, here the `bhk` shape
        /// wrapper `InitHavokForPlaceableWater` builds.
        0x04 m_shape: Ptr,
    }
}

/// Collision filter info the placeable-water rigid bodies get
/// (`0x50000 | 0xb`).
const WATER_COLLISION_FILTER_INFO: u32 = 0x0005_000b;

/// The `bhkRigidBody` and `bhkNiCollisionObject` that
/// `InitHavokForPlaceableWater` makes from a filled-in `cinfo`: builds the
/// body from `cinfo`, a collision object around the reference's 3D root,
/// makes sure the root carries a `BSXFlags` extra data (set to `2, 1`) and
/// links them. Returns the body.
fn build_water_body(e: &mut Engine, me: u32, cinfo: u32) -> u32 {
    let raw = e.call(0x00aa_13e0, &args![0x1cu32]).u32();
    let body = if raw != 0 {
        e.call(0x0056_d380, &args![raw, cinfo]).u32()
    } else {
        0
    };
    let raw = e.call(0x00aa_13e0, &args![0x14u32]).u32();
    let collision = if raw != 0 {
        let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
        e.call(0x0056_d580, &args![raw, node]).u32()
    } else {
        0
    };
    let extra_name = e.call(0x0044_8a80, &args![]).u32();
    let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
    let mut flags = e.call(0x00a5_bdd0, &args![node, extra_name]).u32();
    if flags == 0 {
        let raw = e.call(0x00aa_13e0, &args![0x10u32]).u32();
        flags = if raw != 0 {
            e.call(0x00c4_2f70, &args![raw]).u32()
        } else {
            0
        };
        let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
        e.call(0x00a5_bca0, &args![node, flags]);
    }
    e.call(0x0051_9230, &args![flags, 2u32, 1u32]);
    e.call(0x00c6_b980, &args![collision, body]);
    let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
    e.call(0x00c8_5c10, &args![body, node, 0u32]);
    body
}

// Translated from 0056c8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::InitHavokForPlaceableWater` (Xbox PDB): gives a
/// placeable water reference a static Havok rigid body, so the world's
/// collision sees the water volume. Does nothing without a parent cell, a
/// loaded-cell holder (`004543c0` of the cell) or a base object.
///
/// When the base object is the kind `00452440` accepts, the body is a box
/// shape (`hkpBoxShape`, half extents 2048, 2048, 500) placed at the cell's
/// centre (`GetDataX`/`GetDataY` times 4096 plus 2048) at the cell's water
/// height minus 500, and the body is also flagged through virtual `+0xe4`.
/// Otherwise the shape is the one of the reference's own collision object
/// (`004a8b00` of the 3D root, through `006fa820`/`0043b560`), placed at the
/// root's local rotation (`0056d230`) and the reference's position (virtual
/// `+0x1f4`); a missing collision object or shape is logged, and a shape whose
/// word at +0xc (`0043b230`) is `0x1b` is skipped without a message. Both finish with the body linked to
/// a new `bhkNiCollisionObject` (`0056d580`), the `BSXFlags` extra data of the
/// root, and `0056d2c0` on the cell holder.
///
/// The temporaries are blocks of the engine's memory: a 0xe0-byte
/// `bhkRigidBodyCinfo` (`00c8f510`, destroyed with `0056d730`) and 12- and
/// 16-byte vectors. The compiler's exception-unwinding frame and the
/// 16-byte stack alignment of the prologue are not translated.
pub fn tes_object_refr_init_havok_for_placeable_water(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let holder = if e.call(GET_PARENT_CELL, &args![me]).u32() != 0 {
        let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    if holder == 0 || base_form(e, me) == 0 {
        return;
    }
    let base = base_form(e, me);
    let mut locals = Locals::default();
    if e.call(0x0045_2440, &args![base]).bool() {
        // a box shape under the water
        let bounds_min = locals.alloc(e, 12);
        e.vcall(me, 0x1d8, &args![bounds_min]);
        let bounds_max = locals.alloc(e, 12);
        e.vcall(me, 0x1dc, &args![bounds_max]);
        let extents = locals.alloc(e, 12);
        let half_width = e.global::<f32>(0x0101_8bfc);
        let half_height = e.global::<f32>(0x0101_3d84);
        e.call(
            0x0041_6870,
            &args![extents, half_width, half_width, half_height],
        );
        let extents_vector = locals.alloc(e, 16);
        e.call(LIST_ITEM_SLOT, &args![extents_vector]);
        e.call(0x004a_3e00, &args![extents_vector, extents]);
        let raw = e.call(0x0056_d280, &args![0x30u32]).u32();
        // 0056ca11: FLD dword / FSTP dword, which quiets a signalling NaN.
        let radius = x87_quiet(e.global::<f32>(0x011b_153c));
        let box_shape = if raw != 0 {
            e.call(0x00c9_ddb0, &args![raw, extents_vector, radius])
                .u32()
        } else {
            0
        };
        let raw = e.call(0x0056_d280, &args![0x10u32]).u32();
        let material = if raw != 0 {
            e.call(0x0056_d5f0, &args![raw]).u32()
        } else {
            0
        };
        let raw = e.call(0x0056_d280, &args![0x1cu32]).u32();
        let shape = if raw != 0 {
            e.call(0x00c9_d990, &args![raw, box_shape, material]).u32()
        } else {
            0
        };
        let cinfo = locals.alloc(e, BhkRigidBodyCinfo::SIZE);
        e.call(0x00c8_f510, &args![cinfo]);
        let cinfo_ptr = Ptr::<BhkRigidBodyCinfo>::new(cinfo);
        e.set(cinfo_ptr, BhkRigidBodyCinfo::m_shape, Ptr::<()>::new(shape));
        e.call(0x0056_d360, &args![cinfo, 5u32]);
        let position_vector = locals.alloc(e, 16);
        e.call(LIST_ITEM_SLOT, &args![position_vector]);
        let rotation = locals.alloc(e, 16);
        e.call(0x0062_40d0, &args![rotation]);
        e.call(0x0056_d250, &args![rotation]);
        let position = locals.alloc(e, 12);
        e.call(LIST_ITEM_SLOT, &args![position]);
        let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        let cell_x = e.call(0x0054_4c30, &args![cell]).i32();
        let x = cell_x.wrapping_shl(12).wrapping_add(0x800);
        e.mem.set_f32(position, x as f32);
        let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        let cell_y = e.call(0x0054_4c60, &args![cell]).i32();
        let y = cell_y.wrapping_shl(12).wrapping_add(0x800);
        e.mem.set_f32(position + 4, y as f32);
        let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        let water_height = e.call(0x0054_71e0, &args![cell]).f64();
        // 0056cba2: FSUB qword at the control word's precision and rounding,
        // then FSTP dword.
        let z = x87_sub(water_height, e.global::<f64>(0x0103_01a8));
        e.mem.set_u32(position + 8, x87_store32(z));
        e.call(0x004a_3e00, &args![position_vector, position]);
        e.call(0x0056_d300, &args![cinfo, position_vector]);
        e.call(0x0056_d320, &args![cinfo, rotation]);
        e.set(
            cinfo_ptr,
            BhkRigidBodyCinfo::m_collisionFilterInfo,
            WATER_COLLISION_FILTER_INFO,
        );
        let body = build_water_body(e, me, cinfo);
        e.vcall(body, 0xe4, &args![1u32]);
        e.call(0x0056_d2c0, &args![holder, body]);
        e.call(0x0056_d730, &args![cinfo]);
    } else {
        let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
        let collision = e.call(0x004a_8b00, &args![node]).u32();
        if collision == 0 {
            let id = e.call(FORM_ID, &args![me]).u32();
            e.call(0x005b_5e40, &args![0x0103_016cu32, id]);
            locals.release(e);
            return;
        }
        let rigid = e.call(0x006f_a820, &args![collision]).u32();
        let shape_slot = e.call(0x0043_b560, &args![rigid]).u32();
        let source_shape = ni_pointer_get(e, shape_slot);
        if source_shape == 0 {
            let id = e.call(FORM_ID, &args![me]).u32();
            e.call(0x005b_5e40, &args![0x0103_012cu32, id]);
            locals.release(e);
            return;
        }
        if e.call(0x0043_b230, &args![source_shape]).u32() == 0x1b {
            locals.release(e);
            return;
        }
        let raw = e.call(0x0056_d280, &args![0x10u32]).u32();
        let material = if raw != 0 {
            e.call(0x0056_d5f0, &args![raw]).u32()
        } else {
            0
        };
        let raw = e.call(0x0056_d280, &args![0x1cu32]).u32();
        let shape = if raw != 0 {
            e.call(0x00c9_d990, &args![raw, source_shape, material])
                .u32()
        } else {
            0
        };
        let cinfo = locals.alloc(e, BhkRigidBodyCinfo::SIZE);
        e.call(0x00c8_f510, &args![cinfo]);
        let cinfo_ptr = Ptr::<BhkRigidBodyCinfo>::new(cinfo);
        e.set(cinfo_ptr, BhkRigidBodyCinfo::m_shape, Ptr::<()>::new(shape));
        e.call(0x0056_d360, &args![cinfo, 5u32]);
        let position_vector = locals.alloc(e, 16);
        e.call(LIST_ITEM_SLOT, &args![position_vector]);
        let local_rotation = locals.alloc(e, 16);
        e.call(LIST_ITEM_SLOT, &args![local_rotation]);
        let rotation = locals.alloc(e, 16);
        e.call(0x0062_40d0, &args![rotation]);
        let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
        fn_0056d230(e, Ptr::new(node), Ptr::new(local_rotation));
        let position = e.vcall(me, SLOT_GET_POSITION, &args![]).u32();
        e.call(0x004a_3e00, &args![position_vector, position]);
        e.call(0x0056_1500, &args![rotation, local_rotation]);
        e.call(0x0056_d300, &args![cinfo, position_vector]);
        e.call(0x0056_d320, &args![cinfo, rotation]);
        e.set(
            cinfo_ptr,
            BhkRigidBodyCinfo::m_collisionFilterInfo,
            WATER_COLLISION_FILTER_INFO,
        );
        let body = build_water_body(e, me, cinfo);
        e.call(0x0056_d2c0, &args![holder, body]);
        e.call(0x0056_d730, &args![cinfo]);
    }
    locals.release(e);
}

// ---- The Havok glue after InitHavokForPlaceableWater ---------------------

/// The game's allocator wrapper (`00aa13e0`, cdecl: size) and the sized
/// release (`00aa1460`, cdecl: block, size) the scalar deleting destructors
/// call.
const ALLOCATE: u32 = 0x00aa_13e0;
const FREE_SIZED: u32 = 0x00aa_1460;
/// Releases an object of the `00538ef0` family (`00538eb0`, cdecl: block).
const FREE_OBJECT: u32 = 0x0053_8eb0;
/// Converts the three floats at `source` into the 16-byte vector
/// `destination` (`004a3e00`, cdecl: destination, source; the fourth float
/// is set to 0). Returns `destination`.
const VECTOR3_TO_VECTOR4: u32 = 0x004a_3e00;
/// Copies the 16-byte vector at the argument to `this` (`004a3f10`, an
/// aligned `MOVAPS` copy).
const COPY_VECTOR4: u32 = 0x004a_3f10;
/// Stores the low byte of its argument at `this` (`005407b0`).
const STORE_BYTE: u32 = 0x0054_07b0;
/// The identity on `this` that the game uses as a trivial constructor
/// (`006815c0`); [`LIST_ITEM_SLOT`] is the same function.
const TRIVIAL_CONSTRUCTOR: u32 = LIST_ITEM_SLOT;
/// `ExtraDataList::GetPrimitive` (Xbox PDB, `0041fbe0`) and
/// `ExtraDataList::GetCollisionData` (Xbox PDB, `004211a0`).
const GET_PRIMITIVE: u32 = 0x0041_fbe0;
const GET_COLLISION_DATA: u32 = 0x0042_11a0;
/// The word at `this + 4` of a primitive record (`00726070`): 1 for a box,
/// 2 for a sphere in `InitHavokForPrimitiveTrigger`.
const PRIMITIVE_TYPE: u32 = 0x0072_6070;
/// Copies the three floats at `this + 0x18` of a primitive record into the
/// argument and returns it (`00413fc0`).
const PRIMITIVE_BOUNDS: u32 = 0x0041_3fc0;
/// `a + b` of two 3-float vectors (`00439e90`; `this` = a, arguments:
/// result, b) and `a * s` (`0045bb20`; `this` = a, arguments: result, s).
/// Both return the result vector.
const VECTOR3_ADD: u32 = 0x0043_9e90;
const VECTOR3_SCALE: u32 = 0x0045_bb20;
/// The global float `-1.0` the AABB case scales the box bounds with.
const MINUS_ONE: u32 = 0x0101_2054;
/// The 16-byte constant (0, 0, 0, 1.0f in the exe) `fn_0056d250` stores.
const VECTOR_0001: u32 = 0x010c_71b0;
/// The 3x3 identity matrix `InitHavokForPrimitiveTrigger` compares the
/// reference's orientation with.
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// Collision filter bits every primitive-trigger phantom gets before the
/// reference's own filter value is or-ed in.
const TRIGGER_FILTER_BASE: u32 = 0x0005_0000;
/// Filter value used when the reference has no collision data.
const DEFAULT_COLLISION_FILTER: u32 = 0x16;
/// The collision filter value that always selects the shape phantom over
/// the AABB phantom.
const SHAPE_PHANTOM_FILTER: u32 = 0x17;
/// Vtables set by the constructors below.
const VTABLE_BHK_RIGID_BODY: u32 = 0x0103_01b4;
const VTABLE_BHK_ENTITY: u32 = 0x0103_02bc;
const VTABLE_BHK_WORLD_OBJECT: u32 = 0x0103_0394;
const VTABLE_BHK_COLLISION_OBJECT: u32 = 0x0103_046c;
const VTABLE_BHK_WATER_PHANTOM_CALLBACK_SHAPE: u32 = 0x0103_0534;
const VTABLE_HKP_PHANTOM_CALLBACK_SHAPE: u32 = 0x0103_0570;
const VTABLE_HKP_SHAPE: u32 = 0x0103_05a8;
/// The instance counters the constructors increment.
const COUNT_BHK_WORLD_OBJECT: u32 = 0x0126_810c;
const COUNT_BHK_RIGID_BODY: u32 = 0x0126_81bc;
const COUNT_BHK_ENTITY: u32 = 0x0126_81fc;
/// The `NiRTTI` records the `GetRTTI` methods return.
const RTTI_BHK_WORLD_OBJECT: u32 = 0x0126_8110;
const RTTI_BHK_ENTITY: u32 = 0x0126_8200;
const RTTI_BHK_RIGID_BODY: u32 = 0x0126_81c0;
const RTTI_BHK_COLLISION_OBJECT: u32 = 0x0120_43f8;

fn bump_counter(e: &mut Engine, counter: u32) {
    let count = e.global::<u32>(counter);
    e.set_global(counter, count.wrapping_add(1));
}

fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for word in 0..count {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// The scalar deleting destructors' tail: releases the object when bit 0 of
/// `flags` is set.
fn release_if_requested(e: &mut Engine, this: Ptr, flags: u32, size: u32) {
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, size]);
    }
}

// Translated from 0056d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the 16-byte constant at `010c71b0` (0, 0, 0, 1.0f in the exe)
/// into the vector at `this`. The Havok rotation of the placeable-water body
/// starts as this value.
pub fn fn_0056d250(e: &mut Engine, this: Ptr) {
    let target = e.call(TRIVIAL_CONSTRUCTOR, &args![this]).u32();
    copy_words(e, VECTOR_0001, target, 4);
}

// Translated from 0056d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `size` bytes from Havok's memory router: the thread's router
/// (`00c85750`), its allocator member (`+0x10`, `0044edb0`), whose virtual
/// `+4` returns the block. The size is kept in the half-word at block `+4`.
pub fn fn_0056d280(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.call(0x0044_edb0, &args![router]).u32();
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

// Translated from 0056d2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual `+0x9c` of `object` with `this` (the cell's Havok holder)
/// and returns its byte result; 0 when `object` is null.
pub fn fn_0056d2c0(e: &mut Engine, this: Ptr, object: Ptr) -> u8 {
    if object.addr() == 0 {
        return 0;
    }
    e.vcall(object.addr(), 0x9c, &args![this]).u8()
}

// Translated from 0056d300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 16-byte vector `source` into the member at `this + 0x30`
/// ([`COPY_VECTOR4`]). The rigid-body creation info keeps its position
/// there.
pub fn fn_0056d300(e: &mut Engine, this: Ptr, source: Ptr) {
    e.call(COPY_VECTOR4, &args![this.addr() + 0x30, source]);
}

// Translated from 0056d320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 16-byte vector `source` into the member at `this + 0x40`
/// ([`fn_0056d340`]). The rigid-body creation info keeps its rotation
/// there.
pub fn fn_0056d320(e: &mut Engine, this: Ptr, source: Ptr) {
    fn_0056d340(e, Ptr::new(this.addr() + 0x40), source);
}

// Translated from 0056d340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 16-byte vector `source` to `this` ([`COPY_VECTOR4`]).
pub fn fn_0056d340(e: &mut Engine, this: Ptr, source: Ptr) {
    e.call(COPY_VECTOR4, &args![this, source]);
}

// Translated from 0056d360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the low byte of `value` at `this + 0xd4` ([`STORE_BYTE`]); the
/// callers pass 5 and 2 into the rigid-body creation info.
pub fn fn_0056d360(e: &mut Engine, this: Ptr, value: u32) {
    e.call(STORE_BYTE, &args![this.addr() + 0xd4, value]);
}

// Translated from 0056d380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::bhkRigidBody` (Xbox PDB): builds the `bhkEntity` part
/// ([`fn_0056d410`]), installs the `bhkRigidBody` vtable, constructs the
/// member at `this + 0x14` (`004ee810`), initializes the body from `cinfo`
/// (`bhkRigidBody::Init`, `00c8d650`) and counts the instance. The
/// exception-unwinding frame is not translated.
pub fn bhk_rigid_body_bhk_rigid_body(e: &mut Engine, this: Ptr, cinfo: Ptr) -> Ptr {
    fn_0056d410(e, this);
    e.mem.set_u32(this.addr(), VTABLE_BHK_RIGID_BODY);
    e.call(0x004e_e810, &args![this.addr() + 0x14]);
    e.call(0x00c8_d650, &args![this, cinfo]);
    bump_counter(e, COUNT_BHK_RIGID_BODY);
    this
}

// Translated from 0056d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that installs the vtable `010302bc` (RTTI `bhkEntity`): the
/// `bhkWorldObject` part first ([`fn_0056d440`]), then the vtable, and the
/// instance counter at `012681fc` goes up. Returns `this`.
pub fn fn_0056d410(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056d440(e, this);
    e.mem.set_u32(this.addr(), VTABLE_BHK_ENTITY);
    bump_counter(e, COUNT_BHK_ENTITY);
    this
}

// Translated from 0056d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that installs the vtable `01030394` (RTTI `bhkWorldObject`):
/// the base part (`004b5120`), the vtable, the word at `this + 0x10` set to
/// 0 and the instance counter at `0126810c` up. Returns `this`.
pub fn fn_0056d440(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x004b_5120, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_BHK_WORLD_OBJECT);
    e.mem.set_u32(this.addr() + 0x10, 0);
    bump_counter(e, COUNT_BHK_WORLD_OBJECT);
    this
}

// Translated from 0056d480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::GetRTTI` (Xbox PDB): the address of the class's
/// `NiRTTI` record.
pub fn bhk_world_object_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_WORLD_OBJECT
}

// Translated from 0056d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::ForceAdd` (Xbox PDB; `bool` there): calls virtual `+0x9c`
/// of `this` with `argument` and answers its value (the exe ends on that call
/// and leaves its `EAX`).
pub fn bhk_world_object_force_add(e: &mut Engine, this: Ptr, argument: u32) -> bool {
    e.vcall(this.addr(), 0x9c, &args![argument]).bool()
}

// Translated from 0056d4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::scalar deleting destructor` (Xbox PDB): runs
/// `~bhkWorldObject` (`00c85790`), then frees the 0x14-byte object when bit
/// 0 of `flags` is set. Returns `this`.
pub fn bhk_world_object_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00c8_5790, &args![this]);
    release_if_requested(e, this, flags, 0x14);
    this
}

// Translated from 0056d4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntity::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI`
/// record.
pub fn bhk_entity_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_ENTITY
}

// Translated from 0056d500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntity::scalar deleting destructor` (Xbox PDB): runs `~bhkEntity`
/// (`00c9d620`), then frees the 0x14-byte object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bhk_entity_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00c9_d620, &args![this]);
    release_if_requested(e, this, flags, 0x14);
    this
}

// Translated from 0056d530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI`
/// record.
pub fn bhk_rigid_body_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_RIGID_BODY
}

// Translated from 0056d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::QCinfoSize` (Xbox PDB): the size of the creation info,
/// 0xe0 bytes.
pub fn bhk_rigid_body_q_cinfo_size(_e: &mut Engine, _this: Ptr) -> u32 {
    0xe0
}

// Translated from 0056d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::scalar deleting destructor` (Xbox PDB): runs
/// `~bhkRigidBody` (`00c8e9c0`), then frees the 0x1c-byte object when bit 0
/// of `flags` is set. Returns `this`.
pub fn bhk_rigid_body_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00c8_e9c0, &args![this]);
    release_if_requested(e, this, flags, 0x1c);
    this
}

// Translated from 0056d580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that installs the vtable `0103046c` (RTTI
/// `bhkCollisionObject`): the base part (`00c6b950`, given `node`, the 3D
/// root the collision object belongs to), then the vtable. Returns `this`.
pub fn fn_0056d580(e: &mut Engine, this: Ptr, node: Ptr) -> Ptr {
    e.call(0x00c6_b950, &args![this, node]);
    e.mem.set_u32(this.addr(), VTABLE_BHK_COLLISION_OBJECT);
    this
}

// Translated from 0056d5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::GetRTTI` (Xbox PDB): the address of the class's
/// `NiRTTI` record.
pub fn bhk_collision_object_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_COLLISION_OBJECT
}

// Translated from 0056d5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (the engine map's name for it is a library
/// match): runs `0066d410`, then frees the 0x14-byte object when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_0056d5c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0066_d410, &args![this]);
    release_if_requested(e, this, flags, 0x14);
    this
}

// Translated from 0056d5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that installs the vtable `01030534` (RTTI
/// `bhkWaterPhantomCallbackShape`) after its base ([`fn_0056d610`]).
/// Returns `this`.
pub fn fn_0056d5f0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056d610(e, this);
    e.mem
        .set_u32(this.addr(), VTABLE_BHK_WATER_PHANTOM_CALLBACK_SHAPE);
    this
}

// Translated from 0056d610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that installs the vtable `01030570` (RTTI
/// `hkpPhantomCallbackShape`) after its base ([`fn_0056d640`], shape
/// type 0x1d). Returns `this`.
pub fn fn_0056d610(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056d640(e, this, 0x1d);
    e.mem
        .set_u32(this.addr(), VTABLE_HKP_PHANTOM_CALLBACK_SHAPE);
    this
}

// Translated from 0056d640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that installs the vtable `010305a8` (RTTI `hkpShape`): the
/// referenced-object base (`00538ef0`), the vtable, a trivial constructor
/// on the member at `this + 0xc`, the word at `this + 8` set to 0, and
/// `00537e90` on that member with `shape_type`. Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_0056d640(e: &mut Engine, this: Ptr, shape_type: u32) -> Ptr {
    e.call(0x0053_8ef0, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_HKP_SHAPE);
    e.call(TRIVIAL_CONSTRUCTOR, &args![this.addr() + 0xc]);
    e.mem.set_u32(this.addr() + 8, 0);
    e.call(0x0053_7e90, &args![this.addr() + 0xc, shape_type]);
    this
}

// Translated from 0056d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00538e10` on `this`, the destructor body of the `00538ef0` family
/// ([`fn_0056d710`] and [`fn_0056d6e0`] go through it).
pub fn fn_0056d6c0(e: &mut Engine, this: Ptr) {
    e.call(0x0053_8e10, &args![this]);
}

// Translated from 0056d6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor: [`fn_0056d710`], then `00538eb0` frees
/// the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0056d6e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0056d710(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 0056d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body: [`fn_0056d6c0`] on `this`.
pub fn fn_0056d710(e: &mut Engine, this: Ptr) {
    fn_0056d6c0(e, this);
}

// Translated from 0056d730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys a creation-info record: [`fn_0056d780`] on `this`.
pub fn fn_0056d730(e: &mut Engine, this: Ptr) {
    fn_0056d780(e, this);
}

// Translated from 0056d750 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor: [`fn_0056d7c0`], then `00538eb0` frees the
/// object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0056d750(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0056d7c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 0056d780 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_0056d7a0`] on `this`.
pub fn fn_0056d780(e: &mut Engine, this: Ptr) {
    fn_0056d7a0(e, this);
}

// Translated from 0056d7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `0057c530` on the member at `this + 0xc`.
pub fn fn_0056d7a0(e: &mut Engine, this: Ptr) {
    e.call(0x0057_c530, &args![this.addr() + 0xc]);
}

// Translated from 0056d7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body: [`fn_0056d710`] on `this`.
pub fn fn_0056d7c0(e: &mut Engine, this: Ptr) {
    fn_0056d710(e, this);
}

// Translated from 0056ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a 0x40-byte Havok transform at `this`: the rotation part
/// ([`fn_0056df00`]) and a trivial constructor on the translation vector at
/// `this + 0x30`. (The engine map's library name `SafeSQueue<>` is a false
/// match.) Returns `this`.
pub fn fn_0056ded0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056df00(e, this);
    e.call(TRIVIAL_CONSTRUCTOR, &args![this.addr() + 0x30]);
    this
}

// Translated from 0056df00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the rotation part of a Havok transform at `this` (`00621990`).
/// Returns `this`.
pub fn fn_0056df00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0062_1990, &args![this]);
    this
}

// Translated from 0056df20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShapePhantom::SetTransform` (Xbox PDB): converts the Gamebryo
/// transform to a Havok one in a temporary ([`fn_0056ded0`], [`ni2hk`]) and
/// hands it to [`fn_0056e050`]. The stack-protector check is not
/// translated.
pub fn bhk_shape_phantom_set_transform(e: &mut Engine, this: Ptr, transform: Ptr) {
    let converted = e.mem.alloc(0x40);
    fn_0056ded0(e, Ptr::new(converted));
    let result = ni2hk(e, Ptr::new(converted), transform);
    fn_0056e050(e, this, result);
    e.mem.free(converted);
}

// Translated from 0056df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NI2HK` (Xbox PDB): converts the Gamebryo transform `source` (rotation
/// matrix first, translation at `+0x24`) into the Havok transform `out`:
/// the rotation ([`fn_0056dff0`]) and the translation vector (`004a3e00`,
/// then [`fn_0056d300`]). Returns `out`. The stack-protector check is not
/// translated.
pub fn ni2hk(e: &mut Engine, out: Ptr, source: Ptr) -> Ptr {
    let out = e.call(TRIVIAL_CONSTRUCTOR, &args![out]).u32();
    fn_0056dff0(e, Ptr::new(out), source);
    let vector = e.mem.alloc(16);
    e.call(TRIVIAL_CONSTRUCTOR, &args![vector]);
    let translation = e
        .call(VECTOR3_TO_VECTOR4, &args![vector, source.addr() + 0x24])
        .u32();
    fn_0056d300(e, Ptr::new(out), Ptr::new(translation));
    e.mem.free(vector);
    Ptr::new(out)
}

// Translated from 0056dff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the three rotation columns of the Gamebryo matrix `source` into
/// the vectors of the Havok rotation `out`: for each index `i` it reads
/// column `i` (`00439f50`, three floats) and stores it as a 16-byte vector
/// (`004b4cf0` picks the vector `i` of `out`, `00553fc0` fills it). Returns
/// `out`.
pub fn fn_0056dff0(e: &mut Engine, out: Ptr, source: Ptr) -> Ptr {
    let column = e.mem.alloc(12);
    e.call(TRIVIAL_CONSTRUCTOR, &args![column]);
    for index in 0..3u32 {
        e.call(0x0043_9f50, &args![source, index, column]);
        let row = e.call(0x004b_4cf0, &args![out, index]).u32();
        e.call(0x0055_3fc0, &args![row, column]);
    }
    e.mem.free(column);
    out
}

// Translated from 0056e050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the transform of the Havok object `this` holds: when `004ae750`
/// finds it, `00c9e910` (the engine map names it
/// `hkpShapePhantom::setTransform`) is called with `transform` between two
/// calls of `00a29680` (a bare `RET`) on `this`.
pub fn fn_0056e050(e: &mut Engine, this: Ptr, transform: Ptr) {
    let object = e.call(0x004a_e750, &args![this]).u32();
    if object != 0 {
        e.call(0x00a2_9680, &args![this]);
        e.call(0x00c9_e910, &args![object, transform]);
        e.call(0x00a2_9680, &args![this]);
    }
}

// Translated from 0056e090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a phantom creation-info record at `this` (`0056e0b0`).
/// Returns `this`.
pub fn fn_0056e090(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0056_e0b0, &args![this]);
    this
}

/// The `NiPointer<bhkPhantom>` slot of the reference's loaded data
/// (`LOADED_REF_DATA::spPhantom`, Xbox PDB, `+0x18`), read from the
/// reference each time the way the game does.
fn phantom_slot(e: &Engine, this: Ptr<TESObjectREFR>) -> u32 {
    e.get(this, TESObjectREFR::pLoadedData).addr() + 0x18
}

/// The reference's `NiTransform` that `InitHavokForPrimitiveTrigger` builds:
/// identity matrix constructor (`00476a80`), the orientation copied over it
/// (`GetOrientation`, `0056fa00`), the position (virtual `+0x1f4`) at
/// `+0x24` and scale 1.0 at `+0x30`. Returns the 0x34-byte block.
fn build_trigger_transform(e: &mut Engine, me: u32, locals: &mut Locals) -> u32 {
    let transform = locals.alloc(e, 0x34);
    e.call(0x0047_6a80, &args![transform]);
    let orientation_out = locals.alloc(e, 0x24);
    let orientation = e.call(0x0056_fa00, &args![me, orientation_out]).u32();
    copy_words(e, orientation, transform, 9);
    let position = e.vcall(me, SLOT_GET_POSITION, &args![]).u32();
    copy_words(e, position, transform + 0x24, 3);
    e.mem.set_f32(transform + 0x30, 1.0);
    transform
}

// Translated from 0056d7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::InitHavokForPrimitiveTrigger` (Xbox PDB): gives a
/// primitive trigger volume (the `BGSPrimitive` of the reference's extra
/// data) a Havok phantom, so entering it can be detected. Does nothing
/// without a parent cell and its loaded-cell holder (`004543c0`).
///
/// The reference's scale (`00567490`) and its 3D root's scale (`00440490`)
/// are set to 1.0 and the root's local translation to the origin
/// (`00a59c60`). The reference's orientation, position and scale 1.0 go
/// into an `NiTransform`. The phantom kind depends on the collision filter
/// (the reference's collision data when it has any, else 0x16) and the
/// primitive:
///
/// - a box primitive (type 1), a filter other than 0x17 and an orientation
///   equal to the identity matrix (`011a9448`) get a `bhkAabbPhantom`
///   (`0056e550`) whose box is the position plus and minus the primitive's
///   bounds (`00413fc0`; the minus case scales them by the global `-1.0`
///   at `01012054`);
/// - every other case builds the shape first (a `bhkBoxShape`, `0056e610`,
///   for type 1; a `bhkSphereShape`, `0056e950`, from the first bound for
///   type 2; for other types the shape stays null) and wraps it in a
///   `bhkSimpleShapePhantom` (`0056e2d0`) placed with
///   `bhkShapePhantom::SetTransform` ([`bhk_shape_phantom_set_transform`]).
///
/// Both store the phantom in the loaded data's `spPhantom` (`0066b0d0`).
/// Then a `bhkCollisionObject` ([`fn_0056d580`]) is made for the 3D root, a
/// `BSXFlags` extra data (`00c42f70`, set to 2, 1) is added to the root
/// (`00a5bca0`), the phantom is tied to the root (`00c85c10`) and added to
/// the cell holder ([`fn_0056d2c0`]).
///
/// The temporaries (transform, vectors, creation-info records) are blocks of
/// the engine's memory, freed on every exit. The exception-unwinding frame
/// and the stack-protector check are not translated.
pub fn tes_object_refr_init_havok_for_primitive_trigger(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let list = extra_list(e, me);
    let primitive = e.call(GET_PRIMITIVE, &args![list]).u32();
    let holder = if e.call(GET_PARENT_CELL, &args![me]).u32() != 0 {
        let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    if holder == 0 {
        return;
    }
    let mut locals = Locals::default();
    let mut shape = 0u32;
    let unused_transform = locals.alloc(e, 0x40);
    fn_0056ded0(e, Ptr::new(unused_transform));
    let box_vector = locals.alloc(e, 16);
    e.call(TRIVIAL_CONSTRUCTOR, &args![box_vector]);
    e.call(0x0056_7490, &args![me, 1.0f32]);
    let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
    e.call(0x0044_0490, &args![node, 1.0f32]);
    let origin = locals.alloc(e, 12);
    e.call(0x0043_d410, &args![origin, 0.0f32, 0.0f32, 0.0f32]);
    let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
    e.call(0x00a5_9c60, &args![node, origin]);
    let transform = build_trigger_transform(e, me, &mut locals);

    let list = extra_list(e, me);
    let collision_data = e.call(GET_COLLISION_DATA, &args![list]).u32();
    let filter = if collision_data != 0 {
        ni_pointer_get(e, collision_data)
    } else {
        DEFAULT_COLLISION_FILTER
    };
    let aabb = filter != SHAPE_PHANTOM_FILTER
        && e.call(PRIMITIVE_TYPE, &args![primitive]).u32() == 1
        && e.call(0x004d_9ae0, &args![transform, IDENTITY_MATRIX])
            .bool();
    let position = transform + 0x24;

    if aabb {
        let cinfo = locals.alloc(e, 0x40);
        e.call(0x0056_e4e0, &args![cinfo]);
        let vector = locals.alloc(e, 16);
        e.call(TRIVIAL_CONSTRUCTOR, &args![vector]);
        let sum = locals.alloc(e, 12);
        e.call(TRIVIAL_CONSTRUCTOR, &args![sum]);
        // the box corner at position + bounds goes to cinfo + 0x30
        let bounds = locals.alloc(e, 12);
        let added = locals.alloc(e, 12);
        let result = e.call(PRIMITIVE_BOUNDS, &args![primitive, bounds]).u32();
        let result = e.call(VECTOR3_ADD, &args![result, added, position]).u32();
        copy_words(e, result, sum, 3);
        e.call(VECTOR3_TO_VECTOR4, &args![vector, sum]);
        e.call(COPY_VECTOR4, &args![cinfo + 0x30, vector]);
        // and the one at position - bounds to cinfo + 0x20
        let minus_one = e.global::<f32>(MINUS_ONE);
        let bounds = locals.alloc(e, 12);
        let scaled = locals.alloc(e, 12);
        let added = locals.alloc(e, 12);
        let result = e.call(PRIMITIVE_BOUNDS, &args![primitive, bounds]).u32();
        let result = e
            .call(VECTOR3_SCALE, &args![result, scaled, minus_one])
            .u32();
        let result = e.call(VECTOR3_ADD, &args![result, added, position]).u32();
        copy_words(e, result, sum, 3);
        e.call(VECTOR3_TO_VECTOR4, &args![vector, sum]);
        e.call(COPY_VECTOR4, &args![cinfo + 0x20, vector]);
        e.call(STORE_BYTE, &args![cinfo + 8, 2u32]);
        e.mem.set_u32(cinfo, TRIGGER_FILTER_BASE | filter);
        let raw = e.call(ALLOCATE, &args![0x18u32]).u32();
        let phantom = if raw != 0 {
            e.call(0x0056_e550, &args![raw, cinfo]).u32()
        } else {
            0
        };
        let slot = phantom_slot(e, this);
        e.call(NI_POINTER_ASSIGN, &args![slot, phantom]);
        fn_0056d730(e, Ptr::new(cinfo));
    } else {
        if e.call(PRIMITIVE_TYPE, &args![primitive]).u32() == 1 {
            let bounds = locals.alloc(e, 12);
            let result = e.call(PRIMITIVE_BOUNDS, &args![primitive, bounds]).u32();
            e.call(VECTOR3_TO_VECTOR4, &args![box_vector, result]);
            let raw = e.call(ALLOCATE, &args![0x14u32]).u32();
            shape = if raw != 0 {
                e.call(0x0056_e610, &args![raw, box_vector]).u32()
            } else {
                0
            };
        }
        if e.call(PRIMITIVE_TYPE, &args![primitive]).u32() == 2 {
            let raw = e.call(ALLOCATE, &args![0x14u32]).u32();
            shape = if raw != 0 {
                let bounds = locals.alloc(e, 12);
                let result = e.call(PRIMITIVE_BOUNDS, &args![primitive, bounds]).u32();
                let radius = e.mem.f32(result);
                let radius = e.call(0x004a_3e90, &args![radius]).f32();
                e.call(0x0056_e950, &args![raw, radius, 0u32]).u32()
            } else {
                0
            };
        }
        let cinfo = locals.alloc(e, 0x60);
        fn_0056e090(e, Ptr::new(cinfo));
        let hk_shape = e.call(0x0062_0b80, &args![shape]).u32();
        e.mem.set_u32(cinfo + 4, hk_shape);
        e.call(STORE_BYTE, &args![cinfo + 8, 2u32]);
        e.mem.set_u32(cinfo, TRIGGER_FILTER_BASE | filter);
        let raw = e.call(ALLOCATE, &args![0x18u32]).u32();
        let phantom = if raw != 0 {
            e.call(0x0056_e2d0, &args![raw, cinfo]).u32()
        } else {
            0
        };
        bhk_shape_phantom_set_transform(e, Ptr::new(phantom), Ptr::new(transform));
        let slot = phantom_slot(e, this);
        e.call(NI_POINTER_ASSIGN, &args![slot, phantom]);
        e.call(0x0056_ea90, &args![cinfo]);
    }

    let raw = e.call(ALLOCATE, &args![0x14u32]).u32();
    if raw != 0 {
        let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
        fn_0056d580(e, Ptr::new(raw), Ptr::new(node));
    }
    let raw = e.call(ALLOCATE, &args![0x10u32]).u32();
    let flags = if raw != 0 {
        e.call(0x00c4_2f70, &args![raw]).u32()
    } else {
        0
    };
    e.call(0x0051_9230, &args![flags, 2u32, 1u32]);
    let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
    e.call(0x00a5_bca0, &args![node, flags]);
    let node = e.vcall(me, SLOT_GET_3D, &args![]).u32();
    let slot = phantom_slot(e, this);
    let phantom = ni_pointer_get(e, slot);
    e.call(0x00c8_5c10, &args![phantom, node, 0u32]);
    let slot = phantom_slot(e, this);
    let phantom = ni_pointer_get(e, slot);
    fn_0056d2c0(e, Ptr::new(holder), Ptr::new(phantom));
    locals.release(e);
}

// ---------------------------------------------------------------------
// Third batch: the Havok phantom and shape classes (`0056e0b0` to
// `0056ea00`).

/// Vtables (the RTTI locator before each names the class), instance
/// counters and `NiRTTI` records of the classes below.
const VTABLE_BHK_SIMPLE_SHAPE_PHANTOM: u32 = 0x0103_05dc;
const VTABLE_BHK_SHAPE_PHANTOM: u32 = 0x0103_06b4;
const VTABLE_BHK_PHANTOM: u32 = 0x0103_078c;
const VTABLE_BHK_AABB_PHANTOM: u32 = 0x0103_0864;
const VTABLE_BHK_BOX_SHAPE: u32 = 0x0103_093c;
const VTABLE_BHK_SPHERE_SHAPE: u32 = 0x0103_0cdc;
const COUNT_BHK_SHAPE_PHANTOM: u32 = 0x0126_8208;
const COUNT_BHK_SIMPLE_SHAPE_PHANTOM: u32 = 0x0126_822c;
const COUNT_BHK_PHANTOM: u32 = 0x0126_8238;
const COUNT_BHK_AABB_PHANTOM: u32 = 0x0126_8244;
const COUNT_BHK_BOX_SHAPE: u32 = 0x0126_825c;
const COUNT_BHK_SPHERE_SHAPE: u32 = 0x0126_8280;
const RTTI_BHK_SHAPE_PHANTOM: u32 = 0x0126_8224;
const RTTI_BHK_SIMPLE_SHAPE_PHANTOM: u32 = 0x0126_8230;
const RTTI_BHK_PHANTOM: u32 = 0x0126_823c;
const RTTI_BHK_AABB_PHANTOM: u32 = 0x0126_8248;
const RTTI_BHK_SHAPE: u32 = 0x0126_8100;
const RTTI_BHK_CONVEX_SHAPE: u32 = 0x0126_8254;
const RTTI_BHK_BOX_SHAPE: u32 = 0x0126_8264;
const RTTI_BHK_SPHERE_REP_SHAPE: u32 = 0x0126_8278;
const RTTI_BHK_SPHERE_SHAPE: u32 = 0x0126_8288;
/// The default convex radius (`float`, 0.1) of a box shape's creation info.
const DEFAULT_CONVEX_RADIUS: u32 = 0x010c_72bc;
/// Virtual slot `+0xb0` of the shape classes: initializes the shape from a
/// creation-info record passed by address.
const SLOT_INIT_FROM_CINFO: u32 = 0xb0;

/// The scalar deleting destructors of this batch: run `body` (the class's
/// destructor), then free `size` bytes when bit 0 of `flags` is set.
fn scalar_delete(e: &mut Engine, this: Ptr, flags: u32, body: u32, size: u32) -> Ptr {
    e.call(body, &args![this]);
    release_if_requested(e, this, flags, size);
    this
}

// Translated from 0056e0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the 0x60-byte phantom creation-info record at `this`: the
/// leading part ([`fn_0056e180`]), then the transform block at
/// `this + 0x20` ([`fn_0056e270`], reset to the identity by
/// [`fn_0056e120`]). Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_0056e0b0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056e180(e, this);
    fn_0056e270(e, Ptr::new(this.addr() + 0x20));
    fn_0056e120(e, Ptr::new(this.addr() + 0x20));
    this
}

// Translated from 0056e120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the 0x40-byte block at `this` ([`fn_0056e140`]: three vectors
/// with a 1.0 diagonal) and zeroes the vector at `this + 0x30`
/// (`004b4fc0`).
pub fn fn_0056e120(e: &mut Engine, this: Ptr) {
    fn_0056e140(e, this);
    e.call(0x004b_4fc0, &args![this.addr() + 0x30]);
}

// Translated from 0056e140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes the three 16-byte vectors at `this`, `this + 0x10` and
/// `this + 0x20` (`004b4fc0`) and sets the floats at offsets 0, 0x14 and
/// 0x28 to 1.0: a 3x3 identity matrix in 16-byte rows.
pub fn fn_0056e140(e: &mut Engine, this: Ptr) {
    e.call(0x004b_4fc0, &args![this]);
    e.call(0x004b_4fc0, &args![this.addr() + 0x10]);
    e.call(0x004b_4fc0, &args![this.addr() + 0x20]);
    e.mem.set_f32(this.addr(), 1.0);
    e.mem.set_f32(this.addr() + 0x14, 1.0);
    e.mem.set_f32(this.addr() + 0x28, 1.0);
}

// Translated from 0056e180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the leading 0x20 bytes of the creation-info record
/// ([`fn_0056e1e0`]) and then runs `005407b0` on `this + 8` with 2.
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_0056e180(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056e1e0(e, this);
    e.call(0x0054_07b0, &args![this.addr() + 8, 2u32]);
    this
}

// Translated from 0056e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the members of the record's leading part: `006815c0` on
/// `this` and `this + 8`, `006299a0` on `this + 0xc`, the words at `this`
/// and `this + 4` zeroed, then `005407b0` on `this + 8` with 1. Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_0056e1e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![this]);
    e.call(0x0068_15c0, &args![this.addr() + 8]);
    e.call(0x0062_99a0, &args![this.addr() + 0xc]);
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(0x0054_07b0, &args![this.addr() + 8, 1u32]);
    this
}

// Translated from 0056e270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the block of four vectors at `this` (the engine map names the
/// body `Concurrency::details::SafeSQueue<...>::SafeSQueue` because of
/// identical-code folding): [`fn_0056e2a0`] on `this`, then `004b4fa0` on
/// `this + 0x30`. Returns `this`.
pub fn fn_0056e270(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056e2a0(e, this);
    e.call(0x004b_4fa0, &args![this.addr() + 0x30]);
    this
}

// Translated from 0056e2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `004b4fa0` on the vectors at `this`, `this + 0x10` and
/// `this + 0x20`. Returns `this`.
pub fn fn_0056e2a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x004b_4fa0, &args![this]);
    e.call(0x004b_4fa0, &args![this.addr() + 0x10]);
    e.call(0x004b_4fa0, &args![this.addr() + 0x20]);
    this
}

// Translated from 0056e2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSimpleShapePhantom::bhkSimpleShapePhantom` (Xbox PDB): builds the
/// `bhkShapePhantom` part ([`fn_0056e370`]), installs the
/// `bhkSimpleShapePhantom` vtable, initializes the phantom from `cinfo`
/// (`Init`, `00c9f050`), counts the instance and clears the byte at
/// `this + 0x14` ([`fn_0056e350`]). Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn bhk_simple_shape_phantom_bhk_simple_shape_phantom(
    e: &mut Engine,
    this: Ptr,
    cinfo: Ptr,
) -> Ptr {
    fn_0056e370(e, this);
    e.mem.set_u32(this.addr(), VTABLE_BHK_SIMPLE_SHAPE_PHANTOM);
    e.call(0x00c9_f050, &args![this, cinfo]);
    bump_counter(e, COUNT_BHK_SIMPLE_SHAPE_PHANTOM);
    fn_0056e350(e, this);
    this
}

// Translated from 0056e350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `this + 0x14`.
pub fn fn_0056e350(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x14, 0);
}

// Translated from 0056e370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShapePhantom` constructor (the vtable `010306b4` has RTTI
/// `bhkShapePhantom`): the `bhkPhantom` part ([`fn_0056e3e0`]), then the
/// vtable, the instance counter at `01268208` and [`fn_0056e350`].
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_0056e370(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056e3e0(e, this);
    e.mem.set_u32(this.addr(), VTABLE_BHK_SHAPE_PHANTOM);
    bump_counter(e, COUNT_BHK_SHAPE_PHANTOM);
    fn_0056e350(e, this);
    this
}

// Translated from 0056e3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPhantom` constructor (vtable `0103078c`): the `bhkWorldObject` part
/// ([`fn_0056d440`]), the vtable and the instance counter at `01268238`.
/// Returns `this`.
pub fn fn_0056e3e0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056d440(e, this);
    e.mem.set_u32(this.addr(), VTABLE_BHK_PHANTOM);
    bump_counter(e, COUNT_BHK_PHANTOM);
    this
}

// Translated from 0056e410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPhantom::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI`
/// record.
pub fn bhk_phantom_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_PHANTOM
}

// Translated from 0056e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPhantom::scalar deleting destructor` (Xbox PDB): runs `~bhkPhantom`
/// (`00c9f3b0`), then frees the 0x18-byte object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bhk_phantom_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, 0x00c9_f3b0, 0x18)
}

// Translated from 0056e450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShapePhantom::GetRTTI` (Xbox PDB): the address of the class's
/// `NiRTTI` record.
pub fn bhk_shape_phantom_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_SHAPE_PHANTOM
}

// Translated from 0056e460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShapePhantom::scalar deleting destructor` (Xbox PDB): runs
/// `~bhkShapePhantom` (`00c9eb30`), then frees the 0x18-byte object when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bhk_shape_phantom_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, 0x00c9_eb30, 0x18)
}

// Translated from 0056e490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSimpleShapePhantom::GetRTTI` (Xbox PDB): the address of the class's
/// `NiRTTI` record.
pub fn bhk_simple_shape_phantom_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_SIMPLE_SHAPE_PHANTOM
}

// Translated from 0056e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method that returns the constant 0x60, the size of the creation-info
/// record [`fn_0056e0b0`] builds (the sibling of
/// `bhkRigidBody::QCinfoSize`, which returns 0xe0).
pub fn fn_0056e4a0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x60
}

// Translated from 0056e4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSimpleShapePhantom::scalar deleting destructor` (Xbox PDB): runs
/// `~bhkSimpleShapePhantom` (`00c9eec0`), then frees the 0x18-byte object
/// when bit 0 of `flags` is set. Returns `this`.
pub fn bhk_simple_shape_phantom_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, 0x00c9_eec0, 0x18)
}

// Translated from 0056e4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the leading-part constructor on the creation-info record at `this`
/// ([`fn_0056e180`]), then `00437c10` and `00538f40` on `this + 0x20` and
/// `00538f40` on `this + 0x30` (the record's member cleanup; the callees'
/// roles are not named in the Xbox PDB). Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn fn_0056e4e0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056e180(e, this);
    e.call(0x0043_7c10, &args![this.addr() + 0x20]);
    e.call(0x0053_8f40, &args![this.addr() + 0x20]);
    e.call(0x0053_8f40, &args![this.addr() + 0x30]);
    this
}

// Translated from 0056e550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAabbPhantom::bhkAabbPhantom` (Xbox PDB): builds the `bhkPhantom`
/// part ([`fn_0056e3e0`]), installs the `bhkAabbPhantom` vtable,
/// initializes the phantom from `cinfo` (`Init`, `00c9f890`), counts the
/// instance and clears the byte at `this + 0x14` ([`fn_0056e350`]).
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn bhk_aabb_phantom_bhk_aabb_phantom(e: &mut Engine, this: Ptr, cinfo: Ptr) -> Ptr {
    fn_0056e3e0(e, this);
    e.mem.set_u32(this.addr(), VTABLE_BHK_AABB_PHANTOM);
    e.call(0x00c9_f890, &args![this, cinfo]);
    bump_counter(e, COUNT_BHK_AABB_PHANTOM);
    fn_0056e350(e, this);
    this
}

// Translated from 0056e5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAabbPhantom::GetRTTI` (Xbox PDB): the address of the class's
/// `NiRTTI` record.
pub fn bhk_aabb_phantom_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_AABB_PHANTOM
}

// Translated from 0056e5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAabbPhantom::scalar deleting destructor` (Xbox PDB): runs
/// `~bhkAabbPhantom` (`00c9f530`), then frees the 0x18-byte object when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bhk_aabb_phantom_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, 0x00c9_f530, 0x18)
}

// Translated from 0056e610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBoxShape::bhkBoxShape` (Xbox PDB): builds the base part
/// (`0056e690`), installs the `bhkBoxShape` vtable, counts the instance and
/// initializes the shape from the half extents at `half_extents`
/// ([`fn_0056e810`]). Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn bhk_box_shape_bhk_box_shape(e: &mut Engine, this: Ptr, half_extents: Ptr) -> Ptr {
    e.call(0x0056_e690, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_BHK_BOX_SHAPE);
    bump_counter(e, COUNT_BHK_BOX_SHAPE);
    fn_0056e810(e, this, half_extents);
    this
}

// Translated from 0056e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI`
/// record.
pub fn bhk_shape_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_SHAPE
}

// Translated from 0056e740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::scalar deleting destructor` (Xbox PDB): runs `~bhkShape`
/// (`00c84c50`), then frees the 0x14-byte object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bhk_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, 0x00c8_4c50, 0x14)
}

// Translated from 0056e770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSphereRepShape::GetRTTI` (Xbox PDB): the address of the class's
/// `NiRTTI` record.
pub fn bhk_sphere_rep_shape_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_SPHERE_REP_SHAPE
}

// Translated from 0056e780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSphereRepShape::scalar deleting destructor` (Xbox PDB): runs
/// `~bhkSphereRepShape` (`00ca0540`), then frees the 0x14-byte object when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bhk_sphere_rep_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, 0x00ca_0540, 0x14)
}

// Translated from 0056e7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexShape::GetRTTI` (Xbox PDB): the address of the class's
/// `NiRTTI` record.
pub fn bhk_convex_shape_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_CONVEX_SHAPE
}

// Translated from 0056e7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexShape::scalar deleting destructor` (Xbox PDB): runs
/// `~bhkConvexShape` (`00c9fb00`), then frees the 0x14-byte object when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bhk_convex_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, 0x00c9_fb00, 0x14)
}

// Translated from 0056e7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBoxShape::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI`
/// record.
pub fn bhk_box_shape_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_BOX_SHAPE
}

// Translated from 0056e800 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method that returns the constant 0x20, the size of the box
/// creation-info record [`fn_0056e810`] builds on its stack.
pub fn fn_0056e800(_e: &mut Engine, _this: Ptr) -> u32 {
    0x20
}

// Translated from 0056e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the box shape at `this` from the half extents at
/// `half_extents`: builds the 0x20-byte creation-info record on the stack
/// ([`fn_0056e860`]), copies the half extents into its vector at `+0x10`
/// (`004b4db0`, which reads the components through the pointer) and calls
/// virtual `+0xb0` of `this` with the record.
pub fn fn_0056e810(e: &mut Engine, this: Ptr, half_extents: Ptr) {
    e.with_stack(0x20, |e, cinfo| {
        fn_0056e860(e, cinfo);
        e.call(0x004b_4db0, &args![cinfo.addr() + 0x10, half_extents]);
        e.vcall(this.addr(), SLOT_INIT_FROM_CINFO, &args![cinfo]);
    });
}

// Translated from 0056e860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the box creation-info record at `this`: its leading part
/// ([`fn_0056e8d0`]), then the vector at `this + 0x10` (`004b4fa0`) set to
/// (1.0, 1.0, 1.0) ([`fn_0056e8a0`]). Returns `this`.
pub fn fn_0056e860(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056e8d0(e, this);
    e.call(0x004b_4fa0, &args![this.addr() + 0x10]);
    fn_0056e8a0(e, Ptr::new(this.addr() + 0x10), 1.0);
    this
}

// Translated from 0056e8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the three floats at `this`, `this + 4` and
/// `this + 8`.
pub fn fn_0056e8a0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr(), x87_quiet(value));
    e.mem.set_f32(this.addr() + 4, x87_quiet(value));
    e.mem.set_f32(this.addr() + 8, x87_quiet(value));
}

// Translated from 0056e8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the leading part of the box creation info ([`fn_0056e900`])
/// and stores the default convex radius (the float at `010c72bc`) at
/// `this + 4`. Returns `this`.
pub fn fn_0056e8d0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056e900(e, this);
    let radius: f32 = e.global(DEFAULT_CONVEX_RADIUS);
    e.mem.set_f32(this.addr() + 4, radius);
    this
}

// Translated from 0056e900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `004b5020` (which zeroes the word at `this`) on `this`. Returns
/// `this`.
pub fn fn_0056e900(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x004b_5020, &args![this]);
    this
}

// Translated from 0056e920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBoxShape::scalar deleting destructor` (Xbox PDB): runs
/// `~bhkBoxShape` (`00c9fd70`), then frees the 0x14-byte object when bit 0
/// of `flags` is set. Returns `this`.
pub fn bhk_box_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, 0x00c9_fd70, 0x14)
}

// Translated from 0056e950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSphereShape::bhkSphereShape` (Xbox PDB): builds the base part
/// (`0056e690`), installs the `bhkSphereShape` vtable, counts the instance,
/// converts `radius` with `004a3e90` when `convert` is nonzero and
/// initializes the shape with it ([`fn_0056ea00`]). Returns `this`. The
/// exception-unwinding frame is not translated.
pub fn bhk_sphere_shape_bhk_sphere_shape(
    e: &mut Engine,
    this: Ptr,
    radius: f32,
    convert: u8,
) -> Ptr {
    e.call(0x0056_e690, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_BHK_SPHERE_SHAPE);
    bump_counter(e, COUNT_BHK_SPHERE_SHAPE);
    let radius = if convert != 0 {
        e.call(0x004a_3e90, &args![radius]).f32()
    } else {
        radius
    };
    fn_0056ea00(e, this, radius);
    this
}

// Translated from 0056e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSphereShape::GetRTTI` (Xbox PDB): the address of the class's
/// `NiRTTI` record.
pub fn bhk_sphere_shape_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_BHK_SPHERE_SHAPE
}

// Translated from 0056ea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the sphere shape at `this` with `radius`: builds the 8-byte
/// creation-info record on the stack (`0056ea40` on it, the radius at
/// `+4`) and calls virtual `+0xb0` of `this` with it.
pub fn fn_0056ea00(e: &mut Engine, this: Ptr, radius: f32) {
    e.with_stack(8, |e, cinfo| {
        e.call(0x0056_ea40, &args![cinfo]);
        e.mem.set_f32(cinfo.addr() + 4, radius);
        e.vcall(this.addr(), SLOT_INIT_FROM_CINFO, &args![cinfo]);
    });
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0056a860, fn_0056a860(Ptr) -> bool),
        entry!(0x0056a880, fn_0056a880(Ptr) -> u32),
        entry!(0x0056a8c0, fn_0056a8c0() -> u32),
        entry!(0x0056a8d0, fn_0056a8d0(Ptr<TESObjectREFR>)),
        entry!(0x0056a990, fn_0056a990(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x0056a9b0,
            fn_0056a9b0(Ptr<TESObjectREFR>, Ptr<TESObjectREFR>)
        ),
        entry!(0x0056a9f0, fn_0056a9f0(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x0056aa10,
            fn_0056aa10(Ptr<TESObjectREFR>, Ptr<TESObjectREFR>)
        ),
        entry!(0x0056aa70, fn_0056aa70(Ptr<TESObjectREFR>) -> bool),
        entry!(0x0056aa90, fn_0056aa90(Ptr<TESObjectREFR>, u8)),
        entry!(
            0x0056aac0,
            tes_object_refr_check_enable_parent_loop(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(
            0x0056ac00,
            tes_object_refr_update_enable_state_children_flags_recursive(Ptr<TESObjectREFR>)
        ),
        entry!(0x0056ac90, fn_0056ac90(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x0056acb0,
            fn_0056acb0(Ptr<TESObjectREFR>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x0056ad30,
            tes_object_refr_get_package_start_location_world(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x0056ad80,
            tes_object_refr_get_package_start_location_interior_cell(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x0056add0,
            tes_object_refr_get_package_start_location_coord(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x0056ae10,
            tes_object_refr_set_package_start_location(Ptr<TESObjectREFR>, u32, u32, u32, f32)
        ),
        entry!(0x0056ae60, fn_0056ae60(Ptr<TESObjectREFR>) -> bool),
        entry!(0x0056af00, fn_0056af00(Ptr) -> bool),
        entry!(0x0056af20, fn_0056af20(Ptr) -> bool),
        entry!(0x0056af40, fn_0056af40(Ptr<TESObjectREFR>) -> bool),
        entry!(0x0056afc0, fn_0056afc0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x0056b020, fn_0056b020(Ptr<TESObjectREFR>, u32, u32)),
        entry!(
            0x0056b0b0,
            tes_object_refr_restore_leveled_creature_original_base(Ptr<TESObjectREFR>)
        ),
        entry!(0x0056b140, fn_0056b140(Ptr<TESObjectREFR>) -> u32),
        entry!(0x0056b190, fn_0056b190(Ptr<TESObjectREFR>) -> f32),
        entry!(0x0056b1d0, fn_0056b1d0(Ptr<TESObjectREFR>) -> f32),
        entry!(0x0056b210, fn_0056b210(Ptr<TESObjectREFR>) -> u32),
        entry!(0x0056b250, fn_0056b250(Ptr<TESObjectREFR>) -> bool),
        entry!(
            0x0056b2d0,
            tes_object_refr_load_3d(Ptr<TESObjectREFR>, u8) -> Ptr
        ),
        entry!(0x0056c760, fn_0056c760(Ptr<TESObjectREFR>) -> bool),
        entry!(0x0056c780, fn_0056c780(Ptr<TESObjectREFR>, u8)),
        entry!(0x0056c7d0, fn_0056c7d0(Ptr, u32)),
        entry!(0x0056c7f0, fn_0056c7f0(Ptr) -> u32),
        entry!(
            0x0056c810,
            bs_multi_bound_room_add_joined_multi_bound(Ptr, u32)
        ),
        entry!(0x0056c880, fn_0056c880(Ptr<TESObjectREFR>, u32, u8)),
        entry!(0x0056c8b0, fn_0056c8b0(Ptr<LOADED_REF_DATA>, u32, u8)),
        entry!(
            0x0056c8f0,
            tes_object_refr_init_havok_for_placeable_water(Ptr<TESObjectREFR>)
        ),
        entry!(0x0056d230, fn_0056d230(Ptr, Ptr)),
        entry!(0x0056d250, fn_0056d250(Ptr)),
        entry!(0x0056d280, fn_0056d280(u32) -> u32),
        entry!(0x0056d2c0, fn_0056d2c0(Ptr, Ptr) -> u8),
        entry!(0x0056d300, fn_0056d300(Ptr, Ptr)),
        entry!(0x0056d320, fn_0056d320(Ptr, Ptr)),
        entry!(0x0056d340, fn_0056d340(Ptr, Ptr)),
        entry!(0x0056d360, fn_0056d360(Ptr, u32)),
        entry!(
            0x0056d380,
            bhk_rigid_body_bhk_rigid_body(Ptr, Ptr) -> Ptr
        ),
        entry!(0x0056d410, fn_0056d410(Ptr) -> Ptr),
        entry!(0x0056d440, fn_0056d440(Ptr) -> Ptr),
        entry!(0x0056d480, bhk_world_object_get_rtti(Ptr) -> u32),
        entry!(0x0056d490, bhk_world_object_force_add(Ptr, u32) -> bool),
        entry!(
            0x0056d4c0,
            bhk_world_object_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056d4f0, bhk_entity_get_rtti(Ptr) -> u32),
        entry!(
            0x0056d500,
            bhk_entity_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056d530, bhk_rigid_body_get_rtti(Ptr) -> u32),
        entry!(0x0056d540, bhk_rigid_body_q_cinfo_size(Ptr) -> u32),
        entry!(
            0x0056d550,
            bhk_rigid_body_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056d580, fn_0056d580(Ptr, Ptr) -> Ptr),
        entry!(0x0056d5b0, bhk_collision_object_get_rtti(Ptr) -> u32),
        entry!(0x0056d5c0, fn_0056d5c0(Ptr, u32) -> Ptr),
        entry!(0x0056d5f0, fn_0056d5f0(Ptr) -> Ptr),
        entry!(0x0056d610, fn_0056d610(Ptr) -> Ptr),
        entry!(0x0056d640, fn_0056d640(Ptr, u32) -> Ptr),
        entry!(0x0056d6c0, fn_0056d6c0(Ptr)),
        entry!(0x0056d6e0, fn_0056d6e0(Ptr, u32) -> Ptr),
        entry!(0x0056d710, fn_0056d710(Ptr)),
        entry!(0x0056d730, fn_0056d730(Ptr)),
        entry!(0x0056d750, fn_0056d750(Ptr, u32) -> Ptr),
        entry!(0x0056d780, fn_0056d780(Ptr)),
        entry!(0x0056d7a0, fn_0056d7a0(Ptr)),
        entry!(0x0056d7c0, fn_0056d7c0(Ptr)),
        entry!(
            0x0056d7e0,
            tes_object_refr_init_havok_for_primitive_trigger(Ptr<TESObjectREFR>)
        ),
        entry!(0x0056ded0, fn_0056ded0(Ptr) -> Ptr),
        entry!(0x0056df00, fn_0056df00(Ptr) -> Ptr),
        entry!(0x0056df20, bhk_shape_phantom_set_transform(Ptr, Ptr)),
        entry!(0x0056df80, ni2hk(Ptr, Ptr) -> Ptr),
        entry!(0x0056dff0, fn_0056dff0(Ptr, Ptr) -> Ptr),
        entry!(0x0056e050, fn_0056e050(Ptr, Ptr)),
        entry!(0x0056e090, fn_0056e090(Ptr) -> Ptr),
        entry!(0x0056e0b0, fn_0056e0b0(Ptr) -> Ptr),
        entry!(0x0056e120, fn_0056e120(Ptr)),
        entry!(0x0056e140, fn_0056e140(Ptr)),
        entry!(0x0056e180, fn_0056e180(Ptr) -> Ptr),
        entry!(0x0056e1e0, fn_0056e1e0(Ptr) -> Ptr),
        entry!(0x0056e270, fn_0056e270(Ptr) -> Ptr),
        entry!(0x0056e2a0, fn_0056e2a0(Ptr) -> Ptr),
        entry!(
            0x0056e2d0,
            bhk_simple_shape_phantom_bhk_simple_shape_phantom(Ptr, Ptr) -> Ptr
        ),
        entry!(0x0056e350, fn_0056e350(Ptr)),
        entry!(0x0056e370, fn_0056e370(Ptr) -> Ptr),
        entry!(0x0056e3e0, fn_0056e3e0(Ptr) -> Ptr),
        entry!(0x0056e410, bhk_phantom_get_rtti(Ptr) -> u32),
        entry!(
            0x0056e420,
            bhk_phantom_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056e450, bhk_shape_phantom_get_rtti(Ptr) -> u32),
        entry!(
            0x0056e460,
            bhk_shape_phantom_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056e490, bhk_simple_shape_phantom_get_rtti(Ptr) -> u32),
        entry!(0x0056e4a0, fn_0056e4a0(Ptr) -> u32),
        entry!(
            0x0056e4b0,
            bhk_simple_shape_phantom_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056e4e0, fn_0056e4e0(Ptr) -> Ptr),
        entry!(
            0x0056e550,
            bhk_aabb_phantom_bhk_aabb_phantom(Ptr, Ptr) -> Ptr
        ),
        entry!(0x0056e5d0, bhk_aabb_phantom_get_rtti(Ptr) -> u32),
        entry!(
            0x0056e5e0,
            bhk_aabb_phantom_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056e610, bhk_box_shape_bhk_box_shape(Ptr, Ptr) -> Ptr),
        entry!(0x0056e730, bhk_shape_get_rtti(Ptr) -> u32),
        entry!(
            0x0056e740,
            bhk_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056e770, bhk_sphere_rep_shape_get_rtti(Ptr) -> u32),
        entry!(
            0x0056e780,
            bhk_sphere_rep_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056e7b0, bhk_convex_shape_get_rtti(Ptr) -> u32),
        entry!(
            0x0056e7c0,
            bhk_convex_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056e7f0, bhk_box_shape_get_rtti(Ptr) -> u32),
        entry!(0x0056e800, fn_0056e800(Ptr) -> u32),
        entry!(0x0056e810, fn_0056e810(Ptr, Ptr)),
        entry!(0x0056e860, fn_0056e860(Ptr) -> Ptr),
        entry!(0x0056e8a0, fn_0056e8a0(Ptr, f32)),
        entry!(0x0056e8d0, fn_0056e8d0(Ptr) -> Ptr),
        entry!(0x0056e900, fn_0056e900(Ptr) -> Ptr),
        entry!(
            0x0056e920,
            bhk_box_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0056e950,
            bhk_sphere_shape_bhk_sphere_shape(Ptr, f32, u8) -> Ptr
        ),
        entry!(0x0056e9f0, bhk_sphere_shape_get_rtti(Ptr) -> u32),
        entry!(0x0056ea00, fn_0056ea00(Ptr, f32)),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// Registers the small accessors this part leans on with the bodies the
    /// exe gives them.
    fn install_accessors(e: &mut Engine) {
        // 005d43c0: this + 0x44
        e.register(GET_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        // 007af430: pObjectReference (+0x20)
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        // 00401170: the form type byte
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        // 006815c0: its `this`; 00726070: the next pointer of a list node
        e.register(LIST_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        // 00559450: the first dword; 008d6f30: pParentCell (+0x40)
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GET_PARENT_CELL, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        install_accessors(&mut e);
        e
    }
    /// Maps the pages of the globals the functions read.
    fn map_globals(e: &mut Engine) {
        for page in [
            0x0101_2000,
            0x0101_3000,
            0x0101_7000,
            0x0101_8000,
            0x0103_0000,
            0x011b_1000,
            0x011c_3000,
            0x011c_a000,
            0x011d_d000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// A double that returns `value` in `eax`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double that returns the float `value` in `st0`.
    fn returns_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn addresses(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().map(|(a, _)| *a).collect()
    }

    /// An object with a vtable of its own: `slots` as (byte offset, target).
    fn object_with_vtable(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x500);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        e.mem.set_u32(object, vtable);
        object
    }

    /// A plain object big enough for a `TESObjectREFR` and the test fields.
    fn object(e: &mut Engine) -> Ptr<TESObjectREFR> {
        Ptr::new(e.mem.alloc(0x100))
    }

    /// A form object with a type byte at +4.
    fn form(e: &mut Engine, kind: u8) -> u32 {
        let form = e.mem.alloc(0x100);
        e.mem.set_u8(form + 4, kind);
        form
    }

    // ---- 0056a860, 0056a880, 0056a8c0 ----------------------------------

    #[test]
    fn fn_0056a860_tests_the_pointer_at_0x80() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x0056_a860, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x80, 0x1234);
        assert_eq!(e.call(0x0056_a860, &args![this]).u32(), 1);
    }

    #[test]
    fn fn_0056a880_falls_back_to_the_default_record() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x0056_a880, &args![this]).u32(), 0x011c_a8b4);
        e.mem.set_u32(this + 0x80, 0x1234);
        assert_eq!(e.call(0x0056_a880, &args![this]).u32(), 0x1234);
    }

    #[test]
    fn fn_0056a8c0_is_the_default_record_address() {
        let mut e = engine();
        assert_eq!(e.call(0x0056_a8c0, &args![]).u32(), 0x011c_a8b4);
    }

    // ---- 0056a8d0 -------------------------------------------------------

    #[test]
    fn fn_0056a8d0_removes_the_matching_children_of_the_3d_root() {
        let mut e = engine();
        returns(&mut e, 0x0043_b480, 3);
        let children = [0u32, 0x11, 0x22];
        e.register_double(0x0043_b4a0, move |_, a| children[a[1] as usize].into_ret());
        e.register(0x0045_bad0, |_, a| (a[1] == 0x22).into_ret());
        stub(&mut e, &[0x0050_f5a0, 0x00fe_0001]);
        let node = object_with_vtable(&mut e, 0x20, &[(0xf8, 0x00fe_0001)]);
        returns(&mut e, 0x00fe_0002, node);
        let root = object_with_vtable(&mut e, 0x20, &[(0xc, 0x00fe_0002)]);
        returns(&mut e, 0x00fe_0003, root);
        let this = object_with_vtable(&mut e, 0x100, &[(0x1d0, 0x00fe_0003)]);

        e.call_log = Some(vec![]);
        e.call(0x0056_a8d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0050_f5a0), vec![vec![0x22]]);
        assert_eq!(calls_to(&log, 0x00fe_0001), vec![vec![node, 2, 0]]);
        assert_eq!(calls_to(&log, 0x0045_bad0).len(), 2);

        // no 3D root: nothing happens
        returns(&mut e, 0x00fe_0003, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_a8d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(addresses(&log), vec![0x0056_a8d0, 0x00fe_0003]);
    }

    // ---- 0056a990 .. 0056aa90 ------------------------------------------

    #[test]
    fn fn_0056a990_asks_the_extra_data_list_for_the_merchant_container() {
        let mut e = engine();
        returns(&mut e, 0x0042_1400, 0x4444);
        let this = object(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_a990, &args![this]).u32(), 0x4444);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0042_1400), vec![vec![this.addr() + 0x44]]);
    }

    #[test]
    fn fn_0056a9b0_skips_a_non_persistent_reference() {
        let mut e = engine();
        stub(&mut e, &[0x0042_1430]);
        let this = object(&mut e);
        let other = object(&mut e);
        let list = this.addr() + 0x44;

        returns(&mut e, GET_REF_PERSISTS, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_a9b0, &args![this, other]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x0042_1430),
            vec![vec![list, other.addr()]]
        );

        returns(&mut e, GET_REF_PERSISTS, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_a9b0, &args![this, other]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0042_1430).is_empty());

        // a null reference always goes through
        e.call_log = Some(vec![]);
        e.call(0x0056_a9b0, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x0042_1430),
            vec![vec![list, 0]]
        );
    }

    /// The enable-state parent of an object lives at +0x60 in the tests.
    fn parent_double(e: &mut Engine) {
        e.register(0x0041_da10, |e, a| e.mem.u32(a[0] - 0x44 + 0x60).into_ret());
    }

    #[test]
    fn fn_0056a9f0_is_the_enable_state_parent() {
        let mut e = engine();
        parent_double(&mut e);
        let this = object(&mut e);
        e.mem.set_u32(this.addr() + 0x60, 0x7777);
        assert_eq!(e.call(0x0056_a9f0, &args![this]).u32(), 0x7777);
    }

    #[test]
    fn fn_0056aa10_leaves_the_old_parent_and_joins_the_new_one() {
        let mut e = engine();
        parent_double(&mut e);
        stub(&mut e, &[0x0041_dda0, 0x0041_da40]);
        returns(&mut e, GET_REF_PERSISTS, 1);
        let this = object(&mut e);
        let parent = object(&mut e);
        let other = object(&mut e);
        e.mem.set_u32(this.addr() + 0x60, parent.addr());

        e.call_log = Some(vec![]);
        e.call(0x0056_aa10, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0041_dda0),
            vec![vec![parent.addr() + 0x44, this.addr()]]
        );
        assert_eq!(
            calls_to(&log, 0x0041_da40),
            vec![vec![this.addr() + 0x44, other.addr()]]
        );

        // no parent, and a non-persistent other: nothing at all
        e.mem.set_u32(this.addr() + 0x60, 0);
        returns(&mut e, GET_REF_PERSISTS, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_aa10, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0041_dda0).is_empty());
        assert!(calls_to(&log, 0x0041_da40).is_empty());
    }

    #[test]
    fn fn_0056aa70_and_fn_0056aa90_wrap_the_extra_list_flag() {
        let mut e = engine();
        returns(&mut e, 0x0041_db00, 1);
        stub(&mut e, &[0x0041_db50]);
        let this = object(&mut e);
        assert_eq!(e.call(0x0056_aa70, &args![this]).u32(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_aa90, &args![this, 3u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x0041_db50),
            vec![vec![this.addr() + 0x44, 3]]
        );
    }

    // ---- 0056aac0 -------------------------------------------------------

    fn parent_loop_engine() -> Engine {
        let mut e = engine();
        parent_double(&mut e);
        stub(&mut e, &[0x0057_c470, 0x0084_d310, 0x0057_c8d0]);
        // every link is a reference; the visited set never reports a repeat
        returns(&mut e, 0x0040_13e0, 1);
        returns(&mut e, 0x0057_c850, 0);
        e
    }

    #[test]
    fn check_enable_parent_loop_accepts_a_chain_that_ends() {
        let mut e = parent_loop_engine();
        let a = object(&mut e);
        let b = object(&mut e);
        e.mem.set_u32(a.addr() + 0x60, b.addr());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_aac0, &args![a]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let set = calls_to(&log, 0x0057_c470)[0][0];
        assert_eq!(calls_to(&log, 0x0057_c470), vec![vec![set, 0x25]]);
        assert_eq!(calls_to(&log, 0x0084_d310), vec![vec![set, b.addr(), 1]]);
        assert_eq!(calls_to(&log, 0x0057_c8d0), vec![vec![set]]);
    }

    #[test]
    fn check_enable_parent_loop_rejects_a_chain_back_to_the_start() {
        let mut e = parent_loop_engine();
        let a = object(&mut e);
        let b = object(&mut e);
        e.mem.set_u32(a.addr() + 0x60, b.addr());
        e.mem.set_u32(b.addr() + 0x60, a.addr());
        assert_eq!(e.call(0x0056_aac0, &args![a]).u32(), 0);
    }

    #[test]
    fn check_enable_parent_loop_resolves_a_link_that_is_not_a_reference() {
        let mut e = parent_loop_engine();
        // the first link is not a reference: its parent id is looked up
        returns(&mut e, 0x0040_13e0, 0);
        let a = object(&mut e);
        let b = object(&mut e);
        let c = object(&mut e);
        e.mem.set_u32(a.addr() + 0x60, b.addr());
        e.mem.set_u32(b.addr() + 0x60, 0x0abc_0001);
        // the cast hands back the resolved object
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register_double(0x0048_39c0, move |_, _| c.addr().into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_aac0, &args![a]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_39c0), vec![vec![0x0abc_0001]]);
        assert_eq!(
            calls_to(&log, DYNAMIC_CAST),
            vec![vec![c.addr(), 0, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]]
        );
        assert_eq!(calls_to(&log, 0x0084_d310).len(), 2);
    }

    #[test]
    fn check_enable_parent_loop_stops_when_the_visit_closes_the_chain() {
        let mut e = parent_loop_engine();
        // the visited set reports a repeat and sets its flag
        e.register(0x0057_c850, |e, a| {
            e.mem.set_u8(a[2], 1);
            true.into_ret()
        });
        let a = object(&mut e);
        let b = object(&mut e);
        let c = object(&mut e);
        e.mem.set_u32(a.addr() + 0x60, b.addr());
        e.mem.set_u32(b.addr() + 0x60, c.addr());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_aac0, &args![a]).u32(), 1);
        let log = e.call_log.take().unwrap();
        // only the first link was recorded
        assert_eq!(calls_to(&log, 0x0084_d310).len(), 1);
    }

    // ---- 0056ac00, 0056ac90 --------------------------------------------

    /// Object fields in the tests: +0x70 disabled byte, +0x71 "opposite"
    /// byte, +0x64 head of the children list.
    fn children_engine() -> Engine {
        let mut e = engine();
        e.register(0x0044_0da0, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x70)).into_ret()
        });
        e.register(0x0041_db00, |e, a| {
            u32::from(e.mem.u8(a[0] - 0x44 + 0x71)).into_ret()
        });
        e.register(0x0041_dca0, |e, a| e.mem.u32(a[0] - 0x44 + 0x64).into_ret());
        e.register(0x0048_4af0, |e, a| {
            e.mem.set_u8(a[0] + 0x70, a[1] as u8);
            Ret::default()
        });
        e
    }

    fn list_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    #[test]
    fn update_enable_state_children_propagates_the_disabled_state() {
        let mut e = children_engine();
        let parent = object(&mut e);
        let plain = object(&mut e);
        let same = object(&mut e);
        let inverted = object(&mut e);
        let grandchild = object(&mut e);
        e.mem.set_u8(parent.addr() + 0x70, 1);
        e.mem.set_u8(same.addr() + 0x70, 1);
        e.mem.set_u8(inverted.addr() + 0x71, 1);
        e.mem.set_u8(inverted.addr() + 0x70, 1);
        // `plain` has a child of its own
        let inner = list_node(&mut e, grandchild.addr(), 0);
        e.mem.set_u32(plain.addr() + 0x64, inner);
        let n3 = list_node(&mut e, inverted.addr(), 0);
        let n2 = list_node(&mut e, same.addr(), n3);
        let n1 = list_node(&mut e, plain.addr(), n2);
        e.mem.set_u32(parent.addr() + 0x64, n1);

        e.call_log = Some(vec![]);
        e.call(0x0056_ac00, &args![parent]);
        let log = e.call_log.take().unwrap();
        let set = calls_to(&log, 0x0048_4af0);
        // `plain` becomes disabled, `same` already is; `inverted` (1) flips to
        // enabled; the grandchild follows `plain`
        assert_eq!(
            set,
            vec![
                vec![plain.addr(), 1],
                vec![grandchild.addr(), 1],
                vec![inverted.addr(), 0],
            ]
        );
        assert_eq!(e.mem.u8(inverted.addr() + 0x70), 0);
    }

    #[test]
    fn fn_0056ac90_is_the_children_list_head() {
        let mut e = children_engine();
        let this = object(&mut e);
        e.mem.set_u32(this.addr() + 0x64, 0x5555);
        assert_eq!(e.call(0x0056_ac90, &args![this]).u32(), 0x5555);
    }

    // ---- 0056acb0 -------------------------------------------------------

    #[test]
    fn fn_0056acb0_drops_an_item_into_the_holders_list() {
        let mut e = engine();
        map_globals(&mut e);
        stub(&mut e, &[0x0041_e000, 0x0041_de40, 0x00fe_0010]);
        returns(&mut e, GET_REF_PERSISTS, 0);
        let player = object(&mut e);
        e.set_global(0x011d_ea3c, player.addr());
        let holder = object(&mut e);
        let item = Ptr::<TESObjectREFR>::new(object_with_vtable(
            &mut e,
            0x100,
            &[(0x160, 0x00fe_0011), (SLOT_SET_CHANGED, 0x00fe_0010)],
        ));
        returns(&mut e, 0x00fe_0011, 0);

        e.call_log = Some(vec![]);
        e.call(0x0056_acb0, &args![holder, item]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0041_e000),
            vec![vec![holder.addr() + 0x44, item.addr()]]
        );
        assert_eq!(
            calls_to(&log, 0x0041_de40),
            vec![vec![item.addr() + 0x44, holder.addr()]]
        );
        assert_eq!(
            calls_to(&log, 0x00fe_0010),
            vec![vec![item.addr(), 0x8000_0000]]
        );

        // the player reference, a null side, a persistent item, or a virtual
        // `+0x160` that holds: nothing
        for (h, i, persists, veto) in [
            (player.addr(), item.addr(), 0, 0),
            (0, item.addr(), 0, 0),
            (holder.addr(), 0, 0, 0),
            (holder.addr(), item.addr(), 1, 0),
            (holder.addr(), item.addr(), 0, 1),
        ] {
            returns(&mut e, GET_REF_PERSISTS, persists);
            returns(&mut e, 0x00fe_0011, veto);
            e.call_log = Some(vec![]);
            e.call(0x0056_acb0, &args![h, i]);
            assert!(calls_to(&e.call_log.take().unwrap(), 0x0041_e000).is_empty());
        }
    }

    // ---- 0056ad30 .. 0056ae10 ------------------------------------------

    #[test]
    fn package_start_location_forms_are_filtered_by_type() {
        let mut e = engine();
        let this = object(&mut e);
        let record = e.mem.alloc(0x20);
        returns(&mut e, 0x0041_8b70, record);
        let world = form(&mut e, 0x41);
        let cell = form(&mut e, 0x39);

        e.mem.set_u32(record, world);
        assert_eq!(e.call(0x0056_ad30, &args![this]).u32(), world);
        assert_eq!(e.call(0x0056_ad80, &args![this]).u32(), 0);
        e.mem.set_u32(record, cell);
        assert_eq!(e.call(0x0056_ad30, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0056_ad80, &args![this]).u32(), cell);

        // a record without a form, and no record at all
        e.mem.set_u32(record, 0);
        assert_eq!(e.call(0x0056_ad80, &args![this]).u32(), 0);
        returns(&mut e, 0x0041_8b70, 0);
        assert_eq!(e.call(0x0056_ad30, &args![this]).u32(), 0);
    }

    #[test]
    fn package_start_location_coord_falls_back_to_the_position() {
        let mut e = engine();
        let this = Ptr::<TESObjectREFR>::new(object_with_vtable(
            &mut e,
            0x100,
            &[(SLOT_GET_POSITION, 0x00fe_0020)],
        ));
        returns(&mut e, 0x00fe_0020, 0x9999);
        returns(&mut e, 0x0041_8b70, 0x3000);
        assert_eq!(e.call(0x0056_add0, &args![this]).u32(), 0x3004);
        returns(&mut e, 0x0041_8b70, 0);
        assert_eq!(e.call(0x0056_add0, &args![this]).u32(), 0x9999);
    }

    #[test]
    fn set_package_start_location_marks_the_reference_changed() {
        let mut e = engine();
        stub(&mut e, &[0x0041_ad00, 0x00fe_0030]);
        let this = Ptr::<TESObjectREFR>::new(object_with_vtable(
            &mut e,
            0x100,
            &[(SLOT_SET_CHANGED, 0x00fe_0030)],
        ));
        e.call_log = Some(vec![]);
        e.call(0x0056_ae10, &args![this, 0xaau32, 0xbbu32, 0xccu32, 1.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0041_ad00),
            vec![vec![this.addr() + 0x44, 0xaa, 0xbb, 0xcc, 1.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, 0x00fe_0030), vec![vec![this.addr(), 0x800]]);
    }

    // ---- 0056ae60 .. 0056afc0 ------------------------------------------

    /// A reference whose base object (type `kind`) has a vtable with the
    /// `+0xf8` test returning `test`.
    fn refr_with_base(e: &mut Engine, kind: u8, test: u32) -> (Ptr<TESObjectREFR>, u32) {
        let this = object(e);
        returns(e, 0x00fe_0040, test);
        let base = object_with_vtable(e, 0x100, &[(SLOT_BASE_FORM_TEST, 0x00fe_0040)]);
        e.mem.set_u8(base + 4, kind);
        e.mem.set_u32(this.addr() + 0x20, base);
        (this, base)
    }

    #[test]
    fn fn_0056af40_and_fn_0056afc0_test_the_base_object_and_the_extra_flag() {
        let mut e = engine();
        // a reference without a base object
        let bare = object(&mut e);
        assert_eq!(e.call(0x0056_af40, &args![bare]).u32(), 0);
        assert_eq!(e.call(0x0056_afc0, &args![bare]).u32(), 0);

        let (this, base) = refr_with_base(&mut e, 0x1b, 1);
        returns(&mut e, 0x0042_16d0, 1);
        returns(&mut e, 0x0047_cdb0, 0);
        assert_eq!(e.call(0x0056_af40, &args![this]).u32(), 1);
        assert_eq!(e.call(0x0056_afc0, &args![this]).u32(), 1);

        // the extra flag clear: 0056af40 asks the base object instead
        returns(&mut e, 0x0042_16d0, 0);
        assert_eq!(e.call(0x0056_afc0, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0056_af40, &args![this]).u32(), 0);
        e.call_log = Some(vec![]);
        returns(&mut e, 0x0047_cdb0, 1);
        assert_eq!(e.call(0x0056_af40, &args![this]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0047_cdb0), vec![vec![base + 0x30]]);

        // the base object's own test fails
        let (this, _) = refr_with_base(&mut e, 0x1b, 0);
        assert_eq!(e.call(0x0056_af40, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0056_afc0, &args![this]).u32(), 0);
    }

    #[test]
    fn fn_0056af00_and_fn_0056af20_test_bits() {
        let mut e = engine();
        returns(&mut e, 0x0046_1580, 1);
        let object = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_af00, &args![object]).u32(), 1);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x0046_1580),
            vec![vec![object, 8]]
        );
        assert_eq!(e.call(0x0056_af20, &args![object]).u32(), 0);
        e.mem.set_u8(object + 0x98, 0x82);
        assert_eq!(e.call(0x0056_af20, &args![object]).u32(), 1);
        e.mem.set_u8(object + 0x98, 0x81);
        assert_eq!(e.call(0x0056_af20, &args![object]).u32(), 0);
    }

    #[test]
    fn fn_0056ae60_depends_on_the_base_form_type() {
        let mut e = engine();
        returns(&mut e, 0x0042_16d0, 0);
        returns(&mut e, 0x0047_cdb0, 0);
        returns(&mut e, 0x0046_1580, 1);

        // type 0x1b: bit 2 of the byte at +0x98
        let (this, base) = refr_with_base(&mut e, 0x1b, 1);
        e.mem.set_u8(base + 0x98, 2);
        assert_eq!(e.call(0x0056_ae60, &args![this]).u32(), 1);
        e.mem.set_u8(base + 0x98, 0);
        assert_eq!(e.call(0x0056_ae60, &args![this]).u32(), 0);

        // types 0x2a and 0x2b: 00461580(base + 0x30, 8); 0x29 and 0x2c: false
        e.call_log = Some(vec![]);
        let (this, base) = refr_with_base(&mut e, 0x2b, 1);
        assert_eq!(e.call(0x0056_ae60, &args![this]).u32(), 1);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x0046_1580),
            vec![vec![base + 0x30, 8]]
        );
        let (this, _) = refr_with_base(&mut e, 0x2a, 1);
        assert_eq!(e.call(0x0056_ae60, &args![this]).u32(), 1);
        for kind in [0x29, 0x2c] {
            let (this, _) = refr_with_base(&mut e, kind, 1);
            assert_eq!(e.call(0x0056_ae60, &args![this]).u32(), 0);
        }

        // when 0056af40 holds the answer is false whatever the type
        returns(&mut e, 0x0042_16d0, 1);
        let (this, base) = refr_with_base(&mut e, 0x1b, 1);
        e.mem.set_u8(base + 0x98, 2);
        assert_eq!(e.call(0x0056_ae60, &args![this]).u32(), 0);
        // and without a base object
        let bare = object(&mut e);
        assert_eq!(e.call(0x0056_ae60, &args![bare]).u32(), 0);
    }

    // ---- 0056b020, 0056b0b0 --------------------------------------------

    #[test]
    fn fn_0056b020_picks_the_change_call_by_its_arguments() {
        let mut e = engine();
        stub(&mut e, &[0x0048_4b90, 0x0042_1750, 0x00fe_0050]);
        let base_test = 0x00fe_0040;
        returns(&mut e, base_test, 1);
        let this = Ptr::<TESObjectREFR>::new(object_with_vtable(
            &mut e,
            0x100,
            &[(SLOT_FORCE_CHANGED, 0x00fe_0050)],
        ));
        let base = object_with_vtable(&mut e, 0x100, &[(SLOT_BASE_FORM_TEST, base_test)]);
        e.mem.set_u32(this.addr() + 0x20, base);

        e.call_log = Some(vec![]);
        e.call(0x0056_b020, &args![this, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0048_4b90),
            vec![vec![this.addr(), 0x40000]]
        );
        assert!(calls_to(&log, 0x00fe_0050).is_empty());
        assert_eq!(
            calls_to(&log, 0x0042_1750),
            vec![vec![this.addr() + 0x44, 5, 6]]
        );

        e.call_log = Some(vec![]);
        e.call(0x0056_b020, &args![this, 5u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0048_4b90).is_empty());
        assert_eq!(
            calls_to(&log, 0x00fe_0050),
            vec![vec![this.addr(), 0x40000]]
        );
        assert_eq!(calls_to(&log, 0x0042_1750).len(), 1);

        // the base object's test fails: nothing
        returns(&mut e, base_test, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_b020, &args![this, 5u32, 6u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0042_1750).is_empty());
    }

    #[test]
    fn restore_leveled_creature_original_base_swaps_the_base_object() {
        let mut e = engine();
        map_globals(&mut e);
        let table = e.mem.alloc(0x10);
        e.set_global(0x011c_3f2c, table);
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        returns(&mut e, 0x0046_9860, 1);
        // 0056af40 holds through the extra flag
        returns(&mut e, 0x0042_16d0, 1);
        returns(&mut e, 0x0042_16f0, 0x7e57);
        stub(&mut e, &[0x00fe_0060]);
        returns(&mut e, 0x00fe_0040, 1);
        let this = object(&mut e);
        let base = object_with_vtable(
            &mut e,
            0x100,
            &[(SLOT_BASE_FORM_TEST, 0x00fe_0040), (0x10, 0x00fe_0060)],
        );
        e.mem.set_u32(base + 0xc, 0x00ab_cdef);
        e.mem.set_u32(this.addr() + 0x20, base);

        e.call_log = Some(vec![]);
        e.call(0x0056_b0b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0046_9860), vec![vec![table, 0x00ab_cdef]]);
        assert_eq!(calls_to(&log, 0x00fe_0060), vec![vec![base, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 0x20), 0x7e57);

        // the form ID is refused: untouched
        e.mem.set_u32(this.addr() + 0x20, base);
        returns(&mut e, 0x0046_9860, 0);
        e.call(0x0056_b0b0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x20), base);

        // no base object at all
        e.mem.set_u32(this.addr() + 0x20, 0);
        returns(&mut e, 0x0046_9860, 1);
        e.call(0x0056_b0b0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x20), 0);
    }

    // ---- 0056b140 .. 0056b250 ------------------------------------------

    #[test]
    fn extra_record_readers_require_a_zero_flag_word() {
        let mut e = engine();
        let this = object(&mut e);
        let record = e.mem.alloc(0x20);
        returns(&mut e, 0x0041_82e0, record);
        e.mem.set_f32(record, 1.5);
        e.mem.set_u32(record + 4, 0);
        e.mem.set_f32(record + 8, 2.5);
        e.mem.set_u32(record + 0xc, 0xfeed);

        assert_eq!(e.call(0x0056_b140, &args![this]).u32(), 0xfeed);
        assert_eq!(e.call(0x0056_b190, &args![this]).f32(), 1.5);
        assert_eq!(e.call(0x0056_b1d0, &args![this]).f32(), 2.5);
        assert_eq!(e.call(0x0056_b210, &args![this]).u32(), 0);

        // a non-zero word at +4: the first three read as zero, the last
        // returns it
        e.mem.set_u32(record + 4, 7);
        assert_eq!(e.call(0x0056_b140, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0056_b190, &args![this]).f32(), 0.0);
        assert_eq!(e.call(0x0056_b1d0, &args![this]).f32(), 0.0);
        assert_eq!(e.call(0x0056_b210, &args![this]).u32(), 7);

        // no record
        returns(&mut e, 0x0041_82e0, 0);
        assert_eq!(e.call(0x0056_b140, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0056_b190, &args![this]).f32(), 0.0);
        assert_eq!(e.call(0x0056_b210, &args![this]).u32(), 0);
    }

    #[test]
    fn fn_0056b250_is_true_for_big_non_actor_objects() {
        let mut e = engine();
        map_globals(&mut e);
        e.set_global(0x0103_00a8, 700.0f64);
        stub(&mut e, &[0x00fe_0070]);
        returns(&mut e, 0x0056_4e60, 0);
        returns(&mut e, 0x0044_4ed0, 0);
        returns_float(&mut e, 0x0050_ebf0, 701.0);
        returns(&mut e, 0x00fe_0071, 0);
        let this = Ptr::<TESObjectREFR>::new(object_with_vtable(
            &mut e,
            0x100,
            &[(SLOT_IS_ACTOR, 0x00fe_0071)],
        ));
        let base = e.mem.alloc(0x100);
        e.mem.set_u32(this.addr() + 0x20, base);

        assert_eq!(e.call(0x0056_b250, &args![this]).u32(), 1);
        returns_float(&mut e, 0x0050_ebf0, 700.0);
        assert_eq!(e.call(0x0056_b250, &args![this]).u32(), 0);
        returns_float(&mut e, 0x0050_ebf0, 701.0);

        // an actor, or 00444ed0, or a missing base object: false
        returns(&mut e, 0x00fe_0071, 1);
        assert_eq!(e.call(0x0056_b250, &args![this]).u32(), 0);
        returns(&mut e, 0x00fe_0071, 0);
        returns(&mut e, 0x0044_4ed0, 1);
        assert_eq!(e.call(0x0056_b250, &args![this]).u32(), 0);
        returns(&mut e, 0x0044_4ed0, 0);
        e.mem.set_u32(this.addr() + 0x20, 0);
        assert_eq!(e.call(0x0056_b250, &args![this]).u32(), 0);

        // 00564e60 short-circuits to true
        returns(&mut e, 0x0056_4e60, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_b250, &args![this]).u32(), 1);
        assert_eq!(addresses(&e.call_log.take().unwrap()).len(), 2);
    }

    // ---- 0056c760 .. 0056c8b0, 0056d230 --------------------------------

    #[test]
    fn fn_0056c760_and_fn_0056c780_test_and_change_the_form_flag() {
        let mut e = engine();
        let this = object(&mut e);
        e.mem.set_u32(this.addr() + 8, 0x0000_0020);
        assert_eq!(e.call(0x0056_c760, &args![this]).u32(), 0);
        e.call(0x0056_c780, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x0800_0020);
        assert_eq!(e.call(0x0056_c760, &args![this]).u32(), 1);
        e.call(0x0056_c780, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x0000_0020);
    }

    #[test]
    fn fn_0056c7d0_and_fn_0056c7f0_use_the_fields_at_0xcc_and_0x2c() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.call(0x0056_c7d0, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0xcc), 0x1234);
        e.mem.set_u32(this + 0x2c, 0x5678);
        assert_eq!(e.call(0x0056_c7f0, &args![this]).u32(), 0x5678);
    }

    #[test]
    fn add_joined_multi_bound_appends_a_ni_pointer_to_the_list() {
        let mut e = engine();
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        stub(&mut e, &[0x0057_c590, NI_POINTER_DESTRUCT]);
        let room = e.mem.alloc(0x120);
        e.call_log = Some(vec![]);
        e.call(0x0056_c810, &args![room, 0xbeefu32]);
        let log = e.call_log.take().unwrap();
        let temp = calls_to(&log, NI_POINTER_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, NI_POINTER_CONSTRUCT),
            vec![vec![temp, 0xbeef]]
        );
        assert_eq!(calls_to(&log, 0x0057_c590), vec![vec![room + 0xd0, temp]]);
        assert_eq!(calls_to(&log, NI_POINTER_DESTRUCT), vec![vec![temp]]);
        // the temporary is gone
        assert_eq!(e.mem.block_size(temp), None);
    }

    #[test]
    fn fn_0056c880_and_fn_0056c8b0_set_and_clear_loaded_data_flags() {
        let mut e = engine();
        let this = object(&mut e);
        let loaded = e.mem.alloc(0x1c);
        // without loaded data nothing happens
        e.call(0x0056_c880, &args![this, 4u32, 1u32]);
        e.mem.set_u32(this.addr() + 0x64, loaded);
        e.call(0x0056_c880, &args![this, 4u32, 1u32]);
        e.call(0x0056_c880, &args![this, 2u32, 7u32]);
        assert_eq!(e.mem.u32(loaded + 0x10), 6);
        e.call(0x0056_c880, &args![this, 4u32, 0u32]);
        assert_eq!(e.mem.u32(loaded + 0x10), 2);
        e.call(0x0056_c8b0, &args![loaded, 0xf0u32, 1u32]);
        assert_eq!(e.mem.u32(loaded + 0x10), 0xf2);
        e.call(0x0056_c8b0, &args![loaded, 0x30u32, 0u32]);
        assert_eq!(e.mem.u32(loaded + 0x10), 0xc2);
    }

    #[test]
    fn fn_0056d230_converts_the_local_rotation_to_a_quaternion() {
        let mut e = engine();
        stub(&mut e, &[0x00a6_df40]);
        let node = e.mem.alloc(0x100);
        let out = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0056_d230, &args![node, out]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x00a6_df40),
            vec![vec![out, node + 0x34]]
        );
    }

    // ---- Load3D ---------------------------------------------------------

    /// Every callee of `Load3D` and `InitHavokForPlaceableWater` outside this
    /// file.
    const OUTSIDE_CALLEES: &[u32] = &[
        0x0040_1170,
        0x0040_30b0,
        0x0040_6d00,
        0x0040_77c0,
        0x0040_f6e0,
        0x0041_3f90,
        0x0041_6870,
        0x0041_8890,
        0x0041_a250,
        0x0041_b6b0,
        0x0041_f7e0,
        0x0041_f810,
        0x0041_fbe0,
        0x0042_0810,
        0x0042_08b0,
        0x0042_2a40,
        0x0042_2c20,
        0x0042_ce10,
        0x0043_0830,
        0x0043_7f90,
        0x0043_9ef0,
        0x0043_a2b0,
        0x0043_b230,
        0x0043_b560,
        0x0043_d410,
        0x0043_d450,
        0x0043_fa80,
        0x0043_fcd0,
        0x0044_0460,
        0x0044_0490,
        0x0044_0d80,
        0x0044_0da0,
        0x0044_1780,
        0x0044_17c0,
        0x0044_8a20,
        0x0044_8a80,
        0x0045_0b80,
        0x0045_0f90,
        0x0045_1530,
        0x0045_2370,
        0x0045_2440,
        0x0045_43c0,
        0x0045_c650,
        0x0045_cd60,
        0x0045_ce40,
        0x0045_cec0,
        0x0047_5400,
        0x0047_6ab0,
        0x0047_6f50,
        0x0047_7560,
        0x0047_7780,
        0x0048_3710,
        0x0048_7f50,
        0x0048_afe0,
        0x0048_f7f0,
        0x0049_1040,
        0x0049_1180,
        0x0049_3900,
        0x0049_4710,
        0x0049_55c0,
        0x0049_c680,
        0x004a_0c90,
        0x004a_3e00,
        0x004a_8b00,
        0x004b_4500,
        0x004b_5b20,
        0x004b_7660,
        0x004b_9930,
        0x004e_8030,
        0x004e_d8c0,
        0x004f_24e0,
        0x004f_2640,
        0x0050_8100,
        0x0050_8d70,
        0x0051_9230,
        0x0052_7080,
        0x0054_4c30,
        0x0054_4c60,
        0x0054_71e0,
        0x0055_1480,
        0x0055_9450,
        0x0055_d6d0,
        0x0056_1500,
        0x0056_4cd0,
        0x0056_4f00,
        0x0056_5050,
        0x0056_59f0,
        0x0056_7400,
        0x0056_8e50,
        0x0056_9920,
        0x0056_99b0,
        0x0056_a4a0,
        0x0056_d250,
        0x0056_d280,
        0x0056_d2c0,
        0x0056_d300,
        0x0056_d320,
        0x0056_d360,
        0x0056_d380,
        0x0056_d580,
        0x0056_d5f0,
        0x0056_d730,
        0x0056_fa00,
        0x0057_0f70,
        0x0057_1630,
        0x0057_22c0,
        0x0057_25f0,
        0x0057_6d50,
        0x0057_7330,
        0x0057_7e20,
        0x0057_8300,
        0x0057_84b0,
        0x0057_84f0,
        0x0057_a3c0,
        0x0057_bd80,
        0x0057_c180,
        0x0057_c6a0,
        0x0059_bb30,
        0x005b_5e40,
        0x005d_43c0,
        0x005f_3780,
        0x0060_56f0,
        0x0062_40d0,
        0x0062_c3c0,
        0x0063_3c90,
        0x0063_c8a0,
        0x0066_b0d0,
        0x0068_15c0,
        0x006c_0720,
        0x006c_0c30,
        0x006f_a820,
        0x0070_ec90,
        0x0072_6070,
        0x007a_f430,
        0x0082_2510,
        0x0084_e3a0,
        0x008b_0bd0,
        0x008d_4ab0,
        0x008d_4b60,
        0x008d_6f30,
        0x008d_8520,
        0x008d_a1a0,
        0x0093_7090,
        0x0098_adb0,
        0x00a5_9c60,
        0x00a5_9d30,
        0x00a5_a040,
        0x00a5_bca0,
        0x00a5_bdd0,
        0x00aa_13e0,
        0x00ad_8ce0,
        0x00ad_8f20,
        0x00b5_5b80,
        0x00b5_eed0,
        0x00ba_9120,
        0x00ba_91a0,
        0x00c4_2f70,
        0x00c6_a040,
        0x00c6_b980,
        0x00c6_bd00,
        0x00c7_c150,
        0x00c8_5c10,
        0x00c8_f510,
        0x00c9_b670,
        0x00c9_d990,
        0x00c9_ddb0,
        0x00ec_408c,
        0x00ec_43fb,
    ];

    /// The `TESObjectREFR` vtable slots the 3D code calls; each points at a
    /// double `0x00fd0000 + slot` that returns 0 until a test says otherwise.
    const REFR_SLOTS: &[u32] = &[
        0x48, 0xb8, 0x100, 0x130, 0x150, 0x15c, 0x178, 0x1b8, 0x1d0, 0x1d4, 0x1d8, 0x1dc, 0x1e4,
        0x1f4, 0x22c, 0x388, 0x38c,
    ];

    fn slot_double(slot: u32) -> u32 {
        0x00fd_0000 + slot
    }

    /// An engine where every outside callee is a do-nothing double, the
    /// accessors have their real bodies and the `NiPointer` functions work.
    fn load_engine() -> Engine {
        let mut e = Engine::new();
        map_globals(&mut e);
        stub(&mut e, OUTSIDE_CALLEES);
        for slot in REFR_SLOTS {
            stub(&mut e, &[slot_double(*slot)]);
        }
        install_accessors(&mut e);
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e
    }

    /// A reference with every slot in `REFR_SLOTS`, a base object of form
    /// type `kind` and form ID `id`.
    fn load_refr(e: &mut Engine, kind: u8) -> Ptr<TESObjectREFR> {
        let slots: Vec<(u32, u32)> = REFR_SLOTS.iter().map(|s| (*s, slot_double(*s))).collect();
        let this = object_with_vtable(e, 0x100, &slots);
        let base = base_object(e, kind);
        e.mem.set_u32(this + 0x20, base);
        e.mem.set_u32(this + 0xc, 0x00aa_0001);
        Ptr::new(this)
    }

    /// A base object with the vtable slots `Load3D` calls (`+0xac`, `+0x178`
    /// at `0x00fc0000 + slot`).
    fn base_object(e: &mut Engine, kind: u8) -> u32 {
        stub(e, &[0x00fc_00ac, 0x00fc_0178]);
        let base = object_with_vtable(e, 0x100, &[(0xac, 0x00fc_00ac), (0x178, 0x00fc_0178)]);
        e.mem.set_u8(base + 4, kind);
        base
    }

    /// A node with `IsNode` (`+0xc`) answering itself, `IsFadeNode` (`+0x10`)
    /// answering `fade`, and `+0xbc` a no-op.
    fn node_object(e: &mut Engine, fade: u32) -> u32 {
        stub(e, &[0x00fb_00bc]);
        let node = object_with_vtable(
            e,
            0x200,
            &[(0xc, 0x00fb_000c), (0x10, 0x00fb_0010), (0xbc, 0x00fb_00bc)],
        );
        returns(e, 0x00fb_000c, node);
        returns(e, 0x00fb_0010, fade);
        node
    }

    fn address_log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()
    }

    #[test]
    fn load_3d_gives_up_on_a_disabled_reference() {
        let mut e = load_engine();
        let this = load_refr(&mut e, 0x10);
        returns(&mut e, 0x0044_0da0, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_b2d0, &args![this, 0u32]).u32(), 0);
        let log = address_log(&mut e);
        assert_eq!(
            addresses(&log),
            vec![0x0056_b2d0, 0x0044_0da0, 0x0042_ce10, 0x0044_0d80]
        );

        // `00440d80` alone stops it too
        returns(&mut e, 0x0044_0da0, 0);
        returns(&mut e, 0x0044_0d80, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_b2d0, &args![this, 0u32]).u32(), 0);
        assert_eq!(addresses(&address_log(&mut e)).len(), 4);
    }

    #[test]
    fn load_3d_without_a_base_object_only_runs_the_tail() {
        let mut e = load_engine();
        let this = load_refr(&mut e, 0x10);
        e.mem.set_u32(this.addr() + 0x20, 0);
        // reachable through the tail: the nav mesh obstacle, one shader flag
        returns(&mut e, slot_double(0x15c), 1);
        returns(&mut e, slot_double(0xb8), 1);
        returns(&mut e, 0x006c_0720, 0x1111);
        returns(&mut e, 0x0043_fcd0, 0x2222);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_b2d0, &args![this, 0u32]).u32(), 0x2222);
        let log = address_log(&mut e);
        // the model went into the NiPointer, which is read back at the end
        let slot = calls_to(&log, NI_POINTER_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, NI_POINTER_CONSTRUCT),
            vec![vec![slot, 0x2222]]
        );
        assert_eq!(calls_to(&log, 0x0082_2510), vec![vec![slot, 0]]);
        assert_eq!(calls_to(&log, 0x006c_0c30), vec![vec![0x1111, this.addr()]]);
        assert_eq!(
            calls_to(&log, SET_FLAG_RECURSE),
            vec![vec![0x2222, 0x32, 1]]
        );
        assert_eq!(calls_to(&log, 0x0055_d6d0), vec![vec![this.addr(), 0x2222]]);
        assert_eq!(calls_to(&log, NI_POINTER_DESTRUCT), vec![vec![slot]]);
        assert_eq!(e.mem.block_size(slot), None);
        // no body: no loaded-data flags, no 3D call
        assert!(calls_to(&log, 0x0057_0f70).is_empty());
    }

    /// A reference with a primitive whose `NiPointer` at +0x2c holds `node`,
    /// and a loaded-data block; the model is missing.
    fn primitive_scene(e: &mut Engine, kind: u8) -> (Ptr<TESObjectREFR>, u32, u32, u32) {
        let this = load_refr(e, kind);
        let node = node_object(e, 1);
        let primitive = object_with_vtable(e, 0x100, &[(4, 0x00fe_0004), (8, 0x00fe_0008)]);
        stub(e, &[0x00fe_0004]);
        e.mem.set_u32(primitive + 0x2c, node);
        returns(e, 0x0041_fbe0, primitive);
        let loaded = e.mem.alloc(0x1c);
        e.mem.set_u32(this.addr() + 0x64, loaded);
        (this, node, primitive, loaded)
    }

    #[test]
    fn load_3d_attaches_the_primitive_node() {
        let mut e = load_engine();
        let (this, node, _, loaded) = primitive_scene(&mut e, 0x10);
        // the class ID check: `00413f90` fills 16 bytes, the game clears the
        // last dword before virtual `+8` sees it
        e.register(0x0041_3f90, |e, a| {
            for word in 0..4 {
                e.mem.set_u32(a[1] + 4 * word, 0xdead_beef);
            }
            Ret::default()
        });
        returns(&mut e, 0x004b_9930, 1);
        let seen = Rc::new(Cell::new(0xffff_ffffu32));
        let probe = seen.clone();
        e.register_double(0x00fe_0008, move |e, a| {
            probe.set(e.mem.u32(a[1] + 12));
            Ret::default()
        });
        returns(&mut e, 0x0057_84b0, 1);

        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_b2d0, &args![this, 0u32]).u32(), node);
        let log = address_log(&mut e);
        assert_eq!(seen.get(), 0);
        // flag 0: the 3D is registered through 00570f70
        assert_eq!(calls_to(&log, 0x0057_0f70), vec![vec![this.addr(), node]]);
        assert!(calls_to(&log, 0x0044_1780).is_empty());
        // the loaded-data flags 4 and 2 (the addon flag)
        assert_eq!(e.mem.u32(loaded + 0x10), 6);
        // the base object differs from the global (0): 00450f90 on the node
        assert_eq!(calls_to(&log, 0x0045_0f90)[0], vec![node, 1]);
        // the fade node learns its reference
        assert_eq!(e.mem.u32(node + 0xcc), this.addr());
        // the primitive's virtual +4 ran, then the node was assigned
        assert!(addresses(&log).contains(&0x00fe_0004));
        assert_eq!(calls_to(&log, NI_POINTER_ASSIGN)[0][1], node);
        assert_eq!(calls_to(&log, 0x0055_d6d0), vec![vec![this.addr(), node]]);
    }

    #[test]
    fn load_3d_without_a_primitive_asks_the_base_object_and_honours_the_flag() {
        let mut e = load_engine();
        let this = load_refr(&mut e, 0x10);
        let node = node_object(&mut e, 0);
        // no primitive: the base object's virtual +0x178 loads the node
        returns(&mut e, 0x00fc_0178, node);
        returns(&mut e, 0x0057_84b0, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_b2d0, &args![this, 1u32]).u32(), node);
        let log = address_log(&mut e);
        let base = e.mem.u32(this.addr() + 0x20);
        assert_eq!(calls_to(&log, 0x00fc_0178), vec![vec![base, this.addr()]]);
        // flag 1: the TLS record and the node's 0040f6e0, then 004417c0 at the end
        assert_eq!(calls_to(&log, 0x0044_1780), vec![vec![this.addr(), node]]);
        assert_eq!(calls_to(&log, 0x0040_f6e0), vec![vec![node]]);
        assert!(calls_to(&log, 0x0057_0f70).is_empty());
        assert_eq!(calls_to(&log, 0x0044_17c0).len(), 1);
        // IsFadeNode is false: no field written
        assert_eq!(e.mem.u32(node + 0xcc), 0);
    }

    #[test]
    fn load_3d_updates_an_actors_animation() {
        let mut e = load_engine();
        let (this, node, _, _) = primitive_scene(&mut e, 0x2b);
        returns(&mut e, slot_double(0x100), 1);
        returns(&mut e, 0x008d_4ab0, 1);
        let animation = e.mem.alloc(0x100);
        returns(&mut e, slot_double(0x1e4), animation);
        returns(&mut e, slot_double(0x22c), 1);
        returns(&mut e, 0x0049_4710, 1);
        let section = e.mem.alloc(0x40);
        returns(&mut e, 0x0049_1040, section);
        returns_float(&mut e, 0x0050_8100, 0.25);

        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_b2d0, &args![this, 0u32]).u32(), node);
        let log = address_log(&mut e);
        let me = this.addr();
        // the creature (form type 0x2b) gets virtual +0x388 and the limb check
        assert_eq!(calls_to(&log, 0x008d_4ab0), vec![vec![me, node]]);
        assert_eq!(calls_to(&log, slot_double(0x388)), vec![vec![me, 1]]);
        assert_eq!(calls_to(&log, 0x008d_4b60), vec![vec![me]]);
        assert_eq!(
            calls_to(&log, 0x0049_55c0),
            vec![vec![
                animation,
                0x14,
                0xe0,
                0xffff_ffff,
                0.0f32.to_bits(),
                0xffff_ffff
            ]]
        );
        // the ordinary update path
        assert_eq!(
            calls_to(&log, 0x0098_adb0),
            vec![vec![section, 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, 0x0049_1180),
            vec![vec![animation, me, 0.0f32.to_bits(), 0.25f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, 0x0049_3900), vec![vec![animation, me]]);
        assert!(calls_to(&log, 0x00c9_b670).is_empty());
        // the shared actor tail
        assert_eq!(calls_to(&log, 0x0056_a4a0), vec![vec![me, 0]]);
        assert_eq!(calls_to(&log, 0x0057_6d50), vec![vec![me, node]]);
        assert_eq!(calls_to(&log, 0x0042_2a40), vec![vec![me + 0x44, me]]);
    }

    #[test]
    fn load_3d_knocks_a_dead_actor_down() {
        let mut e = load_engine();
        let (this, node, _, _) = primitive_scene(&mut e, 0x10);
        returns(&mut e, slot_double(0x100), 1);
        let animation = e.mem.alloc(0x100);
        returns(&mut e, slot_double(0x1e4), animation);
        returns(&mut e, slot_double(0x22c), 1);
        returns(&mut e, 0x0049_4710, 1);
        let section = e.mem.alloc(0x40);
        returns(&mut e, 0x0049_1040, section);
        // virtual +0x38c holds: the knock-down path
        returns(&mut e, slot_double(0x38c), 1);
        returns(&mut e, 0x0048_f7f0, 0x6060);
        returns_float(&mut e, 0x005f_3780, 0.5);
        let heading = e.mem.alloc(0x20);
        e.mem.set_f32(heading + 8, 1.25);
        returns(&mut e, 0x0043_0830, heading);
        // the rotated direction the game copies into the knock-down vector
        let vector = e.mem.alloc(0x10);
        e.mem.set_f32(vector, 0.5);
        e.mem.set_f32(vector + 4, 0.25);
        e.mem.set_f32(vector + 8, 0.125);
        returns(&mut e, 0x004b_4500, vector);
        let ragdoll_controller = e.mem.alloc(0x10);
        let actor = this.addr();
        e.mem.set_u32(actor + 0xac, ragdoll_controller);
        let direction = Rc::new(RefCell::new(Vec::new()));
        let probe = direction.clone();
        e.register_double(0x00c9_b670, move |e, a| {
            probe.borrow_mut().extend([
                e.mem.f32(a[1]).to_bits(),
                e.mem.f32(a[1] + 4).to_bits(),
                e.mem.f32(a[1] + 8).to_bits(),
            ]);
            Ret::default()
        });

        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_b2d0, &args![this, 0u32]).u32(), node);
        let log = address_log(&mut e);
        assert_eq!(calls_to(&log, 0x0048_f7f0), vec![vec![section]]);
        assert_eq!(calls_to(&log, 0x005f_3780), vec![vec![0x6060, 1]]);
        assert_eq!(
            calls_to(&log, 0x0049_1180),
            vec![vec![animation, actor, 0.0f32.to_bits(), 0.5f32.to_bits()]]
        );
        // the heading angle goes into the rotation, (0, 1, 0) is rotated
        assert_eq!(calls_to(&log, 0x004a_0c90)[0][1], 1.25f32.to_bits());
        let unit = calls_to(&log, 0x0041_6870);
        assert_eq!(
            unit[0][1..],
            [0.0f32.to_bits(), 1.0f32.to_bits(), 0.0f32.to_bits()]
        );
        assert_eq!(
            calls_to(&log, 0x00c7_c150),
            vec![vec![ragdoll_controller, 1]]
        );
        assert_eq!(
            *direction.borrow(),
            vec![0.5f32.to_bits(), 0.25f32.to_bits(), 0.125f32.to_bits()]
        );
        assert_eq!(calls_to(&log, slot_double(0x48)), vec![vec![actor, 4]]);

        // a dead actor (005722c0 true) is left alone
        returns(&mut e, 0x0057_22c0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        let log = address_log(&mut e);
        assert!(calls_to(&log, 0x0048_f7f0).is_empty());
        assert!(calls_to(&log, slot_double(0x48)).is_empty());
    }

    /// A list node holding `item`.
    fn light_scene(e: &mut Engine, kind: u8) -> (Ptr<TESObjectREFR>, u32, u32, u32) {
        let this = load_refr(e, kind);
        let node = node_object(e, 0);
        // the light slot (a NiPointer) of the reference
        let light_slot = e.mem.alloc(4);
        e.mem.set_u32(light_slot, node);
        // the shadow-scene side
        let finder = e.mem.alloc(0x10);
        returns(e, 0x0070_ec90, finder);
        let record = e.mem.alloc(0x40);
        returns(e, 0x004e_8030, record);
        let owner = e.mem.alloc(0x40);
        returns(e, 0x0040_30b0, owner);
        returns(e, 0x008d_8520, 0xd);
        let property = e.mem.alloc(0x200);
        returns(e, 0x00a5_9d30, property);
        returns(e, 0x0045_0b80, 0x5c5c);
        (this, light_slot, property, record)
    }

    #[test]
    fn load_3d_takes_the_lights_of_a_light_reference_off_the_shadow_scene() {
        let mut e = load_engine();
        let (this, light_slot, property, record) = light_scene(&mut e, 0x1e);
        let me = this.addr();
        // lit-water references: one entry whose 3D exists
        let light_ref = Ptr::<TESObjectREFR>::new(object_with_vtable(
            &mut e,
            0x100,
            &[(SLOT_GET_3D, 0x00fe_0100)],
        ));
        returns(&mut e, 0x00fe_0100, 0x4141);
        let tail = list_node(&mut e, 0, 0);
        let head = list_node(&mut e, light_ref.addr(), tail);
        returns(&mut e, 0x0041_f810, head);
        returns(&mut e, 0x0057_25f0, light_slot);
        // 0049c680 finds nothing: the node is added to the property's set
        returns(&mut e, 0x0049_c680, 0);
        let node = e.mem.u32(light_slot);

        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        let log = address_log(&mut e);
        // the record is looked up by the light reference
        assert_eq!(calls_to(&log, 0x004e_8030)[0][1], light_ref.addr());
        assert_eq!(calls_to(&log, 0x00a5_9d30), vec![vec![record, 3]]);
        let set = calls_to(&log, 0x004e_d8c0);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][0], property + 0x128);
        assert_eq!(e.mem.u8(property + 0x83), 1);
        assert_eq!(calls_to(&log, 0x0049_c680)[0][0], property + 0x128);
        assert_eq!(calls_to(&log, 0x0049_c680)[0][2], 0);
        // the scene node is looked up with 0 and loses the light
        assert_eq!(calls_to(&log, 0x0045_0b80), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x00b5_eed0), vec![vec![0x5c5c, node]]);
        // the sound handle is built and destroyed
        assert_eq!(calls_to(&log, 0x0041_a250).len(), 1);
        assert_eq!(calls_to(&log, 0x0048_3710).len(), 1);
        assert_eq!(calls_to(&log, 0x0041_8890)[0][0], me + 0x44);

        // the property already knows the node: no add, no flag
        e.mem.set_u8(property + 0x83, 0);
        returns(&mut e, 0x0049_c680, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        let log = address_log(&mut e);
        assert!(calls_to(&log, 0x004e_d8c0).is_empty());
        assert_eq!(e.mem.u8(property + 0x83), 0);
        assert_eq!(calls_to(&log, 0x00b5_eed0).len(), 1);

        // an owner of another kind: the light stays
        returns(&mut e, 0x008d_8520, 0xc);
        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        assert!(calls_to(&address_log(&mut e), 0x00b5_eed0).is_empty());
    }

    #[test]
    fn load_3d_handles_the_lights_of_a_water_reference() {
        let mut e = load_engine();
        let (this, light_slot, property, _) = light_scene(&mut e, 0x23);
        let me = this.addr();
        // the reference itself needs a 3D root (virtual +0x1d0)
        returns(&mut e, slot_double(0x1d0), 0x4141);
        let light_ref = e.mem.alloc(0x100);
        let head = list_node(&mut e, light_ref, 0);
        returns(&mut e, 0x0041_f7e0, head);
        returns(&mut e, 0x0057_25f0, light_slot);
        returns(&mut e, 0x0049_c680, 0);
        // a parent cell holder for the texture touch
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(me + 0x40, cell);
        returns(&mut e, 0x0045_43c0, 0x1357);
        returns(&mut e, 0x0059_bb30, 0x2468);

        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        let log = address_log(&mut e);
        // the record is looked up by the water reference itself
        assert_eq!(calls_to(&log, 0x004e_8030)[0][1], me);
        assert_eq!(e.mem.u8(property + 0x83), 1);
        assert_eq!(calls_to(&log, 0x00b5_eed0).len(), 1);
        // the water texture touch: 0045ce40 on the global, 00937090 on the holder's result
        assert_eq!(calls_to(&log, 0x0045_ce40).len(), 1);
        assert_eq!(calls_to(&log, 0x0093_7090), vec![vec![0x2468, 1]]);

        // no 3D root: the light is not touched
        returns(&mut e, slot_double(0x1d0), 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        assert!(calls_to(&address_log(&mut e), 0x00b5_eed0).is_empty());
    }

    #[test]
    fn load_3d_joins_the_multibound_room_and_warns_about_missing_data() {
        let mut e = load_engine();
        let this = load_refr(&mut e, 0x23);
        let me = this.addr();
        // the room check of the NiPointer says "multibound"; the 3D exists and the
        // extra data list has no primitive, so the body is skipped
        let node = node_object(&mut e, 0);
        returns(&mut e, 0x0043_fcd0, node);
        returns(&mut e, 0x0082_2510, 1);
        returns(&mut e, 0x0042_08b0, 0x3003);
        returns(&mut e, 0x0056_9920, 0x4004);
        let room = e.mem.alloc(0x120);
        returns(&mut e, 0x0056_99b0, room);
        returns(&mut e, 0x0045_c650, 0x5005);
        // the room does not know the multibound yet
        returns(&mut e, 0x0057_c6a0, 0);
        stub(&mut e, &[0x0057_c590]);
        // the water has no multibound data: 0057bd80 says no
        returns(&mut e, 0x0057_bd80, 0);
        slot_value(&mut e, 0x130, 0x6006);
        e.mem.set_u32(me + 0xc, 0x00ab_cdef);
        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 1u32]);
        let log = address_log(&mut e);
        // the join went through AddJoinedMultiBound
        assert_eq!(calls_to(&log, 0x0057_c590)[0][0], room + 0xd0);
        assert_eq!(calls_to(&log, 0x0057_c6a0)[0][2], 0);
        // the pending 004417c0 for flag 1 and the log message
        assert_eq!(calls_to(&log, 0x0044_17c0).len(), 1);
        let text = calls_to(&log, 0x0040_6d00);
        assert_eq!(text.len(), 1);
        assert_eq!(
            text[0][1..],
            [0x100, WATER_WITHOUT_MULTIBOUND_FORMAT, 0x6006, 0x00ab_cdef]
        );
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![vec![WATER_LOG, text[0][0]]]
        );

        // already joined, master room flag set, and found data: quiet
        returns(&mut e, 0x0057_c6a0, 1);
        returns(&mut e, 0x0057_bd80, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        let log = address_log(&mut e);
        assert!(calls_to(&log, 0x0057_c590).is_empty());
        assert!(calls_to(&log, 0x0040_6d00).is_empty());
        returns(&mut e, 0x0042_0810, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        assert!(calls_to(&address_log(&mut e), 0x0042_08b0).is_empty());
    }

    fn slot_value(e: &mut Engine, slot: u32, value: u32) {
        returns(e, slot_double(slot), value);
    }

    #[test]
    fn load_3d_adds_the_destruction_models_and_texture_swap() {
        let mut e = load_engine();
        let (this, node, _, _) = primitive_scene(&mut e, 0x10);
        let me = this.addr();
        let base = e.mem.u32(me + 0x20);
        // an object with health and the 0x452370 flag: destructible handling
        returns(&mut e, 0x0045_2370, 1);
        returns_float(&mut e, 0x0041_b6b0, 3.0);
        e.set_global(0x0101_2060, 0.0f64);
        returns(&mut e, 0x0047_5400, 0x7777);
        returns(&mut e, 0x0050_8d70, 1);
        returns(&mut e, 0x0047_7560, 1);
        returns(&mut e, 0x0047_6f50, 1);
        // the model is a texture swap, and the base object asks for platform
        // language textures
        returns(&mut e, 0x0057_1630, 0x8888);
        slot_base(&mut e, base, 0xac, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        let log = address_log(&mut e);
        assert_eq!(calls_to(&log, 0x0047_7780), vec![vec![0x7777, base]]);
        assert_eq!(calls_to(&log, 0x0047_6f50), vec![vec![0x7777, me, 0]]);
        assert_eq!(calls_to(&log, 0x0042_2c20), vec![vec![me + 0x44]]);
        assert_eq!(calls_to(&log, 0x0057_a3c0), vec![vec![me, 0]]);
        assert_eq!(
            calls_to(&log, DYNAMIC_CAST),
            vec![vec![
                0x8888,
                0,
                TYPE_TES_MODEL,
                TYPE_TES_MODEL_TEXTURE_SWAP,
                0
            ]]
        );
        assert_eq!(calls_to(&log, 0x0048_afe0), vec![vec![0x8888, node]]);
        assert_eq!(calls_to(&log, 0x004b_7660), vec![vec![node]]);

        // zero health: no preloading of replacement models
        returns_float(&mut e, 0x0041_b6b0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x0056_b2d0, &args![this, 0u32]);
        assert!(calls_to(&address_log(&mut e), 0x0047_7780).is_empty());
    }

    fn slot_base(e: &mut Engine, base: u32, slot: u32, value: u32) {
        let vtable = e.mem.u32(base);
        let target = e.mem.u32(vtable + slot);
        returns(e, target, value);
    }

    // ---- InitHavokForPlaceableWater ------------------------------------

    type CallTrace = Vec<(u32, Vec<u32>)>;

    /// What the Havok setup reads and builds, with doubles that hand out
    /// distinct blocks.
    struct HavokScene {
        e: Engine,
        this: Ptr<TESObjectREFR>,
        holder: u32,
        node: u32,
        trace: Rc<RefCell<CallTrace>>,
    }

    fn havok_scene(box_kind: bool) -> HavokScene {
        let mut e = load_engine();
        stub(&mut e, &[0x00a6_df40]);
        e.set_global(0x0101_8bfc, 2048.0f32);
        e.set_global(0x0101_3d84, 500.0f32);
        e.set_global(0x011b_153c, 0.05f32);
        e.set_global(0x0103_01a8, 500.0f64);
        let this = load_refr(&mut e, 0x23);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x40, cell);
        let holder = 0x1357;
        returns(&mut e, 0x0045_43c0, holder);
        returns(&mut e, 0x0045_2440, u32::from(box_kind));
        let node = node_object(&mut e, 0);
        returns(&mut e, slot_double(0x1d0), node);
        returns(&mut e, slot_double(0x1f4), 0x9f9f);
        // allocators hand out zeroed blocks, constructors return `this`
        e.register(0x0056_d280, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00aa_13e0, |e, a| e.mem.alloc(a[0]).into_ret());
        for ctor in [
            0x0056_d5f0,
            0x00c9_d990,
            0x00c9_ddb0,
            0x0056_d380,
            0x0056_d580,
            0x00c4_2f70,
        ] {
            e.register(ctor, |_, a| a[0].into_ret());
        }
        returns(&mut e, 0x0044_8a80, 0xabab);
        // the vector constructor stores its three floats
        e.register(0x0041_6870, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            Ret::default()
        });
        // the trace records the calls with the interesting memory
        let trace = Rc::new(RefCell::new(Vec::new()));
        for addr in [
            0x0056_d300,
            0x0056_d320,
            0x0056_d730,
            0x0056_d2c0,
            0x00c6_b980,
            0x00c8_5c10,
        ] {
            let probe = trace.clone();
            e.register_double(addr, move |e, a| {
                // the first argument is a cinfo for the first three
                let mut words = a.to_vec();
                if matches!(addr, 0x0056_d300 | 0x0056_d320 | 0x0056_d730) {
                    words.push(e.mem.u32(a[0]));
                    words.push(e.mem.u32(a[0] + 4));
                }
                probe.borrow_mut().push((addr, words));
                Ret::default()
            });
        }
        HavokScene {
            e,
            this,
            holder,
            node,
            trace,
        }
    }

    #[test]
    fn init_havok_builds_a_box_body_for_a_box_water() {
        let mut s = havok_scene(true);
        // the body is an object whose virtual +0xe4 the box case calls
        stub(&mut s.e, &[0x00fe_00e4]);
        s.e.register(0x0056_d380, |e, a| {
            let vtable = e.mem.alloc(0x200);
            e.mem.set_u32(vtable + 0xe4, 0x00fe_00e4);
            e.mem.set_u32(a[0], vtable);
            a[0].into_ret()
        });
        s.e.register(0x0054_4c30, |_, _| 3u32.into_ret());
        s.e.register(0x0054_4c60, |_, _| (-2i32 as u32).into_ret());
        returns_float(&mut s.e, 0x0054_71e0, 100.0);
        // the vectors that 004a3e00 converts
        let converted = Rc::new(RefCell::new(Vec::new()));
        let probe = converted.clone();
        s.e.register_double(0x004a_3e00, move |e, a| {
            probe.borrow_mut().push([
                e.mem.f32(a[1]).to_bits(),
                e.mem.f32(a[1] + 4).to_bits(),
                e.mem.f32(a[1] + 8).to_bits(),
            ]);
            Ret::default()
        });
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_c8f0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();

        // the box half extents (2048, 2048, 500) go in through 004a3e00 first,
        // then the cell centre and the water height minus 500
        let vectors = converted.borrow();
        assert_eq!(
            vectors[0],
            [2048.0f32.to_bits(), 2048.0f32.to_bits(), 500.0f32.to_bits()]
        );
        assert_eq!(
            vectors[1],
            [
                (3.0f32 * 4096.0 + 2048.0).to_bits(),
                (-2.0f32 * 4096.0 + 2048.0).to_bits(),
                (-400.0f32).to_bits()
            ]
        );
        // the box shape gets the 0.05 convex radius
        let box_ctor = calls_to(&log, 0x00c9_ddb0);
        assert_eq!(box_ctor.len(), 1);
        assert_eq!(box_ctor[0][2], 0.05f32.to_bits());
        // the virtual +0x1d8/+0x1dc queries of the reference ran
        assert_eq!(calls_to(&log, slot_double(0x1d8)).len(), 1);
        assert_eq!(calls_to(&log, slot_double(0x1dc)).len(), 1);

        // the cinfo: shape wrapper at +4, filter info 0x5000b, motion 5
        let trace = s.trace.borrow();
        let find = |addr: u32| trace.iter().find(|(a, _)| *a == addr).unwrap().1.clone();
        let setter = find(0x0056_d300);
        let cinfo = setter[0];
        let wrapper = calls_to(&log, 0x00c9_d990)[0][0];
        assert_eq!(setter[3], wrapper);
        let destroyed = find(0x0056_d730);
        assert_eq!(destroyed[0], cinfo);
        assert_eq!(destroyed[1], WATER_COLLISION_FILTER_INFO);
        assert_eq!(destroyed[2], wrapper);
        assert_eq!(calls_to(&log, 0x0056_d360), vec![vec![cinfo, 5]]);

        // the body, the collision object, BSXFlags (the node has none), links
        let body = calls_to(&log, 0x0056_d380)[0][0];
        assert_eq!(calls_to(&log, 0x0056_d380), vec![vec![body, cinfo]]);
        let collision = calls_to(&log, 0x0056_d580)[0][0];
        assert_eq!(calls_to(&log, 0x0056_d580), vec![vec![collision, s.node]]);
        assert_eq!(calls_to(&log, 0x00a5_bdd0), vec![vec![s.node, 0xabab]]);
        let flags = calls_to(&log, 0x00c4_2f70)[0][0];
        assert_eq!(calls_to(&log, 0x00a5_bca0), vec![vec![s.node, flags]]);
        assert_eq!(calls_to(&log, 0x0051_9230), vec![vec![flags, 2, 1]]);
        assert_eq!(find(0x00c6_b980), vec![collision, body]);
        assert_eq!(find(0x00c8_5c10), vec![body, s.node, 0]);
        // box bodies are also flagged through virtual +0xe4; both finish with
        // the holder
        assert_eq!(find(0x0056_d2c0), vec![s.holder, body]);
        assert_eq!(calls_to(&log, 0x00fe_00e4), vec![vec![body, 1]]);
    }

    #[test]
    fn init_havok_box_water_follows_the_fpu_for_its_height_and_radius() {
        let mut s = havok_scene(true);
        stub(&mut s.e, &[0x00fe_00e4]);
        s.e.register(0x0056_d380, |e, a| {
            let vtable = e.mem.alloc(0x200);
            e.mem.set_u32(vtable + 0xe4, 0x00fe_00e4);
            e.mem.set_u32(a[0], vtable);
            a[0].into_ret()
        });
        s.e.register(0x0054_4c30, |_, _| 0u32.into_ret());
        s.e.register(0x0054_4c60, |_, _| 0u32.into_ret());
        // 2^27 - 500 needs a 25th bit: 24-bit precision rounding down gives
        // 134217224, not the nearest float 134217232.
        returns_float(&mut s.e, 0x0054_71e0, 134_217_728.0);
        // a signalling NaN radius, quieted by 0056ca11's FLD / FSTP dword
        s.e.set_global(0x011b_153c, f32::from_bits(0x7f80_0001));
        let heights = Rc::new(RefCell::new(Vec::new()));
        let probe = heights.clone();
        s.e.register_double(0x004a_3e00, move |e, a| {
            probe.borrow_mut().push(e.mem.u32(a[1] + 8));
            Ret::default()
        });
        s.e.call_log = Some(vec![]);
        crate::x87::set_control_word(0x047f);
        s.e.call(0x0056_c8f0, &args![s.this]);
        crate::x87::set_control_word(crate::x87::DEFAULT_CONTROL_WORD);
        let log = s.e.call_log.take().unwrap();
        assert_eq!(heights.borrow()[1], 134_217_224.0f32.to_bits());
        assert_eq!(calls_to(&log, 0x00c9_ddb0)[0][2], 0x7fc0_0001);
    }

    #[test]
    fn init_havok_reuses_the_shape_of_the_collision_object() {
        let mut s = havok_scene(false);
        // the reference's 3D has a collision object, with a shape in a NiPointer
        let shape = s.e.mem.alloc(0x40);
        let slot = s.e.mem.alloc(4);
        s.e.mem.set_u32(slot, shape);
        returns(&mut s.e, 0x004a_8b00, 0x2020);
        returns(&mut s.e, 0x006f_a820, 0x2121);
        returns(&mut s.e, 0x0043_b560, slot);
        returns(&mut s.e, 0x0043_b230, 5);
        let captured = Rc::new(RefCell::new(Vec::new()));
        let probe = captured.clone();
        s.e.register_double(0x0056_1500, move |_, a| {
            probe.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_c8f0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();

        assert_eq!(calls_to(&log, 0x004a_8b00), vec![vec![s.node]]);
        // the shape wrapper is built from the collision object's shape
        let wrapper = calls_to(&log, 0x00c9_d990);
        assert_eq!(wrapper.len(), 1);
        assert_eq!(wrapper[0][1], shape);
        // the node's local rotation, the position, and the combination
        let rotation = calls_to(&log, 0x00a6_df40);
        assert_eq!(rotation.len(), 1);
        assert_eq!(rotation[0][1], s.node + 0x34);
        assert_eq!(calls_to(&log, 0x004a_3e00)[0][1], 0x9f9f);
        assert_eq!(captured.borrow().len(), 1);
        // no virtual +0xe4 flagging for this kind, but the holder link
        let trace = s.trace.borrow();
        assert!(trace
            .iter()
            .any(|(a, w)| *a == 0x0056_d2c0 && w[0] == s.holder));
        let destroyed = trace.iter().find(|(a, _)| *a == 0x0056_d730).unwrap();
        assert_eq!(destroyed.1[1], WATER_COLLISION_FILTER_INFO);
        assert_eq!(destroyed.1[2], wrapper[0][0]);
    }

    #[test]
    fn init_havok_reports_what_is_missing() {
        // no collision object
        let mut s = havok_scene(false);
        returns(&mut s.e, 0x004a_8b00, 0);
        stub(&mut s.e, &[0x005b_5e40]);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_c8f0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![vec![0x0103_016c, 0x00aa_0001]]
        );
        assert!(calls_to(&log, 0x0056_d280).is_empty());

        // a collision object without a shape
        returns(&mut s.e, 0x004a_8b00, 1);
        returns(&mut s.e, 0x006f_a820, 2);
        let empty = s.e.mem.alloc(4);
        returns(&mut s.e, 0x0043_b560, empty);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_c8f0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![vec![0x0103_012c, 0x00aa_0001]]
        );

        // a shape of type 0x1b is skipped silently
        let shape = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(empty, shape);
        returns(&mut s.e, 0x0043_b230, 0x1b);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_c8f0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0056_d280).is_empty());
        assert!(calls_to(&log, 0x005b_5e40).is_empty());
    }

    #[test]
    fn init_havok_needs_a_parent_cell_holder_and_a_base_object() {
        let mut s = havok_scene(true);
        returns(&mut s.e, 0x0045_43c0, 0);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_c8f0, &args![s.this]);
        assert_eq!(addresses(&s.e.call_log.take().unwrap()).len(), 4);

        // no parent cell
        s.e.mem.set_u32(s.this.addr() + 0x40, 0);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_c8f0, &args![s.this]);
        assert_eq!(addresses(&s.e.call_log.take().unwrap()).len(), 2);
    }

    // ---- 0056d250 .. 0056e090: the Havok glue ------------------------------

    /// Maps the pages of the counters and constants this batch reads.
    fn havok_engine() -> Engine {
        let mut e = engine();
        for page in [0x010c_7000, 0x0126_8000] {
            e.map(page, 0x1000);
        }
        e
    }

    /// Doubles with the real bodies of the small vector helpers.
    fn install_vector_math(e: &mut Engine) {
        // 004a3e00: three floats to a 16-byte vector with w = 0
        e.register(0x004a_3e00, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            e.mem.set_f32(a[0] + 12, 0.0);
            a[0].into_ret()
        });
        // 004a3f10: 16-byte copy
        e.register(0x004a_3f10, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        // 00413fc0: the three floats at +0x18
        e.register(0x0041_3fc0, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[0] + 0x18 + 4 * word);
                e.mem.set_u32(a[1] + 4 * word, value);
            }
            a[1].into_ret()
        });
        // 00439e90: result = this + b
        e.register(0x0043_9e90, |e, a| {
            for word in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * word) + e.mem.f32(a[2] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, sum);
            }
            a[1].into_ret()
        });
        // 0045bb20: result = this * s
        e.register(0x0045_bb20, |e, a| {
            for word in 0..3 {
                let product = e.mem.f32(a[0] + 4 * word) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * word, product);
            }
            a[1].into_ret()
        });
        // 00439f50: column `index` of a row-major 3x3 matrix
        e.register(0x0043_9f50, |e, a| {
            for row in 0..3 {
                let value = e.mem.u32(a[0] + 4 * (a[1] + 3 * row));
                e.mem.set_u32(a[2] + 4 * row, value);
            }
            Ret::default()
        });
        // 004b4cf0: vector `index` of a Havok rotation
        e.register(0x004b_4cf0, |_, a| (a[0] + 16 * a[1]).into_ret());
        // 00553fc0: three floats to a 16-byte vector with w = 0
        e.register(0x0055_3fc0, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            e.mem.set_f32(a[0] + 12, 0.0);
            a[0].into_ret()
        });
    }

    fn floats(e: &Engine, addr: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(addr + 4 * i)).collect()
    }

    #[test]
    fn fn_0056d250_stores_the_constant_vector() {
        let mut e = havok_engine();
        for (i, v) in [0.0f32, 0.0, 0.0, 1.0].iter().enumerate() {
            e.mem.set_f32(0x010c_71b0 + 4 * i as u32, *v);
        }
        let target = e.mem.alloc(16);
        e.mem.set_u32(target, 0x7777);
        e.call(0x0056_d250, &args![target]);
        assert_eq!(floats(&e, target, 4), vec![0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn fn_0056d280_allocates_through_the_memory_router() {
        let mut e = havok_engine();
        returns(&mut e, 0x00c8_5750, 0x5000);
        e.register(0x0044_edb0, |e, a| {
            assert_eq!(a[0], 0x5000);
            let allocator = e.mem.alloc(0x20);
            let vtable = e.mem.alloc(0x20);
            e.mem.set_u32(vtable + 4, 0x00fe_0004);
            e.mem.set_u32(allocator, vtable);
            allocator.into_ret()
        });
        e.register(0x00fe_0004, |e, a| {
            assert_eq!(a[1], 0x1_0030);
            e.mem.alloc(0x40).into_ret()
        });
        let block = e.call(0x0056_d280, &args![0x1_0030u32]).u32();
        assert_eq!(e.mem.u16(block + 4), 0x0030);
    }

    #[test]
    fn fn_0056d2c0_asks_the_object_to_take_the_holder() {
        let mut e = havok_engine();
        returns(&mut e, 0x00fe_009c, 0x0101);
        let object = object_with_vtable(&mut e, 0x20, &[(0x9c, 0x00fe_009c)]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_d2c0, &args![0x1357u32, object]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00fe_009c), vec![vec![object, 0x1357]]);
        assert_eq!(e.call(0x0056_d2c0, &args![0x1357u32, 0u32]).u32(), 0);
    }

    #[test]
    fn fn_0056d300_copies_the_vector_to_offset_0x30() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004a_3f10]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d300, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004a_3f10), vec![vec![0x1030, 0x2000]]);
    }

    #[test]
    fn fn_0056d320_copies_the_vector_to_offset_0x40() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004a_3f10]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d320, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004a_3f10), vec![vec![0x1040, 0x2000]]);
    }

    #[test]
    fn fn_0056d340_copies_the_vector() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004a_3f10]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d340, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004a_3f10), vec![vec![0x1000, 0x2000]]);
    }

    #[test]
    fn fn_0056d360_stores_the_value_at_offset_0xd4() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0054_07b0]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d360, &args![0x1000u32, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0054_07b0), vec![vec![0x10d4, 5]]);
    }

    #[test]
    fn bhk_rigid_body_constructor_chains_the_bases_and_counts() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5120, 0x004e_e810, 0x00c8_d650]);
        e.set_global(0x0126_81bc, 4u32);
        e.set_global(0x0126_81fc, 8u32);
        e.set_global(0x0126_810c, 12u32);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_d380, &args![this, 0x9000u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0103_01b4);
        assert_eq!(calls_to(&log, 0x004b_5120), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x004e_e810), vec![vec![this + 0x14]]);
        assert_eq!(calls_to(&log, 0x00c8_d650), vec![vec![this, 0x9000]]);
        assert_eq!(e.global::<u32>(0x0126_81bc), 5);
        assert_eq!(e.global::<u32>(0x0126_81fc), 9);
        assert_eq!(e.global::<u32>(0x0126_810c), 13);
    }

    #[test]
    fn fn_0056d410_installs_the_entity_vtable() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5120]);
        let this = e.mem.alloc(0x20);
        e.set_global(0x0126_81fc, 1u32);
        assert_eq!(e.call(0x0056_d410, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0103_02bc);
        assert_eq!(e.global::<u32>(0x0126_81fc), 2);
    }

    #[test]
    fn fn_0056d440_installs_the_world_object_vtable() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5120]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x55);
        e.set_global(0x0126_810c, 7u32);
        assert_eq!(e.call(0x0056_d440, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0103_0394);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.global::<u32>(0x0126_810c), 8);
    }

    #[test]
    fn the_get_rtti_methods_return_their_records() {
        let mut e = havok_engine();
        assert_eq!(e.call(0x0056_d480, &args![1u32]).u32(), 0x0126_8110);
        assert_eq!(e.call(0x0056_d4f0, &args![1u32]).u32(), 0x0126_8200);
        assert_eq!(e.call(0x0056_d530, &args![1u32]).u32(), 0x0126_81c0);
        assert_eq!(e.call(0x0056_d5b0, &args![1u32]).u32(), 0x0120_43f8);
    }

    #[test]
    fn bhk_world_object_force_add_calls_virtual_0x9c() {
        let mut e = havok_engine();
        stub(&mut e, &[0x00fe_009c]);
        let this = object_with_vtable(&mut e, 0x20, &[(0x9c, 0x00fe_009c)]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d490, &args![this, 0x42u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00fe_009c), vec![vec![this, 0x42]]);
    }

    /// Checks a scalar deleting destructor: it runs `body`, and frees `size`
    /// bytes only when bit 0 of the flags is set.
    fn check_scalar_deleting(address: u32, body: u32, size: u32) {
        let mut e = havok_engine();
        stub(&mut e, &[body, FREE_SIZED]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![0x3000u32, 0u32]).u32(), 0x3000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, body), vec![vec![0x3000]]);
        assert!(calls_to(&log, FREE_SIZED).is_empty());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![0x3000u32, 3u32]).u32(), 0x3000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FREE_SIZED), vec![vec![0x3000, size]]);
    }

    #[test]
    fn bhk_world_object_destructor_frees_0x14_bytes() {
        check_scalar_deleting(0x0056_d4c0, 0x00c8_5790, 0x14);
    }

    #[test]
    fn bhk_entity_destructor_frees_0x14_bytes() {
        check_scalar_deleting(0x0056_d500, 0x00c9_d620, 0x14);
    }

    #[test]
    fn bhk_rigid_body_destructor_frees_0x1c_bytes() {
        check_scalar_deleting(0x0056_d550, 0x00c8_e9c0, 0x1c);
    }

    #[test]
    fn fn_0056d5c0_frees_0x14_bytes() {
        check_scalar_deleting(0x0056_d5c0, 0x0066_d410, 0x14);
    }

    #[test]
    fn bhk_rigid_body_q_cinfo_size_is_0xe0() {
        let mut e = havok_engine();
        assert_eq!(e.call(0x0056_d540, &args![1u32]).u32(), 0xe0);
    }

    #[test]
    fn fn_0056d580_builds_the_collision_object() {
        let mut e = havok_engine();
        stub(&mut e, &[0x00c6_b950]);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_d580, &args![this, 0x77u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00c6_b950), vec![vec![this, 0x77]]);
        assert_eq!(e.mem.u32(this), 0x0103_046c);
    }

    #[test]
    fn fn_0056d640_builds_the_shape_base() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0053_8ef0, 0x0053_7e90]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x99);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_d640, &args![this, 0x1du32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0103_05a8);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(calls_to(&log, 0x0053_8ef0), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x0053_7e90), vec![vec![this + 0xc, 0x1d]]);
    }

    #[test]
    fn fn_0056d610_builds_the_phantom_callback_shape() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0053_8ef0, 0x0053_7e90]);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_d610, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0103_0570);
        assert_eq!(calls_to(&log, 0x0053_7e90), vec![vec![this + 0xc, 0x1d]]);
    }

    #[test]
    fn fn_0056d5f0_builds_the_water_callback_shape() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0053_8ef0, 0x0053_7e90]);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0056_d5f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0103_0534);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn fn_0056d6c0_runs_the_destructor_body() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0053_8e10]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d6c0, &args![0x3000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0053_8e10), vec![vec![0x3000]]);
    }

    #[test]
    fn fn_0056d710_runs_the_destructor_body() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0053_8e10]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d710, &args![0x3000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0053_8e10), vec![vec![0x3000]]);
    }

    #[test]
    fn fn_0056d7c0_runs_the_destructor_body() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0053_8e10]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d7c0, &args![0x3000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0053_8e10), vec![vec![0x3000]]);
    }

    #[test]
    fn fn_0056d6e0_frees_the_object_on_request() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0053_8e10, 0x0053_8eb0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_d6e0, &args![0x3000u32, 0u32]).u32(), 0x3000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0053_8e10), vec![vec![0x3000]]);
        assert!(calls_to(&log, 0x0053_8eb0).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0056_d6e0, &args![0x3000u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0053_8eb0), vec![vec![0x3000]]);
    }

    #[test]
    fn fn_0056d750_frees_the_object_on_request() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0053_8e10, 0x0053_8eb0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_d750, &args![0x3000u32, 0u32]).u32(), 0x3000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0053_8e10), vec![vec![0x3000]]);
        assert!(calls_to(&log, 0x0053_8eb0).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0056_d750, &args![0x3000u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0053_8eb0), vec![vec![0x3000]]);
    }

    #[test]
    fn fn_0056d7a0_destroys_the_member_at_0xc() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0057_c530]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d7a0, &args![0x3000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0057_c530), vec![vec![0x300c]]);
    }

    #[test]
    fn fn_0056d780_destroys_the_member_at_0xc() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0057_c530]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d780, &args![0x3000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0057_c530), vec![vec![0x300c]]);
    }

    #[test]
    fn fn_0056d730_destroys_the_member_at_0xc() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0057_c530]);
        e.call_log = Some(vec![]);
        e.call(0x0056_d730, &args![0x3000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0057_c530), vec![vec![0x300c]]);
    }

    #[test]
    fn fn_0056df00_builds_the_rotation_part() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0062_1990]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_df00, &args![0x3000u32]).u32(), 0x3000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0062_1990), vec![vec![0x3000]]);
    }

    #[test]
    fn fn_0056ded0_builds_rotation_and_translation() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0062_1990]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_ded0, &args![0x3000u32]).u32(), 0x3000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0062_1990), vec![vec![0x3000]]);
        assert_eq!(calls_to(&log, LIST_ITEM_SLOT), vec![vec![0x3030]]);
    }

    #[test]
    fn fn_0056e090_constructs_the_creation_info() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0056_e0b0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e090, &args![0x3000u32]).u32(), 0x3000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0056_e0b0), vec![vec![0x3000]]);
    }

    #[test]
    fn fn_0056dff0_converts_the_three_columns() {
        let mut e = havok_engine();
        install_vector_math(&mut e);
        let source = e.mem.alloc(0x34);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(source + 4 * i as u32, *v);
        }
        let out = e.mem.alloc(0x40);
        assert_eq!(e.call(0x0056_dff0, &args![out, source]).u32(), out);
        assert_eq!(floats(&e, out, 4), vec![1.0, 4.0, 7.0, 0.0]);
        assert_eq!(floats(&e, out + 16, 4), vec![2.0, 5.0, 8.0, 0.0]);
        assert_eq!(floats(&e, out + 32, 4), vec![3.0, 6.0, 9.0, 0.0]);
    }

    #[test]
    fn ni2hk_converts_rotation_and_translation() {
        let mut e = havok_engine();
        install_vector_math(&mut e);
        let source = e.mem.alloc(0x34);
        for i in 0..9 {
            e.mem.set_f32(source + 4 * i, i as f32);
        }
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(source + 0x24 + 4 * i as u32, *v);
        }
        let out = e.mem.alloc(0x40);
        assert_eq!(e.call(0x0056_df80, &args![out, source]).u32(), out);
        assert_eq!(floats(&e, out, 4), vec![0.0, 3.0, 6.0, 0.0]);
        assert_eq!(floats(&e, out + 0x30, 4), vec![10.0, 20.0, 30.0, 0.0]);
    }

    #[test]
    fn fn_0056e050_needs_the_havok_object() {
        let mut e = havok_engine();
        stub(&mut e, &[0x00a2_9680, 0x00c9_e910]);
        returns(&mut e, 0x004a_e750, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_e050, &args![0x3000u32, 0x4000u32]);
        assert_eq!(addresses(&e.call_log.take().unwrap()).len(), 2);

        returns(&mut e, 0x004a_e750, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0056_e050, &args![0x3000u32, 0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            vec![
                0x0056_e050,
                0x004a_e750,
                0x00a2_9680,
                0x00c9_e910,
                0x00a2_9680
            ]
        );
        assert_eq!(calls_to(&log, 0x00c9_e910), vec![vec![0x5000, 0x4000]]);
    }

    #[test]
    fn bhk_shape_phantom_set_transform_converts_and_applies() {
        let mut e = havok_engine();
        install_vector_math(&mut e);
        stub(&mut e, &[0x0062_1990, 0x00a2_9680]);
        returns(&mut e, 0x004a_e750, 0x5000);
        let applied = Rc::new(RefCell::new(Vec::new()));
        let probe = applied.clone();
        e.register_double(0x00c9_e910, move |e, a| {
            probe.borrow_mut().push((a[0], floats(e, a[1] + 0x30, 3)));
            Ret::default()
        });
        let source = e.mem.alloc(0x34);
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(source + 0x24 + 4 * i as u32, *v);
        }
        e.call(0x0056_df20, &args![0x3000u32, source]);
        assert_eq!(*applied.borrow(), vec![(0x5000, vec![7.0, 8.0, 9.0])]);
    }

    // ---- InitHavokForPrimitiveTrigger ---------------------------------

    type Snapshot = (u32, Vec<f32>);

    struct TriggerScene {
        e: Engine,
        this: Ptr<TESObjectREFR>,
        node: u32,
        loaded: u32,
        holder: u32,
        snapshots: Rc<RefCell<Vec<Snapshot>>>,
        phantom_calls: Rc<RefCell<CallTrace>>,
    }

    /// A trigger reference with a primitive of `kind` (bounds 1, 2, 3), an
    /// optional collision filter, and an orientation that is the identity
    /// matrix when `identity`.
    fn trigger_scene(kind: u32, filter: Option<u32>, identity: bool) -> TriggerScene {
        let mut e = load_engine();
        e.map(0x0126_8000, 0x1000);
        install_vector_math(&mut e);
        e.set_global(0x0101_2054, -1.0f32);
        let this = load_refr(&mut e, 0x23);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x40, cell);
        let loaded = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x64, loaded);
        let holder = 0x1357;
        returns(&mut e, 0x0045_43c0, holder);
        let primitive = e.mem.alloc(0x40);
        e.mem.set_u32(primitive + 4, kind);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(primitive + 0x18 + 4 * i as u32, *v);
        }
        returns(&mut e, 0x0041_fbe0, primitive);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, filter.unwrap_or(0));
        returns(&mut e, 0x0042_11a0, if filter.is_some() { data } else { 0 });
        let node = node_object(&mut e, 0);
        returns(&mut e, slot_double(0x1d0), node);
        let position = e.mem.alloc(0x10);
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        returns(&mut e, slot_double(0x1f4), position);
        e.register(0x0056_fa00, |e, a| {
            for i in 0..9 {
                e.mem.set_f32(a[1] + 4 * i, 0.5 + i as f32);
            }
            a[1].into_ret()
        });
        e.register(0x0047_6a80, |_, _| Ret::default());
        e.register_double(0x004d_9ae0, move |_, a| {
            assert_eq!(a[1], 0x011a_9448);
            u32::from(identity).into_ret()
        });
        e.register(0x00aa_13e0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0056_d580, |_, a| a[0].into_ret());
        stub(
            &mut e,
            &[
                0x0062_1990,
                0x0056_7490,
                0x00a5_9c60,
                0x0056_e4e0,
                0x0056_e0b0,
                0x0056_ea90,
                0x0057_c530,
                0x00c6_b950,
                0x0051_9230,
                0x00a5_bca0,
                0x00c8_5c10,
                0x0054_07b0,
                0x00a2_9680,
                0x00c9_e910,
            ],
        );
        returns(&mut e, 0x004a_e750, 0x5000);
        returns(&mut e, 0x0062_0b80, 0x6262);
        e.register(0x00c4_2f70, |_, a| a[0].into_ret());
        returns_float(&mut e, 0x004a_3e90, 0.25);
        let snapshots = Rc::new(RefCell::new(Vec::new()));
        let phantom_calls = Rc::new(RefCell::new(Vec::new()));
        // the creation-info consumers keep a snapshot of the record and
        // return an object whose virtual +0x9c is a double
        stub(&mut e, &[0x00fe_009c]);
        for addr in [0x0056_e550u32, 0x0056_e2d0] {
            let snapshots = snapshots.clone();
            let calls = phantom_calls.clone();
            e.register_double(addr, move |e, a| {
                snapshots.borrow_mut().push((
                    addr,
                    vec![
                        f32::from_bits(e.mem.u32(a[1])),
                        f32::from_bits(e.mem.u32(a[1] + 4)),
                        f32::from_bits(e.mem.u32(a[1] + 8)),
                        e.mem.f32(a[1] + 0x20),
                        e.mem.f32(a[1] + 0x24),
                        e.mem.f32(a[1] + 0x28),
                        e.mem.f32(a[1] + 0x30),
                        e.mem.f32(a[1] + 0x34),
                        e.mem.f32(a[1] + 0x38),
                    ],
                ));
                calls.borrow_mut().push((addr, a.to_vec()));
                let vtable = e.mem.alloc(0x100);
                e.mem.set_u32(vtable + 0x9c, 0x00fe_009c);
                e.mem.set_u32(a[0], vtable);
                a[0].into_ret()
            });
        }
        for addr in [0x0056_e610u32, 0x0056_e950] {
            let calls = phantom_calls.clone();
            e.register_double(addr, move |e, a| {
                let mut words = a.to_vec();
                if addr == 0x0056_e610 {
                    words.extend((0..4).map(|i| e.mem.f32(a[1] + 4 * i).to_bits()));
                }
                calls.borrow_mut().push((addr, words));
                a[0].into_ret()
            });
        }
        TriggerScene {
            e,
            this,
            node,
            loaded,
            holder,
            snapshots,
            phantom_calls,
        }
    }

    #[test]
    fn init_havok_for_primitive_trigger_needs_a_cell_holder() {
        let mut s = trigger_scene(1, None, true);
        returns(&mut s.e, 0x0045_43c0, 0);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_d7e0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0056_7490).is_empty());
        assert!(calls_to(&log, 0x00aa_13e0).is_empty());

        s.e.mem.set_u32(s.this.addr() + 0x40, 0);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_d7e0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0045_43c0).is_empty());
    }

    #[test]
    fn init_havok_for_primitive_trigger_builds_an_aabb_phantom() {
        let mut s = trigger_scene(1, None, true);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_d7e0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        let this = s.this.addr();

        // scales reset to 1.0 and the root moved to the origin
        assert_eq!(
            calls_to(&log, 0x0056_7490),
            vec![vec![this, 1.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, 0x0044_0490),
            vec![vec![s.node, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, 0x00a5_9c60).len(), 1);
        // the box corners: position - bounds at +0x20, position + bounds at +0x30
        let snapshots = s.snapshots.borrow();
        assert_eq!(snapshots.len(), 1);
        let (addr, words) = &snapshots[0];
        assert_eq!(*addr, 0x0056_e550);
        assert_eq!(f32::to_bits(words[0]), 0x0005_0000 | 0x16);
        assert_eq!(words[3..6], [9.0, 18.0, 27.0]);
        assert_eq!(words[6..9], [11.0, 22.0, 33.0]);
        // no shape was built
        assert!(calls_to(&log, 0x0056_e610).is_empty());
        assert!(calls_to(&log, 0x0056_e2d0).is_empty());
        // the phantom is stored in the loaded data and linked
        let phantom = s.e.mem.u32(s.loaded + 0x18);
        assert_ne!(phantom, 0);
        assert_eq!(calls_to(&log, 0x00c8_5c10), vec![vec![phantom, s.node, 0]]);
        assert_eq!(calls_to(&log, 0x00fe_009c), vec![vec![phantom, s.holder]]);
        // BSXFlags is set to 2, 1 and added to the root
        let added = calls_to(&log, 0x00a5_bca0);
        assert_eq!(added.len(), 1);
        assert_eq!(calls_to(&log, 0x0051_9230), vec![vec![added[0][1], 2, 1]]);
        // the creation info is destroyed through 0056d730
        assert_eq!(calls_to(&log, 0x0057_c530).len(), 1);
    }

    #[test]
    fn init_havok_for_primitive_trigger_uses_a_shape_phantom_for_a_rotated_box() {
        let mut s = trigger_scene(1, None, false);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_d7e0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        // the box shape gets the bounds as a 16-byte vector
        let calls = s.phantom_calls.borrow();
        let shape_call = calls.iter().find(|(a, _)| *a == 0x0056_e610).unwrap();
        assert_eq!(
            shape_call.1[2..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 0]
        );
        // the phantom's creation info: filter and the Havok shape
        let snapshots = s.snapshots.borrow();
        assert_eq!(snapshots[0].0, 0x0056_e2d0);
        assert_eq!(snapshots[0].1[0].to_bits(), 0x0005_0000 | 0x16);
        assert_eq!(calls_to(&log, 0x0062_0b80), vec![vec![shape_call.1[0]]]);
        assert!(calls_to(&log, 0x0056_e550).is_empty());
        // the transform is applied to the new phantom and the info destroyed
        assert_eq!(calls_to(&log, 0x00c9_e910).len(), 1);
        assert_eq!(calls_to(&log, 0x0056_ea90).len(), 1);
        let phantom = s.e.mem.u32(s.loaded + 0x18);
        assert_eq!(calls_to(&log, 0x00fe_009c), vec![vec![phantom, s.holder]]);
    }

    #[test]
    fn init_havok_for_primitive_trigger_filter_0x17_forces_the_shape_phantom() {
        let mut s = trigger_scene(1, Some(0x17), true);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_d7e0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x004d_9ae0).is_empty());
        assert_eq!(calls_to(&log, 0x0056_e2d0).len(), 1);
        assert_eq!(s.snapshots.borrow()[0].1[0].to_bits(), 0x0005_0000 | 0x17);
    }

    #[test]
    fn init_havok_for_primitive_trigger_builds_a_sphere_shape() {
        let mut s = trigger_scene(2, Some(0x30), true);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_d7e0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        // the sphere radius is the converted first bound
        assert_eq!(calls_to(&log, 0x004a_3e90), vec![vec![1.0f32.to_bits()]]);
        let calls = s.phantom_calls.borrow();
        let sphere = calls.iter().find(|(a, _)| *a == 0x0056_e950).unwrap();
        assert_eq!(sphere.1[1..], [0.25f32.to_bits(), 0]);
        assert!(calls_to(&log, 0x0056_e610).is_empty());
        assert_eq!(s.snapshots.borrow()[0].1[0].to_bits(), 0x0005_0000 | 0x30);
    }

    #[test]
    fn init_havok_for_primitive_trigger_leaves_other_primitives_without_a_shape() {
        let mut s = trigger_scene(3, None, true);
        s.e.call_log = Some(vec![]);
        s.e.call(0x0056_d7e0, &args![s.this]);
        let log = s.e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0056_e610).is_empty());
        assert!(calls_to(&log, 0x0056_e950).is_empty());
        assert_eq!(calls_to(&log, 0x0062_0b80), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x0056_e2d0).len(), 1);
    }

    // ---- third batch: phantoms and shapes (0056e0b0 to 0056ea00) ----

    /// A large zeroed object for the constructor tests.
    fn scratch_object(e: &mut Engine) -> u32 {
        e.mem.alloc(0x100)
    }

    /// Checks a `GetRTTI` method: it returns the record's address.
    fn check_rtti(address: u32, record: u32) {
        let mut e = havok_engine();
        assert_eq!(e.call(address, &args![0x3000u32]).u32(), record);
    }

    /// Maps the vtable page of the shape classes (`01030xxx`) and puts
    /// `target` into the `+0xb0` slot of the vtable at `vtable`.
    fn set_init_slot(e: &mut Engine, vtable: u32, target: u32) {
        e.map(0x0103_0000, 0x1000);
        e.mem.set_u32(vtable + 0xb0, target);
    }

    #[test]
    fn fn_0056e0b0_builds_the_creation_info_record() {
        let mut e = havok_engine();
        stub(
            &mut e,
            &[
                0x0068_15c0,
                0x0062_99a0,
                0x0054_07b0,
                0x004b_4fa0,
                0x004b_4fc0,
            ],
        );
        let this = scratch_object(&mut e);
        e.mem.set_u32(this, 0x55);
        e.mem.set_u32(this + 4, 0x66);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e0b0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(
            calls_to(&log, 0x0054_07b0),
            vec![vec![this + 8, 1], vec![this + 8, 2]]
        );
        assert_eq!(
            calls_to(&log, 0x004b_4fa0),
            vec![
                vec![this + 0x20],
                vec![this + 0x30],
                vec![this + 0x40],
                vec![this + 0x50]
            ]
        );
        assert_eq!(e.mem.f32(this + 0x20), 1.0);
        assert_eq!(e.mem.f32(this + 0x34), 1.0);
        assert_eq!(e.mem.f32(this + 0x48), 1.0);
    }

    #[test]
    fn fn_0056e120_resets_the_matrix_and_the_fourth_vector() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_4fc0]);
        e.call_log = Some(vec![]);
        let this = scratch_object(&mut e);
        e.call(0x0056_e120, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x004b_4fc0),
            vec![
                vec![this],
                vec![this + 0x10],
                vec![this + 0x20],
                vec![this + 0x30]
            ]
        );
    }

    #[test]
    fn fn_0056e140_sets_the_identity_diagonal() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_4fc0]);
        let this = scratch_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0056_e140, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x004b_4fc0),
            vec![vec![this], vec![this + 0x10], vec![this + 0x20]]
        );
        assert_eq!(e.mem.f32(this), 1.0);
        assert_eq!(e.mem.f32(this + 0x14), 1.0);
        assert_eq!(e.mem.f32(this + 0x28), 1.0);
        assert_eq!(e.mem.f32(this + 4), 0.0);
    }

    #[test]
    fn fn_0056e180_runs_the_leading_part_then_sets_2() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0068_15c0, 0x0062_99a0, 0x0054_07b0]);
        let this = scratch_object(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e180, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0054_07b0),
            vec![vec![this + 8, 1], vec![this + 8, 2]]
        );
    }

    #[test]
    fn fn_0056e1e0_constructs_the_members_and_zeroes_two_words() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0068_15c0, 0x0062_99a0, 0x0054_07b0]);
        let this = scratch_object(&mut e);
        e.mem.set_u32(this, 7);
        e.mem.set_u32(this + 4, 8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e1e0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0068_15c0),
            vec![vec![this], vec![this + 8]]
        );
        assert_eq!(calls_to(&log, 0x0062_99a0), vec![vec![this + 0xc]]);
        assert_eq!(calls_to(&log, 0x0054_07b0), vec![vec![this + 8, 1]]);
        assert_eq!((e.mem.u32(this), e.mem.u32(this + 4)), (0, 0));
    }

    #[test]
    fn fn_0056e270_constructs_four_vectors() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_4fa0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e270, &args![0x4000u32]).u32(), 0x4000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x004b_4fa0),
            vec![vec![0x4000], vec![0x4010], vec![0x4020], vec![0x4030]]
        );
    }

    #[test]
    fn fn_0056e2a0_constructs_three_vectors() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_4fa0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e2a0, &args![0x4000u32]).u32(), 0x4000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x004b_4fa0),
            vec![vec![0x4000], vec![0x4010], vec![0x4020]]
        );
    }

    #[test]
    fn bhk_simple_shape_phantom_constructor_installs_its_vtable_and_inits() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5120, 0x00c9_f050]);
        let this = scratch_object(&mut e);
        e.mem.set_u8(this + 0x14, 0xff);
        e.set_global(0x0126_8238, 1u32);
        e.set_global(0x0126_8208, 2u32);
        e.set_global(0x0126_822c, 3u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e2d0, &args![this, 0x9000u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0103_05dc);
        assert_eq!(calls_to(&log, 0x00c9_f050), vec![vec![this, 0x9000]]);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(e.global::<u32>(0x0126_8238), 2);
        assert_eq!(e.global::<u32>(0x0126_8208), 3);
        assert_eq!(e.global::<u32>(0x0126_822c), 4);
    }

    #[test]
    fn fn_0056e350_clears_the_byte() {
        let mut e = havok_engine();
        let this = scratch_object(&mut e);
        e.mem.set_u8(this + 0x14, 9);
        e.mem.set_u8(this + 0x15, 9);
        e.call(0x0056_e350, &args![this]);
        assert_eq!((e.mem.u8(this + 0x14), e.mem.u8(this + 0x15)), (0, 9));
    }

    #[test]
    fn fn_0056e370_installs_the_shape_phantom_vtable() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5120]);
        let this = scratch_object(&mut e);
        e.mem.set_u8(this + 0x14, 1);
        assert_eq!(e.call(0x0056_e370, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0103_06b4);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(e.global::<u32>(0x0126_8208), 1);
        assert_eq!(e.global::<u32>(0x0126_8238), 1);
    }

    #[test]
    fn fn_0056e3e0_installs_the_phantom_vtable() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5120]);
        let this = scratch_object(&mut e);
        e.set_global(0x0126_8238, 4u32);
        assert_eq!(e.call(0x0056_e3e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0103_078c);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.global::<u32>(0x0126_8238), 5);
    }

    #[test]
    fn bhk_phantom_get_rtti_returns_its_record() {
        check_rtti(0x0056_e410, 0x0126_823c);
    }

    #[test]
    fn bhk_phantom_destructor_frees_0x18_bytes() {
        check_scalar_deleting(0x0056_e420, 0x00c9_f3b0, 0x18);
    }

    #[test]
    fn bhk_shape_phantom_get_rtti_returns_its_record() {
        check_rtti(0x0056_e450, 0x0126_8224);
    }

    #[test]
    fn bhk_shape_phantom_destructor_frees_0x18_bytes() {
        check_scalar_deleting(0x0056_e460, 0x00c9_eb30, 0x18);
    }

    #[test]
    fn bhk_simple_shape_phantom_get_rtti_returns_its_record() {
        check_rtti(0x0056_e490, 0x0126_8230);
    }

    #[test]
    fn fn_0056e4a0_is_0x60() {
        check_rtti(0x0056_e4a0, 0x60);
    }

    #[test]
    fn bhk_simple_shape_phantom_destructor_frees_0x18_bytes() {
        check_scalar_deleting(0x0056_e4b0, 0x00c9_eec0, 0x18);
    }

    #[test]
    fn fn_0056e4e0_runs_the_member_helpers() {
        let mut e = havok_engine();
        stub(
            &mut e,
            &[
                0x0068_15c0,
                0x0062_99a0,
                0x0054_07b0,
                0x0043_7c10,
                0x0053_8f40,
            ],
        );
        let this = scratch_object(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e4e0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0043_7c10), vec![vec![this + 0x20]]);
        assert_eq!(
            calls_to(&log, 0x0053_8f40),
            vec![vec![this + 0x20], vec![this + 0x30]]
        );
        assert_eq!(
            calls_to(&log, 0x0054_07b0),
            vec![vec![this + 8, 1], vec![this + 8, 2]]
        );
    }

    #[test]
    fn bhk_aabb_phantom_constructor_installs_its_vtable_and_inits() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5120, 0x00c9_f890]);
        let this = scratch_object(&mut e);
        e.mem.set_u8(this + 0x14, 0xff);
        e.set_global(0x0126_8238, 1u32);
        e.set_global(0x0126_8244, 6u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e550, &args![this, 0x9100u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0103_0864);
        assert_eq!(calls_to(&log, 0x00c9_f890), vec![vec![this, 0x9100]]);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(e.global::<u32>(0x0126_8238), 2);
        assert_eq!(e.global::<u32>(0x0126_8244), 7);
    }

    #[test]
    fn bhk_aabb_phantom_get_rtti_returns_its_record() {
        check_rtti(0x0056_e5d0, 0x0126_8248);
    }

    #[test]
    fn bhk_aabb_phantom_destructor_frees_0x18_bytes() {
        check_scalar_deleting(0x0056_e5e0, 0x00c9_f530, 0x18);
    }

    #[test]
    fn bhk_box_shape_constructor_initializes_from_the_half_extents() {
        let mut e = havok_engine();
        stub(
            &mut e,
            &[0x0056_e690, 0x004b_5020, 0x004b_4fa0, 0x004b_4db0],
        );
        e.set_global(0x010c_72bc, 0.1f32);
        let snapshot = Rc::new(RefCell::new(Vec::new()));
        let probe = snapshot.clone();
        e.register_double(0x00fe_00b0, move |e, a| {
            *probe.borrow_mut() = (0..8).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            Ret::default()
        });
        set_init_slot(&mut e, 0x0103_093c, 0x00fe_00b0);
        let this = scratch_object(&mut e);
        e.set_global(0x0126_825c, 2u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e610, &args![this, 0x9200u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0103_093c);
        assert_eq!(e.global::<u32>(0x0126_825c), 3);
        assert_eq!(calls_to(&log, 0x0056_e690), vec![vec![this]]);
        let record = snapshot.borrow().clone();
        assert_eq!(record[1], 0.1f32.to_bits());
        assert_eq!(record[4..7], [1.0f32.to_bits(); 3]);
        let calls = calls_to(&log, 0x004b_4db0);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][1], 0x9200);
        assert_eq!(calls[0][0], calls_to(&log, 0x004b_4fa0)[0][0]);
    }

    #[test]
    fn bhk_shape_get_rtti_returns_its_record() {
        check_rtti(0x0056_e730, 0x0126_8100);
    }

    #[test]
    fn bhk_shape_destructor_frees_0x14_bytes() {
        check_scalar_deleting(0x0056_e740, 0x00c8_4c50, 0x14);
    }

    #[test]
    fn bhk_sphere_rep_shape_get_rtti_returns_its_record() {
        check_rtti(0x0056_e770, 0x0126_8278);
    }

    #[test]
    fn bhk_sphere_rep_shape_destructor_frees_0x14_bytes() {
        check_scalar_deleting(0x0056_e780, 0x00ca_0540, 0x14);
    }

    #[test]
    fn bhk_convex_shape_get_rtti_returns_its_record() {
        check_rtti(0x0056_e7b0, 0x0126_8254);
    }

    #[test]
    fn bhk_convex_shape_destructor_frees_0x14_bytes() {
        check_scalar_deleting(0x0056_e7c0, 0x00c9_fb00, 0x14);
    }

    #[test]
    fn bhk_box_shape_get_rtti_returns_its_record() {
        check_rtti(0x0056_e7f0, 0x0126_8264);
    }

    #[test]
    fn fn_0056e800_is_0x20() {
        check_rtti(0x0056_e800, 0x20);
    }

    #[test]
    fn fn_0056e810_passes_the_filled_record_to_virtual_0xb0() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5020, 0x004b_4fa0]);
        e.set_global(0x010c_72bc, 0.1f32);
        // 004b4db0 stand-in: copies three floats from the pointer
        e.register(0x004b_4db0, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        let snapshot = Rc::new(RefCell::new(Vec::new()));
        let probe = snapshot.clone();
        e.register_double(0x00fe_00b0, move |e, a| {
            *probe.borrow_mut() = (0..8).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            Ret::default()
        });
        let this = object_with_vtable(&mut e, 0x20, &[(0xb0, 0x00fe_00b0)]);
        let half = e.mem.alloc(0x10);
        for (i, v) in [2.0f32, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(half + 4 * i as u32, *v);
        }
        e.call(0x0056_e810, &args![this, half]);
        let record = snapshot.borrow().clone();
        assert_eq!(record[1], 0.1f32.to_bits());
        assert_eq!(
            record[4..7],
            [2.0f32.to_bits(), 3.0f32.to_bits(), 4.0f32.to_bits()]
        );
    }

    #[test]
    fn fn_0056e860_fills_the_box_record_with_defaults() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5020, 0x004b_4fa0]);
        e.set_global(0x010c_72bc, 0.1f32);
        let this = scratch_object(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e860, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004b_5020), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x004b_4fa0), vec![vec![this + 0x10]]);
        assert_eq!(e.mem.f32(this + 4), 0.1);
        for word in 0..3 {
            assert_eq!(e.mem.f32(this + 0x10 + 4 * word), 1.0);
        }
    }

    #[test]
    fn fn_0056e8a0_stores_the_value_three_times() {
        let mut e = havok_engine();
        let this = scratch_object(&mut e);
        e.call(0x0056_e8a0, &args![this, 2.5f32]);
        for word in 0..3 {
            assert_eq!(e.mem.f32(this + 4 * word), 2.5);
        }
        assert_eq!(e.mem.f32(this + 12), 0.0);
    }

    #[test]
    fn fn_0056e8d0_stores_the_default_convex_radius() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5020]);
        e.set_global(0x010c_72bc, 0.1f32);
        let this = scratch_object(&mut e);
        assert_eq!(e.call(0x0056_e8d0, &args![this]).u32(), this);
        assert_eq!(e.mem.f32(this + 4), 0.1);
    }

    #[test]
    fn fn_0056e900_runs_the_word_constructor() {
        let mut e = havok_engine();
        stub(&mut e, &[0x004b_5020]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e900, &args![0x4000u32]).u32(), 0x4000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004b_5020), vec![vec![0x4000]]);
    }

    #[test]
    fn bhk_box_shape_destructor_frees_0x14_bytes() {
        check_scalar_deleting(0x0056_e920, 0x00c9_fd70, 0x14);
    }

    /// A sphere shape whose `+0xb0` slot records the radius it is given.
    fn sphere_scene(converted: Option<f64>) -> (Engine, u32, Rc<Cell<u32>>) {
        let mut e = havok_engine();
        stub(&mut e, &[0x0056_e690, 0x0056_ea40]);
        if let Some(value) = converted {
            returns_float(&mut e, 0x004a_3e90, value);
        }
        let seen = Rc::new(Cell::new(0xffff_ffffu32));
        let probe = seen.clone();
        e.register_double(0x00fe_00b0, move |e, a| {
            probe.set(e.mem.u32(a[1] + 4));
            Ret::default()
        });
        set_init_slot(&mut e, 0x0103_0cdc, 0x00fe_00b0);
        let this = scratch_object(&mut e);
        (e, this, seen)
    }

    #[test]
    fn bhk_sphere_shape_constructor_converts_the_radius_when_asked() {
        let (mut e, this, seen) = sphere_scene(Some(0.25));
        e.set_global(0x0126_8280, 1u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e950, &args![this, 2.0f32, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0103_0cdc);
        assert_eq!(e.global::<u32>(0x0126_8280), 2);
        assert_eq!(calls_to(&log, 0x004a_3e90), vec![vec![2.0f32.to_bits()]]);
        assert_eq!(f32::from_bits(seen.get()), 0.25);
    }

    #[test]
    fn bhk_sphere_shape_constructor_keeps_the_radius_when_not_converting() {
        let (mut e, this, seen) = sphere_scene(None);
        e.call_log = Some(vec![]);
        e.call(0x0056_e950, &args![this, 2.0f32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x004a_3e90).is_empty());
        assert_eq!(f32::from_bits(seen.get()), 2.0);
    }

    #[test]
    fn bhk_sphere_shape_get_rtti_returns_its_record() {
        check_rtti(0x0056_e9f0, 0x0126_8288);
    }

    #[test]
    fn fn_0056ea00_passes_the_radius_record_to_virtual_0xb0() {
        let mut e = havok_engine();
        stub(&mut e, &[0x0056_ea40]);
        let seen = Rc::new(Cell::new(0xffff_ffffu32));
        let probe = seen.clone();
        e.register_double(0x00fe_00b0, move |e, a| {
            probe.set(e.mem.u32(a[1] + 4));
            Ret::default()
        });
        let this = object_with_vtable(&mut e, 0x20, &[(0xb0, 0x00fe_00b0)]);
        e.call_log = Some(vec![]);
        e.call(0x0056_ea00, &args![this, 1.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0056_ea40).len(), 1);
        assert_eq!(f32::from_bits(seen.get()), 1.5);
    }

    #[test]
    fn bhk_world_object_force_add_answers_the_virtual() {
        let mut e = havok_engine();
        e.register(0x00fe_009c, |_, _| true.into_ret());
        let this = object_with_vtable(&mut e, 0x20, &[(0x9c, 0x00fe_009c)]);
        assert!(e.call(0x0056_d490, &args![this, 0x42u32]).bool());
    }
}
