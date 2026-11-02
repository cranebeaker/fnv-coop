//! Commands for the game's shader packages (`Data\Shaders\*.sdp`).

use std::io::Write;
use std::path::{Path, PathBuf};

use shaders::{disassemble, Package};

use crate::fmt::{human_bytes, thousands};
use crate::{expect_args, file_name_of, CliError, Options};

enum Command {
    Info,
    Shader(String),
    Dump(Option<String>),
}

fn parse_command(command: &str, rest: &[String]) -> Result<Command, CliError> {
    match command {
        "info" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::Info)
        }
        "shader" => {
            expect_args(command, rest, 1, 0)?;
            Ok(Command::Shader(rest[0].clone()))
        }
        "dump" => {
            expect_args(command, rest, 0, 1)?;
            Ok(Command::Dump(rest.first().cloned()))
        }
        other => Err(CliError::Usage(format!(
            "'{other}' isn't a shader package command; for .sdp files use info, shader or dump"
        ))),
    }
}

/// Runs a command on a shader package.
pub fn run_file(
    out: &mut impl Write,
    path: &Path,
    command: &str,
    rest: &[String],
    options: &Options,
) -> Result<(), CliError> {
    let command = parse_command(command, rest)?;
    let label = path.display().to_string();
    let bytes = std::fs::read(path).map_err(|e| CliError::Open {
        path: label.clone(),
        message: e.to_string(),
    })?;
    let package = Package::parse(&bytes).map_err(|e| CliError::InFile {
        path: label.clone(),
        message: e.to_string(),
    })?;
    match command {
        Command::Info => info(out, path, bytes.len(), &package, options),
        Command::Shader(name) => {
            let shader = package.get(&name).ok_or_else(|| {
                CliError::NotFound(format!(
                    "no shader named '{name}' in {} (`info` lists them)",
                    file_name_of(path)
                ))
            })?;
            let listing = disassemble(&shader.bytecode).map_err(|e| CliError::InFile {
                path: format!("{label}: {name}"),
                message: e.to_string(),
            })?;
            writeln!(out, "// {} from {}", shader.name, file_name_of(path))?;
            write!(out, "{}", listing.text)?;
            Ok(())
        }
        Command::Dump(folder) => dump(out, path, &package, folder, options),
    }
}

fn info(
    out: &mut impl Write,
    path: &Path,
    size: usize,
    package: &Package,
    options: &Options,
) -> Result<(), CliError> {
    writeln!(
        out,
        "File:     {} ({})",
        file_name_of(path),
        human_bytes(size as u64)
    )?;
    writeln!(out, "Layout:   {}", package.layout)?;
    writeln!(out, "Shaders:  {}", thousands(package.shaders.len()))?;
    writeln!(out)?;
    writeln!(
        out,
        "  {:<28} {:<7} {:>8} {:>5}  Constants",
        "Name", "Model", "Bytes", "Instr"
    )?;
    let mut shown = 0;
    for shader in &package.shaders {
        if !options.matches(&[Some(shader.name.as_str())]) {
            continue;
        }
        if options.limit_reached(shown) {
            break;
        }
        shown += 1;
        match disassemble(&shader.bytecode) {
            Ok(d) => {
                let names: Vec<&str> = d.constants.iter().map(|c| c.name.as_str()).collect();
                writeln!(
                    out,
                    "  {:<28} {:<7} {:>8} {:>5}  {}",
                    shader.name,
                    d.version.to_string(),
                    thousands(shader.bytecode.len()),
                    d.instructions,
                    names.join(", ")
                )?;
            }
            Err(e) => writeln!(out, "  {:<28} unreadable: {e}", shader.name)?,
        }
    }
    Ok(())
}

/// Writes every shader's listing to its own text file, plus an index.
fn dump(
    out: &mut impl Write,
    path: &Path,
    package: &Package,
    folder: Option<String>,
    options: &Options,
) -> Result<(), CliError> {
    let folder = folder.map_or_else(
        || {
            let stem = path
                .file_stem()
                .map_or("shaders".into(), |s| s.to_string_lossy().into_owned());
            PathBuf::from(format!("{stem}_shaders"))
        },
        PathBuf::from,
    );
    let occupied = std::fs::read_dir(&folder).is_ok_and(|mut d| d.next().is_some());
    if occupied && !options.force {
        return Err(CliError::Usage(format!(
            "{} already has files in it; add --force to overwrite them",
            folder.display()
        )));
    }
    std::fs::create_dir_all(&folder).map_err(|e| CliError::Open {
        path: folder.display().to_string(),
        message: e.to_string(),
    })?;
    let mut index = String::new();
    let mut written = 0;
    for shader in &package.shaders {
        if !options.matches(&[Some(shader.name.as_str())]) {
            continue;
        }
        let text = match disassemble(&shader.bytecode) {
            Ok(d) => {
                index.push_str(&format!(
                    "{}\t{}\t{}\n",
                    shader.name,
                    d.version,
                    d.constants
                        .iter()
                        .map(|c| format!("{} {}", c.name, c.registers()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
                format!("// {} from {}\n{}", shader.name, file_name_of(path), d.text)
            }
            Err(e) => {
                index.push_str(&format!("{}\tunreadable: {e}\n", shader.name));
                continue;
            }
        };
        // Names are plain file names in the game's packages; keep anything
        // else from escaping the folder.
        let safe: String = shader
            .name
            .chars()
            .map(|c| if "\\/:*?\"<>|".contains(c) { '_' } else { c })
            .collect();
        let file = folder.join(format!("{safe}.txt"));
        std::fs::write(&file, text).map_err(|e| CliError::Open {
            path: file.display().to_string(),
            message: e.to_string(),
        })?;
        written += 1;
    }
    let index_file = folder.join("index.txt");
    std::fs::write(&index_file, index).map_err(|e| CliError::Open {
        path: index_file.display().to_string(),
        message: e.to_string(),
    })?;
    writeln!(
        out,
        "Wrote {} shader listings and index.txt to {}",
        thousands(written),
        folder.display()
    )?;
    Ok(())
}
