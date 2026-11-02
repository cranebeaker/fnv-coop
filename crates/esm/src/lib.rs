//! Reader for the plugin format used by Fallout 3 and Fallout: New Vegas
//! (`.esm` master files and `.esp` plugins).
//!
//! A plugin is a flat sequence of *groups* (`GRUP`), each holding *records*
//! (a weapon, an NPC, a cell, a placed object...) and, for cells and
//! worldspaces, nested sub-groups. Every record carries a list of typed
//! *subrecords* (`EDID` editor ID, `FULL` display name, `DATA`, ...).
//!
//! This crate reads the file structure, indexes every record by type and
//! form ID, and decodes subrecords on demand, including zlib-compressed
//! records. It does not ship or embed any game data: point it at a plugin
//! from an install you own.
//!
//! ```no_run
//! use esm::{sig, Plugin};
//!
//! let plugin = Plugin::open("Data/FalloutNV.esm")?;
//! for entry in plugin.records_of_type(sig::WEAP) {
//!     let record = plugin.record(entry)?;
//!     println!("{} {:?}", entry.header.form_id, record.full_name());
//! }
//! # Ok::<(), esm::Error>(())
//! ```

mod cursor;
pub mod error;
pub mod inflate;
pub mod load_order;
pub mod plugin;
pub mod record;
pub mod text;
pub mod types;
pub mod weapon;

pub use error::{Error, Result};
pub use load_order::{ActivePlugins, LoadOrder, LoadedPlugin, RecordRef};
pub use plugin::{FormOrigin, Plugin, PluginHeader, RecordEntry};
pub use record::{flags, GroupHeader, GroupKind, Record, RecordHeader, Subrecord};
pub use types::{sig, FormId, FourCC};
pub use weapon::{Weapon, WeaponStats};
