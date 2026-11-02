//! What the engine crate says about each address: translation markers
//! (`// Translated from <addr>`, `// Translation incomplete: <addr>`) and the
//! typed signature of its `entry!` registration.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// A parameter or result type of a registered translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ty {
    U32,
    I32,
    U16,
    I16,
    U8,
    I8,
    Bool,
    F32,
    F64,
    U64,
    Ptr,
    Void,
}

impl Ty {
    pub fn parse(s: &str) -> Option<Ty> {
        let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        Some(match s.as_str() {
            "u32" => Ty::U32,
            "i32" => Ty::I32,
            "u16" => Ty::U16,
            "i16" => Ty::I16,
            "u8" => Ty::U8,
            "i8" => Ty::I8,
            "bool" => Ty::Bool,
            "f32" => Ty::F32,
            "f64" => Ty::F64,
            "u64" => Ty::U64,
            "()" | "" => Ty::Void,
            p if p == "Ptr" || p.starts_with("Ptr<") => Ty::Ptr,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Ty::U32 => "u32",
            Ty::I32 => "i32",
            Ty::U16 => "u16",
            Ty::I16 => "i16",
            Ty::U8 => "u8",
            Ty::I8 => "i8",
            Ty::Bool => "bool",
            Ty::F32 => "f32",
            Ty::F64 => "f64",
            Ty::U64 => "u64",
            Ty::Ptr => "ptr",
            Ty::Void => "void",
        }
    }

    pub fn from_name(s: &str) -> Option<Ty> {
        if s == "ptr" {
            return Some(Ty::Ptr);
        }
        if s == "void" {
            return Some(Ty::Void);
        }
        Ty::parse(s)
    }

    /// Argument words in the uniform form (engine `abi.rs`).
    pub fn words(self) -> usize {
        match self {
            Ty::F64 | Ty::U64 => 2,
            Ty::Void => 0,
            _ => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub params: Vec<Ty>,
    pub ret: Ty,
}

#[derive(Debug, Default)]
pub struct Source {
    /// Addresses on a `Translated from` marker, with the unit file.
    pub translated: BTreeMap<u32, PathBuf>,
    /// Addresses on a `Translation incomplete:` line.
    pub incomplete: BTreeSet<u32>,
    /// `entry!` registrations; `None` when a type is not one this check knows.
    pub signatures: BTreeMap<u32, Option<Signature>>,
}

const TEXT_START: u32 = 0x0040_1000;
const TEXT_END: u32 = 0x00fd_f000;

/// Every 8-hex-digit token on the line that lies in `.text`.
fn addresses(line: &str) -> Vec<u32> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 8 <= b.len() {
        let run = b[i..].iter().take_while(|c| c.is_ascii_hexdigit()).count();
        let bounded = i == 0 || !b[i - 1].is_ascii_alphanumeric();
        if run == 8 && bounded && (i + 8 == b.len() || !b[i + 8].is_ascii_alphanumeric()) {
            if let Ok(v) = u32::from_str_radix(&line[i..i + 8], 16) {
                if (TEXT_START..TEXT_END).contains(&v) {
                    out.push(v);
                }
            }
        }
        i += run.max(1);
    }
    out
}

/// Splits on commas that are not inside `<...>` or `(...)`.
fn split_top(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '<' | '(' => depth += 1,
            '>' | ')' => depth -= 1,
            ',' if depth == 0 => {
                out.push(std::mem::take(&mut cur));
                continue;
            }
            _ => {}
        }
        cur.push(c);
    }
    if !cur.trim().is_empty() {
        out.push(cur);
    }
    out
}

