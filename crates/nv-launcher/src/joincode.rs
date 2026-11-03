//! Join codes (`NVRS-...`, from coop-net) and plain `ip:port` targets.

use coop_net::{JoinCode, JoinTarget};
use std::net::SocketAddr;

pub use crate::settings::DEFAULT_PORT;

/// The launcher makes a random 16 byte token per hosted session, passes it to the viewer
/// (`--coop-token`) and puts it in the join code, so a code is the only way in.
pub fn token_hex(t: &[u8; 16]) -> String {
    t.iter().map(|b| format!("{b:02x}")).collect()
}

/// A token from 8 random bytes (the caller supplies randomness; the other half is the bytes
/// run through a hash so the full 16 bytes vary). `coop_net::random_token` is used by the app.
pub fn token_from_seed(seed: [u8; 8]) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut h: u64 = u64::from_le_bytes(seed) ^ 0x9E37_79B9_7F4A_7C15;
    for chunk in out.chunks_mut(8) {
        h ^= h >> 30;
        h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        h ^= h >> 27;
        h = h.wrapping_mul(0x94D0_49BB_1331_11EB);
        h ^= h >> 31;
        chunk.copy_from_slice(&h.to_le_bytes());
    }
    out
}

/// What a guest typed.
#[derive(Debug, Clone, PartialEq)]
pub enum JoinInput {
    /// A join code, normalised (upper case, dashes) and decoded.
    Code { text: String, target: JoinTarget },
    /// `HOST:PORT`, normalised with the port. The viewer cannot join this without a token:
    /// only probes use it now.
    Addr(String),
    /// Join whichever Steam friend is hosting (`--coop-steam-friend`), no code.
    SteamFriend,
    /// A six character join-service code (normalised). [`resolve_short`] turns it into
    /// the `NVRS-` code it stands for; the viewer can also resolve it itself.
    Short(String),
}

impl JoinInput {
    /// The text for `--coop-join`.
    pub fn viewer_arg(&self) -> &str {
        match self {
            JoinInput::Code { text, .. } => text,
            JoinInput::Addr(a) => a,
            JoinInput::SteamFriend => "",
            JoinInput::Short(c) => c,
        }
    }
}

/// Asks the join service what a short code stands for and returns that join code.
pub fn resolve_short(
    code: &str,
    svc: &coop_net::joinsvc::JoinService,
) -> Result<JoinInput, String> {
    let blob = svc.resolve(code)?;
    match parse_join(&blob)? {
        j @ JoinInput::Code { .. } => Ok(j),
        _ => Err(format!(
            "the join service gave {code} something that is not a join code"
        )),
    }
}

fn valid_host(h: &str) -> bool {
    !h.is_empty()
        && h.len() <= 253
        && h.split('.').all(|l| {
            !l.is_empty()
                && l.len() <= 63
                && l.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
                && !l.starts_with('-')
                && !l.ends_with('-')
        })
}

/// Parses a join code or an address (`1.2.3.4`, `1.2.3.4:5000`, `[::1]:5000`, `name.example`).
pub fn parse_join(input: &str) -> Result<JoinInput, String> {
    let s = input.trim();
    if s.is_empty() {
        return Err("Nothing was entered.".into());
    }
    if s.len() >= 4 && s[..4].eq_ignore_ascii_case("NVRS") {
        let code: JoinCode = s.parse().map_err(|e| format!("{e}"))?;
        return Ok(JoinInput::Code {
            text: code.to_string(),
            target: code.target,
        });
    }
    // a six character code of the join service (no ':' or '.', so not a host name with a port
    // or a domain); the viewer's `coop_session::target` reads it the same way
    if !s.contains(':') && !s.contains('.') {
        if let Some(c) = coop_net::joinsvc::normalize_code(s) {
            return Ok(JoinInput::Short(c));
        }
    }
    if let Some(a) = s.parse::<SocketAddr>().ok().filter(|a| a.port() != 0) {
        return Ok(JoinInput::Addr(a.to_string()));
    }
    if let Ok(ip) = s
        .trim_matches(|c| c == '[' || c == ']')
        .parse::<std::net::IpAddr>()
    {
        return Ok(JoinInput::Addr(
            SocketAddr::new(ip, DEFAULT_PORT).to_string(),
        ));
    }
    let (host, port) = match s.rsplit_once(':') {
        Some((h, p)) => {
            let port: u16 = p
                .parse()
                .ok()
                .filter(|p| *p != 0)
                .ok_or_else(|| format!("'{p}' is not a port number (1 to 65535)."))?;
            (h, port)
        }
        None => (s, DEFAULT_PORT),
    };
    if !valid_host(host) {
        return Err(format!(
            "'{host}' is not an address or a join code. Use NVRS-..., 1.2.3.4 or 1.2.3.4:27961."
        ));
    }
    Ok(JoinInput::Addr(format!("{host}:{port}")))
}

