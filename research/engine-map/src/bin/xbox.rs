//! Xbox 360 prototype map: functions, source modules, vtables and call order
//! from an Xbox build's PDB and its PowerPC PE image (policy: ADR-0002).
//!
//! ```text
//! xbox-map <build.pdb> <build.exe> <out-dir>
//! ```
//!
//! Writes three tab-separated files to `<out-dir>` (private research tree;
//! they hold names and addresses of the prototype, never commit them):
//!
//! - `xb_modules.tsv`: `module, obj, lib, source`. `lib` is the file name of
//!   the static library the object came from, or `exe` for the game's own
//!   objects; `source` is the module's main source file (the `.cpp`/`.c`
//!   whose stem equals the object's, else its first `.cpp`/`.c`).
//! - `xb_funcs.tsv`: `address, size, module, name, signature, calls`. One
//!   row per procedure record (`S_GPROC32`/`S_LPROC32`), sorted by address.
//!   `signature` is the demangled public name at that address, if any.
//!   `calls` lists the targets of every `bl` in the body, and of every `b`
//!   that leaves the body for the start of another procedure (a tail call),
//!   in instruction order.
//! - `xb_vtables.tsv`: `address, mangled, class, slots`. One row per public
//!   `??_7...` (`vftable`) symbol; `class` is its demangled name and the
//!   slots run while they point at a procedure start, up to the next public
//!   symbol.
//!
//! The PE headers are little-endian, the image contents big-endian.

#[path = "../msf.rs"]
mod msf;

use pdb::FallibleIterator;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

type R<T> = Result<T, Box<dyn std::error::Error>>;

struct Image {
    base: u32,
    bytes: Vec<u8>,
    /// (virtual address, virtual size, raw offset, raw size, executable)
    sections: Vec<(u32, u32, u32, u32, bool)>,
}

impl Image {
    fn parse(bytes: Vec<u8>) -> R<Image> {
        let le16 = |o: usize| u16::from_le_bytes([bytes[o], bytes[o + 1]]) as usize;
        let le32 = |o: usize| u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
        let pe = le32(0x3c) as usize;
        if &bytes[pe..pe + 4] != b"PE\0\0" {
            return Err("not a PE image".into());
        }
        let coff = pe + 4;
        let nsec = le16(coff + 2);
        let opt = coff + 20;
        let opt_size = le16(coff + 16);
        let base = le32(opt + 28);
        let mut sections = Vec::new();
        for i in 0..nsec {
            let s = opt + opt_size + i * 40;
            let chars = le32(s + 36);
            sections.push((
                base + le32(s + 12),
                le32(s + 8),
                le32(s + 20),
                le32(s + 16),
                chars & 0x2000_0000 != 0,
            ));
        }
        Ok(Image {
            base,
            bytes,
            sections,
        })
    }

    fn be32(&self, va: u32) -> Option<u32> {
        for &(sva, vsize, raw, rsize, _) in &self.sections {
            if va >= sva && va + 4 <= sva + vsize.max(rsize) {
                let off = va - sva;
                if off + 4 > rsize {
                    return Some(0);
                }
                let o = (raw + off) as usize;
                return Some(u32::from_be_bytes(self.bytes[o..o + 4].try_into().unwrap()));
            }
        }
        None
    }

    fn is_code(&self, va: u32) -> bool {
        self.sections
            .iter()
            .any(|&(sva, vsize, _, _, x)| x && va >= sva && va < sva + vsize)
    }
}

fn demangle(s: &str) -> String {
    msvc_demangler::demangle(s, msvc_demangler::DemangleFlags::llvm())
        .unwrap_or_else(|_| s.to_string())
}

fn clean(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == '\t' || c == '\n' || c == '\r' {
                ' '
            } else {
                c
            }
        })
        .collect()
}

fn stem(p: &str) -> String {
    let f = p.rsplit(['\\', '/']).next().unwrap_or(p);
    f.rsplit_once('.')
        .map(|(a, _)| a)
        .unwrap_or(f)
        .to_ascii_lowercase()
}

