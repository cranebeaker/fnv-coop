//! `NV_SHOT` frame captures from the wgpu backend, numbered by `Present` as the forwarding
//! layer numbers its own (`engine-platform` `d3d9real::shot`), so the two can be compared frame by
//! frame: `wgpu-frame-NNNNNN.bmp` in `NV_SHOT_WGPU`'s folder, or `NV_SHOT`'s. Diagnostic only.

use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Where and which frames to capture.
#[derive(Debug, Clone)]
pub struct Shots {
    pub dir: PathBuf,
    pub every: Duration,
    pub frames: Option<(u64, u64)>,
    /// `NV_SHOT_LIST=N,N,..`: exactly these presents (to match another run's captures).
    pub list: Option<Vec<u64>>,
    last: Option<Instant>,
}

impl Shots {
    /// From the environment: `NV_SHOT=DIR[,every_ms]`, `NV_SHOT_FRAMES=FROM-TO`,
    /// `NV_SHOT_WGPU=DIR` (this backend's folder, when set).
    pub fn from_env() -> Option<Shots> {
        let value = std::env::var("NV_SHOT_WGPU")
            .ok()
            .or_else(|| std::env::var("NV_SHOT").ok())?;
        let mut shots = parse(&value)?;
        if let Ok(own) = std::env::var("NV_SHOT_WGPU") {
            shots.dir = PathBuf::from(own.split(',').next().unwrap_or(&own));
        }
        shots.frames = std::env::var("NV_SHOT_FRAMES")
            .ok()
            .and_then(|v| parse_frames(&v));
        shots.list = std::env::var("NV_SHOT_LIST")
            .ok()
            .map(|v| v.split(',').filter_map(|n| n.trim().parse().ok()).collect());
        let _ = std::fs::create_dir_all(&shots.dir);
        Some(shots)
    }

    /// Whether present `frame` (counted from 1) is taken.
    pub fn take(&mut self, frame: u64) -> bool {
        if let Some(list) = &self.list {
            return list.contains(&frame);
        }
        if self
            .frames
            .is_some_and(|(from, to)| frame < from || frame > to)
        {
            return false;
        }
        if self.last.is_some_and(|last| last.elapsed() < self.every) {
            return false;
        }
        self.last = Some(Instant::now());
        true
    }

    pub fn path(&self, frame: u64) -> PathBuf {
        self.dir.join(format!("wgpu-frame-{frame:06}.bmp"))
    }
}

/// `DIR[,every_ms]` (5000 ms when not given), as `d3d9real` reads `NV_SHOT`.
pub fn parse(value: &str) -> Option<Shots> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    let (dir, every) = match value.rsplit_once(',') {
        Some((dir, ms)) => match ms.trim().parse::<u64>() {
            Ok(ms) => (dir, ms),
            Err(_) => (value, 5000),
        },
        None => (value, 5000),
    };
    Some(Shots {
        dir: PathBuf::from(dir),
        every: Duration::from_millis(every.max(1)),
        frames: None,
        list: None,
        last: None,
    })
}

pub fn parse_frames(value: &str) -> Option<(u64, u64)> {
    let (from, to) = value.trim().split_once('-')?;
    Some((from.trim().parse().ok()?, to.trim().parse().ok()?))
}

/// A top-down 32-bit BMP of `width` x `height` B, G, R, X pixels (the forwarding layer's own
/// layout, so one reader takes both).
pub fn bmp(width: u32, height: u32, bgrx: &[u8]) -> Vec<u8> {
    let image = width * height * 4;
    let mut out = Vec::with_capacity(54 + image as usize);
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&(54 + image).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&54u32.to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(width as i32).to_le_bytes());
    out.extend_from_slice(&(-(height as i32)).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&32u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&image.to_le_bytes());
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(bgrx);
    out
}

/// Reads a BMP written by [`bmp`] (or by the forwarding layer): width, height and B, G, R, X
/// rows top first.
pub fn read_bmp(bytes: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    if bytes.get(0..2)? != b"BM" {
        return None;
    }
    let word = |at: usize| -> Option<u32> {
        Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
    };
    let offset = word(10)? as usize;
    let width = word(18)?;
    let height = word(22)? as i32;
    let bits = u16::from_le_bytes(bytes.get(28..30)?.try_into().ok()?);
    if bits != 32 {
        return None;
    }
    let rows = height.unsigned_abs();
    let size = (width * rows * 4) as usize;
    let data = bytes.get(offset..offset + size)?;
    if height < 0 {
        return Some((width, rows, data.to_vec()));
    }
    // Bottom-up: flip.
    let stride = (width * 4) as usize;
    let mut out = Vec::with_capacity(size);
    for r in (0..rows as usize).rev() {
        out.extend_from_slice(&data[r * stride..(r + 1) * stride]);
    }
    Some((width, rows, out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bmp_reads_back_as_written() {
        let pixels: Vec<u8> = (0..2 * 3 * 4).map(|i| i as u8).collect();
        let (w, h, p) = read_bmp(&bmp(2, 3, &pixels)).unwrap();
        assert_eq!((w, h), (2, 3));
        assert_eq!(p, pixels);
    }

    #[test]
    fn the_frame_range_and_interval_pick_frames() {
        let mut s = parse("x,1").unwrap();
        s.frames = Some((3, 4));
        assert!(!s.take(2));
        assert!(s.take(3));
        assert_eq!(parse_frames(" 10 - 20 "), Some((10, 20)));
    }
}
