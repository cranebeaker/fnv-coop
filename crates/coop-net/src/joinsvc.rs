//! Client of the join service (`services/join-service`): six character short
//! codes that stand for a host's `NVRS-` join code.
//!
//! The service is a Cloudflare worker with three calls (see its README):
//!
//! * `POST /host` with `{"blob": "...", "ttl_s": N}`: 201 `{code, secret, expires_at, ttl_s}`;
//! * `GET /join/:code`: 200 `{blob}`, 404 unknown or expired, 400 malformed;
//! * `DELETE /host/:code` with `X-Host-Secret`: 200 `{ok:true}`, 401, 403, 404.
//!
//! Errors are `{"error": "..."}`, a rate limit is 429. The client speaks the
//! contract over an [`Http`] transport, so it needs no TLS crate:
//! [`CurlHttp`] runs `curl.exe` (in Windows 10 and later) for `https://`, and
//! [`PlainHttp`] speaks HTTP/1.1 over a `TcpStream` for `http://` (tests and a
//! local stand-in service).

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

/// The symbols of a short code (no 0 O 1 I L U S 5), as the service issues them.
pub const CODE_ALPHABET: &str = "ABCDEFGHJKMNPQRTVWXYZ2346789";
/// Length of a short code.
pub const CODE_LEN: usize = 6;
/// The environment variable that names the service when no address is given.
pub const SERVICE_ENV: &str = "NVRS_JOIN_SERVICE";

/// A short code as the service wants it (upper case, no spaces or dashes), or
/// `None` when it is not one.
pub fn normalize_code(s: &str) -> Option<String> {
    let c: String = s
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect::<String>()
        .to_ascii_uppercase();
    (c.len() == CODE_LEN && c.chars().all(|ch| CODE_ALPHABET.contains(ch))).then_some(c)
}

/// One HTTP exchange: the status and the body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reply {
    /// HTTP status.
    pub status: u16,
    /// Response body.
    pub body: String,
}

/// A way to send one HTTP request.
pub trait Http {
    /// Sends `method url` with the extra `headers` and the JSON `body`.
    fn send(
        &self,
        method: &str,
        url: &str,
        headers: &[(&str, &str)],
        body: Option<&str>,
    ) -> Result<Reply, String>;
}

/// HTTP/1.1 over a plain `TcpStream` (`http://` addresses only).
#[derive(Clone, Debug)]
pub struct PlainHttp {
    /// Connect, read and write timeout.
    pub timeout: Duration,
}

impl Default for PlainHttp {
    fn default() -> Self {
        PlainHttp {
            timeout: Duration::from_secs(10),
        }
    }
}

/// `http://HOST[:PORT]/PATH` split into the address and the path.
fn split_http(url: &str) -> Result<(String, String, String), String> {
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| format!("{url}: the plain transport takes http:// addresses only"))?;
    let (hostport, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    if hostport.is_empty() {
        return Err(format!("{url}: no host"));
    }
    let addr = if hostport.contains(':') && !hostport.ends_with(']') {
        hostport.to_string()
    } else {
        format!("{hostport}:80")
    };
    Ok((addr, hostport.to_string(), path.to_string()))
}

/// The status line and body of a raw HTTP/1.x response (no chunked encoding).
fn parse_response(raw: &[u8]) -> Result<Reply, String> {
    let text = String::from_utf8_lossy(raw);
    let (head, body) = text
        .split_once("\r\n\r\n")
        .ok_or("the response has no header end")?;
    let status = head
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or_else(|| format!("bad status line in {:?}", head.lines().next()))?;
    Ok(Reply {
        status,
        body: body.to_string(),
    })
}

