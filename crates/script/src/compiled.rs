//! The compiled form of a script (`SCDA`): what the game actually runs.
//! It's a list of statements, each a 2-byte code and a 2-byte length
//! followed by that many bytes, except that a call on a reference is
//! preceded by a 4-byte marker (code `0x1C` and the reference's number in
//! the record's `SCRO` list, counting from 1). Codes from `0x1000` are
//! function calls (`0x1000` + the function's number); the ones below are
//! the language's keywords.
//!
//! The source is what [`crate::parse`] reads; this is for checking what
//! the game's own compiler made of source that's oddly written.

/// One compiled statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement<'a> {
    /// Where it starts in the compiled bytes.
    pub offset: usize,
    /// For a call on a reference: its number in the `SCRO` list (from 1).
    pub on: Option<u16>,
    pub code: u16,
    pub data: &'a [u8],
}

const REFERENCE_PREFIX: u16 = 0x1C;

/// Splits compiled bytes into statements. Stops at the end or at bytes
/// that don't fit (a length past the end).
pub fn statements(bytes: &[u8]) -> Vec<Statement<'_>> {
    let mut out = Vec::new();
    let mut at = 0;
    let u16_at = |i: usize| -> Option<u16> {
        bytes
            .get(i..i + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
    };
    while let Some(mut code) = u16_at(at) {
        let offset = at;
        let mut on = None;
        if code == REFERENCE_PREFIX {
            on = u16_at(at + 2);
            at += 4;
            match u16_at(at) {
                Some(c) => code = c,
                None => break,
            }
        }
        let Some(len) = u16_at(at + 2) else { break };
        let start = at + 4;
        let end = start + usize::from(len);
        if end > bytes.len() {
            break;
        }
        out.push(Statement {
            offset,
            on,
            code,
            data: &bytes[start..end],
        });
        at = end;
    }
    out
}

/// A token of a compiled expression. Expressions are stored in evaluation
/// order (operands before their operator), each token after a space.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    /// A number or an operator, as text (`110`, `0.5`, `==`, `&&`).
    Text(&'a str),
    /// A local variable: `s` (whole number) or `f` (float), and its index.
    Local(u8, u16),
    /// A global variable, by `SCRO` number.
    Global(u16),
    /// A reference as a value, by `SCRO` number (stored `Z`, or `r` with
    /// nothing after it).
    Ref(u16),
    /// A reference owning the variable (`Local`) or receiving the call
    /// (`Call`) that follows.
    Owner(u16),
    /// A function call: its code and its parameter bytes (which start with
    /// the parameter count).
    Call(u16, &'a [u8]),
}

/// The tokens of a compiled expression, or `None` if the bytes don't read.
pub fn expression(bytes: &[u8]) -> Option<Vec<Token<'_>>> {
    let mut out = Vec::new();
    let mut at = 0;
    let u16_at = |i: usize| -> Option<u16> {
        bytes
            .get(i..i + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
    };
    // Whether the previous token joins the next one (a reference before
    // its variable or call).
    let mut joined = false;
    while at < bytes.len() {
        if bytes[at] == b' ' {
            at += 1;
        } else if !joined {
            return None;
        }
        joined = false;
        match *bytes.get(at)? {
            b'X' => {
                let code = u16_at(at + 1)?;
                let len = usize::from(u16_at(at + 3)?);
                let params = bytes.get(at + 5..at + 5 + len)?;
                out.push(Token::Call(code, params));
                at += 5 + len;
            }
            b'r' => {
                let r = u16_at(at + 1)?;
                at += 3;
                joined = bytes.get(at).is_some_and(|&b| b != b' ');
                out.push(if joined {
                    Token::Owner(r)
                } else {
                    Token::Ref(r)
                });
            }
            c @ (b's' | b'f') => {
                out.push(Token::Local(c, u16_at(at + 1)?));
                at += 3;
            }
            b'G' => {
                out.push(Token::Global(u16_at(at + 1)?));
                at += 3;
            }
            b'Z' => {
                out.push(Token::Ref(u16_at(at + 1)?));
                at += 3;
            }
            _ => {
                let end = bytes[at..]
                    .iter()
                    .position(|&b| b == b' ')
                    .map_or(bytes.len(), |p| at + p);
                out.push(Token::Text(std::str::from_utf8(&bytes[at..end]).ok()?));
                at = end;
            }
        }
    }
    Some(out)
}

/// An expression's tokens as text: `r1.GetDead() 1 ==`.
pub fn expression_text(tokens: &[Token]) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut owner: Option<u16> = None;
    for t in tokens {
        let prefix = owner.take().map(|r| format!("r{r}.")).unwrap_or_default();
        parts.push(match t {
            Token::Text(s) => s.to_string(),
            Token::Local(k, i) => format!("{prefix}{}{i}", *k as char),
            Token::Global(i) => format!("g{i}"),
            Token::Ref(r) => format!("r{r}"),
            Token::Owner(r) => {
                owner = Some(*r);
                continue;
            }
            Token::Call(code, params) => {
                // The parameters after their count, as bytes.
                let bytes: Vec<String> =
                    params.iter().skip(2).map(|b| format!("{b:02X}")).collect();
                format!("{prefix}{}({})", code_name(*code), bytes.join(" "))
            }
        });
    }
    parts.join(" ")
}

