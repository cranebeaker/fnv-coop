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
    let body: Vec<String> = fields
        .iter()
        .map(|(k, v)| format!("\"{}\":{}", esc(k), value(v)))
        .collect();
    format!("{{{}}}", body.join(","))
}

/// Parses one flat JSON object (string, number, boolean and null values; no
/// nesting) into key/value pairs. Strings are unescaped; other values keep
/// their text (`3.5`, `true`, `null`). The control port's request reader.
pub fn parse_flat(text: &str) -> Result<Vec<(String, String)>, String> {
    let c: Vec<char> = text.trim().chars().collect();
    let mut i = 0;
    let ws = |i: &mut usize| {
        while *i < c.len() && c[*i].is_whitespace() {
            *i += 1;
        }
    };
    let string = |i: &mut usize| -> Result<String, String> {
        if c.get(*i) != Some(&'"') {
            return Err(format!("expected a string at {}", *i));
        }
        *i += 1;
        let mut o = String::new();
        loop {
            let ch = *c.get(*i).ok_or("unterminated string")?;
            *i += 1;
            match ch {
                '"' => return Ok(o),
                '\\' => {
                    let e = *c.get(*i).ok_or("unterminated escape")?;
                    *i += 1;
                    match e {
                        'n' => o.push('\n'),
                        'r' => o.push('\r'),
                        't' => o.push('\t'),
                        'u' => {
                            let hex: String = c
                                .get(*i..*i + 4)
                                .ok_or("short unicode escape")?
                                .iter()
                                .collect();
                            *i += 4;
                            let n =
                                u32::from_str_radix(&hex, 16).map_err(|_| "bad unicode escape")?;
                            o.push(char::from_u32(n).unwrap_or('?'));
                        }
                        other => o.push(other),
                    }
                }
                ch => o.push(ch),
            }
        }
    };
    ws(&mut i);
    if c.get(i) != Some(&'{') {
        return Err("expected a JSON object".into());
    }
    i += 1;
    let mut out = Vec::new();
    ws(&mut i);
    if c.get(i) == Some(&'}') {
        return Ok(out);
    }
    loop {
        ws(&mut i);
        let k = string(&mut i)?;
        ws(&mut i);
        if c.get(i) != Some(&':') {
            return Err(format!("expected ':' after \"{k}\""));
        }
        i += 1;
        ws(&mut i);
        let v = if c.get(i) == Some(&'"') {
            string(&mut i)?
        } else {
            let start = i;
            while i < c.len() && !matches!(c[i], ',' | '}') && !c[i].is_whitespace() {
                if matches!(c[i], '{' | '[') {
                    return Err(format!("\"{k}\": nested values are not supported"));
                }
                i += 1;
            }
            if start == i {
                return Err(format!("\"{k}\": missing value"));
            }
            c[start..i].iter().collect()
        };
        out.push((k, v));
        ws(&mut i);
        match c.get(i) {
            Some(',') => i += 1,
            Some('}') => break,
            _ => return Err("expected ',' or '}'".into()),
        }
    }
    i += 1;
    ws(&mut i);
    if i != c.len() {
        return Err("text after the object".into());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flat_requests() {
        let kv = parse_flat(
            r#" {"cmd":"run","line":"SetStage \"Q\" 10","local":true,"n":-2.5,"x":null} "#,
        )
        .unwrap();
        let get = |k: &str| kv.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str());
        assert_eq!(get("cmd"), Some("run"));
        assert_eq!(get("line"), Some("SetStage \"Q\" 10"));
        assert_eq!(get("local"), Some("true"));
        assert_eq!(get("n"), Some("-2.5"));
        assert_eq!(parse_flat("{}").unwrap().len(), 0);
        assert!(parse_flat("{\"a\":[1]}").is_err());
        assert!(parse_flat("{\"a\":1} x").is_err());
        assert!(parse_flat("nope").is_err());
    }

    #[test]
    fn escapes_and_formats() {
        let s = object(&[
            ("a", V::S("x\"y\n")),
            ("n", V::U(3)),
            ("b", V::B(true)),
            ("i", V::I(-2)),
        ]);
        assert_eq!(s, "{\"a\":\"x\\\"y\\n\",\"n\":3,\"b\":true,\"i\":-2}");
    }
}
