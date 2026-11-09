//! xNVSE's inventory reference commands (`Commands_InventoryRef.cpp` (xNVSE)), with ADR-0016's
//! step I-12 (`crate::invref`): a container's stacks as temporary references, and the commands
//! that act on one.

use crate::args::extract_args_ex;
use crate::invref::{self, Data};
use crate::table::{Command, Param, Returns};
use crate::vars::{self, Key, Value};
use crate::{handled, CommandArgs};
use engine_core::prelude::*;
use std::collections::HashMap;

/// `kParams_OneOptionalContainerRef`, `kParams_OneObjectID` (xNVSE). `kParams_OneContainerRef`
/// (`CopyIR`, `CopyIRAlt`) is the same: its one parameter is optional too.
const ONE_OPTIONAL_CONTAINER: &[Param] = &[Param {
    name: "containerRef",
    kind: 0x1a,
    optional: true,
}];
const ONE_OBJECT_ID: &[Param] = &[Param {
    name: "objectID",
    kind: 0x03,
    optional: false,
}];

/// `TESForm::refID`.
const FORM_ID: u32 = 0xc;
/// Where a base form keeps its `TESContainer`: an actor base at `+0x64`, a container
/// (`kFormType_TESObjectCONT`, 0x1B) at `+0x30`; `TESForm::IsActor` (virtual `0x100`).
const ACTOR_BASE_CONTAINER: u32 = 0x64;
const CONT_CONTAINER: u32 = 0x30;
const FORM_TYPE_CONT: u8 = 0x1b;
const IS_ACTOR: u32 = 0x100;
/// `TESContainer::formCountList` (`+4`: a `tList<FormCount>`; `FormCount`: count (`+0`), form
/// (`+4`)).
const CONTAINER_LIST: u32 = 4;

pub static COMMANDS: &[Command] = &[
    Command {
        opcode: 0x14ff,
        name: "RemoveMeIR",
        short: "",
        help: "removes an inventory reference from its container, optionally transferring it to a different container, in much the same way as the vanilla RemoveMe command. The inventory reference becomes invalid once this command is called and should no longer be used.",
        needs_ref: true,
        params: ONE_OPTIONAL_CONTAINER,
        execute: remove_me_ir,
        eval: None,
        returns: Returns::Default,
    },
    Command {
        opcode: 0x1500,
        name: "CopyIR",
        short: "",
        help: "copies an inventory reference to the specified container. The calling object needn't be in a container and remains valid after the command is called. If the calling object is equipped, the copy will not be equipped.",
        needs_ref: true,
        params: ONE_OPTIONAL_CONTAINER,
        execute: copy_ir,
        eval: None,
        returns: Returns::Default,
    },
    Command {
        opcode: 0x1667,
        name: "CopyIRAlt",
        short: "",
        help: "copies an inventory reference to the specified container. The calling object needn't be in a container and remains valid after the command is called. If the calling object is equipped, the copy will not be equipped.",
        needs_ref: true,
        params: ONE_OPTIONAL_CONTAINER,
        execute: copy_ir_alt,
        eval: None,
        returns: Returns::Form,
    },
    Command {
        opcode: 0x1501,
        name: "CreateTempRef",
        short: "",
        help: "creates a temporary reference to the specified form. This reference does not exist in the gameworld or in a container, and remains valid for only one frame. It is mostly useful for creating a stack of one or more items to be added to a container with CopyIR",
        needs_ref: false,
        params: ONE_OBJECT_ID,
        execute: create_temp_ref,
        eval: None,
        returns: Returns::Form,
    },
    Command {
        opcode: 0x1502,
        name: "GetFirstRefForItem",
        short: "",
        help: "returns the first entry in an array of temp refs to objects of the specified type in the calling container",
        needs_ref: true,
        params: ONE_OBJECT_ID,
        execute: get_first_ref_for_item,
        eval: None,
        returns: Returns::Form,
    },
    Command {
        opcode: 0x1503,
        name: "GetNextRefForItem",
        short: "",
        help: "returns the next entry in the array of temp refs to objects of the specified type in the calling container",
        needs_ref: true,
        params: ONE_OBJECT_ID,
        execute: get_next_ref_for_item,
        eval: None,
        returns: Returns::Form,
    },
    Command {
        opcode: 0x15ba,
        name: "GetInvRefsForItem",
        short: "",
        help: "returns an array of temp refs to objects of the specified type in the calling container",
        needs_ref: true,
        params: ONE_OBJECT_ID,
        execute: get_inv_refs_for_item,
        eval: None,
        returns: Returns::Array,
    },
    Command {
        opcode: 0x164f,
        name: "GetSoldItemInvRef",
        short: "",
        help: "Returns the invRef for the currently sold item(in OnSell event)",
        needs_ref: false,
        params: &[],
        execute: get_sold_item_inv_ref,
        eval: None,
        returns: Returns::Form,
    },
    Command {
        opcode: 0x165b,
        name: "IsInventoryRef",
        short: "",
        help: "",
        needs_ref: true,
        params: &[],
        execute: is_inventory_ref,
        eval: None,
        returns: Returns::Default,
    },
];

