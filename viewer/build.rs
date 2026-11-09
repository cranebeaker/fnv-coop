//! The exe's icon: `nv-rs.ico` (Vault Boy holding the nv-rs cartridge, 16 to 256 pixels)
//! becomes the viewer's icon resource on Windows with the MSVC linker.
//!
//! No resource compiler is needed. The build writes the `.res` file itself:
//! - an empty first entry;
//! - one `RT_ICON` entry for each image in the `.ico`;
//! - an `RT_GROUP_ICON` (name 1) that lists them.
//!
//! The linker takes the `.res` as an input. Windows shows the first icon group as the exe's icon,
//! in Explorer, on the taskbar and in the window's title bar.

use std::env;
use std::fs;
use std::path::PathBuf;

const RT_ICON: u16 = 3;
const RT_GROUP_ICON: u16 = 14;
const LANG_EN_US: u16 = 0x0409;

/// One resource: the 32-byte header (numeric type and name), then the data padded to 4 bytes. The
/// empty first entry has language 0, as resource compilers write it.
fn entry(res: &mut Vec<u8>, kind: u16, name: u16, flags: u16, data: &[u8]) {
    let language = if kind == 0 { 0 } else { LANG_EN_US };
    let size = u32::try_from(data.len()).expect("an icon image under 4 GB");
    res.extend_from_slice(&size.to_le_bytes());
    res.extend_from_slice(&32u32.to_le_bytes());
    res.extend_from_slice(&[0xff, 0xff]);
    res.extend_from_slice(&kind.to_le_bytes());
    res.extend_from_slice(&[0xff, 0xff]);
    res.extend_from_slice(&name.to_le_bytes());
    res.extend_from_slice(&0u32.to_le_bytes()); // data version
    res.extend_from_slice(&flags.to_le_bytes());
    res.extend_from_slice(&language.to_le_bytes());
    res.extend_from_slice(&0u32.to_le_bytes()); // version
    res.extend_from_slice(&0u32.to_le_bytes()); // characteristics
    res.extend_from_slice(data);
    while res.len() % 4 != 0 {
        res.push(0);
    }
}

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

fn main() {
    println!("cargo:rerun-if-changed=nv-rs.ico");
    println!("cargo:rerun-if-changed=build.rs");
    let windows = env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows");
    let msvc = env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    if !(windows && msvc) {
        return;
    }
    let ico = fs::read("nv-rs.ico").expect("viewer/nv-rs.ico");
    assert!(
        ico.len() >= 6 && u16_at(&ico, 0) == 0 && u16_at(&ico, 2) == 1,
        "nv-rs.ico is not an icon file"
    );
    let count = usize::from(u16_at(&ico, 4));
    let mut res = Vec::new();
    entry(&mut res, 0, 0, 0, &[]);
    // The group: the .ico's directory with each image's file offset replaced by its resource id.
    let mut group = Vec::with_capacity(6 + 14 * count);
    group.extend_from_slice(&ico[0..6]);
    assert!(
        ico.len() >= 6 + 16 * count,
        "nv-rs.ico's directory is cut short"
    );
    for k in 0..count {
        let dir = 6 + 16 * k;
        let size = usize::try_from(u32_at(&ico, dir + 8)).expect("an image size");
        let offset = usize::try_from(u32_at(&ico, dir + 12)).expect("an image offset");
        let image = offset
            .checked_add(size)
            .and_then(|end| ico.get(offset..end))
            .expect("nv-rs.ico: an image runs past the end of the file");
        let id = u16::try_from(k + 1).expect("fewer than 65535 images");
        entry(&mut res, RT_ICON, id, 0x1010, image);
        group.extend_from_slice(&ico[dir..dir + 12]);
        group.extend_from_slice(&id.to_le_bytes());
    }
    entry(&mut res, RT_GROUP_ICON, 1, 0x1030, &group);
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("nv-rs.res");
    fs::write(&out, res).expect("write nv-rs.res");
    println!("cargo:rustc-link-arg-bins={}", out.display());
}
