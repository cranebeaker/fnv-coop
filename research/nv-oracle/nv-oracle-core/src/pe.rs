//! A bounds-checked reader for the parts of a PE32 file the tools need:
//! the headers, the section table and the import table. Nothing is trusted
//! from the file; every offset is checked before it is used.

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub name: String,
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub raw_offset: u32,
    pub raw_size: u32,
    pub characteristics: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pe {
    pub machine: u16,
    pub characteristics: u16,
    pub dll_characteristics: u16,
    pub image_base: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub entry_rva: u32,
    pub sections: Vec<Section>,
    /// RVA and size of the import directory (zero when absent).
    pub import_dir: (u32, u32),
    /// RVA and size of the TLS directory (zero when absent).
    pub tls_dir: (u32, u32),
    /// Offset in the headers (and so in the file and in a mapped copy) of
    /// the 8-byte TLS entry of the data directory table, when the table is
    /// long enough to have one.
    pub tls_dir_entry: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportName {
    Name(String),
    Ordinal(u16),
}

/// One imported symbol and the address-table slot the loader would fill.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Import {
    pub dll: String,
    pub name: ImportName,
    /// RVA of the 4-byte slot in the import address table.
    pub iat_rva: u32,
}

impl fmt::Display for Import {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            ImportName::Name(n) => write!(f, "{}!{}", self.dll, n),
            ImportName::Ordinal(o) => write!(f, "{}!#{}", self.dll, o),
        }
    }
}

pub const MACHINE_I386: u16 = 0x14c;
const MAX_IMAGE_SIZE: u32 = 0x4000_0000;
/// Index of the TLS entry in the optional header's data directory table.
const TLS_DIRECTORY: usize = 9;
const MAX_IMPORTS: usize = 200_000;