impl Http for PlainHttp {
    fn send(
        &self,
        method: &str,
        url: &str,
        headers: &[(&str, &str)],
        body: Option<&str>,
    ) -> Result<Reply, String> {
        let (addr, host, path) = split_http(url)?;
        let sock = addr
            .to_socket_addrs()
            .map_err(|e| format!("{addr}: {e}"))?
            .next()
            .ok_or_else(|| format!("{addr}: no address"))?;
        let mut s =
            TcpStream::connect_timeout(&sock, self.timeout).map_err(|e| format!("{addr}: {e}"))?;
        s.set_read_timeout(Some(self.timeout))
            .and_then(|_| s.set_write_timeout(Some(self.timeout)))
            .map_err(|e| e.to_string())?;
        let body = body.unwrap_or("");
        let mut req = format!(
            "{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\nAccept: application/json\r\nContent-Length: {}\r\n",
            body.len()
        );
        if !body.is_empty() {
            req.push_str("Content-Type: application/json\r\n");
        }
        for (k, v) in headers {
            req.push_str(&format!("{k}: {v}\r\n"));
        }
        req.push_str("\r\n");
        req.push_str(body);
        s.write_all(req.as_bytes())
            .map_err(|e| format!("{addr}: {e}"))?;
        let mut raw = Vec::new();
        s.read_to_end(&mut raw)
            .map_err(|e| format!("{addr}: {e}"))?;
        parse_response(&raw)
    }
}

/// `curl.exe` as the transport (HTTPS without a TLS crate). Secrets go to
/// curl on its standard input, never on its command line.
#[derive(Clone, Debug)]
pub struct CurlHttp {
    /// The curl program.
    pub exe: PathBuf,
    /// Whole-request timeout, seconds.
    pub timeout_s: u32,
}

impl Default for CurlHttp {
    fn default() -> Self {
        // Windows 10 and later ship curl in System32; elsewhere it is on PATH
        let sys = std::env::var_os("SystemRoot")
            .map(|r| PathBuf::from(r).join("System32").join("curl.exe"))
            .filter(|p| p.is_file());
        CurlHttp {
            exe: sys.unwrap_or_else(|| PathBuf::from("curl")),
            timeout_s: 15,
        }
    }
}

/// The marker curl writes after the body, followed by the status code.
const STATUS_MARK: &str = "\n@@nvrs-status@@";

