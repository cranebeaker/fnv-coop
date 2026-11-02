//! A tiny JSON line writer (no dependency): flat objects of strings,
//! numbers and booleans, which is all the session log and dump need.

/// A JSON value.
#[derive(Clone, Copy, Debug)]
pub enum V<'a> {
    /// String.
    S(&'a str),
    /// Unsigned number.
    U(u64),
    /// Signed number.
    I(i64),
    /// Boolean.
    B(bool),
    /// Seconds, three decimals.
    F(f64),
}

/// Escapes `s` as the inside of a JSON string.
pub fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o
}

fn value(v: &V) -> String {
    match v {
        V::S(s) => format!("\"{}\"", esc(s)),
        V::U(n) => n.to_string(),
        V::I(n) => n.to_string(),
        V::B(b) => b.to_string(),
        V::F(f) => format!("{f:.3}"),
    }
}

/// One flat object on one line.
pub fn object(fields: &[(&str, V)]) -> String {
    let body: Vec<String> = fields.iter().map(|(k, v)| format!("\"{}\":{}", esc(k), value(v))).collect();
    format!("{{{}}}", body.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_and_formats() {
        let s = object(&[("a", V::S("x\"y\n")), ("n", V::U(3)), ("b", V::B(true)), ("i", V::I(-2))]);
        assert_eq!(s, "{\"a\":\"x\\\"y\\n\",\"n\":3,\"b\":true,\"i\":-2}");
    }
}
