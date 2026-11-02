//! The simulator again, built as an example so `coop-sim watch` can rebuild it while the
//! watcher (target/<profile>/coop-sim) is running: a running exe cannot be overwritten on
//! Windows, an example's exe is a different file. Same code as src/main.rs.
include!("../src/main.rs");
