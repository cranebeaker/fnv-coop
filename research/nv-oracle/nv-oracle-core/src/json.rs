//! A small JSON reader and writer, enough for JSON-lines files.

use std::fmt::Write as _;

/// A parsed JSON value. Integers keep full precision (`Int` is wide enough
/// for any `u64` or `i64`); object members keep their file order.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i128),
    Float(f64),
    Str(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

/// A parse error with the byte offset where it was detected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonError {
    pub pos: usize,
    pub msg: String,
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON error at byte {}: {}", self.pos, self.msg)
    }
}

impl std::error::Error for JsonError {}

const MAX_DEPTH: usize = 64;

impl Value {
    /// Parse one complete JSON document (no trailing data allowed).
    pub fn parse(text: &str) -> Result<Value, JsonError> {
        let mut p = Parser {
            s: text.as_bytes(),
            pos: 0,
        };
        p.skip_ws();
        let v = p.value(0)?;
        p.skip_ws();
        if p.pos != p.s.len() {
            return Err(p.err("trailing characters after the JSON value"));
        }
        Ok(v)
    }

    /// First member named `key`, if this is an object.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(members) => members.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_i128(&self) -> Option<i128> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&[(String, Value)]> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Serialise on one line. Non-finite floats become `null`.
    pub fn to_json(&self) -> String {
        let mut out = String::new();
        self.write_into(&mut out);
        out
    }

    fn write_into(&self, out: &mut String) {
        match self {
            Value::Null => out.push_str("null"),
            Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Value::Int(i) => {
                let _ = write!(out, "{i}");
            }
            Value::Float(f) => push_f64(out, *f),
            Value::Str(s) => push_str(out, s),
            Value::Array(items) => {
                out.push('[');
                for (i, v) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    v.write_into(out);
                }
                out.push(']');
            }
            Value::Object(members) => {
                out.push('{');
                for (i, (k, v)) in members.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    push_str(out, k);
                    out.push(':');
                    v.write_into(out);
                }
                out.push('}');
            }
        }
    }
}

struct Parser<'a> {
    s: &'a [u8],
    pos: usize,
}

