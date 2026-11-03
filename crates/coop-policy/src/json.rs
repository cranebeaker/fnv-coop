//! A small JSON reader for the policy file, on serde alone.
//!
//! The crate used `serde_json` before. `world` depends on this crate, so `serde_json` was linked
//! into every crate that tests against `world`. `serde_json` adds `PartialEq<Value>` impls for the
//! integer types, and then `assert_eq!(x, vec![vec![]])` in those tests cannot infer its type
//! (E0282/E0283 in `engine-fallout`). This reader adds no impls on foreign types.

use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::{self, DeserializeOwned, IntoDeserializer, Visitor};
use serde::forward_to_deserialize_any;
use std::fmt;

/// Parse `text` as one JSON value and deserialize it into `T`.
pub fn from_str<T: DeserializeOwned>(text: &str) -> Result<T, Error> {
    let mut p = Parser {
        s: text.as_bytes(),
        i: 0,
    };
    p.ws();
    let v = p.value(0)?;
    p.ws();
    if p.i != p.s.len() {
        return Err(p.err("trailing characters"));
    }
    T::deserialize(v)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Null,
    Bool(bool),
    U(u64),
    I(i64),
    F(f64),
    Str(String),
    Arr(Vec<Value>),
    Obj(Vec<(String, Value)>),
}