/// An expression's shape: `o` for each value, operators as stored
/// (`o o == o &&`). For comparing with a parsed expression.
pub fn expression_shape(tokens: &[Token]) -> String {
    tokens
        .iter()
        .filter_map(|t| match t {
            Token::Owner(_) => None,
            Token::Text(s) if s.parse::<f64>().is_err() => Some(s.to_string()),
            _ => Some("o".to_string()),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The keyword or function a statement code stands for.
pub fn code_name(code: u16) -> String {
    match code {
        0x10 => "Begin".into(),
        0x11 => "End".into(),
        0x15 => "Set".into(),
        0x16 => "If".into(),
        0x17 => "Else".into(),
        0x18 => "ElseIf".into(),
        0x19 => "EndIf".into(),
        0x1D => "ScriptName".into(),
        0x1E => "Return".into(),
        c if c >= 0x1000 => crate::function_name(c - 0x1000),
        c => format!("code {c:#x}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_statements_and_reference_calls() {
        // ScriptName; Begin GameMode (block of 12 bytes); Player(ref 1)
        // .Disable with no parameters; End.
        let bytes = [
            0x1D, 0x00, 0x00, 0x00, // ScriptName
            0x10, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, // Begin
            0x1C, 0x00, 0x01, 0x00, 0x22, 0x10, 0x00, 0x00, // 1.Disable
            0x11, 0x00, 0x00, 0x00, // End
        ];
        let s = statements(&bytes);
        assert_eq!(s.len(), 4);
        assert_eq!(code_name(s[0].code), "ScriptName");
        assert_eq!(s[1].data.len(), 6);
        assert_eq!(s[2].on, Some(1));
        assert_eq!(s[2].offset, 14);
        assert_eq!(code_name(s[2].code), crate::function_name(0x22));
        assert_eq!(code_name(s[3].code), "End");
    }

    #[test]
    fn reads_expressions() {
        // `GetStage VMQYesMan01a 110 != 1` as the game stores it, then a
        // reference's variable and a bare reference.
        let mut bytes = vec![
            b' ', b'X', 0x3A, 0x10, 0x05, 0x00, 0x01, 0x00, b'r', 0x01, 0x00,
        ];
        bytes.extend_from_slice(b" 110 1 != ");
        bytes.extend_from_slice(&[b'r', 0x02, 0x00, b's', 0x04, 0x00, b' ', b'r', 0x03, 0x00]);
        let t = expression(&bytes).unwrap();
        assert_eq!(t.len(), 7);
        assert_eq!(t[0], Token::Call(0x103A, &[0x01, 0x00, b'r', 0x01, 0x00]));
        assert_eq!(t[3], Token::Text("!="));
        assert_eq!(t[4], Token::Owner(2));
        assert_eq!(t[6], Token::Ref(3));
        assert_eq!(
            expression_text(&t),
            format!("{}(72 01 00) 110 1 != r2.s4 r3", crate::function_name(0x3A))
        );
        assert_eq!(expression_shape(&t), "o o o != o o");
    }
}
