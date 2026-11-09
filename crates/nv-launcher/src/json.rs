//! A tiny JSON reader and writer, enough for the settings file. std only.

/// A JSON value. Objects keep their key order.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Value>),
    Obj(Vec<(String, Value)>),
}

impl Value {
    /// The member `key` of an object.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Obj(m) => m.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        if let Value::Num(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}

/// Parses a JSON document.
pub fn parse(text: &str) -> Result<Value, String> {
    let mut p = P {
        s: text.as_bytes(),
        i: 0,
        depth: 0,
    };
    p.ws();
    let v = p.value()?;
    p.ws();
    if p.i != p.s.len() {
        return Err(format!("unexpected data at byte {}", p.i));
    }
    Ok(v)
}

struct P<'a> {
    s: &'a [u8],
    i: usize,
    depth: usize,
}

impl P<'_> {
    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\r' | b'\n') {
            self.i += 1;
        }
    }
    fn eat(&mut self, c: u8) -> Result<(), String> {
        if self.s.get(self.i) == Some(&c) {
            self.i += 1;
            Ok(())
        } else {
            Err(format!("expected '{}' at byte {}", c as char, self.i))
        }
    }
    fn lit(&mut self, word: &str, v: Value) -> Result<Value, String> {
        if self.s[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(v)
        } else {
            Err(format!("bad value at byte {}", self.i))
        }
    }
    fn value(&mut self) -> Result<Value, String> {
        self.depth += 1;
        if self.depth > 64 {
            return Err("nested too deeply".into());
        }
        let r = match self.s.get(self.i) {
            None => Err("unexpected end".into()),
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => self.string().map(Value::Str),
            Some(b't') => self.lit("true", Value::Bool(true)),
            Some(b'f') => self.lit("false", Value::Bool(false)),
            Some(b'n') => self.lit("null", Value::Null),
            Some(_) => self.number(),
        };
        self.depth -= 1;
        r
    }
    fn number(&mut self) -> Result<Value, String> {
        let start = self.i;
        while self.i < self.s.len()
            && matches!(
                self.s[self.i],
                b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E'
            )
        {
            self.i += 1;
        }
        std::str::from_utf8(&self.s[start..self.i])
            .ok()
            .and_then(|t| t.parse::<f64>().ok())
            .filter(|n| n.is_finite())
            .map(Value::Num)
            .ok_or_else(|| format!("bad number at byte {start}"))
    }
    fn hex4(&mut self) -> Result<u32, String> {
        let t = self.s.get(self.i..self.i + 4).ok_or("bad \\u escape")?;
        let t = std::str::from_utf8(t).map_err(|_| "bad \\u escape")?;
        self.i += 4;
        u32::from_str_radix(t, 16).map_err(|_| "bad \\u escape".to_string())
    }
    fn string(&mut self) -> Result<String, String> {
        self.eat(b'"')?;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = *self.s.get(self.i).ok_or("unterminated string")?;
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = *self.s.get(self.i).ok_or("unterminated string")?;
                    self.i += 1;
                    let ch = match e {
                        b'"' => '"',
                        b'\\' => '\\',
                        b'/' => '/',
                        b'b' => '\u{8}',
                        b'f' => '\u{c}',
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'u' => {
                            let mut cp = self.hex4()?;
                            if (0xD800..0xDC00).contains(&cp)
                                && self.s[self.i..].starts_with(b"\\u")
                            {
                                self.i += 2;
                                let lo = self.hex4()?;
                                cp = 0x10000
                                    + ((cp - 0xD800) << 10)
                                    + (lo.wrapping_sub(0xDC00) & 0x3FF);
                            }
                            char::from_u32(cp).unwrap_or('\u{fffd}')
                        }
                        _ => return Err("bad escape".into()),
                    };
                    let mut b = [0u8; 4];
                    out.extend_from_slice(ch.encode_utf8(&mut b).as_bytes());
                }
                c if c < 0x20 => return Err("control character in string".into()),
                c => out.push(c),
            }
        }
        String::from_utf8(out).map_err(|_| "invalid UTF-8".to_string())
    }
    fn array(&mut self) -> Result<Value, String> {
        self.eat(b'[')?;
        let mut v = Vec::new();
        self.ws();
        if self.s.get(self.i) == Some(&b']') {
            self.i += 1;
            return Ok(Value::Arr(v));
        }
        loop {
            self.ws();
            v.push(self.value()?);
            self.ws();
            match self.s.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    self.i += 1;
                    return Ok(Value::Arr(v));
                }
                _ => return Err(format!("expected ',' or ']' at byte {}", self.i)),
            }
        }
    }
    fn object(&mut self) -> Result<Value, String> {
        self.eat(b'{')?;
        let mut m = Vec::new();
        self.ws();
        if self.s.get(self.i) == Some(&b'}') {
            self.i += 1;
            return Ok(Value::Obj(m));
        }
        loop {
            self.ws();
            let k = self.string()?;
            self.ws();
            self.eat(b':')?;
            self.ws();
            let v = self.value()?;
            m.push((k, v));
            self.ws();
            match self.s.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    self.i += 1;
                    return Ok(Value::Obj(m));
                }
                _ => return Err(format!("expected ',' or '}}' at byte {}", self.i)),
            }
        }
    }
}