/// Nesting limit, so a hostile file cannot overflow the stack.
const MAX_DEPTH: usize = 128;

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn err(&self, what: &str) -> Error {
        let done = &self.s[..self.i.min(self.s.len())];
        let line = done.iter().filter(|&&b| b == b'\n').count() + 1;
        let col = self.i - done.iter().rposition(|&b| b == b'\n').map_or(0, |p| p + 1) + 1;
        Error(format!("{what} at line {line} column {col}"))
    }

    fn ws(&mut self) {
        while let Some(b' ' | b'\t' | b'\n' | b'\r') = self.s.get(self.i) {
            self.i += 1;
        }
    }

    fn eat(&mut self, lit: &str) -> Result<(), Error> {
        if self.s[self.i..].starts_with(lit.as_bytes()) {
            self.i += lit.len();
            Ok(())
        } else {
            Err(self.err("expected value"))
        }
    }

    fn value(&mut self, depth: usize) -> Result<Value, Error> {
        if depth > MAX_DEPTH {
            return Err(self.err("nesting too deep"));
        }
        match self.s.get(self.i) {
            None => Err(self.err("EOF while parsing a value")),
            Some(b'n') => self.eat("null").map(|_| Value::Null),
            Some(b't') => self.eat("true").map(|_| Value::Bool(true)),
            Some(b'f') => self.eat("false").map(|_| Value::Bool(false)),
            Some(b'"') => self.string().map(Value::Str),
            Some(b'[') => {
                self.i += 1;
                let mut out = Vec::new();
                self.ws();
                if self.s.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(Value::Arr(out));
                }
                loop {
                    self.ws();
                    out.push(self.value(depth + 1)?);
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(Value::Arr(out));
                        }
                        _ => return Err(self.err("expected `,` or `]`")),
                    }
                }
            }
            Some(b'{') => {
                self.i += 1;
                let mut out = Vec::new();
                self.ws();
                if self.s.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(Value::Obj(out));
                }
                loop {
                    self.ws();
                    if self.s.get(self.i) != Some(&b'"') {
                        return Err(self.err("key must be a string"));
                    }
                    let k = self.string()?;
                    self.ws();
                    if self.s.get(self.i) != Some(&b':') {
                        return Err(self.err("expected `:`"));
                    }
                    self.i += 1;
                    self.ws();
                    out.push((k, self.value(depth + 1)?));
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(Value::Obj(out));
                        }
                        _ => return Err(self.err("expected `,` or `}`")),
                    }
                }
            }
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(_) => Err(self.err("expected value")),
        }
    }

    fn number(&mut self) -> Result<Value, Error> {
        let start = self.i;
        if self.s.get(self.i) == Some(&b'-') {
            self.i += 1;
        }
        let digits = |p: &mut Self| {
            let d = p.i;
            while let Some(b'0'..=b'9') = p.s.get(p.i) {
                p.i += 1;
            }
            p.i - d
        };
        let int_start = self.i;
        if digits(self) == 0 {
            return Err(self.err("invalid number"));
        }
        if self.s[int_start] == b'0' && self.i - int_start > 1 {
            return Err(self.err("invalid number"));
        }
        let mut float = false;
        if self.s.get(self.i) == Some(&b'.') {
            self.i += 1;
            float = true;
            if digits(self) == 0 {
                return Err(self.err("invalid number"));
            }
        }
        if let Some(b'e' | b'E') = self.s.get(self.i) {
            self.i += 1;
            float = true;
            if let Some(b'+' | b'-') = self.s.get(self.i) {
                self.i += 1;
            }
            if digits(self) == 0 {
                return Err(self.err("invalid number"));
            }
        }
        let text = std::str::from_utf8(&self.s[start..self.i]).expect("ASCII digits");
        if !float {
            if let Ok(n) = text.parse::<u64>() {
                return Ok(Value::U(n));
            }
            if let Ok(n) = text.parse::<i64>() {
                return Ok(Value::I(n));
            }
        }
        text.parse::<f64>()
            .map(Value::F)
            .map_err(|_| self.err("invalid number"))
    }

    fn hex4(&mut self) -> Result<u32, Error> {
        let h = self
            .s
            .get(self.i..self.i + 4)
            .ok_or_else(|| self.err("EOF in escape"))?;
        let h = std::str::from_utf8(h).map_err(|_| self.err("invalid escape"))?;
        let n = u32::from_str_radix(h, 16).map_err(|_| self.err("invalid escape"))?;
        self.i += 4;
        Ok(n)
    }

    fn string(&mut self) -> Result<String, Error> {
        self.i += 1; // opening quote
        let mut out = Vec::new();
        loop {
            let b = *self
                .s
                .get(self.i)
                .ok_or_else(|| self.err("EOF while parsing a string"))?;
            self.i += 1;
            match b {
                b'"' => break,
                b'\\' => {
                    let e = *self
                        .s
                        .get(self.i)
                        .ok_or_else(|| self.err("EOF in escape"))?;
                    self.i += 1;
                    let c = match e {
                        b'"' => '"',
                        b'\\' => '\\',
                        b'/' => '/',
                        b'b' => '\u{8}',
                        b'f' => '\u{c}',
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'u' => {
                            let hi = self.hex4()?;
                            let code = if (0xD800..0xDC00).contains(&hi) {
                                if !self.s[self.i..].starts_with(b"\\u") {
                                    return Err(self.err("lone surrogate"));
                                }
                                self.i += 2;
                                let lo = self.hex4()?;
                                if !(0xDC00..0xE000).contains(&lo) {
                                    return Err(self.err("lone surrogate"));
                                }
                                0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00)
                            } else {
                                hi
                            };
                            char::from_u32(code).ok_or_else(|| self.err("lone surrogate"))?
                        }
                        _ => return Err(self.err("invalid escape")),
                    };
                    let mut buf = [0u8; 4];
                    out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                }
                0x00..=0x1F => return Err(self.err("control character in string")),
                _ => out.push(b),
            }
        }
        String::from_utf8(out).map_err(|_| self.err("invalid UTF-8 in string"))
    }
}

impl<'de> IntoDeserializer<'de, Error> for Value {
    type Deserializer = Value;
    fn into_deserializer(self) -> Value {
        self
    }
}