impl Parser<'_> {
    fn err(&self, msg: &str) -> JsonError {
        JsonError {
            pos: self.pos,
            msg: msg.to_string(),
        }
    }

    fn skip_ws(&mut self) {
        while matches!(self.s.get(self.pos), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.pos += 1;
        }
    }

    fn eat(&mut self, c: u8) -> Result<(), JsonError> {
        if self.s.get(self.pos) == Some(&c) {
            self.pos += 1;
            Ok(())
        } else {
            Err(self.err(&format!("expected '{}'", c as char)))
        }
    }

    fn literal(&mut self, word: &str, v: Value) -> Result<Value, JsonError> {
        if self.s[self.pos..].starts_with(word.as_bytes()) {
            self.pos += word.len();
            Ok(v)
        } else {
            Err(self.err("invalid literal"))
        }
    }

    fn value(&mut self, depth: usize) -> Result<Value, JsonError> {
        if depth > MAX_DEPTH {
            return Err(self.err("nesting too deep"));
        }
        match self.s.get(self.pos) {
            None => Err(self.err("unexpected end of input")),
            Some(b'{') => self.object(depth),
            Some(b'[') => self.array(depth),
            Some(b'"') => Ok(Value::Str(self.string()?)),
            Some(b't') => self.literal("true", Value::Bool(true)),
            Some(b'f') => self.literal("false", Value::Bool(false)),
            Some(b'n') => self.literal("null", Value::Null),
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(_) => Err(self.err("unexpected character")),
        }
    }

    fn object(&mut self, depth: usize) -> Result<Value, JsonError> {
        self.eat(b'{')?;
        let mut members = Vec::new();
        self.skip_ws();
        if self.s.get(self.pos) == Some(&b'}') {
            self.pos += 1;
            return Ok(Value::Object(members));
        }
        loop {
            self.skip_ws();
            let key = self.string()?;
            self.skip_ws();
            self.eat(b':')?;
            self.skip_ws();
            let v = self.value(depth + 1)?;
            members.push((key, v));
            self.skip_ws();
            match self.s.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Value::Object(members));
                }
                _ => return Err(self.err("expected ',' or '}'")),
            }
        }
    }

    fn array(&mut self, depth: usize) -> Result<Value, JsonError> {
        self.eat(b'[')?;
        let mut items = Vec::new();
        self.skip_ws();
        if self.s.get(self.pos) == Some(&b']') {
            self.pos += 1;
            return Ok(Value::Array(items));
        }
        loop {
            self.skip_ws();
            items.push(self.value(depth + 1)?);
            self.skip_ws();
            match self.s.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Value::Array(items));
                }
                _ => return Err(self.err("expected ',' or ']'")),
            }
        }
    }

    fn hex4(&mut self) -> Result<u32, JsonError> {
        let end = self.pos + 4;
        let slice = self
            .s
            .get(self.pos..end)
            .ok_or_else(|| self.err("short \\u escape"))?;
        let text = std::str::from_utf8(slice).map_err(|_| self.err("bad \\u escape"))?;
        let v = u32::from_str_radix(text, 16).map_err(|_| self.err("bad \\u escape"))?;
        self.pos = end;
        Ok(v)
    }

    fn string(&mut self) -> Result<String, JsonError> {
        self.eat(b'"')?;
        let mut out = String::new();
        loop {
            let start = self.pos;
            // Copy a run of plain characters at once (input is valid UTF-8
            // because it came from a &str and the delimiters are ASCII).
            while let Some(&c) = self.s.get(self.pos) {
                if c == b'"' || c == b'\\' || c < 0x20 {
                    break;
                }
                self.pos += 1;
            }
            out.push_str(
                std::str::from_utf8(&self.s[start..self.pos])
                    .map_err(|_| self.err("invalid UTF-8"))?,
            );
            match self.s.get(self.pos) {
                None => return Err(self.err("unterminated string")),
                Some(b'"') => {
                    self.pos += 1;
                    return Ok(out);
                }
                Some(b'\\') => {
                    self.pos += 1;
                    let esc = *self
                        .s
                        .get(self.pos)
                        .ok_or_else(|| self.err("unterminated escape"))?;
                    self.pos += 1;
                    match esc {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let hi = self.hex4()?;
                            let ch = if (0xD800..0xDC00).contains(&hi) {
                                if self.s[self.pos..].starts_with(b"\\u") {
                                    self.pos += 2;
                                    let lo = self.hex4()?;
                                    if !(0xDC00..0xE000).contains(&lo) {
                                        return Err(self.err("invalid low surrogate"));
                                    }
                                    char::from_u32(0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00))
                                } else {
                                    None
                                }
                            } else {
                                char::from_u32(hi)
                            };
                            out.push(ch.ok_or_else(|| self.err("invalid unicode escape"))?);
                        }
                        _ => return Err(self.err("invalid escape")),
                    }
                }
                Some(_) => return Err(self.err("control character in string")),
            }
        }
    }

    fn number(&mut self) -> Result<Value, JsonError> {
        let start = self.pos;
        if self.s.get(self.pos) == Some(&b'-') {
            self.pos += 1;
        }
        let digits_start = self.pos;
        while matches!(self.s.get(self.pos), Some(b'0'..=b'9')) {
            self.pos += 1;
        }
        let int_digits = self.pos - digits_start;
        if int_digits == 0 {
            return Err(self.err("expected digits"));
        }
        if int_digits > 1 && self.s[digits_start] == b'0' {
            return Err(self.err("leading zero in number"));
        }
        let mut is_float = false;
        if self.s.get(self.pos) == Some(&b'.') {
            is_float = true;
            self.pos += 1;
            let frac_start = self.pos;
            while matches!(self.s.get(self.pos), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if self.pos == frac_start {
                return Err(self.err("expected digits after '.'"));
            }
        }
        if matches!(self.s.get(self.pos), Some(b'e' | b'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.s.get(self.pos), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let exp_start = self.pos;
            while matches!(self.s.get(self.pos), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if self.pos == exp_start {
                return Err(self.err("expected exponent digits"));
            }
        }
        let text = std::str::from_utf8(&self.s[start..self.pos]).expect("ASCII number");
        if !is_float {
            if let Ok(i) = text.parse::<i128>() {
                return Ok(Value::Int(i));
            }
        }
        text.parse::<f64>()
            .map(Value::Float)
            .map_err(|_| self.err("invalid number"))
    }
}

/// Append `s` as a quoted, escaped JSON string.
pub fn push_str(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Append a float; JSON has no NaN or infinity, so those become `null`.
pub fn push_f64(out: &mut String, f: f64) {
    if f.is_finite() {
        let _ = write!(out, "{f}");
    } else {
        out.push_str("null");
    }
}

/// Builds one JSON object member by member, without an intermediate tree.
/// Used for log lines, where speed and allocation count matter.
pub struct ObjectWriter {
    buf: String,
    first: bool,
}

impl Default for ObjectWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectWriter {
    pub fn new() -> Self {
        ObjectWriter {
            buf: String::from("{"),
            first: true,
        }
    }

    fn key(&mut self, key: &str) {
        if !self.first {
            self.buf.push(',');
        }
        self.first = false;
        push_str(&mut self.buf, key);
        self.buf.push(':');
    }

    pub fn str(&mut self, key: &str, v: &str) -> &mut Self {
        self.key(key);
        push_str(&mut self.buf, v);
        self
    }

    pub fn u64(&mut self, key: &str, v: u64) -> &mut Self {
        self.key(key);
        let _ = write!(self.buf, "{v}");
        self
    }

    pub fn i64(&mut self, key: &str, v: i64) -> &mut Self {
        self.key(key);
        let _ = write!(self.buf, "{v}");
        self
    }

    pub fn bool(&mut self, key: &str, v: bool) -> &mut Self {
        self.key(key);
        self.buf.push_str(if v { "true" } else { "false" });
        self
    }

    pub fn null(&mut self, key: &str) -> &mut Self {
        self.key(key);
        self.buf.push_str("null");
        self
    }

    pub fn f64(&mut self, key: &str, v: f64) -> &mut Self {
        self.key(key);
        push_f64(&mut self.buf, v);
        self
    }

    /// A value that is already valid JSON (a nested object or array).
    pub fn raw(&mut self, key: &str, json: &str) -> &mut Self {
        self.key(key);
        self.buf.push_str(json);
        self
    }

    /// `0x` + 8 upper-case hex digits, as a string.
    pub fn hex32(&mut self, key: &str, v: u32) -> &mut Self {
        self.key(key);
        let _ = write!(self.buf, "\"0x{v:08X}\"");
        self
    }

    /// Lower-case hex of a byte slice, as a string.
    pub fn hex_bytes(&mut self, key: &str, v: &[u8]) -> &mut Self {
        self.key(key);
        self.buf.push('"');
        self.buf.push_str(&crate::hex::encode(v));
        self.buf.push('"');
        self
    }

    pub fn finish(mut self) -> String {
        self.buf.push('}');
        self.buf
    }
}

/// Join already-serialised values into a JSON array.
pub fn array_of(items: &[String]) -> String {
    let mut out = String::from("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(item);
    }
    out.push(']');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_scalars_and_containers() {
        let v =
            Value::parse(r#" {"a":1,"b":[true,false,null],"c":{"d":"x"},"e":-2.5e1} "#).unwrap();
        assert_eq!(v.get("a").unwrap().as_i128(), Some(1));
        let b = v.get("b").unwrap().as_array().unwrap();
        assert_eq!(b[0].as_bool(), Some(true));
        assert_eq!(b[2], Value::Null);
        assert_eq!(v.get("c").unwrap().get("d").unwrap().as_str(), Some("x"));
        assert_eq!(v.get("e").unwrap().as_f64(), Some(-25.0));
    }

    #[test]
    fn integers_keep_full_precision() {
        let v = Value::parse("18446744073709551615").unwrap();
        assert_eq!(v.as_i128(), Some(u64::MAX as i128));
        let v = Value::parse("-9223372036854775808").unwrap();
        assert_eq!(v.as_i128(), Some(i64::MIN as i128));
    }

    #[test]
    fn string_escapes() {
        let v = Value::parse(r#""a\"b\\c\n\tAé😀""#).unwrap();
        assert_eq!(v.as_str(), Some("a\"b\\c\n\tA\u{e9}\u{1f600}"));
    }

    #[test]
    fn rejects_malformed_input() {
        for bad in [
            "",
            "{",
            "[1,]",
            r#"{"a":}"#,
            "01",
            "1.",
            "-",
            "nul",
            r#""abc"#,
            r#""\x""#,
            "[1] 2",
            r#"{"a" 1}"#,
            "\"\u{1}\"",
            r#""\ud800""#,
            "1e",
        ] {
            assert!(Value::parse(bad).is_err(), "should reject {bad:?}");
        }
    }

    #[test]
    fn nesting_is_limited() {
        let deep = "[".repeat(200) + &"]".repeat(200);
        assert!(Value::parse(&deep).is_err());
    }

    #[test]
    fn write_then_parse_round_trips() {
        let v = Value::Object(vec![
            ("s".into(), Value::Str("q\"\\\n\u{1}é".into())),
            ("i".into(), Value::Int(-7)),
            ("f".into(), Value::Float(0.1)),
            (
                "a".into(),
                Value::Array(vec![Value::Null, Value::Bool(true)]),
            ),
        ]);
        let text = v.to_json();
        assert_eq!(Value::parse(&text).unwrap(), v);
        assert!(!text.contains('\n'));
    }

    #[test]
    fn non_finite_floats_are_null() {
        assert_eq!(Value::Float(f64::NAN).to_json(), "null");
        assert_eq!(Value::Float(f64::INFINITY).to_json(), "null");
    }

    #[test]
    fn object_writer_builds_valid_json() {
        let mut o = ObjectWriter::new();
        o.str("id", "a\"b")
            .u64("n", 5)
            .i64("m", -3)
            .bool("ok", true)
            .null("x");
        o.f64("f", 1.5)
            .hex32("h", 0x27f)
            .hex_bytes("b", &[1, 0xab])
            .raw("r", "[1,2]");
        let text = o.finish();
        let v = Value::parse(&text).unwrap();
        assert_eq!(v.get("id").unwrap().as_str(), Some("a\"b"));
        assert_eq!(v.get("h").unwrap().as_str(), Some("0x0000027F"));
        assert_eq!(v.get("b").unwrap().as_str(), Some("01ab"));
        assert_eq!(v.get("r").unwrap().as_array().unwrap().len(), 2);
        assert_eq!(array_of(&["1".into(), "\"a\"".into()]), "[1,\"a\"]");
        assert_eq!(array_of(&[]), "[]");
    }
}
