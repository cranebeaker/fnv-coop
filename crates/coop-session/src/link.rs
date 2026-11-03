//! Opening the session's transport: UDP or Steam, with per-session tokens.
//!
//! The viewer builds a [`LinkRequest`] from its `--coop-*` flags, calls [`open`] and
//! hands the result to `SessionConfig::transport`. Afterwards it writes
//! [`Opened::notices`] into the session log with `Session::log_event`.
//!
//! # Tokens
//!
//! * A UDP host makes a random 16 byte token per session ([`coop_net::random_token`])
//!   unless the launcher gave one with `--coop-token` (so its join code matches). The
//!   token goes into the join code the host logs (`coop_join_code`).
//! * A plain `HOST:PORT` has no token, so a guest can use it only with
//!   `--coop-test-token`, which selects the fixed [`DIRECT_TOKEN`] on both sides (the test
//!   harness runs hosts and guests through a relay by address). Real players use the
//!   `NVRS-` code.
//! * A Steam host puts the token in its join code and in a lobby data key, so a friend who
//!   joins without a code (`--coop-steam-friend`) can read it.
//!
//! # Fallback
//!
//! A host that asked for Steam but cannot have it (no DLL, Steam not running, build without
//! the `steam` feature, lobby not created in time) hosts on UDP instead and records a
//! `transport_fallback` notice. That is a notice, not an error. A guest has no UDP address
//! to fall back to when the code names a Steam lobby, so it gets an [`Error`] that says
//! what to do.

use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::path::PathBuf;

use coop_core::transport::Transport;
use coop_net::joinsvc::{Hosted, JoinService, SERVICE_ENV};
use coop_net::{
    random_token, Endpoint, EndpointConfig, JoinCode, JoinTarget as NetTarget, Token, UdpBackend,
    UdpConfig,
};

use crate::net::{Link, LinkErrors, DIRECT_TOKEN};
use crate::{Error, Role};

/// What the viewer asks for.
#[derive(Clone, Debug)]
pub struct LinkRequest {
    /// Host or guest.
    pub role: Role,
    /// Display name.
    pub name: String,
    /// Host: the UDP port as text (also the fallback port of a Steam host). Guest: what
    /// `--coop-join` was given (empty for `--coop-steam-friend`).
    pub addr: String,
    /// `--coop-steam`.
    pub steam: bool,
    /// `--coop-steam-friend` (guest).
    pub steam_friend: bool,
    /// `--coop-test-token`: the fixed token (harness only).
    pub test_token: bool,
    /// `--coop-token`: the host's token.
    pub token: Option<Token>,
    /// Where to look for `steam_api64.dll` (default: next to the exe).
    pub steam_dll: Option<PathBuf>,
    /// The join service (`https://...` or `http://...`). Host: publish the join code
    /// there and show its six character short code. Guest: resolve a short code
    /// there (default: the `NVRS_JOIN_SERVICE` environment variable).
    pub join_service: Option<String>,
}

impl LinkRequest {
    /// A UDP request with no flags set.
    pub fn new(role: Role, name: &str, addr: &str) -> LinkRequest {
        LinkRequest {
            role,
            name: name.to_string(),
            addr: addr.to_string(),
            steam: false,
            steam_friend: false,
            test_token: false,
            token: None,
            steam_dll: None,
            join_service: None,
        }
    }

    /// The join service a guest resolves short codes with: the request's, else
    /// the environment's.
    pub fn service(&self) -> Option<JoinService> {
        match self
            .join_service
            .as_deref()
            .filter(|s| !s.trim().is_empty())
        {
            Some(u) => Some(JoinService::at(u)),
            None => JoinService::from_env(),
        }
    }
}

/// Something worth a log line, as an event name and string fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notice {
    /// Event name (`transport_fallback`, `coop_join_code`, `steam_lobby`, ...).
    pub ev: &'static str,
    /// Field names and values.
    pub fields: Vec<(&'static str, String)>,
}

impl Notice {
    pub(crate) fn new(ev: &'static str, fields: &[(&'static str, &str)]) -> Notice {
        Notice {
            ev,
            fields: fields.iter().map(|(k, v)| (*k, v.to_string())).collect(),
        }
    }

    /// The field `key`, if set.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// An opened transport and what to tell the player.
