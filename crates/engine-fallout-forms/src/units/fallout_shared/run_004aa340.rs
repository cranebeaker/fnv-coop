//! `fallout shared/run_004aa340` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The ten functions are the members of `BGSTextureUseMap` (Xbox PDB,
//! 0x14 bytes: `pDefaultUser`, then the map `UserMap`
//! `NiTMap<NiTexture *, TEX_USER_DATA *>` at +4) and of `TEX_USER_DATA`
//! (0x10 bytes) that sit between the `bgssceneinfo.cpp` code and
//! `BGSTextureUseMap::WriteToFile` (`004aa900`), so the Xbox PDB gives no
//! names for them. What identifies them: the callback block of
//! `004aa3d0` is the PDB's `BGSTextureUseMap::TexUseCBParam` (map, form)
//! and the sizes match the PDB structs; the callers are
//! `Script::OutputTextureUseMap` (`005b6800`) and the report writer
//! `004a8bb0`. Names stay `fn_<address>`.
//!
//! The map is accessed through its helper functions by address: first
//! position `004b9ba0`, next entry `006b7f20` (`pos`, `key`, `value`
//! out-parameters), lookup `00853130`, `SetAt` `00844700`, `RemoveAll`
//! `00438af0`, destructor body `004a9850`. Exception-unwinding frames are not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, NiTMap};

/// `operator new` (cdecl, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Address of the first entry of the map in `ECX`, or 0 when it is empty.
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
/// `NiTMapBase::GetNext` (`ECX` = map): `pos` (in/out), `key` (out), value
/// (out), each by address.
const MAP_GET_NEXT: u32 = 0x006b_7f20;
/// Looks a key up in the map in `ECX`: key, value (out); true when found.
const MAP_LOOKUP: u32 = 0x0085_3130;
/// `NiTMapBase::SetAt` (`ECX` = map): key, value.
const MAP_SET_AT: u32 = 0x0084_4700;
/// Removes every entry of the map in `ECX`.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// The destructor body of the map in `ECX`.
const MAP_DESTRUCT: u32 = 0x004a_9850;
/// `NiPointer` constructor (`ECX` = pointer slot; one argument, the object).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer` assignment (`ECX` = slot; one argument, the object).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer` destructor (`ECX` = slot).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The list constructor (zeroes the head node, `ECX` = list).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Removes every item of the list in `ECX`.
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// The list destructor (`ECX` = list).
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Whether the list in `ECX` holds the value whose address is passed.
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// Adds the value whose address is passed to the list in `ECX`.
const LIST_ADD: u32 = 0x005a_e3d0;
/// Tests flag bit 0 of the object in `ECX`; true when the flag is set.
const OBJECT_FLAG_SET: u32 = 0x0045_6610;
/// Returns the dword at `this + 0xCC` of a fade node (the code treats it as
/// the reference whose model the node is).
const FADE_NODE_REFERENCE: u32 = 0x009a_d610;
/// Lookup in the hash table in `ECX`: key, byte out-parameter; true when
/// the key is there.
const FORM_TABLE_LOOKUP: u32 = 0x0057_c850;
/// Returns the dword at `this + 0x20` (the engine map names this body
/// `BGSSaveFormBuffer::GetForm`; for a reference it is the base form).
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
/// Returns the byte at `this + 4`: `TESForm::cFormType`.
const FORM_TYPE: u32 = 0x0040_1170;
/// A test on a reference (`ECX`); true skips the reference.
const REFERENCE_IS_SKIPPED: u32 = 0x0044_4ed0;
/// Number of children of a node (`this + 0x9C` array's size, `ECX` = node).
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
/// A child of a node by index (`ECX` = node, one argument).
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
/// `NiAVObject::GetProperty` (Xbox PDB): `this`, property type.
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// Returns the dword at `this + 0x1C` (the engine map's folded name is
/// `PathingLocation::GetWorldspace`).
const PROPERTY_FIELD_1C: u32 = 0x0044_1110;

/// Global: the hash table the filter lookup `FORM_TABLE_LOOKUP` runs on
/// (its role is not identified); 0 when there is none.
const FORM_FILTER_TABLE: u32 = 0x011c_5948;
/// Global: the form that is the current user while a reference's node tree is
/// walked, 0 outside.
const CURRENT_USER_FORM: u32 = 0x011c_5cac;

/// Virtual slot of `IsNode`, `IsFadeNode` and `IsGeometry` in the node's
/// vtable (Xbox PDB names, offsets as the PC code uses them).
const SLOT_IS_NODE: u32 = 0x0c;
const SLOT_IS_FADE_NODE: u32 = 0x10;
const SLOT_IS_GEOMETRY: u32 = 0x18;
/// `BSShaderProperty::GetTextureUse` (Xbox PDB), the virtual that calls
/// back with each texture of the property.
const SLOT_GET_TEXTURE_USE: u32 = 0xbc;
/// The property type `NiAVObject::GetProperty` is asked for.
const SHADER_PROPERTY_TYPE: u32 = 3;
/// The type numbers (`TESForm::cFormType`) of the base forms that are only
/// followed when the walk's flag is set: 0x2a to 0x2d.
const FORM_TYPE_FIRST: u32 = 0x2a;
const FORM_TYPE_LAST: u32 = 0x2d;
/// The size of a `TEX_USER_DATA`.
const TEX_USER_DATA_SIZE: u32 = 0x10;