/// Quotes `s` as a JSON string.
pub fn quote(s: &str) -> String {
    let mut o = String::from("\"");
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
    o.push('"');
    o
}

/// Writes `v` with two-space indentation.
pub fn to_string_pretty(v: &Value) -> String {
    let mut o = String::new();
    write(v, 0, &mut o);
    o.push('\n');
    o
}

fn write(v: &Value, ind: usize, o: &mut String) {
    let pad = |n: usize| "  ".repeat(n);
    match v {
        Value::Null => o.push_str("null"),
        Value::Bool(b) => o.push_str(if *b { "true" } else { "false" }),
        Value::Num(n) => {
            if n.fract() == 0.0 && n.abs() < 1e15 {
                o.push_str(&format!("{}", *n as i64));
            } else {
                o.push_str(&format!("{n}"));
            }
        }
        Value::Str(s) => o.push_str(&quote(s)),
        Value::Arr(a) if a.is_empty() => o.push_str("[]"),
        Value::Arr(a) => {
            o.push_str("[\n");
            for (i, x) in a.iter().enumerate() {
                o.push_str(&pad(ind + 1));
                write(x, ind + 1, o);
                o.push_str(if i + 1 < a.len() { ",\n" } else { "\n" });
            }
            o.push_str(&pad(ind));
            o.push(']');
        }
        Value::Obj(m) if m.is_empty() => o.push_str("{}"),
        Value::Obj(m) => {
            o.push_str("{\n");
            for (i, (k, x)) in m.iter().enumerate() {
                o.push_str(&pad(ind + 1));
                o.push_str(&quote(k));
                o.push_str(": ");
                write(x, ind + 1, o);
                o.push_str(if i + 1 < m.len() { ",\n" } else { "\n" });
            }
            o.push_str(&pad(ind));
            o.push('}');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let t = r#"{"a": "x\\y\"zé", "b": [1, 2.5, true, null], "c": {}}"#;
        let v = parse(t).unwrap();
        assert_eq!(v.get("a").unwrap().as_str().unwrap(), "x\\y\"z\u{e9}");
        assert_eq!(parse(&to_string_pretty(&v)).unwrap(), v);
    }

    #[test]
    fn rejects_garbage() {
        for bad in [
            "",
            "{",
            "{\"a\":}",
            "[1,]",
            "{\"a\":1} x",
            "\"abc",
            "nul",
            "{1:2}",
        ] {
            assert!(parse(bad).is_err(), "{bad}");
        }
        assert!(parse(&"[".repeat(200)).is_err());
    }

    #[test]
    fn surrogate_pair() {
        assert_eq!(
            parse("\"\\ud83d\\ude00\"").unwrap().as_str().unwrap(),
            "\u{1F600}"
        );
    }
}
