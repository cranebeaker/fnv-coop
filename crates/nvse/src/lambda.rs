//! Lambdas (ADR-0016 §7; build step I-7), translated from xNVSE's `LambdaManager.cpp` (xNVSE).
//!
//! A lambda is an `F` token of the expression bytecode: the compiled data of a whole function
//! script (`scn`, `Begin Function`, ..., `End`) inside its parent's expression. Evaluated, it
//! becomes a game `Script` form of its own (made by the game's constructor `005aa0f0`), sharing
//! the parent's variable and reference lists (copied by the game's `005ab930`/`005ab7f0`) and its
//! files, with a form id the next free one after the parent's (or the game's next free id for a
//! parent made at run time). One script per expression position and owner (the quest, the
//! function itself, the calling reference or the parent script), whose parent event list is the
//! evaluating one: calling the lambda runs on the parent's locals.
//!
//! Event handlers and delayed calls keep a lambda's locals alive past its parent's list
//! ([`save_variables`], [`unsave_variables`]): when the game destroys a parent list that a saved
//! lambda needs, a copy of it is kept for the lambda ([`mark_parent_deleted`]), and copies no
//! lambda needs any more are destroyed at the tick ([`erase_unused_saved_lists`]).
//!
//! Not here: `MarkScriptAsDeleted` (xNVSE hooks the script destructor's call at `0x005AA1A0`
//! for it; no seam yet, so a deleted lambda's parent script copy is not made) and
//! `DeleteAllForParentScript` (hot reload, through the plugin API).

use crate::Nvse;
use engine_core::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};

/// `FormHeap_Allocate`, `Script`'s constructor and size, `CopyVarList`, `CopyRefList`,
/// `DataHandler::DisableAssignFormIDs`, `GetNextFreeFormID`, `LookupFormByID`, and
/// `ScriptEventList`'s constructor and size.
const FORM_HEAP_ALLOCATE: u32 = 0x0040_1000;
const SCRIPT_CONSTRUCTOR: u32 = 0x005a_a0f0;
const SCRIPT_SIZE: u32 = 0x54;
const COPY_VAR_LIST: u32 = 0x005a_b930;
const COPY_REF_LIST: u32 = 0x005a_b7f0;
const DISABLE_ASSIGN_FORM_IDS: u32 = 0x0046_4d30;
const NEXT_FREE_FORM_ID: u32 = 0x0046_9800;
const LOOKUP_FORM: u32 = 0x0048_39c0;
const EVENT_LIST_CONSTRUCTOR: u32 = 0x005a_8b80;
const EVENT_LIST_SIZE: u32 = 0x14;
/// `DataHandler`'s singleton.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `TESForm::SetRefID` (`SetFormID`, `00485c10`): the `+0x128` virtual function (Ghidra: the
/// 75th entry of `Script`'s vtable `01037094`).
const SET_FORM_ID: u32 = 0x128;
/// `TESForm`'s deleting destructor (`Destroy(bool)`), `+0x10`.
const DESTROY: u32 = 0x10;

/// `LambdaContext`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Context {
    pub parent_script: u32,
    pub parent_list: u32,
    /// Lambdas held in reference variables of this one (`capturedLambdaVariableScripts`).
    pub captured: Option<Vec<u32>>,
    pub parents: HashSet<u32>,
}

/// `VariableListContext`: the lambdas that keep a list, and whether it is the copy of a
/// destroyed one.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SavedList {
    pub lambdas: HashMap<u32, usize>,
    pub is_copy: bool,
}

/// The lambdas' state (`g_lambdaScriptPosMap`, `g_lambdas`, `g_savedVarLists`), and the copies
/// of the `F` tokens' data (xNVSE copies them once, when the expression's tokens are cached).
#[derive(Default)]
pub struct State {
    by_position: BTreeMap<(u32, u32), u32>,
    lambdas: HashMap<u32, Context>,
    saved: HashMap<u32, SavedList>,
    data: HashMap<u32, (u32, u32)>,
}

fn state(e: &mut Engine) -> &mut State {
    &mut e.ext::<Nvse>().lambdas
}

/// `LambdaManager::IsScriptLambda`.
pub fn is_lambda(e: &Engine, script: u32) -> bool {
    e.ext_get::<Nvse>()
        .is_some_and(|n| n.lambdas.lambdas.contains_key(&script))
}

