//! Connectivity self-test for UDP direct play. No UPnP, no outside service.

use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

const MAGIC: &[u8] = b"NVRS-PROBE1";

#[derive(Debug, Clone, PartialEq)]
pub struct Check {
    pub name: &'static str,
    pub ok: bool,
    pub detail: String,
}

/// How reachable an address is from outside this machine's network.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Loopback,
    /// Private ranges and link-local: only the same network (or a VPN) can reach it.
    Lan,
    Public,
}

pub fn scope(ip: IpAddr) -> Scope {
    match ip {
        IpAddr::V4(a) => {
            let o = a.octets();
            if a.is_loopback() {
                Scope::Loopback
            } else if a.is_private()
                || a.is_link_local()
                || (o[0] == 100 && (64..128).contains(&o[1]))
            {
                Scope::Lan
            } else {
                Scope::Public
            }
        }
        IpAddr::V6(a) => {
            let s = a.segments();
            if a.is_loopback() {
                Scope::Loopback
            } else if (s[0] & 0xfe00) == 0xfc00 || (s[0] & 0xffc0) == 0xfe80 {
                Scope::Lan
            } else {
                Scope::Public
            }
        }
    }
}

/// This machine's main LAN address: the source address the OS would use to reach the internet.
/// A UDP connect sends nothing.
pub fn local_address() -> Option<IpAddr> {
    let s = UdpSocket::bind("0.0.0.0:0").ok()?;
    s.connect("192.0.2.1:9").ok()?;
    s.local_addr()
        .ok()
        .map(|a| a.ip())
        .filter(|ip| !ip.is_unspecified())
}

/// Can this program listen on UDP `port`?
pub fn bind_test(port: u16) -> Check {
    match UdpSocket::bind(("0.0.0.0", port)) {
        Ok(_) => Check { name: "Listen on the port", ok: true, detail: format!("UDP {port} is free on this machine.") },
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => Check {
            name: "Listen on the port",
            ok: false,
            detail: format!("UDP {port} is already in use. A game may already be hosting, or another program uses it. Pick another port in Setup."),
        },
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Check {
            name: "Listen on the port",
            ok: false,
            detail: format!("Windows refused UDP {port} ({e}). Pick another port in Setup."),
        },
        Err(e) => Check { name: "Listen on the port", ok: false, detail: format!("Could not listen on UDP {port}: {e}") },
    }
}

/// Sends a packet to ourselves on `port` and waits for it. Proves the network stack works, not the
/// firewall or router.
pub fn loopback_test(port: u16) -> Check {
    let name = "Local round trip";
    let fail = |d: String| Check {
        name,
        ok: false,
        detail: d,
    };
    let server = match UdpSocket::bind(("127.0.0.1", port)) {
        Ok(s) => s,
        Err(e) => return fail(format!("Could not bind 127.0.0.1:{port}: {e}")),
    };
    let _ = server.set_read_timeout(Some(Duration::from_millis(1500)));
    let client = match UdpSocket::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) => return fail(format!("Could not open a client socket: {e}")),
    };
    if let Err(e) = client.send_to(MAGIC, ("127.0.0.1", port)) {
        return fail(format!("Could not send: {e}"));
    }
    let mut buf = [0u8; 64];
    match server.recv_from(&mut buf) {
        Ok((n, _)) if &buf[..n] == MAGIC => Check {
            name,
            ok: true,
            detail: "A packet sent to this machine arrived.".into(),
        },
        Ok(_) => fail("Received something unexpected.".into()),
        Err(e) => fail(format!(
            "The packet never arrived ({e}). Security software may be blocking this program."
        )),
    }
}

/// Listens on `port` for `secs` and answers every probe. Returns the number answered and the
/// sources seen. `now` ticks are real time; `stop` can end it early.
pub fn listen_echo(
    port: u16,
    secs: u64,
    stop: &dyn Fn() -> bool,
    on_hit: &mut dyn FnMut(SocketAddr),
) -> Result<usize, String> {
    let s = UdpSocket::bind(("0.0.0.0", port))
        .map_err(|e| format!("Could not listen on UDP {port}: {e}"))?;
    s.set_read_timeout(Some(Duration::from_millis(250)))
        .map_err(|e| e.to_string())?;
    let end = Instant::now() + Duration::from_secs(secs);
    let mut hits = 0;
    let mut buf = [0u8; 64];
    while Instant::now() < end && !stop() {
        if let Ok((n, from)) = s.recv_from(&mut buf) {
            if &buf[..n] == MAGIC {
                let _ = s.send_to(MAGIC, from);
                hits += 1;
                on_hit(from);
            }
        }
    }
    Ok(hits)
}

