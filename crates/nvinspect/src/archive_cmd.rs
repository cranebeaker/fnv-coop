//! Commands for BSA archives.

use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use bsa::{archive_flags, content_flags, normalize_path, Archive};

use crate::fmt::{human_bytes, thousands};
use crate::{dds_cmd, expect_args, file_name_of, nif_cmd, CliError, Options};

enum Command {
    Info,
    Files,
    Extract {
        path: String,
        output: Option<String>,
    },
    Nif(String),
    Obj {
        path: String,
        output: Option<String>,
    },
    CheckNifs,
    Dds(String),
    Png {
        path: String,
        output: Option<String>,
    },
    CheckTextures,
}

fn parse_command(command: &str, rest: &[String]) -> Result<Command, CliError> {
    match command {
        "info" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::Info)
        }
        "files" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::Files)
        }
        "extract" => {
            expect_args(command, rest, 1, 1)?;
            Ok(Command::Extract {
                path: rest[0].clone(),
                output: rest.get(1).cloned(),
            })
        }
        "nif" => {
            expect_args(command, rest, 1, 0)?;
            Ok(Command::Nif(rest[0].clone()))
        }
        "obj" => {
            expect_args(command, rest, 1, 1)?;
            Ok(Command::Obj {
                path: rest[0].clone(),
                output: rest.get(1).cloned(),
            })
        }
        "check-nifs" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::CheckNifs)
        }
        "dds" => {
            expect_args(command, rest, 1, 0)?;
            Ok(Command::Dds(rest[0].clone()))
        }
        "png" => {
            expect_args(command, rest, 1, 1)?;
            Ok(Command::Png {
                path: rest[0].clone(),
                output: rest.get(1).cloned(),
            })
        }
        "check-textures" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::CheckTextures)
        }
        "types" | "list" | "weapons" | "show" => Err(CliError::Usage(format!(
            "'{command}' works on plugins and Data folders; see `nvinspect --help` for archive commands"
        ))),
        other => Err(CliError::Usage(format!("unknown command '{other}'"))),
    }
}

pub fn run(
    out: &mut impl Write,
    path: &Path,
    command: &str,
    rest: &[String],
    options: &Options,
) -> Result<(), CliError> {
    let command = parse_command(command, rest)?;
    let archive = Archive::open(path).map_err(|e| match e {
        bsa::Error::Io(io) => CliError::Open {
            path: path.display().to_string(),
            message: io.to_string(),
        },
        other => CliError::InFile {
            path: path.display().to_string(),
            message: other.to_string(),
        },
    })?;
    match command {
        Command::Info => info(out, &archive, path),
        Command::Files => files(out, &archive, options),
        Command::Extract { path, output } => extract(out, &archive, &path, output, options.force),
        Command::Nif(path) => {
            let (nif, entry_path) = read_nif(&archive, &path)?;
            nif_cmd::info(out, &nif, &entry_path)
        }
        Command::Obj { path, output } => {
            let (nif, entry_path) = read_nif(&archive, &path)?;
            let default = nif_cmd::default_obj_name(&entry_path);
            nif_cmd::export_obj(out, &nif, &entry_path, output, &default, options.force)
        }
        Command::CheckNifs => nif_cmd::check_archive(out, &archive, options),
        Command::Dds(path) => {
            let (texture, entry_path) = read_dds(&archive, &path)?;
            dds_cmd::info(out, &texture, &entry_path)
        }
        Command::Png { path, output } => {
            let (texture, entry_path) = read_dds(&archive, &path)?;
            let default = dds_cmd::default_png_name(&entry_path);
            dds_cmd::export_png(out, &texture, &entry_path, output, &default, options.force)
        }
        Command::CheckTextures => dds_cmd::check_archive(out, &archive, options),
    }
}

fn read_dds(archive: &Archive, wanted: &str) -> Result<(dds::Dds, String), CliError> {
    let entry = archive
        .find(wanted)
        .ok_or_else(|| not_found(archive, wanted))?;
    let bytes = archive.read(entry)?;
    Ok((
        dds_cmd::parse_archived(bytes, &entry.path)?,
        entry.path.clone(),
    ))
}

fn read_nif(archive: &Archive, wanted: &str) -> Result<(nif::Nif, String), CliError> {
    let entry = archive
        .find(wanted)
        .ok_or_else(|| not_found(archive, wanted))?;
    let bytes = archive.read(entry)?;
    Ok((
        nif_cmd::parse_archived(bytes, &entry.path)?,
        entry.path.clone(),
    ))
}

