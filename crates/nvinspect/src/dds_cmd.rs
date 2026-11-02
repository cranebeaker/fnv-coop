//! Commands for DDS textures, loose or inside archives.

use std::collections::BTreeMap;
use std::io::Write;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use bsa::Archive;
use dds::Dds;

use crate::fmt::{human_bytes, thousands};
use crate::{expect_args, file_name_of, CliError, Options};

enum Command {
    Info,
    Png(Option<String>),
}

fn parse_command(command: &str, rest: &[String]) -> Result<Command, CliError> {
    match command {
        "info" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::Info)
        }
        "png" => {
            expect_args(command, rest, 0, 1)?;
            Ok(Command::Png(rest.first().cloned()))
        }
        other => Err(CliError::Usage(format!(
            "'{other}' isn't a texture command; for .dds files use info or png"
        ))),
    }
}

fn to_cli(path: &str, e: dds::Error) -> CliError {
    match e {
        dds::Error::Io(io) => CliError::Open {
            path: path.to_string(),
            message: io.to_string(),
        },
        other => CliError::InFile {
            path: path.to_string(),
            message: other.to_string(),
        },
    }
}

/// Runs a command on a loose .dds file.
pub fn run_file(
    out: &mut impl Write,
    path: &Path,
    command: &str,
    rest: &[String],
    options: &Options,
) -> Result<(), CliError> {
    let command = parse_command(command, rest)?;
    let label = path.display().to_string();
    let bytes = std::fs::read(path).map_err(|e| to_cli(&label, e.into()))?;
    let texture = Dds::parse(bytes).map_err(|e| to_cli(&label, e))?;
    let name = file_name_of(path);
    match command {
        Command::Info => info(out, &texture, &name),
        Command::Png(output) => export_png(
            out,
            &texture,
            &name,
            output,
            &default_png_name(&name),
            options.force,
        ),
    }
}

/// Parses a .dds read from an archive.
pub fn parse_archived(bytes: Vec<u8>, path: &str) -> Result<Dds, CliError> {
    Dds::parse(bytes).map_err(|e| to_cli(path, e))
}

pub fn default_png_name(dds_name: &str) -> String {
    let file = dds_name.rsplit(['\\', '/']).next().unwrap_or(dds_name);
    let stem = file.rsplit_once('.').map_or(file, |(stem, _)| stem);
    format!("{stem}.png")
}

pub fn info(out: &mut impl Write, texture: &Dds, label: &str) -> Result<(), CliError> {
    writeln!(out, "File:         {label}")?;
    writeln!(out, "Format:       {}", texture.format().name())?;
    writeln!(
        out,
        "Size:         {} x {}",
        texture.width(),
        texture.height()
    )?;
    let mips = if texture.mip_count() == texture.declared_mip_count() {
        texture.mip_count().to_string()
    } else {
        format!(
            "{} present ({} declared; the smallest are missing)",
            texture.mip_count(),
            texture.declared_mip_count()
        )
    };
    writeln!(out, "Mip levels:   {mips}")?;
    if texture.is_cube_map() {
        writeln!(out, "Cube map:     {} faces", texture.face_count())?;
    }
    let data: usize = (0..texture.mip_count())
        .map(|l| texture.level_size(l))
        .sum::<usize>()
        * texture.face_count();
    writeln!(out, "Image data:   {}", human_bytes(data as u64))?;

    let rgba = texture.decode_rgba(0, 0).map_err(|e| to_cli(label, e))?;
    let alphas = rgba.chunks_exact(4).map(|p| p[3]);
    let (transparent, partial) = alphas.fold((0usize, 0usize), |(t, p), a| match a {
        0 => (t + 1, p),
        255 => (t, p),
        _ => (t, p + 1),
    });
    let pixels = rgba.len() / 4;
    let alpha = if transparent + partial == 0 {
        "fully opaque".to_string()
    } else {
        format!(
            "{:.0}% fully transparent, {:.0}% partly",
            100.0 * transparent as f64 / pixels as f64,
            100.0 * partial as f64 / pixels as f64
        )
    };
    writeln!(out, "Alpha:        {alpha}")?;
    Ok(())
}

