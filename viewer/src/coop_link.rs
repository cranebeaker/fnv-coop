//! Opens the co-op transport from the `--coop-*` flags (UDP or Steam).
//!
//! The flag logic is in `coop_session::link` (tested there); this file only maps
//! `CoopArgs` onto a `LinkRequest` and writes the resulting notices to the session log
//! (`transport_fallback`, `coop_join_code`, `steam_lobby`, `steam_joined`).
//!
//! Steam needs the viewer built with `--features coop-steam`; without it `--coop-steam`
//! on a host logs a `transport_fallback` and hosts on UDP.

use coop_session::json::V;
use coop_session::link::{self, LinkRequest, Notice};
use coop_session::{Role, Session, SessionConfig};

use crate::args::CoopArgs;

/// The request for these flags. A guest's address is `--coop-join`'s value.
pub fn request(c: &CoopArgs, role: Role, name: &str) -> LinkRequest {
    let addr = match (role, c.host, &c.join) {
        (Role::Host, Some(port), _) => port.to_string(),
        (_, _, Some(a)) => a.clone(),
        _ => String::new(),
    };
    let mut r = LinkRequest::new(role, name, &addr);
    r.steam = c.steam;
    r.steam_friend = c.steam_friend;
    r.test_token = c.test_token;
    r.token = c.token;
    r
}

/// Opens the transport into `cfg.transport` and prints the player-facing lines. Returns
/// the notices to log once the session exists.
pub fn open(c: &CoopArgs, role: Role, cfg: &mut SessionConfig) -> Result<Vec<Notice>, String> {
    let o = link::open(&request(c, role, &cfg.name)).map_err(|e| e.to_string())?;
    for l in &o.lines {
        println!("{l}");
    }
    cfg.transport = Some(o.transport);
    Ok(o.notices)
}

/// Writes the notices into the session log.
pub fn log_notices(session: &mut Session, notices: &[Notice]) {
    for n in notices {
        let fields: Vec<(&str, V)> = n
            .fields
            .iter()
            .map(|(k, v)| (*k, V::S(v.as_str())))
            .collect();
        session.log_event(n.ev, &fields);
    }
}