/// The lambda's context, if it is one.
pub fn context(e: &Engine, script: u32) -> Option<Context> {
    e.ext_get::<Nvse>()?.lambdas.lambdas.get(&script).cloned()
}

/// The script data copy of the `F` token at `token` (its length word after it), made once.
pub fn token_data(e: &mut Engine, token: u32, len: u32) -> u32 {
    if let Some(&(data, _)) = state(e).data.get(&token) {
        return data;
    }
    let data = e.call(FORM_HEAP_ALLOCATE, &args![len]).u32();
    let bytes = e.mem.bytes(token, len);
    e.mem.write(data, &bytes);
    state(e).data.insert(token, (data, len));
    data
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), LambdaManager.cpp:
// LambdaManager::CreateLambdaScript (both).
/// The lambda of the expression at `position` (the command's argument data) for the evaluating
/// script and list; 0 when no form id is left in the parent's file.
pub fn create(
    e: &mut Engine,
    position: u32,
    data: u32,
    len: u32,
    parent_script: u32,
    parent_list: u32,
) -> u32 {
    let quest = crate::functions::quest_of(e, parent_script);
    let owner = if quest != 0 {
        e.mem.u32(quest + 0xc)
    } else if crate::functions::is_user_function(e, parent_script) {
        e.mem.u32(parent_script + 0xc)
    } else {
        match crate::context::current(e).map(|c| c.owner_ref) {
            Some(r) if r != 0 => e.mem.u32(r + 0xc),
            _ => e.mem.u32(parent_script + 0xc),
        }
    };
    let key = (position, owner);
    if let Some(&lambda) = state(e).by_position.get(&key) {
        set_parent(e, lambda, parent_list);
        return lambda;
    }
    let lambda = new_script(e, data, len, parent_script);
    let parent_id = e.mem.u32(parent_script + 0xc);
    if parent_id >> 24 == 0xff {
        let handler = e.mem.u32(DATA_HANDLER);
        let id = e.call(NEXT_FREE_FORM_ID, &args![handler]).u32();
        e.vcall(lambda, SET_FORM_ID, &args![id, 1u32]);
    } else {
        // `GetNextFreeFormID(formId)`: the first id after the parent's that no form has.
        let mut id = parent_id;
        loop {
            id = id.wrapping_add(1);
            if e.call(LOOKUP_FORM, &args![id]).u32() == 0 {
                break;
            }
        }
        if id >> 24 != parent_id >> 24 {
            e.vcall(lambda, DESTROY, &args![1u32]);
            return 0;
        }
        e.vcall(lambda, SET_FORM_ID, &args![id, 1u32]);
    }
    let st = state(e);
    st.by_position.insert(key, lambda);
    st.lambdas.insert(
        lambda,
        Context {
            parent_script,
            parent_list,
            ..Context::default()
        },
    );
    lambda
}

/// A `Script` made at run time (no form id assigned while it is built), on `data`, with the
/// parent's variables, references, counts and files.
fn new_script(e: &mut Engine, data: u32, len: u32, parent: u32) -> u32 {
    let handler = e.mem.u32(DATA_HANDLER);
    e.call(DISABLE_ASSIGN_FORM_IDS, &args![handler, 1u32]);
    let memory = e.call(FORM_HEAP_ALLOCATE, &args![SCRIPT_SIZE]).u32();
    let lambda = e.call(SCRIPT_CONSTRUCTOR, &args![memory]).u32();
    e.call(DISABLE_ASSIGN_FORM_IDS, &args![handler, 0u32]);
    e.mem.set_u32(lambda + 0x30, data);
    e.mem.set_u32(lambda + 0x20, len);
    e.call(COPY_VAR_LIST, &args![parent + 0x4c, lambda + 0x4c]);
    e.call(COPY_REF_LIST, &args![parent + 0x44, lambda + 0x44]);
    for at in [0x1c, 0x24, 0x18] {
        let v = e.mem.u32(parent + at);
        e.mem.set_u32(lambda + at, v);
    }
    // `parentScript->mods.ForEach(... Append ...)`.
    let mut node = parent + 0x10;
    while node != 0 {
        let file = e.mem.u32(node);
        if file != 0 {
            list_append(e, lambda + 0x10, file);
        }
        node = e.mem.u32(node + 4);
    }
    lambda
}

