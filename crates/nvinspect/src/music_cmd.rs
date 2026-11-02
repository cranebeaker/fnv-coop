//! Commands for MP3 files: the game's music (`Data\Music`).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use mp3::{Decoder, Problem};

use crate::fmt::{human_bytes, thousands};
use crate::{expect_args, file_name_of, CliError, Options};

enum Command {
    Info,
    Check,
    Wav(Option<String>, Option<f64>),
}

fn parse_command(command: &str, rest: &[String]) -> Result<Command, CliError> {
    match command {
        "info" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::Info)
        }
        "check" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::Check)
        }
        "wav" => {
            expect_args(command, rest, 0, 2)?;
            let seconds = match rest.get(1) {
                Some(s) => Some(
                    s.parse::<f64>()
                        .ok()
                        .filter(|v| v.is_finite() && *v > 0.0)
                        .ok_or_else(|| {
                            CliError::Usage(format!("'{s}' isn't a number of seconds"))
                        })?,
                ),
                None => None,
            };
            Ok(Command::Wav(rest.first().cloned(), seconds))
        }
        other => Err(CliError::Usage(format!(
            "'{other}' isn't an MP3 command; for .mp3 files use info, check or wav"
        ))),
    }
}

fn read(path: &Path) -> Result<Vec<u8>, CliError> {
    std::fs::read(path).map_err(|e| CliError::Open {
        path: path.display().to_string(),
        message: e.to_string(),
    })
}

fn open(path: &Path, bytes: Vec<u8>) -> Result<Decoder, CliError> {
    Decoder::new(bytes).map_err(|e| CliError::InFile {
        path: path.display().to_string(),
        message: e.to_string(),
    })
}

/// Runs a command on one .mp3 file.
pub fn run_file(
    out: &mut impl Write,
    path: &Path,
    command: &str,
    rest: &[String],
    options: &Options,
) -> Result<(), CliError> {
    let command = parse_command(command, rest)?;
    let bytes = read(path)?;
    match command {
        Command::Info => info(out, path, &bytes),
        Command::Check => {
            let size = bytes.len();
            let decoder = open(path, bytes)?;
            let started = Instant::now();
            let stats = Stats::of(decoder);
            describe_stats(out, path, size, &stats, started.elapsed().as_secs_f64())
        }
        Command::Wav(output, seconds) => wav(out, path, bytes, output, seconds, options.force),
    }
}

fn duration(samples: u64, rate: u32) -> String {
    let seconds = samples as f64 / f64::from(rate.max(1));
    let minutes = (seconds / 60.0).floor();
    format!("{}:{:05.2}", minutes, seconds - 60.0 * minutes)
}