fn main() -> R<()> {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage: xbox-map <build.pdb> <build.exe> <out-dir>".into());
    }
    let out = Path::new(&a[3]);
    std::fs::create_dir_all(out)?;
    let img = Image::parse(std::fs::read(&a[2])?)?;
    let msf = msf::repack(&std::fs::read(&a[1])?)?;
    let mut pdb = pdb::PDB::open(std::io::Cursor::new(msf))?;
    let info = pdb.pdb_information()?;
    let pdb_name = Path::new(&a[1])
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    std::fs::write(
        out.join("xb_info.txt"),
        format!(
            "pdb\t{pdb_name}\nguid\t{}\nage\t{}\n",
            info.guid.to_string().to_uppercase(),
            info.age
        ),
    )?;
    let am = pdb.address_map().map_err(|e| format!("address_map: {e}"))?;
    let strings = pdb
        .string_table()
        .map_err(|e| format!("string_table: {e}"))?;
    let va = |rva: u32| img.base + rva;

    // Modules and their procedures.
    let mut modules = Vec::new();
    let mut bad_lines = 0usize;
    let mut procs: BTreeMap<u32, (u32, usize, String)> = BTreeMap::new();
    let di = pdb.debug_information()?;
    let mut it = di.modules()?;
    while let Some(m) = it.next()? {
        let idx = modules.len();
        let obj = m.module_name().to_string();
        let libf = m.object_file_name().to_string();
        let lib = if libf.to_ascii_lowercase().ends_with(".lib") {
            libf.rsplit(['\\', '/']).next().unwrap_or(&libf).to_string()
        } else {
            "exe".to_string()
        };
        let mut source = String::new();
        if let Some(mi) = pdb.module_info(&m)? {
            let want = stem(&obj);
            let mut first = String::new();
            // A few library modules carry truncated line tables; they only
            // lose their source file, never their procedures.
            let files: Vec<String> = match mi.line_program() {
                Ok(lp) => {
                    let mut v = Vec::new();
                    let mut fs = lp.files();
                    loop {
                        match fs.next() {
                            Ok(Some(f)) => match f.name.to_string_lossy(&strings) {
                                Ok(n) => v.push(n.to_string()),
                                Err(_) => bad_lines += 1,
                            },
                            Ok(None) => break,
                            Err(_) => {
                                bad_lines += 1;
                                break;
                            }
                        }
                    }
                    v
                }
                Err(_) => {
                    bad_lines += 1;
                    Vec::new()
                }
            };
            for n in files {
                let l = n.to_ascii_lowercase();
                if !(l.ends_with(".cpp") || l.ends_with(".c") || l.ends_with(".cc")) {
                    continue;
                }
                if stem(&n) == want {
                    source = n;
                    break;
                }
                if first.is_empty() {
                    first = n;
                }
            }
            if source.is_empty() {
                source = first;
            }
            let mut syms = mi.symbols().map_err(|e| format!("symbols {obj}: {e}"))?;
            while let Some(s) = syms.next().map_err(|e| format!("symbols {obj}: {e}"))? {
                if let Ok(pdb::SymbolData::Procedure(p)) = s.parse() {
                    if let Some(rva) = p.offset.to_rva(&am) {
                        procs.entry(va(rva.0)).or_insert((
                            p.len,
                            idx,
                            p.name.to_string().to_string(),
                        ));
                    }
                }
            }
        }
        modules.push((obj, lib, source));
    }

    // Publics: signatures and vtables.
    let mut publics: BTreeMap<u32, String> = BTreeMap::new();
    let gs = pdb.global_symbols().map_err(|e| format!("globals: {e}"))?;
    let mut it = gs.iter();
    while let Some(s) = it.next()? {
        if let Ok(pdb::SymbolData::Public(p)) = s.parse() {
            if let Some(rva) = p.offset.to_rva(&am) {
                publics
                    .entry(va(rva.0))
                    .or_insert_with(|| p.name.to_string().to_string());
            }
        }
    }

    // Section contributions: which module's object supplied each range.
    let mut contribs: BTreeMap<u32, (u32, usize)> = BTreeMap::new();
    let mut sc = di.section_contributions()?;
    while let Some(c) = sc.next()? {
        if let Some(rva) = c.offset.to_rva(&am) {
            contribs.insert(va(rva.0), (c.size, c.module));
        }
    }
    let contrib_at = |a: u32| {
        contribs
            .range(..=a)
            .next_back()
            .filter(|(&s, &(n, _))| a < s + n)
            .map(|(&s, &(n, m))| (s + n, m))
    };

    // Code publics without a procedure record (about 4,800 in MemDebug,
    // e.g. `bhkWorld::Update`) become functions too: they run to the next
    // procedure or code public, within their contribution. Their size is
    // therefore an upper bound; `source` says which kind a row is.
    let mut kind: BTreeMap<u32, &str> = procs.keys().map(|&a| (a, "proc")).collect();
    let code_syms: BTreeSet<u32> = procs
        .keys()
        .copied()
        .chain(publics.keys().copied().filter(|&a| img.is_code(a)))
        .collect();
    for (&addr, name) in &publics {
        if !img.is_code(addr)
            || procs.contains_key(&addr)
            || name.starts_with("??_") && !is_fn_special(name)
        {
            continue;
        }
        let Some((cend, m)) = contrib_at(addr) else {
            continue;
        };
        let next = code_syms
            .range(addr + 1..)
            .next()
            .copied()
            .unwrap_or(u32::MAX);
        let short = msvc_demangler::demangle(name, msvc_demangler::DemangleFlags::NAME_ONLY)
            .unwrap_or_else(|_| name.clone());
        procs.insert(addr, (next.min(cend) - addr, m, short));
        kind.insert(addr, "public");
    }
    let starts: BTreeSet<u32> = procs.keys().copied().collect();

    let mut w = BufWriter::new(File::create(out.join("xb_modules.tsv"))?);
    writeln!(w, "module\tobj\tlib\tsource")?;
    for (i, (o, l, s)) in modules.iter().enumerate() {
        writeln!(w, "{i}\t{}\t{}\t{}", clean(o), clean(l), clean(s))?;
    }

    let mut w = BufWriter::new(File::create(out.join("xb_funcs.tsv"))?);
    writeln!(
        w,
        "address\tsize\tmodule\tsource\tname\tsignature\tcalls\tstrings"
    )?;
    for (&addr, (len, m, name)) in &procs {
        let mut calls = Vec::new();
        let mut strs = Vec::new();
        let mut hi = [None::<u32>; 32];
        let end = addr + len;
        let mut pc = addr;
        while pc < end {
            if let Some(ins) = img.be32(pc) {
                if ins >> 26 == 18 {
                    let mut li = (ins & 0x03ff_fffc) as i32;
                    if li & 0x0200_0000 != 0 {
                        li -= 0x0400_0000;
                    }
                    let t = if ins & 2 != 0 {
                        li as u32
                    } else {
                        pc.wrapping_add(li as u32)
                    };
                    let link = ins & 1 != 0;
                    if link || ((t < addr || t >= end) && starts.contains(&t)) {
                        calls.push(format!("{t:08x}"));
                    }
                }
                if let Some(a) = data_ref(ins, &mut hi) {
                    if let Some(s) = img.ascii_z(a) {
                        strs.push(clean_str(&s));
                    }
                }
            }
            pc += 4;
        }
        let sig = publics.get(&addr).map(|p| demangle(p)).unwrap_or_default();
        writeln!(
            w,
            "{addr:08x}\t{len}\t{m}\t{}\t{}\t{}\t{}\t{}",
            kind[&addr],
            clean(name),
            clean(&sig),
            calls.join(","),
            strs.join("|")
        )?;
    }

    let mut w = BufWriter::new(File::create(out.join("xb_vtables.tsv"))?);
    writeln!(w, "address\tmangled\tclass\tslots")?;
    let mut nvt = 0;
    for (&addr, name) in &publics {
        if !name.starts_with("??_7") {
            continue;
        }
        let next = publics
            .range(addr + 1..)
            .next()
            .map(|(&k, _)| k)
            .unwrap_or(u32::MAX);
        let mut slots = Vec::new();
        let mut p = addr;
        while p < next {
            match img.be32(p) {
                Some(t) if img.is_code(t) => slots.push(format!("{t:08x}")),
                _ => break,
            }
            p += 4;
        }
        writeln!(
            w,
            "{addr:08x}	{}	{}	{}",
            clean(name),
            clean(&demangle(name)),
            slots.join(",")
        )?;
        nvt += 1;
    }
    eprintln!(
        "image base {:08x}, {} modules ({} line-table read errors), {} procedures, {} publics, {} vtables",
        img.base,
        modules.len(),
        bad_lines,
        procs.len(),
        publics.len(),
        nvt
    );
    Ok(())
}

