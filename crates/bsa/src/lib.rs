//! Reader for the BSA archives used by Fallout 3 and Fallout: New Vegas
//! (format version 104; Oblivion's version 103 is read too).
//!
//! An archive is a directory of folders and files followed by the file data.
//! Files may be individually zlib-compressed. Opening an archive reads only
//! the directory; file data is read from disk when asked for, so multi-GB
//! texture archives don't have to fit in memory.
//!
//! ```no_run
//! let archive = bsa::Archive::open("Data/Fallout - Meshes.bsa")?;
//! if let Some(entry) = archive.find("meshes/weapons/1handpistol/10mmpistol.nif") {
//!     let bytes = archive.read(entry)?;
//!     println!("{} bytes", bytes.len());
//! }
//! # Ok::<(), bsa::Error>(())
//! ```

mod archive;
mod error;
pub mod hash;

pub use archive::{
    archive_flags, content_flags, normalize_path, Archive, FileEntry, Folder, HashCheck, Header,
    HEADER_LEN, MAGIC,
};
pub use error::{Error, Result};