/// `tList::Append`: into the first node when it is empty, else a new node after the last.
fn list_append(e: &mut Engine, head: u32, item: u32) {
    let mut last = head;
    while e.mem.u32(last + 4) != 0 {
        last = e.mem.u32(last + 4);
    }
    if e.mem.u32(last) == 0 {
        e.mem.set_u32(last, item);
        return;
    }
    let node = e.call(FORM_HEAP_ALLOCATE, &args![8u32]).u32();
    e.mem.set_u32(node, item);
    e.mem.set_u32(node + 4, 0);
    e.mem.set_u32(last + 4, node);
}

fn set_parent(e: &mut Engine, lambda: u32, list: u32) {
    state(e).lambdas.entry(lambda).or_default().parent_list = list;
}

/// `LambdaManager::GetParentEventList`: the parent list, or the kept list the lambda is saved
/// with.
pub fn parent_event_list(e: &Engine, lambda: u32) -> u32 {
    let Some(st) = e.ext_get::<Nvse>().map(|n| &n.lambdas) else {
        return 0;
    };
    let Some(ctx) = st.lambdas.get(&lambda) else {
        return 0;
    };
    if ctx.parent_list != 0 {
        return ctx.parent_list;
    }
    saved_list_of(st, lambda).unwrap_or(0)
}

/// `GetVarListContextIter`: the saved list holding the lambda.
fn saved_list_of(st: &State, lambda: u32) -> Option<u32> {
    st.saved
        .iter()
        .find(|(_, v)| v.lambdas.contains_key(&lambda))
        .map(|(k, _)| *k)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), LambdaManager.cpp:
