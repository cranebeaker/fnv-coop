//! `fallout shared/run_004f90a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB names none of these functions; the linker placed three
//! groups of them together:
//!
//! - `Copy` and `Compare` of `BGSAddonNode` (`004f90a0`, `004f9110`): the
//!   `BGSAddonNode` vtable (`01024214`) holds them at its `+0x108` and
//!   `+0x10c`, the Xbox PDB's `TESForm::Copy` and `TESForm::Compare`;
//! - `BGSAddonNodeSoundHandleExtra` (`004f91b0`..`004f9360`): the vtable
//!   `010243d4`'s run-time type information names the class, and the Xbox
//!   PDB gives it the static `pTag` (`NiFixedString*`) and the
//!   `BSSoundHandle` field at `+0xc`;
//! - the thunk `004f93a0` of `BGSAddonNode`'s deleting destructor and the
//!   `BGSConstructibleObject` constructor, destructors and
//!   `InitializeData` (`004f93b0`..`004f9570`, vtable `010245a4`).
//!
//! `BGSAddonNode` offsets are the PC ones (`TESForm` is 0x18 bytes on the PC):
//! `iIndex` +0x50, `pSound` +0x54, `Data` (`ADDON_DATA`, 4 bytes) +0x58.
//! `BGSConstructibleObject`'s `pRequiredItems` and `pCreatedItem` are at
//! +0xac and +0xb0.

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast(object, vfDelta, sourceType, targetType, isReference)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TypeDescriptor` of `TESForm` (`.?AVTESForm@@`).
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// `TypeDescriptor` of `BGSAddonNode` (`.?AVBGSAddonNode@@`).
const BGS_ADDON_NODE_TYPE: u32 = 0x0118_61a0;
/// `TESForm::CopyAllComponents(source)`.
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// `TESForm::CompareAllComponents(other)`: nonzero when they differ.
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `memcpy(destination, source, length)`.
const MEMCPY: u32 = 0x0040_1460;
/// `memcmp(first, second, length)`.
const MEMCMP: u32 = 0x00ec_4835;
/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(pointer)`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NiFree`-style deallocation `(pointer, size)`.
const FREE_SIZED: u32 = 0x00aa_1460;
/// `NiFixedString(const char*)` constructor.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
/// Deleting destructor of a heap `NiFixedString` (`this`, flags).
const FIXED_STRING_DELETE: u32 = 0x004b_9900;
/// `NiExtraData(const NiFixedString& name)` constructor.
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b440;
/// `NiExtraData` destructor body.
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
/// `BSSoundHandle` constructor.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
/// `BSSoundHandle::Stop`.
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
/// `BSSoundHandle` destructor (empty).
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// Deleting destructor of `BGSAddonNode` (`this`, flags).
const BGS_ADDON_NODE_DELETING_DESTRUCTOR: u32 = 0x004f_8b90;
/// `TESForm::SetFormType(type)` (writes the byte at `+4`).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// Base constructor of `BGSConstructibleObject`.
const CONSTRUCTIBLE_OBJECT_BASE_CONSTRUCT: u32 = 0x0051_a6a0;
/// Base destructor body of `BGSConstructibleObject`.
const CONSTRUCTIBLE_OBJECT_BASE_DESTRUCT: u32 = 0x0051_a840;

/// `BGSAddonNodeSoundHandleExtra::pTag` (Xbox PDB static `NiFixedString*`).
const SOUND_HANDLE_EXTRA_TAG: u32 = 0x011c_8058;
/// `"ASH"`: the text of the tag.
const SOUND_HANDLE_EXTRA_TAG_TEXT: u32 = 0x0102_4468;
/// `BGSAddonNodeSoundHandleExtra::ms_RTTI` (`NiRTTI`, returned by `GetRTTI`).
const SOUND_HANDLE_EXTRA_RTTI: u32 = 0x011c_806c;
/// Vtable of `BGSAddonNodeSoundHandleExtra`.
const SOUND_HANDLE_EXTRA_VTABLE: u32 = 0x0102_43d4;
/// Size of a `BGSAddonNodeSoundHandleExtra` (the deleting destructor frees
/// that many bytes).
const SOUND_HANDLE_EXTRA_SIZE: u32 = 0x18;
/// Form type byte the `BGSConstructibleObject` constructor sets (the
/// number is the game's; its name is not confirmed here).
const CONSTRUCTIBLE_OBJECT_FORM_TYPE: u32 = 0x32;

/// Vtables of `BGSConstructibleObject` by offset in the object: the form's
/// own at +0 (RTTI `.?AVBGSConstructibleObject@@`) and those of its nine
/// component sub-objects. Constructor and destructor body set the same ones.
const CONSTRUCTIBLE_OBJECT_VTABLES: [(u32, u32); 10] = [
    (0x00, 0x0102_45a4),
    (0x30, 0x0102_458c),
    (0x3c, 0x0102_4568),
    (0x5c, 0x0102_4548),
    (0x68, 0x0102_4534),
    (0x74, 0x0102_4514),
    (0x7c, 0x0102_4500),
    (0x84, 0x0102_44ec),
    (0x8c, 0x0102_44d8),
    (0x9c, 0x0102_44c4),
];

/// Casts a `TESForm*` to `BGSAddonNode*` (null when it is another type).
fn cast_to_addon_node(e: &mut Engine, form: Ptr) -> Ptr {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0i32, TES_FORM_TYPE, BGS_ADDON_NODE_TYPE, 0i32],
    )
    .ptr()
}

// Translated from 004f90a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNode::Copy` (Xbox PDB `TESForm::Copy`, vtable `+0x108`): casts
/// `form` to a `BGSAddonNode`; for one, copies the components
/// (`TESForm::CopyAllComponents`), `iIndex`, `pSound` and the 4 bytes of
/// `Data`. Any other form is ignored.
pub fn bgs_addon_node_copy(e: &mut Engine, this: Ptr, form: Ptr) {
    let source = cast_to_addon_node(e, form);
    if source.is_null() {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    // BGSAddonNode::iIndex (Xbox PDB) +0x50, pSound +0x54.
    let index = e.mem.u32(source.addr() + 0x50);
    e.mem.set_u32(this.addr() + 0x50, index);
    let sound = e.mem.u32(source.addr() + 0x54);
    e.mem.set_u32(this.addr() + 0x54, sound);
    // BGSAddonNode::Data (ADDON_DATA, 4 bytes) +0x58.
    e.call(
        MEMCPY,
        &args![this.addr() + 0x58, source.addr() + 0x58, 4u32],
    );
}

// Translated from 004f9110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNode::Compare` (Xbox PDB `TESForm::Compare`, vtable `+0x10c`):
/// true when `form` differs. Casts `form` to a `BGSAddonNode` (another type
/// differs), then differs when `TESForm::CompareAllComponents` says so, or
/// `iIndex`, `pSound` or the 4 bytes of `Data` differ.
pub fn bgs_addon_node_compare(e: &mut Engine, this: Ptr, form: Ptr) -> bool {
    let other = cast_to_addon_node(e, form);
    if other.is_null() {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool() {
        return true;
    }
    if e.mem.u32(this.addr() + 0x50) != e.mem.u32(other.addr() + 0x50) {
        return true;
    }
    if e.mem.u32(this.addr() + 0x54) != e.mem.u32(other.addr() + 0x54) {
        return true;
    }
    e.call(
        MEMCMP,
        &args![this.addr() + 0x58, other.addr() + 0x58, 4u32],
    )
    .i32()
        != 0
}

// Translated from 004f91b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNodeSoundHandleExtra` constructor: `NiExtraData` constructor
/// with `pTag` as the name, its own vtable, and the `BSSoundHandle` at
/// `+0xc`. Returns `this`.
// Translation incomplete: 004f91b0 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn bgs_addon_node_sound_handle_extra_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let tag = fn_004f9230(e);
    e.call(EXTRA_DATA_CONSTRUCT, &args![this, tag]);
    e.mem.set_u32(this.addr(), SOUND_HANDLE_EXTRA_VTABLE);
    e.call(SOUND_HANDLE_CONSTRUCT, &args![this.addr() + 0xc]);
    this
}

// Translated from 004f9220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNodeSoundHandleExtra::GetRTTI` (vtable `+8`): the class's
/// static `NiRTTI`. `this` is not read.
pub fn bgs_addon_node_sound_handle_extra_get_rtti(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(SOUND_HANDLE_EXTRA_RTTI)
}

// Translated from 004f9230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNodeSoundHandleExtra::pTag` (Xbox PDB static
/// `NiFixedString*`), as created by `004f92e0`.
pub fn fn_004f9230(e: &mut Engine) -> Ptr {
    Ptr::new(e.mem.u32(SOUND_HANDLE_EXTRA_TAG))
}

// Translated from 004f9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNodeSoundHandleExtra` deleting destructor (first slot of its
/// vtable): runs the destructor body and, with bit 0 of `flags` set, frees
/// the 0x18 byte object. Returns `this`.
pub fn bgs_addon_node_sound_handle_extra_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bgs_addon_node_sound_handle_extra_destruct(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, SOUND_HANDLE_EXTRA_SIZE]);
    }
    this
}