layout! {
    /// `BGSTextureUseMap` (Xbox PDB), 0x14 bytes.
    pub struct BGSTextureUseMap: 0x14 {
        /// `pDefaultUser` (Xbox PDB): `TESForm *`.
        0x00 pDefaultUser: Ptr,
        /// `UserMap` (Xbox PDB): `NiTMap<NiTexture *, TEX_USER_DATA *>`.
        0x04 UserMap: crate::ptr::Inline<NiTMap>,
    }

    /// `TEX_USER_DATA` (Xbox PDB), 0x10 bytes.
    #[allow(non_camel_case_types)]
    pub struct TEX_USER_DATA: 0x10 {
        /// `spTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0x00 spTexture: Ptr,
        /// `ulTextureSize` (Xbox PDB).
        0x04 ulTextureSize: u32,
        /// `UserList` (Xbox PDB): `BSSimpleList<TESForm *>`.
        0x08 UserList: crate::ptr::Inline<BSSimpleList>,
    }

    /// `BGSTextureUseMap::TexUseCBParam` (Xbox PDB), 8 bytes.
    pub struct TexUseCBParam: 0x08 {
        /// `pTextureUseMap` (Xbox PDB).
        0x00 pTextureUseMap: Ptr,
        /// `pForm` (Xbox PDB): `TESForm *`.
        0x04 pForm: Ptr,
    }
}

/// Visits every entry of the map at `map`: `visit(engine, key, value)`.
fn for_each_entry(e: &mut Engine, map: Ptr, mut visit: impl FnMut(&mut Engine, u32, u32)) {
    let mut position = e.call(MAP_FIRST_POSITION, &args![map]).u32();
    while position != 0 {
        let (next, key, value) = e.with_stack(12, |e, block| {
            let slots = block.addr();
            e.mem.set_u32(slots, position);
            e.mem.set_u32(slots + 4, 0);
            e.mem.set_u32(slots + 8, 0);
            e.call(MAP_GET_NEXT, &args![map, slots, slots + 4, slots + 8]);
            (e.mem.u32(slots), e.mem.u32(slots + 4), e.mem.u32(slots + 8))
        });
        visit(e, key, value);
        position = next;
    }
}

// Translated from 004aa340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004aa340 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// The destructor body of `BGSTextureUseMap`: removes every entry (deleting
/// the `TEX_USER_DATA`s), then runs the map's destructor body.
pub fn fn_004aa340(e: &mut Engine, this: Ptr<BGSTextureUseMap>) {
    fn_004aa5d0(e, this);
    let map = this.at(BGSTextureUseMap::UserMap);
    e.call(MAP_DESTRUCT, &args![map]);
}

// Translated from 004aa3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the default user and collects the textures used by the node tree
/// below `node` (`fn_004aa3d0`); `flag` is that walk's flag.
pub fn fn_004aa3a0(
    e: &mut Engine,
    this: Ptr<BGSTextureUseMap>,
    node: Ptr,
    default_user: Ptr,
    flag: u8,
) {
    e.set(this, BGSTextureUseMap::pDefaultUser, default_user);
    fn_004aa3d0(e, this, node, flag);
}

