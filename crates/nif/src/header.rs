//! The NIF file header: version, block type table, block sizes and the
//! shared string table.

use crate::error::{Error, Result};
use crate::reader::{latin1, Reader};

pub const V20_2_0_5: u32 = 0x1402_0005;
/// The version used by Fallout 3 and Fallout: New Vegas (and Oblivion's
/// later files, Skyrim and Fallout 4 with different Bethesda versions).
pub const V20_2_0_7: u32 = 0x1402_0007;

/// Formats a packed version number, e.g. `0x14020007` as `20.2.0.7`.
pub fn version_string(v: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        v >> 24,
        (v >> 16) & 0xFF,
        (v >> 8) & 0xFF,
        v & 0xFF
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// The text line the file starts with.
    pub version_line: String,
    pub version: u32,
    /// 11 for Fallout 3 and New Vegas.
    pub user_version: u32,
    /// Bethesda's own version: 34 for Fallout 3 and New Vegas, 0 if absent.
    pub bs_version: u32,
    pub author: String,
    pub process_script: String,
    pub export_script: String,
    /// Distinct block type names used in the file.
    pub block_types: Vec<String>,
    /// For each block, an index into `block_types`.
    pub block_type_index: Vec<u16>,
    /// For each block, its size in bytes.
    pub block_sizes: Vec<u32>,
    /// Strings that blocks refer to by index (names and the like).
    pub strings: Vec<String>,
    pub groups: Vec<u32>,
    /// File offset of the first block.
    pub data_start: usize,
}

/// Reads a Bethesda "export string": a u8 length (including a trailing
/// NUL) followed by the characters.
fn export_string(r: &mut Reader, what: &str) -> Result<String> {
    let len = usize::from(r.u8(what)?);
    Ok(latin1(r.take(len, what)?))
}

pub(crate) fn parse_header(bytes: &[u8]) -> Result<Header> {
    let newline = bytes.iter().take(128).position(|&b| b == b'\n');
    let line = newline.map(|n| String::from_utf8_lossy(&bytes[..n]).into_owned());
    let valid = line.as_deref().is_some_and(|l| {
        l.starts_with("Gamebryo File Format") || l.starts_with("NetImmerse File Format")
    });
    let (Some(newline), Some(version_line), true) = (newline, line, valid) else {
        let preview: String = bytes
            .iter()
            .take(24)
            .map(|&b| {
                if b.is_ascii_graphic() || b == b' ' {
                    char::from(b)
                } else {
                    '.'
                }
            })
            .collect();
        return Err(Error::NotANif { found: preview });
    };

    let mut r = Reader::at(bytes, newline + 1);
    let version = r.u32("the version number")?;
    if version < V20_2_0_5 {
        return Err(Error::Unsupported(format!(
            "version {} files have no block size table (Fallout 3 and New Vegas use 20.2.0.7)",
            version_string(version)
        )));
    }
    if version > V20_2_0_7 {
        return Err(Error::Unsupported(format!(
            "version {} is newer than Fallout 3 and New Vegas use (20.2.0.7)",
            version_string(version)
        )));
    }
    if r.u8("the endian flag")? != 1 {
        return Err(Error::Unsupported("big-endian (console) file".into()));
    }
    let user_version = r.u32("the user version")?;
    let num_blocks = r.u32("the block count")? as usize;

    let (mut bs_version, mut author, mut process_script, mut export_script) =
        (0, String::new(), String::new(), String::new());
    if version == V20_2_0_7 && user_version >= 3 {
        bs_version = r.u32("the Bethesda version")?;
        author = export_string(&mut r, "the author")?;
        if bs_version > 130 {
            r.u32("the Bethesda header")?;
        }
        if bs_version < 131 {
            process_script = export_string(&mut r, "the process script")?;
        }
        export_script = export_string(&mut r, "the export script")?;
        if bs_version == 130 {
            export_string(&mut r, "the max file path")?;
        }
    }

    let num_types = usize::from(r.u16("the block type count")?);
    let block_types = r.counted(num_types, 4, "block type names", |r| {
        r.sized_string("a block type name")
    })?;
    let block_type_index = r.counted(num_blocks, 2, "block type indexes", |r| {
        Ok(r.u16("a block type index")? & 0x7FFF)
    })?;
    if let Some(bad) = block_type_index
        .iter()
        .find(|&&i| usize::from(i) >= num_types)
    {
        return Err(Error::Malformed {
            offset: r.offset(),
            reason: format!("a block uses type {bad}, but only {num_types} types are listed"),
        });
    }
    let block_sizes = r.counted(num_blocks, 4, "block sizes", |r| r.u32("a block size"))?;

    let num_strings = r.u32("the string count")? as usize;
    let _max_string_length = r.u32("the string table")?;
    let strings = r.counted(num_strings, 4, "the string table", |r| {
        r.sized_string("a string")
    })?;
    let num_groups = r.u32("the group count")? as usize;
    let groups = r.counted(num_groups, 4, "groups", |r| r.u32("a group"))?;

    Ok(Header {
        version_line,
        version,
        user_version,
        bs_version,
        author,
        process_script,
        export_script,
        block_types,
        block_type_index,
        block_sizes,
        strings,
        groups,
        data_start: r.pos(),
    })
}