pub struct Opened {
    /// The transport for `SessionConfig::transport`.
    pub transport: Box<dyn Transport>,
    /// Errors the link reports while it runs.
    pub errors: LinkErrors,
    /// `"udp"` or `"steam"`: what is actually in use.
    pub kind: &'static str,
    /// Host: the join code to give friends.
    pub join_code: Option<String>,
    /// Host: the join service's short code for `join_code`, with the secret that
    /// removes it ([`remove_short_code`]).
    pub short_code: Option<Hosted>,
    /// Log events to write once the session exists.
    pub notices: Vec<Notice>,
    /// Lines to print for the player.
    pub lines: Vec<String>,
    /// Steam invite and friends hooks (Steam transport only).
    #[cfg(feature = "steam")]
    pub steam: Option<crate::steam_transport::SteamHandle>,
}

impl Opened {
    pub(crate) fn new(
        transport: Box<dyn Transport>,
        errors: LinkErrors,
        kind: &'static str,
    ) -> Opened {
        Opened {
            transport,
            errors,
            kind,
            join_code: None,
            short_code: None,
            notices: Vec::new(),
            lines: Vec::new(),
            #[cfg(feature = "steam")]
            steam: None,
        }
    }
}

/// A token as 32 lower case hex digits.
pub fn token_hex(t: &Token) -> String {
    t.iter().map(|b| format!("{b:02x}")).collect()
}

/// Parses 32 hex digits.
pub fn parse_token_hex(s: &str) -> Option<Token> {
    let s = s.trim();
    if s.len() != 32 || !s.is_ascii() {
        return None;
    }
    let mut out = [0u8; 16];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

/// The token a host uses: the given one, the fixed test token, or a fresh random one.
pub fn host_token(req: &LinkRequest) -> Token {
    if let Some(t) = req.token {
        t
    } else if req.test_token {
        DIRECT_TOKEN
    } else {
        random_token()
    }
}

/// Best guess of this machine's LAN address (a UDP "connect" sends nothing).
pub fn guess_local_ip() -> IpAddr {
    UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| {
            s.connect("192.0.2.1:9")?;
            s.local_addr()
        })
        .map(|a| a.ip())
        .ok()
        .filter(|ip| !ip.is_unspecified())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
}

fn resolve(addr: &str) -> Result<SocketAddr, Error> {
    addr.to_socket_addrs()
        .map_err(|e| Error::Target(format!("{addr}: {e}")))?
        .next()
        .ok_or_else(|| Error::Target(format!("{addr}: no address")))
}

fn endpoint_cfg(req: &LinkRequest) -> EndpointConfig {
    EndpointConfig {
        name: req.name.clone(),
        ..EndpointConfig::default()
    }
}

/// Opens the transport the request asks for.
pub fn open(req: &LinkRequest) -> Result<Opened, Error> {
    match req.role {
        Role::Host => open_host(req),
        Role::Guest => open_guest(req),
    }
}

fn open_host(req: &LinkRequest) -> Result<Opened, Error> {
    let token = host_token(req);
    let mut fallback: Option<Notice> = None;
    if req.steam {
        match try_steam_host(req, token) {
            Ok(o) => return Ok(o),
            Err(why) => {
                fallback = Some(Notice::new(
                    "transport_fallback",
                    &[("from", "steam"), ("to", "udp"), ("reason", &why)],
                ));
            }
        }
    }
    let mut o = open_udp_host(req, token)?;
    if let Some(n) = fallback {
        o.lines.insert(
            0,
            format!(
                "Co-op: Steam is not available ({}); hosting on UDP instead.",
                n.get("reason").unwrap_or("")
            ),
        );
        o.notices.insert(0, n);
    }
    Ok(o)
}

#[cfg(feature = "steam")]
fn try_steam_host(req: &LinkRequest, token: Token) -> Result<Opened, String> {
    crate::steam_transport::open_host(req, token)
}

#[cfg(not(feature = "steam"))]
fn try_steam_host(_req: &LinkRequest, _token: Token) -> Result<Opened, String> {
    Err("this viewer was built without Steam support (cargo feature coop-steam)".into())
}