// LambdaManager::MarkParentAsDeleted.
/// A list is going away: no lambda has it as parent any more, and when saved lambdas need it, a
/// copy takes its place.
pub fn mark_parent_deleted(e: &mut Engine, list: u32) {
    let st = state(e);
    for ctx in st.lambdas.values_mut() {
        if ctx.parent_list == list {
            ctx.parent_list = 0;
        }
    }
    let Some(saved) = st.saved.get(&list).cloned() else {
        return;
    };
    if saved.is_copy {
        return;
    }
    let copy = copy_event_list(e, list);
    let st = state(e);
    st.saved.remove(&list);
    st.saved.insert(
        copy,
        SavedList {
            is_copy: true,
            ..saved
        },
    );
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameAPI.cpp: ScriptEventList::Copy.
/// A new list of the same script with a local per local: arrays referred again, strings copied
/// (as permanent strings), other values as they are.
pub fn copy_event_list(e: &mut Engine, list: u32) -> u32 {
    let memory = e.call(FORM_HEAP_ALLOCATE, &args![EVENT_LIST_SIZE]).u32();
    let copy = e.call(EVENT_LIST_CONSTRUCTOR, &args![memory]).u32();
    let script = e.mem.u32(list);
    e.mem.set_u32(copy, script);
    let vars_list = e.call(FORM_HEAP_ALLOCATE, &args![8u32]).u32();
    e.mem.set_u32(vars_list, 0);
    e.mem.set_u32(vars_list + 4, 0);
    e.mem.set_u32(copy + 0xc, vars_list);
    let modi = crate::vars::script_mod_index(e, script);
    let kinds = crate::vars::collected_kinds(e, list);
    let mut node = e.mem.u32(list + 0xc);
    while node != 0 {
        let var = e.mem.u32(node);
        node = e.mem.u32(node + 4);
        if var == 0 {
            continue;
        }
        let new_var = e.call(FORM_HEAP_ALLOCATE, &args![0x10u32]).u32();
        e.mem.set_u32(new_var + 4, 0);
        list_append(e, vars_list, new_var);
        e.mem.set_u32(new_var, e.mem.u32(var));
        let data = e.mem.f64(var + 8);
        match kinds.get(&var) {
            Some(crate::vars::VarType::Array) => {
                if data != 0.0 {
                    e.mem.set_f64(new_var + 8, 0.0);
                    let id = crate::args::f64_to_i32(data) as u32;
                    crate::vars::add_reference_double(e, new_var + 8, id, modi);
                    crate::vars::collect(e, copy, new_var, crate::vars::VarType::Array);
                } else {
                    e.mem.set_f64(new_var + 8, 0.0);
                }
            }
            Some(crate::vars::VarType::String) => {
                let id = crate::args::f64_to_i32(data) as u32;
                match crate::vars::string(e, id) {
                    Some(text) => {
                        let new_id = crate::vars::add_string(e, modi, &text, false);
                        e.mem.set_f64(new_var + 8, f64::from(new_id));
                        crate::vars::collect(e, copy, new_var, crate::vars::VarType::String);
                    }
                    None => e.mem.set_f64(new_var + 8, 0.0),
                }
            }
            _ => e.mem.set_f64(new_var + 8, data),
        }
    }
    copy
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), LambdaManager.cpp: CaptureChildLambdas.
/// The lambdas held in the lambda's reference variables (resolved on its parent list), found
/// once.
fn capture_children(e: &mut Engine, lambda: u32) {
    let Some(ctx) = context(e, lambda) else {
        return;
    };
    if ctx.captured.is_some() {
        return;
    }
    let mut found = Vec::new();
    let mut node = lambda + 0x44;
    while node != 0 {
        let ref_var = e.mem.u32(node);
        node = e.mem.u32(node + 4);
        if ref_var == 0 || e.mem.u32(ref_var + 0xc) == 0 {
            continue;
        }
        crate::args::resolve_ref_variable(e, ref_var, ctx.parent_list);
        let form = e.mem.u32(ref_var + 8);
        if form != 0
            && e.mem.u8(form + 4) == 0x11
            && form != lambda
            && is_lambda(e, form)
            && !ctx.parents.contains(&form)
        {
            found.push(form);
        }
    }
    if let Some(c) = state(e).lambdas.get_mut(&lambda) {
        c.captured = Some(found);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), LambdaManager.cpp: SaveLambdaVariables.
/// Keeps the lambda's list (and its child lambdas' lists) for a handler or delayed call.
pub fn save_variables(e: &mut Engine, lambda: u32) {
    save_with_parent(e, lambda, None);
}

fn save_with_parent(e: &mut Engine, lambda: u32, parent: Option<u32>) {
    let Some(ctx) = context(e, lambda) else {
        return;
    };
    let list = if ctx.parent_list != 0 {
        ctx.parent_list
    } else {
        match saved_list_of(state(e), lambda) {
            Some(l) => l,
            None => return,
        }
    };
    *state(e)
        .saved
        .entry(list)
        .or_default()
        .lambdas
        .entry(lambda)
        .or_insert(0) += 1;
    if let Some(p) = parent {
        let mut parents = context(e, p).map(|c| c.parents).unwrap_or_default();
        parents.insert(p);
        if let Some(c) = state(e).lambdas.get_mut(&lambda) {
            c.parents = parents;
        }
    }
    capture_children(e, lambda);
    let children = context(e, lambda)
        .and_then(|c| c.captured)
        .unwrap_or_default();
    for child in children {
        save_with_parent(e, child, Some(lambda));
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), LambdaManager.cpp: UnsaveLambdaVariables.
/// Gives up what [`save_variables`] kept.
pub fn unsave_variables(e: &mut Engine, lambda: u32) {
    unsave_with_parent(e, lambda, 0);
}

fn unsave_with_parent(e: &mut Engine, lambda: u32, parent: u32) {
    if context(e, lambda).is_none() {
        return;
    }
    let Some(list) = saved_list_of(state(e), lambda) else {
        return;
    };
    if let Some(saved) = state(e).saved.get_mut(&list) {
        if let Some(count) = saved.lambdas.get_mut(&lambda) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                saved.lambdas.remove(&lambda);
            }
        }
    }
    let children = context(e, lambda)
        .and_then(|c| c.captured)
        .unwrap_or_default();
    for child in children {
        if child != parent {
            unsave_with_parent(e, child, lambda);
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), LambdaManager.cpp:
// LambdaManager::EraseUnusedSavedVariableLists.
/// At the tick: kept lists no lambda needs are dropped, copies destroyed.
pub fn erase_unused_saved_lists(e: &mut Engine) {
    let unused: Vec<(u32, bool)> = state(e)
        .saved
        .iter()
        .filter(|(_, v)| v.lambdas.is_empty())
        .map(|(k, v)| (*k, v.is_copy))
        .collect();
    for (list, is_copy) in unused {
        state(e).saved.remove(&list);
        if is_copy {
            crate::functions::delete_event_list(e, list);
        }
    }
}

/// Whether a list is kept for lambdas (tests and the co-save).
pub fn is_saved(e: &Engine, list: u32) -> bool {
    e.ext_get::<Nvse>()
        .is_some_and(|n| n.lambdas.saved.contains_key(&list))
}
