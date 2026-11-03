//! Policy rows keyed to the quest ids of the actual load order.
//!
//! The policy file keys a row by `(plugin file name, local id)` and carries the editor id.
//! The id a quest has at run time depends on where its plugin sits in the load order, so a
//! session remaps the table once at start: a DLC loaded second, fifth, or not at all still
//! gets the right rows (or none), with a different order from the one the file was
//! imported with (mods, another store's order).

use coop_policy::{PolicyTable, QuestRow, RemapReport};
use esm::{FormId, LoadOrder};

/// The runtime id of `row`'s quest in `order`, or `None` when it is not loaded.
/// First by (plugin, local id); when that plugin is absent or has no such record, by
/// editor id (a quest a mod moved or merged).
pub fn resolve_row(order: &LoadOrder, row: &QuestRow) -> Option<u32> {
    let own = order
        .plugins()
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(&row.plugin))
        .map(|p| (u32::from(p.load_index) << 24) | (row.local & 0x00FF_FFFF));
    if let Some(id) = own.filter(|id| order.get(FormId(*id)).is_some()) {
        return Some(id);
    }
    // a quest a mod moved or merged; failing that, the row's own slot in its plugin (a trimmed
    // data set may not carry the record, the row still names the quest)
    order.form_by_editor_id(&row.edid).map(|f| f.0).or(own)
}

/// `table` with every row keyed by its id in `order`; unmatched rows are dropped and listed.
pub fn resolve_policy(
    table: &PolicyTable,
    order: &LoadOrder,
) -> Result<(PolicyTable, RemapReport), String> {
    table
        .remap(|r| resolve_row(order, r))
        .map_err(|e| format!("{e}"))
}

/// The bundled table resolved against `order`.
pub fn bundled_for(order: &LoadOrder) -> Result<(PolicyTable, RemapReport), String> {
    let t = PolicyTable::bundled().map_err(|e| format!("{e}"))?;
    resolve_policy(&t, order)
}