/// `InvRefArrayItem`: the stacks `GetFirstRefForItem` listed for a script.
#[derive(Default)]
pub struct State {
    /// By the calling script's id (`g_invRefArray`): the item, the next index, the references.
    lists: HashMap<u32, (u32, usize, Vec<u32>)>,
}

fn reference_arg(e: &mut Engine, a: &CommandArgs) -> Option<u32> {
    let read = extract_args_ex(e, a);
    read.ok
        .then(|| read.args.first().and_then(|x| x.form()).unwrap_or(0))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_InventoryRef.cpp:
// Cmd_RemoveMeIR_Execute.
fn remove_me_ir(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if a.this_obj != 0 {
        let id = e.mem.u32(a.this_obj + FORM_ID);
        if invref::for_ref_id(e, id) {
            if let Some(dest) = reference_arg(e, &a) {
                let done = if dest != 0 {
                    invref::move_to_container(e, id, dest)
                } else {
                    invref::remove_from_container(e, id)
                };
                a.set_result(e, if done { 1.0 } else { 0.0 });
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_InventoryRef.cpp:
// Cmd_CopyIR_Execute.
fn copy_ir(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if a.this_obj != 0 {
        let id = e.mem.u32(a.this_obj + FORM_ID);
        if invref::for_ref_id(e, id) {
            if let Some(dest) = reference_arg(e, &a).filter(|&d| d != 0) {
                let (done, _) = invref::copy_to_container(e, id, dest, false);
                a.set_result(e, if done { 1.0 } else { 0.0 });
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_InventoryRef.cpp:
// Cmd_CopyIRAlt_Execute.
/// The copy's inventory reference. When the copy fails xNVSE reads an uninitialised pointer;
/// here the result stays 0.
fn copy_ir_alt(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    e.mem.set_u32(a.result, 0);
    if a.this_obj != 0 {
        let id = e.mem.u32(a.this_obj + FORM_ID);
        if invref::for_ref_id(e, id) {
            if let Some(dest) = reference_arg(e, &a).filter(|&d| d != 0) {
                let (_, copy) = invref::copy_to_container(e, id, dest, true);
                if copy != 0 {
                    let copy_id = e.mem.u32(copy + FORM_ID);
                    let refr = invref::get_ref(e, copy_id);
                    if refr != 0 {
                        a.set_form_result(e, e.mem.u32(refr + FORM_ID));
                    }
                }
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_InventoryRef.cpp:
// Cmd_CreateTempRef_Execute.
fn create_temp_ref(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    e.mem.set_u32(a.result, 0);
    if let Some(form) = reference_arg(e, &a) {
        if form != 0 {
            let refr = invref::create(
                e,
                0,
                Data {
                    item: form,
                    entry: 0,
                    xdata: 0,
                },
                false,
            );
            let id = e.mem.u32(refr + FORM_ID);
            let refr = invref::get_ref(e, id);
            if refr != 0 {
                a.set_form_result(e, e.mem.u32(refr + FORM_ID));
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), TESContainer::GetCountForForm.
/// The base container's count of `item` (every entry of it).
pub fn base_count(e: &Engine, container: u32, item: u32) -> i32 {
    invref::list_items(e, container + CONTAINER_LIST)
        .into_iter()
        .filter(|&fc| e.mem.u32(fc + 4) == item)
        .map(|fc| e.mem.u32(fc) as i32)
        .sum()
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameObjects.cpp:
// TESObjectREFR::GetContainer.
/// A reference's base container: an actor's or a container's, else 0.
pub fn container_of(e: &mut Engine, reference: u32) -> u32 {
    let base = super::base_form(e, reference);
    if e.vcall(reference, IS_ACTOR, &[]).bool() {
        return base + ACTOR_BASE_CONTAINER;
    }
    if base != 0 && e.mem.u8(base + 4) == FORM_TYPE_CONT {
        return base + CONT_CONTAINER;
    }
    0
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_InventoryRef.cpp: the stack walk
// of Cmd_GetFirstRefForItem_Execute and Cmd_GetInvRefsForItem_Execute.
/// An inventory reference (with an entry of its own) for each stack of `item` in `reference`:
/// the stacks with their own extra data first (their counts, at most what is left), then the
/// rest. None without a container or changes.
pub(crate) fn stacks_for_item(e: &mut Engine, reference: u32, item: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let container = container_of(e, reference);
    if container == 0 {
        return out;
    }
    let list = invref::entry_list(e, reference);
    if list == 0 {
        return out;
    }
    let mut count = base_count(e, container, item);
    let entry = invref::find_for_item(e, list, item);
    if entry != 0 {
        let delta = e.mem.u32(entry + invref::ENTRY_COUNT_DELTA) as i32;
        if count != 0 {
            if invref::has_extra_leveled_item(e, entry) {
                count = delta;
            } else {
                count += delta;
            }
        } else {
            count = delta;
        }
        let extend = e.mem.u32(entry + invref::ENTRY_EXTEND_DATA);
        if count > 0 && extend != 0 {
            for xdata in invref::list_items(e, extend) {
                let mut n = invref::count_for_extra_list(e, xdata);
                if n < 1 {
                    continue;
                }
                if n > count {
                    n = count;
                }
                count -= n;
                let refr = invref::create_entry(e, reference, item, n, xdata);
                out.push(e.mem.u32(refr + FORM_ID));
                if count == 0 {
                    break;
                }
            }
        }
    }
    if count > 0 {
        let refr = invref::create_entry(e, reference, item, count, 0);
        out.push(e.mem.u32(refr + FORM_ID));
    }
    out
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_InventoryRef.cpp:
// Cmd_GetFirstRefForItem_Execute.
fn get_first_ref_for_item(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    e.mem.set_u32(a.result, 0);
    let script_id = e.mem.u32(a.script + FORM_ID);
    // The script's list starts again (made empty when it was there).
    e.ext::<crate::Nvse>()
        .inv_ref_arrays
        .lists
        .insert(script_id, (0, 0, Vec::new()));
    let item = if a.this_obj != 0 {
        reference_arg(e, &a).unwrap_or(0)
    } else {
        0
    };
    let refs = if item != 0 {
        stacks_for_item(e, a.this_obj, item)
    } else {
        Vec::new()
    };
    let first = refs.first().copied();
    let list = (item, usize::from(first.is_some()), refs);
    e.ext::<crate::Nvse>()
        .inv_ref_arrays
        .lists
        .insert(script_id, list);
    if let Some(id) = first {
        a.set_form_result(e, id);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_InventoryRef.cpp:
// Cmd_GetNextRefForItem_Execute.
fn get_next_ref_for_item(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    e.mem.set_u32(a.result, 0);
    if a.this_obj == 0 {
        return handled(true);
    }
    let Some(item) = reference_arg(e, &a) else {
        return handled(true);
    };
    if item == 0 {
        return handled(true);
    }
    let script_id = e.mem.u32(a.script + FORM_ID);
    let lists = &mut e.ext::<crate::Nvse>().inv_ref_arrays.lists;
    let next = match lists.get_mut(&script_id) {
        Some((listed, next, refs)) if *listed == item => {
            if *next < refs.len() {
                *next += 1;
                Some(refs[*next - 1])
            } else {
                None
            }
        }
        _ => return get_first_ref_for_item(e, w),
    };
    if let Some(id) = next {
        a.set_form_result(e, id);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_InventoryRef.cpp:
// Cmd_GetInvRefsForItem_Execute.
fn get_inv_refs_for_item(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let modi = vars::script_mod_index(e, a.script);
    let array = vars::create_array(e, vars::KEY_NUMBER, true, modi);
    a.set_result(e, f64::from(array));
    if a.this_obj == 0 {
        return handled(true);
    }
    let Some(item) = reference_arg(e, &a) else {
        return handled(true);
    };
    if item == 0 {
        return handled(true);
    }
    for (i, id) in stacks_for_item(e, a.this_obj, item).into_iter().enumerate() {
        vars::set_element(e, array, &Key::Number(i as f64), Value::Form(id));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Script.cpp:
// Cmd_GetSoldItemInvRef_Execute.
fn get_sold_item_inv_ref(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let refr = invref::sold_item_ref(e);
    if refr != 0 {
        a.set_form_result(e, e.mem.u32(refr + FORM_ID));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_InventoryRef.cpp:
// Cmd_IsInventoryRef_Execute.
fn is_inventory_ref(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if a.this_obj != 0 {
        let id = e.mem.u32(a.this_obj + FORM_ID);
        let is = invref::for_ref_id(e, id);
        a.set_result(e, if is { 1.0 } else { 0.0 });
    }
    handled(true)
}