// Translated from 004aa3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the scene graph below `node` and records which form uses which
/// texture.
///
/// Nothing happens for a null node or one whose flag bit 0 is set. A node
/// (virtual `IsNode`) is a fade node (`IsFadeNode`) of a reference: that
/// reference's form is looked up in the global filter table; unless it is
/// listed there, a base form of type `0x2a..=0x2d` is only followed when
/// `flag` is set, a reference for which `00444ed0` answers true is skipped,
/// and otherwise the reference becomes the current user while its children
/// are walked (the global is cleared afterwards). A node without a fade
/// reference walks its children. Anything that is not a node but is
/// geometry (`IsGeometry`) with a shader property (property type 3) whose
/// field `+0x1C` is not -1 gets its `GetTextureUse` (virtual `0xBC`) called
/// with `fn_004aa6f0` and a `TexUseCBParam` of this map and the current
/// user (the default user when there is none).
pub fn fn_004aa3d0(e: &mut Engine, this: Ptr<BGSTextureUseMap>, node: Ptr, flag: u8) {
    if node.is_null() {
        return;
    }
    if e.call(OBJECT_FLAG_SET, &args![node]).bool() {
        return;
    }
    let as_node: Ptr = e.vcall(node.addr(), SLOT_IS_NODE, &args![]).ptr();
    if !as_node.is_null() {
        let mut in_filter_table = false;
        let mut set_current_user = false;
        let mut fade_node = Ptr::NULL;
        if e.vcall(as_node.addr(), SLOT_IS_FADE_NODE, &args![]).u32() != 0 {
            fade_node = as_node;
        }
        if !fade_node.is_null() {
            let form = e.call(FADE_NODE_REFERENCE, &args![fade_node]).u32();
            let table = e.global::<u32>(FORM_FILTER_TABLE);
            if table != 0 {
                e.with_stack(1, |e, found| {
                    e.mem.set_u8(found.addr(), 0);
                    e.call(FORM_TABLE_LOOKUP, &args![table, form, found]);
                    in_filter_table = e.mem.u8(found.addr()) != 0;
                });
            }
            if form != 0 && !in_filter_table {
                let base_form = e.call(REFERENCE_BASE_FORM, &args![form]).u32();
                let form_type = e.call(FORM_TYPE, &args![base_form]).u32() & 0xff;
                let follows_only_with_flag =
                    (FORM_TYPE_FIRST..=FORM_TYPE_LAST).contains(&form_type);
                if follows_only_with_flag && flag == 0 {
                    return;
                }
                if e.call(REFERENCE_IS_SKIPPED, &args![form]).bool() {
                    return;
                }
                set_current_user = true;
                e.set_global(CURRENT_USER_FORM, form);
            }
        }
        if !in_filter_table {
            let count = e.call(NODE_CHILD_COUNT, &args![as_node]).u32();
            for index in 0..count {
                let child = e.call(NODE_CHILD_AT, &args![as_node, index]).u32();
                fn_004aa3d0(e, this, Ptr::new(child), flag);
            }
        }
        if set_current_user {
            e.set_global(CURRENT_USER_FORM, 0u32);
        }
    } else {
        let geometry: Ptr = e.vcall(node.addr(), SLOT_IS_GEOMETRY, &args![]).ptr();
        if geometry.is_null() {
            return;
        }
        let property: Ptr = e
            .call(GET_PROPERTY, &args![geometry, SHADER_PROPERTY_TYPE])
            .ptr();
        let shader_property: Ptr = if property.is_null() {
            Ptr::NULL
        } else if e.call(PROPERTY_FIELD_1C, &args![property]).i32() != -1 {
            property
        } else {
            Ptr::NULL
        };
        if shader_property.is_null() {
            return;
        }
        let current_user = e.global::<u32>(CURRENT_USER_FORM);
        let user = if current_user != 0 {
            current_user
        } else {
            e.get(this, BGSTextureUseMap::pDefaultUser).addr()
        };
        e.with_stack(8, |e, param| {
            e.mem.set_u32(param.addr(), this.addr());
            e.mem.set_u32(param.addr() + 4, user);
            e.vcall(
                shader_property.addr(),
                SLOT_GET_TEXTURE_USE,
                &args![0x004a_a6f0u32, param],
            );
        });
    }
}

// Translated from 004aa5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every `TEX_USER_DATA` of the map (scalar deleting destructor with
/// the delete flag) and removes all entries.
pub fn fn_004aa5d0(e: &mut Engine, this: Ptr<BGSTextureUseMap>) {
    let map = this.at(BGSTextureUseMap::UserMap);
    for_each_entry(e, map.cast(), |e, _, value| {
        if value != 0 {
            fn_004aa650(e, Ptr::new(value), 1);
        }
    });
    e.call(MAP_REMOVE_ALL, &args![map]);
}

