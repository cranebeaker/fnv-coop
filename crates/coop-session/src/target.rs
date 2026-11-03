//! What the player types after `--coop-join` / `--coop-host`.

/// Port used when an address gives none.
pub const DEFAULT_PORT: u16 = 27961;

/// Characters a join code is made of (no 0 O 1 I L U S 5; see
/// `services/join-service`).
pub const CODE_ALPHABET: &str = "ABCDEFGHJKMNPQRTVWXYZ2346789";
/// Length of a join code.
pub const CODE_LEN: usize = 6;

/// Where a guest connects.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JoinTarget {
    /// `host:port` (the default port filled in).
    Addr(String),
    /// A six character join-service code (normalized to upper case).
    Code(String),
    /// A packed `NVRS-...` code (address and token in the code itself).
    Packed(String),
}

/// Normalizes a join code: upper case, spaces and dashes dropped. `None`
/// when the result is not a well formed code.
pub fn normalize_code(s: &str) -> Option<String> {
    let c: String = s
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect::<String>()
        .to_ascii_uppercase();
    (c.len() == CODE_LEN && c.chars().all(|ch| CODE_ALPHABET.contains(ch))).then_some(c)
}

impl JoinTarget {
    /// Parses an address or a join code.
    pub fn parse(s: &str) -> Result<JoinTarget, String> {
        let s = s.trim();
        if s.is_empty() {
            return Err("empty join target".into());
        }
        if s.get(..5).is_some_and(|p| p.eq_ignore_ascii_case("NVRS-")) {
            return Ok(JoinTarget::Packed(s.to_string()));
        }
        let looks_like_addr =
            s.contains(':') || s.contains('.') || s.eq_ignore_ascii_case("localhost");
        if !looks_like_addr {
            return normalize_code(s).map(JoinTarget::Code).ok_or_else(|| {
                format!("'{s}' is neither an address (HOST:PORT) nor a six character join code")
            });
        }
        Ok(JoinTarget::Addr(with_port(s)?))
    }
}

/// `host` or `host:port` or `[v6]:port` as `host:port`.
fn with_port(s: &str) -> Result<String, String> {
    let (host, port) = if let Some(rest) = s.strip_prefix('[') {
        let (h, tail) = rest
            .split_once(']')
            .ok_or_else(|| format!("bad address '{s}'"))?;
        match tail.strip_prefix(':') {
            Some(p) => (format!("[{h}]"), Some(p)),
            None if tail.is_empty() => (format!("[{h}]"), None),
            None => return Err(format!("bad address '{s}'")),
        }
    } else if s.matches(':').count() > 1 {
        return Err(format!(
            "an IPv6 address needs brackets: '[{s}]:{DEFAULT_PORT}'"
        ));
    } else {
        match s.split_once(':') {
            Some((h, p)) => (h.to_string(), Some(p)),
            None => (s.to_string(), None),
        }
    };
    if host.is_empty() || host == "[]" {
        return Err(format!("no host in '{s}'"));
    }
    let port = match port {
        Some(p) => p
            .parse::<u16>()
            .ok()
            .filter(|p| *p != 0)
            .ok_or_else(|| format!("bad port in '{s}'"))?,
        None => DEFAULT_PORT,
    };
    Ok(format!("{host}:{port}"))
}

/// A port to host on: `PORT`, `:PORT` or `ADDR:PORT` (only the port is
/// used for binding; empty means [`DEFAULT_PORT`]).
pub fn parse_host_port(s: &str) -> Result<u16, String> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(DEFAULT_PORT);
    }
    let p = s.rsplit(':').next().unwrap_or(s);
    p.parse::<u16>()
        .ok()
        .filter(|p| *p != 0)
        .ok_or_else(|| format!("bad port in '{s}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_get_the_default_port() {
        assert_eq!(
            JoinTarget::parse("10.0.0.5").unwrap(),
            JoinTarget::Addr("10.0.0.5:27961".into())
        );
        assert_eq!(
            JoinTarget::parse("10.0.0.5:99").unwrap(),
            JoinTarget::Addr("10.0.0.5:99".into())
        );
        assert_eq!(
            JoinTarget::parse("localhost").unwrap(),
            JoinTarget::Addr("localhost:27961".into())
        );
        assert_eq!(
            JoinTarget::parse("[::1]:5").unwrap(),
            JoinTarget::Addr("[::1]:5".into())
        );
        assert!(JoinTarget::parse("::1").is_err());
        assert!(JoinTarget::parse("host:0").is_err());
        assert!(JoinTarget::parse("host:x").is_err());
        assert!(JoinTarget::parse(":5").is_err());
    }

    #[test]
    fn codes_are_normalized() {
        assert_eq!(
            JoinTarget::parse("abc-234").unwrap(),
            JoinTarget::Code("ABC234".into())
        );
        assert_eq!(
            JoinTarget::parse(" k7 m3 xw ").unwrap(),
            JoinTarget::Code("K7M3XW".into())
        );
        assert!(JoinTarget::parse("ABC").is_err());
        // 0, O, 1, I, L, U, S, 5 are not in the alphabet
        assert!(JoinTarget::parse("ABC0EF").is_err());
        assert!(JoinTarget::parse("").is_err());
    }

    #[test]
    fn host_ports() {
        assert_eq!(parse_host_port("").unwrap(), DEFAULT_PORT);
        assert_eq!(parse_host_port("4000").unwrap(), 4000);
        assert_eq!(parse_host_port("0.0.0.0:4001").unwrap(), 4001);
        assert!(parse_host_port("0").is_err());
        assert!(parse_host_port("70000").is_err());
    }
}