// Translated from 004f9270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNodeSoundHandleExtra` destructor body: sets its vtable, stops
/// the sound (`BSSoundHandle::Stop`), destroys the handle at `+0xc`, then
/// runs the `NiExtraData` destructor body.
// Translation incomplete: 004f9270 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn bgs_addon_node_sound_handle_extra_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SOUND_HANDLE_EXTRA_VTABLE);
    e.call(SOUND_HANDLE_STOP, &args![this.addr() + 0xc]);
    e.call(SOUND_HANDLE_DESTRUCT, &args![this.addr() + 0xc]);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 004f92e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates `BGSAddonNodeSoundHandleExtra::pTag` (Xbox PDB static
/// `NiFixedString*`): a 4 byte block holding the fixed string `"ASH"`
/// (null when the allocation fails).
// Translation incomplete: 004f92e0 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn fn_004f92e0(e: &mut Engine) {
    let block = e.call(OPERATOR_NEW, &args![4u32]).ptr::<()>();
    let tag = if block.is_null() {
        0
    } else {
        e.call(
            FIXED_STRING_CONSTRUCT,
            &args![block, SOUND_HANDLE_EXTRA_TAG_TEXT],
        )
        .u32()
    };
    e.mem.set_u32(SOUND_HANDLE_EXTRA_TAG, tag);
}