pub fn export_png(
    out: &mut impl Write,
    texture: &Dds,
    label: &str,
    output: Option<String>,
    default_name: &str,
    force: bool,
) -> Result<(), CliError> {
    let target = match output {
        Some(o) if Path::new(&o).is_dir() => Path::new(&o).join(default_name),
        Some(o) => PathBuf::from(o),
        None => PathBuf::from(default_name),
    };
    if target.exists() && !force {
        return Err(CliError::NotFound(format!(
            "{} already exists; add --force to overwrite it",
            target.display()
        )));
    }

    // Cube maps are laid out as their faces side by side.
    let (w, h) = (texture.width() as usize, texture.height() as usize);
    let faces = texture.face_count();
    let mut image = vec![0u8; w * faces * h * 4];
    for face in 0..faces {
        let rgba = texture.decode_rgba(face, 0).map_err(|e| to_cli(label, e))?;
        for y in 0..h {
            let src = &rgba[y * w * 4..(y + 1) * w * 4];
            let dst = (y * w * faces + face * w) * 4;
            image[dst..dst + w * 4].copy_from_slice(src);
        }
    }
    let mut file = std::fs::File::create(&target).map_err(|e| CliError::Open {
        path: target.display().to_string(),
        message: e.to_string(),
    })?;
    dds::png::write_rgba(&mut file, (w * faces) as u32, h as u32, &image)?;
    let note = if faces > 1 {
        format!(", {faces} cube faces side by side")
    } else {
        String::new()
    };
    writeln!(
        out,
        "Wrote {} ({} x {}{note})",
        target.display(),
        w * faces,
        h
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// check-textures: decode every texture in an archive
// ---------------------------------------------------------------------------

#[derive(Default)]
struct Totals {
    decoded: usize,
    by_format: BTreeMap<String, usize>,
    cube_maps: usize,
    missing_mips: usize,
    no_mips: usize,
    not_power_of_two: usize,
    largest: (u32, u32, String),
    failures: Vec<(String, String)>,
    failure_kinds: BTreeMap<String, usize>,
}

impl Totals {
    fn merge(&mut self, other: Totals) {
        self.decoded += other.decoded;
        for (k, v) in other.by_format {
            *self.by_format.entry(k).or_default() += v;
        }
        self.cube_maps += other.cube_maps;
        self.missing_mips += other.missing_mips;
        self.no_mips += other.no_mips;
        self.not_power_of_two += other.not_power_of_two;
        if u64::from(other.largest.0) * u64::from(other.largest.1)
            > u64::from(self.largest.0) * u64::from(self.largest.1)
        {
            self.largest = other.largest;
        }
        self.failures.extend(other.failures);
        for (k, v) in other.failure_kinds {
            *self.failure_kinds.entry(k).or_default() += v;
        }
    }

    fn fail(&mut self, path: &str, kind: &str, message: String) {
        self.failures.push((path.to_string(), message));
        *self.failure_kinds.entry(kind.to_string()).or_default() += 1;
    }
}

fn check_one(archive: &Archive, index: usize, totals: &mut Totals) {
    let entry = &archive.files()[index];
    let bytes = match archive.read(entry) {
        Ok(b) => b,
        Err(e) => return totals.fail(&entry.path, "archive read", e.to_string()),
    };
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| -> dds::Result<Dds> {
        let texture = Dds::parse(bytes)?;
        for face in 0..texture.face_count() {
            texture.decode_rgba(face, 0)?;
        }
        Ok(texture)
    }));
    match outcome {
        Ok(Ok(t)) => {
            totals.decoded += 1;
            *totals.by_format.entry(t.format().name()).or_default() += 1;
            totals.cube_maps += usize::from(t.is_cube_map());
            totals.missing_mips += usize::from(t.mip_count() < t.declared_mip_count());
            totals.no_mips +=
                usize::from(t.declared_mip_count() == 1 && t.width().max(t.height()) > 1);
            totals.not_power_of_two +=
                usize::from(!t.width().is_power_of_two() || !t.height().is_power_of_two());
            if u64::from(t.width()) * u64::from(t.height())
                > u64::from(totals.largest.0) * u64::from(totals.largest.1)
            {
                totals.largest = (t.width(), t.height(), entry.path.clone());
            }
        }
        Ok(Err(e)) => {
            let kind = match &e {
                dds::Error::Unsupported(_) => "unsupported format",
                dds::Error::NotADds { .. } => "not a DDS file",
                _ => "damaged or truncated",
            };
            totals.fail(&entry.path, kind, e.to_string());
        }
        Err(_) => totals.fail(
            &entry.path,
            "crash (reader bug)",
            "the reader panicked on this file".into(),
        ),
    }
}

