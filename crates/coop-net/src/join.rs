//! Join codes: a copy/paste-able string naming where to connect plus the
//! 16-byte session token.
//!
//! Format: `NVRS-` then groups of five Crockford base32 characters separated
//! by dashes. The payload is `[version][kind][target...][token 16][crc16]`.
//! Parsing ignores case, dashes and spaces and accepts the usual Crockford
//! look-alikes (`O` for 0, `I` and `L` for 1).
//!
//! Kinds: 1 = Steam lobby (u64), 2 = IPv4 + port, 3 = IPv6 + port.
//! Kind 4 is reserved for a future iroh ticket (see the crate docs); a code of
//! an unknown kind parses to [`JoinError::UnsupportedKind`] so old builds fail
//! with a clear message instead of garbage.

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

use thiserror::Error;

use crate::endpoint::Token;
use crate::liveness::TOKEN_LEN;

const VERSION: u8 = 1;
const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Where to connect.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum JoinTarget {
    /// A Steam lobby id.
    SteamLobby(u64),
    /// A host's UDP address.
    Direct(SocketAddr),
}

/// A target plus the session token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinCode {
    /// Where to connect.
    pub target: JoinTarget,
    /// Session token the host checks at the handshake.
    pub token: Token,
}

/// Join code parse failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum JoinError {
    /// Missing `NVRS-` prefix.
    #[error("not an nv-rs join code")]
    BadPrefix,
    /// A character outside the alphabet.
    #[error("invalid character {0:?}")]
    BadChar(char),
    /// Too short or truncated.
    #[error("join code is truncated")]
    Truncated,
    /// Checksum mismatch (typo).
    #[error("join code checksum mismatch, check for typos")]
    BadChecksum,
    /// Version from a newer build.
    #[error("join code version {0} is not supported")]
    UnsupportedVersion(u8),
    /// Target kind this build cannot use.
    #[error("join code target kind {0} is not supported by this build")]
    UnsupportedKind(u8),
}

fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &b in data {
        crc ^= (b as u16) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ 0x1021
            } else {
                crc << 1
            };
        }
    }
    crc
}

fn b32_encode(data: &[u8]) -> String {
    let mut out = String::new();
    let (mut acc, mut bits) = (0u32, 0u32);
    for &b in data {
        acc = (acc << 8) | b as u32;
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(ALPHABET[((acc >> bits) & 31) as usize] as char);
        }
        acc &= (1 << bits) - 1;
    }
    if bits > 0 {
        out.push(ALPHABET[((acc << (5 - bits)) & 31) as usize] as char);
    }
    out
}

fn b32_decode(s: &str) -> Result<Vec<u8>, JoinError> {
    let mut out = Vec::new();
    let (mut acc, mut bits) = (0u32, 0u32);
    for c in s.chars() {
        let c = match c.to_ascii_uppercase() {
            'O' => '0',
            'I' | 'L' => '1',
            x => x,
        };
        let v = ALPHABET
            .iter()
            .position(|&a| a as char == c)
            .ok_or(JoinError::BadChar(c))? as u32;
        acc = (acc << 5) | v;
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

impl JoinCode {
    /// Builds a code.
    pub fn new(target: JoinTarget, token: Token) -> Self {
        JoinCode { target, token }
    }

    fn payload(&self) -> Vec<u8> {
        let mut p = vec![VERSION];
        match &self.target {
            JoinTarget::SteamLobby(id) => {
                p.push(1);
                p.extend_from_slice(&id.to_le_bytes());
            }
            JoinTarget::Direct(SocketAddr::V4(a)) => {
                p.push(2);
                p.extend_from_slice(&a.ip().octets());
                p.extend_from_slice(&a.port().to_le_bytes());
            }
            JoinTarget::Direct(SocketAddr::V6(a)) => {
                p.push(3);
                p.extend_from_slice(&a.ip().octets());
                p.extend_from_slice(&a.port().to_le_bytes());
            }
        }
        p.extend_from_slice(&self.token);
        let c = crc16(&p);
        p.extend_from_slice(&c.to_be_bytes());
        p
    }
}

impl fmt::Display for JoinCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = b32_encode(&self.payload());
        f.write_str("NVRS")?;
        for chunk in s.as_bytes().chunks(5) {
            f.write_str("-")?;
            f.write_str(std::str::from_utf8(chunk).unwrap())?;
        }
        Ok(())
    }
}

