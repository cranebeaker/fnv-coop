//! The names of the game's data, from the plugin files that the running game loaded.

use std::path::Path;

use coop_engine::files::File;
use coop_engine::game::NameTable;
use esm::{FourCC, LoadOrder, Plugin};

/// The record types whose forms get a name: a quest and a global are shared state, and a
/// cell is a place.
const KINDS: [&[u8; 4]; 3] = [b"QUST", b"GLOB", b"CELL"];

/// The name table for these loaded files (the program's own list, in load order), read from
/// the plugin files of `data`. `Err` when a file cannot be read, or when the files give a
/// plugin another load index than the program does.
pub fn table(data: &Path, files: &[File]) -> Result<NameTable, String> {
    if files.is_empty() {
        return Err("the game loaded no plugin file".into());
    }
    let mut list = Vec::new();
    for file in files {
        let path = data.join(&file.name);
        let plugin = Plugin::open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        list.push((file.name.clone(), Some(path), plugin));
    }
    let order = LoadOrder::from_plugins(list).map_err(|e| format!("{e}"))?;
    let mut table = NameTable::default();
    for (plugin, file) in order.plugins().iter().zip(files) {
        if plugin.load_index != file.index {
            return Err(format!(
                "{} has the load index {:02X} in the game and {:02X} in the plugin files",
                file.name, file.index, plugin.load_index
            ));
        }
        table.add_plugin(&plugin.name, plugin.load_index);
    }
    for kind in KINDS {
        let name = std::str::from_utf8(kind).unwrap_or("????");
        for record in order.records_of_type(FourCC::new(kind)) {
            let editor_id = record.editor_id().ok().flatten().unwrap_or_default();
            table.add_form(name, record.form_id.0, &editor_id);
        }
    }
    Ok(table)
}

/// The hash of the ordered plugin list, as each program of the session contract makes it:
/// FNV-1a over each lower-case name and a zero byte.
pub fn mods_hash(files: &[File]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for file in files {
        for byte in file.name.to_ascii_lowercase().bytes().chain([0]) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(index: u8, name: &str) -> File {
        File {
            index,
            name: name.into(),
        }
    }

    #[test]
    fn no_file_and_a_missing_file_are_errors() {
        let dir = std::env::temp_dir();
        assert!(table(&dir, &[]).unwrap_err().contains("no plugin file"));
        let why = table(&dir, &[file(0, "NoSuchPlugin-coop-app.esm")]).unwrap_err();
        assert!(why.contains("NoSuchPlugin-coop-app.esm"), "{why}");
    }

    /// The hash follows the order and ignores the case, as a load order does.
    #[test]
    fn the_mods_hash_follows_the_order() {
        let a = [file(0, "FalloutNV.esm"), file(1, "DeadMoney.esm")];
        let b = [file(0, "falloutnv.ESM"), file(1, "deadmoney.esm")];
        let c = [file(0, "DeadMoney.esm"), file(1, "FalloutNV.esm")];
        assert_eq!(mods_hash(&a), mods_hash(&b));
        assert_ne!(mods_hash(&a), mods_hash(&c));
        assert_ne!(mods_hash(&a), mods_hash(&a[..1]));
    }
}