pub fn check_archive(
    out: &mut impl Write,
    archive: &Archive,
    options: &Options,
) -> Result<(), CliError> {
    let mut targets: Vec<usize> = archive
        .files()
        .iter()
        .enumerate()
        .filter(|(_, f)| f.name.ends_with(".dds") && options.matches(&[Some(&f.path)]))
        .map(|(i, _)| i)
        .collect();
    if let Some(limit) = options.limit {
        targets.truncate(limit);
    }
    if targets.is_empty() {
        writeln!(out, "No .dds files to check in this archive.")?;
        return Ok(());
    }

    let started = Instant::now();
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let totals = Mutex::new(Totals::default());
    let threads = std::thread::available_parallelism()
        .map_or(4, |n| n.get())
        .min(16);
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                let mut local = Totals::default();
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(&index) = targets.get(i) else { break };
                    check_one(archive, index, &mut local);
                    let finished = done.fetch_add(1, Ordering::Relaxed) + 1;
                    if finished % 2000 == 0 {
                        eprintln!(
                            "  checked {} of {}...",
                            thousands(finished),
                            thousands(targets.len())
                        );
                    }
                }
                totals
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .merge(local);
            });
        }
    });
    panic::set_hook(previous_hook);

    let mut totals = totals.into_inner().unwrap_or_else(|p| p.into_inner());
    writeln!(
        out,
        "Checked {} .dds files in {:.1} s",
        thousands(targets.len()),
        started.elapsed().as_secs_f64()
    )?;
    writeln!(
        out,
        "  decoded completely: {:>9}",
        thousands(totals.decoded)
    )?;
    writeln!(
        out,
        "  failed:             {:>9}",
        thousands(totals.failures.len())
    )?;
    writeln!(
        out,
        "  cube maps:          {:>9}",
        thousands(totals.cube_maps)
    )?;
    writeln!(
        out,
        "  no mip levels:      {:>9}",
        thousands(totals.no_mips)
    )?;
    writeln!(
        out,
        "  missing some mips:  {:>9}",
        thousands(totals.missing_mips)
    )?;
    writeln!(
        out,
        "  not power-of-two:   {:>9}",
        thousands(totals.not_power_of_two)
    )?;
    if totals.largest.0 > 0 {
        writeln!(
            out,
            "  largest:            {} x {} ({})",
            totals.largest.0, totals.largest.1, totals.largest.2
        )?;
    }
    let mut formats: Vec<_> = totals.by_format.into_iter().collect();
    formats.sort_by_key(|entry| std::cmp::Reverse(entry.1));
    writeln!(out, "\nFormats:")?;
    for (name, n) in &formats {
        writeln!(out, "  {name:<16} {:>9}", thousands(*n))?;
    }
    if !totals.failures.is_empty() {
        writeln!(out, "\nFailures by cause:")?;
        for (k, n) in &totals.failure_kinds {
            writeln!(out, "  {k:<28} {:>7}", thousands(*n))?;
        }
        totals.failures.sort();
        writeln!(out, "\nFirst failures:")?;
        for (path, message) in totals.failures.iter().take(15) {
            writeln!(out, "  {path}\n    {message}")?;
        }
    }
    Ok(())
}
