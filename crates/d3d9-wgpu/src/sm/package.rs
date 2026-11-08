//! Shader packages (`Data\Shaders\shaderpackageNNN.sdp`): named Direct3D 9 shaders, one after
//! another. Read from the player's install; nothing from them is committed.

use super::Error;

/// Every shader in a package, in file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub shaders: Vec<Shader>,
}

/// One compiled shader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shader {
    /// The name the game looks it up by, e.g. `SLS2001.vso`.
    pub name: String,
    /// Direct3D 9 bytecode: a version token, instructions, the end token.
    pub bytecode: Vec<u8>,
}

/// Bytes reserved for each shader's name in the game's layout.
const NAME_LEN: usize = 256;

impl Package {
    /// Reads a package: a 12-byte header (the shader count at +4), then each shader's 256-byte
    /// name, its size and its bytecode.
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let count = u32_at(bytes, 4)
            .ok_or_else(|| Error::NotAPackage("shorter than its header".into()))?
            as usize;
        if count == 0 || count > 100_000 {
            return Err(Error::NotAPackage(format!("{count} shaders")));
        }
        let mut at = 12;
        let mut shaders = Vec::with_capacity(count);
        for k in 0..count {
            let cut = || Error::NotAPackage(format!("cut short in shader {k}"));
            let name = bytes.get(at..at + NAME_LEN).ok_or_else(cut)?;
            let size = u32_at(bytes, at + NAME_LEN).ok_or_else(cut)? as usize;
            let start = at + NAME_LEN + 4;
            let code = bytes
                .get(start..start.checked_add(size).ok_or_else(cut)?)
                .ok_or_else(cut)?;
            if !plausible(code) {
                return Err(Error::NotAPackage(format!(
                    "shader {k} is not Direct3D 9 bytecode"
                )));
            }
            shaders.push(Shader {
                name: c_string(name),
                bytecode: code.to_vec(),
            });
            at = start + size;
        }
        Ok(Self { shaders })
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

/// Starts with a version token and ends with the end token.
fn plausible(code: &[u8]) -> bool {
    code.len() >= 8
        && code.len() % 4 == 0
        && u32_at(code, 0).is_some_and(super::is_version)
        && u32_at(code, code.len() - 4) == Some(super::END)
}

fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(shaders: &[(&str, &[u32])]) -> Vec<u8> {
        let mut out = vec![0u8; 12];
        out[4..8].copy_from_slice(&(shaders.len() as u32).to_le_bytes());
        for (name, tokens) in shaders {
            let mut n = [0u8; NAME_LEN];
            n[..name.len()].copy_from_slice(name.as_bytes());
            out.extend_from_slice(&n);
            out.extend_from_slice(&((tokens.len() * 4) as u32).to_le_bytes());
            for t in *tokens {
                out.extend_from_slice(&t.to_le_bytes());
            }
        }
        out
    }

    #[test]
    fn a_package_gives_each_shader_by_name() {
        let vs = [0xFFFE_0200, super::super::END];
        let ps = [0xFFFF_0300, super::super::END];
        let p = Package::parse(&package(&[("A.vso", &vs), ("B.pso", &ps)])).unwrap();
        assert_eq!(p.shaders.len(), 2);
        assert_eq!(p.get("b.PSO").unwrap().bytecode.len(), 8);
    }

    #[test]
    fn a_cut_package_is_refused() {
        let vs = [0xFFFE_0200, super::super::END];
        let mut bytes = package(&[("A.vso", &vs)]);
        bytes.truncate(bytes.len() - 2);
        assert!(Package::parse(&bytes).is_err());
    }
}