impl FromStr for JoinCode {
    type Err = JoinError;
    fn from_str(s: &str) -> Result<Self, JoinError> {
        let s = s.trim();
        let Some(rest) = s
            .get(..4)
            .filter(|p| p.eq_ignore_ascii_case("NVRS"))
            .map(|_| &s[4..])
        else {
            return Err(JoinError::BadPrefix);
        };
        let body: String = rest
            .chars()
            .filter(|c| *c != '-' && !c.is_whitespace())
            .collect();
        let bytes = b32_decode(&body)?;
        if bytes.len() < 2 + TOKEN_LEN + 2 {
            return Err(JoinError::Truncated);
        }
        let (version, kind) = (bytes[0], bytes[1]);
        if version != VERSION {
            return Err(JoinError::UnsupportedVersion(version));
        }
        let tlen = match kind {
            1 => 8,
            2 => 6,
            3 => 18,
            k => {
                // Still validate nothing; unknown kinds cannot be sized.
                return Err(JoinError::UnsupportedKind(k));
            }
        };
        let total = 2 + tlen + TOKEN_LEN + 2;
        if bytes.len() < total {
            return Err(JoinError::Truncated);
        }
        // Base32 padding may add at most one extra zero byte; extra data beyond
        // that is rejected through the checksum position.
        let (body, crc) = bytes[..total].split_at(total - 2);
        if crc16(body).to_be_bytes() != [crc[0], crc[1]] {
            return Err(JoinError::BadChecksum);
        }
        if bytes[total..].iter().any(|&b| b != 0) {
            return Err(JoinError::Truncated);
        }
        let t = &body[2..2 + tlen];
        let target = match kind {
            1 => JoinTarget::SteamLobby(u64::from_le_bytes(t.try_into().unwrap())),
            2 => {
                let ip = Ipv4Addr::new(t[0], t[1], t[2], t[3]);
                JoinTarget::Direct(SocketAddr::new(
                    IpAddr::V4(ip),
                    u16::from_le_bytes([t[4], t[5]]),
                ))
            }
            _ => {
                let mut o = [0u8; 16];
                o.copy_from_slice(&t[..16]);
                JoinTarget::Direct(SocketAddr::new(
                    IpAddr::V6(Ipv6Addr::from(o)),
                    u16::from_le_bytes([t[16], t[17]]),
                ))
            }
        };
        let mut token = [0u8; TOKEN_LEN];
        token.copy_from_slice(&body[2 + tlen..]);
        Ok(JoinCode { target, token })
    }
}

/// Makes a fresh 16-byte session token. Not a cryptographic secret: it keeps
/// strangers and stale clients out of a friends' session, nothing more. It is
/// seeded from the OS-randomised hasher state and the clock.
pub fn random_token() -> Token {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let rs = RandomState::new();
    let mut out = [0u8; TOKEN_LEN];
    for (i, chunk) in out.chunks_mut(8).enumerate() {
        let mut h = rs.build_hasher();
        h.write_u128(nanos);
        h.write_usize(i);
        chunk.copy_from_slice(&h.finish().to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn steam_code_roundtrip_and_shape() {
        let c = JoinCode::new(JoinTarget::SteamLobby(109775241234567890), [9; 16]);
        let s = c.to_string();
        assert!(s.starts_with("NVRS-"));
        assert!(s.chars().all(|ch| ch == '-' || ch.is_ascii_alphanumeric()));
        assert_eq!(s.parse::<JoinCode>().unwrap(), c);
        // Case, spaces and look-alikes are forgiven.
        let messy = s.to_lowercase().replace('-', " ").replace('0', "o");
        assert_eq!(messy.parse::<JoinCode>().unwrap(), c);
    }

    #[test]
    fn typo_detected() {
        let c = JoinCode::new(
            JoinTarget::Direct("192.168.1.5:7777".parse().unwrap()),
            random_token(),
        );
        let s = c.to_string();
        let mut chars: Vec<char> = s.chars().collect();
        let i = chars
            .iter()
            .rposition(|c| c.is_ascii_alphanumeric())
            .unwrap()
            - 8;
        chars[i] = if chars[i] == 'A' { 'B' } else { 'A' };
        let bad: String = chars.into_iter().collect();
        assert!(matches!(
            bad.parse::<JoinCode>(),
            Err(JoinError::BadChecksum)
        ));
    }

    #[test]
    fn errors() {
        assert_eq!("hello".parse::<JoinCode>(), Err(JoinError::BadPrefix));
        assert_eq!("NVRS-0000".parse::<JoinCode>(), Err(JoinError::Truncated));
        assert!(matches!(
            "NVRS-0!".parse::<JoinCode>(),
            Err(JoinError::BadChar('!'))
        ));
    }

    #[test]
    fn unknown_kind_is_clear() {
        let mut p = vec![VERSION, 4, 1, 2, 3];
        p.extend_from_slice(&[0; 16]);
        p.extend_from_slice(&[0, 0]);
        let s = format!("NVRS-{}", b32_encode(&p));
        assert_eq!(s.parse::<JoinCode>(), Err(JoinError::UnsupportedKind(4)));
    }

    #[test]
    fn tokens_differ() {
        assert_ne!(random_token(), random_token());
    }

    proptest! {
        #[test]
        fn roundtrip_any(id in any::<u64>(), tok in any::<[u8; 16]>(), ip in any::<[u8; 4]>(), port in any::<u16>(), v6 in any::<[u8; 16]>()) {
            for t in [
                JoinTarget::SteamLobby(id),
                JoinTarget::Direct(SocketAddr::new(IpAddr::V4(Ipv4Addr::from(ip)), port)),
                JoinTarget::Direct(SocketAddr::new(IpAddr::V6(Ipv6Addr::from(v6)), port)),
            ] {
                let c = JoinCode::new(t, tok);
                prop_assert_eq!(c.to_string().parse::<JoinCode>().unwrap(), c);
            }
        }

        #[test]
        fn garbage_never_panics(s in ".{0,80}") {
            let _ = s.parse::<JoinCode>();
            let _ = format!("NVRS-{s}").parse::<JoinCode>();
        }
    }
}