fn info(out: &mut impl Write, path: &Path, bytes: &[u8]) -> Result<(), CliError> {
    let in_file = |e: mp3::Error| CliError::InFile {
        path: path.display().to_string(),
        message: e.to_string(),
    };
    let scan = mp3::scan(bytes).map_err(in_file)?;
    let decoder = Decoder::new(bytes).map_err(in_file)?;
    let info = decoder.info();
    writeln!(
        out,
        "File:         {} ({})",
        file_name_of(path),
        human_bytes(bytes.len() as u64)
    )?;
    let mut tags = Vec::new();
    if scan.tags.id3v2 > 0 {
        tags.push(format!("ID3v2 ({} bytes)", thousands(scan.tags.id3v2)));
    }
    if scan.tags.ape > 0 {
        tags.push(format!("APEv2 ({} bytes)", thousands(scan.tags.ape)));
    }
    if scan.tags.lyrics3 > 0 {
        tags.push(format!("Lyrics3 ({} bytes)", thousands(scan.tags.lyrics3)));
    }
    if scan.tags.id3v1 {
        tags.push("ID3v1".into());
    }
    writeln!(
        out,
        "Tags:         {}",
        if tags.is_empty() {
            "none".to_string()
        } else {
            tags.join(", ")
        }
    )?;
    let rate = if scan.min_bitrate == scan.max_bitrate {
        format!("{} kbit/s", scan.min_bitrate / 1000)
    } else {
        format!(
            "{}-{} kbit/s (variable)",
            scan.min_bitrate / 1000,
            scan.max_bitrate / 1000
        )
    };
    writeln!(
        out,
        "Format:       {} Layer III, {} Hz, {}, {rate}",
        info.version.name(),
        thousands(info.sample_rate as usize),
        info.mode.name(),
    )?;
    let checksums = if info.protected {
        "with checksums"
    } else {
        "no checksums"
    };
    writeln!(
        out,
        "Frames:       {} of {} samples, {checksums}{}",
        thousands(scan.frames),
        thousands(info.samples_per_frame),
        if scan.mode_changes > 0 {
            format!(", {} change channel mode", thousands(scan.mode_changes))
        } else {
            String::new()
        }
    )?;
    let samples = (scan.frames * info.samples_per_frame) as u64;
    writeln!(
        out,
        "Duration:     {} ({} samples per channel)",
        duration(samples, info.sample_rate),
        thousands(samples as usize)
    )?;
    match &info.xing {
        Some(x) => {
            let frames = x.frames.map_or("frames not given".into(), |f| {
                let matches = if f as usize == scan.frames {
                    "matches the count"
                } else {
                    "the count differs"
                };
                format!("{} frames ({matches})", thousands(f as usize))
            });
            let bytes_note = x.bytes.map_or(String::new(), |b| {
                format!(", {} bytes", thousands(b as usize))
            });
            let toc = if x.toc { ", seek table" } else { "" };
            writeln!(
                out,
                "Info frame:   {} header: {frames}{bytes_note}{toc}",
                x.tag
            )?;
        }
        None => writeln!(out, "Info frame:   none")?,
    }
    if let Some(v) = &info.vbri {
        writeln!(
            out,
            "VBRI header:  {} frames, {} bytes, delay {}",
            thousands(v.frames as usize),
            thousands(v.bytes as usize),
            v.delay
        )?;
    }
    match (&info.lame, info.gapless_samples) {
        (Some(lame), gapless) => {
            writeln!(
                out,
                "Encoder:      {}: delay {} samples, padding {} (tag checksum {})",
                lame.encoder.trim_end(),
                lame.delay,
                lame.padding,
                if lame.crc_ok { "ok" } else { "wrong" }
            )?;
            if let Some(total) = gapless {
                writeln!(
                    out,
                    "Gapless:      {} samples per channel ({}): the first {} and last {} are cut",
                    thousands(total as usize),
                    duration(total, info.sample_rate),
                    lame.delay + mp3::DECODER_DELAY,
                    i64::from(lame.padding) - i64::from(mp3::DECODER_DELAY)
                )?;
            }
        }
        (None, _) => writeln!(
            out,
            "Encoder:      not recorded (no LAME tag: no encoder delay or padding to trim)"
        )?,
    }
    if scan.skipped > 0 {
        writeln!(
            out,
            "Skipped:      {} bytes between frames that weren't frames",
            thousands(scan.skipped)
        )?;
    }
    Ok(())
}

/// What a whole decode found.
struct Stats {
    rate: u32,
    channels: usize,
    frames: usize,
    samples: u64,
    problems: usize,
    first_problem: Option<Problem>,
    peak: [f64; 2],
    sum: [f64; 2],
    squares: [f64; 2],
    /// Samples past full scale before clipping to 16 bits.
    clipped: usize,
    not_finite: usize,
    /// Samples per channel before the first one above -80 dBFS.
    leading_silence: u64,
    skipped: usize,
}

impl Stats {
    fn of(mut decoder: Decoder) -> Stats {
        let channels = usize::from(decoder.channels());
        let mut s = Stats {
            rate: decoder.sample_rate(),
            channels,
            frames: 0,
            samples: 0,
            problems: 0,
            first_problem: None,
            peak: [0.0; 2],
            sum: [0.0; 2],
            squares: [0.0; 2],
            clipped: 0,
            not_finite: 0,
            leading_silence: 0,
            skipped: 0,
        };
        let mut heard = false;
        for frame in decoder.by_ref() {
            s.frames += 1;
            if let Some(p) = frame.problem {
                s.problems += 1;
                s.first_problem.get_or_insert(p);
            }
            for group in frame.samples.chunks_exact(channels) {
                for (ch, &v) in group.iter().enumerate() {
                    if !v.is_finite() {
                        s.not_finite += 1;
                        continue;
                    }
                    let v = f64::from(v);
                    s.peak[ch] = s.peak[ch].max(v.abs());
                    s.sum[ch] += v;
                    s.squares[ch] += v * v;
                    if !(-1.0..32767.0 / 32768.0).contains(&v) {
                        s.clipped += 1;
                    }
                    heard |= v.abs() > 1e-4;
                }
                if !heard {
                    s.leading_silence += 1;
                }
                s.samples += 1;
            }
        }
        s.skipped = decoder.skipped_bytes();
        s
    }

    fn db(v: f64) -> String {
        if v <= 0.0 {
            "silent".into()
        } else {
            format!("{:.1} dBFS", 20.0 * v.log10())
        }
    }