/// `??_` names that are functions: constructors, destructors, operators
/// and the compiler's deleting destructors. Data (`??_7` vftables, `??_R`
/// RTTI, `??_C` string literals, `??_8` vbtables) is not.
fn is_fn_special(name: &str) -> bool {
    let k = &name[3..4];
    !matches!(k, "7" | "8" | "R" | "C")
}

/// Address formed by `lis rX, hi` and a later instruction that adds a low
/// half to rX: `addi`, `ori`, or a D-form load or store (`lwz`, `lfs`,
/// `stw`, ...). `hi` holds the last `lis` value per register; it is
/// updated here. Instruction order is used, not control flow, so an address
/// is only a candidate; callers accept it only if a string is there.
fn data_ref(ins: u32, hi: &mut [Option<u32>; 32]) -> Option<u32> {
    let op = ins >> 26;
    let rd = ((ins >> 21) & 31) as usize;
    let ra = ((ins >> 16) & 31) as usize;
    let imm = ins & 0xffff;
    let simm = imm as u16 as i16 as i32 as u32;
    match op {
        15 if ra == 0 => {
            hi[rd] = Some(imm << 16);
            None
        }
        14 if ra != 0 => hi[ra].map(|h| h.wrapping_add(simm)),
        // ori rA, rS, UIMM: the source register is in the rD field.
        24 => hi[rd].map(|h| h | imm),
        32..=55 if ra != 0 => hi[ra].map(|h| h.wrapping_add(simm)),
        _ => None,
    }
}

