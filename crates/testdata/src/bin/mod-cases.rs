//! Writes the mod setups the load-order and archive tests use
//! (`testdata::mods`) to a folder, one subfolder per case, so they can be
//! inspected or tried in nvinspect, the viewer or the original game.
//!
//! ```text
//! cargo run -p testdata --bin mod-cases -- <folder> [case ...]
//! ```

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(out) = args.first().map(PathBuf::from) else {
        eprintln!("usage: mod-cases <folder> [case ...]");
        eprintln!("cases:");
        for c in testdata::mods::cases() {
            eprintln!("    {}", c.name);
        }
        return ExitCode::FAILURE;
    };
    let wanted = &args[1..];
    let mut written = 0;
    for case in testdata::mods::cases() {
        if !wanted.is_empty() && !wanted.iter().any(|w| w == case.name) {
            continue;
        }
        let dir = out.join(case.name);
        if let Err(e) = case.write(&dir) {
            eprintln!("could not write {}: {e}", dir.display());
            return ExitCode::FAILURE;
        }
        println!("{}", dir.display());
        written += 1;
    }
    if written == 0 {
        eprintln!("no case matches {wanted:?}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
