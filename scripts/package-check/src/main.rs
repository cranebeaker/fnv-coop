//! package-check: what a package's files must not hold, checked before the ZIP is made
//! (`scripts/package-playtest.ps1`). Standalone, no dependencies.
//!
//! `package-check FOLDER [--needle TEXT]... [--program-needle TEXT]... [--game FILE]... [--max-run BYTES]`
//!
//! - Every file under FOLDER (with `--program-needle`, every `.exe`) is searched for each needle, as bytes and as UTF-16, any ASCII case
//!   (the build machine's user name and folders, private tool names).
//! - With `--game` (the player's own FalloutNV.exe, or a DLL; only read, on the packager's machine):
//!   the longest runs of bytes the program files share with it, at least [`MIN_RUN`] long, not
//!   counting runs of one repeated byte or word. A run this long would be data copied from the game.
//!
//!   Every `--game` file must be readable (one that isn't fails the check, it is never skipped). With
//!   `--max-run`, a shared run longer than that fails the check too: the C runtime's own tables are
//!   in both programs (the longest seen is about 2 KB), game data would be more.
//!
//! Prints what it finds; status 1 when a needle is found or a shared run is over the limit, 2 when
//! a `--game` file can't be read or the command line is wrong (a `--max-run` that isn't a number
//! included).
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// The shortest shared run reported.
const MIN_RUN: usize = 64;
/// The game's file is indexed in windows of this length, every `STEP` bytes: any shared run of at
/// least `WINDOW + STEP` bytes holds an indexed window.
const WINDOW: usize = 48;
const STEP: usize = 16;

fn files_under(folder: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files_under(&path, out);
        } else {
            out.push(path);
        }
    }
}

fn lower(b: u8) -> u8 {
    b.to_ascii_lowercase()
}

/// Offsets of `needle` (lower case ASCII) in `data`, any ASCII case.
fn find_all(data: &[u8], needle: &[u8]) -> Vec<usize> {
    let Some(&first) = needle.first() else {
        return vec![];
    };
    let mut found = Vec::new();
    let end = data.len().saturating_sub(needle.len());
    let mut at = 0;
    while at <= end && !data.is_empty() {
        if lower(data[at]) == first
            && data[at..at + needle.len()]
                .iter()
                .zip(needle)
                .all(|(a, b)| lower(*a) == *b)
        {
            found.push(at);
        }
        at += 1;
    }
    found
}

/// The two forms of a needle: bytes and UTF-16.
fn forms(needle: &str) -> [Vec<u8>; 2] {
    let bytes = needle.to_ascii_lowercase().into_bytes();
    let wide = bytes.iter().flat_map(|&b| [b, 0]).collect();
    [bytes, wide]
}

fn shown(data: &[u8], at: usize) -> String {
    let from = at.saturating_sub(24);
    let to = (at + 72).min(data.len());
    data[from..to]
        .iter()
        .filter(|&&b| b != 0)
        .map(|&b| {
            if (32..127).contains(&b) {
                b as char
            } else {
                '.'
            }
        })
        .collect()
}

