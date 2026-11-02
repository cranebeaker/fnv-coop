//! Commands for Bink movies (`Data\Video\*.bik`).

use std::io::Write;
use std::path::Path;
use std::time::Instant;

use bink::{AudioDecoder, Decoder, Movie};

use crate::fmt::{human_bytes, thousands};
use crate::{expect_args, CliError, Options};

enum Command {
    Info,
    /// Decode every frame and write each plane's SHA-256, as a file
    /// (`research/nv-oracle`'s `nv-bink` writes the same format from the
    /// game's own library) or to the output. With `bgrx`, the hash of the
    /// frame converted to 32-bit colour as the game shows it instead.
    Frames(Option<String>, bool),
    /// Decode an audio track to raw interleaved 16-bit little-endian PCM.
    Audio(String, usize),
}

fn parse_command(command: &str, rest: &[String]) -> Result<Command, CliError> {
    match command {
        "info" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::Info)
        }
        "audio" => {
            expect_args(command, rest, 1, 2)?;
            let track = match rest.get(1) {
                Some(t) => t
                    .parse()
                    .map_err(|_| CliError::Usage(format!("'{t}' isn't a track number")))?,
                None => 0,
            };
            Ok(Command::Audio(rest[0].clone(), track))
        }
        "frames" => {
            expect_args(command, rest, 0, 2)?;
            let bgrx = match rest.get(1).map(String::as_str) {
                None => false,
                Some("bgrx") => true,
                Some(other) => {
                    return Err(CliError::Usage(format!(
                        "'{other}' isn't a frame format; the only one is bgrx"
                    )))
                }
            };
            Ok(Command::Frames(rest.first().cloned(), bgrx))
        }
        other => Err(CliError::Usage(format!(
            "'{other}' isn't a movie command; for .bik files use info, frames or audio"
        ))),
    }
}

/// Runs a command on one .bik file.
pub fn run_file(
    out: &mut impl Write,
    path: &Path,
    command: &str,
    rest: &[String],
    options: &Options,
) -> Result<(), CliError> {
    let command = parse_command(command, rest)?;
    let bytes = std::fs::read(path).map_err(|e| CliError::Open {
        path: path.display().to_string(),
        message: e.to_string(),
    })?;
    let movie = Movie::parse(&bytes).map_err(|e| CliError::InFile {
        path: path.display().to_string(),
        message: e.to_string(),
    })?;
    match command {
        Command::Info => info(out, &movie, bytes.len()),
        Command::Frames(file, bgrx) => frames(out, path, &movie, file, bgrx, options.force),
        Command::Audio(file, track) => audio(out, &movie, &file, track, options.force),
    }
}

fn info(out: &mut impl Write, m: &Movie, size: usize) -> Result<(), CliError> {
    let frames = m.frame_count();
    let seconds = if m.fps() > 0.0 {
        frames as f64 / m.fps()
    } else {
        0.0
    };
    writeln!(
        out,
        "Bink revision '{}', {} x {}, {} frames at {:.3} fps ({}/{}), {:.1} s, {}",
        m.revision as char,
        m.width,
        m.height,
        thousands(frames as usize),
        m.fps(),
        m.fps_num,
        m.fps_den,
        seconds,
        human_bytes(size as u64)
    )?;
    writeln!(
        out,
        "video flags {:08x}{}{}; largest frame {}",
        m.video_flags,
        if m.video_flags & bink::FLAG_ALPHA != 0 {
            ", alpha plane"
        } else {
            ""
        },
        if m.video_flags & bink::FLAG_GRAY != 0 {
            ", grey"
        } else {
            ""
        },
        human_bytes(m.largest_frame as u64)
    )?;
    let mut keyframes = 0;
    let mut video = 0u64;
    let mut audio = vec![0u64; m.audio.len()];
    for i in 0..frames {
        let p = m.packet(i).map_err(|e| CliError::InFile {
            path: format!("frame {i}"),
            message: e.to_string(),
        })?;
        keyframes += p.keyframe as u32;
        video += p.video.len() as u64;
        for (t, a) in p.audio.iter().enumerate() {
            audio[t] += a.len() as u64;
        }
    }
    writeln!(
        out,
        "{keyframes} keyframes; video data {}",
        human_bytes(video)
    )?;
    for (t, track) in m.audio.iter().enumerate() {
        writeln!(
            out,
            "audio track {} (id {}): {} Hz, {} channel(s), {}{}, data {}",
            t,
            track.id,
            track.sample_rate,
            track.channels(),
            if track.flags & bink::AUDIO_DCT != 0 {
                "DCT"
            } else {
                "RDFT"
            },
            if track.flags & bink::AUDIO_16BIT != 0 {
                ", 16-bit"
            } else {
                ""
            },
            human_bytes(audio[t])
        )?;
    }
    Ok(())
}