// Translated from 004f9360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `BGSAddonNodeSoundHandleExtra::pTag`: deletes the fixed string
/// block when there is one (deleting destructor with flag 1) and clears the
/// static.
pub fn fn_004f9360(e: &mut Engine) {
    let tag = e.mem.u32(SOUND_HANDLE_EXTRA_TAG);
    if tag != 0 {
        e.call(FIXED_STRING_DELETE, &args![tag, 1u32]);
    }
    e.mem.set_u32(SOUND_HANDLE_EXTRA_TAG, 0);
}

// Translated from 004f93a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjusting thunk in the vtable of `BGSAddonNode`'s `TESModelTextureSwap`
/// sub-object (vtable `01024200`): `this` points `0x30` bytes into the
/// `BGSAddonNode`; calls its deleting destructor (`004f8b90`) on the whole
/// object with the same `flags`.
pub fn bgs_addon_node_deleting_destructor_thunk(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(
        BGS_ADDON_NODE_DELETING_DESTRUCTOR,
        &args![this.addr().wrapping_sub(0x30), flags],
    )
    .ptr()
}

/// Sets the vtables of a `BGSConstructibleObject` and its components.
fn set_constructible_object_vtables(e: &mut Engine, this: Ptr) {
    for (offset, vtable) in CONSTRUCTIBLE_OBJECT_VTABLES {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
}

// Translated from 004f93b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSConstructibleObject` constructor: the base constructor, the vtables
/// of the object and its components, form type 0x32 and `InitializeData`.
/// Returns `this`.
// Translation incomplete: 004f93b0 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn bgs_constructible_object_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CONSTRUCTIBLE_OBJECT_BASE_CONSTRUCT, &args![this]);
    set_constructible_object_vtables(e, this);
    e.call(SET_FORM_TYPE, &args![this, CONSTRUCTIBLE_OBJECT_FORM_TYPE]);
    bgs_constructible_object_initialize_data(e, this);
    this
}