impl Image {
    /// The same rule as `NvEngineMap.java`: a NUL-terminated printable
    /// ASCII string (tab, newline and carriage return allowed) of 4 to
    /// 1024 characters in non-executable data.
    fn ascii_z(&self, va: u32) -> Option<String> {
        for &(sva, vsize, raw, rsize, x) in &self.sections {
            if x || va < sva || va >= sva + vsize {
                continue;
            }
            let off = va - sva;
            if off >= rsize {
                return None;
            }
            let start = (raw + off) as usize;
            let end = (raw + rsize) as usize;
            let mut s = String::new();
            for &c in &self.bytes[start..end.min(start + 1025)] {
                if c == 0 {
                    return (s.len() >= 4).then_some(s);
                }
                if !((0x20..=0x7e).contains(&c) || c == 9 || c == 10 || c == 13) {
                    return None;
                }
                s.push(c as char);
            }
            return None;
        }
        None
    }
}

/// `NvEngineMap.java`'s `clean`: at most 60 characters, control
/// characters, DEL, ',' and '|' replaced by '_'.
fn clean_str(s: &str) -> String {
    s.chars()
        .take(60)
        .map(|c| {
            if (c as u32) < 0x20 || c == '|' || c == ',' || c == '\x7f' {
                '_'
            } else {
                c
            }
        })
        .collect()
}
