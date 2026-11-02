//! A small JSON reader for nv-call's result lines (no crates; nv-oracle-core's
//! reader can't be a path dependency of this standalone crate because it
//! inherits its version from the nv-oracle workspace).

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
    pub fn parse(text: &str) -> Option<Value> {
        let mut p = Parser {
            s: text.as_bytes(),
            i: 0,
        };
        let v = p.value()?;
        p.ws();
        (p.i == p.s.len()).then_some(v)
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Obj(kv) => kv.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_num(&self) -> Option<f64> {
        match self {
            Value::Num(n) => Some(*n),
            _ => None,
        }
    }
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Obj(_))
    }
    /// Compact text form (for reports).
    pub fn show(&self) -> String {
        match self {
            Value::Null => "null".into(),
            Value::Bool(b) => b.to_string(),
            Value::Num(n) => n.to_string(),
            Value::Str(s) => format!("{s:?}"),
            Value::Arr(a) => format!(
                "[{}]",
                a.iter().map(Value::show).collect::<Vec<_>>().join(",")
            ),
            Value::Obj(kv) => {
                format!(
                    "{{{}}}",
                    kv.iter()
                        .map(|(k, v)| format!("{k:?}:{}", v.show()))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
        }
    }
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn ws(&mut self) {
        while self.i < self.s.len() && self.s[self.i].is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn eat(&mut self, c: u8) -> Option<()> {
        self.ws();
        (self.s.get(self.i) == Some(&c)).then(|| self.i += 1)
    }
    fn lit(&mut self, word: &str, v: Value) -> Option<Value> {
        self.s[self.i..].starts_with(word.as_bytes()).then(|| {
            self.i += word.len();
            v
        })
    }
    fn value(&mut self) -> Option<Value> {
        self.ws();
        match *self.s.get(self.i)? {
            b'{' => {
                self.i += 1;
                let mut kv = Vec::new();
                if self.eat(b'}').is_some() {
                    return Some(Value::Obj(kv));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.eat(b':')?;
                    kv.push((k, self.value()?));
                    if self.eat(b',').is_none() {
                        self.eat(b'}')?;
                        return Some(Value::Obj(kv));
                    }
                }
            }
            b'[' => {
                self.i += 1;
                let mut a = Vec::new();
                if self.eat(b']').is_some() {
                    return Some(Value::Arr(a));
                }
                loop {
                    a.push(self.value()?);
                    if self.eat(b',').is_none() {
                        self.eat(b']')?;
                        return Some(Value::Arr(a));
                    }
                }
            }
            b'"' => self.string().map(Value::Str),
            b't' => self.lit("true", Value::Bool(true)),
            b'f' => self.lit("false", Value::Bool(false)),
            b'n' => self.lit("null", Value::Null),
            _ => {
                let start = self.i;
                while self.i < self.s.len()
                    && matches!(
                        self.s[self.i],
                        b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9'
                    )
                {
                    self.i += 1;
                }
                std::str::from_utf8(&self.s[start..self.i])
                    .ok()?
                    .parse()
                    .ok()
                    .map(Value::Num)
            }
        }
    }
    fn string(&mut self) -> Option<String> {
        if self.s.get(self.i) != Some(&b'"') {
            return None;
        }
        self.i += 1;
        let mut out = String::new();
        loop {
            let c = *self.s.get(self.i)?;
            self.i += 1;
            match c {
                b'"' => return Some(out),
                b'\\' => {
                    let e = *self.s.get(self.i)?;
                    self.i += 1;
                    match e {
                        b'n' => out.push('\n'),
                        b't' => out.push('\t'),
                        b'r' => out.push('\r'),
                        b'u' => {
                            let h = std::str::from_utf8(self.s.get(self.i..self.i + 4)?).ok()?;
                            self.i += 4;
                            out.push(
                                char::from_u32(u32::from_str_radix(h, 16).ok()?).unwrap_or('?'),
                            );
                        }
                        other => out.push(other as char),
                    }
                }
                _ => {
                    // copy one UTF-8 sequence
                    let start = self.i - 1;
                    let len = match c {
                        0xf0..=0xff => 4,
                        0xe0..=0xef => 3,
                        0xc0..=0xdf => 2,
                        _ => 1,
                    };
                    self.i = start + len;
                    out.push_str(std::str::from_utf8(self.s.get(start..self.i)?).ok()?);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_result_shapes() {
        let v = Value::parse(r#"{"a":"x\"y","b":[1,-2.5e3,null,true],"c":{"d":{}}}"#).unwrap();
        assert_eq!(v.get("a").unwrap().as_str(), Some("x\"y"));
        assert_eq!(
            v.get("b"),
            Some(&Value::Arr(vec![
                Value::Num(1.0),
                Value::Num(-2500.0),
                Value::Null,
                Value::Bool(true)
            ]))
        );
        assert!(v.get("c").unwrap().get("d").unwrap().is_object());
        assert!(Value::parse("{\"a\":1} x").is_none());
    }
}