fn open_udp_host(req: &LinkRequest, token: Token) -> Result<Opened, Error> {
    let port = crate::target::parse_host_port(&req.addr).map_err(Error::Target)?;
    let bind = SocketAddr::from(([0, 0, 0, 0], port));
    let b = UdpBackend::host(bind, token, UdpConfig::default())
        .map_err(|e| Error::Io(format!("{}: {e}", req.addr)))?;
    let errors = LinkErrors::default();
    let ep = Endpoint::host(b, token, endpoint_cfg(req));
    let link = Link {
        ep,
        guest: false,
        errors: errors.clone(),
        retry_at: None,
    };
    let mut o = Opened::new(Box::new(link), errors, "udp");
    let ip = guess_local_ip();
    let code = JoinCode::new(NetTarget::Direct(SocketAddr::new(ip, port)), token).to_string();
    o.lines.push(format!("Co-op: hosting on UDP port {port}."));
    if req.test_token {
        o.lines.push(
            "Co-op: test token in use; guests join by HOST:PORT with --coop-test-token.".into(),
        );
    } else {
        o.lines.push(format!(
            "Co-op: join code for this machine's address {ip}: {code} (for play over the internet, forward UDP port {port} and use your public address in the code)"
        ));
    }
    o.notices.push(Notice::new(
        "coop_join_code",
        &[
            ("transport", "udp"),
            ("code", &code),
            ("address", &format!("{ip}:{port}")),
            ("token", &token_hex(&token)),
            ("test_token", if req.test_token { "true" } else { "false" }),
        ],
    ));
    if let Some(url) = req.join_service.as_deref().filter(|u| !u.trim().is_empty()) {
        publish_short_code(&mut o, url, &code);
    }
    o.join_code = Some(code);
    Ok(o)
}

/// Host: publishes the join code to the join service at `url` and records its
/// short code (`coop_short_code`), or why there is none (`coop_short_code_failed`;
/// the `NVRS-` code still works). The secret stays in [`Opened::short_code`] only.
fn publish_short_code(o: &mut Opened, url: &str, code: &str) {
    match JoinService::at(url).host(code, None) {
        Ok(h) => {
            o.lines.push(format!(
                "Co-op: short join code {} (from the join service, valid for {} minutes)",
                h.code,
                (h.ttl_s / 60).max(1)
            ));
            o.notices.push(Notice::new(
                "coop_short_code",
                &[
                    ("code", &h.code),
                    ("service", url),
                    ("expires_at", &h.expires_at.to_string()),
                ],
            ));
            o.short_code = Some(h);
        }
        Err(e) => {
            o.lines.push(format!(
                "Co-op: no short join code ({e}); give friends the NVRS- code"
            ));
            o.notices.push(Notice::new(
                "coop_short_code_failed",
                &[("service", url), ("reason", &e)],
            ));
        }
    }
}

/// Host: takes the short code off the join service at session end (needs the
/// secret it was given).
pub fn remove_short_code(url: &str, h: &Hosted) -> Result<(), String> {
    JoinService::at(url).remove(&h.code, &h.secret)
}

/// Guest: the `NVRS-` code a short code stands for, from the join service.
pub fn resolve_short_code(req: &LinkRequest, code: &str) -> Result<JoinCode, Error> {
    let svc = req.service().ok_or_else(|| {
        Error::Target(format!(
            "{code} is a short join code: it needs the join service, and none is set (pass its address, or set {SERVICE_ENV}); or use the host's NVRS- code"
        ))
    })?;
    let blob = svc.resolve(code).map_err(Error::Target)?;
    blob.parse::<JoinCode>().map_err(|e| {
        Error::Target(format!(
            "the join service gave {code} a code this build cannot read: {e}"
        ))
    })
}

fn open_guest(req: &LinkRequest) -> Result<Opened, Error> {
    if req.steam_friend {
        return guest_steam(req, None);
    }
    match crate::target::JoinTarget::parse(&req.addr).map_err(Error::Target)? {
        crate::target::JoinTarget::Addr(a) => {
            if !req.test_token {
                return Err(Error::Target(format!(
                    "{a}: a plain address has no session token. Ask the host for the NVRS- join code (the host's log has it as coop_join_code)"
                )));
            }
            udp_guest(req, resolve(&a)?, DIRECT_TOKEN)
        }
        crate::target::JoinTarget::Packed(c) => {
            let code: JoinCode = c.parse().map_err(|e| Error::Target(format!("{e}")))?;
            match code.target {
                NetTarget::Direct(a) => udp_guest(req, a, code.token),
                NetTarget::SteamLobby(_) => guest_steam(req, Some(code)),
                _ => Err(Error::Target("this join code needs a newer build".into())),
            }
        }
        crate::target::JoinTarget::Code(c) => {
            let code = resolve_short_code(req, &c)?;
            let mut o = match code.target {
                NetTarget::Direct(a) => udp_guest(req, a, code.token)?,
                NetTarget::SteamLobby(_) => guest_steam(req, Some(code))?,
                _ => return Err(Error::Target("this join code needs a newer build".into())),
            };
            o.notices
                .push(Notice::new("coop_short_code_resolved", &[("code", &c)]));
            Ok(o)
        }
    }
}