fn frames(
    out: &mut impl Write,
    path: &Path,
    m: &Movie,
    file: Option<String>,
    bgrx: bool,
    force: bool,
) -> Result<(), CliError> {
    let mut target: Box<dyn Write> = match &file {
        Some(name) => {
            if Path::new(name).exists() && !force {
                return Err(CliError::Usage(format!(
                    "'{name}' already exists; add --force to overwrite it"
                )));
            }
            Box::new(std::io::BufWriter::new(std::fs::File::create(name)?))
        }
        None => Box::new(std::io::sink()),
    };
    let mut decoder = Decoder::new(m.width, m.height, m.revision, m.video_flags);
    let (w, h) = (m.width as usize, m.height as usize);
    let luma = w * h;
    let chroma = (w / 2) * (h / 2);
    let mut buf = Vec::new();
    let started = Instant::now();
    let mut failed = 0u32;
    for i in 0..m.frame_count() {
        let p = m.packet(i).map_err(|e| CliError::InFile {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;
        let status = match decoder.decode(p.video) {
            Ok(()) => 0,
            Err(e) => {
                failed += 1;
                if failed <= 10 {
                    writeln!(out, "frame {i}: {e}")?;
                }
                1
            }
        };
        let line = if bgrx {
            decoder.to_bgrx(&mut buf);
            format!("{i}\t{status}\t{}", crate::sha256::hex_digest(&buf))
        } else {
            decoder.to_yv12(&mut buf);
            format!(
                "{i}\t{status}\t{}\t{}\t{}",
                crate::sha256::hex_digest(&buf[..luma]),
                crate::sha256::hex_digest(&buf[luma..luma + chroma]),
                crate::sha256::hex_digest(&buf[luma + chroma..luma + 2 * chroma])
            )
        };
        if file.is_some() {
            writeln!(target, "{line}")?;
        } else {
            writeln!(out, "{line}")?;
        }
    }
    target.flush()?;
    writeln!(
        out,
        "{} frames decoded in {:.1} s; {} with errors",
        m.frame_count(),
        started.elapsed().as_secs_f64(),
        failed
    )?;
    Ok(())
}

fn audio(
    out: &mut impl Write,
    m: &Movie,
    file: &str,
    track: usize,
    force: bool,
) -> Result<(), CliError> {
    let Some(t) = m.audio.get(track) else {
        return Err(CliError::Usage(format!(
            "the movie has {} audio track(s); there is no track {track}",
            m.audio.len()
        )));
    };
    if Path::new(file).exists() && !force {
        return Err(CliError::Usage(format!(
            "'{file}' already exists; add --force to overwrite it"
        )));
    }
    let mut decoder =
        AudioDecoder::new(t.sample_rate as u32, t.channels(), t.flags).map_err(|e| {
            CliError::InFile {
                path: format!("audio track {track}"),
                message: e.to_string(),
            }
        })?;
    let started = Instant::now();
    let mut samples = Vec::new();
    for i in 0..m.frame_count() {
        let p = m.packet(i).map_err(|e| CliError::InFile {
            path: format!("frame {i}"),
            message: e.to_string(),
        })?;
        decoder
            .decode_packet(p.audio[track], &mut samples)
            .map_err(|e| CliError::InFile {
                path: format!("frame {i}, audio track {track}"),
                message: e.to_string(),
            })?;
    }
    let mut bytes = Vec::with_capacity(samples.len() * 2);
    for v in &samples {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    std::fs::write(file, &bytes)?;
    let frames = samples.len() / t.channels() as usize;
    writeln!(
        out,
        "{} samples per channel ({:.3} s at {} Hz, {} channel(s)) decoded in {:.1} s to {file}",
        thousands(frames),
        frames as f64 / t.sample_rate as f64,
        t.sample_rate,
        t.channels(),
        started.elapsed().as_secs_f64()
    )?;
    Ok(())
}