    fn rms(&self, ch: usize) -> f64 {
        (self.squares[ch] / self.samples.max(1) as f64).sqrt()
    }

    fn mean(&self, ch: usize) -> f64 {
        self.sum[ch] / self.samples.max(1) as f64
    }
}

fn describe_stats(
    out: &mut impl Write,
    path: &Path,
    size: usize,
    s: &Stats,
    seconds: f64,
) -> Result<(), CliError> {
    let length = s.samples as f64 / f64::from(s.rate);
    writeln!(
        out,
        "File:         {} ({})",
        file_name_of(path),
        human_bytes(size as u64)
    )?;
    writeln!(
        out,
        "Decoded:      {} frames in {:.2} s ({:.0}x real time)",
        thousands(s.frames),
        seconds,
        length / seconds.max(1e-9)
    )?;
    writeln!(
        out,
        "Samples:      {} per channel ({}), {} Hz, {} channel{}",
        thousands(s.samples as usize),
        duration(s.samples, s.rate),
        thousands(s.rate as usize),
        s.channels,
        if s.channels == 1 { "" } else { "s" }
    )?;
    match s.first_problem {
        None => writeln!(out, "Problems:     none")?,
        Some(p) => writeln!(
            out,
            "Problems:     {} frames (first: {p})",
            thousands(s.problems)
        )?,
    }
    let per_channel =
        |f: &dyn Fn(usize) -> String| (0..s.channels).map(f).collect::<Vec<_>>().join(", ");
    writeln!(
        out,
        "Peak:         {}",
        per_channel(&|ch| Stats::db(s.peak[ch]))
    )?;
    writeln!(
        out,
        "Loudness:     {} (RMS)",
        per_channel(&|ch| Stats::db(s.rms(ch)))
    )?;
    writeln!(
        out,
        "DC offset:    {}",
        per_channel(&|ch| format!("{:+.6}", s.mean(ch)))
    )?;
    writeln!(
        out,
        "Clipped:      {} samples past full scale, {} not numbers",
        thousands(s.clipped),
        thousands(s.not_finite)
    )?;
    writeln!(
        out,
        "Silence:      {} samples ({:.1} ms) before the first sound",
        thousands(s.leading_silence as usize),
        s.leading_silence as f64 * 1000.0 / f64::from(s.rate)
    )?;
    if s.skipped > 0 {
        writeln!(
            out,
            "Skipped:      {} bytes between frames",
            thousands(s.skipped)
        )?;
    }
    Ok(())
}

fn wav(
    out: &mut impl Write,
    path: &Path,
    bytes: Vec<u8>,
    output: Option<String>,
    seconds: Option<f64>,
    force: bool,
) -> Result<(), CliError> {
    let target = output.map_or_else(
        || {
            let stem = path
                .file_stem()
                .map_or("music".into(), |s| s.to_string_lossy().into_owned());
            PathBuf::from(format!("{stem}.wav"))
        },
        PathBuf::from,
    );
    if target.exists() && !force {
        return Err(CliError::Usage(format!(
            "{} already exists; add --force to overwrite it",
            target.display()
        )));
    }
    let mut decoder = open(path, bytes)?;
    let (rate, channels) = (decoder.sample_rate(), decoder.channels());
    let limit = seconds.map_or(usize::MAX, |s| {
        (s * f64::from(rate)).round() as usize * usize::from(channels)
    });
    let mut samples = Vec::new();
    let mut buffer = vec![0i16; 8192];
    while samples.len() < limit {
        let want = buffer.len().min(limit - samples.len());
        let n = decoder.read_i16(&mut buffer[..want]);
        samples.extend_from_slice(&buffer[..n]);
        if n < want {
            break;
        }
    }
    let file = std::fs::File::create(&target).map_err(|e| CliError::Open {
        path: target.display().to_string(),
        message: e.to_string(),
    })?;
    let mut writer = std::io::BufWriter::new(file);
    mp3::wav::write(&mut writer, rate, channels, &samples)?;
    writer.flush()?;
    let per_channel = samples.len() / usize::from(channels);
    writeln!(
        out,
        "Wrote {} ({} Hz, {} channels, {}) to {}",
        human_bytes(44 + 2 * samples.len() as u64),
        thousands(rate as usize),
        channels,
        duration(per_channel as u64, rate),
        target.display()
    )?;
    Ok(())
}

/// Whether a folder (or one below it) holds .mp3 files.
pub fn has_music(folder: &Path) -> bool {
    !mp3_files(folder).is_empty()
}