fn udp_guest(req: &LinkRequest, target: SocketAddr, token: Token) -> Result<Opened, Error> {
    let b = UdpBackend::guest(target, token, UdpConfig::default())
        .map_err(|e| Error::Io(format!("{target}: {e}")))?;
    let errors = LinkErrors::default();
    let ep = Endpoint::guest(b, token, endpoint_cfg(req));
    let link = Link {
        ep,
        guest: true,
        errors: errors.clone(),
        retry_at: None,
    };
    Ok(Opened::new(Box::new(link), errors, "udp"))
}

#[cfg(feature = "steam")]
fn guest_steam(req: &LinkRequest, code: Option<JoinCode>) -> Result<Opened, Error> {
    crate::steam_transport::open_guest(req, code)
}

#[cfg(not(feature = "steam"))]
fn guest_steam(_req: &LinkRequest, _code: Option<JoinCode>) -> Result<Opened, Error> {
    Err(Error::Target(
        "this join needs Steam, but this viewer was built without Steam support. Ask the host for a UDP join code or use the viewer built with the coop-steam feature".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_hex_round_trip() {
        let t = random_token();
        assert_eq!(parse_token_hex(&token_hex(&t)), Some(t));
        assert_eq!(parse_token_hex("12"), None);
        assert_eq!(parse_token_hex(&"zz".repeat(16)), None);
    }

    #[test]
    fn host_token_choices() {
        let mut r = LinkRequest::new(Role::Host, "h", "27961");
        let a = host_token(&r);
        let b = host_token(&r);
        assert_ne!(a, b, "random per session");
        assert_ne!(a, DIRECT_TOKEN);
        r.test_token = true;
        assert_eq!(host_token(&r), DIRECT_TOKEN);
        r.token = Some([7; 16]);
        assert_eq!(host_token(&r), [7; 16], "an explicit token wins");
    }

    #[test]
    fn plain_address_needs_the_test_token() {
        let r = LinkRequest::new(Role::Guest, "g", "127.0.0.1:9");
        let e = open(&r).err().expect("refused");
        assert!(e.to_string().contains("NVRS-"), "{e}");
        let mut t = r.clone();
        t.test_token = true;
        let o = open(&t).expect("test token joins by address");
        assert_eq!(o.kind, "udp");
    }

    fn free_port() -> u16 {
        UdpSocket::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }

    #[test]
    fn udp_host_logs_a_code_that_carries_its_random_token() {
        let port = free_port();
        let o = open(&LinkRequest::new(Role::Host, "h", &port.to_string())).unwrap();
        let code = o.join_code.clone().expect("code");
        let n = o
            .notices
            .iter()
            .find(|n| n.ev == "coop_join_code")
            .expect("coop_join_code notice");
        assert_eq!(n.get("code"), Some(code.as_str()));
        let parsed: JoinCode = code.parse().unwrap();
        assert_eq!(token_hex(&parsed.token), n.get("token").unwrap());
        assert_ne!(parsed.token, DIRECT_TOKEN);
        match parsed.target {
            NetTarget::Direct(a) => assert_eq!(a.port(), port),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn steam_host_without_steam_falls_back_to_udp_with_a_notice() {
        let port = free_port();
        let mut r = LinkRequest::new(Role::Host, "h", &port.to_string());
        r.steam = true;
        // No DLL next to the test exe (and, without the feature, no Steam code at all).
        r.steam_dll = Some(PathBuf::from("definitely-not-here/steam_api64.dll"));
        let o = open(&r).unwrap();
        assert_eq!(o.kind, "udp");
        let n = &o.notices[0];
        assert_eq!(n.ev, "transport_fallback");
        assert_eq!(n.get("from"), Some("steam"));
        assert_eq!(n.get("to"), Some("udp"));
        assert!(!n.get("reason").unwrap().is_empty());
        assert!(o.join_code.is_some(), "the UDP code is still logged");
    }

    #[test]
    fn steam_code_without_steam_is_a_clear_error() {
        let code =
            JoinCode::new(NetTarget::SteamLobby(109_775_241_000_000_001), [3; 16]).to_string();
        let mut r = LinkRequest::new(Role::Guest, "g", &code);
        r.steam_dll = Some(PathBuf::from("definitely-not-here/steam_api64.dll"));
        let e = open(&r).err().expect("no way to join");
        assert!(e.to_string().to_ascii_lowercase().contains("steam"), "{e}");
    }
}
