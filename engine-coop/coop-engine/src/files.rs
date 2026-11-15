//! The plugin files that the running program loaded, in its own load order.
//!
//! The load order gives each form id its top byte. The adapter takes it from the program,
//! not from a file of the player's folder: the program is the one source that cannot differ
//! from what the game uses.

use crate::addr;
use crate::program::Program;

/// One loaded plugin file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    /// The load index: the top byte of the ids of the file's own forms.
    pub index: u8,
    /// The file's name, as the program keeps it (`FalloutNV.esm`).
    pub name: String,
}

/// The most files that a load order has: one for each value of a form id's top byte but the
/// last, which the program keeps for the forms that it makes at run time.
pub const MOST: u32 = 255;

/// The program's loaded files, in load order. Empty before the program made its data
/// handler. `Err` when the list does not have the layout that the adapter knows.
pub fn loaded(p: &dyn Program) -> Result<Vec<File>, String> {
    let handler = p.u32(addr::DATA_HANDLER);
    if handler == 0 {
        return Ok(Vec::new());
    }
    let count = p.u32(handler + addr::FILES_COUNT);
    if count > MOST {
        return Err(format!("the program's file count is {count}"));
    }
    let mut files = Vec::new();
    for slot in 0..count {
        let file = p.u32(handler + addr::FILES + 4 * slot);
        if file == 0 {
            return Err(format!("the program's file {slot} of {count} is not there"));
        }
        let mut bytes = [0u8; addr::FILE_NAME_BYTES as usize];
        p.read(file + addr::FILE_NAME, &mut bytes);
        let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
        let name = String::from_utf8_lossy(&bytes[..end]).into_owned();
        let index = p.u32(file + addr::FILE_INDEX);
        // The list is in load order: a file's index is its place.
        if index != slot || name.is_empty() {
            return Err(format!(
                "the program's file {slot} has the index {index} and the name {name:?}"
            ));
        }
        files.push(File {
            index: slot as u8,
            name,
        });
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    #[test]
    fn the_files_come_in_load_order_with_their_names() {
        let mut p = FakeProgram::new();
        assert_eq!(loaded(&p), Ok(Vec::new()), "no data handler yet");
        p.add_file("FalloutNV.esm");
        p.add_file("DeadMoney.esm");
        let files = loaded(&p).unwrap();
        assert_eq!(
            files,
            [
                File {
                    index: 0,
                    name: "FalloutNV.esm".into()
                },
                File {
                    index: 1,
                    name: "DeadMoney.esm".into()
                }
            ]
        );
    }

    /// A list that is not in load order is not the layout that the adapter knows: the
    /// adapter must not give form ids from it.
    #[test]
    fn a_list_with_another_layout_is_refused() {
        let mut p = FakeProgram::new();
        p.add_file("FalloutNV.esm");
        let second = p.add_file("DeadMoney.esm");
        p.set_u32(second + addr::FILE_INDEX, 7);
        let why = loaded(&p).unwrap_err();
        assert!(why.contains("index 7"), "{why}");

        let mut p = FakeProgram::new();
        p.add_file("FalloutNV.esm");
        let handler = p.u32(addr::DATA_HANDLER);
        p.set_u32(handler + addr::FILES_COUNT, 300);
        assert!(loaded(&p).unwrap_err().contains("300"));
    }
}