// Translated from 004f9480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSConstructibleObject` deleting destructor (vtable `+0x10`, Xbox PDB
/// `~TESForm`): runs the destructor body and, with bit 0 of `flags` set,
/// frees the object. Returns `this`.
pub fn bgs_constructible_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bgs_constructible_object_destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004f94b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSConstructibleObject` destructor body: resets the vtables of the
/// object and its components, runs the empty destructor of the member at
/// `this` (`00483710`), then the base destructor body.
// Translation incomplete: 004f94b0 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn bgs_constructible_object_destruct(e: &mut Engine, this: Ptr) {
    set_constructible_object_vtables(e, this);
    e.call(SOUND_HANDLE_DESTRUCT, &args![this]);
    e.call(CONSTRUCTIBLE_OBJECT_BASE_DESTRUCT, &args![this]);
}

// Translated from 004f9570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSConstructibleObject::InitializeData` (vtable `+0x14`, Xbox PDB
/// `TESForm::InitializeData`): clears `pRequiredItems` (+0xac) and
/// `pCreatedItem` (+0xb0).
pub fn bgs_constructible_object_initialize_data(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xac, 0);
    e.mem.set_u32(this.addr() + 0xb0, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f90a0, bgs_addon_node_copy(Ptr, Ptr)),
        entry!(0x004f9110, bgs_addon_node_compare(Ptr, Ptr) -> bool),
        entry!(
            0x004f91b0,
            bgs_addon_node_sound_handle_extra_construct(Ptr) -> Ptr
        ),
        entry!(
            0x004f9220,
            bgs_addon_node_sound_handle_extra_get_rtti(Ptr) -> Ptr
        ),
        entry!(0x004f9230, fn_004f9230() -> Ptr),
        entry!(
            0x004f9240,
            bgs_addon_node_sound_handle_extra_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004f9270, bgs_addon_node_sound_handle_extra_destruct(Ptr)),
        entry!(0x004f92e0, fn_004f92e0()),
        entry!(0x004f9360, fn_004f9360()),
        entry!(
            0x004f93a0,
            bgs_addon_node_deleting_destructor_thunk(Ptr, u32) -> Ptr
        ),
        entry!(0x004f93b0, bgs_constructible_object_construct(Ptr) -> Ptr),
        entry!(
            0x004f9480,
            bgs_constructible_object_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004f94b0, bgs_constructible_object_destruct(Ptr)),
        entry!(0x004f9570, bgs_constructible_object_initialize_data(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// A fake `BGSAddonNode`: index, sound and four data bytes.
    fn addon_node(e: &mut Engine, index: u32, sound: u32, data: u32) -> Ptr {
        let node = Ptr::<()>::new(e.mem.alloc(0x60));
        e.mem.set_u32(node.addr() + 0x50, index);
        e.mem.set_u32(node.addr() + 0x54, sound);
        e.mem.set_u32(node.addr() + 0x58, data);
        node
    }

    /// Makes the dynamic cast succeed for `forms` (returning them) and fail
    /// otherwise, checking the type descriptors it is given.
    fn cast_double(e: &mut Engine, forms: Vec<u32>) {
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!(a[1..], [0, TES_FORM_TYPE, BGS_ADDON_NODE_TYPE, 0]);
            Ret {
                eax: if forms.contains(&a[0]) { a[0] } else { 0 },
                ..Ret::default()
            }
        });
    }

    #[test]
    fn copy_takes_index_sound_and_data_of_an_addon_node() {
        let mut e = Engine::new();
        let this = addon_node(&mut e, 1, 2, 0x1111);
        let source = addon_node(&mut e, 7, 0x0abc, 0x2222_3333);
        cast_double(&mut e, vec![source.addr()]);
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            assert_eq!(a[2], 4);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x004f_90a0, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[2],
            (COPY_ALL_COMPONENTS, vec![this.addr(), source.addr()])
        );
        assert_eq!(e.mem.u32(this.addr() + 0x50), 7);
        assert_eq!(e.mem.u32(this.addr() + 0x54), 0x0abc);
        assert_eq!(e.mem.u32(this.addr() + 0x58), 0x2222_3333);
    }

    #[test]
    fn copy_ignores_other_forms() {
        let mut e = Engine::new();
        let this = addon_node(&mut e, 1, 2, 3);
        let other = Ptr::<()>::new(e.mem.alloc(0x60));
        cast_double(&mut e, vec![]);
        e.register(COPY_ALL_COMPONENTS, |_, _| panic!("copied"));
        e.call(0x004f_90a0, &args![this, other]);
        assert_eq!(e.mem.u32(this.addr() + 0x50), 1);
    }

    fn compare_with(e: &mut Engine, this: Ptr, other: Ptr, components_differ: bool) -> bool {
        cast_double(e, vec![other.addr()]);
        e.register_double(COMPARE_ALL_COMPONENTS, move |_, _| Ret {
            eax: components_differ as u32,
            ..Ret::default()
        });
        e.register(MEMCMP, |e, a| {
            let (x, y) = (e.mem.u32(a[0]), e.mem.u32(a[1]));
            Ret {
                eax: x.cmp(&y) as i32 as u32,
                ..Ret::default()
            }
        });
        e.call(0x004f_9110, &args![this, other]).bool()
    }

    #[test]
    fn compare_is_equal_for_identical_addon_nodes() {
        let mut e = Engine::new();
        let this = addon_node(&mut e, 4, 5, 6);
        let other = addon_node(&mut e, 4, 5, 6);
        assert!(!compare_with(&mut e, this, other, false));
    }

    #[test]
    fn compare_differs_on_any_field() {
        let mut e = Engine::new();
        let this = addon_node(&mut e, 4, 5, 6);
        let other = addon_node(&mut e, 4, 5, 6);
        assert!(compare_with(&mut e, this, other, true), "components");
        let other = addon_node(&mut e, 9, 5, 6);
        assert!(compare_with(&mut e, this, other, false), "index");
        let other = addon_node(&mut e, 4, 9, 6);
        assert!(compare_with(&mut e, this, other, false), "sound");
        let other = addon_node(&mut e, 4, 5, 9);
        assert!(compare_with(&mut e, this, other, false), "data");
    }

    #[test]
    fn compare_differs_for_another_form_type() {
        let mut e = Engine::new();
        let this = addon_node(&mut e, 4, 5, 6);
        let other = Ptr::<()>::new(e.mem.alloc(0x60));
        cast_double(&mut e, vec![]);
        e.register(COMPARE_ALL_COMPONENTS, |_, _| panic!("compared"));
        assert!(e.call(0x004f_9110, &args![this, other]).bool());
    }

    #[test]
    fn sound_handle_extra_construct_sets_name_vtable_and_handle() {
        let mut e = Engine::new();
        e.map(0x011c_8000, 0x1000);
        e.mem.set_u32(SOUND_HANDLE_EXTRA_TAG, 0x0abc);
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(SOUND_HANDLE_CONSTRUCT, |_, _| Ret::default());
        let this = Ptr::<()>::new(e.mem.alloc(0x18));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_91b0, &args![this]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (EXTRA_DATA_CONSTRUCT, vec![this.addr(), 0x0abc]));
        assert_eq!(log[2], (SOUND_HANDLE_CONSTRUCT, vec![this.addr() + 0xc]));
        assert_eq!(e.mem.u32(this.addr()), SOUND_HANDLE_EXTRA_VTABLE);
    }

    #[test]
    fn get_rtti_returns_the_static_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004f_9220, &args![0u32]).u32(), 0x011c_806c);
    }

    #[test]
    fn tag_is_stored_read_and_cleared() {
        let mut e = Engine::new();
        e.map(0x011c_8000, 0x1000);
        let block = e.mem.alloc(4);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a, [4]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(FIXED_STRING_CONSTRUCT, |_, a| {
            assert_eq!(a[1], SOUND_HANDLE_EXTRA_TAG_TEXT);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.call(0x004f_92e0, &[]);
        assert_eq!(e.call(0x004f_9230, &[]).u32(), block);
        e.register(FIXED_STRING_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_9360, &[]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (FIXED_STRING_DELETE, vec![block, 1])
        );
        assert_eq!(e.call(0x004f_9230, &[]).u32(), 0);
    }

    #[test]
    fn tag_is_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.map(0x011c_8000, 0x1000);
        e.mem.set_u32(SOUND_HANDLE_EXTRA_TAG, 0x1234);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call(0x004f_92e0, &[]);
        assert_eq!(e.mem.u32(SOUND_HANDLE_EXTRA_TAG), 0);
    }

    #[test]
    fn destroying_a_missing_tag_deletes_nothing() {
        let mut e = Engine::new();
        e.map(0x011c_8000, 0x1000);
        e.register(FIXED_STRING_DELETE, |_, _| panic!("deleted"));
        e.call(0x004f_9360, &[]);
        assert_eq!(e.mem.u32(SOUND_HANDLE_EXTRA_TAG), 0);
    }

    #[test]
    fn sound_handle_extra_destructor_stops_the_sound_then_unwinds() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_DESTRUCT,
                EXTRA_DATA_DESTRUCT,
            ],
        );
        let this = Ptr::<()>::new(e.mem.alloc(0x18));
        e.call_log = Some(vec![]);
        e.call(0x004f_9270, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr()), SOUND_HANDLE_EXTRA_VTABLE);
        assert_eq!(log[1], (SOUND_HANDLE_STOP, vec![this.addr() + 0xc]));
        assert_eq!(log[2], (SOUND_HANDLE_DESTRUCT, vec![this.addr() + 0xc]));
        assert_eq!(log[3], (EXTRA_DATA_DESTRUCT, vec![this.addr()]));
    }

    #[test]
    fn sound_handle_extra_deleting_destructor_frees_with_the_flag() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_DESTRUCT,
                EXTRA_DATA_DESTRUCT,
                FREE_SIZED,
            ],
        );
        let this = Ptr::<()>::new(e.mem.alloc(0x18));
        e.call_log = Some(vec![]);
        e.call(0x004f_9240, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_9240, &args![this, 1u32]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[4], (FREE_SIZED, vec![this.addr(), 0x18]));
    }

    #[test]
    fn thunk_calls_the_whole_objects_deleting_destructor() {
        let mut e = Engine::new();
        e.register(BGS_ADDON_NODE_DELETING_DESTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x004f_93a0, &args![0x5030u32, 1u32]);
        assert_eq!(result.u32(), 0x5000);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (BGS_ADDON_NODE_DELETING_DESTRUCTOR, vec![0x5000, 1])
        );
    }

    #[test]
    fn constructible_object_construct_builds_in_order() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[CONSTRUCTIBLE_OBJECT_BASE_CONSTRUCT, SET_FORM_TYPE],
        );
        let this = Ptr::<()>::new(e.mem.alloc(0xc4));
        e.mem.set_u32(this.addr() + 0xac, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_93b0, &args![this]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (CONSTRUCTIBLE_OBJECT_BASE_CONSTRUCT, vec![this.addr()])
        );
        assert_eq!(log[2], (SET_FORM_TYPE, vec![this.addr(), 0x32]));
        assert_eq!(e.mem.u32(this.addr()), 0x0102_45a4);
        assert_eq!(e.mem.u32(this.addr() + 0x9c), 0x0102_44c4);
        assert_eq!(e.mem.u32(this.addr() + 0xac), 0, "InitializeData ran");
    }

    #[test]
    fn constructible_object_destructor_resets_vtables_then_runs_the_base() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[SOUND_HANDLE_DESTRUCT, CONSTRUCTIBLE_OBJECT_BASE_DESTRUCT],
        );
        let this = Ptr::<()>::new(e.mem.alloc(0xc4));
        e.call_log = Some(vec![]);
        e.call(0x004f_94b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0x0102_458c);
        assert_eq!(e.mem.u32(this.addr() + 0x68), 0x0102_4534);
        assert_eq!(log[1], (SOUND_HANDLE_DESTRUCT, vec![this.addr()]));
        assert_eq!(
            log[2],
            (CONSTRUCTIBLE_OBJECT_BASE_DESTRUCT, vec![this.addr()])
        );
    }

    #[test]
    fn constructible_object_deleting_destructor_frees_with_the_flag() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                SOUND_HANDLE_DESTRUCT,
                CONSTRUCTIBLE_OBJECT_BASE_DESTRUCT,
                OPERATOR_DELETE,
            ],
        );
        let this = Ptr::<()>::new(e.mem.alloc(0xc4));
        e.call_log = Some(vec![]);
        e.call(0x004f_9480, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_9480, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(
            e.call_log.take().unwrap()[3],
            (OPERATOR_DELETE, vec![this.addr()])
        );
    }

    #[test]
    fn initialize_data_clears_the_two_pointers() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(0xc4));
        e.mem.set_u32(this.addr() + 0xac, 5);
        e.mem.set_u32(this.addr() + 0xb0, 6);
        e.mem.set_u32(this.addr() + 0xb4, 7);
        e.call(0x004f_9570, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0xac), 0);
        assert_eq!(e.mem.u32(this.addr() + 0xb0), 0);
        assert_eq!(e.mem.u32(this.addr() + 0xb4), 7);
    }
}