/// Parses the inside of `entry!( ... )`:
/// `0x00d87410, path::name(T1, T2) -> R`.
pub fn parse_entry(inner: &str) -> Option<(u32, Option<Signature>)> {
    let flat: String = inner.split_whitespace().collect::<Vec<_>>().join(" ");
    let (addr_text, rest) = flat.split_once(',')?;
    let addr_text = addr_text.trim().trim_start_matches("0x").replace('_', "");
    let addr = u32::from_str_radix(&addr_text, 16).ok()?;
    let open = rest.find('(')?;
    // the matching ')' of the parameter list
    let mut depth = 0i32;
    let mut close = None;
    for (i, c) in rest[open..].char_indices() {
        match c {
            '(' | '<' => depth += 1,
            ')' | '>' => {
                depth -= 1;
                if depth == 0 && c == ')' {
                    close = Some(open + i);
                    break;
                }
            }
            _ => {}
        }
    }
    let close = close?;
    let params_text = &rest[open + 1..close];
    let after = rest[close + 1..].trim();
    let ret_text = after.strip_prefix("->").map(str::trim).unwrap_or("");
    let mut params = Vec::new();
    for p in split_top(params_text) {
        match Ty::parse(&p) {
            Some(t) if t != Ty::Void => params.push(t),
            _ => return Some((addr, None)),
        }
    }
    let sig = Ty::parse(ret_text).map(|ret| Signature { params, ret });
    Some((addr, sig))
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// Scans the `src` of every engine crate: `<repo>/crates/engine` (the
/// facade) and `<repo>/crates/engine-*` (the core and the part crates).
pub fn scan(repo: &Path) -> Source {
    let mut files = Vec::new();
    if let Ok(rd) = fs::read_dir(repo.join("crates")) {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name == "engine" || name.starts_with("engine-") {
                rust_files(&e.path().join("src"), &mut files);
            }
        }
    }
    files.sort();
    let mut src = Source::default();
    for f in files {
        let Ok(text) = fs::read_to_string(&f) else {
            continue;
        };
        let rel = f.strip_prefix(repo).unwrap_or(&f).to_path_buf();
        let lines: Vec<&str> = text.lines().collect();
        for (i, l) in lines.iter().enumerate() {
            if l.contains("Translation incomplete:") {
                src.incomplete.extend(addresses(l));
            } else if l.contains("Translated from") {
                let mut here = addresses(l);
                if here.is_empty() && i > 0 {
                    here = addresses(lines[i - 1]);
                }
                for a in here {
                    src.translated.entry(a).or_insert_with(|| rel.clone());
                }
            }
        }
        let mut rest = text.as_str();
        while let Some(at) = rest.find("entry!(") {
            let body = &rest[at + "entry!(".len()..];
            let mut depth = 1i32;
            let mut end = None;
            for (i, c) in body.char_indices() {
                match c {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            let Some(end) = end else { break };
            if let Some((addr, sig)) = parse_entry(&body[..end]) {
                src.signatures.insert(addr, sig);
            }
            rest = &body[end..];
        }
    }
    src
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_forms() {
        let (a, s) = parse_entry("0x00d87410, get_iterator(Ptr<HkCachedHashMap>) -> i32").unwrap();
        assert_eq!(a, 0x00d8_7410);
        let s = s.unwrap();
        assert_eq!(s.params, vec![Ty::Ptr]);
        assert_eq!(s.ret, Ty::I32);

        let (a, s) =
            parse_entry("\n 0x0070_39b0,\n   m::f(u32, f64, Ptr<A<B, C>>, bool)\n").unwrap();
        assert_eq!(a, 0x0070_39b0);
        let s = s.unwrap();
        assert_eq!(s.params, vec![Ty::U32, Ty::F64, Ty::Ptr, Ty::Bool]);
        assert_eq!(s.ret, Ty::Void);

        let (_, s) = parse_entry("0x00401000, f(Array, u32)").unwrap();
        assert!(s.is_none(), "unknown parameter type");
    }

    #[test]
    fn marker_addresses() {
        assert_eq!(
            addresses(
                "// Translated from 00c95a80 and 00c90e60 (decompiled, FalloutNV.exe 1.4.0.525)"
            ),
            vec![0x00c9_5a80, 0x00c9_0e60]
        );
        // a 0x literal is a constant, not a marker citation; outside .text anyway
        assert!(addresses("const X: u32 = 0x0119b354;").is_empty());
        assert!(addresses("deadbeef0").is_empty());
    }
}