/// The code a host shows for `addr` (an IP address, with or without a port, default `port`),
/// carrying the session `token`.
pub fn host_code(addr: &str, port: u16, token: [u8; 16]) -> Result<String, String> {
    let a = addr.trim();
    let sock: SocketAddr = if let Ok(s) = a.parse::<SocketAddr>() {
        s
    } else if let Ok(ip) = a
        .trim_matches(|c| c == '[' || c == ']')
        .parse::<std::net::IpAddr>()
    {
        SocketAddr::new(ip, port)
    } else {
        return Err(format!(
            "'{a}' is not an IP address. A join code needs one (for example 192.168.1.20)."
        ));
    };
    Ok(JoinCode::new(JoinTarget::Direct(sock), token).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_round_trip_and_messy_input() {
        let code = host_code("192.168.1.20", 27961, [5; 16]).unwrap();
        assert!(code.starts_with("NVRS-"));
        let messy = format!("  {}  ", code.to_ascii_lowercase().replace('-', " "));
        match parse_join(&messy).unwrap() {
            JoinInput::Code { text, target } => {
                assert_eq!(text, code);
                assert_eq!(
                    target,
                    JoinTarget::Direct("192.168.1.20:27961".parse().unwrap())
                );
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn typo_in_code_is_reported() {
        let code = host_code("10.0.0.2:4000", 1, [6; 16]).unwrap();
        let mut bad = code.clone();
        let last = bad.pop().unwrap();
        bad.push(if last == '0' { '1' } else { '0' });
        let e = parse_join(&bad).unwrap_err();
        assert!(e.contains("checksum") || e.contains("truncated"), "{e}");
        assert!(parse_join("NVRS-").is_err());
    }

    #[test]
    fn addresses() {
        assert_eq!(
            parse_join("1.2.3.4").unwrap(),
            JoinInput::Addr("1.2.3.4:27961".into())
        );
        assert_eq!(
            parse_join("1.2.3.4:5000").unwrap(),
            JoinInput::Addr("1.2.3.4:5000".into())
        );
        assert_eq!(
            parse_join("[::1]:5000").unwrap(),
            JoinInput::Addr("[::1]:5000".into())
        );
        assert_eq!(
            parse_join("pc.example.com").unwrap(),
            JoinInput::Addr("pc.example.com:27961".into())
        );
        assert_eq!(
            parse_join("pc.example.com:99").unwrap().viewer_arg(),
            "pc.example.com:99"
        );
        for bad in [
            "",
            "1.2.3.4:0",
            "1.2.3.4:99999",
            "bad host",
            "a..b",
            "-x.com",
            "x;rm",
        ] {
            assert!(parse_join(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn short_codes_are_codes_not_host_names() {
        for typed in ["K7M3XW", "k7m-3xw", " k7 m3 xw "] {
            assert_eq!(
                parse_join(typed).unwrap(),
                JoinInput::Short("K7M3XW".into()),
                "{typed}"
            );
        }
        assert_eq!(parse_join("K7M3XW").unwrap().viewer_arg(), "K7M3XW");
        // a host name with a dot or a port stays an address
        assert_eq!(
            parse_join("K7M3XW.lan").unwrap(),
            JoinInput::Addr("K7M3XW.lan:27961".into())
        );
        assert_eq!(
            parse_join("K7M3XW:99").unwrap(),
            JoinInput::Addr("K7M3XW:99".into())
        );
    }

    #[test]
    fn host_code_needs_an_ip() {
        assert!(host_code("my-pc", 1, [0; 16]).is_err());
        assert!(host_code("8.8.8.8", 27961, [0; 16]).is_ok());
    }

    #[test]
    fn codes_carry_the_hosts_token_and_tokens_differ() {
        let t = token_from_seed([1, 2, 3, 4, 5, 6, 7, 8]);
        let code = host_code("10.0.0.2", 4000, t).unwrap();
        match parse_join(&code).unwrap() {
            JoinInput::Code { .. } => {}
            other => panic!("{other:?}"),
        }
        let parsed: JoinCode = code.parse().unwrap();
        assert_eq!(parsed.token, t);
        assert_ne!(t, token_from_seed([2, 2, 3, 4, 5, 6, 7, 8]));
        assert_ne!(t[..8], t[8..]);
        assert_eq!(token_hex(&[0xab; 16]).len(), 32);
    }

    #[test]
    fn steam_lobby_codes_parse() {
        let c = JoinCode::new(JoinTarget::SteamLobby(109_775_241_000_000_001), [3; 16]).to_string();
        assert!(matches!(
            parse_join(&c).unwrap(),
            JoinInput::Code {
                target: JoinTarget::SteamLobby(_),
                ..
            }
        ));
    }
}
