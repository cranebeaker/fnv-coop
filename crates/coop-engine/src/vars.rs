//! A quest script's variables as the host sends them and a guest applies them (docs/coop/SPECS.md
//! S5). They are story state: dialogue conditions and other scripts read them (`GetQuestVariable`,
//! `Quest.Variable`), and on a guest the quest script that writes them does not run. Each quest's
//! `ScriptLocals` (+0x5c) is made when the quest is initialised at load, on every machine, so the
//! guest's quest has the same variables; only their values travel.

use engine_core::prelude::*;

use crate::{addr, forms, quest};

/// The variable list of quest `id`'s script: (variable object, its id), in list order. Empty when
/// the quest has no script.
fn locals(e: &mut Engine, id: u32) -> Option<Vec<(u32, u32)>> {
    let q = forms::quest(e, id)?;
    let locals = e.mem.u32(q.wrapping_add(addr::QUEST_SCRIPT_LOCALS_OFFSET));
    if locals == 0 {
        return Some(Vec::new());
    }
    let first = e.mem.u32(locals.wrapping_add(addr::LOCALS_LIST_OFFSET));
    Some(
        quest::list_items(e, first)
            .into_iter()
            .map(|v| (v, e.mem.u32(v.wrapping_add(addr::LOCAL_ID_OFFSET))))
            .collect(),
    )
}

/// Quest `id`'s script variables: (variable id, the value's bits), in list order. `None` when `id`
/// is not a quest.
pub fn quest_vars(e: &mut Engine, id: u32) -> Option<Vec<(u32, u64)>> {
    Some(
        locals(e, id)?
            .into_iter()
            .map(|(v, n)| (n, e.mem.u64(v.wrapping_add(addr::LOCAL_VALUE_OFFSET))))
            .collect(),
    )
}

/// Writes quest `id`'s variable `var` as `ScriptLocals::SetVariable` stores it (the value in place,
/// so the getter's cached variable address stays right). False when there is no such quest or
/// variable.
pub fn set_quest_var(e: &mut Engine, id: u32, var: u32, bits: u64) -> bool {
    let Some(list) = locals(e, id) else {
        return false;
    };
    match list.into_iter().find(|(_, n)| *n == var) {
        Some((v, _)) => {
            e.mem
                .set_u64(v.wrapping_add(addr::LOCAL_VALUE_OFFSET), bits);
            true
        }
        None => false,
    }
}