impl Http for CurlHttp {
    fn send(
        &self,
        method: &str,
        url: &str,
        headers: &[(&str, &str)],
        body: Option<&str>,
    ) -> Result<Reply, String> {
        if !(url.starts_with("https://") || url.starts_with("http://")) {
            return Err(format!("{url}: not an http(s) address"));
        }
        let mut cmd = Command::new(&self.exe);
        cmd.args([
            "-sS",
            "--max-time",
            &self.timeout_s.to_string(),
            "-X",
            method,
        ])
        .args(["-H", "Accept: application/json"])
        .args(["-w", &format!("{STATUS_MARK}%{{http_code}}")]);
        // the body or the headers come in on stdin, so a secret is not in the process list
        let stdin = match body {
            Some(b) => {
                cmd.args([
                    "-H",
                    "Content-Type: application/json",
                    "--data-binary",
                    "@-",
                ]);
                for (k, v) in headers {
                    cmd.args(["-H", &format!("{k}: {v}")]);
                }
                b.to_string()
            }
            None if !headers.is_empty() => {
                cmd.args(["-H", "@-"]);
                headers.iter().map(|(k, v)| format!("{k}: {v}\n")).collect()
            }
            None => String::new(),
        };
        cmd.arg("--").arg(url);
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = cmd
            .spawn()
            .map_err(|e| format!("{}: {e}", self.exe.display()))?;
        if let Some(mut si) = child.stdin.take() {
            si.write_all(stdin.as_bytes())
                .map_err(|e| format!("curl stdin: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("curl: {e}"))?;
        let text = String::from_utf8_lossy(&out.stdout).to_string();
        let Some((body, code)) = text.rsplit_once(STATUS_MARK) else {
            return Err(format!(
                "curl failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        };
        let status = code
            .trim()
            .parse::<u16>()
            .ok()
            .filter(|s| *s != 0)
            .ok_or_else(|| {
                format!(
                    "curl reached no server: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                )
            })?;
        Ok(Reply {
            status,
            body: body.to_string(),
        })
    }
}

/// The transport an address needs: curl for `https://`, the plain one for `http://`.
pub fn http_for(base: &str) -> Box<dyn Http> {
    if base.starts_with("https://") {
        Box::new(CurlHttp::default())
    } else {
        Box::new(PlainHttp::default())
    }
}

/// A value of a flat JSON object.
#[derive(Clone, Debug, PartialEq)]
pub enum Flat {
    /// A string.
    Str(String),
    /// A number.
    Num(f64),
    /// true, false.
    Bool(bool),
    /// null, or a nested value (skipped).
    Other,
}

/// Parses a flat JSON object (`{"k": "v", "n": 1, ...}`); nested values are
/// skipped as [`Flat::Other`].
pub fn parse_flat(text: &str) -> Option<BTreeMap<String, Flat>> {
    let b = text.trim().as_bytes();
    let mut i = 0;
    let ws = |i: &mut usize| {
        while *i < b.len() && (b[*i] as char).is_ascii_whitespace() {
            *i += 1;
        }
    };
    fn string(b: &[u8], i: &mut usize) -> Option<String> {
        if b.get(*i) != Some(&b'"') {
            return None;
        }
        *i += 1;
        let mut out = Vec::new();
        while *i < b.len() {
            match b[*i] {
                b'"' => {
                    *i += 1;
                    return String::from_utf8(out).ok();
                }
                b'\\' => {
                    let c = *b.get(*i + 1)?;
                    *i += 2;
                    match c {
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'u' => {
                            let hex = std::str::from_utf8(b.get(*i..*i + 4)?).ok()?;
                            let cp = u32::from_str_radix(hex, 16).ok()?;
                            *i += 4;
                            let ch = char::from_u32(cp).unwrap_or('\u{fffd}');
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        other => out.push(other),
                    }
                }
                c => {
                    out.push(c);
                    *i += 1;
                }
            }
        }
        None
    }
    fn skip_value(b: &[u8], i: &mut usize) -> Option<()> {
        let mut depth = 0i32;
        while *i < b.len() {
            match b[*i] {
                b'"' => {
                    string(b, i)?;
                    continue;
                }
                b'{' | b'[' => depth += 1,
                b'}' | b']' => {
                    if depth == 0 {
                        return Some(());
                    }
                    depth -= 1;
                    if depth == 0 {
                        *i += 1;
                        return Some(());
                    }
                }
                b',' if depth == 0 => return Some(()),
                _ => {}
            }
            *i += 1;
        }
        Some(())
    }
    ws(&mut i);
    if b.get(i) != Some(&b'{') {
        return None;
    }
    i += 1;
    let mut m = BTreeMap::new();
    loop {
        ws(&mut i);
        if b.get(i) == Some(&b'}') {
            return Some(m);
        }
        let k = string(b, &mut i)?;
        ws(&mut i);
        if b.get(i) != Some(&b':') {
            return None;
        }
        i += 1;
        ws(&mut i);
        let v = match b.get(i)? {
            b'"' => Flat::Str(string(b, &mut i)?),
            b't' if b[i..].starts_with(b"true") => {
                i += 4;
                Flat::Bool(true)
            }
            b'f' if b[i..].starts_with(b"false") => {
                i += 5;
                Flat::Bool(false)
            }
            b'-' | b'0'..=b'9' => {
                let s = i;
                while i < b.len() && matches!(b[i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
                {
                    i += 1;
                }
                Flat::Num(std::str::from_utf8(&b[s..i]).ok()?.parse().ok()?)
            }
            _ => {
                skip_value(b, &mut i)?;
                Flat::Other
            }
        };
        m.insert(k, v);
        ws(&mut i);
        match b.get(i)? {
            b',' => i += 1,
            b'}' => return Some(m),
            _ => return None,
        }
    }
}

/// A JSON string literal of `s`.
pub fn json_str(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// A code the service gave the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hosted {
    /// The six character code to give friends.
    pub code: String,
    /// The secret that removes the code again (shown once, keep it).
    pub secret: String,
    /// When the code expires, ms since the Unix epoch.
    pub expires_at: u64,
    /// The time to live the service granted, seconds.
    pub ttl_s: u32,
}

/// The join service at one address.
pub struct JoinService {
    base: String,
    http: Box<dyn Http>,
}

impl std::fmt::Debug for JoinService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JoinService")
            .field("base", &self.base)
            .finish()
    }
}

fn get_str(m: &BTreeMap<String, Flat>, k: &str) -> Option<String> {
    match m.get(k) {
        Some(Flat::Str(s)) => Some(s.clone()),
        _ => None,
    }
}

fn get_num(m: &BTreeMap<String, Flat>, k: &str) -> Option<f64> {
    match m.get(k) {
        Some(Flat::Num(n)) => Some(*n),
        _ => None,
    }
}

impl JoinService {
    /// The service at `base` (`https://...` or `http://...`, no trailing path)
    /// over `http`.
    pub fn new(base: &str, http: Box<dyn Http>) -> JoinService {
        JoinService {
            base: base.trim().trim_end_matches('/').to_string(),
            http,
        }
    }

    /// The service at `base` over the transport its scheme needs ([`http_for`]).
    pub fn at(base: &str) -> JoinService {
        JoinService::new(base, http_for(base.trim()))
    }

    /// The service the environment names ([`SERVICE_ENV`]), if any.
    pub fn from_env() -> Option<JoinService> {
        std::env::var(SERVICE_ENV)
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| JoinService::at(&s))
    }

    /// The address.
    pub fn base(&self) -> &str {
        &self.base
    }

    fn error(&self, r: &Reply, what: &str) -> String {
        let why = parse_flat(&r.body)
            .and_then(|m| get_str(&m, "error"))
            .unwrap_or_else(|| r.body.chars().take(80).collect());
        match r.status {
            429 => {
                format!("the join service is busy (rate limited); try again in a minute ({what})")
            }
            _ => format!("the join service refused {what}: {} {why}", r.status),
        }
    }

    /// `POST /host`: publishes `blob` (the host's `NVRS-` code) and returns
    /// its short code. `ttl_s` is clamped by the service (60 to 21600 s).
    pub fn host(&self, blob: &str, ttl_s: Option<u32>) -> Result<Hosted, String> {
        let body = match ttl_s {
            Some(t) => format!("{{\"blob\":{},\"ttl_s\":{t}}}", json_str(blob)),
            None => format!("{{\"blob\":{}}}", json_str(blob)),
        };
        let r = self
            .http
            .send("POST", &format!("{}/host", self.base), &[], Some(&body))?;
        if r.status != 201 {
            return Err(self.error(&r, "the code"));
        }
        let m = parse_flat(&r.body).ok_or("the join service sent no JSON object")?;
        let code = get_str(&m, "code")
            .and_then(|c| normalize_code(&c))
            .ok_or("the join service sent no valid code")?;
        Ok(Hosted {
            code,
            secret: get_str(&m, "secret").ok_or("the join service sent no secret")?,
            expires_at: get_num(&m, "expires_at").unwrap_or(0.0) as u64,
            ttl_s: get_num(&m, "ttl_s").unwrap_or(0.0) as u32,
        })
    }

    /// `GET /join/:code`: the blob (the host's `NVRS-` code) a short code stands for.
    pub fn resolve(&self, code: &str) -> Result<String, String> {
        let c = normalize_code(code)
            .ok_or_else(|| format!("'{code}' is not a {CODE_LEN} character join code"))?;
        let r = self
            .http
            .send("GET", &format!("{}/join/{c}", self.base), &[], None)?;
        match r.status {
            200 => {
                let m = parse_flat(&r.body).ok_or("the join service sent no JSON object")?;
                get_str(&m, "blob").ok_or_else(|| "the join service sent no blob".into())
            }
            404 => Err(format!(
                "the join code {c} is unknown or has expired; ask the host for a new one"
            )),
            _ => Err(self.error(&r, &format!("the code {c}"))),
        }
    }

    /// `DELETE /host/:code` with the secret from [`JoinService::host`].
    pub fn remove(&self, code: &str, secret: &str) -> Result<(), String> {
        let c = normalize_code(code).ok_or_else(|| format!("'{code}' is not a join code"))?;
        let r = self.http.send(
            "DELETE",
            &format!("{}/host/{c}", self.base),
            &[("X-Host-Secret", secret)],
            None,
        )?;
        match r.status {
            200 => Ok(()),
            _ => Err(self.error(&r, &format!("removing {c}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// One recorded request: method, url, headers, body.
    type Sent = (String, String, Vec<(String, String)>, Option<String>);

    /// Records requests and answers from a script.
    struct Fake {
        seen: RefCell<Vec<Sent>>,
        answers: RefCell<Vec<Reply>>,
    }

    impl Http for &Fake {
        fn send(
            &self,
            method: &str,
            url: &str,
            headers: &[(&str, &str)],
            body: Option<&str>,
        ) -> Result<Reply, String> {
            self.seen.borrow_mut().push((
                method.into(),
                url.into(),
                headers
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
                body.map(str::to_string),
            ));
            Ok(self.answers.borrow_mut().remove(0))
        }
    }

    fn fake(answers: Vec<(u16, &str)>) -> &'static Fake {
        Box::leak(Box::new(Fake {
            seen: RefCell::new(Vec::new()),
            answers: RefCell::new(
                answers
                    .into_iter()
                    .map(|(s, b)| Reply {
                        status: s,
                        body: b.into(),
                    })
                    .collect(),
            ),
        }))
    }

    #[test]
    fn the_three_calls_follow_the_contract() {
        let f = fake(vec![
            (
                201,
                r#"{"code":"K7M3XW","secret":"ab12","expires_at":1700000000000,"ttl_s":1800}"#,
            ),
            (200, r#"{"blob":"NVRS-XYZ"}"#),
            (404, r#"{"error":"unknown or expired code"}"#),
            (200, r#"{"ok":true}"#),
            (403, r#"{"error":"wrong secret"}"#),
        ]);
        let s = JoinService::new("http://svc.example/", Box::new(f));
        let h = s.host("NVRS-XYZ", Some(600)).unwrap();
        assert_eq!(h.code, "K7M3XW");
        assert_eq!(h.secret, "ab12");
        assert_eq!(h.ttl_s, 1800);
        assert_eq!(s.resolve("k7m-3xw").unwrap(), "NVRS-XYZ");
        assert!(s
            .resolve("K7M3XW")
            .unwrap_err()
            .contains("unknown or has expired"));
        s.remove("K7M3XW", "ab12").unwrap();
        assert!(s.remove("K7M3XW", "bad").unwrap_err().contains("403"));
        let seen = f.seen.borrow();
        assert_eq!(seen[0].0, "POST");
        assert_eq!(seen[0].1, "http://svc.example/host");
        assert_eq!(
            seen[0].3.as_deref(),
            Some(r#"{"blob":"NVRS-XYZ","ttl_s":600}"#)
        );
        assert_eq!(seen[1].1, "http://svc.example/join/K7M3XW");
        assert_eq!(seen[3].0, "DELETE");
        assert_eq!(seen[3].1, "http://svc.example/host/K7M3XW");
        assert_eq!(seen[3].2, vec![("X-Host-Secret".into(), "ab12".into())]);
    }

    #[test]
    fn a_malformed_code_is_refused_before_any_request() {
        let f = fake(vec![]);
        let s = JoinService::new("http://svc.example", Box::new(f));
        assert!(s.resolve("ABC0EF").is_err());
        assert!(f.seen.borrow().is_empty());
    }

    #[test]
    fn flat_json_and_strings() {
        let m =
            parse_flat(r#" {"a":"x\"yA","n":-1.5e2,"b":false,"o":{"z":[1,2]},"s":"t"} "#).unwrap();
        assert_eq!(m["a"], Flat::Str("x\"yA".into()));
        assert_eq!(m["n"], Flat::Num(-150.0));
        assert_eq!(m["b"], Flat::Bool(false));
        assert_eq!(m["o"], Flat::Other);
        assert_eq!(m["s"], Flat::Str("t".into()));
        assert!(parse_flat("[1]").is_none());
        assert_eq!(json_str("a\"b\\c\n"), r#""a\"b\\c\u000a""#);
        assert_eq!(normalize_code(" k7 m3-xw "), Some("K7M3XW".into()));
        assert_eq!(normalize_code("K7M3X0"), None);
    }

    #[test]
    fn plain_http_splits_addresses_and_responses() {
        assert_eq!(
            split_http("http://127.0.0.1:8787/join/ABC").unwrap(),
            (
                "127.0.0.1:8787".to_string(),
                "127.0.0.1:8787".to_string(),
                "/join/ABC".to_string()
            )
        );
        assert_eq!(split_http("http://svc").unwrap().0, "svc:80");
        assert!(split_http("https://svc").is_err());
        let r = parse_response(b"HTTP/1.1 404 Not Found\r\nA: b\r\n\r\n{\"error\":\"x\"}").unwrap();
        assert_eq!(r.status, 404);
        assert_eq!(r.body, "{\"error\":\"x\"}");
    }
}
