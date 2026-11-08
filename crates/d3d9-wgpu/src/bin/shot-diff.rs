//! Compares the wgpu backend's `NV_SHOT` frames with the forwarding layer's, frame by frame
//! (`shot-diff <folder> [wgpu folder] [diff image folder]`): `frame-NNNNNN-*.bmp` (the real
//! Direct3D 9) against `wgpu-frame-NNNNNN.bmp` of the same present. Per frame: pixels that differ,
//! the largest and mean difference per channel, pixels off by more than 2 and by more than 16, and
//! PSNR; with a third folder, an image of each frame's differences (x8, red where over 16).

use d3d9_wgpu::shot::{bmp, read_bmp};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn frames(dir: &Path, prefix: &str) -> BTreeMap<u64, PathBuf> {
    let mut out = BTreeMap::new();
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if let Some(rest) = name.strip_prefix(prefix) {
            if let Some(n) = rest.get(..6).and_then(|n| n.parse::<u64>().ok()) {
                if name.ends_with(".bmp") {
                    out.insert(n, e.path());
                }
            }
        }
    }
    out
}

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = PathBuf::from(args.next().expect("the NV_SHOT folder"));
    let wdir = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| dir.clone());
    let out = args.next().map(PathBuf::from);
    if let Some(o) = &out {
        let _ = std::fs::create_dir_all(o);
    }
    let real = frames(&dir, "frame-");
    let wgpu = frames(&wdir, "wgpu-frame-");
    println!("frame\tpixels\tdiffer\tdiffer%\tmax\tmean\tover2\tover16\tpsnr");
    let (mut all, mut identical) = (0, 0);
    for (n, rp) in &real {
        let Some(wp) = wgpu.get(n) else { continue };
        let (Some(a), Some(b)) = (
            std::fs::read(rp).ok().and_then(|x| read_bmp(&x)),
            std::fs::read(wp).ok().and_then(|x| read_bmp(&x)),
        ) else {
            println!("{n}\tunreadable");
            continue;
        };
        if (a.0, a.1) != (b.0, b.1) {
            println!("{n}\tsizes differ: {}x{} / {}x{}", a.0, a.1, b.0, b.1);
            continue;
        }
        let (w, h) = (a.0, a.1);
        let mut differ = 0u64;
        let mut over2 = 0u64;
        let mut over16 = 0u64;
        let mut max = 0u32;
        let mut sum = 0u64;
        let mut sq = 0u64;
        let mut image = vec![0u8; (w * h * 4) as usize];
        for i in 0..(w * h) as usize {
            let mut m = 0u32;
            for c in 0..3 {
                let d = (i32::from(a.2[4 * i + c]) - i32::from(b.2[4 * i + c])).unsigned_abs();
                sum += u64::from(d);
                sq += u64::from(d * d);
                m = m.max(d);
            }
            if m > 0 {
                differ += 1;
            }
            if m > 2 {
                over2 += 1;
            }
            if m > 16 {
                over16 += 1;
                image[4 * i + 2] = 255;
            } else {
                let v = (m * 8).min(255) as u8;
                image[4 * i] = v;
                image[4 * i + 1] = v;
                image[4 * i + 2] = v;
            }
            max = max.max(m);
        }
        let px = u64::from(w * h);
        let mse = sq as f64 / (3 * px) as f64;
        let psnr = if mse == 0.0 {
            f64::INFINITY
        } else {
            10.0 * (255.0f64 * 255.0 / mse).log10()
        };
        all += 1;
        if differ == 0 {
            identical += 1;
        }
        println!(
            "{n}\t{px}\t{differ}\t{:.3}\t{max}\t{:.4}\t{over2}\t{over16}\t{psnr:.2}",
            100.0 * differ as f64 / px as f64,
            sum as f64 / (3 * px) as f64
        );
        if let Some(o) = &out {
            let _ = std::fs::write(o.join(format!("diff-{n:06}.bmp")), bmp(w, h, &image));
        }
    }
    println!("# {all} frames compared, {identical} identical");
}