fn fnv(data: &[u8]) -> u64 {
    data.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// Whether a run is only padding: one byte or one 4-byte word repeated.
fn is_filler(run: &[u8]) -> bool {
    run.len() >= 8 && run.iter().enumerate().all(|(i, b)| *b == run[i % 4])
}

/// The maximal runs of `data` of at least `MIN_RUN` bytes shared with the files indexed in `index`.
/// (offset in `data`, length, the indexed file's number).
fn shared_runs(
    data: &[u8],
    reference: &[Vec<u8>],
    index: &HashMap<u64, Vec<(u32, u32)>>,
) -> Vec<(usize, usize, usize)> {
    let mut runs = Vec::new();
    let mut at = 0usize;
    let mut skip_to = 0usize;
    while at + WINDOW <= data.len() {
        if at < skip_to {
            at += 1;
            continue;
        }
        if let Some(places) = index.get(&fnv(&data[at..at + WINDOW])) {
            // (length, the file, how far it reaches back).
            let mut best = (0usize, 0usize, 0usize);
            for &(file, place) in places {
                let (reference, place) = (&reference[file as usize], place as usize);
                if reference[place..place + WINDOW] != data[at..at + WINDOW] {
                    continue;
                }
                let mut length = WINDOW;
                while at + length < data.len()
                    && place + length < reference.len()
                    && data[at + length] == reference[place + length]
                {
                    length += 1;
                }
                let mut back = 0;
                while at > back
                    && place > back
                    && data[at - back - 1] == reference[place - back - 1]
                {
                    back += 1;
                }
                if length + back > best.0 {
                    best = (length + back, file as usize, back);
                }
            }
            if best.0 >= MIN_RUN {
                runs.push((at - best.2, best.0, best.1));
                skip_to = at - best.2 + best.0;
            }
        }
        at += 1;
    }
    runs
}

/// The `--game` files, all of them: a missing or unreadable one is an error, not a skipped check.
fn load_reference(games: &[PathBuf]) -> Result<Vec<Vec<u8>>, String> {
    games
        .iter()
        .map(|g| match std::fs::read(g) {
            Ok(data) if !data.is_empty() => Ok(data),
            Ok(_) => Err(format!("{}: empty", g.display())),
            Err(e) => Err(format!("{}: {e}", g.display())),
        })
        .collect()
}

/// What the command line asks for.
#[derive(Debug, Default, PartialEq)]
struct Arguments {
    folder: PathBuf,
    needles: Vec<String>,
    program_needles: Vec<String>,
    games: Vec<PathBuf>,
    max_run: Option<usize>,
}

const USAGE: &str = "usage: package-check FOLDER [--needle TEXT]... [--program-needle TEXT]... [--game FILE]... [--max-run BYTES]";

/// The command line after the program's name. A switch without its value, an unknown switch or a
/// `--max-run` that isn't a number is an error: a limit that was asked for is never dropped.
fn parse_arguments(args: &[String]) -> Result<Arguments, String> {
    let Some(folder) = args.first() else {
        return Err(USAGE.to_string());
    };
    let mut out = Arguments {
        folder: PathBuf::from(folder),
        ..Arguments::default()
    };
    let mut rest = args[1..].iter();
    while let Some(arg) = rest.next() {
        let value = rest
            .next()
            .cloned()
            .ok_or_else(|| format!("{arg} needs a value. {USAGE}"));
        match arg.as_str() {
            "--needle" => out.needles.push(value?),
            "--program-needle" => out.program_needles.push(value?),
            "--game" => out.games.push(PathBuf::from(value?)),
            "--max-run" => {
                let n = value?;
                let limit = n
                    .trim()
                    .parse()
                    .map_err(|_| format!("--max-run {n:?}: use a number of bytes"))?;
                out.max_run = Some(limit);
            }
            other => return Err(format!("what is {other}? {USAGE}")),
        }
    }
    Ok(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Arguments {
        folder,
        needles,
        program_needles,
        games,
        max_run,
    } = match parse_arguments(&args) {
        Ok(arguments) => arguments,
        Err(why) => {
            eprintln!("{why}");
            std::process::exit(2);
        }
    };
    let mut files = Vec::new();
    files_under(&folder, &mut files);
    files.sort();
    let mut bad = 0usize;
    for file in &files {
        let Ok(data) = std::fs::read(file) else {
            println!("{}: can't be read", file.display());
            bad += 1;
            continue;
        };
        let is_program = file
            .extension()
            .is_some_and(|x| x.eq_ignore_ascii_case("exe"));
        let extra: &[String] = if is_program { &program_needles } else { &[] };
        for needle in needles.iter().chain(extra) {
            for form in forms(needle) {
                let hits = find_all(&data, &form);
                if let Some(&first) = hits.first() {
                    println!(
                        "FOUND {:?} {} times in {}: ...{}...",
                        needle,
                        hits.len(),
                        file.display(),
                        shown(&data, first)
                    );
                    bad += hits.len();
                }
            }
        }
    }
    println!(
        "{} files, {} needles: {} hits",
        files.len(),
        needles.len(),
        bad
    );

    if !games.is_empty() {
        let reference = match load_reference(&games) {
            Ok(reference) => reference,
            Err(why) => {
                eprintln!("--game: {why}");
                std::process::exit(2);
            }
        };
        let mut index: HashMap<u64, Vec<(u32, u32)>> = HashMap::new();
        let mut distinct = HashSet::new();
        for (n, data) in reference.iter().enumerate() {
            let mut at = 0;
            while at + WINDOW <= data.len() {
                let window = &data[at..at + WINDOW];
                if !is_filler(window) && distinct.insert(fnv(window)) {
                    index
                        .entry(fnv(window))
                        .or_default()
                        .push((n as u32, at as u32));
                }
                at += STEP;
            }
        }
        println!(
            "reference: {} files, {} windows",
            reference.len(),
            distinct.len()
        );
        for file in &files {
            let extension = file
                .extension()
                .map(|x| x.to_string_lossy().to_ascii_lowercase());
            if extension.as_deref() != Some("exe") {
                continue;
            }
            let Ok(data) = std::fs::read(file) else {
                continue;
            };
            let mut runs: Vec<_> = shared_runs(&data, &reference, &index)
                .into_iter()
                .filter(|&(at, length, _)| !is_filler(&data[at..at + length]))
                .collect();
            runs.sort_by_key(|r| std::cmp::Reverse(r.1));
            let total: usize = runs.iter().map(|r| r.1).sum();
            println!(
                "{}: {} bytes, shares {} runs of {MIN_RUN}+ bytes, {} bytes in all, longest {}",
                file.display(),
                data.len(),
                runs.len(),
                total,
                runs.first().map_or(0, |r| r.1)
            );
            if let Some(limit) = max_run.filter(|&l| runs.first().is_some_and(|r| r.1 > l)) {
                println!(
                    "FOUND a run shared with the game longer than {limit} bytes in {}",
                    file.display()
                );
                bad += 1;
            }
            for &(at, length, n) in runs.iter().take(25) {
                println!(
                    "  {length} bytes at {at:#x}, with {}: {}",
                    games[n].file_name().unwrap_or_default().to_string_lossy(),
                    shown(&data, at + 24).chars().take(90).collect::<String>()
                );
            }
        }
    }
    std::process::exit(i32::from(bad != 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needles_are_found_in_either_form_and_any_case() {
        let [bytes, wide] = forms("Secret");
        assert_eq!(find_all(b"xx SECRET secret", &bytes), [3, 10]);
        assert_eq!(find_all(b"S\0e\0c\0R\0e\0T\0", &wide), [0]);
        assert!(find_all(b"", &bytes).is_empty());
    }

    #[test]
    fn a_max_run_that_is_not_a_number_is_an_error_not_no_limit() {
        let args = |list: &[&str]| -> Vec<String> { list.iter().map(|s| s.to_string()).collect() };
        let parsed =
            parse_arguments(&args(&["pkg", "--max-run", "4096", "--game", "g.exe"])).unwrap();
        assert_eq!(parsed.max_run, Some(4096));
        assert_eq!(parsed.games, [PathBuf::from("g.exe")]);
        assert_eq!(parse_arguments(&args(&["pkg"])).unwrap().max_run, None);
        for bad in [
            &["pkg", "--max-run", "4k"][..],
            &["pkg", "--max-run", "-1"],
            &["pkg", "--max-run", ""],
            &["pkg", "--max-run"],
            &["pkg", "--needle"],
            &["pkg", "--maxrun", "4096"],
            &[],
        ] {
            assert!(parse_arguments(&args(bad)).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn a_game_file_that_cannot_be_read_is_an_error_not_a_skip() {
        let dir = std::env::temp_dir().join(format!("package-check-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let good = dir.join("good.exe");
        std::fs::write(&good, b"MZ bytes").unwrap();
        assert_eq!(
            load_reference(std::slice::from_ref(&good)).unwrap().len(),
            1
        );
        let missing = dir.join("missing.exe");
        assert!(load_reference(&[good.clone(), missing]).is_err());
        let empty = dir.join("empty.exe");
        std::fs::write(&empty, b"").unwrap();
        assert!(load_reference(&[empty]).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_long_shared_run_is_found_and_filler_is_not() {
        let shared: Vec<u8> = (0..200u32).map(|i| (i * 7 % 251) as u8).collect();
        let mut reference = vec![1u8; 37];
        reference.extend_from_slice(&shared);
        reference.extend_from_slice(&[9u8; 100]);
        let mut data = vec![2u8; 101];
        data.extend_from_slice(&shared);
        data.extend_from_slice(&[3u8; 50]);
        let reference = vec![reference];
        let mut index: HashMap<u64, Vec<(u32, u32)>> = HashMap::new();
        let mut at = 0;
        while at + WINDOW <= reference[0].len() {
            let w = &reference[0][at..at + WINDOW];
            if !is_filler(w) {
                index.entry(fnv(w)).or_default().push((0, at as u32));
            }
            at += STEP;
        }
        let runs = shared_runs(&data, &reference, &index);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].1, 200);
        assert!(is_filler(&[0xcc; 64]));
        assert!(is_filler(&[1, 2, 3, 4, 1, 2, 3, 4]));
        assert!(!is_filler(&shared));
    }
}