fn info(out: &mut impl Write, archive: &Archive, path: &Path) -> Result<(), CliError> {
    let h = archive.header();
    let compressed = archive.files().iter().filter(|f| f.compressed).count();
    writeln!(
        out,
        "File:         {} ({})",
        file_name_of(path),
        human_bytes(archive.len())
    )?;
    writeln!(out, "Format:       BSA version {}", h.version)?;
    writeln!(
        out,
        "Flags:        {:#05x} ({})",
        h.archive_flags,
        archive_flags::describe(h.archive_flags)
    )?;
    writeln!(
        out,
        "Contents:     {}",
        content_flags::describe(h.content_flags)
    )?;
    writeln!(out, "Folders:      {}", thousands(archive.folders().len()))?;
    writeln!(
        out,
        "Files:        {} ({} compressed)",
        thousands(archive.files().len()),
        thousands(compressed)
    )?;

    // Every stored name hash should match the name read for it; if they do,
    // the names and file records were paired up correctly.
    let check = archive.check_name_hashes();
    let all_match =
        check.files_matched == check.files_total && check.folders_matched == check.folders_total;
    if all_match {
        writeln!(
            out,
            "Name hashes:  all {} match",
            thousands(check.files_total + check.folders_total)
        )?;
    } else {
        writeln!(
            out,
            "Name hashes:  {} of {} file names and {} of {} folder names match their stored hash",
            thousands(check.files_matched),
            thousands(check.files_total),
            thousands(check.folders_matched),
            thousands(check.folders_total)
        )?;
    }

    let mut by_extension: HashMap<String, (usize, u64)> = HashMap::new();
    for f in archive.files() {
        let ext = f
            .name
            .rsplit_once('.')
            .map_or("(none)".to_string(), |(_, e)| format!(".{e}"));
        let slot = by_extension.entry(ext).or_default();
        slot.0 += 1;
        slot.1 += u64::from(f.stored_size);
    }
    let mut by_extension: Vec<_> = by_extension.into_iter().collect();
    by_extension.sort_by(|a, b| b.1 .1.cmp(&a.1 .1).then(a.0.cmp(&b.0)));
    writeln!(out, "\nFile types (by stored size):")?;
    for (ext, (count, bytes)) in by_extension.iter().take(10) {
        let files = if *count == 1 { "file " } else { "files" };
        writeln!(
            out,
            "  {ext:<8} {:>9} {files}  {:>10}",
            thousands(*count),
            human_bytes(*bytes)
        )?;
    }
    Ok(())
}

fn files(out: &mut impl Write, archive: &Archive, options: &Options) -> Result<(), CliError> {
    writeln!(out, "{:>10}  C  Path", "Stored")?;
    let mut shown = 0;
    for f in archive.files() {
        if options.limit_reached(shown) {
            break;
        }
        if !options.matches(&[Some(&f.path)]) {
            continue;
        }
        let c = if f.compressed { "c" } else { " " };
        writeln!(
            out,
            "{:>10}  {c}  {}",
            human_bytes(u64::from(f.stored_size)),
            f.path
        )?;
        shown += 1;
    }
    writeln!(
        out,
        "\n{} shown of {} files (C = compressed)",
        thousands(shown),
        thousands(archive.files().len())
    )?;
    Ok(())
}

fn extract(
    out: &mut impl Write,
    archive: &Archive,
    wanted: &str,
    output: Option<String>,
    force: bool,
) -> Result<(), CliError> {
    let entry = archive
        .find(wanted)
        .ok_or_else(|| not_found(archive, wanted))?;
    let data = archive.read(entry)?;

    let target = match output {
        Some(o) if Path::new(&o).is_dir() => Path::new(&o).join(&entry.name),
        Some(o) => PathBuf::from(o),
        None => PathBuf::from(&entry.name),
    };
    if target.exists() && !force {
        return Err(CliError::NotFound(format!(
            "{} already exists; add --force to overwrite it",
            target.display()
        )));
    }
    std::fs::write(&target, &data).map_err(|e| CliError::Open {
        path: target.display().to_string(),
        message: e.to_string(),
    })?;
    writeln!(
        out,
        "Wrote {} ({})",
        target.display(),
        human_bytes(data.len() as u64)
    )?;
    Ok(())
}

/// A not-found error that suggests files with the same name elsewhere.
fn not_found(archive: &Archive, wanted: &str) -> CliError {
    let wanted = normalize_path(wanted);
    let name = wanted.rsplit('\\').next().unwrap_or(&wanted).to_string();
    let similar: Vec<&str> = archive
        .files()
        .iter()
        .filter(|f| f.name == name)
        .take(5)
        .map(|f| f.path.as_str())
        .collect();
    let hint = if similar.is_empty() {
        format!("Run `files --grep {name}` to search for it.")
    } else {
        format!("Files with that name: {}", similar.join(", "))
    };
    CliError::NotFound(format!("no file '{wanted}' in this archive. {hint}"))
}
