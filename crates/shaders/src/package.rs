//! Shader packages (`.sdp`): named Direct3D 9 shaders, one after another.

use crate::Error;

/// Every shader in a package, in file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub shaders: Vec<Shader>,
    /// How the file was read, for messages.
    pub layout: &'static str,
}

/// One compiled shader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shader {
    /// The name the game looks it up by, e.g. `SLS2001.vso`.
    pub name: String,
    /// Direct3D 9 bytecode: a version token, instructions, the end token.
    pub bytecode: Vec<u8>,
    /// Where the bytecode starts in the file.
    pub offset: usize,
}

/// Bytes reserved for each shader's name in the game's layout.
const NAME_LEN: usize = 256;
/// The token every Direct3D 9 shader ends with.
const END: u32 = 0x0000_FFFF;

impl Package {
    /// Reads a package. The game's layout (a 12-byte header, then each
    /// shader's 256-byte name, size and bytecode) is tried first; anything
    /// else is searched for shaders by their version and end tokens.
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if let Some(shaders) = parse_indexed(bytes) {
            return Ok(Self {
                shaders,
                layout: "header, then a name, size and bytecode for each shader",
            });
        }
        let shaders = scan(bytes);
        if shaders.is_empty() {
            return Err(Error::NotAPackage(
                "no Direct3D 9 shaders found in it".into(),
            ));
        }
        Ok(Self {
            shaders,
            layout: "shaders found by searching the file",
        })
    }

    /// A shader by name, ignoring case.
    pub fn get(&self, name: &str) -> Option<&Shader> {
        self.shaders
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(name))
    }
}

fn u32_at(bytes: &[u8], at: usize) -> Option<u32> {
    let b = bytes.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// A vertex (`0xFFFE`) or pixel (`0xFFFF`) shader version token, model 1-3.
pub(crate) fn is_version(token: u32) -> bool {
    let kind = token >> 16;
    let major = (token >> 8) & 0xFF;
    (kind == 0xFFFF || kind == 0xFFFE) && (1..=3).contains(&major)
}

/// Starts with a version token and ends with the end token.
fn plausible(code: &[u8]) -> bool {
    code.len() >= 8
        && code.len() % 4 == 0
        && u32_at(code, 0).is_some_and(is_version)
        && u32_at(code, code.len() - 4) == Some(END)
}

fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

fn parse_indexed(bytes: &[u8]) -> Option<Vec<Shader>> {
    let count = u32_at(bytes, 4)? as usize;
    if count == 0 || count > 100_000 {
        return None;
    }
    let mut at = 12;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let name = bytes.get(at..at + NAME_LEN)?;
        let size = u32_at(bytes, at + NAME_LEN)? as usize;
        let start = at + NAME_LEN + 4;
        let code = bytes.get(start..start.checked_add(size)?)?;
        if !plausible(code) {
            return None;
        }
        out.push(Shader {
            name: c_string(name),
            bytecode: code.to_vec(),
            offset: start,
        });
        at = start + size;
    }
    Some(out)
}

fn scan(bytes: &[u8]) -> Vec<Shader> {
    let mut out = Vec::new();
    let mut at = 4;
    while at + 8 <= bytes.len() {
        if u32_at(bytes, at).is_some_and(is_version) {
            let size = u32_at(bytes, at - 4).unwrap_or(0) as usize;
            if let Some(code) = at.checked_add(size).and_then(|end| bytes.get(at..end)) {
                if plausible(code) {
                    out.push(Shader {
                        name: name_before(bytes, at - 4),
                        bytecode: code.to_vec(),
                        offset: at,
                    });
                    at += size;
                    continue;
                }
            }
        }
        at += 1;
    }
    out
}

/// The printable text ending just before `end`, past any zero padding.
fn name_before(bytes: &[u8], end: usize) -> String {
    let mut stop = end;
    while stop > 0 && bytes[stop - 1] == 0 {
        stop -= 1;
    }
    let mut start = stop;
    while start > 0 && stop - start < NAME_LEN && (0x20..0x7F).contains(&bytes[start - 1]) {
        start -= 1;
    }
    String::from_utf8_lossy(&bytes[start..stop]).into_owned()
}