impl<'de> de::Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Error> {
        match self {
            Value::Null => v.visit_unit(),
            Value::Bool(b) => v.visit_bool(b),
            Value::U(n) => v.visit_u64(n),
            Value::I(n) => v.visit_i64(n),
            Value::F(n) => v.visit_f64(n),
            Value::Str(s) => v.visit_string(s),
            Value::Arr(a) => {
                let mut d = SeqDeserializer::new(a.into_iter());
                let out = v.visit_seq(&mut d)?;
                d.end()?;
                Ok(out)
            }
            Value::Obj(o) => {
                let mut d = MapDeserializer::new(o.into_iter());
                let out = v.visit_map(&mut d)?;
                d.end()?;
                Ok(out)
            }
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Error> {
        match self {
            Value::Null => v.visit_none(),
            other => v.visit_some(other),
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        v: V,
    ) -> Result<V::Value, Error> {
        match self {
            // A unit variant is a string.
            Value::Str(s) => v.visit_enum(s.into_deserializer()),
            // Any other variant is a map with one entry, as serde_json writes it.
            Value::Obj(mut o) if o.len() == 1 => {
                let (k, val) = o.pop().expect("one entry");
                v.visit_enum(Variant { key: k, value: val })
            }
            _ => Err(de::Error::custom(
                "expected a string or a map with one entry for an enum",
            )),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

struct Variant {
    key: String,
    value: Value,
}

impl<'de> de::EnumAccess<'de> for Variant {
    type Error = Error;
    type Variant = Value;
    fn variant_seed<S: de::DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, Value), Error> {
        let key: serde::de::value::StringDeserializer<Error> = self.key.into_deserializer();
        Ok((seed.deserialize(key)?, self.value))
    }
}

impl<'de> de::VariantAccess<'de> for Value {
    type Error = Error;
    fn unit_variant(self) -> Result<(), Error> {
        match self {
            Value::Null => Ok(()),
            _ => Err(de::Error::custom("expected null for a unit variant")),
        }
    }
    fn newtype_variant_seed<S: de::DeserializeSeed<'de>>(self, seed: S) -> Result<S::Value, Error> {
        seed.deserialize(self)
    }
    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, v: V) -> Result<V::Value, Error> {
        de::Deserializer::deserialize_seq(self, v)
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        v: V,
    ) -> Result<V::Value, Error> {
        de::Deserializer::deserialize_map(self, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::collections::BTreeMap;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum Kind {
        PerPlayer,
        Shared,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Row {
        name: String,
        n: u32,
        neg: i32,
        f: f64,
        opt: Option<u32>,
        none: Option<u32>,
        #[serde(default)]
        missing: Vec<u32>,
        list: Vec<Kind>,
        map: BTreeMap<Kind, bool>,
    }

    #[test]
    fn reads_every_value_kind() {
        let r: Row = from_str(
            r#" { "name": "a\"b\\c\u00e9\ud83d\ude00", "n": 7, "neg": -3, "f": 1.5e1,
                  "opt": 4, "none": null, "list": ["shared", "per_player"],
                  "map": {"per_player": true, "shared": false} } "#,
        )
        .unwrap();
        assert_eq!(r.name, "a\"b\\c\u{e9}\u{1F600}");
        assert_eq!(
            (r.n, r.neg, r.f, r.opt, r.none),
            (7, -3, 15.0, Some(4), None)
        );
        assert!(r.missing.is_empty());
        assert_eq!(r.list, [Kind::Shared, Kind::PerPlayer]);
        assert!(r.map[&Kind::PerPlayer]);
    }

    #[test]
    fn rejects_bad_input() {
        for bad in [
            "",
            "{",
            "[1,]",
            "{\"a\":1,}",
            "01",
            "1.",
            "\"\\x\"",
            "\"\\ud800\"",
            "\"a\nb\"",
            "{} x",
            "tru",
        ] {
            assert!(
                from_str::<serde::de::IgnoredAny>(bad).is_err(),
                "accepted {bad:?}"
            );
        }
        let deep = "[".repeat(MAX_DEPTH + 2) + &"]".repeat(MAX_DEPTH + 2);
        assert!(from_str::<serde::de::IgnoredAny>(&deep).is_err());
    }

    #[test]
    fn type_and_field_errors_come_from_serde() {
        assert!(from_str::<u8>("300").is_err());
        assert!(from_str::<u32>("-1").is_err());
        assert!(from_str::<Kind>("\"other\"").is_err());
        let e = from_str::<Row>(r#"{"name":"x","bogus":1}"#).unwrap_err();
        assert!(e.to_string().contains("bogus"), "{e}");
    }

    #[test]
    fn error_has_line_and_column() {
        let e = from_str::<serde::de::IgnoredAny>("{\n  \"a\": ?\n}").unwrap_err();
        assert_eq!(e.to_string(), "expected value at line 2 column 8");
    }
}
