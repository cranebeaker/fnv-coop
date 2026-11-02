//! Maps the data sections of the player's own FalloutNV.exe into game
//! memory at their addresses, so translated code reads the game's real
//! constants, strings, vtables and initial global values. Nothing from the
//! executable is part of this repository; the sections are read at run time
//! from the installation, like the game's data files.
//!
//! The Steam executable is packed, but only its `.text` is encrypted: its
//! `.rdata`, `.data`, `.tls` and `CONST` are byte-identical to the unpacked
//! file's (checked 2026-10-09). Each section is checked against the FNV-1a
//! 64-bit hash of version 1.4.0.525's section before it is mapped; a
//! different build is refused, because every address in the translations
//! belongs to 1.4.0.525.

use crate::mem::Mem;
use std::path::Path;

/// Image base of FalloutNV.exe.
pub const IMAGE_BASE: u32 = 0x0040_0000;

/// (name, virtual address, virtual size, FNV-1a 64 of the raw bytes) of
/// the sections that are mapped, FalloutNV.exe 1.4.0.525.
pub const SECTIONS: [(&str, u32, u32, u64); 4] = [
    (".rdata", 0x00FD_F000, RDATA_VSIZE, RDATA_HASH),
    (".data", 0x0118_3000, DATA_VSIZE, DATA_HASH),
    (".tls", 0x0127_2000, TLS_VSIZE, TLS_HASH),
    ("CONST", 0x0127_3000, CONST_VSIZE, CONST_HASH),
];

include!("exe_hashes.rs");

pub fn fnv1a64(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// One section header of a PE file.
#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub va: u32,
    pub vsize: u32,
    pub raw: Vec<u8>,
}