fn mp3_files(folder: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut folders = vec![folder.to_path_buf()];
    while let Some(dir) = folders.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                folders.push(path);
            } else if path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("mp3"))
            {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// Decodes every .mp3 in a folder and its subfolders: one line each.
pub fn run_folder(
    out: &mut impl Write,
    folder: &Path,
    command: &str,
    rest: &[String],
    options: &Options,
) -> Result<(), CliError> {
    if command != "check" && command != "info" {
        return Err(CliError::Usage(format!(
            "'{command}' isn't a music folder command; for a folder of .mp3 files use check"
        )));
    }
    expect_args(command, rest, 0, 0)?;
    let files: Vec<PathBuf> = mp3_files(folder)
        .into_iter()
        .filter(|p| options.matches(&[p.to_str()]))
        .take(options.limit.unwrap_or(usize::MAX))
        .collect();
    let started = Instant::now();
    let next = AtomicUsize::new(0);
    // By file: the decode's statistics and the frames the header declares.
    type Outcome = Result<(Stats, Option<u32>), String>;
    let results: Mutex<Vec<(usize, Outcome)>> = Mutex::new(Vec::new());
    let workers = std::thread::available_parallelism().map_or(4, |n| n.get());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(path) = files.get(i) else {
                    break;
                };
                let result = std::fs::read(path)
                    .map_err(|e| e.to_string())
                    .and_then(|bytes| {
                        let decoder = Decoder::new(bytes).map_err(|e| e.to_string())?;
                        let declared = decoder.info().xing.as_ref().and_then(|x| x.frames);
                        Ok((Stats::of(decoder), declared))
                    });
                results.lock().unwrap().push((i, result));
            });
        }
    });
    let mut results = results.into_inner().unwrap();
    results.sort_by_key(|(i, _)| *i);

    writeln!(
        out,
        "{:<52} {:>6} {:>9} {:>6} {:>2}  {:>10} {:>10} {:>9} {:>6} {:>8}  Problems",
        "File", "Frames", "Length", "Hz", "Ch", "Peak", "RMS", "DC", "Clip", "Lead ms"
    )?;
    let (mut ok, mut failed, mut total_frames, mut problem_frames, mut seconds) =
        (0, 0, 0usize, 0usize, 0.0f64);
    for (i, result) in &results {
        let path = &files[*i];
        let name = path
            .strip_prefix(folder)
            .unwrap_or(path)
            .display()
            .to_string();
        match result {
            Err(e) => {
                failed += 1;
                writeln!(out, "{name:<52} could not be decoded: {e}")?;
            }
            Ok((s, declared)) => {
                ok += 1;
                total_frames += s.frames;
                problem_frames += s.problems;
                seconds += s.samples as f64 / f64::from(s.rate);
                let peak = (0..s.channels).map(|c| s.peak[c]).fold(0.0, f64::max);
                let rms = (0..s.channels).map(|c| s.rms(c)).fold(0.0, f64::max);
                let dc = (0..s.channels).map(|c| s.mean(c).abs()).fold(0.0, f64::max);
                let mut notes = Vec::new();
                if let Some(p) = s.first_problem {
                    notes.push(format!("{} ({p})", s.problems));
                }
                if s.not_finite > 0 {
                    notes.push(format!("{} not numbers", s.not_finite));
                }
                if let Some(d) = *declared {
                    if d as usize != s.frames {
                        notes.push(format!("header says {d} frames"));
                    }
                }
                if s.skipped > 0 {
                    notes.push(format!("{} bytes skipped", s.skipped));
                }
                writeln!(
                    out,
                    "{:<52} {:>6} {:>9} {:>6} {:>2}  {:>10} {:>10} {:>9.6} {:>6} {:>8.1}  {}",
                    crate::fmt::truncate(&name, 52),
                    s.frames,
                    duration(s.samples, s.rate),
                    s.rate,
                    s.channels,
                    Stats::db(peak),
                    Stats::db(rms),
                    dc,
                    s.clipped,
                    s.leading_silence as f64 * 1000.0 / f64::from(s.rate),
                    if notes.is_empty() {
                        "-".into()
                    } else {
                        notes.join("; ")
                    }
                )?;
            }
        }
    }
    writeln!(out)?;
    writeln!(
        out,
        "{} files decoded ({} failed): {} frames, {} of audio, {} frames with problems, in {:.1} s",
        ok,
        failed,
        thousands(total_frames),
        duration((seconds * 1000.0) as u64, 1000),
        thousands(problem_frames),
        started.elapsed().as_secs_f64()
    )?;
    Ok(())
}