// Translated from 004aa650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `TEX_USER_DATA`: the destructor body,
/// then the block is freed when bit 0 of `flags` is set.
pub fn fn_004aa650(e: &mut Engine, this: Ptr<TEX_USER_DATA>, flags: u32) -> Ptr<TEX_USER_DATA> {
    fn_004aa680(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004aa680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004aa680 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// The destructor body of `TEX_USER_DATA`: empties and destroys the user
/// list, then releases the texture pointer.
pub fn fn_004aa680(e: &mut Engine, this: Ptr<TEX_USER_DATA>) {
    let list = this.at(TEX_USER_DATA::UserList);
    e.call(LIST_REMOVE_ALL, &args![list]);
    e.call(LIST_DESTRUCT, &args![list]);
    e.call(NI_POINTER_DESTRUCT, &args![this]);
}

// Translated from 004aa6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `fn_004aa3d0` hands to `GetTextureUse` (cdecl: texture,
/// texture size, a `TexUseCBParam`): when the parameter names a map, records
/// the texture, its size and the parameter's form in it (`fn_004aa720`).
pub fn fn_004aa6f0(e: &mut Engine, texture: Ptr, size: u32, param: Ptr<TexUseCBParam>) {
    let map: Ptr<BGSTextureUseMap> = e.get(param, TexUseCBParam::pTextureUseMap).cast();
    if map.is_null() {
        return;
    }
    let form = e.get(param, TexUseCBParam::pForm);
    fn_004aa720(e, map, texture, size, form);
}

// Translated from 004aa720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004aa720 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// Records that `form` uses `texture` (of `size`) in the map. Nothing happens
/// for a null texture or form. A texture without an entry gets a new
/// `TEX_USER_DATA` (0x10 bytes) that holds the texture and its size; then the
/// form is added to the entry's user list unless it is there already.
pub fn fn_004aa720(
    e: &mut Engine,
    this: Ptr<BGSTextureUseMap>,
    texture: Ptr,
    size: u32,
    form: Ptr,
) {
    if texture.is_null() || form.is_null() {
        return;
    }
    let map = this.at(BGSTextureUseMap::UserMap);
    let mut data: Ptr<TEX_USER_DATA> = Ptr::NULL;
    let found = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let found = e.call(MAP_LOOKUP, &args![map, texture, slot]).bool();
        data = Ptr::new(e.mem.u32(slot.addr()));
        found
    });
    if !found || data.is_null() {
        let block: Ptr<TEX_USER_DATA> = e.call(OPERATOR_NEW, &args![TEX_USER_DATA_SIZE]).ptr();
        data = if block.is_null() {
            Ptr::NULL
        } else {
            fn_004aa830(e, block)
        };
        e.call(MAP_SET_AT, &args![map, texture, data]);
        e.call(NI_POINTER_ASSIGN, &args![data, texture]);
        e.set(data, TEX_USER_DATA::ulTextureSize, size);
    }
    let list = data.at(TEX_USER_DATA::UserList);
    e.with_stack(4, |e, form_slot| {
        e.mem.set_u32(form_slot.addr(), form.addr());
        if !e.call(LIST_CONTAINS, &args![list, form_slot]).bool() {
            e.call(LIST_ADD, &args![list, form_slot]);
        }
    });
}

// Translated from 004aa830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004aa830 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// The `TEX_USER_DATA` constructor: a null texture pointer and an empty
/// user list. The size is not initialized.
pub fn fn_004aa830(e: &mut Engine, this: Ptr<TEX_USER_DATA>) -> Ptr<TEX_USER_DATA> {
    e.call(NI_POINTER_CONSTRUCT, &args![this, 0]);
    let list = this.at(TEX_USER_DATA::UserList);
    e.call(LIST_CONSTRUCT, &args![list]);
    this
}

