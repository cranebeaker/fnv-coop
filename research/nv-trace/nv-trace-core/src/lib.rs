//! nv-trace's portable parts: the record format both recorders write ([`format`]), the
//! function lists ([`list`]), the trace diff ([`diff`]), the path checks of the player-folder
//! guard ([`guard`]) and the key script ([`keys`]). No external crates, no platform code, so
//! all of it is unit-tested on any host.

pub mod d3d;
pub mod diff;
pub mod dump;
pub mod format;
pub mod genlist;
pub mod guard;
pub mod keys;
pub mod list;