/// The section headers and raw bytes of a PE file.
pub fn sections(file: &[u8]) -> Result<Vec<Section>, String> {
    let le16 = |o: usize| -> Result<u16, String> {
        file.get(o..o + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .ok_or_else(|| "truncated PE header".to_string())
    };
    let le32 = |o: usize| -> Result<u32, String> {
        file.get(o..o + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or_else(|| "truncated PE header".to_string())
    };
    let pe = le32(0x3c)? as usize;
    if file.get(pe..pe + 4) != Some(b"PE\0\0") {
        return Err("not a PE file".into());
    }
    let n = le16(pe + 6)? as usize;
    let opt = le16(pe + 20)? as usize;
    let base = le32(pe + 24 + 28)?;
    let mut out = Vec::new();
    for i in 0..n {
        let s = pe + 24 + opt + 40 * i;
        let name_bytes = file.get(s..s + 8).ok_or("truncated section table")?;
        let name = String::from_utf8_lossy(name_bytes)
            .trim_end_matches('\0')
            .to_string();
        let vsize = le32(s + 8)?;
        let va = base + le32(s + 12)?;
        let rsize = le32(s + 16)? as usize;
        let rptr = le32(s + 20)? as usize;
        let raw = file
            .get(rptr..rptr + rsize)
            .ok_or_else(|| format!("section {name} runs past the end of the file"))?
            .to_vec();
        out.push(Section {
            name,
            va,
            vsize,
            raw,
        });
    }
    Ok(out)
}

/// One imported function: the address of its import-table slot (what the
/// game calls through, `call dword ptr [slot]`), its DLL and its name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub slot: u32,
    pub dll: String,
    pub name: String,
}

/// The import table of a PE file (by-ordinal imports are named `#<n>`).
pub fn imports(file: &[u8]) -> Result<Vec<Import>, String> {
    let secs = sections(file)?;
    let rd = |o: usize| -> Result<u32, String> {
        file.get(o..o + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or_else(|| "truncated PE file".to_string())
    };
    let pe = rd(0x3c)? as usize;
    let opt = pe + 24;
    let base = rd(opt + 28)?;
    let dir_rva = rd(opt + 96 + 8)?;
    // File bytes at a virtual address, through the section that holds it.
    let at = |va: u32| -> Option<&[u8]> {
        secs.iter().find_map(|s| {
            let off = va.checked_sub(s.va)? as usize;
            s.raw.get(off..)
        })
    };
    let cstr = |va: u32| -> String {
        let b = at(va).unwrap_or(&[]);
        let n = b.iter().position(|&c| c == 0).unwrap_or(b.len());
        String::from_utf8_lossy(&b[..n]).into_owned()
    };
    let word = |va: u32| -> u32 {
        at(va)
            .and_then(|b| b.get(..4))
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .unwrap_or(0)
    };
    let mut out = Vec::new();
    let mut d = base + dir_rva;
    loop {
        let (oft, name, ft) = (word(d), word(d + 12), word(d + 16));
        if name == 0 && ft == 0 {
            break;
        }
        let dll = cstr(base + name);
        let lookup = if oft != 0 { oft } else { ft };
        for i in 0.. {
            let entry = word(base + lookup + 4 * i);
            if entry == 0 {
                break;
            }
            let name = if entry & 0x8000_0000 != 0 {
                format!("#{}", entry & 0xffff)
            } else {
                cstr(base + entry + 2)
            };
            out.push(Import {
                slot: base + ft + 4 * i,
                dll: dll.clone(),
                name,
            });
        }
        d += 20;
    }
    Ok(out)
}

/// The FalloutNV.exe of an installation, given its `Data` folder: the
/// executable sits in the folder that contains `Data`.
pub fn installed_exe(data_dir: &Path) -> std::path::PathBuf {
    let game_dir = data_dir.parent().unwrap_or(data_dir);
    game_dir.join("FalloutNV.exe")
}

/// The data sections of `file` (the bytes of `exe`) after checking that it
/// is FalloutNV.exe 1.4.0.525, the build every translated address belongs
/// to. Any other build is refused with a message that says so.
pub fn checked_sections(exe: &Path, file: &[u8]) -> Result<Vec<Section>, String> {
    let refuse = |what: String| {
        format!(
            "{}: {what}. nv-rs needs FalloutNV.exe version 1.4.0.525 (the build it \
             was traced from); other builds are not supported yet.",
            exe.display()
        )
    };
    let secs = sections(file).map_err(refuse)?;
    for (name, va, _, hash) in SECTIONS {
        let s = secs
            .iter()
            .find(|s| s.name == name)
            .ok_or_else(|| refuse(format!("no {name} section")))?;
        if s.va != va || fnv1a64(&s.raw) != hash {
            return Err(refuse(format!(
                "section {name} is not 1.4.0.525's (address {:08x}, hash {:016x})",
                s.va,
                fnv1a64(&s.raw)
            )));
        }
    }
    Ok(secs)
}

/// Reads `exe` and checks it ([`checked_sections`]) without mapping it:
/// the startup check of an installation's executable.
pub fn verify(exe: &Path) -> Result<(), String> {
    let file = std::fs::read(exe).map_err(|e| format!("{}: {e}", exe.display()))?;
    checked_sections(exe, &file).map(|_| ())
}

/// Maps `.rdata`, `.data`, `.tls` and `CONST` of `exe` (the installed
/// FalloutNV.exe, packed or unpacked) after checking each against 1.4.0.525.
pub fn map_data_sections(mem: &mut Mem, exe: &Path) -> Result<(), String> {
    let file = std::fs::read(exe).map_err(|e| format!("{}: {e}", exe.display()))?;
    let secs = checked_sections(exe, &file)?;
    for (name, va, vsize, _) in SECTIONS {
        let s = secs.iter().find(|s| s.name == name).expect("checked above");
        mem.map(va, vsize.max(s.raw.len() as u32));
        let n = s.raw.len().min(vsize as usize);
        mem.write(va, &s.raw[..n]);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn other_files_are_refused_with_the_supported_build_named() {
        let err = checked_sections(Path::new("x.exe"), b"MZ but not a PE file").unwrap_err();
        assert!(err.starts_with("x.exe: "), "{err}");
        assert!(err.contains("version 1.4.0.525"), "{err}");
        let data = Path::new("C:/Games/Fallout New Vegas/Data");
        assert_eq!(
            installed_exe(data),
            Path::new("C:/Games/Fallout New Vegas/FalloutNV.exe")
        );
    }

    #[test]
    fn fnv_reference_values() {
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
    }

    /// With `NV_EXE` set, the import table parses and the slot the
    /// garbage collector calls (`call [0x00fdf0e4]`) is InterlockedExchange.
    #[test]
    fn imports_of_installed_exe_when_given() {
        let Ok(path) = std::env::var("NV_EXE") else {
            return;
        };
        let file = std::fs::read(&path).unwrap();
        let imps = imports(&file).unwrap();
        let x = imps.iter().find(|i| i.slot == 0x00fd_f0e4).unwrap();
        assert_eq!(x.name, "InterlockedExchange");
        let mut e = crate::Engine::with_exe(Path::new(&path)).unwrap();
        let p = e.mem.alloc(4);
        e.mem.set_u32(p, 5);
        assert_eq!(
            e.call(0x00fd_f0e4, &crate::args![crate::Ptr::<()>::new(p), 9i32])
                .i32(),
            5
        );
        assert_eq!(e.mem.u32(p), 9);
    }

    /// With `NV_EXE` set to an installed FalloutNV.exe 1.4.0.525, the data
    /// sections map and a known RTTI type name reads back.
    #[test]
    fn maps_installed_exe_when_given() {
        let Ok(path) = std::env::var("NV_EXE") else {
            return;
        };
        let mut m = Mem::new();
        map_data_sections(&mut m, Path::new(&path)).unwrap();
        // The complete object locator of BGSDehydrationStage's vtable
        // (0101144c) is at 011014e8; its TypeDescriptor's name is
        // ".?AVBGSDehydrationStage@@".
        let col = m.u32(0x0101_144c - 4);
        assert_eq!(col, 0x0110_14e8);
        let td = m.u32(col + 12);
        assert_eq!(m.cstr(td + 8), b".?AVBGSDehydrationStage@@");
    }
}