fn u16_at(d: &[u8], off: usize) -> Result<u16, String> {
    d.get(off..off + 2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .ok_or_else(|| format!("file ends at offset {off:#x}"))
}

fn u32_at(d: &[u8], off: usize) -> Result<u32, String> {
    d.get(off..off + 4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or_else(|| format!("file ends at offset {off:#x}"))
}

impl Pe {
    /// Parse a PE32 (32-bit) file. PE32+ and non-x86 images are rejected.
    pub fn parse(data: &[u8]) -> Result<Pe, String> {
        if data.get(..2) != Some(b"MZ") {
            return Err("not a PE file (missing MZ header)".into());
        }
        let pe_off = u32_at(data, 0x3c)? as usize;
        if data.get(pe_off..pe_off + 4) != Some(b"PE\0\0") {
            return Err("missing PE signature".into());
        }
        let coff = pe_off + 4;
        let machine = u16_at(data, coff)?;
        let section_count = u16_at(data, coff + 2)? as usize;
        let opt_size = u16_at(data, coff + 16)? as usize;
        let characteristics = u16_at(data, coff + 18)?;
        let opt = coff + 20;
        if machine != MACHINE_I386 {
            return Err(format!("machine {machine:#06x} is not 32-bit x86"));
        }
        let magic = u16_at(data, opt)?;
        if magic != 0x10b {
            return Err(format!("optional header magic {magic:#06x} is not PE32"));
        }
        if opt_size < 96 {
            return Err("optional header too small".into());
        }
        let entry_rva = u32_at(data, opt + 16)?;
        let image_base = u32_at(data, opt + 28)?;
        let size_of_image = u32_at(data, opt + 56)?;
        let size_of_headers = u32_at(data, opt + 60)?;
        let dll_characteristics = u16_at(data, opt + 70)?;
        let dir_count = u32_at(data, opt + 92)? as usize;
        let import_dir = if dir_count > 1 && opt_size >= 96 + 16 {
            (u32_at(data, opt + 96 + 8)?, u32_at(data, opt + 96 + 12)?)
        } else {
            (0, 0)
        };
        // Data directory 9 is the TLS directory.
        let tls_dir_entry = (dir_count > TLS_DIRECTORY && opt_size >= 96 + (TLS_DIRECTORY + 1) * 8)
            .then_some(opt + 96 + TLS_DIRECTORY * 8);
        let tls_dir = match tls_dir_entry {
            Some(at) => (u32_at(data, at)?, u32_at(data, at + 4)?),
            None => (0, 0),
        };
        if size_of_image == 0 || size_of_image > MAX_IMAGE_SIZE {
            return Err(format!("implausible SizeOfImage {size_of_image:#x}"));
        }
        if size_of_headers as usize > data.len() || size_of_headers > size_of_image {
            return Err("SizeOfHeaders exceeds the file or the image".into());
        }
        let table = opt + opt_size;
        let mut sections = Vec::with_capacity(section_count);
        for i in 0..section_count {
            let at = table + i * 40;
            let raw = data
                .get(at..at + 40)
                .ok_or("section table runs past the end of the file")?;
            let name_end = raw[..8].iter().position(|&b| b == 0).unwrap_or(8);
            let s = Section {
                name: String::from_utf8_lossy(&raw[..name_end]).into_owned(),
                virtual_size: u32_at(raw, 8)?,
                virtual_address: u32_at(raw, 12)?,
                raw_size: u32_at(raw, 16)?,
                raw_offset: u32_at(raw, 20)?,
                characteristics: u32_at(raw, 36)?,
            };
            let extent = u64::from(s.virtual_address) + u64::from(s.virtual_size.max(s.raw_size));
            if extent > u64::from(size_of_image) {
                return Err(format!("section {} extends past SizeOfImage", s.name));
            }
            if s.raw_size > 0 && u64::from(s.raw_offset) + u64::from(s.raw_size) > data.len() as u64
            {
                return Err(format!(
                    "section {} raw data runs past the end of the file",
                    s.name
                ));
            }
            sections.push(s);
        }
        Ok(Pe {
            machine,
            characteristics,
            dll_characteristics,
            image_base,
            size_of_image,
            size_of_headers,
            entry_rva,
            sections,
            import_dir,
            tls_dir,
            tls_dir_entry,
        })
    }

    /// File offset of an RVA, if the RVA is backed by file data.
    pub fn rva_to_offset(&self, rva: u32) -> Option<usize> {
        if rva < self.size_of_headers {
            return Some(rva as usize);
        }
        self.sections.iter().find_map(|s| {
            let delta = rva.checked_sub(s.virtual_address)?;
            (delta < s.raw_size).then(|| s.raw_offset as usize + delta as usize)
        })
    }

    fn cstr(&self, data: &[u8], rva: u32) -> Result<String, String> {
        let off = self
            .rva_to_offset(rva)
            .ok_or_else(|| format!("RVA {rva:#x} is not backed by the file"))?;
        let tail = data
            .get(off..)
            .ok_or("string offset past the end of the file")?;
        let end = tail
            .iter()
            .position(|&b| b == 0)
            .ok_or("unterminated string")?;
        if end > 512 {
            return Err("import name too long".into());
        }
        Ok(String::from_utf8_lossy(&tail[..end]).into_owned())
    }

    /// Read the import directory: every imported symbol with the RVA of its
    /// address-table slot. Delay-load and bound imports are not read.
    pub fn imports(&self, data: &[u8]) -> Result<Vec<Import>, String> {
        let mut out = Vec::new();
        if self.import_dir.0 == 0 {
            return Ok(out);
        }
        let mut desc_rva = self.import_dir.0;
        for _ in 0..4096 {
            let off = self
                .rva_to_offset(desc_rva)
                .ok_or("import descriptor outside the file data")?;
            let d = data
                .get(off..off + 20)
                .ok_or("import descriptor runs past the end of the file")?;
            let (oft, name_rva, ft) = (u32_at(d, 0)?, u32_at(d, 12)?, u32_at(d, 16)?);
            if oft == 0 && name_rva == 0 && ft == 0 {
                return Ok(out);
            }
            let dll = self.cstr(data, name_rva)?;
            let lookup = if oft != 0 { oft } else { ft };
            for i in 0u32.. {
                if out.len() >= MAX_IMPORTS {
                    return Err("too many imports".into());
                }
                let entry_off = self
                    .rva_to_offset(lookup + i * 4)
                    .ok_or("import lookup table outside the file data")?;
                let entry = u32_at(data, entry_off)?;
                if entry == 0 {
                    break;
                }
                let name = if entry & 0x8000_0000 != 0 {
                    ImportName::Ordinal((entry & 0xffff) as u16)
                } else {
                    ImportName::Name(self.cstr(data, entry + 2)?)
                };
                out.push(Import {
                    dll: dll.clone(),
                    name,
                    iat_rva: ft + i * 4,
                });
            }
            desc_rva += 20;
        }
        Err("import directory has too many descriptors".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put32(v: &mut [u8], off: usize, x: u32) {
        v[off..off + 4].copy_from_slice(&x.to_le_bytes());
    }

    /// A minimal PE32 built from scratch: headers, a .text and an .idata
    /// section, importing `GetTickCount` and ordinal 7 from KERNEL32.dll and
    /// `puts` from msvcrt.dll.
    fn build() -> Vec<u8> {
        let mut f = vec![0u8; 0x600];
        f[..2].copy_from_slice(b"MZ");
        put32(&mut f, 0x3c, 0x80);
        let pe = 0x80;
        f[pe..pe + 4].copy_from_slice(b"PE\0\0");
        let coff = pe + 4;
        f[coff..coff + 2].copy_from_slice(&0x14cu16.to_le_bytes());
        f[coff + 2..coff + 4].copy_from_slice(&2u16.to_le_bytes());
        f[coff + 16..coff + 18].copy_from_slice(&224u16.to_le_bytes());
        f[coff + 18..coff + 20].copy_from_slice(&0x0102u16.to_le_bytes());
        let opt = coff + 20;
        f[opt..opt + 2].copy_from_slice(&0x10bu16.to_le_bytes());
        put32(&mut f, opt + 16, 0x1000);
        put32(&mut f, opt + 28, 0x0040_0000);
        put32(&mut f, opt + 56, 0x3000);
        put32(&mut f, opt + 60, 0x200);
        put32(&mut f, opt + 92, 16);
        put32(&mut f, opt + 96 + 8, 0x2000); // import dir RVA
        put32(&mut f, opt + 96 + 12, 60);
        let table = opt + 224;
        // .text: VA 0x1000, raw 0x200..0x400
        f[table..table + 5].copy_from_slice(b".text");
        put32(&mut f, table + 8, 0x100);
        put32(&mut f, table + 12, 0x1000);
        put32(&mut f, table + 16, 0x200);
        put32(&mut f, table + 20, 0x200);
        put32(&mut f, table + 36, 0x6000_0020);
        // .idata: VA 0x2000, raw 0x400..0x600
        let t2 = table + 40;
        f[t2..t2 + 6].copy_from_slice(b".idata");
        put32(&mut f, t2 + 8, 0x200);
        put32(&mut f, t2 + 12, 0x2000);
        put32(&mut f, t2 + 16, 0x200);
        put32(&mut f, t2 + 20, 0x400);
        put32(&mut f, t2 + 36, 0xC000_0040);
        // Layout inside .idata (RVA = 0x2000 + n, file = 0x400 + n):
        let d = 0x400;
        // descriptor 0: KERNEL32.dll, ILT at +0x40, IAT at +0x60
        put32(&mut f, d, 0x2040);
        put32(&mut f, d + 12, 0x2080);
        put32(&mut f, d + 16, 0x2060);
        // descriptor 1: msvcrt.dll, ILT at +0x50, IAT at +0x70 (OFT zero uses IAT)
        put32(&mut f, d + 20, 0);
        put32(&mut f, d + 20 + 12, 0x2090);
        put32(&mut f, d + 20 + 16, 0x2070);
        // ILT of KERNEL32: name entry, ordinal 7, terminator
        put32(&mut f, d + 0x40, 0x20a0);
        put32(&mut f, d + 0x44, 0x8000_0007);
        // IAT of KERNEL32 mirrors the ILT.
        put32(&mut f, d + 0x60, 0x20a0);
        put32(&mut f, d + 0x64, 0x8000_0007);
        // msvcrt uses the IAT as its lookup table.
        put32(&mut f, d + 0x70, 0x20b0);
        f[d + 0x80..d + 0x80 + 13].copy_from_slice(b"KERNEL32.dll\0");
        f[d + 0x90..d + 0x90 + 11].copy_from_slice(b"msvcrt.dll\0");
        f[d + 0xa2..d + 0xa2 + 13].copy_from_slice(b"GetTickCount\0");
        f[d + 0xb2..d + 0xb2 + 5].copy_from_slice(b"puts\0");
        f
    }

    #[test]
    fn parses_headers_and_sections() {
        let data = build();
        let pe = Pe::parse(&data).unwrap();
        assert_eq!(pe.image_base, 0x0040_0000);
        assert_eq!(pe.size_of_image, 0x3000);
        assert_eq!(pe.entry_rva, 0x1000);
        assert_eq!(pe.sections.len(), 2);
        assert_eq!(pe.sections[0].name, ".text");
        assert_eq!(pe.sections[1].virtual_address, 0x2000);
        assert_eq!(pe.rva_to_offset(0x10), Some(0x10));
        assert_eq!(pe.rva_to_offset(0x1010), Some(0x210));
        assert_eq!(pe.rva_to_offset(0x2005), Some(0x405));
        assert_eq!(pe.rva_to_offset(0x2800), None);
        assert_eq!(pe.rva_to_offset(0x0f00), None);
    }

    #[test]
    fn finds_the_tls_directory_entry() {
        let mut data = build();
        let pe = Pe::parse(&data).unwrap();
        assert_eq!(pe.tls_dir, (0, 0));
        // 0x80 + 24 is the optional header, the table starts 96 bytes in.
        assert_eq!(pe.tls_dir_entry, Some(0x80 + 24 + 96 + 9 * 8));
        put32(&mut data, 0x80 + 24 + 96 + 9 * 8, 0x2100);
        put32(&mut data, 0x80 + 24 + 96 + 9 * 8 + 4, 0x18);
        assert_eq!(Pe::parse(&data).unwrap().tls_dir, (0x2100, 0x18));
        // A table with fewer than ten entries has none.
        let mut short = data.clone();
        put32(&mut short, 0x80 + 24 + 92, 9);
        let pe = Pe::parse(&short).unwrap();
        assert_eq!((pe.tls_dir, pe.tls_dir_entry), ((0, 0), None));
    }

    #[test]
    fn reads_imports() {
        let data = build();
        let pe = Pe::parse(&data).unwrap();
        let imports = pe.imports(&data).unwrap();
        assert_eq!(imports.len(), 3);
        assert_eq!(imports[0].dll, "KERNEL32.dll");
        assert_eq!(imports[0].name, ImportName::Name("GetTickCount".into()));
        assert_eq!(imports[0].iat_rva, 0x2060);
        assert_eq!(imports[1].name, ImportName::Ordinal(7));
        assert_eq!(imports[1].iat_rva, 0x2064);
        assert_eq!(imports[2].dll, "msvcrt.dll");
        assert_eq!(imports[2].name, ImportName::Name("puts".into()));
        assert_eq!(imports[2].iat_rva, 0x2070);
        assert_eq!(imports[0].to_string(), "KERNEL32.dll!GetTickCount");
        assert_eq!(imports[1].to_string(), "KERNEL32.dll!#7");
    }

    #[test]
    fn rejects_bad_files() {
        let good = build();
        assert!(Pe::parse(b"").is_err());
        assert!(Pe::parse(b"MZ").is_err());
        let mut d = good.clone();
        d[0] = b'X';
        assert!(Pe::parse(&d).unwrap_err().contains("MZ"));
        let mut d = good.clone();
        d[0x80] = b'X';
        assert!(Pe::parse(&d).unwrap_err().contains("signature"));
        let mut d = good.clone();
        d[0x84] = 0x64; // machine 0x8664
        d[0x85] = 0x86;
        assert!(Pe::parse(&d).unwrap_err().contains("32-bit"));
        let mut d = good.clone();
        d[0x98] = 0x0b;
        d[0x99] = 0x02; // PE32+
        assert!(Pe::parse(&d).unwrap_err().contains("PE32"));
        // Truncated inside the section table and inside section data.
        assert!(Pe::parse(&good[..0x100]).is_err());
        assert!(Pe::parse(&good[..0x500]).is_err());
        // SizeOfImage too small for the sections.
        let mut d = good.clone();
        put32(&mut d, 0x98 + 56, 0x1800);
        assert!(Pe::parse(&d).unwrap_err().contains("SizeOfImage"));
    }

    #[test]
    fn malformed_imports_are_errors_not_panics() {
        let mut data = build();
        let pe = Pe::parse(&data).unwrap();
        // Point the first name entry outside the file.
        put32(&mut data, 0x400 + 0x40, 0x9000);
        assert!(pe.imports(&data).is_err());
        // A descriptor table that never terminates.
        let mut data = build();
        let mut pe = Pe::parse(&data).unwrap();
        pe.import_dir.0 = 0x2100;
        for i in 0..(0x100 / 4) {
            put32(&mut data, 0x400 + 0x100 + i * 4, 0x2040);
        }
        assert!(pe.imports(&data).is_err());
    }

    #[test]
    fn no_import_directory_is_empty() {
        let data = build();
        let mut pe = Pe::parse(&data).unwrap();
        pe.import_dir = (0, 0);
        assert!(pe.imports(&data).unwrap().is_empty());
    }
}