// Translated from 004aa890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The total of the `ulTextureSize` of every entry in the map.
pub fn fn_004aa890(e: &mut Engine, this: Ptr<BGSTextureUseMap>) -> u32 {
    let map = this.at(BGSTextureUseMap::UserMap);
    let mut total = 0u32;
    for_each_entry(e, map.cast(), |e, _, value| {
        if value != 0 {
            total = total.wrapping_add(e.mem.u32(value + 4));
        }
    });
    total
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004aa340, fn_004aa340(Ptr<BGSTextureUseMap>)),
        entry!(0x004aa3a0, fn_004aa3a0(Ptr<BGSTextureUseMap>, Ptr, Ptr, u8)),
        entry!(0x004aa3d0, fn_004aa3d0(Ptr<BGSTextureUseMap>, Ptr, u8)),
        entry!(0x004aa5d0, fn_004aa5d0(Ptr<BGSTextureUseMap>)),
        entry!(
            0x004aa650,
            fn_004aa650(Ptr<TEX_USER_DATA>, u32) -> Ptr<TEX_USER_DATA>
        ),
        entry!(0x004aa680, fn_004aa680(Ptr<TEX_USER_DATA>)),
        entry!(0x004aa6f0, fn_004aa6f0(Ptr, u32, Ptr<TexUseCBParam>)),
        entry!(
            0x004aa720,
            fn_004aa720(Ptr<BGSTextureUseMap>, Ptr, u32, Ptr)
        ),
        entry!(
            0x004aa830,
            fn_004aa830(Ptr<TEX_USER_DATA>) -> Ptr<TEX_USER_DATA>
        ),
        entry!(0x004aa890, fn_004aa890(Ptr<BGSTextureUseMap>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Entries = Rc<RefCell<Vec<(u32, u32)>>>;

    /// A model of the map functions on a list of (key, value) entries: a
    /// position is the entry's index plus one, 0 meaning the end.
    fn engine_with_map() -> (Engine, Entries) {
        let mut e = Engine::new();
        let entries: Entries = Rc::new(RefCell::new(Vec::new()));
        let list = entries.clone();
        e.register_double(MAP_FIRST_POSITION, move |_, _| Ret {
            eax: (!list.borrow().is_empty()) as u32,
            ..Ret::default()
        });
        let list = entries.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let position = e.mem.u32(a[1]);
            let (key, value) = list.borrow()[position as usize - 1];
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            let next = if (position as usize) < list.borrow().len() {
                position + 1
            } else {
                0
            };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let list = entries.clone();
        e.register_double(MAP_LOOKUP, move |e, a| {
            let found = list.borrow().iter().find(|(key, _)| *key == a[1]).copied();
            if let Some((_, value)) = found {
                e.mem.set_u32(a[2], value);
            }
            Ret {
                eax: found.is_some() as u32,
                ..Ret::default()
            }
        });
        let list = entries.clone();
        e.register_double(MAP_SET_AT, move |_, a| {
            list.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        let list = entries.clone();
        e.register_double(MAP_REMOVE_ALL, move |_, _| {
            list.borrow_mut().clear();
            Ret::default()
        });
        e.register(MAP_DESTRUCT, |_, _| Ret::default());
        e.register_double(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(NI_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(LIST_CONTAINS, |e, a| {
            // The list is modelled by its head node's item chain.
            let wanted = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == wanted {
                    return Ret {
                        eax: 1,
                        ..Ret::default()
                    };
                }
                node = e.mem.u32(node + 4);
            }
            Ret::default()
        });
        e.register(LIST_ADD, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, wanted);
            } else {
                let fresh = e.mem.alloc(8);
                e.mem.set_u32(fresh, wanted);
                e.mem.set_u32(node + 4, fresh);
            }
            Ret::default()
        });
        e.map(0x011c_5000, 0x1000);
        (e, entries)
    }

    fn texture_map(e: &mut Engine) -> Ptr<BGSTextureUseMap> {
        e.new_object()
    }

    fn user_data(e: &mut Engine, size: u32) -> u32 {
        let data = e.mem.alloc(TEX_USER_DATA_SIZE);
        e.mem.set_u32(data + 4, size);
        data
    }

    #[test]
    fn total_size_adds_every_entry() {
        let (mut e, entries) = engine_with_map();
        let map = texture_map(&mut e);
        assert_eq!(e.call(0x004a_a890, &args![map]).u32(), 0);
        for (key, size) in [(1, 100), (2, 250), (3, 7)] {
            let data = user_data(&mut e, size);
            entries.borrow_mut().push((key, data));
        }
        assert_eq!(e.call(0x004a_a890, &args![map]).u32(), 357);
    }

    #[test]
    fn clearing_deletes_each_entry_then_removes_all() {
        let (mut e, entries) = engine_with_map();
        let map = texture_map(&mut e);
        let first = user_data(&mut e, 1);
        let second = user_data(&mut e, 2);
        entries.borrow_mut().extend([(1, first), (2, second)]);
        entries.borrow_mut().push((3, 0));
        e.call_log = Some(vec![]);
        e.call(0x004a_a5d0, &args![map]);
        let log = e.call_log.take().unwrap();
        let deleted: Vec<u32> = log
            .iter()
            .filter(|(addr, _)| *addr == OPERATOR_DELETE)
            .map(|(_, a)| a[0])
            .collect();
        assert_eq!(deleted, vec![first, second]);
        assert!(entries.borrow().is_empty());
        assert_eq!(log.last().unwrap().0, MAP_REMOVE_ALL);
    }

    #[test]
    fn destructor_body_clears_then_destroys_the_map() {
        let (mut e, entries) = engine_with_map();
        let map = texture_map(&mut e);
        let data = user_data(&mut e, 1);
        entries.borrow_mut().push((1, data));
        e.call_log = Some(vec![]);
        e.call(0x004a_a340, &args![map]);
        let log = e.call_log.take().unwrap();
        let destruct = log.iter().find(|(addr, _)| *addr == MAP_DESTRUCT).unwrap();
        assert_eq!(destruct.1[0], map.addr() + 4);
        assert!(entries.borrow().is_empty());
    }

    #[test]
    fn user_data_destructors() {
        let (mut e, _) = engine_with_map();
        let data = user_data(&mut e, 5);
        e.call_log = Some(vec![]);
        let back = e.call(0x004a_a650, &args![data, 0u32]).u32();
        assert_eq!(back, data);
        let log = e.call_log.take().unwrap();
        // List emptied and destroyed, then the texture pointer released.
        let order: Vec<(u32, u32)> = log.iter().skip(1).map(|(addr, a)| (*addr, a[0])).collect();
        assert_eq!(
            order,
            vec![
                (LIST_REMOVE_ALL, data + 8),
                (LIST_DESTRUCT, data + 8),
                (NI_POINTER_DESTRUCT, data)
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x004a_a650, &args![data, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(OPERATOR_DELETE, vec![data]));
    }

    #[test]
    fn user_data_constructor_clears_pointer_and_list() {
        let (mut e, _) = engine_with_map();
        let data = e.mem.alloc(TEX_USER_DATA_SIZE);
        e.mem.set_u32(data + 8, 0x55);
        e.mem.set_u32(data + 12, 0x66);
        e.mem.set_u32(data + 4, 0x77);
        let back = e.call(0x004a_a830, &args![data]).u32();
        assert_eq!(back, data);
        assert_eq!(e.mem.u32(data), 0);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(data + 12), 0);
        // The size is not initialized.
        assert_eq!(e.mem.u32(data + 4), 0x77);
    }

    #[test]
    fn recording_a_new_texture_creates_an_entry_with_one_user() {
        let (mut e, entries) = engine_with_map();
        let map = texture_map(&mut e);
        e.call(0x004a_a720, &args![map, 0x1000u32, 4096u32, 0x2000u32]);
        let list = entries.borrow().clone();
        assert_eq!(list.len(), 1);
        let (key, data) = list[0];
        assert_eq!(key, 0x1000);
        assert_eq!(e.mem.u32(data), 0x1000);
        assert_eq!(e.mem.u32(data + 4), 4096);
        assert_eq!(e.mem.u32(data + 8), 0x2000);
    }

    #[test]
    fn recording_again_adds_new_users_once() {
        let (mut e, entries) = engine_with_map();
        let map = texture_map(&mut e);
        e.call(0x004a_a720, &args![map, 0x1000u32, 4096u32, 0x2000u32]);
        e.call(0x004a_a720, &args![map, 0x1000u32, 9999u32, 0x2000u32]);
        e.call(0x004a_a720, &args![map, 0x1000u32, 9999u32, 0x3000u32]);
        let list = entries.borrow().clone();
        assert_eq!(list.len(), 1);
        let data = list[0].1;
        // The size of the first sighting stays.
        assert_eq!(e.mem.u32(data + 4), 4096);
        assert_eq!(e.mem.u32(data + 8), 0x2000);
        let second = e.mem.u32(data + 12);
        assert_eq!(e.mem.u32(second), 0x3000);
        assert_eq!(e.mem.u32(second + 4), 0);
    }

    #[test]
    fn recording_ignores_a_null_texture_or_form() {
        let (mut e, entries) = engine_with_map();
        let map = texture_map(&mut e);
        e.call(0x004a_a720, &args![map, 0u32, 1u32, 0x2000u32]);
        e.call(0x004a_a720, &args![map, 0x1000u32, 1u32, 0u32]);
        assert!(entries.borrow().is_empty());
    }

    #[test]
    fn callback_forwards_only_with_a_map() {
        let (mut e, entries) = engine_with_map();
        let map = texture_map(&mut e);
        let param = e.mem.alloc(8);
        e.mem.set_u32(param + 4, 0x2000);
        e.call(0x004a_a6f0, &args![0x1000u32, 64u32, param]);
        assert!(entries.borrow().is_empty());
        e.mem.set_u32(param, map.addr());
        e.call(0x004a_a6f0, &args![0x1000u32, 64u32, param]);
        let list = entries.borrow().clone();
        assert_eq!(list.len(), 1);
        assert_eq!(e.mem.u32(list[0].1 + 4), 64);
        assert_eq!(e.mem.u32(list[0].1 + 8), 0x2000);
    }

    /// A scene of objects with a vtable of `IsNode` (+0xC), `IsFadeNode`
    /// (+0x10), `IsGeometry` (+0x18) and `GetTextureUse` (+0xBC) doubles.
    /// Object fields used by the doubles: +8 kind (1 node, 2 geometry),
    /// +0xC fade flag, +0x10 shader property, +0x30 flags, +0xCC reference.
    struct Scene {
        children: Rc<RefCell<HashMap<u32, Vec<u32>>>>,
        callbacks: Rc<RefCell<Vec<(u32, u32, u32)>>>,
        vtable: u32,
    }

    fn scene(e: &mut Engine) -> Scene {
        let children: Rc<RefCell<HashMap<u32, Vec<u32>>>> = Rc::default();
        let callbacks: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::default();
        e.register(0x0a00_0001, |e, a| Ret {
            eax: if e.mem.u32(a[0] + 8) == 1 { a[0] } else { 0 },
            ..Ret::default()
        });
        e.register(0x0a00_0002, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x0c),
            ..Ret::default()
        });
        e.register(0x0a00_0003, |e, a| Ret {
            eax: if e.mem.u32(a[0] + 8) == 2 { a[0] } else { 0 },
            ..Ret::default()
        });
        let sink = callbacks.clone();
        e.register_double(0x0a00_0004, move |e, a| {
            // (property, callback, parameter): the parameter block is read now.
            sink.borrow_mut()
                .push((a[1], e.mem.u32(a[2]), e.mem.u32(a[2] + 4)));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x0c, 0x0a00_0001);
        e.mem.set_u32(vtable + 0x10, 0x0a00_0002);
        e.mem.set_u32(vtable + 0x18, 0x0a00_0003);
        e.mem.set_u32(vtable + 0xbc, 0x0a00_0004);
        e.register(OBJECT_FLAG_SET, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30) & 1,
            ..Ret::default()
        });
        e.register(FADE_NODE_REFERENCE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xcc),
            ..Ret::default()
        });
        e.register(FORM_TABLE_LOOKUP, |e, a| {
            // The table holds forms in a list; a[0] is its address.
            let mut listed = false;
            let mut node = e.mem.u32(a[0]);
            while node != 0 {
                if e.mem.u32(node) == a[1] {
                    listed = true;
                }
                node = e.mem.u32(node + 4);
            }
            if listed {
                e.mem.set_u8(a[2], 1);
            }
            Ret {
                eax: listed as u32,
                ..Ret::default()
            }
        });
        e.register(REFERENCE_BASE_FORM, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x20),
            ..Ret::default()
        });
        e.register(FORM_TYPE, |e, a| Ret {
            eax: e.mem.u8(a[0] + 4) as u32,
            ..Ret::default()
        });
        e.register(REFERENCE_IS_SKIPPED, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        let table = children.clone();
        e.register_double(NODE_CHILD_COUNT, move |_, a| Ret {
            eax: table
                .borrow()
                .get(&a[0])
                .map_or(0, |list| list.len() as u32),
            ..Ret::default()
        });
        let table = children.clone();
        e.register_double(NODE_CHILD_AT, move |_, a| Ret {
            eax: table.borrow()[&a[0]][a[1] as usize],
            ..Ret::default()
        });
        e.register(GET_PROPERTY, |e, a| {
            assert_eq!(a[1], SHADER_PROPERTY_TYPE);
            Ret {
                eax: e.mem.u32(a[0] + 0x10),
                ..Ret::default()
            }
        });
        e.register(PROPERTY_FIELD_1C, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x1c),
            ..Ret::default()
        });
        Scene {
            children,
            callbacks,
            vtable,
        }
    }

    fn object(e: &mut Engine, scene: &Scene, kind: u32) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, scene.vtable);
        e.mem.set_u32(object + 8, kind);
        object
    }

    /// A geometry object with a shader property whose field `+0x1C` is
    /// `property_field`.
    fn geometry(e: &mut Engine, scene: &Scene, property_field: u32) -> u32 {
        let object = object(e, scene, 2);
        let property = e.mem.alloc(0x40);
        e.mem.set_u32(property, scene.vtable);
        e.mem.set_u32(property + 0x1c, property_field);
        e.mem.set_u32(object + 0x10, property);
        object
    }

    #[test]
    fn walking_visits_geometry_with_the_default_user() {
        let (mut e, _) = engine_with_map();
        let scene = scene(&mut e);
        let map = texture_map(&mut e);
        e.set(map, BGSTextureUseMap::pDefaultUser, Ptr::new(0xd0d0));
        let root = object(&mut e, &scene, 1);
        let first = geometry(&mut e, &scene, 0);
        let second = geometry(&mut e, &scene, 5);
        // A property whose field is -1, and one without a property.
        let hidden = geometry(&mut e, &scene, u32::MAX);
        let bare = object(&mut e, &scene, 2);
        scene
            .children
            .borrow_mut()
            .insert(root, vec![first, second, hidden, bare]);
        e.call(0x004a_a3d0, &args![map, root, 0u32]);
        assert_eq!(
            *scene.callbacks.borrow(),
            vec![
                (0x004a_a6f0, map.addr(), 0xd0d0),
                (0x004a_a6f0, map.addr(), 0xd0d0)
            ]
        );
    }

    #[test]
    fn walking_ignores_null_and_flagged_nodes() {
        let (mut e, _) = engine_with_map();
        let scene = scene(&mut e);
        let map = texture_map(&mut e);
        e.call(0x004a_a3d0, &args![map, 0u32, 0u32]);
        let flagged = geometry(&mut e, &scene, 0);
        e.mem.set_u32(flagged + 0x30, 1);
        e.call(0x004a_a3d0, &args![map, flagged, 0u32]);
        assert!(scene.callbacks.borrow().is_empty());
        // An object that is neither a node nor geometry does nothing.
        let other = object(&mut e, &scene, 3);
        e.call(0x004a_a3d0, &args![map, other, 0u32]);
        assert!(scene.callbacks.borrow().is_empty());
    }

    /// A fade node whose reference is a form with the given base form type.
    fn fade_node(e: &mut Engine, scene: &Scene, base_type: u8) -> (u32, u32) {
        let node = object(e, scene, 1);
        e.mem.set_u32(node + 0x0c, 1);
        let form = e.mem.alloc(0x40);
        let base = e.mem.alloc(0x10);
        e.mem.set_u8(base + 4, base_type);
        e.mem.set_u32(form + 0x20, base);
        e.mem.set_u32(node + 0xcc, form);
        (node, form)
    }

    #[test]
    fn a_fade_node_makes_its_reference_the_current_user() {
        let (mut e, _) = engine_with_map();
        let scene = scene(&mut e);
        let map = texture_map(&mut e);
        e.set(map, BGSTextureUseMap::pDefaultUser, Ptr::new(0xd0d0));
        let (node, form) = fade_node(&mut e, &scene, 0x10);
        let child = geometry(&mut e, &scene, 0);
        scene.children.borrow_mut().insert(node, vec![child]);
        e.call(0x004a_a3d0, &args![map, node, 0u32]);
        assert_eq!(
            *scene.callbacks.borrow(),
            vec![(0x004a_a6f0, map.addr(), form)]
        );
        // The current user is cleared after the walk.
        assert_eq!(e.global::<u32>(CURRENT_USER_FORM), 0);
    }

    #[test]
    fn some_base_forms_are_only_followed_with_the_flag() {
        let (mut e, _) = engine_with_map();
        let scene = scene(&mut e);
        let map = texture_map(&mut e);
        let (node, form) = fade_node(&mut e, &scene, 0x2b);
        let child = geometry(&mut e, &scene, 0);
        scene.children.borrow_mut().insert(node, vec![child]);
        e.call(0x004a_a3d0, &args![map, node, 0u32]);
        assert!(scene.callbacks.borrow().is_empty());
        e.call(0x004a_a3d0, &args![map, node, 1u32]);
        assert_eq!(
            *scene.callbacks.borrow(),
            vec![(0x004a_a6f0, map.addr(), form)]
        );
        // The ends of the range 0x2a to 0x2d are in, 0x29 and 0x2e out.
        for (base_type, followed) in [(0x29u8, true), (0x2a, false), (0x2d, false), (0x2e, true)] {
            scene.callbacks.borrow_mut().clear();
            let (node, _) = fade_node(&mut e, &scene, base_type);
            let child = geometry(&mut e, &scene, 0);
            scene.children.borrow_mut().insert(node, vec![child]);
            e.call(0x004a_a3d0, &args![map, node, 0u32]);
            assert_eq!(
                !scene.callbacks.borrow().is_empty(),
                followed,
                "{base_type:#x}"
            );
        }
    }

    #[test]
    fn a_skipped_reference_stops_the_walk_without_a_user() {
        let (mut e, _) = engine_with_map();
        let scene = scene(&mut e);
        let map = texture_map(&mut e);
        let (node, form) = fade_node(&mut e, &scene, 0x10);
        e.mem.set_u32(form + 0x34, 1);
        let child = geometry(&mut e, &scene, 0);
        scene.children.borrow_mut().insert(node, vec![child]);
        e.call(0x004a_a3d0, &args![map, node, 0u32]);
        assert!(scene.callbacks.borrow().is_empty());
        assert_eq!(e.global::<u32>(CURRENT_USER_FORM), 0);
    }

    #[test]
    fn a_form_in_the_filter_table_is_not_walked() {
        let (mut e, _) = engine_with_map();
        let scene = scene(&mut e);
        let map = texture_map(&mut e);
        let (node, form) = fade_node(&mut e, &scene, 0x10);
        let child = geometry(&mut e, &scene, 0);
        scene.children.borrow_mut().insert(node, vec![child]);
        // The filter table (a list of forms) holds this reference.
        let table = e.mem.alloc(8);
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, form);
        e.mem.set_u32(table, item);
        e.set_global(FORM_FILTER_TABLE, table);
        e.call(0x004a_a3d0, &args![map, node, 0u32]);
        assert!(scene.callbacks.borrow().is_empty());
        assert_eq!(e.global::<u32>(CURRENT_USER_FORM), 0);
    }

    #[test]
    fn the_current_user_wins_over_the_default_user() {
        let (mut e, _) = engine_with_map();
        let scene = scene(&mut e);
        let map = texture_map(&mut e);
        e.set(map, BGSTextureUseMap::pDefaultUser, Ptr::new(0xd0d0));
        e.set_global(CURRENT_USER_FORM, 0xbeef_u32);
        let geometry = geometry(&mut e, &scene, 0);
        e.call(0x004a_a3d0, &args![map, geometry, 0u32]);
        assert_eq!(
            *scene.callbacks.borrow(),
            vec![(0x004a_a6f0, map.addr(), 0xbeef)]
        );
    }

    #[test]
    fn collecting_sets_the_default_user_and_walks() {
        let (mut e, _) = engine_with_map();
        let scene = scene(&mut e);
        let map = texture_map(&mut e);
        let geometry = geometry(&mut e, &scene, 0);
        e.call(0x004a_a3a0, &args![map, geometry, 0x4242u32, 1u32]);
        assert_eq!(e.get(map, BGSTextureUseMap::pDefaultUser).addr(), 0x4242);
        assert_eq!(
            *scene.callbacks.borrow(),
            vec![(0x004a_a6f0, map.addr(), 0x4242)]
        );
    }
}