/// Sends a probe to `target` and waits for the echo from a listening launcher.
pub fn probe(target: SocketAddr, wait: Duration) -> Check {
    let name = "Reach the other machine";
    let bind = if target.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    };
    let s = match UdpSocket::bind(bind) {
        Ok(s) => s,
        Err(e) => {
            return Check {
                name,
                ok: false,
                detail: format!("Could not open a socket: {e}"),
            }
        }
    };
    let _ = s.set_read_timeout(Some(Duration::from_millis(500)));
    let end = Instant::now() + wait;
    let mut buf = [0u8; 64];
    while Instant::now() < end {
        let _ = s.send_to(MAGIC, target);
        if let Ok((n, _)) = s.recv_from(&mut buf) {
            if &buf[..n] == MAGIC {
                return Check {
                    name,
                    ok: true,
                    detail: format!(
                        "{target} answered. Direct play to that address and port should work."
                    ),
                };
            }
        }
    }
    Check {
        name,
        ok: false,
        detail: format!("No answer from {target}. Is the other launcher listening (menu: Port check, option 3)? If it is, a firewall or router is dropping UDP on that port."),
    }
}

/// Plain steps for a host whose friends cannot reach them.
pub fn instructions(port: u16, local: Option<IpAddr>) -> Vec<String> {
    let mut v = vec![
        format!("Friends on the same network (same Wi-Fi or router) join with your LAN address and port {port}."),
        match local {
            Some(ip) => format!("Your LAN address looks like {ip}."),
            None => "Your LAN address could not be found; run ipconfig and read IPv4 Address.".into(),
        },
        "When Windows asks whether nv-viewer.exe may use the network, allow it on Private networks (and Public if you are on a public network).".into(),
        format!("Friends over the internet: forward UDP port {port} on your router to this PC, then give them your public IP (search the web for \"what is my IP\") and the port."),
        "No router access, or behind carrier NAT: use a VPN-style overlay (a private network app such as ZeroTier or Tailscale) and share the address it gives you.".into(),
        "UPnP is not used. The Steam friends option, when this build has it, needs no port forwarding at all.".into(),
    ];
    v.retain(|l| !l.is_empty());
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn free_port() -> u16 {
        UdpSocket::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }

    #[test]
    fn scopes() {
        assert_eq!(scope("127.0.0.1".parse().unwrap()), Scope::Loopback);
        assert_eq!(scope("192.168.1.5".parse().unwrap()), Scope::Lan);
        assert_eq!(scope("10.1.2.3".parse().unwrap()), Scope::Lan);
        assert_eq!(scope("172.20.0.1".parse().unwrap()), Scope::Lan);
        assert_eq!(scope("100.100.1.1".parse().unwrap()), Scope::Lan);
        assert_eq!(scope("8.8.8.8".parse().unwrap()), Scope::Public);
        assert_eq!(scope("fe80::1".parse().unwrap()), Scope::Lan);
        assert_eq!(scope("2001:db8::1".parse().unwrap()), Scope::Public);
    }

    #[test]
    fn bind_and_in_use() {
        let p = free_port();
        assert!(bind_test(p).ok);
        let hold = UdpSocket::bind(("0.0.0.0", p)).unwrap();
        let c = bind_test(p);
        assert!(!c.ok);
        assert!(c.detail.contains("already in use"));
        drop(hold);
    }

    #[test]
    fn loopback_works() {
        assert!(loopback_test(free_port()).ok);
    }

    #[test]
    fn probe_gets_echo_and_times_out() {
        let p = free_port();
        let h = std::thread::spawn(move || {
            let mut seen = 0;
            let r = listen_echo(p, 3, &|| false, &mut |_| seen += 1);
            (r, seen)
        });
        std::thread::sleep(Duration::from_millis(300));
        let c = probe(
            SocketAddr::from(([127, 0, 0, 1], p)),
            Duration::from_secs(2),
        );
        assert!(c.ok, "{}", c.detail);
        let (r, seen) = h.join().unwrap();
        assert!(r.unwrap() >= 1 && seen >= 1);
        let dead = probe(
            SocketAddr::from(([127, 0, 0, 1], free_port())),
            Duration::from_millis(800),
        );
        assert!(!dead.ok);
    }

    #[test]
    fn instructions_mention_port() {
        let t = instructions(4242, None).join("\n");
        assert!(t.contains("4242") && t.contains("forward"));
    }
}
